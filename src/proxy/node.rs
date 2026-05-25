use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::protocol::{anytls::AnyTlsConfig, trojan::TrojanConfig, tuic::TuicConfig, vless::VlessConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ProxyProtocol {
    Vless(VlessConfig),
    Trojan(TrojanConfig),
    #[serde(rename = "anytls")]
    AnyTls(AnyTlsConfig),
    Tuic(TuicConfig),
}

impl ProxyProtocol {
    pub fn label(&self) -> &'static str {
        match self {
            ProxyProtocol::Vless(_) => "VLESS",
            ProxyProtocol::Trojan(_) => "TROJAN",
            ProxyProtocol::AnyTls(_) => "AnyTLS",
            ProxyProtocol::Tuic(_) => "TUIC",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeSource {
    Manual,
    Subscription(Uuid),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyNode {
    pub id: Uuid,
    pub name: String,
    pub server: String,
    pub port: u16,
    pub protocol: ProxyProtocol,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub latency: Option<u32>,
    #[serde(default = "default_manual")]
    pub source: NodeSource,
}

fn default_manual() -> NodeSource {
    NodeSource::Manual
}

impl ProxyNode {
    pub fn new(name: String, server: String, port: u16, protocol: ProxyProtocol) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            server,
            port,
            protocol,
            enabled: true,
            latency: None,
            source: NodeSource::Manual,
        }
    }

    pub fn protocol_label(&self) -> &'static str {
        self.protocol.label()
    }

    pub fn display_address(&self) -> String {
        format!("{}:{}", self.server, self.port)
    }
}
