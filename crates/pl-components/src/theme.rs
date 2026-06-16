use crate::{Color, globals::RADIUS_MD};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AppTheme {
    #[default]
    Dark,
    Light,
}

impl AppTheme {
    pub fn to_visuals(&self) -> egui::Visuals {
        match self {
            AppTheme::Dark => Self::dark_visuals(),
            AppTheme::Light => Self::light_visuals(),
        }
    }

    fn dark_visuals() -> egui::Visuals {
        let mut v = egui::Visuals::dark();

        // ── Backgrounds — this is what fixes your screenshot ──────────────────
        v.window_fill = Color::BG_APP; // #0D0E11 — outermost window
        v.panel_fill = Color::BG_PANEL; // #111217 — ALL panels uniform
        v.faint_bg_color = Color::BG_SURFACE; // subtle alternating rows
        v.extreme_bg_color = Color::BG_ELEVATED; // inputs, text areas

        // ── Widgets ───────────────────────────────────────────────────────────
        v.widgets.noninteractive.bg_fill = Color::BG_PANEL;
        v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, Color::TEXT_DIM);
        v.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, Color::BORDER);

        v.widgets.inactive.bg_fill = Color::BG_ELEVATED;
        v.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Color::TEXT_SECONDARY);
        v.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, Color::BORDER);

        v.widgets.hovered.bg_fill = Color::BG_HOVER;
        v.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, Color::TEXT_PRIMARY);
        v.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, Color::BRAND_BLUE);

        v.widgets.active.bg_fill = Color::BG_SELECTED;
        v.widgets.active.fg_stroke = egui::Stroke::new(1.0, Color::TEXT_PRIMARY);
        v.widgets.active.bg_stroke = egui::Stroke::new(1.0, Color::BRAND_BLUE);

        v.widgets.open.bg_fill = Color::BG_SELECTED;
        v.widgets.open.fg_stroke = egui::Stroke::new(1.0, Color::TEXT_PRIMARY);

        // ── Text ──────────────────────────────────────────────────────────────
        v.override_text_color = Some(Color::TEXT_PRIMARY);

        // ── Selection ─────────────────────────────────────────────────────────
        v.selection.bg_fill = Color::BG_SELECTED;
        v.selection.stroke = egui::Stroke::new(1.0, Color::BRAND_BLUE);

        // ── Borders and strokes ───────────────────────────────────────────────
        v.window_stroke = egui::Stroke::new(1.0, Color::BORDER);
        v.hyperlink_color = Color::BRAND_BLUE;
        v.window_corner_radius = egui::CornerRadius::same(RADIUS_MD as u8);

        v
    }

    // ── Light ─────────────────────────────────────────────────────────────────
    fn light_visuals() -> egui::Visuals {
        let mut v = egui::Visuals::light();

        // ── Backgrounds ──────────────────────────────────────────────────────
        v.window_fill = egui::Color32::from_rgb(245, 247, 250); // #F5F7FA — outermost
        v.panel_fill = egui::Color32::from_rgb(255, 255, 255); // #FFFFFF — panels
        v.faint_bg_color = egui::Color32::from_rgb(240, 242, 248); // subtle alternating rows
        v.extreme_bg_color = egui::Color32::from_rgb(255, 255, 255); // TextEdit, CodeEditor bg

        // ── Widgets ──────────────────────────────────────────────────────────
        // noninteractive — labels, separators, static text
        v.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(255, 255, 255);
        v.widgets.noninteractive.fg_stroke =
            egui::Stroke::new(1.0, egui::Color32::from_rgb(30, 35, 60)); // dark — readable labels
        v.widgets.noninteractive.bg_stroke =
            egui::Stroke::new(1.0, egui::Color32::from_rgb(210, 215, 230));

        // inactive — text inputs, dropdowns at rest
        v.widgets.inactive.bg_fill = egui::Color32::from_rgb(240, 240, 240); // WHITE bg for inputs
        v.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(18, 22, 42)); // near-black text
        v.widgets.inactive.bg_stroke =
            egui::Stroke::new(1.0, egui::Color32::from_rgb(160, 160, 160)); // subtle border

        // hovered
        v.widgets.hovered.bg_fill = egui::Color32::from_rgb(235, 240, 255);
        v.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(18, 22, 42));
        v.widgets.hovered.bg_stroke = egui::Stroke::new(1.5, Color::BRAND_BLUE);

        // active — clicked / focused
        v.widgets.active.bg_fill = egui::Color32::from_rgb(220, 230, 255);
        v.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(18, 22, 42));
        v.widgets.active.bg_stroke = egui::Stroke::new(1.5, Color::BRAND_BLUE);

        // open — combo boxes when expanded
        v.widgets.open.bg_fill = egui::Color32::from_rgb(235, 240, 255);
        v.widgets.open.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(18, 22, 42));
        v.widgets.open.bg_stroke = egui::Stroke::new(1.5, Color::BRAND_BLUE);

        // ── Text ─────────────────────────────────────────────────────────────
        // Single source of truth — near-black on white
        v.override_text_color = Some(egui::Color32::from_rgb(18, 22, 42));

        // ── Selection ────────────────────────────────────────────────────────
        v.selection.bg_fill = egui::Color32::from_rgb(200, 212, 255);
        v.selection.stroke = egui::Stroke::new(1.0, Color::BRAND_BLUE);

        // ── Borders ──────────────────────────────────────────────────────────
        v.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(210, 215, 230));
        v.hyperlink_color = Color::BRAND_BLUE;
        v.window_corner_radius = egui::CornerRadius::same(RADIUS_MD as u8);

        v
    }
}

impl From<String> for AppTheme {
    fn from(theme: String) -> Self {
        match theme.as_str() {
            "dark" => AppTheme::Dark,
            "light" => AppTheme::Light,
            _ => AppTheme::Dark,
        }
    }
}

impl ToString for AppTheme {
    fn to_string(&self) -> String {
        match self {
            AppTheme::Dark => "dark".to_string(),
            AppTheme::Light => "light".to_string(),
        }
    }
}
