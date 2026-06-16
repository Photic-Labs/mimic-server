pub const GET_PORT_SQL: &str = "SELECT port FROM app_config LIMIT 1";
pub const GET_API_PREFIX_SQL: &str = "SELECT prefix FROM app_config LIMIT 1";
pub const INSERT_TRAFFIC_LOG_SQL: &str = "INSERT INTO traffic_logs (route_id, timestamp, method, url_hit, latency_ms, response_status) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";
pub const GET_MOCKED_ROUTES_SQL: &str =
    "SELECT id, method, path, status_code, response_path FROM mocked_routes ORDER BY id ASC";
