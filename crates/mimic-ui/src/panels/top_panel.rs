// mimic_ui/src/panels/top_bar_panel.rs

use crate::store::server_store::ServerStatus;
use egui::{CornerRadius, FontFamily, FontId, Margin, RichText, Sense};
use pl_components::{
    globals::{FONT_LG, FONT_XL, INPUT_PAD_H, MARGIN_X},
    Color, PLButton,
};

const BAR_HEIGHT: f32 = 48.0;
const ICON_SIZE: f32 = 32.0;

#[derive(Default)]
pub struct TopBarPanel {}

// ── Public ────────────────────────────────────────────────────────────────────

impl TopBarPanel {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        server_status: &ServerStatus,
        on_start: &mut bool,
        on_stop: &mut bool,
        on_settings_toggle: &mut bool,
    ) {
        egui::Frame::new()
            .inner_margin(Margin {
                left: MARGIN_X,
                right: MARGIN_X,
                top: 0,
                bottom: 0,
            })
            .show(ui, |ui| {
                // ── Reserve the exact bar height ──────────────────────────────
                // This is the key — we tell egui exactly how tall this row is
                // BEFORE drawing anything. All children then center against this.
                let (bar_rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), BAR_HEIGHT),
                    Sense::hover(),
                );

                // ── Left: brand — pinned to vertical center of bar_rect ───────
                let left_rect = egui::Rect::from_min_size(
                    bar_rect.min,
                    egui::vec2(bar_rect.width() * 0.5, BAR_HEIGHT),
                );

                let mut left_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(left_rect)
                        .layout(egui::Layout::left_to_right(egui::Align::Center)),
                );
                self.render_brand(&mut left_ui);

                // ── Right: controls — pinned to vertical center of bar_rect ───
                let right_rect = egui::Rect::from_min_size(
                    bar_rect.min,
                    egui::vec2(bar_rect.width(), BAR_HEIGHT),
                );

                let mut right_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(right_rect)
                        .layout(egui::Layout::right_to_left(egui::Align::Center)),
                );
                if PLButton::new("⚙")
                    .icon_only()
                    .purple()
                    .show(&mut right_ui)
                    .clicked()
                {
                    *on_settings_toggle = true;
                }
                self.render_controls(&mut right_ui, server_status, on_start, on_stop);
            });
    }
}

// ── Brand ─────────────────────────────────────────────────────────────────────

impl TopBarPanel {
    fn render_brand(&self, ui: &mut egui::Ui) {
        ui.spacing_mut().item_spacing.x = 0.0;

        self.render_brand_icon(ui);
        ui.add_space(INPUT_PAD_H);
        self.render_app_title(ui);
    }

    fn render_brand_icon(&self, ui: &mut egui::Ui) {
        // allocate_exact_size respects the parent layout's Align::Center
        // so this rect is already vertically centered inside bar_rect
        let (icon_rect, _) =
            ui.allocate_exact_size(egui::vec2(ICON_SIZE, ICON_SIZE), Sense::hover());

        if ui.is_rect_visible(icon_rect) {
            // Blue rounded square
            ui.painter()
                .rect_filled(icon_rect, CornerRadius::same(7), Color::BRAND_BLUE);

            // 🌐 emoji — Monospace so egui picks up the emoji glyph correctly
            ui.painter().text(
                icon_rect.center(),
                egui::Align2::CENTER_CENTER,
                "🌐",
                FontId::new(FONT_XL, FontFamily::Monospace),
                Color::TEXT_PRIMARY,
            );
        }
    }

    fn render_app_title(&self, ui: &mut egui::Ui) {
        // "Mimic" — off white bold
        ui.label(
            RichText::new("Mimic")
                .size(FONT_XL)
                .strong()
                .color(ui.visuals().text_color())
                .family(FontFamily::Name("Bold".into())),
        );

        ui.add_space(4.0);

        // "Server" — brand blue bold
        ui.label(
            RichText::new("Server")
                .size(FONT_XL)
                .strong()
                .color(Color::BRAND_BLUE)
                .family(FontFamily::Name("Bold".into())),
        );

        ui.add_space(12.0);

        // ── Vertical divider ──────────────────────────────────────────────────
        // Paint it relative to the current cursor, centered on ICON_SIZE
        let cursor = ui.cursor();
        let center_y = cursor.min.y + (ICON_SIZE / 2.0);

        ui.painter().line_segment(
            [
                egui::pos2(cursor.min.x, center_y - 11.0),
                egui::pos2(cursor.min.x, 48.0),
            ],
            egui::Stroke::new(1.0, Color::BORDER),
        );

        // Advance cursor past the 1px divider line
        ui.allocate_exact_size(egui::vec2(1.0, ICON_SIZE), Sense::hover());

        ui.add_space(12.0);

        // Subtitle
        ui.label(
            RichText::new("API Mock Engine")
                .size(FONT_LG)
                .color(Color::TEXT_DIM)
                .family(FontFamily::Proportional),
        );
    }
}

