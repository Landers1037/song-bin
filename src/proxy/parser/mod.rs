pub mod clash_parser;
pub mod singbox_parser;
pub mod uri_parser;

use anyhow::Result;

use super::node::ProxyNode;

pub fn parse_subscription_content(content: &str) -> Result<Vec<ProxyNode>> {
    if let Ok(nodes) = singbox_parser::parse(content) {
        if !nodes.is_empty() {
            return Ok(nodes);
        }
    }

    if let Ok(nodes) = clash_parser::parse(content) {
        if !nodes.is_empty() {
            return Ok(nodes);
        }
    }

    // Try base64-decoded URI list
    let decoded = if let Ok(bytes) = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        content.trim(),
    ) {
        String::from_utf8_lossy(&bytes).to_string()
    } else if let Ok(bytes) = base64::Engine::decode(
        &base64::engine::general_purpose::URL_SAFE_NO_PAD,
        content.trim(),
    ) {
        String::from_utf8_lossy(&bytes).to_string()
    } else {
        content.to_string()
    };

    let nodes = uri_parser::parse_lines(&decoded)?;
    if !nodes.is_empty() {
        return Ok(nodes);
    }

    anyhow::bail!("Unable to parse subscription content: unrecognized format")
}
