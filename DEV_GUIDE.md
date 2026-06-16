# MimicServer — Developer Guide

> For contributors and maintainers.  
> Org: [PhoticLabs](https://github.com/photiclabs)

---

## Architecture — How a Request Flows

```
Incoming HTTP request
        │
        ▼
Axum listener (bound in host_controller::start)
        │
        ├── GET /~health → health_handler → 200 OK
        │
        └── everything else → mock_handler (fallback)
                │
                ├── normalize method + path
                │     method.trim().to_uppercase()
                │     path.trim().trim_end_matches('/')
                │
                ├── acquire read lock on SharedRoutingTable
                │     HashMap.get((method, path))
                │   release lock immediately
                │
                ├── HIT → read .json file from disk
                │         validate JSON
                │         respond with configured status code
                │
                ├── MISS → respond 404
                │
                └── send TrafficLog to mpsc channel (fire-and-forget)
                          │
                          ▼
                    log writer task (separate SQLite connection)
                    writes to traffic_logs table
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

# Run the app in dev mode (debug assertions on, console logging active)
cargo run

# Run with backtrace on panic
RUST_BACKTRACE=1 cargo run

# Build release binary
cargo build --release

# Check a single crate
cargo check -p mimic-core
cargo check -p mimic-ui
cargo check -p pl-components
```

---

## Database

MimicServer uses SQLite via `rusqlite`. The database file is `mimic.db` in the working directory.

### Schema

```sql
-- Server configuration
CREATE TABLE IF NOT EXISTS app_config (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- Seed default port
INSERT OR IGNORE INTO app_config VALUES ('port', '8080');

-- Mock routes
CREATE TABLE IF NOT EXISTS mocked_routes (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    group_name    TEXT    NOT NULL,
    method        TEXT    NOT NULL,       -- uppercase: GET, POST, PUT, DELETE, PATCH
    path          TEXT    NOT NULL,       -- leading slash: /api/auth
    status_code   INTEGER NOT NULL DEFAULT 200,
    response_path TEXT,                   -- absolute path to .json file, nullable
    created_at    INTEGER NOT NULL DEFAULT (unixepoch())
);

-- Traffic logs
CREATE TABLE IF NOT EXISTS traffic_logs (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    route_id        INTEGER,              -- NULL if no route matched (404)
    timestamp       INTEGER NOT NULL,     -- Unix seconds (chrono::Utc::now())
    method          TEXT    NOT NULL,
    url_hit         TEXT    NOT NULL,
    latency_ms      INTEGER NOT NULL,
    response_status INTEGER NOT NULL,
    FOREIGN KEY (route_id) REFERENCES mocked_routes(id) ON DELETE SET NULL
);
```

### Inspecting the database manually

```bash
sqlite3 mimic.db

# List all routes
SELECT id, method, path, status_code, response_path FROM mocked_routes;

# List recent traffic
SELECT method, url_hit, response_status, latency_ms FROM traffic_logs ORDER BY id DESC LIMIT 20;

# Check config
SELECT * FROM app_config;
```

---


## Core Crate — `mimic-core`

### `types.rs`

All shared types. No logic.

| Type | Description |
|---|---|
| `RouteEntry` | One compiled route: `route_id`, `status_code`, `response_path` |
| `RoutingTable` | `HashMap<(String, String), RouteEntry>` — key is `(METHOD, /path)` |
| `SharedRoutingTable` | `Arc<RwLock<RoutingTable>>` — shared between handler and UI |
| `TrafficLog` | One log entry per HTTP request |
| `ServerHandle` | Returned to UI: `port`, `routing_table`, `shutdown_tx` |
| `ServerError` | `PortInUse(u16)` · `DbError(rusqlite::Error)` · `BindError(String)` |

### `compiler.rs`

```rust
pub fn compile_routing_table(conn: &Connection) -> Result<RoutingTable, rusqlite::Error>
pub fn normalize_key(method: &str, path: &str) -> (String, String)
```

Reads all rows from `mocked_routes`, normalizes each key, inserts into `HashMap`.  
Called on server start and after every route save or delete.

### `mock_handler.rs`

Axum `State` extractor receives `HandlerState { routing_table, log_sender }`.  
Lock is held **only** for the `HashMap` lookup — never during file I/O.  
All error cases return a typed JSON body — the server never panics on bad input.

### `host_controller.rs`

```rust
pub async fn start(db_path: &str) -> Result<ServerHandle, ServerError>
```

Steps in order:
1. Open SQLite connection — read port from `app_config`
2. Compile routing table
3. Create `mpsc::unbounded_channel` for traffic logs
4. Build `HandlerState`
5. Build Axum router — `/~health` + fallback
6. **Bind TCP listener before spawning** — surfaces `PortInUse` immediately
7. Create `oneshot` shutdown channel
8. Spawn Axum server task with graceful shutdown
9. Spawn traffic log writer task (its own SQLite connection)
10. Return `ServerHandle`

---

## UI Crate — `mimic-ui`

### Tokio + eframe Integration

`eframe::run_native` blocks the thread. It must run inside the Tokio runtime but cannot call `Runtime::new()` again (nested runtimes panic).

**Pattern used throughout the project:**

```rust
// main.rs — runtime is created once here
#[tokio::main]
async fn main() {
    tokio::task::block_in_place(|| {
        eframe::run_native(
            "Mimic Server",
            eframe::NativeOptions::default(),
            Box::new(|cc| Ok(Box::new(MimicApp::new(cc)))),
        )
        .unwrap();
    });
}

// app.rs — calling async from egui update()
tokio::task::block_in_place(|| {
    tokio::runtime::Handle::current().block_on(async {
        self.store.server.start().await;
    })
});
```

**Never do this — it panics:**
```rust
// WRONG
let rt = tokio::runtime::Runtime::new().unwrap();
rt.block_on(self.store.server.start()); // panics — nested runtime
```

### Signal Pattern in `update()`

egui's `update()` holds a mutable borrow on `self` for the entire frame.  
You cannot call `self.store.server.start()` inside a panel closure that also borrows `self`.

**Correct pattern:**

```rust
fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // 1. Collect signals as plain bools during draw
    let mut start_clicked = false;
    let mut stop_clicked  = false;

    egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
        self.top_bar.show(ui, &self.store.server.status, &mut start_clicked, &mut stop_clicked);
    });

    // 2. Act on signals AFTER all panels are drawn
    if start_clicked {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.store.server.start().await;
            })
        });
        ctx.request_repaint();
    }

    if stop_clicked {
        self.store.server.stop();
        ctx.request_repaint();
    }
}
```

### `ServerStatus` State Machine

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

### Vertical Centering in egui

`set_min_height` does not center children — it only sets the container size.  
Correct pattern for a fixed-height top bar:

```rust
// 1. Allocate the exact rect — gives you known min/max Y
let (bar_rect, _) = ui.allocate_exact_size(
    egui::vec2(ui.available_width(), BAR_HEIGHT),
    egui::Sense::hover(),
);

// 2. Create child UI with explicit max_rect + Align::Center
let mut child = ui.new_child(
    egui::UiBuilder::new()
        .max_rect(bar_rect)
        .layout(egui::Layout::left_to_right(egui::Align::Center)),
);

// 3. Draw inside child — content is vertically centered against bar_rect
child.label("This is centered");
```

---

## Design System — `pl-components`

### Brand Identity

| Token | Value | Usage |
|---|---|---|
| `BRAND_BLUE` | `#2563FF` | Primary actions, accent, wordmark |
| `LIGHT_BLUE` | `#CCD4FF` | Hover states, highlights |
| `PURPLE` | `#7C3AED` | Secondary accent |
| `DARK_NAVY` | `#00117` | Deep backgrounds |
| `OFF_WHITE` | `#F5F7FA` | Primary text |
| `BG_MAIN` | `#0D0E11` | Main window background |
| `BG_PANEL` | `#111217` | Panel backgrounds |
| `BG_ELEVATED` | `#181920` | Cards, inputs, badges |
| `BORDER` | `#262834` | Strokes, dividers |
| `SUCCESS` | `#22C55E` | Running status, 2xx responses |
| `DANGER` | `#EF4444` | Errors, 5xx responses, Stop button |

### Typography

- **Font:** Manrope (Regular + Bold)
- **Loaded via:** `egui` font data in `app.rs` `CreationContext`
- **Font family names:** `"Manrope"` (regular) · `"Bold"` (bold weight)

### Logo Tiers

| Tier | Usage |
|---|---|
| Tier 1 — Marketing Logo | Website hero, pitch decks, marketing materials |
| Tier 2 — Enterprise Logo | Product interfaces, CLI, corporate stationery |
| Tier 3 — System Primitive | App icon (16×16 and 32×32), top bar brand icon |

The **top bar brand icon** uses the Tier 3 System Primitive:  
32×32 rounded blue square (`#2563FF`, radius 7) with a centered `🌐` glyph in `FontFamily::Monospace`.

---

## Adding a New Route (data flow)

1. User fills in method, path, status code, response file path in the sidebar panel
2. UI calls `app_store.save_route(...)` → `INSERT INTO mocked_routes`
3. UI calls `server_store.reload_routes(&conn)` → `compile_routing_table` → `handle.reload(table)`
4. `handle.reload()` acquires write lock on `SharedRoutingTable`, swaps the HashMap, releases lock
5. Next incoming request sees the new route immediately — no restart

---

## Adding a New Panel

1. Create `crates/mimic-ui/src/panels/your_panel.rs`
2. Implement a `pub struct YourPanel` with `pub fn show(&mut self, ui: &mut egui::Ui, ...)`
3. Add `pub mod your_panel;` to `panels/mod.rs`
4. Add `your_panel: YourPanel` field to `MimicApp`
5. Call `self.your_panel.show(...)` inside the appropriate `egui::*Panel` in `app.rs`

---

## Adding a New Component to `pl-components`

1. Create `crates/pl-components/src/components/ms_your_component.rs`
2. Follow the existing `MSButton` pattern — accept `egui::Ui`, return `egui::Response`
3. Use `Color::*` and `theme::*` constants — never hardcode hex values
4. Re-export from `crates/pl-components/src/lib.rs`

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

## Gitignore Recommendations

```gitignore
# Build artifacts
/target

# Database — local state, not committed
mimic.db

# Response payload files — developer-local
/responses/

# OS artifacts
.DS_Store
Thumbs.db
```

---

## Common Errors

### `Cannot start a runtime from within a runtime`

**Cause:** Calling `Runtime::new().block_on()` inside `eframe::run_native` which is already inside `#[tokio::main]`.

**Fix:**
```rust
tokio::task::block_in_place(|| {
    tokio::runtime::Handle::current().block_on(async {
        self.store.server.start().await;
    })
});
```

---

### `Address already in use (os error 98)`

**Cause:** Another process is already on the configured port.

**Fix:** Change the port in the Settings panel, or kill the other process:
```bash
# macOS / Linux
lsof -i :8080
kill -9 <PID>

# Windows
netstat -ano | findstr :8080
taskkill /PID <PID> /F
```

---

### `rusqlite::Error: no such table`

**Cause:** Database schema not initialized on first run.

**Fix:** Ensure `db::init(&conn)` is called in `AppStore::new()` before any queries.

---

### Top bar content not vertically centered

**Cause:** Using `set_min_height` or `ui.horizontal()` without an explicit `max_rect`.

**Fix:** Use `allocate_exact_size` + `ui.new_child()` with `Layout::Align::Center`. See the Vertical Centering section above.

---

## Contributing

1. Fork the repository on [GitHub](https://github.com/photiclabs)
2. Create a branch: `git checkout -b feature/your-feature`
3. Run `cargo check --workspace` before committing
4. Run `cargo clippy --workspace` and resolve all warnings
5. Open a pull request against `main`

---

*MimicServer is a [PhoticLabs](https://github.com/photiclabs) product.*  
*Built with Rust. Designed with intention.*
