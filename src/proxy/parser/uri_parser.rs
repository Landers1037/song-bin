use anyhow::{Context as _, Result};
use url::Url;

use crate::proxy::node::{NodeSource, ProxyNode, ProxyProtocol};
use crate::proxy::protocol::anytls::AnyTlsConfig;
use crate::proxy::protocol::trojan::TrojanConfig;
use crate::proxy::protocol::tuic::TuicConfig;
use crate::proxy::protocol::vless::{TlsConfig, VlessConfig};
use crate::proxy::protocol::vmess::VMessConfig;

pub fn parse_lines(content: &str) -> Result<Vec<ProxyNode>> {
    let mut nodes = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Ok(node) = parse_uri(line) {
            nodes.push(node);
        }
    }

    Ok(nodes)
}

pub fn parse_uri(uri: &str) -> Result<ProxyNode> {
    let scheme_end = uri
        .find("://")
        .ok_or_else(|| anyhow::anyhow!("Invalid URI: no scheme"))?;
    let scheme = &uri[..scheme_end];

    match scheme {
        "vless" => parse_vless(uri),
        "vmess" => parse_vmess(uri),
        "trojan" => parse_trojan(uri),
        "tuic" => parse_tuic(uri),
        "anytls" => parse_anytls(uri),
        _ => anyhow::bail!("不支持的 URI 协议: {}", scheme),
    }
}

fn parse_vless(uri: &str) -> Result<ProxyNode> {
    let url = Url::parse(uri)?;
    let uuid = url.username().to_string();
    let server = url
        .host_str()
        .ok_or_else(|| anyhow::anyhow!("No host"))?
        .to_string();
    let port = url.port().unwrap_or(443);
    let name = url_fragment_name(&url);

    let params: std::collections::HashMap<String, String> =
        url.query_pairs().map(|(k, v)| (k.to_string(), v.to_string())).collect();

    let flow = params.get("flow").cloned().unwrap_or_default();
    let sni = params
        .get("sni")
        .or_else(|| params.get("peer"))
        .cloned()
        .unwrap_or_default();
    let security = params.get("security").cloned().unwrap_or_default();

    let tls = TlsConfig {
        enabled: security != "none",
        server_name: sni,
        insecure: params
            .get("allowInsecure")
            .map(|v| v == "1")
            .unwrap_or(false),
        alpn: params
            .get("alpn")
            .map(|v| v.split(',').map(String::from).collect())
            .unwrap_or_default(),
        reality: None,
    };

    Ok(ProxyNode {
        id: uuid::Uuid::new_v4(),
        name,
        server,
        port,
        protocol: ProxyProtocol::Vless(VlessConfig {
            uuid,
            flow,
            tls,
            transport: None,
        }),
        enabled: true,
        latency: None,
        source: NodeSource::Manual,
    })
}

fn parse_vmess(uri: &str) -> Result<ProxyNode> {
    let encoded = uri
        .strip_prefix("vmess://")
        .ok_or_else(|| anyhow::anyhow!("无效的 vmess URI"))?;
    let decoded = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        encoded.trim(),
    )
    .or_else(|_| {
        base64::Engine::decode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            encoded.trim(),
        )
    })
    .map_err(|_| anyhow::anyhow!("vmess URI base64 解码失败"))?;
    let json_str = String::from_utf8(decoded).context("vmess URI 不是有效 UTF-8")?;
    let v: serde_json::Value = serde_json::from_str(&json_str).context("vmess URI JSON 解析失败")?;

    let server = v
        .get("add")
        .or_else(|| v.get("host"))
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string();
    let port = v
        .get("port")
        .and_then(|x| x.as_u64().or_else(|| x.as_str().and_then(|s| s.parse().ok())))
        .unwrap_or(443) as u16;
    let uuid = v
        .get("id")
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string();
    let name = v
        .get("ps")
        .and_then(|x| x.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .unwrap_or_else(|| format!("{}:{}", server, port));
    let alter_id = v
        .get("aid")
        .and_then(|x| x.as_u64().or_else(|| x.as_str().and_then(|s| s.parse().ok())))
        .unwrap_or(0) as u16;
    let security = v
        .get("scy")
        .or_else(|| v.get("security"))
        .and_then(|x| x.as_str())
        .unwrap_or("auto")
        .to_string();
    let sni = v
        .get("sni")
        .or_else(|| v.get("host"))
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string();
    let tls_enabled = v
        .get("tls")
        .and_then(|x| x.as_str())
        .map(|s| s == "tls")
        .unwrap_or(false);

    let tls = TlsConfig {
        enabled: tls_enabled,
        server_name: sni,
        insecure: false,
        alpn: Vec::new(),
        reality: None,
    };

    Ok(ProxyNode {
        id: uuid::Uuid::new_v4(),
        name,
        server,
        port,
        protocol: ProxyProtocol::Vmess(VMessConfig {
            uuid,
            alter_id,
            security,
            tls,
            transport: None,
        }),
        enabled: true,
        latency: None,
        source: NodeSource::Manual,
    })
}

