use anyhow::Result;
use mimic_core::{db, models::MockedApiRoute};
use rand::Rng;
use rusqlite::Connection;
use std::collections::HashMap;

use crate::store::store_traits::Loadable;

pub struct MockedRouteStore {
    pub routes: HashMap<String, Vec<MockedApiRoute>>,
}

impl Loadable for MockedRouteStore {
    fn load_from_db(conn: &Connection) -> MockedRouteStore {
        Self {
            routes: db::mocked_routes::get_all_routes(conn).unwrap_or_default(),
        }
    }

    fn refresh_data(&mut self, conn: &Connection) {
        self.routes = db::mocked_routes::get_all_routes(conn)
            .unwrap_or_default()
            .into_iter()
            .map(|(group_id, routes)| (group_id, routes))
            .collect();
    }
}

impl MockedRouteStore {
    pub fn add_mocked_route(&mut self, conn: &Connection, parent_group_id: &str) -> Result<()> {
        let id = rand::thread_rng().gen_range(1000..9999);
        db::mocked_routes::add_route_to_group(
            conn,
            parent_group_id,
            "GET",
            format!("/new-route-{}", id).as_str(),
            200,
            "JSON",
            None,
        )?;
        Ok(())
    }

    pub fn update_mocked_route(
        &mut self,
        conn: &Connection,
        route_id: &str,
        method: &str,
        path: &str,
        status_code: u16,
        response_path: String,
    ) -> Result<()> {
        db::mocked_routes::update_route(
            conn,
            route_id,
            method,
            path,
            status_code,
            "JSON",
            response_path,
        )?;
        Ok(())
    }

    pub fn delete_mocked_route(&mut self, conn: &Connection, route_id: &str) -> Result<()> {
        db::mocked_routes::delete_route(conn, route_id)?;
        db::traffice_logs::clear_logs_for(conn, route_id)?;
        Ok(())
    }
}
