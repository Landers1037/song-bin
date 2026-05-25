mod colors;

pub use colors::ThemeColors;

use std::fs;
use std::path::{Path, PathBuf};

use gpui::{App, Global, SharedString};
use gpui_component::{ActiveTheme as _, Theme as ComponentTheme, ThemeMode, ThemeRegistry};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeKind {
    Light,
    Dark,
}

pub struct ThemeManager {
    pub current: ThemeKind,
    pub colors: ThemeColors,
}

impl Global for ThemeManager {}

impl ThemeManager {
    pub fn new(kind: ThemeKind) -> Self {
        let colors = match kind {
            ThemeKind::Light => ThemeColors::light(),
            ThemeKind::Dark => ThemeColors::dark(),
        };
        Self {
            current: kind,
            colors,
        }
    }

    pub fn switch_theme(&mut self, kind: ThemeKind) {
        self.current = kind;
        self.colors = match kind {
            ThemeKind::Light => ThemeColors::light(),
            ThemeKind::Dark => ThemeColors::dark(),
        };
    }

    pub fn toggle(&mut self) {
        match self.current {
            ThemeKind::Light => self.switch_theme(ThemeKind::Dark),
            ThemeKind::Dark => self.switch_theme(ThemeKind::Light),
        }
    }

    pub fn sync_from_component(&mut self, cx: &App) {
        let component = cx.theme();
        self.current = if component.mode.is_dark() {
            ThemeKind::Dark
        } else {
            ThemeKind::Light
        };
        self.colors = ThemeColors::from_component(component);
    }
}

pub fn init(cx: &mut App) {
    cx.set_global(ThemeManager::new(ThemeKind::Dark));
}

pub fn init_with_kind(cx: &mut App, kind: ThemeKind) {
    cx.set_global(ThemeManager::new(kind));
}

pub fn theme(cx: &App) -> &ThemeManager {
    cx.global::<ThemeManager>()
}

pub fn themes_dir() -> PathBuf {
    let manifest_themes = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("themes");
    if manifest_themes.exists() {
        return manifest_themes;
    }

    if let Ok(cwd) = std::env::current_dir() {
        let cwd_themes = cwd.join("themes");
        if cwd_themes.exists() {
            return cwd_themes;
        }
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            for rel in ["themes", "../themes", "../../themes", "../../../themes"] {
                let candidate = dir.join(rel);
                if candidate.exists() {
                    return candidate;
                }
            }
        }
    }

    manifest_themes
}

pub fn load_themes_from_dir(dir: &Path, cx: &mut App) {
    if !dir.exists() {
        log::warn!("Themes directory not found: {}", dir.display());
        return;
    }

    let registry = ThemeRegistry::global_mut(cx);
    let mut loaded = 0usize;

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) => {
            log::warn!("Failed to read themes directory {}: {}", dir.display(), err);
            return;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        match fs::read_to_string(&path) {
            Ok(content) => match registry.load_themes_from_str(&content) {
                Ok(()) => loaded += 1,
                Err(err) => log::warn!("Failed to parse theme {}: {}", path.display(), err),
            },
            Err(err) => log::warn!("Failed to read theme {}: {}", path.display(), err),
        }
    }

    log::info!(
        "Loaded {} theme file(s) from {} ({} themes available)",
        loaded,
        dir.display(),
        registry.themes().len()
    );
}

pub fn init_component_themes(cx: &mut App, color_theme: &SharedString, kind: ThemeKind) {
    let theme_name = color_theme.clone();
    let mode = match kind {
        ThemeKind::Light => ThemeMode::Light,
        ThemeKind::Dark => ThemeMode::Dark,
    };

    let themes_dir = themes_dir();
    load_themes_from_dir(&themes_dir, cx);

    if let Err(err) = ThemeRegistry::watch_dir(themes_dir, cx, move |cx| {
        apply_color_theme(&theme_name, cx);
        sync_component_theme(cx);
    }) {
        log::warn!("Failed to watch themes directory: {}", err);
    }

    ComponentTheme::global_mut(cx).mode = mode;
    apply_color_theme(color_theme, cx);
    sync_component_theme(cx);
}

pub fn apply_color_theme(theme_name: &SharedString, cx: &mut App) {
    if let Some(theme_config) = ThemeRegistry::global(cx).themes().get(theme_name).cloned() {
        ComponentTheme::global_mut(cx).apply_config(&theme_config);
    }
}

pub fn sync_component_theme(cx: &mut App) {
    let component = cx.theme();
    let kind = if component.mode.is_dark() {
        ThemeKind::Dark
    } else {
        ThemeKind::Light
    };
    let colors = ThemeColors::from_component(component);
    {
        let manager = cx.global_mut::<ThemeManager>();
        manager.current = kind;
        manager.colors = colors;
    }
    cx.refresh_windows();
}

pub fn set_theme_mode(mode: ThemeMode, window: Option<&mut gpui::Window>, cx: &mut App) {
    ComponentTheme::change(mode, window, cx);
    sync_component_theme(cx);
}
