use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use gpui::SharedString;
use serde::{Deserialize, Serialize};

use crate::theme::ThemeKind;
use crate::ui::statusbar::ProxyMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    #[serde(default = "default_dark")]
    pub theme: ThemeKind,
    #[serde(default = "default_color_theme")]
    pub color_theme: SharedString,
    #[serde(default = "default_rule")]
    pub proxy_mode: ProxyMode,
    #[serde(default)]
    pub auto_start: bool,
    #[serde(default = "default_clash_api_port")]
    pub clash_api_port: u16,
    #[serde(default = "default_mixed_port")]
    pub mixed_port: u16,
    #[serde(default)]
    pub active_node_id: Option<uuid::Uuid>,
    #[serde(default)]
    pub system_proxy: bool,
}

fn default_dark() -> ThemeKind {
    ThemeKind::Dark
}

fn default_color_theme() -> SharedString {
    "Default Dark".into()
}

fn default_rule() -> ProxyMode {
    ProxyMode::Rule
}

fn default_clash_api_port() -> u16 {
    9090
}

fn default_mixed_port() -> u16 {
    7890
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: ThemeKind::Dark,
            color_theme: default_color_theme(),
            proxy_mode: ProxyMode::Rule,
            auto_start: false,
            clash_api_port: 9090,
            mixed_port: 7890,
            active_node_id: None,
            system_proxy: false,
        }
    }
}

impl AppSettings {
    pub fn load() -> Self {
        let path = Self::file_path();
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => match serde_json::from_str(&content) {
                    Ok(settings) => return settings,
                    Err(e) => log::warn!("Failed to parse settings: {}", e),
                },
                Err(e) => log::warn!("Failed to read settings file: {}", e),
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::file_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn data_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("song-bin")
    }

    fn file_path() -> PathBuf {
        Self::data_dir().join("settings.json")
    }
}
