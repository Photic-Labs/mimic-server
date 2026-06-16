use std::sync::{Arc, RwLock};

use axum::{body::Body, extract::Request};
use rusqlite::{Connection, Error};
use tokio::sync::mpsc;

use crate::{
    queries::{GET_API_PREFIX_SQL, GET_PORT_SQL, INSERT_TRAFFIC_LOG_SQL},
    types::MockAPICallerLog,
    RouteEntry, RoutingTable,
};

pub fn read_port(conn: &Connection) -> Result<u16, Error> {
    let result = conn.query_row(GET_PORT_SQL, [], |row| {
        Ok(row.get::<_, u16>("port").unwrap_or(8080))
    });
    match result {
        Ok(val) => Ok(val),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            println!("[mimic] No port in app_config — defaulting to 8080");
            Ok(8080)
        }
        Err(e) => Err(e),
    }
}

pub fn read_global_prefix(conn: &Connection) -> Result<String, Error> {
    let result = conn.query_row(GET_API_PREFIX_SQL, [], |row| {
        Ok(row.get::<_, String>("prefix").unwrap_or_default())
    });
    match result {
        Ok(val) => Ok(val),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(String::new()),
        Err(e) => Err(e),
    }
}

pub fn get_request_entry(
    request: Request<Body>,
    routing_table: Arc<RwLock<RoutingTable>>,
    global_prefix: &str,
) -> (Option<RouteEntry>, String, String, String) {
    let method = request.method().as_str().to_string();
    let raw_path = request.uri().path().to_string();

    // ── Prefix enforcement ───────────────────────────────────────────
    let stripped_path = if !global_prefix.is_empty() {
        match raw_path.strip_prefix(global_prefix) {
            Some(remainder) => {
                if remainder.is_empty() {
                    "/".to_string()
                } else if remainder.starts_with('/') {
                    remainder.to_string()
                } else {
                    // Partial match e.g. prefix="/api", path="/apiXYZ" → reject
                    return (None, method, raw_path.clone(), raw_path);
                }
            }
            // Prefix configured but path doesn't start with it → hard 404
            None => return (None, method, raw_path.clone(), raw_path),
        }
    } else {
        raw_path.clone()
    };

    let (method, path) = normalize_key(&method, &stripped_path);

    match routing_table.read() {
        Ok(table) => (
            match_route_entry(&table, &method, &path),
            method,
            path,
            raw_path, // ← the original URL that was hit
        ),
        Err(e) => {
            eprintln!("[handler] Routing table lock poisoned: {e}");
            (None, method, raw_path.clone(), raw_path)
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TRAFFIC LOG WRITER
// Runs in its own task — drains the channel and writes to SQLite.
// When the channel closes (server stops), this task exits cleanly.
// ─────────────────────────────────────────────────────────────────────────────

pub async fn write_mock_api_caller_logs(
    db_path: &str,
    mut rx: mpsc::UnboundedReceiver<MockAPICallerLog>,
) {
    let conn = match Connection::open(db_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[traffic] Failed to open DB for log writer: {e}");
            return;
        }
    };

    while let Some(log) = rx.recv().await {
        if let Err(e) = insert_traffic_log(&conn, &log) {
            eprintln!("[traffic] Failed to write log: {e}");
            // Don't crash the writer — just skip this entry and continue
        }
    }
}

fn insert_traffic_log(conn: &Connection, log: &MockAPICallerLog) -> Result<(), rusqlite::Error> {
    conn.execute(
        INSERT_TRAFFIC_LOG_SQL,
        rusqlite::params![
            log.route_id,
            log.timestamp,
            log.method,
            log.url_hit,
            log.latency_ms,
            log.response_status,
        ],
    )?;
    Ok(())
}

/// Normalize a method + path pair into a consistent HashMap key.
///
/// Rules:
///   - method  → trimmed, uppercased          "get " → "GET"
///   - path    → trimmed, trailing / stripped  "/api/auth/" → "/api/auth"
///   - empty path → normalized to "/"
///
/// Both the compiler and the handler use this same function
/// so keys always match regardless of how the user typed the path.
pub fn normalize_key(method: &str, path: &str) -> (String, String) {
    let method = method.trim().to_uppercase();

    let path = path.trim();
    let path = path.trim_end_matches('/');
    let path = if path.is_empty() {
        "/".to_string()
    } else {
        path.to_string()
    };

    (method, path)
}

/// Build a consistent JSON error body for all failure cases
pub fn error_body(error_code: &str, method: &str, path: &str, detail: Option<&str>) -> String {
    let mut obj = serde_json::json!({
        "error":  error_code,
        "method": method,
        "path":   path,
    });

    if let Some(d) = detail {
        obj["detail"] = serde_json::Value::String(d.to_string());
    }

    obj.to_string()
}

/// Two-phase route lookup.
///
/// Phase 1 — Exact match (O(1) HashMap lookup)
/// Phase 2 — Pattern match: split both stored key and incoming path
///            by '/' and check segment by segment. A stored segment
///            that starts with ':' matches ANY incoming segment.
///
/// Returns the first matching RouteEntry, or None.
pub fn match_route_entry(table: &RoutingTable, method: &str, path: &str) -> Option<RouteEntry> {
    // ── Phase 1: exact match ─────────────────────────────────────
    let key = (method.to_string(), path.to_string());
    if let Some(entry) = table.get(&key) {
        return Some(entry.clone());
    }

    // ── Phase 2: pattern match ───────────────────────────────────
    let incoming_segments: Vec<&str> = path.split('/').collect();

    for ((stored_method, stored_path), entry) in table.iter() {
        // Method must match exactly
        if stored_method != method {
            continue;
        }

        // Must contain at least one ':' param — skip pure static routes
        // (they would have matched in Phase 1 already)
        if !stored_path.contains(':') {
            continue;
        }

        let stored_segments: Vec<&str> = stored_path.split('/').collect();

        // Segment count must match
        if stored_segments.len() != incoming_segments.len() {
            continue;
        }

        // Walk segment by segment
        let matched = stored_segments
            .iter()
            .zip(incoming_segments.iter())
            .all(|(stored, incoming)| stored.starts_with(':') || stored == incoming);

        if matched {
            return Some(entry.clone());
        }
    }

    None
}
