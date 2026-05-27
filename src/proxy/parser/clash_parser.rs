use anyhow::Result;
use serde_yaml::Value;

use crate::proxy::node::{NodeSource, ProxyNode, ProxyProtocol};
use crate::proxy::protocol::trojan::TrojanConfig;
use crate::proxy::protocol::vless::{TlsConfig, VlessConfig};
use crate::proxy::protocol::vmess::VMessConfig;

pub fn parse(content: &str) -> Result<Vec<ProxyNode>> {
    let yaml: Value = serde_yaml::from_str(content)?;
    let proxies = yaml
        .get("proxies")
        .and_then(|v| v.as_sequence())
        .ok_or_else(|| anyhow::anyhow!("No proxies array found in YAML"))?;

    let mut nodes = Vec::new();

    for proxy in proxies {
        let p_type = proxy.get("type").and_then(|v| v.as_str()).unwrap_or("");
        let name = proxy
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("unnamed")
            .to_string();
        let server = proxy
            .get("server")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let port = proxy
            .get("port")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u16;

        if server.is_empty() || port == 0 {
            continue;
        }

        let sni = proxy
            .get("sni")
            .or_else(|| proxy.get("servername"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let skip_cert = proxy
            .get("skip-cert-verify")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let alpn = proxy
            .get("alpn")
            .and_then(|v| v.as_sequence())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let tls = TlsConfig {
            enabled: true,
            server_name: sni,
            insecure: skip_cert,
            alpn,
            reality: None,
        };

        let protocol = match p_type {
            "vless" => {
                let uuid = proxy
                    .get("uuid")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let flow = proxy
                    .get("flow")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                ProxyProtocol::Vless(VlessConfig {
                    uuid,
                    flow,
                    tls,
                    transport: None,
                })
            }
            "trojan" => {
                let password = proxy
                    .get("password")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                ProxyProtocol::Trojan(TrojanConfig {
                    password,
                    tls,
                    transport: None,
                })
            }
            "vmess" => {
                let uuid = proxy
                    .get("uuid")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let alter_id = proxy
                    .get("alterId")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u16;
                let security = proxy
                    .get("cipher")
                    .and_then(|v| v.as_str())
                    .unwrap_or("auto")
                    .to_string();
                ProxyProtocol::Vmess(VMessConfig {
                    uuid,
                    alter_id,
                    security,
                    tls,
                    transport: None,
                })
            }
            _ => continue,
        };

        nodes.push(ProxyNode {
            id: uuid::Uuid::new_v4(),
            name,
            server,
            port,
            protocol,
            enabled: true,
            latency: None,
            source: NodeSource::Manual,
        });
    }

    Ok(nodes)
}
