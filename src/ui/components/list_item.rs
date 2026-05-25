use gpui::{prelude::*, px, App, IntoElement, SharedString, Window};

use crate::theme;

#[derive(IntoElement)]
pub struct ListItem {
    label: SharedString,
    sublabel: Option<SharedString>,
    selected: bool,
    on_click: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl ListItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            sublabel: None,
            selected: false,
            on_click: None,
        }
    }

    pub fn sublabel(mut self, sublabel: impl Into<SharedString>) -> Self {
        self.sublabel = Some(sublabel.into());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for ListItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = &theme::theme(cx).colors;

        let bg = if self.selected {
            colors.element_selected
        } else {
            gpui::transparent_black()
        };

        let left_border = if self.selected {
            colors.accent
        } else {
            gpui::transparent_black()
        };

        let mut el = gpui::div()
            .id(self.label.clone())
            .w_full()
            .px(px(12.))
            .py(px(8.))
            .bg(bg)
            .border_l(px(2.))
            .border_color(left_border)
            .cursor_pointer()
            .hover(move |style| style.bg(colors.element_hover))
            .child(
                gpui::div()
                    .flex()
                    .flex_col()
                    .gap(px(2.))
                    .child(
                        gpui::div()
                            .text_size(px(13.))
                            .text_color(colors.text_primary)
                            .child(self.label),
                    )
                    .when_some(self.sublabel, |this, sub| {
                        this.child(
                            gpui::div()
                                .text_size(px(11.))
                                .text_color(colors.text_secondary)
                                .child(sub),
                        )
                    }),
            );

        if let Some(on_click) = self.on_click {
            el = el.on_click(move |_, window, cx| on_click(window, cx));
        }

        el
    }
}
