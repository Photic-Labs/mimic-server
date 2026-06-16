use crate::{
    panels::helpers::{
        app_helpers::{method_tag_colors, status_color},
        details_helpers::{validate_path, HTTP_METHODS, HTTP_STATUS_CODES},
    },
    store::app_store::AppStore,
};
use mimic_core::models::MockedApiRoute;
use pl_components::{
    globals::{FONT_LG, FONT_MD, SPACE_LG, SPACE_MD, SPACE_SM, SPACE_XL, SPACE_XS},
    PLBadge, PLButton, PLLabel, PLTextInput,
};
use rusqlite::Connection;

#[derive(Default)]
pub struct DetailsPanel {
    edit_method: String,
    edit_path: String,
    edit_status_code: u16,
    edit_response_path: String,
    path_error: Option<String>,
    pub loaded_route_id: Option<String>,
}

// ── Public ─────────────────────────────────────────────────────────────────────
impl DetailsPanel {
    pub fn show(&mut self, ui: &mut egui::Ui, conn: &Connection, app_store: &mut AppStore) {
        let selected_route = app_store.details_store.selected_route.clone();
        match selected_route {
            None => self.render_empty_state(ui),
            Some(route) => {
                if self.loaded_route_id != Some(route.id.clone()) {
                    self.prefill_the_state(&route);
                }
                self.render_detail(ui, conn, &route, app_store);
            }
        }
    }
}

// ── Private ────────────────────────────────────────────────────────────────────
impl DetailsPanel {
    fn prefill_the_state(&mut self, route: &MockedApiRoute) {
        self.loaded_route_id = Some(route.id.clone());
        self.edit_method = route.method.clone();
        self.edit_path = route.path.clone();
        self.edit_status_code = route.status_code;
        self.edit_response_path = route.response_path.clone().unwrap_or_default();
        self.path_error = None;
    }

    fn render_empty_state(&self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            PLLabel::empty("← Select a route to view or edit").show(ui);
        });
    }

    fn render_detail(
        &mut self,
        ui: &mut egui::Ui,
        conn: &Connection,
        route: &MockedApiRoute,
        app_store: &mut AppStore,
    ) {
        // ── Top bar — fixed height, no scroll ────────────────────────────────
        self.render_top_bar(ui, conn, route, app_store);

        ui.add_space(SPACE_SM);
        ui.separator();
        ui.add_space(SPACE_LG);

        // ── Scrollable form body ──────────────────────────────────────────────
        egui::ScrollArea::vertical()
            .id_salt("detail_scroll_area")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Constrain form to a readable max width, left-aligned
                let form_width = ui.available_width().min(680.0);
                ui.set_max_width(form_width);

                self.render_form_body(ui, conn, app_store);
            });
    }

    // ── Top bar ───────────────────────────────────────────────────────────────
    // [Method badge]  [Path text]  →→→  [Delete button]
    fn render_top_bar(
        &mut self,
        ui: &mut egui::Ui,
        conn: &Connection,
        route: &MockedApiRoute,
        app_store: &mut AppStore,
    ) {
        // Fixed height row so badge + path + delete are all baseline-aligned
        ui.horizontal(|ui| {
            ui.set_min_height(32.0);

            // ── Method badge ──────────────────────────────────────────────────
            let (bg, fg) = method_tag_colors(&route.method);
            PLBadge::new(&route.method, bg, fg).show(ui);

            ui.add_space(SPACE_MD);

            // ── Route path — secondary heading ────────────────────────────────
            ui.label(
                egui::RichText::new(&route.path)
                    .size(FONT_LG)
                    .color(ui.visuals().text_color()),
            );

            // ── Delete — pushed to the right ──────────────────────────────────
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if PLButton::new("Delete")
                    .danger()
                    .tooltip("Delete this route")
                    .show(ui)
                    .clicked()
                {
                    self.handle_delete(conn, route.id.as_str(), app_store);
                }
            });
        });
    }

    // ── Form body — all fields stacked vertically ─────────────────────────────
    fn render_form_body(&mut self, ui: &mut egui::Ui, conn: &Connection, app_store: &mut AppStore) {
        let available = ui.available_width();

        // ── Section 1: Method + Status Code ──────────────────────────────────
        // Two columns: Method is narrow (140px), Status Code takes the rest
        // Both columns are top-aligned so their labels line up on the same baseline
        self.render_method_status_row(ui);

        ui.add_space(SPACE_XL);

        // ── Section 2: Path ───────────────────────────────────────────────────
        self.render_path_field(ui, available);

        ui.add_space(SPACE_XL);

        // ── Section 3: Response File ──────────────────────────────────────────
        self.render_response_file_field(ui, available);

        ui.add_space(SPACE_XL);

        // ── Section 4: Save ───────────────────────────────────────────────────
        self.render_save_row(ui, conn, app_store);

        // Bottom breathing room
        ui.add_space(SPACE_XL);
    }
}

