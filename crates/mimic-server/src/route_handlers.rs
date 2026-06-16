use std::time::SystemTime;

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    response::Response,
};
use tokio::time::Instant;

use crate::{
    helpers::{error_body, get_request_entry},
    types::{MockAPICallerLog, ServerHandlerState},
};

// ─────────────────────────────────────────────────────────────────────────────
// HEALTH CHECK
// The only hardcoded route in the entire server.
// Always responds 200 — used to confirm the server is alive.
// Prefix ~ ensures it never collides with a real API path.
// ─────────────────────────────────────────────────────────────────────────────
pub async fn health_handler() -> Response {
    let body = serde_json::json!({
        "status":  "ok",
        "service": "MimicServer | For API Mocking",
        "server_time": SystemTime::now()
    })
    .to_string();

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(body))
        .unwrap()
}

// ─────────────────────────────────────────────────────────────────────────────
// MOCK HANDLER
// The fallback handler — Axum routes ALL unmatched requests here.
// This is the middleware interception point you designed.
//
// Flow:
//   1. Extract method + path from the request
//   2. Normalize both (same function the compiler used)
//   3. Look up in the shared routing table
//   4. Hit  → read .json file → respond with configured status code
//   5. Miss → respond 404 with a helpful JSON body
//   6. Build a TrafficLog entry and return it for the caller to persist
// ─────────────────────────────────────────────────────────────────────────────
pub async fn mock_routes_handler(
    State(state): State<ServerHandlerState>,
    request: Request<Body>,
) -> Response {
    let started_at = Instant::now();
    // Get server entry from the table based on the request
    let (entry, method, path, url_hit) = get_request_entry(
        request,
        state.routing_table,
        &state.global_prefix, // e.g. "" or "/api/v1"
    );

    let (response, route_id, response_status) = match entry {
        Some(route) => {
            let (body, final_status) = match &route.response_path {
                Some(file_path) => match tokio::fs::read_to_string(file_path).await {
                    Ok(content) => match serde_json::from_str::<serde_json::Value>(&content) {
                        Ok(_) => (content, route.status),
                        Err(_) => (
                            error_body(
                                "invalid_json_file",
                                &method,
                                &path,
                                Some("Response file exists but is not valid JSON"),
                            ),
                            500,
                        ),
                    },
                    Err(e) => (
                        error_body(
                            "response_file_not_found",
                            &method,
                            &path,
                            Some(&e.to_string()),
                        ),
                        500,
                    ),
                },
                None => (
                    error_body(
                        "no_response_file_configured",
                        &method,
                        &path,
                        Some("Add a response file to this route in MimicServer"),
                    ),
                    501,
                ),
            };
            let status = StatusCode::from_u16(final_status).unwrap_or(StatusCode::OK);

            let latency_ms = started_at.elapsed().as_millis() as u32;

            let response = Response::builder()
                .status(status)
                .header("Content-Type", "application/json")
                .header("X-Mimic-Route-Id", route.route_id.to_string())
                .header("X-Mimic-Latency-Ms", latency_ms.to_string())
                .header("X-Powered-By", "MimicServer")
                .body(Body::from(body))
                .unwrap();

            (response, Some(route.route_id), final_status)
        }
        None => {
            let body = error_body("route_not_found", &method, &path, None);

            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json")
                .header("X-Powered-By", "MimicServer")
                .body(Body::from(body))
                .unwrap();

            (response, None, 404)
        }
    };

    let latency_ms = started_at.elapsed().as_millis() as u32;
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let mock_api_log = MockAPICallerLog {
        route_id: route_id,
        timestamp,
        method,
        url_hit,
        latency_ms,
        response_status,
    };
    let _ = state.logging_sender.send(mock_api_log); // Fire & Forget of the mock api log
    response
}
