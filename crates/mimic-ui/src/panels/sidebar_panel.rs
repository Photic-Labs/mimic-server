use egui::{Align, Color32, RichText};
use mimic_core::models::{ApiGroup, MockedApiRoute};
use pl_components::{PLButton, PLLabel, PLTextInput};
use rusqlite::Connection;

use crate::{
    constants::{
        DEFAULT_MARGIN, DEFAULT_SPACING, FONT_SIZE, FONT_SIZE_SM, FONT_SIZE_TITLE, MARGIN_LARGE,
        SECTION_SPACING,
    },
    panels::helpers::app_helpers::method_tag_colors,
    store::app_store::AppStore,
};

#[derive(Default)]
pub struct SidebarPanel {
    search_query: String,
    new_group_name: String,
    show_group_name_input: bool,
}

// ── Public ─────────────────────────────────────────────────────────────────────
impl SidebarPanel {
    pub fn show(&mut self, ui: &mut egui::Ui, conn: &Connection, app_store: &mut AppStore) {
        ui.vertical(|ui| {
            self.render_header(ui);
            ui.separator();
            ui.add_space(DEFAULT_SPACING);
            self.render_search_bar(ui);
            ui.separator();
            ui.add_space(DEFAULT_SPACING);
            self.add_group_container(ui, conn, app_store);
            ui.separator();
            ui.add_space(DEFAULT_SPACING);
            self.render_groups_list(ui, conn, app_store);
            ui.add_space(DEFAULT_SPACING);
        });
    }
}

// ── Private ────────────────────────────────────────────────────────────────────
impl SidebarPanel {
    fn render_header(&mut self, ui: &mut egui::Ui) {
        ui.add_space(SECTION_SPACING);
        ui.add_sized(
            [ui.available_width(), DEFAULT_MARGIN],
            egui::Label::new(RichText::new("API Groups").heading()),
        );
    }

