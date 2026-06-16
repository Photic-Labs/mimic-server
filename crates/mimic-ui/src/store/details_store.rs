use anyhow::Result;
use mimic_core::{
    db,
    models::{MockedApiRoute, TrafficLog},
};
use rusqlite::Connection;

pub struct DetailsStore {
    pub selected_route_id: Option<String>,
    pub selected_route: Option<MockedApiRoute>,
    pub route_logs: Option<Vec<TrafficLog>>,
}

impl Default for DetailsStore {
    fn default() -> Self {
        Self {
            selected_route_id: None,
            selected_route: None,
            route_logs: None,
        }
    }
}

impl DetailsStore {
    pub fn on_route_selected(&mut self, conn: &Connection, route_id: &str) -> Result<()> {
        self.selected_route_id = Some(route_id.to_string());
        self.selected_route = Some(db::mocked_routes::get_route(conn, route_id)?);
        self.route_logs = Some(db::traffice_logs::get_logs_for(conn, route_id)?);
        Ok(())
    }

    pub fn on_clear_selection(&mut self) {
        *self = Self::default();
    }

    // pub fn is_route_selected(&self, route_id: i64) -> bool {
    //     self.selected_route_id == Some(route_id)
    // }
    //
    pub fn on_reload_logs(&mut self, conn: &Connection) -> Result<()> {
        if let Some(route_id) = self.selected_route_id.clone() {
            self.route_logs = Some(db::traffice_logs::get_logs_for(conn, route_id.as_str())?);
        }
        Ok(())
    }

    pub fn on_clear_logs(&mut self, conn: &Connection) -> Result<()> {
        if let Some(route_id) = self.selected_route_id.clone() {
            db::traffice_logs::clear_logs_for(conn, route_id.as_str())?;
        }
        self.route_logs = None;
        Ok(())
    }
}
