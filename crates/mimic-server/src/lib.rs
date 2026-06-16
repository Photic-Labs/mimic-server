// mimic_core/src/server/mod.rs

pub(crate) mod helpers;
pub(crate) mod queries;
pub(crate) mod route_handlers;
pub mod route_table_helper;
pub mod server;
pub mod types;

// These will be uncommented as we build each stage
// pub mod compiler;
// pub mod mock_handler;
// pub mod host_controller;

pub use types::{RouteEntry, RoutingTable, ServerError, ServerHandle, SharedRoutingTable};
