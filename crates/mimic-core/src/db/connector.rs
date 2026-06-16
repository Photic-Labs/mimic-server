use anyhow::Context;
use rusqlite::Connection;
use std::io::Result;

use crate::db::queries::{
    API_GROUPS_SQL, APP_CONFIG_SEED_SQL, APP_CONFIG_SQL, MOCKED_ROUTES_SQL, ROUTE_TAGS_SQL,
    TRAFFIC_LOGS_SQL,
};

pub fn open_connection(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)
        .with_context(|| format!("Failed to open DB at: {db_path}"))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    // Enable foreign keys and WAL journal mode
    conn.execute_batch("PRAGMA foreign_keys = ON;")
        .with_context(|| "Failed to enable foreign keys")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    conn.execute_batch("PRAGMA journal_mode = WAL;")
        .with_context(|| "Failed to set journal mode to WAL")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    // Create tables and seed data
    bootstrap_database(&conn)
        .with_context(|| "Failed to bootstrap database")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    seed_database(&conn)
        .with_context(|| "Failed to seed database")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(conn)
}

pub fn bootstrap_database(conn: &Connection) -> Result<()> {
    // Create APP_CONFIG table
    conn.execute(APP_CONFIG_SQL, [])
        .with_context(|| "Failed to create app_config table")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    // Create API groups table
    conn.execute(API_GROUPS_SQL, [])
        .with_context(|| "Failed to create api_groups table")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    // Create Mocked Routes table
    conn.execute(MOCKED_ROUTES_SQL, [])
        .with_context(|| "Failed to create mocked_routes table")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    // Create Route Tags
    conn.execute(ROUTE_TAGS_SQL, [])
        .with_context(|| "Failed to create route tags table")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    // Create Traffic Logs
    conn.execute(TRAFFIC_LOGS_SQL, [])
        .with_context(|| "Failed to create traffic logs table")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(())
}

pub fn seed_database(conn: &Connection) -> Result<()> {
    conn.execute(APP_CONFIG_SEED_SQL, [])
        .with_context(|| "Failed to seed app_config table")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(())
}
