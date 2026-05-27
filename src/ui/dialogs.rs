use gpui::{prelude::*, App, Entity, SharedString, Window, px};

use gpui_component::{
    Root, WindowExt as _,
    button::{Button, ButtonVariants as _},
    dialog::{
        DialogAction, DialogClose, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    },
    input::{Input, InputState},
    v_flex,
};

use crate::proxy::import::{self, ImportKind, ImportResult};
use crate::proxy::node::{ProxyNode, ProxyProtocol};
use crate::proxy::protocol::anytls::AnyTlsConfig;
use crate::proxy::protocol::trojan::TrojanConfig;
use crate::proxy::protocol::vless::{TlsConfig, VlessConfig};
use crate::proxy::protocol::vmess::VMessConfig;
use crate::state::app_state::AppState;
use crate::ui::sidebar::{Sidebar, SidebarTab};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewNodeProtocol {
    Vless,
    Vmess,
    AnyTls,
    Trojan,
}

impl NewNodeProtocol {
    fn title(self) -> &'static str {
        match self {
            Self::Vless => "新建 VLESS 节点",
            Self::Vmess => "新建 VMess 节点",
            Self::AnyTls => "新建 AnyTLS 节点",
            Self::Trojan => "新建 Trojan 节点",
        }
    }
}

pub fn open_import_node_url_dialog(
    window: &mut Window,
    cx: &mut App,
    sidebar: Entity<Sidebar>,
) {
    log::info!("[dialog] 触发打开: 从 URL 中导入节点");
    open_url_dialog(
        window,
        cx,
        sidebar,
        ImportKind::Node,
        "从 URL 中导入节点",
        "粘贴节点分享链接或订阅 URL",
    );
}

pub fn open_import_subscription_url_dialog(
    window: &mut Window,
    cx: &mut App,
    sidebar: Entity<Sidebar>,
) {
    log::info!("[dialog] 触发打开: 从 URL 中导入订阅");
    open_url_dialog(
        window,
        cx,
        sidebar,
        ImportKind::Subscription,
        "从 URL 中导入订阅",
        "粘贴订阅链接（http/https）",
    );
}

fn open_url_dialog(
    window: &mut Window,
    cx: &mut App,
    sidebar: Entity<Sidebar>,
    kind: ImportKind,
    title: &'static str,
    hint: &'static str,
) {
    let url_input = new_input(window, cx, "https://");

    window.open_dialog(cx, move |dialog, _window, _cx| {
        let url_for_content = url_input.clone();
        let url_for_ok = url_input.clone();
        let sidebar_for_ok = sidebar.clone();

        dialog
            .w(px(480.))
            .keyboard(true)
            .overlay_closable(true)
            .content(move |content, _, _| {
                content
                    .child(
                        DialogHeader::new()
                            .p_4()
                            .child(DialogTitle::new().child(title))
                            .child(DialogDescription::new().child(hint)),
                    )
                    .child(v_flex().px_4().pb_4().gap_3().child(Input::new(&url_for_content)))
                    .child(
                        DialogFooter::new()
                            .p_4()
                            .justify_end()
                            .gap_2()
                            .child(DialogClose::new().child(
                                Button::new("cancel-import").outline().label("取消"),
                            ))
                            .child(DialogAction::new().child(
                                Button::new("confirm-import").primary().label("导入"),
                            )),
                    )
            })
            .on_ok(move |_, window, cx| {
                finish_url_import(window, cx, &url_for_ok, kind, sidebar_for_ok.clone())
            })
    });
}

fn finish_url_import(
    window: &mut Window,
    cx: &mut App,
    url_input: &Entity<InputState>,
    kind: ImportKind,
    sidebar: Entity<Sidebar>,
) -> bool {
    let url = url_input.read(cx).value().trim().to_string();
    match import::import_from_url(&url, kind) {
        Ok(result) => {
            let msg = import_success_message(&result);
            import::apply_import(cx.global_mut::<AppState>(), result);
            switch_sidebar_tab(
                &sidebar,
                match kind {
                    ImportKind::Node => SidebarTab::Nodes,
                    ImportKind::Subscription => SidebarTab::Subscriptions,
                },
                cx,
            );
            cx.refresh_windows();
            window.push_notification(msg, cx);
            true
        }
        Err(err) => {
            window.push_notification(format!("导入失败: {}", err), cx);
            false
        }
    }
}