fn parse_anytls(uri: &str) -> Result<ProxyNode> {
    let url = Url::parse(uri)?;
    let password = url.username().to_string();
    let server = url
        .host_str()
        .ok_or_else(|| anyhow::anyhow!("No host"))?
        .to_string();
    let port = url.port().unwrap_or(443);
    let name = url_fragment_name(&url);

    let params: std::collections::HashMap<String, String> =
        url.query_pairs().map(|(k, v)| (k.to_string(), v.to_string())).collect();

    let sni = params.get("sni").cloned().unwrap_or_default();

    let tls = TlsConfig {
        enabled: true,
        server_name: sni,
        insecure: params
            .get("allowInsecure")
            .map(|v| v == "1")
            .unwrap_or(false),
        alpn: params
            .get("alpn")
            .map(|v| v.split(',').map(String::from).collect())
            .unwrap_or_default(),
        reality: None,
    };

    Ok(ProxyNode {
        id: uuid::Uuid::new_v4(),
        name,
        server,
        port,
        protocol: ProxyProtocol::AnyTls(AnyTlsConfig {
            password,
            tls,
            ..Default::default()
        }),
        enabled: true,
        latency: None,
        source: NodeSource::Manual,
    })
}

fn parse_trojan(uri: &str) -> Result<ProxyNode> {
    let url = Url::parse(uri)?;
    let password = url.username().to_string();
    let server = url
        .host_str()
        .ok_or_else(|| anyhow::anyhow!("No host"))?
        .to_string();
    let port = url.port().unwrap_or(443);
    let name = url_fragment_name(&url);

    let params: std::collections::HashMap<String, String> =
        url.query_pairs().map(|(k, v)| (k.to_string(), v.to_string())).collect();

    let sni = params.get("sni").cloned().unwrap_or_default();

    let tls = TlsConfig {
        enabled: true,
        server_name: sni,
        insecure: params
            .get("allowInsecure")
            .map(|v| v == "1")
            .unwrap_or(false),
        alpn: params
            .get("alpn")
            .map(|v| v.split(',').map(String::from).collect())
            .unwrap_or_default(),
        reality: None,
    };

    Ok(ProxyNode {
        id: uuid::Uuid::new_v4(),
        name,
        server,
        port,
        protocol: ProxyProtocol::Trojan(TrojanConfig {
            password,
            tls,
            transport: None,
        }),
        enabled: true,
        latency: None,
        source: NodeSource::Manual,
    })
}

fn parse_tuic(uri: &str) -> Result<ProxyNode> {
    let url = Url::parse(uri)?;
    let uuid = url.username().to_string();
    let password = url
        .password()
        .unwrap_or("")
        .to_string();
    let server = url
        .host_str()
        .ok_or_else(|| anyhow::anyhow!("No host"))?
        .to_string();
    let port = url.port().unwrap_or(443);
    let name = url_fragment_name(&url);

    let params: std::collections::HashMap<String, String> =
        url.query_pairs().map(|(k, v)| (k.to_string(), v.to_string())).collect();

    let sni = params.get("sni").cloned().unwrap_or_default();
    let cc = params
        .get("congestion_control")
        .cloned()
        .unwrap_or_else(|| "bbr".into());

    let tls = TlsConfig {
        enabled: true,
        server_name: sni,
        insecure: false,
        alpn: params
            .get("alpn")
            .map(|v| v.split(',').map(String::from).collect())
            .unwrap_or_default(),
        reality: None,
    };

    Ok(ProxyNode {
        id: uuid::Uuid::new_v4(),
        name,
        server,
        port,
        protocol: ProxyProtocol::Tuic(TuicConfig {
            uuid,
            password,
            congestion_control: cc,
            tls,
            ..Default::default()
        }),
        enabled: true,
        latency: None,
        source: NodeSource::Manual,
    })
}

fn url_fragment_name(url: &Url) -> String {
    url.fragment()
        .map(|f| urlencoding::decode(f).unwrap_or_else(|_| f.into()).to_string())
        .unwrap_or_else(|| {
            format!(
                "{}:{}",
                url.host_str().unwrap_or("unknown"),
                url.port().unwrap_or(0)
            )
        })
}
