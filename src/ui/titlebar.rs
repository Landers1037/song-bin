use gpui::{prelude::*, px, Anchor, App, Entity, IntoElement, SharedString, Window, WindowControlArea};

use gpui_component::{
    ActiveTheme as _, Icon, IconName, Sizable, Size, ThemeMode,
    button::{Button, ButtonVariants as _},
    menu::{DropdownMenu as _, PopupMenuItem},
};

use crate::ui::dialogs::{self, NewNodeProtocol};

use crate::state::app_state::AppState;
use crate::utils::app_log;
use crate::theme::{self, ThemeKind};
use crate::ui::components::button::{Button as AppButton, ButtonVariant};
use crate::ui::panel::{Panel, PanelContent};
use crate::ui::sidebar::{Sidebar, SidebarTab};

#[derive(IntoElement)]
pub struct TitleBar {
    sidebar: Entity<Sidebar>,
    panel: Entity<Panel>,
}

impl TitleBar {
    pub fn new(
        sidebar: Entity<Sidebar>,
        panel: Entity<Panel>,
    ) -> Self {
        Self {
            sidebar,
            panel,
        }
    }
}

impl RenderOnce for TitleBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = &theme::theme(cx).colors;
        let is_dark = theme::theme(cx).current == ThemeKind::Dark;
        let theme_icon = if is_dark {
            IconName::Sun
        } else {
            IconName::Moon
        };

        let sidebar = self.sidebar.clone();
        let panel = self.panel.clone();

        gpui::div()
            .id("titlebar")
            .flex()
            .items_center()
            .w_full()
            .h(px(38.))
            .px(px(12.))
            .bg(colors.titlebar_bg)
            .border_b_1()
            .border_color(colors.border)
            .child(
                gpui::div()
                    .flex()
                    .items_center()
                    .gap(px(16.))
                    .flex_none()
                    .child(
                        gpui::div()
                            .text_size(px(14.))
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(colors.accent)
                            .child("song-bin"),
                    )
                    .child(nodes_dropdown_menu(
                        sidebar.clone(),
                        panel.clone(),
                        colors.text_secondary,
                    ))
                    .child(menu_item(
                        "订阅",
                        IconName::BookOpen,
                        colors.text_secondary,
                        {
                            let sidebar = sidebar.clone();
                            let panel = panel.clone();
                            move |_, _, cx| {
                                app_log::ui_info(cx, "[titlebar] 菜单点击: 订阅");
                                sidebar.update(cx, |s, cx| {
                                    s.active_tab = SidebarTab::Subscriptions;
                                    s.selected_index = None;
                                    cx.notify();
                                });
                                panel.update(cx, |p, cx| {
                                    p.content = PanelContent::Welcome;
                                    cx.notify();
                                });
                            }
                        },
                    ))
                    .child(menu_item(
                        "日志",
                        IconName::SquareTerminal,
                        colors.text_secondary,
                        {
                            let panel = panel.clone();
                            move |_, _, cx| {
                                panel.update(cx, |p, cx| {
                                    p.content = PanelContent::Logs;
                                    cx.notify();
                                });
                            }
                        },
                    )),
            )
            .child(
                gpui::div()
                    .id("titlebar-drag")
                    .flex_1()
                    .h_full()
                    .window_control_area(WindowControlArea::Drag),
            )
            .child(
                gpui::div()
                    .flex()
                    .items_center()
                    .gap(px(4.))
                    .flex_none()
                    .child(
                        AppButton::new("")
                            .variant(ButtonVariant::Ghost)
                            .icon(theme_icon)
                            .on_click(|window, cx| {
                                let mode = if cx.theme().mode.is_dark() {
                                    ThemeMode::Light
                                } else {
                                    ThemeMode::Dark
                                };
                                theme::set_theme_mode(mode, Some(window), cx);
                                cx.global_mut::<AppState>().settings.theme = theme::theme(cx).current;
                                let _ = cx.global_mut::<AppState>().settings.save();
                            }),
                    )
                    .child(
                        AppButton::new("设置")
                            .variant(ButtonVariant::Ghost)
                            .icon(IconName::Settings2)
                            .on_click({
                                let panel = panel.clone();
                                move |_window, cx| {
                                    panel.update(cx, |p, cx| {
                                        p.content = PanelContent::Settings;
                                        cx.notify();
                                    });
                                }
                            }),
                    )
                    .child(window_control_button("—", WindowControlArea::Min, |window, _cx| {
                        window.minimize_window();
                    }))
                    .child(window_control_button("□", WindowControlArea::Max, |window, _cx| {
                        window.toggle_fullscreen();
                    }))
                    .child(window_close_button()),
            )
    }
}

