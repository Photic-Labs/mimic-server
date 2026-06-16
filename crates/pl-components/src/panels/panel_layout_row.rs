use egui::Ui;

use crate::{
    PLLabel,
    globals::{FONT_MD, FONT_SM, SPACE_XS},
};

/// A single panel row: label + description on the left, control on the right.
///
/// Usage:
///   PLPanelRow::new("Theme")
///       .description("Choose your preferred appearance")
///       .show(ui, |ui| {
///           // right side — your control goes here
///       });
pub struct PLPanelRow<'a> {
    label: &'a str,
    description: Option<&'a str>,
    disabled: bool,
}

impl<'a> PLPanelRow<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            description: None,
            disabled: false,
        }
    }

    pub fn description(mut self, text: &'a str) -> Self {
        self.description = Some(text);
        self
    }

    /// Dims the label and description — used for Phase 2 disabled items.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn show<CFn: FnOnce(&mut Ui)>(self, ui: &mut Ui, control: CFn) {
        let label_color = if self.disabled {
            ui.visuals().weak_text_color()
        } else {
            ui.visuals().text_color()
        };

        ui.horizontal(|ui| {
            // ── Left — label + description ────────────────────────────────────
            ui.vertical(|ui| {
                PLLabel::field(self.label)
                    .size(FONT_MD)
                    .color(label_color)
                    .show(ui);

                if let Some(desc) = self.description {
                    ui.add_space(SPACE_XS);
                    PLLabel::field(desc)
                        .size(FONT_SM)
                        .color(label_color)
                        .show(ui);
                }
            });

            // ── Right — control ───────────────────────────────────────────────
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if self.disabled {
                    ui.disable();
                }
                control(ui);
            });
        });
    }
}
