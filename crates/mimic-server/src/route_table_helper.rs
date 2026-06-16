// ─────────────────────────────────────────────────────────────────────────────
// PUBLIC API
// ─────────────────────────────────────────────────────────────────────────────

use anyhow::Result;
use rusqlite::Connection;

use crate::{helpers::normalize_key, queries::GET_MOCKED_ROUTES_SQL, RouteEntry, RoutingTable};

/// Read all routes from SQLite and compile them into a RoutingTable.
///
/// This is called:
///   1. On server start — to load the initial routing table
///   2. On reload()    — after the user saves or deletes a route
///
/// The returned HashMap is cheap to swap into the Arc<RwLock<>> —
/// it replaces the old table atomically from the handler's perspective.
pub fn get_routing_table(conn: &Connection) -> Result<RoutingTable, rusqlite::Error> {
    let mut stmt = conn.prepare(GET_MOCKED_ROUTES_SQL)?;
    let rows = stmt.query_map([], |row| {
        let method: String = row.get(1)?;
        let path: String = row.get(2)?;
        let status_code: u16 = row.get(3)?;
        let response_path: Option<String> = row.get(4)?;
        let route_id: String = row.get(0)?;

        Ok((
            method,
            path,
            RouteEntry {
                route_id,
                status: status_code,
                response_path,
            },
        ))
    })?;
    let mut table = RoutingTable::new();
    for row in rows {
        let (method, path, entry) = row?;
        let key = normalize_key(&method, &path);
        // If two routes have the same method + path (shouldn't happen,
        // but just in case) — last one wins, log a warning
        if table.contains_key(&key) {
            eprintln!(
                "[compiler] WARNING: duplicate route key {:?} — \
                         route_id {} overrides previous entry",
                key, entry.route_id
            );
        }
        table.insert(key, entry);
    }
    Ok(table)
}
