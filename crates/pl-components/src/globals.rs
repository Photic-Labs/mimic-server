// branding/theme.rs
use egui::Color32;

// ── Typography ────────────────────────────────────────────────────────────────
pub const FONT_XS: f32 = 10.0;
pub const FONT_SM: f32 = 11.0;
pub const FONT_BASE: f32 = 13.0;
pub const FONT_MD: f32 = 14.0;
pub const FONT_LG: f32 = 16.0;
pub const FONT_XL: f32 = 20.0;

// ── Spacing ───────────────────────────────────────────────────────────────────
pub const SPACE_XS: f32 = 2.0;
pub const SPACE_SM: f32 = 4.0;
pub const SPACE_MD: f32 = 8.0;
pub const SPACE_LG: f32 = 16.0;
pub const SPACE_XL: f32 = 24.0;
pub const SPACE_2XL: f32 = 32.0;
pub const MARGIN_X: i8 = 20;
pub const MARGIN_Y: i8 = 10;

// ── Component padding — the fix ───────────────────────────────────────────────
// Buttons
pub const BTN_PAD_H: f32 = 16.0; // horizontal inner padding
pub const BTN_PAD_V: f32 = 8.0; // vertical inner padding
pub const BTN_PAD_H_SM: f32 = 10.0; // small button horizontal
pub const BTN_PAD_V_SM: f32 = 3.0; // small button vertical

// Inputs
pub const INPUT_PAD_H: f32 = 10.0;
pub const INPUT_PAD_V: f32 = 6.0;

// Badges
pub const BADGE_PAD_H: f32 = 8.0;
pub const BADGE_PAD_V: f32 = 3.0;

// Cards / Frames
pub const CARD_PAD: f32 = 16.0;
pub const SECTION_PAD: f32 = 12.0;

// ── Geometry ──────────────────────────────────────────────────────────────────
pub const RADIUS_SM: f32 = 4.0;
pub const RADIUS_MD: f32 = 6.0;
pub const RADIUS_LG: f32 = 10.0;

// Derived min heights from padding + font
pub const BTN_HEIGHT: f32 = FONT_MD + (BTN_PAD_V * 2.0); // = 26px
pub const BTN_HEIGHT_SM: f32 = FONT_SM + (BTN_PAD_V_SM * 2.0); // = 17px
pub const INPUT_HEIGHT: f32 = FONT_BASE + (INPUT_PAD_V * 2.0); // = 25px
pub const ROW_HEIGHT: f32 = 28.0;
pub const HEADER_HEIGHT: f32 = 36.0;

// ─────────────────────────────────────────────────────────────────────────────
// COLOR TOKENS
// ─────────────────────────────────────────────────────────────────────────────
pub struct Color;

impl Color {
    // Brand
    pub const BRAND_BLUE: Color32 = Color32::from_rgb(37, 99, 255);
    pub const BRAND_BLUE_LIGHT: Color32 = Color32::from_rgb(204, 212, 255);
    pub const BRAND_PURPLE: Color32 = Color32::from_rgb(124, 58, 237);
    pub const BRAND_OFF_WHITE: Color32 = Color32::from_rgb(245, 247, 250);

    // Backgrounds
    pub const BG_APP: Color32 = Color32::from_rgb(13, 14, 17);
    pub const BG_PANEL: Color32 = Color32::from_rgb(17, 18, 23);
    pub const BG_SURFACE: Color32 = Color32::from_rgb(28, 30, 38);
    pub const BG_ELEVATED: Color32 = Color32::from_rgb(24, 25, 32);
    pub const BG_HOVER: Color32 = Color32::from_rgb(46, 50, 72);
    pub const BG_SELECTED: Color32 = Color32::from_rgb(30, 58, 110);
    // Disabled & Muted States
    pub const BG_DISABLED: Color32 = Color32::from_rgb(22, 23, 29); // just above BG_SURFACE, no energy
    pub const BORDER_DISABLED: Color32 = Color32::from_rgb(35, 37, 47); // subtle, between SURFACE and ELEVATED
    pub const TEXT_DISABLED: Color32 = Color32::from_rgb(72, 76, 98); // muted blue-grey, ~30% readable
    pub const TEXT_PLACEHOLDER: Color32 = Color32::from_rgb(88, 93, 120); // slightly brighter than disabled, ~40%

    // Text
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(245, 247, 250);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(160, 165, 185);
    pub const TEXT_DIM: Color32 = Color32::from_rgb(90, 96, 128);
    pub const TEXT_ACCENT: Color32 = Color32::from_rgb(37, 99, 255);

    // Borders
    pub const BORDER: Color32 = Color32::from_rgb(42, 46, 64);
    pub const BORDER_FOCUS: Color32 = Color32::from_rgb(37, 99, 255);

    // Semantic
    pub const SUCCESS: Color32 = Color32::from_rgb(34, 197, 94);
    pub const WARNING: Color32 = Color32::from_rgb(234, 179, 8);
    pub const DANGER: Color32 = Color32::from_rgb(239, 68, 68);
    pub const INFO: Color32 = Color32::from_rgb(37, 99, 255);

    // Button fills
    pub const BTN_PRIMARY: Color32 = Color32::from_rgb(37, 99, 255);
    pub const BTN_PRIMARY_HOV: Color32 = Color32::from_rgb(59, 118, 255);
    pub const BTN_PURPLE: Color32 = Color32::from_rgb(124, 58, 237);
    pub const BTN_PURPLE_HOV: Color32 = Color32::from_rgb(139, 78, 255);
    pub const BTN_DANGER: Color32 = Color32::from_rgb(40, 18, 18);
    pub const BTN_DANGER_HOV: Color32 = Color32::from_rgb(70, 25, 25);
    pub const BTN_GHOST: Color32 = Color32::TRANSPARENT;
    pub const BTN_GHOST_HOV: Color32 = Color32::from_rgb(46, 50, 72);
    pub const BTN_SUBTLE: Color32 = Color32::from_rgb(28, 30, 38);
    pub const BTN_SUBTLE_HOV: Color32 = Color32::from_rgb(36, 39, 58);
}
