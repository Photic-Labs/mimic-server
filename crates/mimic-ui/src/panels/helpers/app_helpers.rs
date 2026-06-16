use egui::{Color32, RichText};

use crate::constants::FONT_SIZE_SM;

pub const COL_METHOD: f32 = 64.0_f32;
pub const COL_STATUS: f32 = 64.0_f32;
pub const COL_LATENCY: f32 = 80.0_f32;
pub const COL_TIME: f32 = 200.0_f32;

/// Returns a color per HTTP method for visual scanning.
pub fn method_tag_colors(method: &str) -> (Color32, Color32) {
    match method.to_uppercase().as_str() {
        "GET" => (
            Color32::from_rgb(30, 80, 140),
            Color32::from_rgb(97, 175, 254),
        ),
        "POST" => (
            Color32::from_rgb(20, 90, 55),
            Color32::from_rgb(73, 204, 144),
        ),
        "PUT" => (
            Color32::from_rgb(110, 65, 10),
            Color32::from_rgb(252, 161, 48),
        ),
        "PATCH" => (
            Color32::from_rgb(15, 90, 80),
            Color32::from_rgb(80, 227, 194),
        ),
        "DELETE" => (
            Color32::from_rgb(110, 20, 20),
            Color32::from_rgb(249, 62, 62),
        ),
        _ => (Color32::from_rgb(60, 60, 60), Color32::GRAY),
    }
}

pub fn get_available_width(available_width: f32) -> f32 {
    available_width - COL_METHOD - COL_STATUS - COL_LATENCY - COL_TIME - 16.0
}

pub fn dim_header(text: &str) -> egui::RichText {
    RichText::new(text)
        .size(FONT_SIZE_SM)
        .strong()
        .color(Color32::GRAY)
}

pub fn status_color(status: u16) -> Color32 {
    match status {
        200..=299 => Color32::from_rgb(73, 204, 144), // green
        300..=399 => Color32::from_rgb(97, 175, 254), // blue
        400..=499 => Color32::from_rgb(252, 161, 48), // orange
        500..=599 => Color32::from_rgb(249, 62, 62),  // red
        _ => Color32::GRAY,
    }
}

pub fn latency_color(ms: i64) -> Color32 {
    match ms {
        0..=99 => Color32::from_rgb(73, 204, 144), // green  — fast
        100..=499 => Color32::from_rgb(252, 161, 48), // orange — acceptable
        _ => Color32::from_rgb(249, 62, 62),       // red    — slow
    }
}

/// Formats a Unix timestamp (seconds since epoch) as "DD/MM HH:MM"
/// Pure arithmetic — no external crates needed.
pub fn format_timestamp(unix_secs: i64) -> String {
    // ── Days since epoch ─────────────────────────────────────────
    let secs_in_day = 86_400i64;
    let time_of_day = unix_secs % secs_in_day;
    let days = unix_secs / secs_in_day;

    // ── HH:MM ────────────────────────────────────────────────────
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;

    // ── DD/MM from days since 1970-01-01 ────────────────────────
    // Gregorian calendar algorithm (handles leap years correctly)
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097; // day of era [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // year of era [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // day of year [0, 365]
    let mp = (5 * doy + 2) / 153; // month prime [0, 11]
    let day = doy - (153 * mp + 2) / 5 + 1; // day [1, 31]
    let month = if mp < 10 { mp + 3 } else { mp - 9 }; // month [1, 12]
    let _year = if month <= 2 { y + 1 } else { y }; // unused for DD/MM

    format!("{:02}/{:02} {:02}:{:02}", day, month, hours, minutes)
}
