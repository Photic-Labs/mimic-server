use serde::{Deserialize, Serialize};

// ── AppConfig ─────────────────────────────────────────────────────────────────
// Maps to: app_config table
// id is always 1 (single-row config), not exposed in the struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub prefix: String,
    pub theme: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 8080,
            prefix: "".into(),
            theme: "dark".into(),
        }
    }
}

// ── ApiGroup ──────────────────────────────────────────────────────────────────
// Maps to: api_groups table
// id: TEXT PRIMARY KEY — UUID4 generated in Rust on insert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiGroup {
    pub id: String, // uuid4
    pub name: String,
}

// ── MockedApiRoute ────────────────────────────────────────────────────────────
// Maps to: mocked_routes table
// id:       TEXT PRIMARY KEY — UUID4
// group_id: TEXT FK → api_groups.id — UUID4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockedApiRoute {
    pub id: String,                    // uuid4
    pub group_id: String,              // uuid4 → api_groups.id
    pub method: String,                // GET, POST, PUT, PATCH, DELETE
    pub path: String,                  // e.g. /auth/me
    pub status_code: u16,              // e.g. 200
    pub response_type: String,         // e.g. "file", "inline", "passthrough"
    pub response_path: Option<String>, // path to .json file on disk
}

// ── TrafficLog ────────────────────────────────────────────────────────────────
// Maps to: traffic_logs table
// id:       INTEGER AUTOINCREMENT — stays i64, traffic logs are local-only
// route_id: TEXT FK → mocked_routes.id — UUID4, nullable (route may be deleted)
// timestamp: Unix epoch INTEGER from strftime('%s','now')
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficLog {
    pub id: i64,
    pub route_id: Option<String>, // uuid4 → mocked_routes.id, nullable
    pub timestamp: i64,           // Unix epoch seconds
    pub method: String,
    pub url_hit: String,
    pub latency_ms: Option<i64>,
    pub response_status: Option<u16>,
}

// ── RouteTag ──────────────────────────────────────────────────────────────────
// Maps to: route_tags table
// route_id: TEXT FK → mocked_routes.id — UUID4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteTag {
    pub route_id: String, // uuid4 → mocked_routes.id
    pub tag_name: String,
}
