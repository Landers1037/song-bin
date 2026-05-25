use serde::{Deserialize, Serialize};

use super::vless::TlsConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuicConfig {
    pub uuid: String,
    pub password: String,
    #[serde(default = "default_congestion")]
    pub congestion_control: String,
    #[serde(default = "default_udp_relay_mode")]
    pub udp_relay_mode: String,
    #[serde(default)]
    pub tls: TlsConfig,
    #[serde(default)]
    pub zero_rtt_handshake: bool,
    #[serde(default)]
    pub heartbeat: String,
}

impl Default for TuicConfig {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            password: String::new(),
            congestion_control: default_congestion(),
            udp_relay_mode: default_udp_relay_mode(),
            tls: TlsConfig::default(),
            zero_rtt_handshake: false,
            heartbeat: "10s".into(),
        }
    }
}

fn default_congestion() -> String {
    "bbr".into()
}

fn default_udp_relay_mode() -> String {
    "native".into()
}
