// branding/pl_frame.rs
use crate::globals::{CARD_PAD, Color, RADIUS_LG, RADIUS_MD, SECTION_PAD};
use egui::{Response, Ui};

pub struct PLCard;

impl PLCard {
    pub fn show(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) -> Response {
        egui::Frame::new()
            .fill(Color::BG_SURFACE)
            .corner_radius(egui::CornerRadius::same(RADIUS_LG as u8))
            .inner_margin(egui::Margin::same(CARD_PAD as i8))
            .stroke(egui::Stroke::new(1.0, Color::BORDER))
            .show(ui, add_contents)
            .response
    }

    pub fn elevated(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) -> Response {
        egui::Frame::new()
            .fill(Color::BG_ELEVATED)
            .corner_radius(egui::CornerRadius::same(RADIUS_MD as u8))
            .inner_margin(egui::Margin::same(SECTION_PAD as i8))
            .stroke(egui::Stroke::new(1.0, Color::BORDER))
            .show(ui, add_contents)
            .response
    }
}
