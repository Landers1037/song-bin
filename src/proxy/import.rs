use anyhow::{Context as _, Result};
use chrono::Utc;
use url::Url;

use crate::proxy::node::ProxyNode;
use crate::proxy::parser::{self, uri_parser};
use crate::proxy::subscription::Subscription;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportKind {
    Node,
    Subscription,
}

#[derive(Debug)]
pub enum ImportResult {
    Nodes(Vec<ProxyNode>),
    Subscription(Subscription),
}

pub fn import_from_url(url: &str, kind: ImportKind) -> Result<ImportResult> {
    let url = url.trim();
    if url.is_empty() {
        anyhow::bail!("URL 不能为空");
    }

    if is_share_uri(url) {
        return import_share_uri(url, kind);
    }

    if url.starts_with("http://") || url.starts_with("https://") {
        let content = fetch_url(url)?;
        return import_fetched_content(url, &content, kind);
    }

    anyhow::bail!("不支持的 URL 格式，请输入 http(s) 链接或节点分享链接")
}

pub fn apply_import(state: &mut crate::state::app_state::AppState, result: ImportResult) {
    match result {
        ImportResult::Nodes(nodes) => {
            for node in nodes {
                state.add_node(node);
            }
        }
        ImportResult::Subscription(sub) => {
            state.add_subscription(sub);
        }
    }
    state.save_all();
}

fn is_share_uri(s: &str) -> bool {
    const SCHEMES: &[&str] = &["vless://", "vmess://", "trojan://", "tuic://", "anytls://"];
    SCHEMES.iter().any(|scheme| s.starts_with(scheme))
}

fn import_share_uri(uri: &str, kind: ImportKind) -> Result<ImportResult> {
    let node = uri_parser::parse_uri(uri)?;
    match kind {
        ImportKind::Node => Ok(ImportResult::Nodes(vec![node])),
        ImportKind::Subscription => anyhow::bail!(
            "该链接为单个节点分享链接，请使用「从 URL 中导入节点」"
        ),
    }
}

fn import_fetched_content(url: &str, content: &str, kind: ImportKind) -> Result<ImportResult> {
    let trimmed = content.trim();

    if is_share_uri(trimmed) {
        return import_share_uri(trimmed, kind);
    }

    if let Ok(node) = uri_parser::parse_uri(trimmed) {
        return match kind {
            ImportKind::Node => Ok(ImportResult::Nodes(vec![node])),
            ImportKind::Subscription => anyhow::bail!(
                "该链接内容为单个节点，请使用「从 URL 中导入节点」"
            ),
        };
    }

    let nodes = parser::parse_subscription_content(content)?;

    match kind {
        ImportKind::Node => Ok(ImportResult::Nodes(nodes)),
        ImportKind::Subscription => {
            let name = subscription_name_from_url(url);
            let mut sub = Subscription::new(name, url.to_string());
            sub.nodes = nodes;
            sub.last_updated = Some(Utc::now());
            Ok(ImportResult::Subscription(sub))
        }
    }
}

fn subscription_name_from_url(url: &str) -> String {
    Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(String::from))
        .unwrap_or_else(|| "订阅".to_string())
}

fn fetch_url(url: &str) -> Result<String> {
    let body = ureq::get(url)
        .header("User-Agent", "song-bin/0.1")
        .call()
        .with_context(|| format!("请求失败: {}", url))?
        .body_mut()
        .read_to_string()
        .context("读取响应内容失败")?;
    Ok(body)
}