// ── Controls ──────────────────────────────────────────────────────────────────

impl TopBarPanel {
    fn render_controls(
        &self,
        ui: &mut egui::Ui,
        status: &ServerStatus,
        on_start: &mut bool,
        on_stop: &mut bool,
    ) {
        ui.spacing_mut().item_spacing.x = 8.0;

        match status {
            // ── Stopped ───────────────────────────────────────────────────────
            ServerStatus::Stopped => {
                if PLButton::new("Start Server")
                    .icon("▶")
                    .primary()
                    .font_size(FONT_LG)
                    .width(130.0)
                    .show(ui)
                    .clicked()
                {
                    *on_start = true;
                }
            }

            // ── Starting ──────────────────────────────────────────────────────
            ServerStatus::Starting => {
                PLButton::new("Starting...")
                    .icon("⏳")
                    .ghost()
                    .font_size(FONT_LG)
                    .width(130.0)
                    .enabled(false)
                    .show(ui);
            }

            // ── Running ───────────────────────────────────────────────────────
            ServerStatus::Running { port } => {
                // Stop button
                if PLButton::new("Stop")
                    .icon("■")
                    .danger()
                    .font_size(FONT_LG)
                    .width(130.0)
                    .show(ui)
                    .clicked()
                {
                    *on_stop = true;
                }

                ui.add_space(4.0);

                // Port badge
                egui::Frame::new()
                    .fill(Color::BG_ELEVATED)
                    .inner_margin(Margin::symmetric(8, 5))
                    .stroke(egui::Stroke::new(1.0, Color::BORDER))
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(format!("localhost:{port}"))
                                .size(FONT_LG)
                                .color(Color::TEXT_SECONDARY)
                                .family(FontFamily::Monospace),
                        );
                    });

                ui.add_space(4.0);

                self.render_status_pill(
                    ui,
                    "Running",
                    Color::SUCCESS,
                    egui::Color32::from_rgba_unmultiplied(34, 197, 94, 25),
                );
            }

            // ── Error ─────────────────────────────────────────────────────────
            ServerStatus::Error(msg) => {
                if PLButton::new("Retry")
                    .icon("↺")
                    .primary()
                    .font_size(FONT_LG)
                    .width(130.0)
                    .show(ui)
                    .clicked()
                {
                    *on_start = true;
                }

                ui.add_space(4.0);

                egui::Frame::new()
                    .outer_margin(Margin::symmetric(10, 5))
                    .inner_margin(Margin::symmetric(10, 5))
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.x = 6.0;
                        self.render_dot(ui, Color::DANGER);
                        ui.label(
                            RichText::new("Error")
                                .size(FONT_LG)
                                .strong()
                                .color(Color::DANGER),
                        )
                        .on_hover_text(msg.as_str());
                    });
            }
        }
    }

    fn render_status_pill(
        &self,
        ui: &mut egui::Ui,
        label: &str,
        color: egui::Color32,
        bg_color: egui::Color32,
    ) {
        egui::Frame::new()
            .fill(bg_color)
            .inner_margin(Margin::symmetric(10, 5))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                self.render_dot(ui, color);
                ui.label(RichText::new(label).size(FONT_LG).strong().color(color));
            });
    }

    fn render_dot(&self, ui: &mut egui::Ui, color: egui::Color32) {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(8.0, 8.0), Sense::hover());

        if ui.is_rect_visible(rect) {
            ui.painter().circle_filled(
                rect.center(),
                4.0,
                egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 50),
            );
            ui.painter().circle_filled(rect.center(), 2.5, color);
        }
    }
}
