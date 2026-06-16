pub const APP_CONFIG_SQL: &str = "
    CREATE TABLE IF NOT EXISTS app_config (
        id      INTEGER PRIMARY KEY CHECK (id = 1),
        host    TEXT    NOT NULL DEFAULT '127.0.0.1',
        port    INTEGER NOT NULL DEFAULT 8080,
        theme   TEXT    NOT NULL DEFAULT 'dark',
        prefix  TEXT    NOT NULL DEFAULT ''
    );
";

pub const API_GROUPS_SQL: &str = "
    CREATE TABLE IF NOT EXISTS api_groups (
        id      TEXT PRIMARY KEY,
        name    TEXT NOT NULL UNIQUE
    );
";

pub const MOCKED_ROUTES_SQL: &str = "
    CREATE TABLE IF NOT EXISTS mocked_routes (
        id            TEXT    PRIMARY KEY,
        group_id      TEXT    NOT NULL,
        method        TEXT    NOT NULL,
        path          TEXT    NOT NULL,
        status_code   INTEGER NOT NULL,
        response_type TEXT    NOT NULL,
        response_path TEXT,
        FOREIGN KEY (group_id) REFERENCES api_groups(id) ON DELETE CASCADE,
        UNIQUE(method, path)
    );
";

pub const ROUTE_TAGS_SQL: &str = "
    CREATE TABLE IF NOT EXISTS route_tags (
        route_id  TEXT NOT NULL,
        tag_name  TEXT NOT NULL,
        PRIMARY KEY (route_id, tag_name),
        FOREIGN KEY (route_id) REFERENCES mocked_routes(id) ON DELETE CASCADE
    );
";

pub const TRAFFIC_LOGS_SQL: &str = "
    CREATE TABLE IF NOT EXISTS traffic_logs (
        id              INTEGER PRIMARY KEY AUTOINCREMENT,
        route_id        TEXT,
        timestamp       INTEGER DEFAULT (strftime('%s', 'now')),
        method          TEXT    NOT NULL,
        url_hit         TEXT    NOT NULL,
        latency_ms      INTEGER,
        response_status INTEGER,
        FOREIGN KEY (route_id) REFERENCES mocked_routes(id) ON DELETE SET NULL
    );
";

// ── Config ────────────────────────────────────────────────────────────────────

pub const APP_CONFIG_SEED_SQL: &str = "
    INSERT OR IGNORE INTO app_config (id, host, port, prefix, theme)
    VALUES (1, '127.0.0.1', 8080, '', 'dark')
";

pub const LOAD_CONFIG_SQL: &str = "
    SELECT host, port, prefix, theme FROM app_config LIMIT 1
";

pub const SAVE_CONFIG_SQL: &str = "
    UPDATE app_config SET port = ?2, prefix = ?3, theme = ?4 WHERE id = 1
";

// ── API Groups ────────────────────────────────────────────────────────────────

// ?1 = id (uuid4 from Rust), ?2 = name
pub const INSERT_API_GROUP_SQL: &str = "
    INSERT INTO api_groups (id, name) VALUES (?1, ?2)
";

pub const LOAD_API_GROUPS_SQL: &str = "
    SELECT id, name FROM api_groups ORDER BY name
";

pub const DELETE_API_GROUP_SQL: &str = "
    DELETE FROM api_groups WHERE id = ?1
";

// ── Mocked Routes ─────────────────────────────────────────────────────────────

// ?1 = id (uuid4), ?2 = group_id, ?3 = method, ?4 = path,
// ?5 = status_code, ?6 = response_type, ?7 = response_path
pub const ADD_ROUTE_TO_GROUP_SQL: &str = "
    INSERT INTO mocked_routes (id, group_id, method, path, status_code, response_type, response_path)
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
";

pub const LOAD_ALL_ROUTES_SQL: &str = "
    SELECT id, group_id, method, path, status_code, response_type, response_path
    FROM mocked_routes
    ORDER BY group_id, method, path
";

pub const GET_MOCKED_ROUTE_SQL: &str = "
    SELECT id, group_id, method, path, status_code, response_type, response_path
    FROM mocked_routes WHERE id = ?1
";

// ?1 = method, ?2 = path, ?3 = status_code, ?4 = response_type,
// ?5 = response_path, ?6 = id
pub const UPDATE_MOCKED_ROUTE_SQL: &str = "
    UPDATE mocked_routes
    SET method = ?1, path = ?2, status_code = ?3, response_type = ?4, response_path = ?5
    WHERE id = ?6
";

pub const DELETE_ROUTE_FROM_GROUP_SQL: &str = "
    DELETE FROM mocked_routes WHERE id = ?1
";

// ── Traffic Logs ──────────────────────────────────────────────────────────────

pub const GET_TRAFFIC_LOGS_SQL: &str = "
    SELECT id, route_id, timestamp, method, url_hit, latency_ms, response_status
    FROM traffic_logs
    ORDER BY id DESC
";

pub const GET_TRAFFIC_LOGS_FOR_ROUTE_SQL: &str = "
    SELECT id, route_id, timestamp, method, url_hit, latency_ms, response_status
    FROM traffic_logs WHERE route_id = ?1
    ORDER BY id DESC
";

pub const CLEAR_TRAFFIC_LOGS_SQL: &str = "
    DELETE FROM traffic_logs
";

pub const CLEAR_TRAFFIC_LOGS_FOR_ROUTE_SQL: &str = "
    DELETE FROM traffic_logs WHERE route_id = ?1
";

// ── Export / Import (.pls) ────────────────────────────────────────────────────

pub const EXPORT_GROUPS_SQL: &str = "
    SELECT id, name FROM api_groups ORDER BY name
";

pub const EXPORT_ROUTES_SQL: &str = "
    SELECT id, group_id, method, path, status_code, response_type, response_path
    FROM mocked_routes
    ORDER BY group_id, method, path
";

pub const CLEAN_IMPORT_ROUTES_SQL: &str = "
    DELETE FROM mocked_routes
";

pub const CLEAN_IMPORT_GROUPS_SQL: &str = "
    DELETE FROM api_groups
";
