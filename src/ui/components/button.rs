use gpui::{prelude::*, px, App, IntoElement, SharedString, Window};

use gpui_component::{Icon, IconName, Sizable, Size};

use crate::theme;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
}

#[derive(IntoElement)]
pub struct Button {
    label: SharedString,
    icon: Option<IconName>,
    variant: ButtonVariant,
    on_click: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl Button {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            variant: ButtonVariant::Secondary,
            on_click: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = &theme::theme(cx).colors;

        let (bg, text_color) = match self.variant {
            ButtonVariant::Primary => (colors.accent, colors.text_on_accent),
            ButtonVariant::Secondary => (colors.element_bg, colors.text_primary),
            ButtonVariant::Ghost => (gpui::transparent_black(), colors.text_primary),
        };

        let hover_bg = match self.variant {
            ButtonVariant::Primary => colors.accent_hover,
            ButtonVariant::Secondary => colors.element_hover,
            ButtonVariant::Ghost => colors.element_hover,
        };

        let label = self.label.clone();
        let button_id = if label.is_empty() {
            SharedString::from("btn-icon")
        } else {
            SharedString::from(format!("btn-{}", label))
        };

        let content = gpui::div()
            .flex()
            .items_center()
            .gap(px(4.))
            .when_some(self.icon, |this, icon| {
                this.child(Icon::new(icon).with_size(Size::Small))
            })
            .when(!label.is_empty(), |this| this.child(label));

        let mut el = gpui::div()
            .id(button_id)
            .flex_none()
            .px(px(12.))
            .py(px(4.))
            .rounded(px(4.))
            .bg(bg)
            .text_color(text_color)
            .text_size(px(13.))
            .cursor_pointer()
            .hover(move |style| style.bg(hover_bg))
            .active(|style| style.opacity(0.8))
            .block_mouse_except_scroll()
            .child(content);

        if let Some(on_click) = self.on_click {
            el = el.on_click(move |_, window, cx| on_click(window, cx));
        }

        el
    }
}
