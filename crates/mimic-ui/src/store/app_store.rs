use std::path::PathBuf;

use anyhow::Result;
use rusqlite::Connection;

use crate::store::{
    api_group_store::ApiGroupStore, details_store::DetailsStore,
    mocked_route_store::MockedRouteStore, server_store::ServerStore, settings_store::SettingsStore,
    store_traits::Loadable,
};

pub struct AppStore {
    pub api_groups_store: ApiGroupStore,
    pub routes_store: MockedRouteStore,
    pub details_store: DetailsStore,
    pub server_store: ServerStore,
    pub settings_store: SettingsStore,
}

impl AppStore {
    pub fn load(conn: &Connection, db_path: PathBuf) -> Self {
        Self {
            api_groups_store: ApiGroupStore::load_from_db(conn),
            routes_store: MockedRouteStore::load_from_db(conn),
            details_store: DetailsStore::default(),
            server_store: ServerStore::new(db_path.to_str().unwrap()),
            settings_store: SettingsStore::load(conn),
        }
    }

    pub fn on_select_route(&mut self, conn: &Connection, route_id: &str) -> Result<()> {
        self.details_store.on_route_selected(conn, route_id)?;
        Ok(())
    }

    pub fn refresh_store(&mut self, conn: &Connection) {
        self.api_groups_store.refresh_data(conn);
        self.routes_store.refresh_data(conn);
        /* --> Server restart for the newly added routes or groups to reflect <-- */
        self.server_store.reload_routes(conn);
    }
}
