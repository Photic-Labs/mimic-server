# MimicServer — Developer Guide

> For contributors and maintainers.
> Org: [PhoticLabs](https://github.com/Photic-Labs)

---

## Workspace Structure

The project is a Cargo workspace with four crates:

```
mimic-server/
├── Cargo.toml                  # workspace root
├── crates/
│   ├── mimic-core/             # data layer — models, DB, config
│   ├── mimic-server/           # HTTP layer — Axum mock server
│   ├── mimic-ui/               # Desktop GUI (egui/eframe) — binary entrypoint
│   └── pl-components/          # Design system — shared widgets
├── examples/                   # example response payloads + test index.html
├── DEV_GUIDE.md
└── README.md
```

---

## Architecture — How a Request Flows

```
Incoming HTTP request
        │
        ▼
Axum listener (crates/mimic-server/src/server.rs::start_server)
        │
        ├── GET /health → health_handler (hardcoded, always 200)
        │
        └── everything else → mock_routes_handler (fallback)
                │
                ├── extract method + path from request
                ├── strip global_prefix if configured
                │     (e.g. /api/v1/users → /users)
                ├── normalize: method.trim().to_uppercase(),
                │     path.trim_end_matches('/')
                │
                ├── acquire read lock on SharedRoutingTable
                │     HashMap.get((METHOD, /path))
                │     release lock immediately
                │
                ├── Phase 1 — exact match (O(1) HashMap lookup)
                │   ├── HIT → read .json file from disk
                │   │         validate JSON
                │   │         respond with configured status code
                │   │
                │   └── MISS → Phase 2 — pattern match
                │         iterate table, split stored path by '/',
                │         match segments — ':' prefix matches any value
                │         (e.g. /api/user/:id matches /api/user/42)
                │
                ├── HIT (any phase) → build response with headers:
                │     Content-Type: application/json
                │     X-Mimic-Route-Id: <uuid>
                │     X-Mimic-Latency-Ms: <ms>
                │     X-Powered-By: MimicServer
                │
                ├── MISS → respond 404 with JSON error body
                │
                └── send MockAPICallerLog to mpsc channel (fire-and-forget)
                          │
                          ▼
                    log writer task (separate SQLite connection,
                    spawned in start_server, drains the channel
                    and INSERTs into traffic_logs table)
```

---

## Crate Breakdown

### `mimic-core` — Data Layer

| Path | Description |
|---|---|
| `src/config.rs` | Platform data directories, DB path, payloads dir, `ensure_dirs()` |
| `src/models.rs` | Shared types: `AppConfig`, `ApiGroup`, `MockedApiRoute`, `TrafficLog`, `RouteTag` |
| `src/db/connector.rs` | Opens SQLite connection, runs bootstrap + seed |
| `src/db/queries.rs` | All raw SQL strings in one place |
| `src/db/app_config.rs` | Load/save `app_config` table (port, host, prefix, theme) |
| `src/db/api_groups.rs` | CRUD for `api_groups` table |
| `src/db/mocked_routes.rs` | CRUD for `mocked_routes` table, grouped by `group_id` |
| `src/db/traffice_logs.rs` | Load/clear logs for a specific route |

No async — all synchronous SQLite calls. The `mimic-server` crate wraps these with Tokio where needed.

### `mimic-server` — HTTP Mock Server

| Path | Description |
|---|---|
| `src/server.rs` | `start_server()` — builds Axum router, binds TCP, spawns server + log writer tasks |
| `src/route_handlers.rs` | `health_handler()` and `mock_routes_handler()` — the fallback handler |
| `src/route_table_helper.rs` | `get_routing_table()` — reads all routes from SQLite, compiles into a `RoutingTable` (HashMap) |
| `src/helpers.rs` | Route normalization, two-phase match (exact + pattern), error body builder, traffic log writer, port/prefix readers |
| `src/queries.rs` | SQL strings used by the server |
| `src/types.rs` | `RouteEntry`, `RoutingTable`, `SharedRoutingTable`, `ServerHandle`, `ServerHandlerState`, `MockAPICallerLog`, `ServerError` |

### `mimic-ui` — Desktop GUI (binary entrypoint)

| Path | Description |
|---|---|
| `src/main.rs` | `#[tokio::main]` — sets up tracing, opens DB, registers fonts, runs eframe |
| `src/app.rs` | `MimicServerApp` — implements `eframe::App`, owns all panels + stores |
| `src/constants.rs` | Layout/sizing constants |
| `src/panels/top_panel.rs` | Top bar — brand icon, title, Start/Stop/Retry buttons, port badge, settings button |
| `src/panels/sidebar_panel.rs` | Left sidebar — API groups list, route tree, search, "Add Group" |
| `src/panels/details_panel.rs` | Center — route editor (method, path, status, response file, save/delete) |
| `src/panels/traffic_panel.rs` | Bottom — traffic log table for the selected route |
| `src/panels/helpers/app_helpers.rs` | Method tag colors, status colors, latency colors, timestamp formatting |
| `src/panels/helpers/details_helpers.rs` | Path validation, HTTP method/status code lists |
| `src/store/app_store.rs` | `AppStore` — aggregates all sub-stores, central refresh |
| `src/store/server_store.rs` | `ServerStore` — start/stop/reload, `ServerStatus` state machine |
| `src/store/settings_store.rs` | `SettingsStore` — theme, port, prefix with validation |
| `src/store/api_group_store.rs` | `ApiGroupStore` — loads groups from DB, create group |
| `src/store/mocked_route_store.rs` | `MockedRouteStore` — loads routes grouped by group_id, add/update/delete |
| `src/store/details_store.rs` | `DetailsStore` — currently selected route, its logs |
| `src/store/store_traits.rs` | `Loadable` trait — `load_from_db()` + `refresh_data()` |
| `src/modals/settings_modal.rs` | Settings window — port, prefix, theme, reset |

### `pl-components` — Design System

| Path | Description |
|---|---|
| `src/globals.rs` | `Color` tokens, font sizes, spacing, radii, padding constants |
| `src/theme.rs` | `AppTheme` enum — `to_visuals()` for dark/light |
| `src/components/pl_button.rs` | `PLButton` — primary, danger, subtle, ghost, purple variants |
| `src/components/pl_input.rs` | `PLTextInput` — styled text input with label, hint, icon |
| `src/components/pl_label.rs` | `PLLabel` — heading, body, field, error, success, accent, empty |
| `src/components/pl_badge.rs` | `PLBadge` — method badges, count badges |
| `src/components/pl_frame.rs` | `PLCard` — framed card container |
| `src/components/pl_section_label.rs` | `PLSectionLabel` — section headers |
| `src/panels/panel_header.rs` | `PLPanelHeader` — modal panel headers with title/subtitle/actions |
| `src/panels/panel_layout_row.rs` | `PLPanelRow` — label + content rows for settings |

---

## Database

MimicServer uses SQLite via `rusqlite`. The database file is `mimic_server_v1.db` in the platform data directory:

- **macOS:** `~/Library/Application Support/MimicServer/mimic_server_v1.db`
- **Linux:** `~/.local/share/MimicServer/mimic_server_v1.db`
- **Windows:** `C:\Users\<USER>\AppData\Roaming\MimicServer\mimic_server_v1.db`

### Schema

```sql
-- Server configuration (single-row table, id always 1)
CREATE TABLE IF NOT EXISTS app_config (
    id      INTEGER PRIMARY KEY CHECK (id = 1),
    host    TEXT    NOT NULL DEFAULT '127.0.0.1',
    port    INTEGER NOT NULL DEFAULT 8080,
    theme   TEXT    NOT NULL DEFAULT 'dark',
    prefix  TEXT    NOT NULL DEFAULT ''
);

-- API groups (logical folders for organizing routes)
CREATE TABLE IF NOT EXISTS api_groups (
    id      TEXT PRIMARY KEY,          -- UUID v4
    name    TEXT NOT NULL UNIQUE
);

-- Mocked routes, each belongs to a group
CREATE TABLE IF NOT EXISTS mocked_routes (
    id            TEXT    PRIMARY KEY, -- UUID v4
    group_id      TEXT    NOT NULL,
    method        TEXT    NOT NULL,    -- GET, POST, PUT, PATCH, DELETE
    path          TEXT    NOT NULL,    -- /api/resource/:id (':' prefix = param)
    status_code   INTEGER NOT NULL,
    response_type TEXT    NOT NULL,    -- "file", "inline", "passthrough"
    response_path TEXT,                -- absolute path to .json file
    FOREIGN KEY (group_id) REFERENCES api_groups(id) ON DELETE CASCADE,
    UNIQUE(method, path)
);

-- Route tags (for future filtering/tagging)
CREATE TABLE IF NOT EXISTS route_tags (
    route_id  TEXT NOT NULL,
    tag_name  TEXT NOT NULL,
    PRIMARY KEY (route_id, tag_name),
    FOREIGN KEY (route_id) REFERENCES mocked_routes(id) ON DELETE CASCADE
);

-- Traffic logs for every request the mock server handles
CREATE TABLE IF NOT EXISTS traffic_logs (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    route_id        TEXT,                              -- NULL if no route matched (404)
    timestamp       INTEGER DEFAULT (strftime('%s', 'now')),
    method          TEXT    NOT NULL,
    url_hit         TEXT    NOT NULL,                   -- original URL that was hit
    latency_ms      INTEGER,
    response_status INTEGER,
    FOREIGN KEY (route_id) REFERENCES mocked_routes(id) ON DELETE SET NULL
);
```

### Seed data

```sql
INSERT OR IGNORE INTO app_config (id, host, port, prefix, theme)
VALUES (1, '127.0.0.1', 8080, '', 'dark');
```

### Inspecting the database manually

```bash
sqlite3 ~/Library/Application\ Support/MimicServer/mimic_server_v1.db

# List all groups
SELECT * FROM api_groups;

# List all routes
SELECT id, method, path, status_code, response_path FROM mocked_routes;

# List recent traffic
SELECT method, url_hit, response_status, latency_ms FROM traffic_logs ORDER BY id DESC LIMIT 20;

# Check config
SELECT * FROM app_config;
```

---

## Routing — Two-Phase Lookup

The mock handler in `helpers.rs` uses a two-phase approach:

**Phase 1 — Exact match:** O(1) HashMap lookup by `(METHOD, /path)`. Catches all static routes.

**Phase 2 — Pattern match:** Iterates the routing table looking for stored paths containing `:` segments. A stored segment starting with `:` (e.g. `:id`) matches any incoming segment in that position. Segment count must match exactly.

Example:
- Stored: `GET /api/user/:id`
- Incoming: `GET /api/user/42` → match ✓
- Incoming: `GET /api/user/42/profile` → no match (3 vs 4 segments)

---

## Server State Machine

```
Stopped
   │
   │ user clicks Start
   ▼
Starting          ← button disabled, shows ⏳
   │
   ├── Ok(handle) → Running { port }   ← green pill, Stop button
   │
   └── Err(e)     → Error(msg)         ← red pill, Retry button
                        │
                        │ user clicks Retry
                        ▼
                     Starting
```

Managed by `ServerStore` in `crates/mimic-ui/src/store/server_store.rs`.

---

## UI Patterns

### Tokio + eframe Integration

`eframe::run_native` blocks the thread. It must run inside the Tokio runtime but cannot create a nested runtime.

```rust
// main.rs — runtime is created once
#[tokio::main]
async fn main() {
    // ... setup ...
    tokio::task::block_in_place(|| {
        eframe::run_native("Mimic Server", ..., Box::new(|cc| Ok(Box::new(App::new(cc)))))
            .unwrap();
    });
}

// app.rs — calling async from egui update()
tokio::task::block_in_place(|| {
    tokio::runtime::Handle::current().block_on(async {
        self.app_store.server_store.start().await;
    })
});
```

**Never create a new runtime inside eframe** — it panics with "Cannot start a runtime from within a runtime".

### Signal Pattern in `update()`

egui's `update()` holds a mutable borrow on `self` for the entire frame. Collect signals as bools during draw, then act on them after:

```rust
fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    let mut start_clicked = false;

    egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
        self.top_bar.show(ui, &self.store.server.status, &mut start_clicked, ...);
    });

    // Act AFTER all panels
    if start_clicked {
        tokio::task::block_in_place(|| { ... });
        ctx.request_repaint();
    }
}
```

### Vertical Centering in egui

Use `allocate_exact_size` + `UiBuilder` with `Align::Center`:

```rust
let (bar_rect, _) = ui.allocate_exact_size(
    egui::vec2(ui.available_width(), BAR_HEIGHT),
    egui::Sense::hover(),
);
let mut child = ui.new_child(
    egui::UiBuilder::new()
        .max_rect(bar_rect)
        .layout(egui::Layout::left_to_right(egui::Align::Center)),
);
child.label("Centered content");
```

---

## Prerequisites

| Tool | Version | Install |
|---|---|---|
| Rust | 1.75+ | https://rustup.rs |
| cargo | Included with Rust | — |
| SQLite | Any | Pre-installed on macOS/Linux. Windows: https://sqlite.org |

No other tooling required. No Node.js. No Python. No Docker.

---

## Development Workflow

```bash
# Check all crates compile
cargo check --workspace

# Run the app in dev mode
cargo run

# Run with backtrace on panic
RUST_BACKTRACE=1 cargo run

# Build release binary
cargo build --release

# Check a single crate
cargo check -p mimic-core
cargo check -p mimic-server
cargo check -p mimic-ui
cargo check -p pl-components

# Lint
cargo clippy --workspace
```

---

## Adding a New Route (data flow)

1. User fills in method, path, status code, response file in the details panel
2. `DetailsPanel::handle_save()` → `MockedRouteStore::update_mocked_route()` → `UPDATE mocked_routes`
3. `AppStore::refresh_store()` called → calls `ServerStore::reload_routes()`
4. `reload_routes()` reads all routes from DB → `get_routing_table()` → `handle.reload(table)`
5. `ServerHandle::reload()` acquires write lock on `SharedRoutingTable`, swaps the HashMap, releases lock
6. Next incoming request sees the new route immediately — no restart

---

## Adding a New Panel

1. Create `crates/mimic-ui/src/panels/your_panel.rs`
2. Implement a `pub struct YourPanel` with `pub fn show(&mut self, ui: &mut egui::Ui, conn, app_store, ...)`
3. Add `pub mod your_panel;` to `panels/mod.rs`
4. Add `your_panel: YourPanel` field to `MimicServerApp` in `app.rs`
5. Call `self.your_panel.show(...)` inside the appropriate `egui::*Panel` in `app.rs`

---

## Adding a New Component to `pl-components`

1. Create `crates/pl-components/src/components/pl_your_component.rs`
2. Follow the existing `PLButton` pattern — accept `egui::Ui`, return `egui::Response`
3. Use `Color::*` and spacing constants — never hardcode values
4. Re-export from `crates/pl-components/src/lib.rs`

---

## Adding a New DB Table

1. Add the `CREATE TABLE` SQL to `crates/mimic-core/src/db/queries.rs`
2. Add the bootstrap call in `crates/mimic-core/src/db/connector.rs::bootstrap_database()`
3. Create a new module `crates/mimic-core/src/db/your_table.rs` with row mapping + CRUD functions
4. Add `pub mod your_table;` to `crates/mimic-core/src/db/mod.rs`
5. Add the corresponding struct to `crates/mimic-core/src/models.rs`

---

## Build Targets

```bash
# macOS — Apple Silicon
cargo build --release --target aarch64-apple-darwin

# macOS — Intel
cargo build --release --target x86_64-apple-darwin

# Windows
cargo build --release --target x86_64-pc-windows-msvc

# Linux
cargo build --release --target x86_64-unknown-linux-gnu
```

Target binary size: `< 20MB` stripped release build.

---

## Responding to User Queries (Implementation Plan)

When implementing a new user-facing feature, follow this checklist:

1. **Data layer** (`mimic-core`):
   - Add/modify struct in `models.rs`
   - Add SQL queries in `db/queries.rs`
   - Implement CRUD in a `db/` module
   - Register table creation in `db/connector.rs` if new

2. **Server layer** (`mimic-server`):
   - Add new route handler in `route_handlers.rs` if needed
   - Register the route in `server.rs`

3. **Store layer** (`mimic-ui/src/store/`):
   - Create or extend a store struct implementing `Loadable`
   - Wire into `AppStore`

4. **UI layer** (`mimic-ui/src/panels/`):
   - Create/reuse a panel
   - Add to `MimicServerApp` in `app.rs`

5. **Components** (`pl-components`):
   - Add new widgets to `components/` if existing ones don't suffice

---

## Common Errors

### `Cannot start a runtime from within a runtime`

**Cause:** Calling `Runtime::new().block_on()` inside `eframe::run_native`.

**Fix:**
```rust
tokio::task::block_in_place(|| {
    tokio::runtime::Handle::current().block_on(async {
        self.store.server.start().await;
    })
});
```

### `Address already in use (os error 98)`

**Cause:** Another process is on the configured port.

```bash
lsof -i :8080
kill -9 <PID>
```

### Top bar content not vertically centered

**Cause:** Using `set_min_height` without explicit `max_rect`.

**Fix:** Use `allocate_exact_size` + `ui.new_child()` with `Layout::Align::Center`.

### `no such table`

**Cause:** Schema not initialized. Ensure `db::connector::open_connection()` runs before any queries.

---

## Contributing

1. Fork the repository on [GitHub](https://github.com/Photic-Labs)
2. Create a branch: `git checkout -b feature/your-feature`
3. Run `cargo check --workspace` before committing
4. Run `cargo clippy --workspace` and resolve all warnings
5. Open a pull request against `main`

---

*MimicServer is a [PhoticLabs](https://github.com/Photic-Labs) product.*
*Built with Rust. Designed with intention.*
