use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VlessConfig {
    pub uuid: String,
    #[serde(default = "default_flow")]
    pub flow: String,
    #[serde(default)]
    pub tls: TlsConfig,
    #[serde(default)]
    pub transport: Option<TransportConfig>,
}

impl Default for VlessConfig {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            flow: default_flow(),
            tls: TlsConfig::default(),
            transport: None,
        }
    }
}

fn default_flow() -> String {
    "xtls-rprx-vision".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub server_name: String,
    #[serde(default)]
    pub insecure: bool,
    #[serde(default)]
    pub alpn: Vec<String>,
    #[serde(default)]
    pub reality: Option<RealityConfig>,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            server_name: String::new(),
            insecure: false,
            alpn: vec!["h2".into(), "http/1.1".into()],
            reality: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityConfig {
    pub public_key: String,
    #[serde(default)]
    pub short_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TransportConfig {
    #[serde(rename = "ws")]
    WebSocket {
        #[serde(default)]
        path: String,
        #[serde(default)]
        headers: std::collections::HashMap<String, String>,
    },
    #[serde(rename = "grpc")]
    Grpc {
        #[serde(default)]
        service_name: String,
    },
    #[serde(rename = "http")]
    Http {
        #[serde(default)]
        host: Vec<String>,
        #[serde(default)]
        path: String,
    },
}

fn default_true() -> bool {
    true
}
