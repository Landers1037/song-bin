use anyhow::Result;
use serde_json::Value;

use crate::proxy::node::{NodeSource, ProxyNode, ProxyProtocol};
use crate::proxy::protocol::anytls::AnyTlsConfig;
use crate::proxy::protocol::trojan::TrojanConfig;
use crate::proxy::protocol::tuic::TuicConfig;
use crate::proxy::protocol::vless::{TlsConfig, VlessConfig};
use crate::proxy::protocol::vmess::VMessConfig;

pub fn parse(content: &str) -> Result<Vec<ProxyNode>> {
    let config: Value = serde_json::from_str(content)?;
    let outbounds = config
        .get("outbounds")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow::anyhow!("No outbounds array found"))?;

    let mut nodes = Vec::new();

    for ob in outbounds {
        let ob_type = ob.get("type").and_then(|v| v.as_str()).unwrap_or("");
        let tag = ob
            .get("tag")
            .and_then(|v| v.as_str())
            .unwrap_or("unnamed")
            .to_string();
        let server = ob
            .get("server")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let port = ob
            .get("server_port")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u16;

        if server.is_empty() || port == 0 {
            continue;
        }

        let tls = parse_tls(ob.get("tls"));

        let protocol = match ob_type {
            "vless" => {
                let uuid = ob
                    .get("uuid")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let flow = ob
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
            "vmess" => {
                let uuid = ob
                    .get("uuid")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let alter_id = ob
                    .get("alter_id")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u16;
                let security = ob
                    .get("security")
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
            "trojan" => {
                let password = ob
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
            "anytls" => {
                let password = ob
                    .get("password")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                ProxyProtocol::AnyTls(AnyTlsConfig {
                    password,
                    tls,
                    ..Default::default()
                })
            }
            "tuic" => {
                let uuid = ob
                    .get("uuid")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let password = ob
                    .get("password")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let cc = ob
                    .get("congestion_control")
                    .and_then(|v| v.as_str())
                    .unwrap_or("bbr")
                    .to_string();
                ProxyProtocol::Tuic(TuicConfig {
                    uuid,
                    password,
                    congestion_control: cc,
                    tls,
                    ..Default::default()
                })
            }
            _ => continue,
        };

        nodes.push(ProxyNode {
            id: uuid::Uuid::new_v4(),
            name: tag,
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

fn parse_tls(tls_val: Option<&Value>) -> TlsConfig {
    let Some(tls) = tls_val else {
        return TlsConfig::default();
    };

    TlsConfig {
        enabled: tls
            .get("enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(true),
        server_name: tls
            .get("server_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        insecure: tls
            .get("insecure")
            .and_then(|v| v.as_bool())
            .unwrap_or(false),
        alpn: tls
            .get("alpn")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default(),
        reality: None,
    }
}
