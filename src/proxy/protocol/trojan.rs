use serde::{Deserialize, Serialize};

use super::vless::{TlsConfig, TransportConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrojanConfig {
    pub password: String,
    #[serde(default)]
    pub tls: TlsConfig,
    #[serde(default)]
    pub transport: Option<TransportConfig>,
}

impl Default for TrojanConfig {
    fn default() -> Self {
        Self {
            password: String::new(),
            tls: TlsConfig::default(),
            transport: None,
        }
    }
}
