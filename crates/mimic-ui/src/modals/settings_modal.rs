use pl_components::{
    components::pl_section_label::PLSectionLabel,
    globals::{FONT_SM, SPACE_LG, SPACE_MD, SPACE_SM, SPACE_XS},
    panels::{panel_header::PLPanelHeader, panel_layout_row::PLPanelRow},
    theme::AppTheme,
    PLButton, PLButtonVariant, PLLabel, PLTextInput,
};
use rusqlite::Connection;

use crate::store::settings_store::SettingsStore;

#[derive(Default)]
pub struct SettingsModal {
    is_open: bool,
}

impl SettingsModal {
    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        conn: &Connection,
        settings: &mut SettingsStore,
    ) -> Option<AppTheme> {
        if !self.is_open {
            return None;
        }

        let mut theme_changed: Option<AppTheme> = None;

        egui::Window::new("settings_modal")
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .movable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .fixed_size([360.0, 0.0])
            .frame(
                egui::Frame::new()
                    .fill(ui.visuals().faint_bg_color)
                    .corner_radius(egui::CornerRadius::same(0))
                    .stroke(egui::Stroke::new(1.0, ui.visuals().code_bg_color))
                    .inner_margin(egui::Margin::same(SPACE_LG as i8)),
            )
            .show(ui.ctx(), |ui| {
                self.render_header(ui, conn, settings, &mut theme_changed);
                self.render_appearance_section(ui, conn, settings, &mut theme_changed);
                self.render_server_section(ui, conn, settings);
                // self.render_system_section(ui, conn, settings);
                ui.add_space(SPACE_LG);
                self.render_footer(ui);
            });

        theme_changed
    }
}

// Private methods
impl SettingsModal {
    fn render_header(
        &mut self,
        ui: &mut egui::Ui,
        conn: &Connection,
        settings: &mut SettingsStore,
        on_theme_changed: &mut Option<AppTheme>,
    ) {
        PLPanelHeader::new("⚙  Settings")
            .subtitle("Change preferences of the application")
            .right(|ui| {
                if PLButton::new("")
                    .ghost()
                    .icon("X")
                    .icon_only()
                    .show(ui)
                    .clicked()
                {
                    self.is_open = false;
                }
                if PLButton::new("")
                    .purple()
                    .icon("↺")
                    .tooltip("Reset settings")
                    .icon_only()
                    .show(ui)
                    .clicked()
                {
                    settings.reset_settings(conn);
                    *on_theme_changed = Some(settings.theme.into());
                }
            })
            .show(ui);
    }

    fn render_appearance_section(
        &mut self,
        ui: &mut egui::Ui,
        conn: &Connection,
        settings: &mut SettingsStore,
        on_theme_changed: &mut Option<AppTheme>,
    ) {
        PLSectionLabel::new("Appearance").show(ui);
        PLLabel::field("Choose your preferred appearance")
            .color(ui.visuals().weak_text_color())
            .show(ui);
        ui.add_space(SPACE_SM);
        PLPanelRow::new("Theme").show(ui, |ui| {
            let light_variant = if settings.theme == AppTheme::Light {
                PLButtonVariant::Primary
            } else {
                PLButtonVariant::Subtle
            };
            if PLButton::new("Light")
                .set_variant(light_variant)
                .icon("☀")
                .enabled(settings.theme != AppTheme::Light)
                .show(ui)
                .clicked()
            {
                settings.apply_theme(conn, AppTheme::Light);
                *on_theme_changed = Some(AppTheme::Light);
            }

            ui.add_space(SPACE_XS);

            // Dark button
            let dark_variant = if settings.theme == AppTheme::Dark {
                PLButtonVariant::Primary
            } else {
                PLButtonVariant::Subtle
            };
            if PLButton::new("Dark")
                .set_variant(dark_variant)
                .icon("🌙")
                .enabled(settings.theme != AppTheme::Dark)
                .show(ui)
                .clicked()
            {
                settings.apply_theme(conn, AppTheme::Dark);
                *on_theme_changed = Some(AppTheme::Dark);
            }
        });
        ui.add_space(SPACE_MD);
    }

