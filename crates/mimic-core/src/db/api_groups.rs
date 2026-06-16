use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::{
    db::queries::{DELETE_API_GROUP_SQL, INSERT_API_GROUP_SQL, LOAD_API_GROUPS_SQL},
    models::ApiGroup,
};

// ── Row mapper ────────────────────────────────────────────────────────────────
fn map_group(row: &rusqlite::Row) -> rusqlite::Result<ApiGroup> {
    Ok(ApiGroup {
        id: row.get(0)?,
        name: row.get(1)?,
    })
}

pub fn load_api_groups(conn: &Connection) -> Result<Vec<ApiGroup>> {
    let mut stmt = conn.prepare(LOAD_API_GROUPS_SQL)?;

    let rows = stmt
        .query_map([], |row| map_group(row))?
        .collect::<rusqlite::Result<Vec<_>>>()
        .context("Failed to read api_groups rows")?;

    Ok(rows)
}

// ── Save group — generates UUID, returns it ───────────────────────────────────
// SQL param order: ?1=id, ?2=name
pub fn save_api_group(conn: &Connection, name: &str) -> Result<String> {
    let id = Uuid::new_v4().to_string();

    conn.execute(INSERT_API_GROUP_SQL, params![id, name])
        .context("Failed to insert api_group")?;

    Ok(id) // return the UUID so the caller can add it to app state immediately
}

// ── Delete group ──────────────────────────────────────────────────────────────
// Cascades to mocked_routes via ON DELETE CASCADE
pub fn delete_api_group(conn: &Connection, group_id: &str) -> Result<()> {
    conn.execute(DELETE_API_GROUP_SQL, params![group_id])
        .context("Failed to delete api_group")?;

    Ok(())
}
