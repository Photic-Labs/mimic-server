use crate::globals::{
    BTN_HEIGHT, BTN_HEIGHT_SM, BTN_PAD_H, BTN_PAD_H_SM, BTN_PAD_V, BTN_PAD_V_SM, Color, FONT_MD,
    FONT_SM, RADIUS_SM,
};
use egui::{Color32, CornerRadius, Response, Ui, Vec2};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PLButtonVariant {
    Primary,
    Purple,
    Danger,
    Ghost,
    Subtle,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PLButtonIconPos {
    Leading,
    Trailing,
}

pub struct PLButton {
    label: String,
    variant: PLButtonVariant,
    width: Option<f32>,
    full_width: bool,
    enabled: bool,
    loading: bool,
    tooltip: Option<String>,
    small: bool,
    icon: Option<String>,
    font_size: f32,
    icon_pos: PLButtonIconPos,
    icon_only: bool,
}

impl PLButton {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: PLButtonVariant::Subtle,
            width: None,
            full_width: false,
            enabled: true,
            loading: false,
            tooltip: None,
            small: false,
            icon: None,
            font_size: FONT_MD,
            icon_pos: PLButtonIconPos::Leading,
            icon_only: false,
        }
    }

    // ── Variant builders ─────────────────────────────────────────────────────
    pub fn primary(mut self) -> Self {
        self.variant = PLButtonVariant::Primary;
        self
    }
    pub fn purple(mut self) -> Self {
        self.variant = PLButtonVariant::Purple;
        self
    }
    pub fn danger(mut self) -> Self {
        self.variant = PLButtonVariant::Danger;
        self
    }
    pub fn ghost(mut self) -> Self {
        self.variant = PLButtonVariant::Ghost;
        self
    }
    pub fn subtle(mut self) -> Self {
        self.variant = PLButtonVariant::Subtle;
        self
    }
    pub fn set_variant(mut self, variant: PLButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    // ── Option builders ──────────────────────────────────────────────────────
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }
    pub fn enabled(mut self, v: bool) -> Self {
        self.enabled = v;
        self
    }
    pub fn loading(mut self, v: bool) -> Self {
        self.loading = v;
        self
    }
    pub fn tooltip(mut self, t: impl Into<String>) -> Self {
        self.tooltip = Some(t.into());
        self
    }
    pub fn small(mut self) -> Self {
        self.small = true;
        self
    }
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }
    pub fn icon(mut self, i: impl Into<String>) -> Self {
        self.icon = Some(i.into());
        self
    }
    pub fn icon_trailing(mut self) -> Self {
        self.icon_pos = PLButtonIconPos::Trailing;
        self
    }
    pub fn icon_only(mut self) -> Self {
        self.icon_only = true;
        self
    }

    // ── Render ───────────────────────────────────────────────────────────────
    pub fn show(self, ui: &mut Ui) -> Response {
        let is_interactive = self.enabled && !self.loading;

        // ── Resolve colors — theme-aware, uses ui.visuals() ──────────────────
        let (bg, bg_hov, bg_disabled, fg, fg_disabled, border_color) = self.resolve_colors(ui);

        let font_size = if self.small { FONT_SM } else { self.font_size };
        let pad_h = if self.small { BTN_PAD_H_SM } else { BTN_PAD_H };
        let _pad_v = if self.small { BTN_PAD_V_SM } else { BTN_PAD_V };
        let height = if self.small {
            BTN_HEIGHT_SM
        } else {
            BTN_HEIGHT
        };
        let radius = CornerRadius::same(RADIUS_SM as u8);

        // Build display string ────────────────────────────────────────────────
        let display: String = if self.icon_only {
            self.icon.clone().unwrap_or_else(|| self.label.clone())
        } else {
            match (&self.icon, self.icon_pos) {
                (Some(ic), PLButtonIconPos::Leading) => format!("{} {}", ic, self.label),
                (Some(ic), PLButtonIconPos::Trailing) => format!("{} {}", self.label, ic),
                (None, _) => self.label.clone(),
            }
        };

        // Measure text width ─────────────────────────────────────────────────
        let text_width = ui.fonts_mut(|f| {
            f.layout_no_wrap(display.clone(), egui::FontId::proportional(font_size), fg)
                .size()
                .x
        });

        // Resolve desired size ────────────────────────────────────────────────
        let desired_width = if let Some(w) = self.width {
            w
        } else if self.full_width {
            ui.available_width()
        } else if self.icon_only {
            height
        } else {
            text_width + (pad_h * 2.0)
        };

        let desired = Vec2::new(desired_width, height);

        // Allocate & paint ────────────────────────────────────────────────────
        let response = ui
            .add_enabled_ui(is_interactive, |ui| {
                let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click());

                if ui.is_rect_visible(rect) {
                    let fill = if !is_interactive {
                        bg_disabled
                    } else if response.hovered() {
                        bg_hov
                    } else {
                        bg
                    };

                    // Background
                    ui.painter().rect_filled(rect, radius, fill);

                    // Border — Ghost always; others skip it
                    if self.variant == PLButtonVariant::Ghost {
                        let border = if is_interactive {
                            border_color
                        } else {
                            Color32::from_rgba_unmultiplied(
                                border_color.r(),
                                border_color.g(),
                                border_color.b(),
                                80,
                            )
                        };
                        ui.painter().rect_stroke(
                            rect,
                            radius,
                            egui::Stroke::new(1.0, border),
                            egui::StrokeKind::Inside,
                        );
                    }

                    // Loading spinner
                    if self.loading {
                        let spinner_size = font_size * 0.9;
                        let spinner_rect =
                            egui::Rect::from_center_size(rect.center(), Vec2::splat(spinner_size));
                        egui::Spinner::new()
                            .size(spinner_size)
                            .color(fg_disabled)
                            .paint_at(ui, spinner_rect);
                    } else {
                        let label_color = if is_interactive { fg } else { fg_disabled };
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            &display,
                            egui::FontId::proportional(font_size),
                            label_color,
                        );
                    }
                }

                response
            })
            .inner;

        if response.hovered() && is_interactive {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        match self.tooltip {
            Some(tip) => response.on_hover_text(tip),
            None => response,
        }
    }

    // ── Color resolution — theme-aware ────────────────────────────────────────
    /// Returns `(bg, bg_hov, bg_disabled, fg, fg_disabled, border_color)`.
    ///
    /// Brand-colored variants (Primary, Purple, Danger) keep their explicit
    /// brand fills — they are intentional in both themes.
    /// Neutral variants (Ghost, Subtle) read from `ui.visuals()` so they
    /// automatically adapt to dark / light.
    fn resolve_colors(&self, ui: &Ui) -> (Color32, Color32, Color32, Color32, Color32, Color32) {
        let vis = ui.visuals();

        // fg_disabled and border_color are always theme-derived
        let fg_disabled = vis.widgets.noninteractive.fg_stroke.color;
        let border_color = vis.widgets.inactive.bg_stroke.color;

        let (bg, bg_hov, fg) = match self.variant {
            // ── Brand variants — explicit fills, same in both themes ──────────
            // These are intentional brand colors. White label on colored fill
            // is readable in both dark and light mode.
            PLButtonVariant::Primary => (
                Color::BTN_PRIMARY,     // #2563FF
                Color::BTN_PRIMARY_HOV, // #3B76FF
                Color32::WHITE,         // always white on blue fill
            ),
            PLButtonVariant::Purple => (
                Color::BTN_PURPLE,     // #7C3AED
                Color::BTN_PURPLE_HOV, // #8B4EFF
                Color32::WHITE,        // always white on purple fill
            ),
            PLButtonVariant::Danger => (
                // ── Danger adapts per theme ───────────────────────────────────
                // Dark:  dark red fill, red text  (subtle destructive)
                // Light: light red fill, red text (readable on white bg)
                if vis.dark_mode {
                    Color::BTN_DANGER // rgb(40, 18, 18) — dark red
                } else {
                    Color32::from_rgb(255, 235, 235) // light red tint
                },
                if vis.dark_mode {
                    Color::BTN_DANGER_HOV // rgb(70, 25, 25)
                } else {
                    Color32::from_rgb(255, 210, 210) // deeper red tint on hover
                },
                Color::DANGER, // #EF4444 — red text, readable on both fills
            ),

            // ── Neutral variants — fully theme-derived ────────────────────────
            PLButtonVariant::Ghost => (
                Color32::TRANSPARENT,
                vis.widgets.hovered.bg_fill,          // theme hover bg
                vis.widgets.inactive.fg_stroke.color, // theme text
            ),
            PLButtonVariant::Subtle => (
                vis.widgets.inactive.bg_fill,         // theme surface bg
                vis.widgets.hovered.bg_fill,          // theme hover bg
                vis.widgets.inactive.fg_stroke.color, // theme text
            ),
        };

        // Disabled background: same hue, ~40% opacity
        let bg_disabled =
            Color32::from_rgba_unmultiplied(bg.r(), bg.g(), bg.b(), (bg.a() as f32 * 0.4) as u8);

        (bg, bg_hov, bg_disabled, fg, fg_disabled, border_color)
    }
}
