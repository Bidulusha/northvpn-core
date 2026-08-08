use serde::{Deserialize, Serialize};

use crate::xray_configs::{Inbounds, InboundsSettings, TunSettings};

pub fn default_tun_inbound() -> Inbounds {
    Inbounds {
        tag: Some("tun-in".to_string()),
        port: None,
        listen: None,
        protocol: Some("tun".to_string()),
        settings: Some(
            InboundsSettings::Tun(
                TunSettings {
                    name:                       Some("utun10".to_string()),
                    desc:                       Some("Wintun".to_string()),
                    mtu:                        Some(1500),
                    gateway:                    Some(vec!["10.0.0.1/16".to_string(), "fc00::1/64".to_string()]),
                    dns:                        Some(vec!["1.1.1.1".to_string(), "8.8.8.8".to_string()]),
                    user_level:                 Some(0),
                    auto_system_routing_table:  Some(vec!["0.0.0.0/0".to_string(), "::/0".to_string()]),
                    auto_outbounds_inteface:    Some("auto".to_string())
            })),
        sniffing: None,
        stream_settings: None,
    }
}