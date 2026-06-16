use anyhow::{Ok, Result};
use mimic_core::{db, models::ApiGroup};
use rusqlite::Connection;

use crate::store::store_traits::Loadable;

pub struct ApiGroupStore {
    pub api_groups: Vec<ApiGroup>,
}

impl Loadable for ApiGroupStore {
    fn load_from_db(conn: &Connection) -> Self {
        Self {
            api_groups: db::api_groups::load_api_groups(conn).unwrap_or_default(),
        }
    }

    fn refresh_data(&mut self, conn: &Connection) {
        self.api_groups = db::api_groups::load_api_groups(&conn).unwrap_or_default();
    }
}

impl ApiGroupStore {
    pub fn create_api_group(&mut self, conn: &Connection, group_name: &str) -> Result<()> {
        db::api_groups::save_api_group(conn, group_name)?;
        self.refresh_data(conn);
        Ok(())
    }

    // pub fn delete_api_group(&mut self, conn: &Connection, group_id: i64) -> Result<()> {
    //     db::api_groups::delete_api_group(conn, group_id)?;
    //     self.refresh_data(conn);
    //     Ok(())
    // }
}
