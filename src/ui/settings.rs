use gpui::{prelude::*, px, Context, Entity, SharedString, Window};

use gpui_component::{
    ActiveTheme as _, ThemeMode,
    WindowExt as _,
    h_flex, select::{Select, SelectState}, switch::Switch,
};

use crate::state::app_state::AppState;
use crate::state::settings::UiLogLevel;
use crate::{core::downloader::CoreDownloader, utils::paths};
use crate::theme;
use crate::theme::ThemeColors;
use crate::ui::panel::{LogLevelOption, Panel, ThemeOption};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsTab {
    #[default]
    Appearance,
    Proxy,
    Core,
    Tun,
    Log,
    About,
}

impl SettingsTab {
    pub fn label(self) -> &'static str {
        match self {
            Self::Appearance => "外观设置",
            Self::Proxy => "代理设置",
            Self::Core => "核心设置",
            Self::Tun => "TUN 设置",
            Self::Log => "日志设置",
            Self::About => "关于",
        }
    }

    pub fn all() -> &'static [SettingsTab] {
        &[
            Self::Appearance,
            Self::Proxy,
            Self::Core,
            Self::Tun,
            Self::Log,
            Self::About,
        ]
    }
}

pub fn render_settings_page(
    panel: &Panel,
    window: &mut Window,
    cx: &mut Context<Panel>,
    colors: ThemeColors,
) -> impl IntoElement {
    let active_tab = panel.settings_tab;

    gpui::div()
        .flex()
        .flex_1()
        .h_full()
        .child(render_settings_nav(active_tab, cx.entity().clone(), colors))
        .child(
            gpui::div()
                .flex_1()
                .flex()
                .flex_col()
                .h_full()
                .overflow_hidden()
                .border_l_1()
                .border_color(colors.border)
                .p(px(24.))
                .child(render_settings_detail(
                    active_tab,
                    window,
                    cx,
                    panel,
                    colors,
                )),
        )
}

fn render_settings_nav(
    active_tab: SettingsTab,
    panel: Entity<Panel>,
    colors: ThemeColors,
) -> impl IntoElement {
    gpui::div()
        .flex()
        .flex_col()
        .w(px(180.))
        .flex_none()
        .py(px(16.))
        .border_r_1()
        .border_color(colors.border)
        .child(
            gpui::div()
                .px(px(16.))
                .pb(px(12.))
                .text_size(px(16.))
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(colors.text_primary)
                .child("设置"),
        )
        .children(SettingsTab::all().iter().map(|&tab| {
            settings_nav_item(tab, active_tab == tab, panel.clone(), colors)
        }))
}

fn settings_nav_item(
    tab: SettingsTab,
    active: bool,
    panel: Entity<Panel>,
    colors: ThemeColors,
) -> impl IntoElement {
    let bg = if active {
        colors.element_selected
    } else {
        gpui::transparent_black()
    };
    let text_color = if active {
        colors.accent
    } else {
        colors.text_secondary
    };

    gpui::div()
        .id(SharedString::from(format!("settings-tab-{}", tab.label())))
        .mx(px(8.))
        .px(px(12.))
        .py(px(8.))
        .rounded(px(6.))
        .bg(bg)
        .text_size(px(13.))
        .text_color(text_color)
        .cursor_pointer()
        .on_click(move |_, _, cx| {
            panel.update(cx, |panel, cx| {
                panel.settings_tab = tab;
                cx.notify();
            });
        })
        .child(tab.label().to_string())
}

fn render_settings_detail(
    tab: SettingsTab,
    window: &mut Window,
    cx: &mut Context<Panel>,
    panel: &Panel,
    colors: ThemeColors,
) -> impl IntoElement {
    match tab {
        SettingsTab::Appearance => {
            render_appearance_settings(window, cx, &panel.theme_select, colors).into_any_element()
        }
        SettingsTab::Proxy => render_proxy_settings(cx, colors).into_any_element(),
        SettingsTab::Core => render_core_settings(window, cx, colors).into_any_element(),
        SettingsTab::Tun => {
            render_placeholder_section("TUN 设置", "TUN 模式与网卡配置（开发中）", colors)
                .into_any_element()
        }
        SettingsTab::Log => {
            render_log_settings(cx, &panel.log_level_select, colors).into_any_element()
        }
        SettingsTab::About => render_about(colors).into_any_element(),
    }
}