    fn render_server_section(
        &mut self,
        ui: &mut egui::Ui,
        conn: &Connection,
        settings: &mut SettingsStore,
    ) {
        PLSectionLabel::new("Server").show(ui);
        PLLabel::field("Changes take effect after restarting the server.")
            .color(ui.visuals().weak_text_color())
            .show(ui);
        ui.add_space(SPACE_SM);
        self.render_port_field(ui, conn, settings);
        ui.add_space(SPACE_SM);
        self.render_prefix_field(ui, conn, settings);
    }

    fn render_port_field(
        &mut self,
        ui: &mut egui::Ui,
        conn: &Connection,
        settings: &mut SettingsStore,
    ) {
        PLPanelRow::new("Port").show(ui, |ui| {
            if PLButton::new("Apply").primary().show(ui).clicked() {
                settings.apply_port(conn);
            }

            ui.add_space(SPACE_XS);

            ui.add_sized([72.0, 26.0], |ui: &mut egui::Ui| {
                PLTextInput::new(&mut settings.port_input)
                    .hint("8080")
                    .show(ui)
            });
        });
        if let Some(ref err) = settings.port_error {
            ui.add_space(SPACE_XS);
            PLLabel::error(err.as_str()).size(FONT_SM).show(ui);
        }
    }

    fn render_prefix_field(
        &mut self,
        ui: &mut egui::Ui,
        conn: &Connection,
        settings: &mut SettingsStore,
    ) {
        PLPanelRow::new("API Prefix")
            .description("Prefix all the API routes")
            .show(ui, |ui| {
                if PLButton::new("Apply").primary().show(ui).clicked() {
                    settings.apply_prefix(conn);
                }

                ui.add_space(SPACE_XS);

                ui.add_sized([120.0, 26.0], |ui: &mut egui::Ui| {
                    PLTextInput::new(&mut settings.prefix_input)
                        .hint("/api/v1")
                        .show(ui)
                });
            });
        if let Some(ref err) = settings.prefix_error {
            ui.add_space(SPACE_XS);
            PLLabel::error(err.as_str()).size(FONT_SM).show(ui);
        }
    }

    // fn render_system_section(
    //     &mut self,
    //     ui: &mut egui::Ui,
    //     conn: &Connection,
    //     settings: &mut SettingsStore,
    // ) {
    //     PLSectionLabel::new("System").show(ui);
    //     PLLabel::accent("Phase 2 - In progress")
    //         .color(ui.visuals().weak_text_color())
    //         .show(ui);
    //     ui.add_space(SPACE_SM);
    //     PLPanelRow::new("Run in Background")
    //         .description("Keep server running in the taskbar when window is closed.")
    //         .disabled() // ← Phase 2 — greys out label + control
    //         .show(ui, |ui| {
    //             ui.add(egui::Checkbox::new(&mut settings.run_in_bg, ""));
    //         });

    //     ui.add_space(SPACE_SM);

    //     // Phase 2 badge
    //     egui::Frame::new()
    //         .fill(Color::BG_SURFACE)
    //         .corner_radius(egui::CornerRadius::same(4))
    //         .inner_margin(egui::Margin::symmetric(
    //             BTN_PAD_H_SM as i8,
    //             BTN_PAD_V_SM as i8,
    //         ))
    //         .show(ui, |ui| {
    //             PLLabel::accent("Coming in Phase 2")
    //                 .size(FONT_SM)
    //                 .color(Color::TEXT_DIM)
    //                 .show(ui);
    //         });

    //     ui.add_space(SPACE_LG);
    // }

    fn render_footer(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.horizontal(|ui| {
            PLLabel::body("MimicServer")
                .size(FONT_SM)
                .color(ui.visuals().text_color())
                .show(ui);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                PLLabel::body(env!("CARGO_PKG_VERSION"))
                    .size(FONT_SM)
                    .color(ui.visuals().text_color())
                    .show(ui);
            });
        });
    }
}
