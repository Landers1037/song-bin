use gpui::{prelude::*, Context, Entity, Render, Window};

use crate::theme;
use crate::ui::panel::Panel;
use crate::ui::sidebar::Sidebar;
use crate::ui::statusbar::StatusBar;
use crate::ui::titlebar::TitleBar;

pub struct MainView {
    sidebar: Entity<Sidebar>,
    panel: Entity<Panel>,
    status_bar: Entity<StatusBar>,
}

impl MainView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let sidebar = cx.new(|cx| Sidebar::new(cx));
        let panel = cx.new(|cx| Panel::new(window, cx));
        let status_bar = cx.new(|cx| StatusBar::new(cx));

        Self {
            sidebar,
            panel,
            status_bar,
        }
    }
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = &theme::theme(cx).colors;

        gpui::div()
            .flex()
            .flex_col()
            .size_full()
            .bg(colors.background)
            .text_color(colors.text_primary)
            .child(TitleBar::new(self.sidebar.clone(), self.panel.clone()))
            .child(
                gpui::div()
                    .flex()
                    .flex_1()
                    .overflow_hidden()
                    .child(self.sidebar.clone())
                    .child(self.panel.clone()),
            )
            .child(self.status_bar.clone())
    }
}
