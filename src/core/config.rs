use serde_json::{json, Value};

use crate::proxy::node::{ProxyNode, ProxyProtocol};
use crate::ui::statusbar::ProxyMode;

pub fn generate_config(
    node: &ProxyNode,
    mode: ProxyMode,
    mixed_port: u16,
    clash_api_port: u16,
) -> Value {
    let outbound = node_to_outbound(node);

    let route = match mode {
        ProxyMode::Global => json!({
            "final": "proxy",
            "auto_detect_interface": true,
        }),
        ProxyMode::Direct => json!({
            "final": "direct",
            "auto_detect_interface": true,
        }),
        ProxyMode::Rule => json!({
            "final": "proxy",
            "auto_detect_interface": true,
            "rules": [
                {
                    "protocol": "dns",
                    "outbound": "dns-out"
                },
                {
                    "geoip": ["private"],
                    "outbound": "direct"
                },
                {
                    "geosite": ["cn"],
                    "geoip": ["cn"],
                    "outbound": "direct"
                }
            ]
        }),
    };

    json!({
        "log": {
            "level": "info",
            "timestamp": true
        },
        "dns": {
            "servers": [
                {
                    "tag": "google",
                    "address": "tls://8.8.8.8"
                },
                {
                    "tag": "local",
                    "address": "223.5.5.5",
                    "detour": "direct"
                }
            ],
            "rules": [
                {
                    "geosite": ["cn"],
                    "server": "local"
                }
            ]
        },
        "inbounds": [
            {
                "type": "mixed",
                "tag": "mixed-in",
                "listen": "127.0.0.1",
                "listen_port": mixed_port
            }
        ],
        "outbounds": [
            outbound,
            {
                "type": "direct",
                "tag": "direct"
            },
            {
                "type": "block",
                "tag": "block"
            },
            {
                "type": "dns",
                "tag": "dns-out"
            }
        ],
        "route": route,
        "experimental": {
            "clash_api": {
                "external_controller": format!("127.0.0.1:{}", clash_api_port),
                "store_selected": true
            }
        }
    })
}

fn node_to_outbound(node: &ProxyNode) -> Value {
    match &node.protocol {
        ProxyProtocol::Vless(cfg) => {
            let mut ob = json!({
                "type": "vless",
                "tag": "proxy",
                "server": node.server,
                "server_port": node.port,
                "uuid": cfg.uuid,
            });
            if !cfg.flow.is_empty() {
                ob["flow"] = json!(cfg.flow);
            }
            apply_tls(&mut ob, &cfg.tls);
            ob
        }
        ProxyProtocol::Trojan(cfg) => {
            let mut ob = json!({
                "type": "trojan",
                "tag": "proxy",
                "server": node.server,
                "server_port": node.port,
                "password": cfg.password,
            });
            apply_tls(&mut ob, &cfg.tls);
            ob
        }
        ProxyProtocol::AnyTls(cfg) => {
            let mut ob = json!({
                "type": "anytls",
                "tag": "proxy",
                "server": node.server,
                "server_port": node.port,
                "password": cfg.password,
                "idle_timeout": cfg.idle_timeout,
            });
            if cfg.min_padding_len > 0 || cfg.max_padding_len > 0 {
                ob["padding_scheme"] = json!(format!("{}:{}", cfg.min_padding_len, cfg.max_padding_len));
            }
            apply_tls(&mut ob, &cfg.tls);
            ob
        }
        ProxyProtocol::Tuic(cfg) => {
            let mut ob = json!({
                "type": "tuic",
                "tag": "proxy",
                "server": node.server,
                "server_port": node.port,
                "uuid": cfg.uuid,
                "password": cfg.password,
                "congestion_control": cfg.congestion_control,
                "udp_relay_mode": cfg.udp_relay_mode,
                "zero_rtt_handshake": cfg.zero_rtt_handshake,
            });
            if !cfg.heartbeat.is_empty() {
                ob["heartbeat"] = json!(cfg.heartbeat);
            }
            apply_tls(&mut ob, &cfg.tls);
            ob
        }
    }
}

fn apply_tls(ob: &mut Value, tls: &crate::proxy::protocol::vless::TlsConfig) {
    if !tls.enabled {
        return;
    }

    let mut tls_obj = json!({
        "enabled": true,
    });

    if !tls.server_name.is_empty() {
        tls_obj["server_name"] = json!(tls.server_name);
    }
    if tls.insecure {
        tls_obj["insecure"] = json!(true);
    }
    if !tls.alpn.is_empty() {
        tls_obj["alpn"] = json!(tls.alpn);
    }
    if let Some(reality) = &tls.reality {
        tls_obj["reality"] = json!({
            "enabled": true,
            "public_key": reality.public_key,
            "short_id": reality.short_id,
        });
    }

    ob["tls"] = tls_obj;
}
