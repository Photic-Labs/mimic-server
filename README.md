# MimicServer | v1.0

> A [PhoticLabs](https://github.com/photiclabs) product.

**MimicServer** is a local-first API mocking engine and gateway for developers.  
Define mock routes, serve JSON responses instantly, and inspect live traffic —  
all from a fast, native desktop app with zero cloud dependency.

---

## Why MimicServer

| Problem | MimicServer's Answer |
|---|---|
| Mock platforms require internet | Runs 100% locally — no account, no cloud |
| Electron apps are heavy and slow | Native Rust binary — `< 30MB`, instant startup |
| JSON config files break silently | SQLite — relational, transactional, crash-safe |
| Server restarts drop in-flight requests | Hot reload via `Arc<RwLock<>>` — zero downtime |
| Traffic is invisible during development | Built-in traffic log — every request recorded |

---

## Features

- **Mock any HTTP endpoint** — define method, path, status code, and a `.json` response file
- **Instant hot reload** — save a route and it is live immediately, no server restart
- **Traffic log** — see every request: method, status, URL, latency, timestamp
- **Port configuration** — run on any port via the settings panel
- **Health endpoint** — `GET /health` always available to confirm the server is alive
- **Cross-platform** — single binary for Windows, macOS, and Linux
- **Branded UI** — dark, professional developer-tool aesthetic built on the PhoticLabs design system

---

## Tech Stack

| Layer | Choice |
|---|---|
| UI | `egui` + `eframe` |
| Design System | `pl-components` (PhotoicLabs internal crate) |
| Async Runtime | Tokio |
| HTTP | Axum + Hyper |
| Database | SQLite via `rusqlite` |
| Payload Storage | Raw `.json` files on disk |

---

## Out of Scope (by design)

- No path parameters (`/api/user/:id`) — ignored
- No request header inspection — ignored
- No response templating — static `.json` files only
- No authentication on the mock server
- No cloud sync — local-first, always


---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) 1.75 or later
- Cargo (included with Rust)

### Build and Run

```bash
# Clone the repository
git clone https://github.com/photiclabs/mimic-server.git
cd mimic-server

# Run in development mode
cargo run

# Build release binary
cargo build --release
```

The app opens a native window.  
The database (`mimic.db`) is created automatically on first run.

### First Use

1. Open the app
2. Click **▶ Start Server** in the top bar
3. The server binds to `localhost:8080` by default
4. Add a route in the sidebar — set method, path, status code, and a `.json` file
5. Hit the endpoint from your app or `curl`
6. Watch the request appear in the Traffic Log panel

```bash
# Confirm the server is alive
curl http://localhost:8080/~health

# Hit a configured mock route
curl http://localhost:8080/api/your-route
```

---

## Configuration

Port is stored in the `app_config` SQLite table.  
Default: `8080`.  
Change it in the Settings panel — takes effect on next server start.

---

## Response Files

Each route points to a `.json` file on disk.  
The file is read fresh on every request — edit it externally and the next request picks up the change immediately.

```json
{
  "id": 1,
  "name": "John Doe",
  "email": "johndoe@annonymus.com"
}
```

If the file is missing or contains invalid JSON, MimicServer responds `500` with a descriptive error body instead of crashing.

---

## HTTP Response Headers

Every matched route returns these headers:

```
Content-Type:         application/json
X-Mimic-Route-Id:    <route_id>
X-Mimic-Latency-Ms:  <latency>
X-Powered-By:        MimicServer
```

---

## Error Responses

| Scenario | Status | Error Code |
|---|---|---|
| Route matched, `.json` file missing | `500` | `response_file_not_found` |
| Route matched, file is invalid JSON | `500` | `invalid_json_file` |
| Route matched, no file configured | `501` | `no_response_file_configured` |
| No matching route | `404` | `route_not_found` |


---

## License

MIT © [PhotoicLabs](https://github.com/photiclabs)
