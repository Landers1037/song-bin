use gpui::{prelude::*, px, Context, Entity, Render, Window};

use crate::state::app_state::AppState;
use crate::theme;
use crate::ui::components::list_item::ListItem;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SidebarTab {
    Nodes,
    Subscriptions,
}

pub struct Sidebar {
    pub active_tab: SidebarTab,
    pub selected_index: Option<usize>,
}

impl Sidebar {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            active_tab: SidebarTab::Nodes,
            selected_index: None,
        }
    }
}

impl Render for Sidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = &theme::theme(cx).colors;
        let active_tab = self.active_tab;
        let selected_index = self.selected_index;

        let app_state = cx.global::<AppState>();
        let items: Vec<(String, String)> = match active_tab {
            SidebarTab::Nodes => app_state
                .nodes
                .iter()
                .map(|n| (n.name.clone(), format!("{} · {}", n.protocol_label(), n.display_address())))
                .collect(),
            SidebarTab::Subscriptions => app_state
                .subscriptions
                .iter()
                .map(|s| (s.name.clone(), format!("{} nodes", s.node_count())))
                .collect(),
        };

        gpui::div()
            .flex()
            .flex_col()
            .w(px(240.))
            .h_full()
            .flex_none()
            .bg(colors.sidebar_bg)
            .border_r_1()
            .border_color(colors.border)
            .child(
                gpui::div()
                    .flex()
                    .w_full()
                    .border_b_1()
                    .border_color(colors.border)
                    .child(tab_button(
                        "Nodes",
                        active_tab == SidebarTab::Nodes,
                        cx.entity().clone(),
                        SidebarTab::Nodes,
                        colors.accent,
                        colors.text_secondary,
                    ))
                    .child(tab_button(
                        "Subs",
                        active_tab == SidebarTab::Subscriptions,
                        cx.entity().clone(),
                        SidebarTab::Subscriptions,
                        colors.accent,
                        colors.text_secondary,
                    )),
            )
            .child(
                gpui::div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_y_hidden()
                    .children(items.into_iter().enumerate().map(move |(i, (name, sub))| {
                        let is_selected = selected_index == Some(i);
                        ListItem::new(name).sublabel(sub).selected(is_selected)
                    })),
            )
    }
}

fn tab_button(
    label: &str,
    active: bool,
    entity: Entity<Sidebar>,
    tab: SidebarTab,
    accent: gpui::Hsla,
    text_secondary: gpui::Hsla,
) -> impl IntoElement {
    let text_color = if active { accent } else { text_secondary };
    let bottom_border = if active { accent } else { gpui::transparent_black() };

    gpui::div()
        .id(gpui::SharedString::from(label.to_string()))
        .flex_1()
        .flex()
        .items_center()
        .justify_center()
        .py(px(8.))
        .text_size(px(12.))
        .font_weight(gpui::FontWeight::MEDIUM)
        .text_color(text_color)
        .border_b(px(2.))
        .border_color(bottom_border)
        .cursor_pointer()
        .on_click(move |_, _, cx| {
            entity.update(cx, |sidebar, cx| {
                sidebar.active_tab = tab;
                sidebar.selected_index = None;
                cx.notify();
            });
        })
        .child(label.to_string())
}
