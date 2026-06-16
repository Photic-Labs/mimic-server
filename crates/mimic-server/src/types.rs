use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use tokio::sync::{mpsc, oneshot};

#[derive(Debug, Clone)]
pub struct RouteEntry {
    pub route_id: String,
    pub status: u16,
    pub response_path: Option<String>,
}

pub type RoutingTable = HashMap<(String, String), RouteEntry>;

pub type SharedRoutingTable = Arc<RwLock<RoutingTable>>;

pub struct ServerHandle {
    pub port: u16,
    pub shutdown_channel: Option<oneshot::Sender<()>>,
    pub routing_table: SharedRoutingTable,
}

impl ServerHandle {
    pub(crate) fn new(
        port: u16,
        shutdown_channel: oneshot::Sender<()>,
        routing_table: SharedRoutingTable,
    ) -> Self {
        Self {
            port,
            shutdown_channel: Some(shutdown_channel),
            routing_table,
        }
    }

    pub fn stop(&mut self) {
        if let Some(shutdown_channel) = self.shutdown_channel.take() {
            let _ = shutdown_channel.send(());
        }
    }

    pub fn reload(&self, new_table: RoutingTable) {
        if let Ok(mut table) = self.routing_table.write() {
            *table = new_table;
        }
    }

    pub fn is_running(&self) -> bool {
        self.shutdown_channel.is_some()
    }
}

pub enum ServerError {
    PortInUse(u16),
    IoError(std::io::Error),
    InvalidPort,
    PortBindError(String),
    DbError(rusqlite::Error),
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::PortInUse(p) => write!(f, "Port {p} is already in use"),
            ServerError::IoError(e) => write!(f, "IO error: {}", e),
            ServerError::InvalidPort => write!(f, "Invalid port"),
            ServerError::PortBindError(e) => write!(f, "Port bind error: {}", e),
            ServerError::DbError(e) => write!(f, "Database error: {}", e),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MockAPICallerLog {
    pub route_id: Option<String>,

    /// Unix timestamp in seconds
    pub timestamp: i64,

    /// Uppercased HTTP method e.g. "GET", "POST"
    pub method: String,

    /// The exact path that was hit e.g. "/api/auth"
    pub url_hit: String,

    /// How long the handler took to respond in milliseconds
    pub latency_ms: u32,

    /// The HTTP status code that was returned
    pub response_status: u16,
}

#[derive(Debug, Clone)]
pub(crate) struct ServerHandlerState {
    pub routing_table: SharedRoutingTable,
    pub logging_sender: mpsc::UnboundedSender<MockAPICallerLog>,
    pub global_prefix: String,
}
