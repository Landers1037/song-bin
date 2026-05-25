use anyhow::Result;
use url::Url;

use crate::proxy::node::{NodeSource, ProxyNode, ProxyProtocol};
use crate::proxy::protocol::trojan::TrojanConfig;
use crate::proxy::protocol::tuic::TuicConfig;
use crate::proxy::protocol::vless::{TlsConfig, VlessConfig};

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

fn parse_uri(uri: &str) -> Result<ProxyNode> {
    let scheme_end = uri
        .find("://")
        .ok_or_else(|| anyhow::anyhow!("Invalid URI: no scheme"))?;
    let scheme = &uri[..scheme_end];

    match scheme {
        "vless" => parse_vless(uri),
        "trojan" => parse_trojan(uri),
        "tuic" => parse_tuic(uri),
        _ => anyhow::bail!("Unsupported URI scheme: {}", scheme),
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
