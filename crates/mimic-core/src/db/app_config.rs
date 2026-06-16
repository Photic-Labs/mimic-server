use anyhow::{Context, Result};
use rusqlite::{params, Connection};

use crate::{
    db::queries::{LOAD_CONFIG_SQL, SAVE_CONFIG_SQL},
    models::AppConfig,
};

pub fn load_config(conn: &Connection) -> Result<AppConfig> {
    conn.query_row(LOAD_CONFIG_SQL, [], |row| {
        Ok(AppConfig {
            host: row.get(0).unwrap_or("127.0.0.1".into()),
            port: row.get::<_, u32>(1).unwrap_or(8080) as u16,
            prefix: row.get(2).unwrap_or("".into()),
            theme: row.get(3).unwrap_or("dark".into()),
        })
    })
    .context("Failed to load app config")
}

pub fn save_config(conn: &Connection, cfg: AppConfig) -> Result<()> {
    conn.execute(
        SAVE_CONFIG_SQL,
        params![1, cfg.port as i64, cfg.prefix, cfg.theme],
    )
    .context("Failed to save app config")?;
    Ok(())
}