fn nodes_dropdown_menu(
    sidebar: Entity<Sidebar>,
    panel: Entity<Panel>,
    color: gpui::Hsla,
) -> impl IntoElement {
    Button::new("menu-nodes")
        .ghost()
        .small()
        .compact()
        .text_size(px(12.))
        .text_color(color)
        .icon(IconName::Network)
        .label("节点")
        .dropdown_caret(true)
        .dropdown_menu_with_anchor(Anchor::BottomLeft, move |menu, _, _| {
            let sidebar = sidebar.clone();
            let panel = panel.clone();

            menu.min_w(px(200.))
                .item(PopupMenuItem::new("显示节点列表").on_click({
                    let sidebar = sidebar.clone();
                    let panel = panel.clone();
                    move |_, _, cx| {
                        sidebar.update(cx, |s, cx| {
                            s.active_tab = SidebarTab::Nodes;
                            s.selected_index = None;
                            cx.notify();
                        });
                        panel.update(cx, |p, cx| {
                            p.content = PanelContent::Welcome;
                            cx.notify();
                        });
                    }
                }))
                .separator()
                .item(PopupMenuItem::new("从 URL 中导入节点").on_click({
                    let sidebar = sidebar.clone();
                    move |_, window, cx| {
                        app_log::ui_info(cx, "[titlebar] 节点菜单点击: 从 URL 中导入节点");
                        dialogs::open_import_node_url_dialog(window, cx, sidebar.clone());
                    }
                }))
                .item(PopupMenuItem::new("从 URL 中导入订阅").on_click({
                    let sidebar = sidebar.clone();
                    move |_, window, cx| {
                        app_log::ui_info(cx, "[titlebar] 节点菜单点击: 从 URL 中导入订阅");
                        dialogs::open_import_subscription_url_dialog(window, cx, sidebar.clone());
                    }
                }))
                .separator()
                .item(PopupMenuItem::new("新建 VLESS 节点").on_click({
                    let sidebar = sidebar.clone();
                    move |_, window, cx| {
                        app_log::ui_info(cx, "[titlebar] 节点菜单点击: 新建 VLESS 节点");
                        dialogs::open_new_node_dialog(window, cx, sidebar.clone(), NewNodeProtocol::Vless);
                    }
                }))
                .item(PopupMenuItem::new("新建 VMess 节点").on_click({
                    let sidebar = sidebar.clone();
                    move |_, window, cx| {
                        app_log::ui_info(cx, "[titlebar] 节点菜单点击: 新建 VMess 节点");
                        dialogs::open_new_node_dialog(window, cx, sidebar.clone(), NewNodeProtocol::Vmess);
                    }
                }))
                .item(PopupMenuItem::new("新建 AnyTLS 节点").on_click({
                    let sidebar = sidebar.clone();
                    move |_, window, cx| {
                        app_log::ui_info(cx, "[titlebar] 节点菜单点击: 新建 AnyTLS 节点");
                        dialogs::open_new_node_dialog(window, cx, sidebar.clone(), NewNodeProtocol::AnyTls);
                    }
                }))
                .item(PopupMenuItem::new("新建 Trojan 节点").on_click({
                    let sidebar = sidebar.clone();
                    move |_, window, cx| {
                        app_log::ui_info(cx, "[titlebar] 节点菜单点击: 新建 Trojan 节点");
                        dialogs::open_new_node_dialog(window, cx, sidebar.clone(), NewNodeProtocol::Trojan);
                    }
                }))
        })
}

fn menu_item(
    label: &str,
    icon: IconName,
    color: gpui::Hsla,
    on_click: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    gpui::div()
        .id(SharedString::from(format!("menu-{}", label)))
        .flex()
        .items_center()
        .gap(px(4.))
        .text_size(px(12.))
        .text_color(color)
        .cursor_pointer()
        .block_mouse_except_scroll()
        .hover(move |style| style.text_color(gpui::hsla(0., 0., 0.9, 1.0)))
        .on_click(on_click)
        .child(Icon::new(icon).with_size(Size::Small))
        .child(label.to_string())
}

fn window_control_button(
    label: &str,
    area: WindowControlArea,
    on_click: impl Fn(&mut Window, &mut App) + 'static,
) -> impl IntoElement {
    gpui::div()
        .id(SharedString::from(format!("wc-{}", label)))
        .flex_none()
        .w(px(32.))
        .h(px(26.))
        .flex()
        .items_center()
        .justify_center()
        .text_size(px(13.))
        .text_color(gpui::hsla(0., 0., 0.6, 1.0))
        .rounded(px(4.))
        .cursor_pointer()
        .window_control_area(area)
        .hover(|style| style.bg(gpui::hsla(0., 0., 1.0, 0.1)))
        .on_click(move |_, window, cx| on_click(window, cx))
        .child(label.to_string())
}

fn window_close_button() -> impl IntoElement {
    gpui::div()
        .id("wc-close")
        .flex_none()
        .w(px(32.))
        .h(px(26.))
        .flex()
        .items_center()
        .justify_center()
        .text_size(px(13.))
        .text_color(gpui::hsla(0., 0., 0.6, 1.0))
        .rounded(px(4.))
        .cursor_pointer()
        .window_control_area(WindowControlArea::Close)
        .hover(|style| style.bg(gpui::hsla(0., 0.8, 0.5, 1.0)).text_color(gpui::white()))
        .on_click(|_, window, _cx| {
            window.remove_window();
        })
        .child("✕")
}
