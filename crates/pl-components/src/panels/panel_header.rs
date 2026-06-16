use egui::Ui;

use crate::{
    PLLabel,
    globals::{FONT_LG, FONT_SM, SPACE_MD, SPACE_SM},
};

/// Sticky panel header — title left, actions right, separator below.
///
/// Usage:
///   PLPanelHeader::new("⚙  Settings")
///       .right(|ui| { if PLButton::new("✕").show(ui).clicked() { ... } })
///       .show(ui);
pub struct PLPanelHeader<'a, R = fn(&mut Ui)>
where
    R: FnOnce(&mut Ui),
{
    title: &'a str,
    subtitle: Option<&'a str>,
    font_size: f32,
    right_slot: Option<R>,
}

impl<'a> PLPanelHeader<'a, fn(&mut Ui)> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            subtitle: None,
            font_size: FONT_LG,
            right_slot: None,
        }
    }
}

// Builder
impl<'a, R> PLPanelHeader<'a, R>
where
    R: FnOnce(&mut Ui),
{
    pub fn subtitle(mut self, subtitle: &'a str) -> Self {
        self.subtitle = Some(subtitle);
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn right<NR: FnOnce(&mut Ui)>(self, slot: NR) -> PLPanelHeader<'a, NR> {
        PLPanelHeader {
            title: self.title,
            subtitle: self.subtitle,
            font_size: self.font_size,
            right_slot: Some(slot),
        }
    }

    pub fn show(self, ui: &mut Ui) {
        ui.add_space(SPACE_SM);

        ui.horizontal(|ui| {
            // ── Left — title + optional subtitle ─────────────────────────────
            ui.vertical(|ui| {
                PLLabel::heading(self.title)
                    .bold()
                    .color(ui.visuals().text_color())
                    .show(ui);

                if let Some(sub) = self.subtitle {
                    ui.add_space(2.0);
                    PLLabel::body(sub)
                        .size(FONT_SM)
                        .color(ui.visuals().weak_text_color())
                        .show(ui);
                }
            });

            // ── Right — actions slot ──────────────────────────────────────────
            if let Some(slot) = self.right_slot {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), slot);
            }
        });

        ui.add_space(SPACE_SM);
        ui.separator();
        ui.add_space(SPACE_MD);
    }
}