// ── Form Field Renderers ───────────────────────────────────────────────────────
impl DetailsPanel {
    // ── Method + Status Code — two-column row ─────────────────────────────────
    fn render_method_status_row(&mut self, ui: &mut egui::Ui) {
        let text_color = ui.visuals().widgets.inactive.fg_stroke.color;

        // Use columns() for proper equal-top alignment
        // Col 0 = Method (fixed 140px), Col 1 = Status Code (rest)
        ui.columns(2, |cols| {
            // ── Col 0: Method ─────────────────────────────────────────────────
            cols[0].set_max_width(160.0);
            cols[0].vertical(|ui| {
                PLLabel::body("Method")
                    .color(ui.visuals().text_color())
                    .show(ui);
                ui.add_space(SPACE_SM);

                egui::ComboBox::from_id_salt("detail_method")
                    .selected_text(
                        egui::RichText::new(&self.edit_method)
                            .size(FONT_MD)
                            .strong()
                            .color(text_color),
                    )
                    .width(140.0)
                    .show_ui(ui, |ui| {
                        for method in HTTP_METHODS {
                            ui.selectable_value(
                                &mut self.edit_method,
                                method.to_string(),
                                egui::RichText::new(*method).size(FONT_MD).color(text_color),
                            );
                        }
                    });
            });

            // ── Col 1: Status Code ────────────────────────────────────────────
            cols[1].vertical(|ui| {
                PLLabel::body("Status Code")
                    .color(ui.visuals().text_color())
                    .show(ui);
                ui.add_space(SPACE_SM);

                let fallback_color = ui.visuals().widgets.inactive.fg_stroke.color;
                let selected_label = HTTP_STATUS_CODES
                    .iter()
                    .find(|(code, _)| *code == self.edit_status_code)
                    .map(|(_, label)| *label)
                    .unwrap_or("Select status");

                let selected_color = if self.edit_status_code == 0 {
                    fallback_color
                } else {
                    status_color(self.edit_status_code)
                };

                egui::ComboBox::from_id_salt("detail_status")
                    .selected_text(
                        egui::RichText::new(selected_label)
                            .size(FONT_MD)
                            .color(selected_color),
                    )
                    .width(220.0)
                    .show_ui(ui, |ui| {
                        for (code, label) in HTTP_STATUS_CODES {
                            ui.selectable_value(
                                &mut self.edit_status_code,
                                *code,
                                egui::RichText::new(*label)
                                    .size(FONT_MD)
                                    .color(status_color(*code)),
                            );
                        }
                    });
            });
        });
    }

    // ── Path field — full width ───────────────────────────────────────────────
    fn render_path_field(&mut self, ui: &mut egui::Ui, width: f32) {
        let path_response = PLTextInput::new(&mut self.edit_path)
            .label("Path")
            .hint("/api/resource/:id")
            .monospace()
            .width(width)
            .show(ui);

        if path_response.lost_focus() {
            self.path_error = validate_path(&self.edit_path);
        }

        // Validation feedback — tight spacing below input
        ui.add_space(SPACE_XS);
        match &self.path_error {
            Some(err) => {
                PLLabel::error(&format!("✕  {err}")).show(ui);
            }
            None if !self.edit_path.is_empty() => {
                PLLabel::success("✓  Valid path").show(ui);
            }
            _ => {}
        }
    }