fn import_success_message(result: &ImportResult) -> SharedString {
    match result {
        ImportResult::Nodes(nodes) => format!("已导入 {} 个节点", nodes.len()).into(),
        ImportResult::Subscription(sub) => {
            format!("已添加订阅「{}」，共 {} 个节点", sub.name, sub.node_count()).into()
        }
    }
}

pub fn open_new_node_dialog(
    window: &mut Window,
    cx: &mut App,
    sidebar: Entity<Sidebar>,
    protocol: NewNodeProtocol,
) {
    log::info!("[dialog] 触发打开: {}", protocol.title());
    let name_input = new_input(window, cx, "节点名称");
    let server_input = new_input(window, cx, "服务器地址");
    let port_input = new_input(window, cx, "443");
    let secret_input = new_input(window, cx, protocol_secret_placeholder(protocol));
    let extra_input = new_input(window, cx, protocol_extra_placeholder(protocol));
    let title = protocol.title();

    window.open_dialog(cx, move |dialog, _, _| {
        let name_for_content = name_input.clone();
        let server_for_content = server_input.clone();
        let port_for_content = port_input.clone();
        let secret_for_content = secret_input.clone();
        let extra_for_content = extra_input.clone();

        let name_for_ok = name_input.clone();
        let server_for_ok = server_input.clone();
        let port_for_ok = port_input.clone();
        let secret_for_ok = secret_input.clone();
        let extra_for_ok = extra_input.clone();
        let sidebar_for_ok = sidebar.clone();

        dialog
            .w(px(440.))
            .keyboard(true)
            .overlay_closable(true)
            .content(move |content, _, _| {
                content
                    .child(
                        DialogHeader::new()
                            .p_4()
                            .child(DialogTitle::new().child(title)),
                    )
                    .child(
                        v_flex()
                            .px_4()
                            .pb_4()
                            .gap_3()
                            .child(form_field("名称", &name_for_content))
                            .child(form_field("地址", &server_for_content))
                            .child(form_field("端口", &port_for_content))
                            .child(form_field(protocol_secret_label(protocol), &secret_for_content))
                            .child(form_field(protocol_extra_label(protocol), &extra_for_content)),
                    )
                    .child(
                        DialogFooter::new()
                            .p_4()
                            .justify_end()
                            .gap_2()
                            .child(DialogClose::new().child(
                                Button::new("cancel-node").outline().label("取消"),
                            ))
                            .child(DialogAction::new().child(
                                Button::new("confirm-node").primary().label("保存"),
                            )),
                    )
            })
            .on_ok(move |_, window, cx| {
                finish_new_node(
                    window,
                    cx,
                    protocol,
                    sidebar_for_ok.clone(),
                    &name_for_ok,
                    &server_for_ok,
                    &port_for_ok,
                    &secret_for_ok,
                    &extra_for_ok,
                )
            })
    });
}

