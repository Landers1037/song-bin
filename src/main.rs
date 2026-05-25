mod app;
mod core;
mod proxy;
mod state;
mod theme;
mod ui;
mod utils;

use gpui::{prelude::*, px, size, App, Bounds, WindowBounds, WindowOptions};
use gpui_component::Root;
use gpui_component_assets::Assets;
use single_instance::SingleInstance;

fn main() {
    env_logger::init();

    let instance = SingleInstance::new("song-bin-mutex-v1").unwrap();
    if !instance.is_single() {
        log::info!("Another instance is already running, exiting.");
        return;
    }

    log::info!("Starting song-bin...");

    let app_state = state::app_state::AppState::load();

    run_app(app_state);
}

fn run_app(app_state: state::app_state::AppState) {
    let theme_kind = app_state.settings.theme;
    let color_theme = app_state.settings.color_theme.clone();

    gpui_platform::application()
        .with_assets(Assets)
        .run(move |cx: &mut App| {
            gpui_component::init(cx);
            theme::init_with_kind(cx, theme_kind);
            theme::init_component_themes(cx, &color_theme, theme_kind);
            cx.set_global(app_state);

            let bounds = Bounds::centered(None, size(px(1000.), px(680.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: None,
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| app::MainView::new(window, cx));
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )
            .unwrap();

            cx.activate(true);
        });
}
