use std::borrow::Cow;

use gpui::{App, SharedString};
use gpui_component::Theme;

pub const FONT_FAMILY: &str = "Noto Sans SC";

pub fn init(cx: &mut App) {
    let fonts = [include_bytes!("NotoSansSC-VariableFont_wght.ttf")]
        .iter()
        .map(|bytes| Cow::Borrowed(&bytes[..]))
        .collect();

    if let Err(err) = cx.text_system().add_fonts(fonts) {
        log::warn!("Failed to load Noto Sans SC font: {}", err);
    }

    apply(cx);
}

pub fn apply(cx: &mut App) {
    let family = SharedString::from(FONT_FAMILY);
    let theme = Theme::global_mut(cx);
    theme.font_family = family.clone();
    theme.mono_font_family = family;
}