fn render_appearance_settings(
    _window: &mut Window,
    cx: &mut Context<Panel>,
    theme_select: &Entity<SelectState<Vec<ThemeOption>>>,
    colors: ThemeColors,
) -> gpui::Div {
    let is_dark = cx.theme().mode.is_dark();
    let current_theme = cx.theme().theme_name().clone();

    section(
        "外观设置",
        gpui::div()
            .flex()
            .flex_col()
            .gap(px(12.))
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
        colors,
    )
}

fn render_proxy_settings(cx: &mut Context<Panel>, colors: ThemeColors) -> gpui::Div {
    let settings = &cx.global::<AppState>().settings;
    let mode_name = settings.proxy_mode.label();

    section(
        "代理设置",
        gpui::div()
            .flex()
            .flex_col()
            .gap(px(4.))
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
            .child(setting_row_text(
                "系统代理",
                if settings.system_proxy {
                    "已启用"
                } else {
                    "未启用"
                },
                colors,
            )),
        colors,
    )
}

fn render_core_settings(
    _window: &mut Window,
    _cx: &mut Context<Panel>,
    colors: ThemeColors,
) -> gpui::Div {
    let install_dir = paths::app_install_dir();
    let core_dir = install_dir.join("sing-box-core");
    let core_path = core_dir.join("sing-box.exe");
    let installed = core_path.exists();

    let install_dir_text = install_dir.display().to_string();
    let core_path_text = core_path.display().to_string();

    section(
        "核心设置",
        gpui::div()
            .flex()
            .flex_col()
            .gap(px(4.))
            .child(setting_row(
                "程序安装目录",
                h_flex()
                    .flex_1()
                    .min_w(px(0.))
                    .items_center()
                    .gap(px(8.))
                    .child(
                        gpui::div()
                            .flex_1()
                            .min_w(px(0.))
                            .overflow_hidden()
                            .text_size(px(13.))
                            .text_color(colors.text_primary)
                            .child(install_dir_text),
                    )
                    .child(
                        crate::ui::components::button::Button::new("打开核心目录")
                            .element_id("open-core-dir-install-row")
                            .variant(crate::ui::components::button::ButtonVariant::Secondary)
                            .on_click({
                                let core_dir = core_dir.clone();
                                move |window, cx| {
                                    match paths::open_dir_in_explorer(&core_dir) {
                                        Ok(()) => window.push_notification("已打开核心目录", cx),
                                        Err(err) => window.push_notification(
                                            format!("打开目录失败：{err}"),
                                            cx,
                                        ),
                                    }
                                }
                            }),
                    ),
                colors,
            ))
            .child(setting_row_text("核心安装路径", &core_path_text, colors))
            .child(setting_row_text(
                "核心状态",
                if installed {
                    "已安装"
                } else {
                    "未找到 sing-box.exe（将从 sing-box-core 目录扫描）"
                },
                colors,
            ))
            .child(
                h_flex()
                    .items_center()
                    .gap(px(8.))
                    .child(
                        crate::ui::components::button::Button::new("更新核心")
                            .element_id("update-core")
                            .variant(crate::ui::components::button::ButtonVariant::Secondary)
                            .on_click(|window, cx| {
                                window
                                    .spawn(cx, |cx: &mut gpui::AsyncWindowContext| {
                                        let mut cx = cx.clone();
                                        async move {
                                            let result = cx
                                                .background_executor()
                                                .spawn(async move { CoreDownloader::download_latest() })
                                                .await;

                                            let _ = cx.update(|window, cx| match result {
                                                Ok(path) => {
                                                    window.push_notification(
                                                        format!("核心已更新：{}", path.display()),
                                                        cx,
                                                    );
                                                    cx.refresh_windows();
                                                }
                                                Err(err) => window
                                                    .push_notification(format!("更新失败：{err}"), cx),
                                            });
                                        }
                                    })
                                    .detach();
                            }),
                    )
                    .child(
                        crate::ui::components::button::Button::new("下载核心")
                            .element_id("download-core")
                            .variant(crate::ui::components::button::ButtonVariant::Secondary)
                            .on_click(|window, cx| {
                                window
                                    .spawn(cx, |cx: &mut gpui::AsyncWindowContext| {
                                        let mut cx = cx.clone();
                                        async move {
                                            let result = cx
                                                .background_executor()
                                                .spawn(async move { CoreDownloader::download_latest() })
                                                .await;

                                            let _ = cx.update(|window, cx| match result {
                                                Ok(path) => {
                                                    window.push_notification(
                                                        format!("核心已下载：{}", path.display()),
                                                        cx,
                                                    );
                                                    cx.refresh_windows();
                                                }
                                                Err(err) => window
                                                    .push_notification(format!("下载失败：{err}"), cx),
                                            });
                                        }
                                    })
                                    .detach();
                            }),
                    ),
            ),
        colors,
    )
}

