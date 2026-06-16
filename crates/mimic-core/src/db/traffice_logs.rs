use anyhow::Result;
use rusqlite::params;

use crate::{
    db::queries::{CLEAR_TRAFFIC_LOGS_FOR_ROUTE_SQL, GET_TRAFFIC_LOGS_FOR_ROUTE_SQL},
    models::TrafficLog,
};

pub fn get_logs_for(conn: &rusqlite::Connection, route_id: &str) -> Result<Vec<TrafficLog>> {
    let mut stmt = conn.prepare(GET_TRAFFIC_LOGS_FOR_ROUTE_SQL)?;
    let rows = stmt.query_map(params![route_id], |row| {
        Ok(TrafficLog {
            id: row.get(0)?,
            route_id: row.get(1)?,
            timestamp: row.get(2)?,
            method: row.get(3)?,
            url_hit: row.get(4)?,
            latency_ms: row.get(5)?,
            response_status: row.get(6)?,
        })
    })?;
    let mut logs: Vec<TrafficLog> = Vec::new();
    for row in rows {
        logs.push(row?);
    }
    Ok(logs)
}

pub fn clear_logs_for(conn: &rusqlite::Connection, route_id: &str) -> Result<()> {
    let mut stmt = conn.prepare(CLEAR_TRAFFIC_LOGS_FOR_ROUTE_SQL)?;
    stmt.execute(params![route_id])?;
    Ok(())
}
