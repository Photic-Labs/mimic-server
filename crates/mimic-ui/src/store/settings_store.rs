use mimic_core::{
    db::app_config::{load_config, save_config},
    models::AppConfig,
};
use pl_components::theme::AppTheme;
use regex::Regex;
use rusqlite::Connection;
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct SettingsStore {
    pub theme: AppTheme,
    pub prefix: String,
    pub prefix_input: String,
    pub port: u16,
    pub port_input: String,
    pub port_error: Option<String>,
    pub prefix_error: Option<String>,
    pub run_in_bg: bool,
}

impl SettingsStore {
    pub fn load(conn: &Connection) -> Self {
        let mut theme = AppTheme::Dark;
        let mut port = 8080;
        let mut prefix = String::from("");
        match load_config(conn) {
            Ok(app_config) => {
                theme = app_config.theme.to_string().into();
                port = app_config.port;
                prefix = app_config.prefix;
            }
            Err(e) => {
                eprintln!("Error getting app config {e}");
            }
        };
        Self {
            theme,
            prefix: prefix.clone(),
            port,
            prefix_input: prefix,
            port_input: port.to_string(),
            port_error: None,
            prefix_error: None,
            run_in_bg: false,
        }
    }

    pub fn apply_theme(&mut self, conn: &Connection, new_theme: AppTheme) {
        match save_config(
            conn,
            AppConfig {
                theme: new_theme.to_string(),
                port: self.port,
                prefix: self.prefix.clone(),
                ..Default::default()
            },
        ) {
            Ok(_) => {
                self.theme = new_theme;
            }
            Err(e) => {
                eprintln!("Error saving app config {e}");
            }
        };
    }

    pub fn apply_port(&mut self, conn: &Connection) {
        match self.port_input.trim().parse::<u16>() {
            Ok(port) if port >= 1024 => {
                self.port = port;
                self.port_error = None;
                match save_config(
                    conn,
                    AppConfig {
                        port: port,
                        theme: self.theme.to_string(),
                        prefix: self.prefix.clone(),
                        ..Default::default()
                    },
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error saving app config {e}");
                    }
                }
            }
            Ok(_) => {
                self.port_error = Some("Port must be greater than 1023".to_string());
            }
            Err(_) => {
                self.port_error = Some("Invalid port".to_string());
            }
        }
    }

    pub fn apply_prefix(&mut self, conn: &Connection) {
        match validate_multi_segment_path(self.prefix_input.trim()) {
            Some(prefix) => {
                self.prefix = prefix.to_string();
                self.prefix_error = None;
                match save_config(
                    conn,
                    AppConfig {
                        theme: self.theme.to_string(),
                        port: self.port,
                        prefix: self.prefix.clone(),
                        ..Default::default()
                    },
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error saving app config {e}");
                    }
                }
            }
            None => {
                self.prefix_error = Some("Invalid prefix".to_string());
            }
        }
    }

    pub fn reset_settings(&mut self, conn: &Connection) {
        let app_config = AppConfig {
            ..Default::default()
        };

        match save_config(conn, app_config.clone()) {
            Ok(_) => {
                self.theme = app_config.theme.into();
                self.port = app_config.port;
                self.prefix = app_config.prefix.clone();
                self.run_in_bg = false;
                self.port_error = None;
                self.prefix_error = None;

                // ✅ Sync display buffers to the actual default values
                // NOT String::new() — that leaves the box empty
                self.port_input = app_config.port.to_string();
                self.prefix_input = app_config.prefix.clone();
            }
            Err(e) => {
                eprintln!("Error resetting settings: {e}");
            }
        }
    }
}

pub fn validate_multi_segment_path(path: &str) -> Option<&str> {
    if path.is_empty() {
        return Some(path);
    }
    // ^            : Start of string
    // (/[a-zA-Z0-9_]+)+ : One or more repetitions of (slash followed by 1+ word characters)
    // $            : End of string
    static RE: OnceLock<Regex> = OnceLock::new();
    let regex = RE.get_or_init(|| Regex::new(r"^(/[a-zA-Z0-9_]+)+$").unwrap());

    if regex.is_match(path) {
        Some(path)
    } else {
        None
    }
}
