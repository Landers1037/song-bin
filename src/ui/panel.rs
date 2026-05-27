use gpui::{prelude::*, px, App, Context, Entity, Render, SharedString, Window};

use gpui_component::{
    IndexPath, ThemeRegistry,
    select::{SelectEvent, SelectItem, SelectState},
};

use crate::state::app_state::AppState;
use crate::state::settings::UiLogLevel;
use crate::theme;
use crate::ui::settings::{self, SettingsTab};

pub struct Panel {
    pub content: PanelContent,
    pub settings_tab: SettingsTab,
    pub theme_select: Entity<SelectState<Vec<ThemeOption>>>,
    pub log_level_select: Entity<SelectState<Vec<LogLevelOption>>>,
}

#[derive(Clone, PartialEq)]
pub enum PanelContent {
    Welcome,
    NodeDetail { name: String, protocol: String },
    Settings,
    Logs,
}

#[derive(Clone, PartialEq)]
pub struct ThemeOption {
    name: SharedString,
    is_active: bool,
}

impl ThemeOption {
    pub fn new(name: impl Into<SharedString>, is_active: bool) -> Self {
        Self {
            name: name.into(),
            is_active,
        }
    }
}

impl SelectItem for ThemeOption {
    type Value = SharedString;

    fn title(&self) -> SharedString {
        self.name.clone()
    }

    fn value(&self) -> &Self::Value {
        &self.name
    }
}

#[derive(Clone, PartialEq)]
pub struct LogLevelOption {
    level: UiLogLevel,
    is_active: bool,
}

impl LogLevelOption {
    pub fn new(level: UiLogLevel, is_active: bool) -> Self {
        Self { level, is_active }
    }
}

impl SelectItem for LogLevelOption {
    type Value = UiLogLevel;

    fn title(&self) -> SharedString {
        self.level.label().into()
    }

    fn value(&self) -> &Self::Value {
        &self.level
    }
}

impl Panel {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let app_state = cx.global::<AppState>();
        let active_theme = app_state.settings.color_theme.clone();
        let active_log_level = app_state.settings.ui_log_level;

        let theme_items = build_theme_options(cx, &active_theme);
        let theme_selected = theme_items
            .iter()
            .position(|item| item.name == active_theme)
            .map(|idx| IndexPath::default().row(idx));
        let theme_select = cx.new(|cx| {
            SelectState::new(theme_items, theme_selected, window, cx)
        });

        let log_items = settings::build_log_level_options(active_log_level);
        let log_selected = log_items
            .iter()
            .position(|item| item.level == active_log_level)
            .map(|idx| IndexPath::default().row(idx));
        let log_level_select = cx.new(|cx| SelectState::new(log_items, log_selected, window, cx));

        cx.subscribe(&theme_select, |_, _, event: &SelectEvent<Vec<ThemeOption>>, cx| {
            let SelectEvent::Confirm(theme_name) = event;
            if let Some(theme_name) = theme_name {
                theme::apply_color_theme(theme_name, cx);
                theme::sync_component_theme(cx);
                cx.global_mut::<AppState>().settings.color_theme = theme_name.clone();
                let _ = cx.global_mut::<AppState>().settings.save();
            }
        })
        .detach();

        cx.subscribe(&log_level_select, |_, _, event: &SelectEvent<Vec<LogLevelOption>>, cx| {
            let SelectEvent::Confirm(level) = event;
            if let Some(level) = level {
                cx.global_mut::<AppState>().settings.ui_log_level = *level;
                let _ = cx.global_mut::<AppState>().settings.save();
                crate::utils::app_log::ui_info(
                    cx,
                    format!("[settings] UI 日志级别已设为 {}", level.label()),
                );
            }
        })
        .detach();

        Self {
            content: PanelContent::Welcome,
            settings_tab: SettingsTab::default(),
            theme_select,
            log_level_select,
        }
    }
}

fn build_theme_options(cx: &App, active_theme: &SharedString) -> Vec<ThemeOption> {
    ThemeRegistry::global(cx)
        .sorted_themes()
        .into_iter()
        .map(|theme| ThemeOption::new(theme.name.clone(), theme.name == *active_theme))
        .collect()
}

impl Render for Panel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = theme::theme(cx).colors.clone();

        match &self.content {
            PanelContent::Welcome => gpui::div()
                .flex()
                .flex_1()
                .flex_col()
                .h_full()
                .bg(colors.background)
                .p(px(24.))
                .child(render_welcome(colors)),
            PanelContent::NodeDetail { name, protocol } => gpui::div()
                .flex()
                .flex_1()
                .flex_col()
                .h_full()
                .bg(colors.background)
                .p(px(24.))
                .child(render_node_detail(name, protocol, colors)),
            PanelContent::Settings => gpui::div()
                .flex()
                .flex_1()
                .flex_col()
                .h_full()
                .bg(colors.background)
                .child(settings::render_settings_page(self, window, cx, colors)),
            PanelContent::Logs => gpui::div()
                .flex()
                .flex_1()
                .flex_col()
                .h_full()
                .bg(colors.background)
                .p(px(24.))
                .child(render_logs(colors)),
        }
    }
}

fn render_welcome(colors: crate::theme::ThemeColors) -> gpui::Div {
    gpui::div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .flex_1()
        .gap(px(12.))
        .child(
            gpui::div()
                .text_size(px(28.))
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(colors.text_primary)
                .child("song-bin"),
        )
        .child(
            gpui::div()
                .text_size(px(14.))
                .text_color(colors.text_secondary)
                .child("基于 sing-box 的代理客户端"),
        )
        .child(
            gpui::div()
                .mt(px(24.))
                .text_size(px(12.))
                .text_color(colors.text_disabled)
                .child("从左侧选择节点或订阅开始使用"),
        )
}

fn render_node_detail(
    name: &str,
    protocol: &str,
    colors: crate::theme::ThemeColors,
) -> gpui::Div {
    gpui::div()
        .flex()
        .flex_col()
        .gap(px(16.))
        .child(
            gpui::div()
                .text_size(px(20.))
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(colors.text_primary)
                .child(name.to_string()),
        )
        .child(
            gpui::div()
                .flex()
                .items_center()
                .gap(px(8.))
                .child(
                    gpui::div()
                        .px(px(8.))
                        .py(px(2.))
                        .rounded(px(4.))
                        .bg(colors.accent)
                        .text_size(px(11.))
                        .text_color(colors.text_on_accent)
                        .child(protocol.to_string()),
                ),
        )
}

fn render_logs(colors: crate::theme::ThemeColors) -> gpui::Div {
    gpui::div()
        .flex()
        .flex_col()
        .gap(px(16.))
        .child(
            gpui::div()
                .text_size(px(20.))
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(colors.text_primary)
                .child("日志"),
        )
        .child(
            gpui::div()
                .flex_1()
                .p(px(12.))
                .rounded(px(6.))
                .bg(colors.element_bg)
                .text_size(px(12.))
                .text_color(colors.text_secondary)
                .child("日志输出将在此显示（开发中）"),
        )
}
