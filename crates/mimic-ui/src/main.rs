#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod constants;
mod modals;
mod panels;
mod store;

use anyhow::Result;
use eframe::NativeOptions;
use pl_components::theme::AppTheme;
use tracing_subscriber::EnvFilter;

use crate::{
    app::MimicServerApp,
    constants::{APP_TITLE, APP_TITLE_SHORT, DEF_HEIGHT, DEF_WIDTH, MIN_HEIGHT, MIN_WIDTH},
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    mimic_core::config::ensure_dirs().expect("Failed to create MimicServer data directories");
    let db_path = mimic_core::config::db_path();
    eprintln!("[MimicServer] Opening DB at: {db_path:?}");
    let conn = mimic_core::db::connector::open_connection(
        db_path.to_str().expect("DB path is not valid UTF-8"),
    )
    .expect("Failed to initialize database");
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("assets/icons/icon.png"))
        .expect("Failed to load app icon");

    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(APP_TITLE)
            .with_icon(icon)
            .with_inner_size([DEF_WIDTH, DEF_HEIGHT])
            .with_min_inner_size([MIN_WIDTH, MIN_HEIGHT]),
        ..Default::default()
    };

    tokio::task::block_in_place(|| {
        eframe::run_native(
            APP_TITLE_SHORT,
            native_options,
            Box::new(|cc| {
                let mut fonts = egui::FontDefinitions::default();

                // ── Register font data ────────────────────────────────────────────────
                fonts.font_data.insert(
                    "Manrope".to_owned(),
                    egui::FontData::from_static(include_bytes!("assets/fonts/Manrope-Regular.ttf"))
                        .into(),
                );
                fonts.font_data.insert(
                    "Manrope-Bold".to_owned(),
                    egui::FontData::from_static(include_bytes!("assets/fonts/Manrope-Bold.ttf"))
                        .into(),
                );
                fonts.font_data.insert(
                    "NotoSans".to_owned(),
                    egui::FontData::from_static(include_bytes!(
                        "assets/fonts/NotoSansSymbols2-Regular.ttf"
                    ))
                    .into(),
                );

                // ── Proportional: Manrope first, NotoSans as Unicode fallback ──────────
                let proportional = fonts
                    .families
                    .entry(egui::FontFamily::Proportional)
                    .or_default();
                proportional.insert(0, "NotoSans".to_owned()); // fallback (unicode glyphs)
                proportional.insert(0, "Manrope".to_owned()); // primary (overwrites index 0)

                // ── Register Inter-Bold as a named family ─────────────────────────────
                // Use it with: FontFamily::Name("Bold".into())
                fonts.families.insert(
                    egui::FontFamily::Name("Bold".into()),
                    vec!["Manrope-Bold".to_owned(), "NotoSans".to_owned()],
                );
                fonts.families.insert(
                    egui::FontFamily::Name("NotoSans-Bold".into()),
                    vec!["NotoSans".to_owned()],
                );

                // ── Monospace: keep egui default but add NotoSans fallback ────────────
                fonts
                    .families
                    .entry(egui::FontFamily::Monospace)
                    .or_default()
                    .push("NotoSans".to_owned());

                cc.egui_ctx.set_fonts(fonts);
                // ── Apply dark theme before first frame ───────────────────────────────────
                let initial_theme = AppTheme::Light;
                cc.egui_ctx.set_theme(match initial_theme {
                    AppTheme::Dark => egui::ThemePreference::Dark,
                    AppTheme::Light => egui::ThemePreference::Light,
                });
                cc.egui_ctx.set_visuals(AppTheme::Dark.to_visuals());

                Ok(Box::new(MimicServerApp::new(cc, conn, db_path)))
            }),
        )
        .map_err(|e| anyhow::anyhow!("{e}"))
    })
}
