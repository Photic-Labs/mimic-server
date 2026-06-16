// branding/pl_input.rs
use crate::{
    PLLabel,
    globals::{Color, RADIUS_MD},
};
use egui::{Frame, Response, Ui};

// ─────────────────────────────────────────────────────────────────────────────
// Size token
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum InputSize {
    Sm,
    #[default]
    Md,
    Lg,
}

struct SizeTokens {
    pad_h: f32,
    pad_v: f32,
    icon_size: f32,
    icon_gap: f32,
}

impl InputSize {
    fn tokens(self) -> SizeTokens {
        match self {
            InputSize::Sm => SizeTokens {
                pad_h: 8.0,
                pad_v: 4.0,
                icon_size: 12.0,
                icon_gap: 6.0,
            },
            InputSize::Md => SizeTokens {
                pad_h: 12.0,
                pad_v: 7.0,
                icon_size: 14.0,
                icon_gap: 8.0,
            },
            InputSize::Lg => SizeTokens {
                pad_h: 16.0,
                pad_v: 10.0,
                icon_size: 16.0,
                icon_gap: 10.0,
            },
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Icon
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Clone)]
struct Icon {
    glyph: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Component
// ─────────────────────────────────────────────────────────────────────────────
pub struct PLTextInput<'a> {
    value: &'a mut String,
    hint: Option<String>,
    label: Option<String>,
    icon_leading: Option<Icon>,
    icon_trailing: Option<Icon>,
    width: Option<f32>,
    size: InputSize,
    monospace: bool,
    password: bool,
    multiline: bool,
    rows: usize,
    disabled: bool,
}

impl<'a> PLTextInput<'a> {
    pub fn new(value: &'a mut String) -> Self {
        Self {
            value,
            hint: None,
            label: None,
            icon_leading: None,
            icon_trailing: None,
            width: None,
            size: InputSize::Md,
            monospace: false,
            password: false,
            multiline: false,
            rows: 4,
            disabled: false,
        }
    }