    // ── Response File — input + Browse + ✕ all on one line ───────────────────
    fn render_response_file_field(&mut self, ui: &mut egui::Ui, width: f32) {
        // Label sits above the row
        PLLabel::body("Response File")
            .color(ui.visuals().text_color())
            .show(ui);
        ui.add_space(SPACE_SM);

        // ── Single horizontal row: [input][Browse][✕] ────────────────────────
        // Reserve space for Browse (80px) + gap (SPACE_SM) + optional ✕ (28px)
        let has_file = !self.edit_response_path.is_empty();
        let browse_w = 80.0;
        let clear_w = if has_file { 28.0 + SPACE_SM } else { 0.0 };
        let gap = SPACE_SM;
        let input_w = width - browse_w - gap - clear_w;

        ui.horizontal(|ui| {
            // Vertically center everything in this row
            ui.set_min_height(28.0);

            // Input
            PLTextInput::new(&mut self.edit_response_path)
                .hint("No file selected")
                .monospace()
                .width(input_w)
                .show(ui);

            ui.add_space(gap);

            // Browse
            if PLButton::new("Browse")
                .subtle()
                .width(browse_w)
                .tooltip("Pick a JSON response file")
                .show(ui)
                .clicked()
            {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .set_title("Select Response File")
                    .pick_file()
                {
                    self.edit_response_path = path.to_string_lossy().to_string();
                }
            }

            // ✕ — only when a file is selected
            if has_file {
                ui.add_space(SPACE_SM);
                if PLButton::new("X")
                    .width(28.0)
                    .tooltip("Clear file")
                    .show(ui)
                    .clicked()
                {
                    self.edit_response_path.clear();
                }
            }
        });

        // File name hint — only when set
        if has_file {
            ui.add_space(SPACE_XS);
            let file_name = std::path::Path::new(&self.edit_response_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&self.edit_response_path);
            PLLabel::accent(&format!("📄  {file_name}")).show(ui);
        }
    }

    // ── Save row — error hint left, Save button right ─────────────────────────
    fn render_save_row(&self, ui: &mut egui::Ui, conn: &Connection, app_store: &mut AppStore) {
        let has_error = self.path_error.is_some();
        let path_empty = self.edit_path.trim().is_empty();
        let resp_empty = self.edit_response_path.trim().is_empty();
        let can_save = !has_error && !path_empty && !resp_empty;

        ui.horizontal(|ui| {
            // ── Error hint — left side ────────────────────────────────────────
            if has_error {
                PLLabel::error("Fix path errors before saving").show(ui);
            }

            // ── Save button — right side ──────────────────────────────────────
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if PLButton::new("Save Changes")
                    .primary()
                    .enabled(can_save)
                    .tooltip(if can_save {
                        "Save route changes"
                    } else {
                        "Fix errors before saving"
                    })
                    .show(ui)
                    .clicked()
                {
                    self.handle_save(conn, self.loaded_route_id.clone(), app_store);
                }
            });
        });
    }
}

// ── Event Handlers ─────────────────────────────────────────────────────────────
impl DetailsPanel {
    fn handle_delete(&mut self, conn: &Connection, route_id: &str, app_store: &mut AppStore) {
        match app_store.routes_store.delete_mocked_route(conn, route_id) {
            Ok(_) => {
                app_store.refresh_store(conn);
                app_store.details_store.on_clear_selection();
                self.loaded_route_id = None;
            }
            Err(e) => eprintln!("[details] Failed to delete route {route_id}: {e}"),
        }
    }

    fn handle_save(
        &self,
        conn: &Connection,
        loaded_route_id: Option<String>,
        app_store: &mut AppStore,
    ) {
        let route_id = match loaded_route_id {
            Some(id) => id,
            None => {
                eprintln!("[details] handle_save called with no loaded_route_id");
                return;
            }
        };
        let route_id = route_id.as_str();

        let response_path = if self.edit_response_path.is_empty() {
            None
        } else {
            Some(self.edit_response_path.clone())
        };

        match app_store.routes_store.update_mocked_route(
            conn,
            route_id,
            &self.edit_method,
            &self.edit_path,
            self.edit_status_code,
            response_path.unwrap(),
        ) {
            Ok(_) => {
                app_store.refresh_store(conn);
                let _ = app_store.details_store.on_route_selected(conn, route_id);
            }
            Err(e) => eprintln!("[details] Failed to save route {route_id}: {e}"),
        }
    }
}
