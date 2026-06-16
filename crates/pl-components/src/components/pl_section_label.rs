// branding/components/pl_section_label.rs

use crate::components::pl_label::PLLabel;
use crate::globals::{FONT_LG, SPACE_SM, SPACE_XS};
use egui::Ui;

/// Dimmed uppercase section divider label.
///
/// Usage:  PLSectionLabel::new("Appearance").show(ui);
pub struct PLSectionLabel<'a> {
    text: &'a str,
}

impl<'a> PLSectionLabel<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    pub fn show(self, ui: &mut Ui) {
        ui.add_space(SPACE_SM);
        PLLabel::body(self.text)
            .size(FONT_LG)
            .bold()
            .color(ui.visuals().strong_text_color())
            .show(ui);
        ui.add_space(SPACE_XS);
    }
}
