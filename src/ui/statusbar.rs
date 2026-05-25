use gpui::{prelude::*, px, Context, Render, SharedString, Window};

use crate::theme;

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ProxyMode {
    Rule,
    Global,
    Direct,
}

impl ProxyMode {
    pub fn label(&self) -> &'static str {
        match self {
            ProxyMode::Rule => "Rule",
            ProxyMode::Global => "Global",
            ProxyMode::Direct => "Direct",
        }
    }
}

pub struct StatusBar {
    pub active_node: Option<String>,
    pub active_protocol: Option<String>,
    pub proxy_mode: ProxyMode,
    pub upload_speed: String,
    pub download_speed: String,
    pub connected: bool,
}

impl StatusBar {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            active_node: None,
            active_protocol: None,
            proxy_mode: ProxyMode::Rule,
            upload_speed: "0 B/s".into(),
            download_speed: "0 B/s".into(),
            connected: false,
        }
    }
}

impl Render for StatusBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = &theme::theme(cx).colors;

        let status_dot_color = if self.connected {
            colors.success
        } else {
            colors.text_disabled
        };

        let node_text: SharedString = self
            .active_node
            .clone()
            .unwrap_or_else(|| "No active node".to_string())
            .into();

        let mode_label: SharedString = self.proxy_mode.label().into();
        let upload: SharedString = format!("↑ {}", self.upload_speed).into();
        let download: SharedString = format!("↓ {}", self.download_speed).into();

        gpui::div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(28.))
            .px(px(12.))
            .bg(colors.statusbar_bg)
            .border_t_1()
            .border_color(colors.border)
            .text_size(px(11.))
            .child(
                // Left: status + active node
                gpui::div()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .child(
                        gpui::div()
                            .w(px(6.))
                            .h(px(6.))
                            .rounded_full()
                            .bg(status_dot_color),
                    )
                    .child(
                        gpui::div()
                            .text_color(colors.text_secondary)
                            .child(node_text),
                    )
                    .when_some(self.active_protocol.clone(), |this, proto| {
                        this.child(
                            gpui::div()
                                .px(px(4.))
                                .py(px(1.))
                                .rounded(px(2.))
                                .bg(colors.element_bg)
                                .text_size(px(9.))
                                .text_color(colors.text_secondary)
                                .child(proto),
                        )
                    }),
            )
            .child(
                // Center: proxy mode
                gpui::div()
                    .flex()
                    .items_center()
                    .child(
                        gpui::div()
                            .px(px(8.))
                            .py(px(2.))
                            .rounded(px(3.))
                            .bg(colors.element_bg)
                            .text_color(colors.text_primary)
                            .child(mode_label),
                    ),
            )
            .child(
                // Right: speed
                gpui::div()
                    .flex()
                    .items_center()
                    .gap(px(12.))
                    .child(
                        gpui::div()
                            .text_color(colors.success)
                            .child(upload),
                    )
                    .child(
                        gpui::div()
                            .text_color(colors.accent)
                            .child(download),
                    ),
            )
    }
}