fn render_log_settings(
    cx: &mut Context<Panel>,
    log_level_select: &Entity<SelectState<Vec<LogLevelOption>>>,
    colors: ThemeColors,
) -> gpui::Div {
    let current = cx.global::<AppState>().settings.ui_log_level;

    section(
        "日志设置",
        gpui::div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .child(
                gpui::div()
                    .text_size(px(13.))
                    .text_color(colors.text_secondary)
                    .child("控制菜单与对话框相关的 UI 调试日志输出级别"),
            )
            .child(setting_row(
                "UI 日志级别",
                gpui::div()
                    .w(px(160.))
                    .child(Select::new(log_level_select)),
                colors,
            ))
            .child(
                gpui::div()
                    .p(px(12.))
                    .rounded(px(6.))
                    .bg(colors.element_bg)
                    .flex()
                    .flex_col()
                    .gap(px(6.))
                    .child(
                        gpui::div()
                            .text_size(px(12.))
                            .text_color(colors.text_disabled)
                            .child(format!("当前级别：{}", current.label())),
                    )
                    .child(level_hint("None", "不输出 UI 调试日志", colors))
                    .child(level_hint("Info", "输出菜单点击、对话框打开等信息", colors))
                    .child(level_hint("Error", "仅输出错误级别日志", colors))
                    .child(level_hint("Debug", "输出全部 UI 调试日志", colors)),
            ),
        colors,
    )
}

fn render_about(colors: ThemeColors) -> gpui::Div {
    section(
        "关于",
        gpui::div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .child(
                gpui::div()
                    .text_size(px(18.))
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(colors.text_primary)
                    .child("song-bin"),
            )
            .child(
                gpui::div()
                    .text_size(px(13.))
                    .text_color(colors.text_secondary)
                    .child("版本 0.1.0"),
            )
            .child(
                gpui::div()
                    .text_size(px(13.))
                    .text_color(colors.text_secondary)
                    .child("基于 sing-box 的代理客户端"),
            ),
        colors,
    )
}

fn render_placeholder_section(
    title: &str,
    description: &str,
    colors: ThemeColors,
) -> gpui::Div {
    section(
        title,
        gpui::div()
            .p(px(12.))
            .rounded(px(6.))
            .bg(colors.element_bg)
            .text_size(px(13.))
            .text_color(colors.text_secondary)
            .child(description.to_string()),
        colors,
    )
}

fn section(title: &str, body: impl IntoElement, colors: ThemeColors) -> gpui::Div {
    gpui::div()
        .flex()
        .flex_col()
        .gap(px(16.))
        .child(
            gpui::div()
                .text_size(px(18.))
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(colors.text_primary)
                .child(title.to_string()),
        )
        .child(body)
}

fn level_hint(level: &str, desc: &str, colors: ThemeColors) -> gpui::Div {
    gpui::div()
        .flex()
        .gap(px(8.))
        .child(
            gpui::div()
                .w(px(48.))
                .text_size(px(12.))
                .font_weight(gpui::FontWeight::MEDIUM)
                .text_color(colors.text_primary)
                .child(level.to_string()),
        )
        .child(
            gpui::div()
                .text_size(px(12.))
                .text_color(colors.text_disabled)
                .child(desc.to_string()),
        )
}

pub fn setting_row(
    label: &str,
    value: impl IntoElement,
    colors: ThemeColors,
) -> gpui::Div {
    gpui::div()
        .flex()
        .items_center()
        .justify_between()
        .gap(px(12.))
        .py(px(8.))
        .border_b_1()
        .border_color(colors.border)
        .child(
            gpui::div()
                .flex_none()
                .text_size(px(13.))
                .text_color(colors.text_secondary)
                .child(label.to_string()),
        )
        .child(
            gpui::div()
                .flex_1()
                .min_w(px(0.))
                .flex()
                .justify_end()
                .items_center()
                .child(value),
        )
}

pub fn setting_row_text(label: &str, value: &str, colors: ThemeColors) -> gpui::Div {
    setting_row(
        label,
        gpui::div()
            .text_size(px(13.))
            .text_color(colors.text_primary)
            .child(value.to_string()),
        colors,
    )
}

pub fn build_log_level_options(active: UiLogLevel) -> Vec<LogLevelOption> {
    UiLogLevel::all()
        .iter()
        .copied()
        .map(|level| LogLevelOption::new(level, level == active))
        .collect()
}
