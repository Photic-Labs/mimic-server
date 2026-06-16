// branding/pl_label.rs
use crate::globals::{Color, FONT_BASE, FONT_LG, FONT_SM, FONT_XL, FONT_XS};
use egui::{Color32, Response, RichText, Ui};

pub struct PLLabel {
    text: String,
    size: f32,
    color: Color32,
    bold: bool,
    italic: bool,
}

impl PLLabel {
    pub fn heading(text: impl Into<String>) -> Self {
        Self::build(text, FONT_XL, Color::TEXT_PRIMARY, true, false)
    }
    pub fn subheading(text: impl Into<String>) -> Self {
        Self::build(text, FONT_LG, Color::TEXT_PRIMARY, true, false)
    }
    pub fn body(text: impl Into<String>) -> Self {
        Self::build(text, FONT_BASE, Color::TEXT_PRIMARY, false, false)
    }
    pub fn field(text: impl Into<String>) -> Self {
        Self::build(text, FONT_SM, Color::TEXT_SECONDARY, false, false)
    }
    pub fn caption(text: impl Into<String>) -> Self {
        Self::build(text, FONT_XS, Color::TEXT_DIM, false, false)
    }
    pub fn empty(text: impl Into<String>) -> Self {
        Self::build(text, FONT_BASE, Color::TEXT_DIM, false, true)
    }
    pub fn accent(text: impl Into<String>) -> Self {
        Self::build(text, FONT_BASE, Color::TEXT_ACCENT, false, false)
    }
    pub fn success(text: impl Into<String>) -> Self {
        Self::build(text, FONT_SM, Color::SUCCESS, false, false)
    }
    pub fn error(text: impl Into<String>) -> Self {
        Self::build(text, FONT_SM, Color::DANGER, false, false)
    }
    pub fn warning(text: impl Into<String>) -> Self {
        Self::build(text, FONT_SM, Color::WARNING, false, false)
    }

    // ── Modifiers ─────────────────────────────────────────────────────────────
    pub fn color(mut self, c: Color32) -> Self {
        self.color = c;
        self
    }
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    pub fn size(mut self, s: f32) -> Self {
        self.size = s;
        self
    }

    // ── Render ────────────────────────────────────────────────────────────────
    pub fn show(self, ui: &mut Ui) -> Response {
        let mut rich = RichText::new(&self.text).size(self.size).color(self.color);
        if self.bold {
            rich = rich.strong();
        }
        if self.italic {
            rich = rich.italics();
        }
        ui.label(rich)
    }

    fn build(text: impl Into<String>, size: f32, color: Color32, bold: bool, italic: bool) -> Self {
        Self {
            text: text.into(),
            size,
            color,
            bold,
            italic,
        }
    }
}
