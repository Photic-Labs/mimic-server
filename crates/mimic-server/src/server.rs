use std::sync::{Arc, RwLock};

use axum::{
    http::{HeaderName, Method},
    routing::get,
    Router,
};
use rusqlite::Connection;
use tokio::{
    net::TcpListener,
    sync::{mpsc, oneshot},
};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    helpers::{read_global_prefix, read_port, write_mock_api_caller_logs},
    route_handlers::{health_handler, mock_routes_handler},
    route_table_helper::get_routing_table,
    types::{MockAPICallerLog, ServerHandlerState},
    RoutingTable, ServerError, ServerHandle, SharedRoutingTable,
};

pub async fn start_server(db_path: &str) -> Result<ServerHandle, ServerError> {
    // Read PORT defined from the app_config
    let conn = Connection::open(db_path).map_err(ServerError::DbError)?;
    let server_port = read_port(&conn).map_err(ServerError::DbError)?;
    // Wherever you build ServerHandlerState
    let global_prefix = read_global_prefix(&conn).unwrap_or_default();
    // normalize it — strip trailing slash, ensure leading slash if non-empty
    let global_prefix = if global_prefix.is_empty() {
        String::new()
    } else {
        format!("/{}", global_prefix.trim_matches('/'))
    };
    // Prepare the routing table
    let routing_table: RoutingTable = get_routing_table(&conn).map_err(ServerError::DbError)?;
    let shared_routing_table: SharedRoutingTable = Arc::new(RwLock::new(routing_table));
    // Create MockAPICallerLog Channels
    let (log_tx, log_rx) = mpsc::unbounded_channel::<MockAPICallerLog>();
    // Create an AXUM Router
    let app_state = ServerHandlerState {
        logging_sender: log_tx,
        routing_table: shared_routing_table.clone(),
        global_prefix,
    };
    let cors = CorsLayer::new()
        // ── Allow the origins that will call this server ──────────────
        // For local dev with index.html opened from disk or localhost,
        // Any is fine. Tighten this in production.
        .allow_origin(Any)
        // ── Allow the HTTP methods your routes use ────────────────────
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS, // ← preflight
        ])
        // ── Allow the headers your index.html sends ───────────────────
        .allow_headers([
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
        ]);
    let app = Router::new()
        .route("/health", get(health_handler))
        .fallback(mock_routes_handler)
        .layer(cors)
        .with_state(app_state);
    // Create APP Listener for the AXUM Server
    let tcp_listener = get_tcp_listener(server_port).await?;

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    tokio::spawn(async move {
        println!("[mimic] Server running on http://localhost:{server_port}");
        println!("[mimic] Health: http://localhost:{server_port}/health");
        axum::serve(tcp_listener, app)
            .with_graceful_shutdown(async move {
                let _ = shutdown_rx.await;
                println!("[mimic] Shutdown signal received — stopping server");
            })
            .await
            .unwrap_or_else(|e| eprintln!("[mimic] Server error: {e}"));

        println!("[mimic] Server stopped");
    });

    // Create task for MockAPICallLog
    let db_path_owned = db_path.to_string();
    tokio::spawn(async move { write_mock_api_caller_logs(&db_path_owned, log_rx).await });

    Ok(ServerHandle::new(
        server_port,
        shutdown_tx,
        shared_routing_table,
    ))
}

async fn get_tcp_listener(port: u16) -> Result<TcpListener, ServerError> {
    let addr = format!("0.0.0.0:{port}");
    tokio::net::TcpListener::bind(&addr).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::AddrInUse {
            ServerError::PortInUse(port)
        } else {
            ServerError::PortBindError(e.to_string())
        }
    })
}