    fn render_search_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                ui.add_space(2.0);
                PLTextInput::new(&mut self.search_query)
                    .hint("Search API...")
                    .icon_leading("🔍")
                    .monospace()
                    .show(ui)
                    .on_hover_text("Filter routes by path or method");
            });
        });
    }

    fn add_group_container(
        &mut self,
        ui: &mut egui::Ui,
        conn: &Connection,
        app_store: &mut AppStore,
    ) {
        if self.show_group_name_input {
            ui.horizontal(|ui| {
                let container = PLTextInput::new(&mut self.new_group_name)
                    .hint("Enter the API Group Name")
                    .width(ui.available_width() - MARGIN_LARGE)
                    .show(ui);

                container.request_focus();

                let pressed_enter =
                    container.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                if pressed_enter || ui.small_button("✓").clicked() {
                    self.add_new_group_event(conn, app_store);
                }

                if ui.small_button("✗").clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape))
                {
                    self.cancel_new_group_event();
                }
            });
        } else {
            if PLButton::new("Add Group")
                .primary()
                .icon("➕")
                .width(ui.available_width())
                .tooltip("Click to add new API group")
                .show(ui)
                .clicked()
            {
                self.new_group_name.clear();
                self.show_group_name_input = true;
            }
        }
    }

    fn render_groups_list(
        &mut self,
        ui: &mut egui::Ui,
        conn: &Connection,
        app_store: &mut AppStore,
    ) {
        egui::ScrollArea::vertical()
            .id_salt("api_groups_list")
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                if app_store.api_groups_store.api_groups.is_empty() {
                    ui.add_space(SECTION_SPACING);
                    PLLabel::field("No API groups found.")
                        .italic()
                        .color(ui.visuals().weak_text_color())
                        .show(ui);
                    return;
                }

                // ── Clone both upfront to avoid simultaneous borrow of app_store ──
                let groups: Vec<ApiGroup> = app_store.api_groups_store.api_groups.clone();

                let routes = app_store.routes_store.routes.clone();

                for group in &groups {
                    // ── Owned Vec avoids lifetime tie to local `routes` ──
                    let group_routes: Vec<MockedApiRoute> =
                        routes.get(&group.id).cloned().unwrap_or_default();

                    self.render_api_group(ui, conn, group, &group_routes, app_store);
                }
            });
    }

    fn render_api_group(
        &mut self,
        ui: &mut egui::Ui,
        conn: &Connection,
        group: &ApiGroup,
        group_routes: &[MockedApiRoute],
        app_store: &mut AppStore,
    ) {
        // ── Manual header row: triangle + name left, "+" button right ──────────
        // CollapsingHeader must NOT go inside ui.horizontal() — it is a block
        // widget that manages its own layout. Instead we use egui memory to track
        // open state ourselves and draw the "+" button in the same row manually.

        let header_id = egui::Id::new("group_open").with(group.id.as_str());

        let is_open = ui.memory(|m| m.data.get_temp::<bool>(header_id).unwrap_or(true));

        // ── Full-width header row ─────────────────────────────────────────────
        let available_width = ui.available_width();
        let (header_rect, _) = ui.allocate_exact_size(
            egui::vec2(available_width, DEFAULT_MARGIN),
            egui::Sense::hover(),
        );

        // Toggle region — everything except the "+" button (last 28px)
        let toggle_rect = egui::Rect::from_min_max(
            header_rect.min,
            egui::pos2(header_rect.right() - 28.0, header_rect.max.y),
        );
        let toggle_response =
            ui.interact(toggle_rect, header_id.with("toggle"), egui::Sense::click());
        if toggle_response.clicked() {
            ui.memory_mut(|m| m.data.insert_temp(header_id, !is_open));
        }

        // "+" button region — rightmost 24px
        let btn_rect = egui::Rect::from_min_max(
            egui::pos2(header_rect.right() - 24.0, header_rect.min.y),
            header_rect.max,
        );
        let btn_response = ui.interact(btn_rect, header_id.with("add_btn"), egui::Sense::click());

        // ── Paint header ──────────────────────────────────────────────────────
        let painter = ui.painter();

        // Triangle indicator
        let arrow = if is_open { "▾" } else { "▸" };
        painter.text(
            header_rect.left_center() + egui::vec2(2.0, 0.0),
            egui::Align2::LEFT_CENTER,
            arrow,
            egui::FontId::proportional(FONT_SIZE_TITLE),
            ui.visuals().widgets.inactive.fg_stroke.color, // ← dim but visible
        );

        // Group name — primary text, correct as-is
        painter.text(
            header_rect.left_center() + egui::vec2(18.0, 0.0),
            egui::Align2::LEFT_CENTER,
            &group.name,
            egui::FontId::proportional(FONT_SIZE_TITLE),
            ui.visuals().text_color(),
        );

        // "+" button
        painter.text(
            btn_rect.center(),
            egui::Align2::CENTER_CENTER,
            "+",
            egui::FontId::proportional(FONT_SIZE_TITLE),
            if btn_response.hovered() {
                ui.visuals().weak_text_color()
            } else {
                ui.visuals().text_color()
            },
        );

        if btn_response.on_hover_text("Add new Route").clicked() {
            self.handle_add_route_event(conn, group.id.as_str(), app_store);
        }

        // ── Routes list ───────────────────────────────────────────────────────
        if is_open {
            ui.indent(format!("api_group_{}", group.id), |ui| {
                self.render_group_routes_list(ui, conn, group_routes, app_store);
            });
        }
    }

    fn render_group_routes_list(
        &self,
        ui: &mut egui::Ui,
        conn: &Connection,
        group_routes: &[MockedApiRoute],
        app_store: &mut AppStore,
    ) {
        let query = self.search_query.to_lowercase();

        let filtered: Vec<&MockedApiRoute> = group_routes
            .iter()
            .filter(|r| {
                query.is_empty()
                    || r.path.to_lowercase().contains(&query)
                    || r.method.to_lowercase().contains(&query)
            })
            .collect();

        if filtered.is_empty() {
            PLLabel::field("No routes yet")
                .italic()
                .color(ui.visuals().weak_text_color())
                .show(ui);
            return;
        }

        for route in filtered {
            self.render_route_row(ui, conn, route, app_store);
            ui.add_space(4.0);
        }
    }

    fn render_route_row(
        &self,
        ui: &mut egui::Ui,
        conn: &Connection,
        route: &MockedApiRoute,
        app_store: &mut AppStore,
    ) {
        let is_selected = app_store
            .details_store
            .selected_route
            .as_ref()
            .map_or(false, |r| r.id == route.id);

        let available_width = ui.available_width();
        let row_id = ui.id().with(route.id.as_str());

        let (row_rect, _) = ui.allocate_exact_size(
            egui::vec2(available_width, DEFAULT_MARGIN),
            egui::Sense::hover(),
        );

        // ── Single interact call — covers both click and hover ────────────────
        let response = ui.interact(row_rect, row_id, egui::Sense::click());
        let is_hovered = response.hovered();
        let visuals = ui.visuals();

        // ── Background ────────────────────────────────────────────────────────
        let bg_color = if is_selected {
            visuals.selection.bg_fill
        } else if is_hovered {
            visuals.widgets.hovered.bg_fill
        } else {
            Color32::TRANSPARENT
        };

        ui.painter()
            .rect_filled(row_rect, egui::CornerRadius::same(4), bg_color);

        // ── Content ───────────────────────────────────────────────────────────
        let text_color = if is_selected || is_hovered {
            visuals.widgets.active.fg_stroke.color
        } else {
            visuals.widgets.inactive.fg_stroke.color
        };

        let mut child_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(row_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );

        child_ui.horizontal(|ui| {
            ui.label(RichText::new(&route.path).size(FONT_SIZE).color(text_color));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let (bg, fg) = method_tag_colors(&route.method);
                egui::Frame::new()
                    .fill(bg)
                    .corner_radius(egui::CornerRadius::same(4))
                    .inner_margin(egui::Margin::symmetric(6, 2))
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(&route.method)
                                .size(FONT_SIZE_SM)
                                .strong()
                                .color(fg),
                        );
                    });
            });
        });

        // ── Click handler ─────────────────────────────────────────────────────
        if response.clicked() {
            match app_store.on_select_route(conn, route.id.as_str()) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("[sidebar] Failed to select route {}: {e}", route.id);
                    app_store.details_store.selected_route_id = None;
                }
            }
        }

        // ── Cursor ────────────────────────────────────────────────────────────
        if is_hovered {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
    }
}

// ── DB / Event Handlers ────────────────────────────────────────────────────────
impl SidebarPanel {
    fn add_new_group_event(&mut self, conn: &Connection, app_store: &mut AppStore) {
        // ── Must own the string before mutably borrowing app_store ────────────
        let group_name = self.new_group_name.trim().to_string(); // ← fix: .to_string()
        if !group_name.is_empty() {
            match app_store
                .api_groups_store
                .create_api_group(conn, &group_name)
            {
                Ok(_) => {
                    self.show_group_name_input = false;
                    self.new_group_name.clear();
                }
                Err(e) => eprintln!("Failed to save API group: {e}"),
            }
        }
    }

    fn cancel_new_group_event(&mut self) {
        self.show_group_name_input = false;
        self.new_group_name.clear();
    }

    fn handle_add_route_event(
        &self,
        conn: &Connection,
        parent_group_id: &str,
        app_store: &mut AppStore,
    ) {
        match app_store
            .routes_store
            .add_mocked_route(conn, parent_group_id)
        {
            Ok(_) => app_store.refresh_store(conn),
            Err(e) => eprintln!("[sidebar] Failed to create route: {e}"),
        }
    }
}
