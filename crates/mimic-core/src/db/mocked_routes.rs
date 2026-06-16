use std::collections::HashMap;

use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::{
    db::queries::{
        ADD_ROUTE_TO_GROUP_SQL, DELETE_ROUTE_FROM_GROUP_SQL, GET_MOCKED_ROUTE_SQL,
        LOAD_ALL_ROUTES_SQL, UPDATE_MOCKED_ROUTE_SQL,
    },
    models::MockedApiRoute,
};

// ── Row mapper — single source of truth ───────────────────────────────────────
fn map_route(row: &rusqlite::Row) -> rusqlite::Result<MockedApiRoute> {
    Ok(MockedApiRoute {
        id: row.get(0)?,
        group_id: row.get(1)?,
        method: row.get(2)?,
        path: row.get(3)?,
        status_code: row.get::<_, u32>(4)? as u16,
        response_type: row.get(5)?,
        response_path: row.get(6)?,
    })
}

// ── Load all routes — grouped by group_id ────────────────────────────────────
pub fn get_all_routes(conn: &Connection) -> Result<HashMap<String, Vec<MockedApiRoute>>> {
    let mut stmt = conn.prepare(LOAD_ALL_ROUTES_SQL)?;

    let rows = stmt.query_map([], |row| map_route(row))?;

    let mut map: HashMap<String, Vec<MockedApiRoute>> = HashMap::new();
    for row in rows {
        let route = row.context("Failed to read route row")?;
        map.entry(route.group_id.clone()).or_default().push(route);
    }

    Ok(map)
}

// ── Add route — generates UUID, returns it ────────────────────────────────────
// SQL param order: ?1=id, ?2=group_id, ?3=method, ?4=path,
//                  ?5=status_code, ?6=response_type, ?7=response_path
pub fn add_route_to_group(
    conn: &Connection,
    parent_group_id: &str,
    method: &str,
    path: &str,
    status_code: u16,
    response_type: &str,
    response_path: Option<String>,
) -> Result<String> {
    let id = Uuid::new_v4().to_string();

    conn.execute(
        ADD_ROUTE_TO_GROUP_SQL,
        params![
            id,
            parent_group_id,
            method,
            path,
            status_code as i64,
            response_type,
            response_path,
        ],
    )
    .context("Failed to insert route")?;

    Ok(id)
}

// ── Get single route by UUID ──────────────────────────────────────────────────
pub fn get_route(conn: &Connection, route_id: &str) -> Result<MockedApiRoute> {
    let mut stmt = conn.prepare(GET_MOCKED_ROUTE_SQL)?;

    let route = stmt
        .query_row(params![route_id], |row| map_route(row))
        .context("Route not found")?;

    Ok(route)
}

// ── Update route ──────────────────────────────────────────────────────────────
// SQL param order: ?1=method, ?2=path, ?3=status_code,
//                  ?4=response_type, ?5=response_path, ?6=id
pub fn update_route(
    conn: &Connection,
    route_id: &str,
    method: &str,
    path: &str,
    status_code: u16,
    response_type: &str,
    response_path: String,
) -> Result<()> {
    conn.execute(
        UPDATE_MOCKED_ROUTE_SQL,
        params![
            method,
            path,
            status_code as i64,
            response_type,
            response_path,
            route_id,
        ],
    )
    .context("Failed to update route")?;

    Ok(())
}

// ── Delete route ──────────────────────────────────────────────────────────────
pub fn delete_route(conn: &Connection, route_id: &str) -> Result<()> {
    conn.execute(DELETE_ROUTE_FROM_GROUP_SQL, params![route_id])
        .context("Failed to delete route")?;

    Ok(())
}