    pub fn hint(mut self, h: impl Into<String>) -> Self {
        self.hint = Some(h.into());
        self
    }
    pub fn label(mut self, l: impl Into<String>) -> Self {
        self.label = Some(l.into());
        self
    }
    pub fn icon_leading(mut self, glyph: impl Into<String>) -> Self {
        self.icon_leading = Some(Icon {
            glyph: glyph.into(),
        });
        self
    }
    pub fn icon_trailing(mut self, glyph: impl Into<String>) -> Self {
        self.icon_trailing = Some(Icon {
            glyph: glyph.into(),
        });
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
    pub fn size(mut self, s: InputSize) -> Self {
        self.size = s;
        self
    }
    pub fn monospace(mut self) -> Self {
        self.monospace = true;
        self
    }
    pub fn password(mut self) -> Self {
        self.password = true;
        self
    }
    pub fn multiline(mut self, rows: usize) -> Self {
        self.multiline = true;
        self.rows = rows;
        self
    }
    pub fn disabled(mut self, yes: bool) -> Self {
        self.disabled = yes;
        self
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Render
    // ─────────────────────────────────────────────────────────────────────────
    pub fn show(self, ui: &mut Ui) -> Response {
        let tok = self.size.tokens();
        let vis = ui.visuals().clone(); // ← single source of truth, theme-aware

        // ── Resolve colors from visuals — adapts to dark AND light ───────────
        let fill = if self.disabled {
            vis.widgets.noninteractive.bg_fill // dim, non-interactive bg
        } else {
            vis.extreme_bg_color // input/textbox bg
        };

        let text_color = if self.disabled {
            vis.widgets.noninteractive.fg_stroke.color // dim text
        } else {
            vis.widgets.inactive.fg_stroke.color // normal input text
        };

        let hint_color = if self.disabled {
            vis.widgets.noninteractive.fg_stroke.color
        } else {
            // Placeholder: midpoint between text and bg — visually recessed
            // Use the noninteractive fg which is always dimmer than inactive
            vis.widgets.noninteractive.fg_stroke.color
        };

        let border_color = if self.disabled {
            vis.widgets.noninteractive.bg_stroke.color // dim border
        } else {
            vis.widgets.inactive.bg_stroke.color // normal border
        };

        let label_color = if self.disabled {
            vis.widgets.noninteractive.fg_stroke.color
        } else {
            vis.widgets.active.text_color()
        };

        // ── Outer width ──────────────────────────────────────────────────────
        let outer_width = self.width.unwrap_or_else(|| ui.available_width());

        // ── Icon slot widths ─────────────────────────────────────────────────
        let leading_slot = self
            .icon_leading
            .as_ref()
            .map(|_| tok.icon_size + tok.icon_gap)
            .unwrap_or(0.0);
        let trailing_slot = self
            .icon_trailing
            .as_ref()
            .map(|_| tok.icon_size + tok.icon_gap)
            .unwrap_or(0.0);

        let edit_width = outer_width - (tok.pad_h * 2.0) - leading_slot - trailing_slot;

        // ─────────────────────────────────────────────────────────────────────
        // Layout
        // ─────────────────────────────────────────────────────────────────────
        let outer_response = ui.vertical(|ui| {
            // ── Field label ──────────────────────────────────────────────────
            if let Some(lbl) = &self.label {
                PLLabel::body(lbl).color(label_color).show(ui);
                ui.add_space(4.0);
            }

            // ── Frame ────────────────────────────────────────────────────────
            let frame_resp = egui::Frame::new()
                .fill(fill)
                .corner_radius(egui::CornerRadius::same(RADIUS_MD as u8))
                .stroke(egui::Stroke::new(1.0, border_color))
                .inner_margin(egui::Margin {
                    left: tok.pad_h as i8,
                    right: tok.pad_h as i8,
                    top: tok.pad_v as i8,
                    bottom: tok.pad_v as i8,
                })
                .show(ui, |ui| {
                    ui.set_width(outer_width - (tok.pad_h * 2.0));

                    let edit_response = ui
                        .horizontal(|ui| {
                            if self.disabled {
                                ui.disable();
                            }
                            ui.spacing_mut().item_spacing.x = 0.0;

                            // Leading icon
                            if let Some(icon) = &self.icon_leading {
                                ui.label(
                                    egui::RichText::new(&icon.glyph)
                                        .size(tok.icon_size)
                                        .color(hint_color),
                                );
                                ui.add_space(tok.icon_gap);
                            }

                            // TextEdit
                            let edit_resp = if self.multiline {
                                let mut w = egui::TextEdit::multiline(self.value)
                                    .desired_width(edit_width)
                                    .desired_rows(self.rows)
                                    .frame(Frame::new())
                                    .text_color(text_color)
                                    .font(egui::TextStyle::Monospace);
                                if let Some(h) = &self.hint {
                                    w = w.hint_text(egui::RichText::new(h).color(hint_color));
                                }
                                ui.add(w)
                            } else {
                                let font = if self.monospace {
                                    egui::TextStyle::Monospace
                                } else {
                                    egui::TextStyle::Body
                                };
                                let mut w = egui::TextEdit::singleline(self.value)
                                    .desired_width(edit_width)
                                    .frame(Frame::new())
                                    .text_color(text_color)
                                    .font(font);
                                if let Some(h) = &self.hint {
                                    w = w.hint_text(egui::RichText::new(h).color(hint_color));
                                }
                                if self.password {
                                    w = w.password(true);
                                }
                                ui.add(w)
                            };

                            // Trailing icon
                            if let Some(icon) = &self.icon_trailing {
                                ui.add_space(tok.icon_gap);
                                ui.label(
                                    egui::RichText::new(&icon.glyph)
                                        .size(tok.icon_size)
                                        .color(hint_color),
                                );
                            }

                            edit_resp
                        })
                        .inner;

                    edit_response
                });

            // ── Focus ring ───────────────────────────────────────────────────
            if frame_resp.inner.has_focus() && !self.disabled {
                ui.painter().rect_stroke(
                    frame_resp.response.rect,
                    egui::CornerRadius::same(RADIUS_MD as u8),
                    egui::Stroke::new(1.5, Color::BORDER_FOCUS),
                    egui::StrokeKind::Outside,
                );
            }

            frame_resp.inner
        });

        outer_response.inner
    }
}