fn finish_new_node(
    window: &mut Window,
    cx: &mut App,
    protocol: NewNodeProtocol,
    sidebar: Entity<Sidebar>,
    name_input: &Entity<InputState>,
    server_input: &Entity<InputState>,
    port_input: &Entity<InputState>,
    secret_input: &Entity<InputState>,
    extra_input: &Entity<InputState>,
) -> bool {
    let name = name_input.read(cx).value().trim().to_string();
    let server = server_input.read(cx).value().trim().to_string();
    let port_str = port_input.read(cx).value().trim().to_string();
    let secret = secret_input.read(cx).value().trim().to_string();
    let extra = extra_input.read(cx).value().trim().to_string();

    if name.is_empty() || server.is_empty() {
        window.push_notification("请填写名称和地址", cx);
        return false;
    }
    let port: u16 = match port_str.parse() {
        Ok(p) if p > 0 => p,
        _ => {
            window.push_notification("端口无效", cx);
            return false;
        }
    };
    if secret.is_empty() {
        window.push_notification("请填写认证信息", cx);
        return false;
    }

    let tls = TlsConfig {
        enabled: true,
        server_name: if looks_like_sni(&extra) {
            extra.clone()
        } else {
            server.clone()
        },
        ..Default::default()
    };

    let proxy_protocol = build_protocol(protocol, &secret, &extra, tls);
    let node = ProxyNode::new(name, server, port, proxy_protocol);
    cx.global_mut::<AppState>().add_node(node);
    cx.global_mut::<AppState>().save_all();
    switch_sidebar_tab(&sidebar, SidebarTab::Nodes, cx);
    cx.refresh_windows();
    window.push_notification("节点已添加", cx);
    true
}

fn looks_like_sni(s: &str) -> bool {
    !s.is_empty() && !s.chars().all(|c| c.is_ascii_digit())
}

fn build_protocol(
    protocol: NewNodeProtocol,
    secret: &str,
    extra: &str,
    tls: TlsConfig,
) -> ProxyProtocol {
    match protocol {
        NewNodeProtocol::Vless => ProxyProtocol::Vless(VlessConfig {
            uuid: secret.to_string(),
            flow: "xtls-rprx-vision".into(),
            tls,
            transport: None,
        }),
        NewNodeProtocol::Vmess => ProxyProtocol::Vmess(VMessConfig {
            uuid: secret.to_string(),
            alter_id: extra.parse().unwrap_or(0),
            security: "auto".into(),
            tls,
            transport: None,
        }),
        NewNodeProtocol::AnyTls => ProxyProtocol::AnyTls(AnyTlsConfig {
            password: secret.to_string(),
            tls,
            ..Default::default()
        }),
        NewNodeProtocol::Trojan => ProxyProtocol::Trojan(TrojanConfig {
            password: secret.to_string(),
            tls,
            transport: None,
        }),
    }
}

fn new_input(window: &mut Window, cx: &mut App, placeholder: &str) -> Entity<InputState> {
    let ph: SharedString = placeholder.into();
    Root::update(window, cx, |_, window, cx| {
        cx.new(|cx| InputState::new(window, cx).placeholder(ph))
    })
}

fn form_field(label: &'static str, input: &Entity<InputState>) -> impl gpui::IntoElement {
    v_flex()
        .gap_1()
        .child(
            gpui::div()
                .text_size(px(12.))
                .child(label.to_string()),
        )
        .child(Input::new(input))
}

fn protocol_secret_label(protocol: NewNodeProtocol) -> &'static str {
    match protocol {
        NewNodeProtocol::Vless | NewNodeProtocol::Vmess => "UUID",
        NewNodeProtocol::AnyTls | NewNodeProtocol::Trojan => "密码",
    }
}

fn protocol_secret_placeholder(protocol: NewNodeProtocol) -> &'static str {
    protocol_secret_label(protocol)
}

fn protocol_extra_label(protocol: NewNodeProtocol) -> &'static str {
    match protocol {
        NewNodeProtocol::Vless => "SNI（可选）",
        NewNodeProtocol::Vmess => "Alter ID（可选）",
        NewNodeProtocol::AnyTls | NewNodeProtocol::Trojan => "SNI（可选）",
    }
}

fn protocol_extra_placeholder(protocol: NewNodeProtocol) -> &'static str {
    match protocol {
        NewNodeProtocol::Vless => "留空则使用服务器地址",
        NewNodeProtocol::Vmess => "0",
        NewNodeProtocol::AnyTls | NewNodeProtocol::Trojan => "留空则使用服务器地址",
    }
}

pub fn switch_sidebar_tab(sidebar: &Entity<Sidebar>, tab: SidebarTab, cx: &mut App) {
    sidebar.update(cx, |s, cx| {
        s.active_tab = tab;
        s.selected_index = None;
        cx.notify();
    });
}
