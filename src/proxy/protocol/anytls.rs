use serde::{Deserialize, Serialize};

use super::vless::TlsConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnyTlsConfig {
    pub password: String,
    #[serde(default)]
    pub tls: TlsConfig,
    #[serde(default = "default_idle_timeout")]
    pub idle_timeout: String,
    #[serde(default = "default_min_padding")]
    pub min_padding_len: u32,
    #[serde(default = "default_max_padding")]
    pub max_padding_len: u32,
}

impl Default for AnyTlsConfig {
    fn default() -> Self {
        Self {
            password: String::new(),
            tls: TlsConfig::default(),
            idle_timeout: default_idle_timeout(),
            min_padding_len: default_min_padding(),
            max_padding_len: default_max_padding(),
        }
    }
}

fn default_idle_timeout() -> String {
    "15m".into()
}

fn default_min_padding() -> u32 {
    0
}

fn default_max_padding() -> u32 {
    0
}
