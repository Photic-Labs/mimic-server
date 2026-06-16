// branding/pl_badge.rs
use crate::globals::{BADGE_PAD_H, BADGE_PAD_V, Color, FONT_SM, RADIUS_SM};
use egui::{Color32, Response, RichText, Ui};

// ── Internal color mode ───────────────────────────────────────────────────────
enum BadgeMode {
    /// Caller-supplied explicit colors — method badges, semantic badges
    Explicit {
        text: String,
        bg: Color32,
        fg: Color32,
    },

    /// Neutral count pill — bg/fg resolved from visuals at render time
    Count { text: String },

    /// Brand accent — blue fill, white text — intentional in both themes
    Accent { text: String },
}

pub struct PLBadge {
    mode: BadgeMode,
}

impl PLBadge {
    // ── Constructors ──────────────────────────────────────────────────────────

    /// Explicit bg + fg — use for method badges, status badges.
    /// Caller owns the semantic color meaning.
    pub fn new(text: impl Into<String>, bg: Color32, fg: Color32) -> Self {
        Self {
            mode: BadgeMode::Explicit {
                text: text.into(),
                bg,
                fg,
            },
        }
    }

    /// Neutral count pill — adapts to dark/light theme automatically.
    pub fn count(count: usize) -> Self {
        Self {
            mode: BadgeMode::Count {
                text: count.to_string(),
            },
        }
    }

    /// Brand blue accent badge — intentional brand color, same in both themes.
    pub fn accent(text: impl Into<String>) -> Self {
        Self {
            mode: BadgeMode::Accent { text: text.into() },
        }
    }

    // ── Render ────────────────────────────────────────────────────────────────
    pub fn show(self, ui: &mut Ui) -> Response {
        // ── Resolve text + colors — deferred to render time ───────────────────
        let (text, bg, fg) = match self.mode {
            BadgeMode::Explicit { text, bg, fg } => (text, bg, fg),

            BadgeMode::Count { text } => {
                let vis = ui.visuals();
                (
                    text,
                    // Dark:  BG_ELEVATED — rgb(24,25,32)  — dark surface
                    // Light: inactive.bg_fill — rgb(255,255,255) — white surface
                    vis.widgets.inactive.bg_fill,
                    // Dark:  TEXT_SECONDARY — rgb(160,165,185) — dim grey
                    // Light: rgb(18,22,42)  — near black
                    vis.widgets.inactive.fg_stroke.color,
                )
            }

            BadgeMode::Accent { text } => (
                text,
                Color::BRAND_BLUE, // #2563FF — intentional brand, same in both themes
                Color32::WHITE,    // always white on blue fill — readable in both themes
            ),
        };

        // ── Paint ─────────────────────────────────────────────────────────────
        egui::Frame::new()
            .fill(bg)
            .corner_radius(egui::CornerRadius::same(RADIUS_SM as u8))
            .inner_margin(egui::Margin::symmetric(
                BADGE_PAD_H as i8,
                BADGE_PAD_V as i8,
            ))
            .show(ui, |ui| {
                ui.label(RichText::new(&text).size(FONT_SM).strong().color(fg));
            })
            .response
    }
}
