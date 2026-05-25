use gpui::{hsla, Hsla};

#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub background: Hsla,
    pub surface: Hsla,
    pub sidebar_bg: Hsla,
    pub titlebar_bg: Hsla,
    pub statusbar_bg: Hsla,

    pub text_primary: Hsla,
    pub text_secondary: Hsla,
    pub text_disabled: Hsla,
    pub text_on_accent: Hsla,

    pub accent: Hsla,
    pub accent_hover: Hsla,

    pub border: Hsla,
    pub border_light: Hsla,

    pub element_bg: Hsla,
    pub element_hover: Hsla,
    pub element_selected: Hsla,

    pub success: Hsla,
    pub warning: Hsla,
    pub error: Hsla,

    pub scrollbar: Hsla,
}

impl ThemeColors {
    pub fn from_component(theme: &gpui_component::ThemeColor) -> Self {
        Self {
            background: theme.background,
            surface: theme.popover,
            sidebar_bg: theme.sidebar,
            titlebar_bg: theme.title_bar,
            statusbar_bg: theme.title_bar,
            text_primary: theme.foreground,
            text_secondary: theme.muted_foreground,
            text_disabled: theme.muted_foreground,
            text_on_accent: theme.primary_foreground,
            accent: theme.primary,
            accent_hover: theme.primary_hover,
            border: theme.border,
            border_light: theme.border,
            element_bg: theme.muted,
            element_hover: theme.list_hover,
            element_selected: theme.list_active,
            success: theme.success,
            warning: theme.warning,
            error: theme.danger,
            scrollbar: theme.scrollbar_thumb,
        }
    }

    pub fn dark() -> Self {
        Self {
            background: hsla(220. / 360., 0.16, 0.12, 1.0),
            surface: hsla(220. / 360., 0.16, 0.15, 1.0),
            sidebar_bg: hsla(220. / 360., 0.18, 0.10, 1.0),
            titlebar_bg: hsla(220. / 360., 0.18, 0.09, 1.0),
            statusbar_bg: hsla(220. / 360., 0.18, 0.09, 1.0),

            text_primary: hsla(0., 0., 0.93, 1.0),
            text_secondary: hsla(0., 0., 0.60, 1.0),
            text_disabled: hsla(0., 0., 0.38, 1.0),
            text_on_accent: hsla(0., 0., 1.0, 1.0),

            accent: hsla(215. / 360., 0.92, 0.56, 1.0),
            accent_hover: hsla(215. / 360., 0.92, 0.64, 1.0),

            border: hsla(0., 0., 1.0, 0.08),
            border_light: hsla(0., 0., 1.0, 0.05),

            element_bg: hsla(0., 0., 1.0, 0.06),
            element_hover: hsla(0., 0., 1.0, 0.10),
            element_selected: hsla(215. / 360., 0.92, 0.56, 0.20),

            success: hsla(142. / 360., 0.71, 0.45, 1.0),
            warning: hsla(38. / 360., 0.92, 0.50, 1.0),
            error: hsla(0., 0.84, 0.60, 1.0),

            scrollbar: hsla(0., 0., 1.0, 0.15),
        }
    }

    pub fn light() -> Self {
        Self {
            background: hsla(0., 0., 0.98, 1.0),
            surface: hsla(0., 0., 1.0, 1.0),
            sidebar_bg: hsla(220. / 360., 0.14, 0.96, 1.0),
            titlebar_bg: hsla(0., 0., 1.0, 1.0),
            statusbar_bg: hsla(220. / 360., 0.14, 0.96, 1.0),

            text_primary: hsla(0., 0., 0.10, 1.0),
            text_secondary: hsla(0., 0., 0.44, 1.0),
            text_disabled: hsla(0., 0., 0.64, 1.0),
            text_on_accent: hsla(0., 0., 1.0, 1.0),

            accent: hsla(215. / 360., 0.92, 0.50, 1.0),
            accent_hover: hsla(215. / 360., 0.92, 0.42, 1.0),

            border: hsla(0., 0., 0.0, 0.10),
            border_light: hsla(0., 0., 0.0, 0.06),

            element_bg: hsla(0., 0., 0.0, 0.04),
            element_hover: hsla(0., 0., 0.0, 0.08),
            element_selected: hsla(215. / 360., 0.92, 0.50, 0.12),

            success: hsla(142. / 360., 0.71, 0.35, 1.0),
            warning: hsla(38. / 360., 0.92, 0.45, 1.0),
            error: hsla(0., 0.84, 0.50, 1.0),

            scrollbar: hsla(0., 0., 0.0, 0.15),
        }
    }
}
