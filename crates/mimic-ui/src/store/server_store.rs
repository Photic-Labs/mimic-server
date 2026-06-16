// ─────────────────────────────────────────────────────────────────────────────
// SERVER STATUS
// What the UI reads to decide what to render in the top bar
// ─────────────────────────────────────────────────────────────────────────────

use mimic_server::{route_table_helper::get_routing_table, server::start_server, ServerHandle};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum ServerStatus {
    #[default]
    Stopped,
    Starting,
    Running {
        port: u16,
    },
    Error(String),
}

// ─────────────────────────────────────────────────────────────────────────────
// SERVER STORE
// Owned by AppStore — one instance for the lifetime of the app
// ─────────────────────────────────────────────────────────────────────────────

pub struct ServerStore {
    /// The live handle — None when server is stopped
    handle: Option<ServerHandle>,
    /// What the UI reads
    pub status: ServerStatus,
    /// Absolute path to mimic.db — passed to host_controller
    db_path: String,
}

impl ServerStore {
    pub fn new(db_path: impl Into<String>) -> Self {
        Self {
            handle: None,
            status: ServerStatus::Stopped,
            db_path: db_path.into(),
        }
    }

    // ── Is the server currently running? ─────────────────────────────────────
    pub fn is_running(&self) -> bool {
        matches!(self.status, ServerStatus::Running { .. })
    }

    // ── Is the server starting? ───────────────────────────────────────────────
    pub fn is_starting(&self) -> bool {
        matches!(self.status, ServerStatus::Starting)
    }
}

impl ServerStore {
    /// Called when user clicks "Start Server"
    /// Marks status as Starting immediately so the UI can disable the button
    /// Returns the future — caller must .await it inside a tokio context
    pub async fn start(&mut self) {
        // Guard — don't double-start
        if self.is_running() || self.is_starting() {
            return;
        }

        self.status = ServerStatus::Starting;

        match start_server(&self.db_path).await {
            Ok(handle) => {
                let port = handle.port;
                self.handle = Some(handle);
                self.status = ServerStatus::Running { port };
                println!("[server_store] Server started on port {port}");
            }
            Err(e) => {
                self.status = ServerStatus::Error(e.to_string());
                eprintln!("[server_store] Start failed: {e}");
            }
        }
    }

    /// Called when user clicks "Stop Server"
    pub fn stop(&mut self) {
        if let Some(mut handle) = self.handle.take() {
            handle.stop();
            self.status = ServerStatus::Stopped;
            println!("[server_store] Server stopped");
        }
    }

    /// Called after user saves or deletes a route
    /// Reloads routing table without restarting the server
    pub fn reload_routes(&self, conn: &rusqlite::Connection) {
        if let Some(handle) = &self.handle {
            match get_routing_table(conn) {
                Ok(table) => {
                    handle.reload(table);
                    println!("[server_store] Routes reloaded");
                }
                Err(e) => {
                    eprintln!("[server_store] Route reload failed: {e}");
                }
            }
        }
    }

    // /// Current port if running
    // pub fn port(&self) -> Option<u16> {
    //     match &self.status {
    //         ServerStatus::Running { port } => Some(*port),
    //         _ => None,
    //     }
    // }

    // /// Current error message if in error state
    // pub fn error_message(&self) -> Option<&str> {
    //     match &self.status {
    //         ServerStatus::Error(msg) => Some(msg.as_str()),
    //         _ => None,
    //     }
    // }
}
