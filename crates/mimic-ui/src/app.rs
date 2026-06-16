use std::path::PathBuf;

use pl_components::theme::AppTheme;
use rusqlite::Connection;

use crate::{
    modals::settings_modal::SettingsModal,
    panels::{
        details_panel::DetailsPanel, sidebar_panel::SidebarPanel, top_panel::TopBarPanel,
        traffic_panel::TrafficPanel,
    },
    store::app_store::AppStore,
};

pub struct MimicServerApp {
    pub conn: Connection,
    pub app_store: AppStore,
    pub sidebar: SidebarPanel,
    pub top_panel: TopBarPanel,
    pub traffic_panel: TrafficPanel,
    pub details_panel: DetailsPanel,
    pub settings_modal: SettingsModal,
}

impl MimicServerApp {
    pub fn new(cc: &eframe::CreationContext<'_>, conn: Connection, db_path: PathBuf) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        let app_store = AppStore::load(&conn, db_path);
        // ── Restore persisted theme ────────────────────────────────────────
        // Apply persisted theme before first frame
        let visuals = app_store.settings_store.theme.to_visuals();
        cc.egui_ctx.set_theme(match app_store.settings_store.theme {
            AppTheme::Dark => egui::ThemePreference::Dark,
            AppTheme::Light => egui::ThemePreference::Light,
        });
        cc.egui_ctx.set_visuals(visuals);
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            app_store,
            conn,
            sidebar: SidebarPanel::default(),
            top_panel: TopBarPanel::default(),
            traffic_panel: TrafficPanel::default(),
            details_panel: DetailsPanel::default(),
            settings_modal: SettingsModal::default(),
        }
    }
}

impl eframe::App for MimicServerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // ── Global Event Controls ──────────────────────────────────────────────────
        let mut start_clicked = false;
        let mut stop_clicked = false;
        let mut on_settings_toggled = false;

        egui::Panel::top("top_bar")
            .max_size(50.0)
            .default_size(50.0)
            .min_size(50.0)
            .resizable(false)
            .show_inside(ui, |ui| {
                self.top_panel.show(
                    ui,
                    &self.app_store.server_store.status,
                    &mut start_clicked,
                    &mut stop_clicked,
                    &mut on_settings_toggled,
                );
            });
        // ── Sidebar ──────────────────────────────────────────────────
        egui::Panel::left("api_groups")
            .resizable(true)
            .default_size(ui.available_width() * 0.2)
            .min_size(ui.available_width() * 0.2)
            .max_size(ui.available_width() / 1.75)
            .show_inside(ui, |ui| {
                self.sidebar.show(ui, &self.conn, &mut self.app_store);
            });
        // ── Bottom traffic log ────────────────────────────────────────────
        // In your main update() loop
        egui::Panel::bottom("traffic_panel")
            .resizable(true)
            .default_size(250.0)
            .min_size(120.0)
            .show_inside(ui, |ui| {
                self.traffic_panel.show(ui, &self.conn, &mut self.app_store);
            });
        // ── Center Layout ────────────────────────────────────────────
        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.details_panel.show(ui, &self.conn, &mut self.app_store);
        });

        if let Some(new_theme) =
            self.settings_modal
                .show(ui, &self.conn, &mut self.app_store.settings_store)
        {
            // Theme changed inside settings — apply immediately
            ui.ctx().set_theme(match new_theme {
                AppTheme::Dark => egui::ThemePreference::Dark,
                AppTheme::Light => egui::ThemePreference::Light,
            });
            ui.ctx().set_visuals(new_theme.to_visuals());
            ui.ctx().request_repaint();
        }

        // ── Handle signals AFTER panels are drawn ─────────────────────────────
        if start_clicked {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    self.app_store.server_store.start().await;
                });
            });
            ui.request_repaint();
        }

        if stop_clicked {
            self.app_store.server_store.stop();
            ui.request_repaint();
        }

        if on_settings_toggled {
            self.settings_modal.open();
        }
    }
}
