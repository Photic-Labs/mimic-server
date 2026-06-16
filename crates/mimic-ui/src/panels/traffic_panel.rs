use mimic_core::models::TrafficLog;
use pl_components::{
    globals::{FONT_BASE, FONT_LG, FONT_SM, RADIUS_SM, ROW_HEIGHT},
    Color, PLBadge, PLButton, PLLabel,
};
use rusqlite::Connection;

use crate::{
    panels::helpers::app_helpers::{
        dim_header, format_timestamp, get_available_width, latency_color, method_tag_colors,
        status_color, COL_LATENCY, COL_METHOD, COL_STATUS, COL_TIME,
    },
    store::app_store::AppStore,
};

#[derive(Default)]
pub struct TrafficPanel;

impl TrafficPanel {
    pub fn show(&mut self, ui: &mut egui::Ui, conn: &Connection, app_store: &mut AppStore) {
        let logs = &app_store
            .details_store
            .route_logs
            .clone()
            .unwrap_or_default();
        self.render_header(ui, conn, app_store, logs.len());
        self.render_logs_table(ui, logs);
    }
}

// Private Methods
impl TrafficPanel {
    fn render_header(
        &mut self,
        ui: &mut egui::Ui,
        conn: &Connection,
        app_store: &mut AppStore,
        logs_count: usize,
    ) {
        ui.horizontal(|ui| {
            PLLabel::heading("🔁  Traffic Log")
                .size(FONT_LG)
                .color(Color::TEXT_PRIMARY)
                .show(ui);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if PLButton::new("🗑")
                    .tooltip("Clear all logs")
                    .show(ui)
                    .clicked()
                {
                    match app_store.details_store.on_clear_logs(conn) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Failed to clear logs: {}", e);
                        }
                    }
                }

                // ── Reload button ─────────────────────────────────────────────
                if PLButton::new("↺").tooltip("Reload logs").show(ui).clicked() {
                    match app_store.details_store.on_reload_logs(conn) {
                        Ok(_) => {}
                        Err(e) => eprintln!("[traffic_panel] Failed to reload logs: {e}"),
                    }
                }

                // ── Count badge ───────────────────────────────────────────────
                if logs_count > 0 {
                    PLBadge::count(logs_count).show(ui);
                }
            });
        });
        ui.separator();
    }

    fn render_logs_table(&self, ui: &mut egui::Ui, logs: &[TrafficLog]) {
        // ── Empty state ───────────────────────────────────────────────────────
        if logs.is_empty() {
            ui.add_space(8.0);
            PLLabel::empty("No traffic recorded yet. Hit an endpoint to see logs here.")
                .color(Color::TEXT_DIM)
                .show(ui);
            return;
        }
        let url_col_width = get_available_width(ui.available_width());
        self.render_table_header(ui, url_col_width);
        self.render_table_content(ui, logs, url_col_width);
    }

    fn render_table_header(&self, ui: &mut egui::Ui, url_col_width: f32) {
        ui.horizontal(|ui| {
            ui.add_sized([COL_METHOD, 16.0], egui::Label::new(dim_header("METHOD")));
            ui.add_sized([COL_STATUS, 16.0], egui::Label::new(dim_header("STATUS")));
            ui.add_sized([url_col_width, 16.0], egui::Label::new(dim_header("URL")));
            ui.add_sized([COL_LATENCY, 16.0], egui::Label::new(dim_header("LATENCY")));
            ui.add_sized([COL_TIME, 16.0], egui::Label::new(dim_header("TIME")));
        });
        ui.separator();
    }

    fn render_table_content(&self, ui: &mut egui::Ui, logs: &[TrafficLog], url_col_width: f32) {
        egui::ScrollArea::vertical()
            .id_salt("traffic_log_scroll")
            .stick_to_bottom(true) // ← auto scroll to latest
            .show(ui, |ui| {
                for log in logs.iter().rev() {
                    // newest first
                    self.render_log_row(ui, log, url_col_width);
                    ui.separator();
                }
            });
    }

    fn render_log_row(&self, ui: &mut egui::Ui, log: &TrafficLog, url_col_width: f32) {
        ui.horizontal(|ui| {
            self.render_method_pill(ui, log);
            self.render_status_code_pill(ui, log);
            self.render_url_pill(ui, log, url_col_width);
            self.render_latency_pill(ui, log);
            self.render_timestamp_pill(ui, log);
        });
    }

    fn render_method_pill(&self, ui: &mut egui::Ui, log: &TrafficLog) {
        // ── Method pill ───────────────────────────────────────────────────
        ui.add_sized([COL_METHOD, ROW_HEIGHT], |ui: &mut egui::Ui| {
            let (bg, fg) = method_tag_colors(&log.method);
            egui::Frame::new()
                .fill(bg)
                .corner_radius(egui::CornerRadius::same(RADIUS_SM as u8))
                .inner_margin(egui::Margin::symmetric(6, 2))
                .show(ui, |ui| {
                    PLLabel::body(&log.method).bold().color(fg).show(ui);
                })
                .response
        });
    }

    fn render_status_code_pill(&self, ui: &mut egui::Ui, log: &TrafficLog) {
        // ── Status code ───────────────────────────────────────────────────
        let (status_text, status_color) = match log.response_status {
            Some(s) => (format!("{s}"), status_color(s)),
            None => ("—".to_string(), Color::TEXT_DIM),
        };
        ui.add_sized([COL_STATUS, ROW_HEIGHT], |ui: &mut egui::Ui| {
            PLLabel::accent(status_text)
                .size(FONT_BASE)
                .color(status_color)
                .show(ui)
        });
    }

    fn render_url_pill(&self, ui: &mut egui::Ui, log: &TrafficLog, url_col_width: f32) {
        // ── URL ───────────────────────────────────────────────────────────
        ui.add_sized([url_col_width, 20.0], |ui: &mut egui::Ui| {
            PLLabel::body(&log.url_hit)
                .size(FONT_BASE)
                .color(Color::TEXT_SECONDARY)
                .show(ui)
        });
    }

    fn render_latency_pill(&self, ui: &mut egui::Ui, log: &TrafficLog) {
        // ── Latency ───────────────────────────────────────────────────────
        let (latency_text, latency_color) = match log.latency_ms {
            Some(ms) => (format!("{ms}ms"), latency_color(ms)),
            None => ("—".to_string(), Color::TEXT_DIM),
        };
        ui.add_sized([COL_LATENCY, ROW_HEIGHT], |ui: &mut egui::Ui| {
            PLLabel::accent(latency_text)
                .size(FONT_BASE)
                .color(latency_color)
                .show(ui)
        });
    }

    fn render_timestamp_pill(&self, ui: &mut egui::Ui, log: &TrafficLog) {
        // ── Timestamp ─────────────────────────────────────────────────────
        // Show only time portion HH:MM:SS for brevity
        let time_display = format_timestamp(log.timestamp);
        ui.add_sized([COL_TIME, ROW_HEIGHT], |ui: &mut egui::Ui| {
            PLLabel::body(time_display)
                .size(FONT_SM)
                .color(Color::TEXT_DIM)
                .show(ui)
        });
    }
}
