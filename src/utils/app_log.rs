use gpui::App;

use crate::state::app_state::AppState;
use crate::state::settings::UiLogLevel;

fn level_enabled(level: UiLogLevel, target: log::Level) -> bool {
    match level {
        UiLogLevel::None => false,
        UiLogLevel::Error => target == log::Level::Error,
        UiLogLevel::Info => matches!(
            target,
            log::Level::Error | log::Level::Warn | log::Level::Info
        ),
        UiLogLevel::Debug => true,
    }
}

fn ui_log_level(cx: &App) -> UiLogLevel {
    cx.global::<AppState>().settings.ui_log_level
}

pub fn ui_info(cx: &App, message: impl AsRef<str>) {
    if level_enabled(ui_log_level(cx), log::Level::Info) {
        log::info!("{}", message.as_ref());
    }
}

pub fn ui_debug(cx: &App, message: impl AsRef<str>) {
    if level_enabled(ui_log_level(cx), log::Level::Debug) {
        log::debug!("{}", message.as_ref());
    }
}

pub fn ui_error(cx: &App, message: impl AsRef<str>) {
    if level_enabled(ui_log_level(cx), log::Level::Error) {
        log::error!("{}", message.as_ref());
    }
}
