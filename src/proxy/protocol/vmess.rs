use serde::{Deserialize, Serialize};

use super::vless::{TlsConfig, TransportConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMessConfig {
    pub uuid: String,
    #[serde(default)]
    pub alter_id: u16,
    #[serde(default = "default_security")]
    pub security: String,
    #[serde(default)]
    pub tls: TlsConfig,
    #[serde(default)]
    pub transport: Option<TransportConfig>,
}

impl Default for VMessConfig {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            alter_id: 0,
            security: default_security(),
            tls: TlsConfig::default(),
            transport: None,
        }
    }
}

fn default_security() -> String {
    "auto".into()
}
