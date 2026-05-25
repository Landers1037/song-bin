use gpui::{prelude::*, px, App, Context, Entity, Render, SharedString, Window};

use gpui_component::{
    ActiveTheme as _, IndexPath, ThemeMode, ThemeRegistry,
    h_flex, select::{Select, SelectEvent, SelectItem, SelectState}, switch::Switch,
};

use crate::state::app_state::AppState;
use crate::theme;

pub struct Panel {
    pub content: PanelContent,
    theme_select: Entity<SelectState<Vec<ThemeOption>>>,
}

#[derive(Clone, PartialEq)]
pub enum PanelContent {
    Welcome,
    NodeDetail { name: String, protocol: String },
    Settings,
    Logs,
}

#[derive(Clone, PartialEq)]
struct ThemeOption {
    name: SharedString,
    is_active: bool,
}

impl ThemeOption {
    fn new(name: impl Into<SharedString>, is_active: bool) -> Self {
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

impl Panel {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let active_theme = cx
            .global::<AppState>()
            .settings
            .color_theme
            .clone();
        let items = build_theme_options(cx, &active_theme);
        let selected_index = items
            .iter()
            .position(|item| item.name == active_theme)
            .map(|idx| IndexPath::default().row(idx));
        let theme_select = cx.new(|cx| SelectState::new(items, selected_index, window, cx));

        cx.subscribe(&theme_select, move |_, _, event: &SelectEvent<Vec<ThemeOption>>, cx| {
            let SelectEvent::Confirm(theme_name) = event;
            if let Some(theme_name) = theme_name {
                theme::apply_color_theme(theme_name, cx);
                theme::sync_component_theme(cx);
                cx.global_mut::<AppState>().settings.color_theme = theme_name.clone();
                let _ = cx.global_mut::<AppState>().settings.save();
            }
        })
        .detach();

        Self {
            content: PanelContent::Welcome,
            theme_select,
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
        match &self.content {
            PanelContent::Welcome => {
                let colors = theme::theme(cx).colors;
                gpui::div()
                    .flex()
                    .flex_1()
                    .flex_col()
                    .h_full()
                    .bg(colors.background)
                    .p(px(24.))
                    .child(render_welcome(colors))
            }
            PanelContent::NodeDetail { name, protocol } => {
                let colors = theme::theme(cx).colors;
                gpui::div()
                    .flex()
                    .flex_1()
                    .flex_col()
                    .h_full()
                    .bg(colors.background)
                    .p(px(24.))
                    .child(render_node_detail(name, protocol, colors))
            }
            PanelContent::Settings => {
                let colors = theme::theme(cx).colors;
                gpui::div()
                    .flex()
                    .flex_1()
                    .flex_col()
                    .h_full()
                    .bg(colors.background)
                    .p(px(24.))
                    .child(render_settings(window, cx, &self.theme_select, colors))
            }
            PanelContent::Logs => {
                let colors = theme::theme(cx).colors;
                gpui::div()
                    .flex()
                    .flex_1()
                    .flex_col()
                    .h_full()
                    .bg(colors.background)
                    .p(px(24.))
                    .child(render_logs(colors))
            }
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

fn render_settings(
    _window: &mut Window,
    cx: &mut Context<Panel>,
    theme_select: &Entity<SelectState<Vec<ThemeOption>>>,
    colors: crate::theme::ThemeColors,
) -> gpui::Div {
    let settings = &cx.global::<AppState>().settings;
    let mode_name = settings.proxy_mode.label();
    let is_dark = cx.theme().mode.is_dark();
    let current_theme = cx.theme().theme_name().clone();

    gpui::div()
        .flex()
        .flex_col()
        .gap(px(20.))
        .child(
            gpui::div()
                .text_size(px(20.))
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(colors.text_primary)
                .child("设置"),
        )
        .child(
            gpui::div()
                .flex()
                .flex_col()
                .gap(px(12.))
                .child(
                    gpui::div()
                        .text_size(px(14.))
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_color(colors.text_primary)
                        .child("外观"),
                )
                .child(setting_row(
                    "深色模式",
                    Switch::new("dark-mode")
                        .checked(is_dark)
                        .on_click(|checked, window, cx| {
                            let mode = if *checked {
                                ThemeMode::Dark
                            } else {
                                ThemeMode::Light
                            };
                            theme::set_theme_mode(mode, Some(window), cx);
                            cx.global_mut::<AppState>().settings.theme = theme::theme(cx).current;
                            let _ = cx.global_mut::<AppState>().settings.save();
                            cx.refresh_windows();
                        }),
                    colors,
                ))
                .child(
                    gpui::div()
                        .flex()
                        .flex_col()
                        .gap(px(8.))
                        .child(
                            gpui::div()
                                .text_size(px(13.))
                                .text_color(colors.text_secondary)
                                .child("配色主题"),
                        )
                        .child(
                            h_flex()
                                .w_full()
                                .items_center()
                                .justify_between()
                                .gap(px(12.))
                                .child(
                                    gpui::div()
                                        .text_size(px(12.))
                                        .text_color(colors.text_disabled)
                                        .child(current_theme),
                                )
                                .child(
                                    gpui::div()
                                        .w(px(240.))
                                        .child(Select::new(theme_select)),
                                ),
                        ),
                ),
        )
        .child(setting_row_text("代理模式", mode_name, colors))
        .child(setting_row_text(
            "混合代理端口",
            &settings.mixed_port.to_string(),
            colors,
        ))
        .child(setting_row_text(
            "Clash API 端口",
            &settings.clash_api_port.to_string(),
            colors,
        ))
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

fn setting_row(
    label: &str,
    value: impl IntoElement,
    colors: crate::theme::ThemeColors,
) -> gpui::Div {
    gpui::div()
        .flex()
        .items_center()
        .justify_between()
        .py(px(8.))
        .border_b_1()
        .border_color(colors.border)
        .child(
            gpui::div()
                .text_size(px(13.))
                .text_color(colors.text_secondary)
                .child(label.to_string()),
        )
        .child(value)
}

fn setting_row_text(label: &str, value: &str, colors: crate::theme::ThemeColors) -> gpui::Div {
    setting_row(
        label,
        gpui::div()
            .text_size(px(13.))
            .text_color(colors.text_primary)
            .child(value.to_string()),
        colors,
    )
}
