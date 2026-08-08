pub mod tun_config;
pub use tun_config::default_tun_inbound;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap};
use serde_with::skip_serializing_none;

#[allow(unused)]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct XrayConfig {
    pub env: Option<Env>,
    pub log: Option<Log>,
    pub api: Option<Api>,
    pub dns: Option<Dns>,
    pub routing: Option<Routing>,
    pub policy: Option<Policy>,
    pub inbounds: Option<Vec<Inbounds>>,
    pub outbounds: Option<Vec<Outbounds>>,
    pub stats: Option<Stats>,
    pub fakedns: Option<Fakedns>,
    pub metrics: Option<Metrics>,
    pub observatory: Option<Observatory>,
    #[serde(rename = "burstObservatory")]
    pub burst_observatory: Option<BurstObservatory>,
    pub geodata: Option<Geodata>,
    pub version: Option<Version>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Env {
    #[serde(rename = "XRAY_LOCATION_ASSET")]
    pub xray_location_asset: Option<String>,
    #[serde(rename = "XRAY_BUF_READV")]
    pub xray_buf_readv: Option<String>,
    #[serde(rename = "XRAY_RAY_BUFFER_SIZE")]
    pub xray_ray_buffer_size: Option<String>,
    #[serde(rename = "GODEBUG")]
    pub godebug: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub access: Option<String>,     
    pub error: Option<String>,       
    pub loglevel: Option<String>,
    #[serde(rename="dnsLog")]
    pub dns_log: Option<bool>,
    pub mask_address: Option<String>
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Api {
    pub tag: Option<String>,      
    pub listen: Option<String>,          
    pub services: Option<Vec<String>>,     
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Dns {
    pub servers: Option<Vec<DnsServer>>,
    pub hosts: Option<HashMap<String, String>>,   
    #[serde(rename="clientIp")]
    pub client_ip: Option<String>,
    #[serde(rename="queryStrategy")]                
    pub query_strategy: Option<String>,
    #[serde(rename="disableCache")]           
    pub disable_cache: Option<bool>,
    #[serde(rename="disableFallback")]
    pub disable_fallback: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DnsServer {
    Address(String),
    Object(DnsServerObject)
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerObject {
    pub address: Option<String>,
    pub port: Option<u16>,
    pub domains: Option<Vec<String>>,
    #[serde(rename="expectedIPs")]
    pub expected_ips: Option<Vec<String>>,
    #[serde(rename="unexpectedIPs")]
    pub unexpected_ips: Option<Vec<String>>,
    #[serde(rename="skipFallback")]
    pub skip_fallback: Option<bool>,
    #[serde(rename="finalQuery")]
    pub final_query: Option<bool>,
    pub tag: Option<String>,
    #[serde(rename="clientIP")]
    pub client_ip: Option<String>,
    #[serde(rename="queryStrategy")]
    pub query_strategy: Option<String>,
    #[serde(rename="disableCache")]
    pub disable_cache: Option<bool>
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Routing {
    #[serde(rename="domainStrategy")]
    pub domain_strategy: Option<String>,   
    pub rules: Option<Vec<serde_json::Value>>,
    pub balancers: Option<Vec<serde_json::Value>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Policy {
    pub levels: Option<HashMap<String, PolicyLevel>>,
    pub system: Option<SystemPolicy>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyLevel {
    pub handshake: Option<u32>,
    #[serde(rename="connIdle")]
    pub conn_idle: Option<u32>,
    #[serde(rename="uplinkOnly")]
    pub uplink_only: Option<u32>,
    #[serde(rename="downlinkOnly")]
    pub downlink_only: Option<u32>,
    #[serde(rename="statsUserUplink")]
    pub stats_user_uplink: Option<bool>,
    #[serde(rename="statsUserDownlink")]
    pub stats_user_downlink: Option<bool>,
    #[serde(rename="bufferSize")]
    pub buffer_size: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemPolicy {
    #[serde(rename="statsInboundUplink")]
    pub stats_inbound_uplink: Option<bool>,
    #[serde(rename="statsInboundDownlink")]
    pub stats_inbound_downlink: Option<bool>,
    #[serde(rename="statsOutboundUplink")]
    pub stats_outbound_uplink: Option<bool>,
    #[serde(rename="statsOutboundDownlink")]
    pub stats_outbound_downlink: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Inbounds {
    pub port: Option<u16>,
    pub listen: Option<String>,           
    pub protocol: Option<String>,         
    pub settings: Option<InboundsSettings>, 
    #[serde(rename="streamSettings")]
    pub stream_settings: Option<serde_json::Value>, 
    pub sniffing: Option<Sniffing>,
    pub tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InboundsSettings {
    Tun(TunSettings)
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TunSettings {
    name: Option<String>,
    desc: Option<String>,
    mtu: Option<u16>,
    gateway: Option<Vec<String>>,
    dns: Option<Vec<String>>,
    #[serde(rename="userLevel")]
    user_level: Option<i32>,
    #[serde(rename="autoSystemRoutingTable")]
    auto_system_routing_table: Option<Vec<String>>,
    #[serde(rename="autoOutboundsInterface")]
    auto_outbounds_inteface: Option<String>
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Sniffing {
    pub enabled: Option<bool>,
    #[serde(rename="destOverride")]
    pub dest_override: Option<Vec<String>>,
    #[serde(rename="routeOnly")]
    pub route_only: Option<bool>,
    #[serde(rename="metadataOnly")]
    pub metadata_only: Option<bool>,
    #[serde(rename="ipsExcluded")]
    pub ips_excluded: Option<Vec<String>>,
    #[serde(rename="domainsExcluded")]
    pub domains_excluded: Option<Vec<String>>
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Outbounds {
    #[serde(rename="sendThrough")]
    pub send_through: Option<String>,
    pub protocol: Option<String>,
    pub settings: Option<serde_json::Value>,
    #[serde(rename="streamSettings")]
    pub stream_settings: Option<serde_json::Value>,
    pub tag: Option<String>,
    #[serde(rename="proxySettings")]
    pub proxy_settings: Option<ProxySettings>,
    pub mux: Option<Mux>,
    #[serde(rename="targetStrategy")]
    pub target_strategy: Option<String>
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ProxySettings {
    pub tag: Option<String>,
    #[serde(rename="transportLayer")]
    pub transport_layer: Option<bool>
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Mux {
    pub enabled: Option<bool>,
    pub concurrency: Option<u8>,
    #[serde(rename="xudpConcurrency")]
    pub xudp_concurrency: Option<u16>,
    #[serde(rename="xudpProxyUDP443")]
    pub xudp_proxy_udp443: Option<String>
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Fakedns {
    Vector(Vec<FakednsObject>),
    Object(FakednsObject)
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct FakednsObject {
    #[serde(rename="poolSize")]
    pub pool_size: Option<u32>,
    #[serde(rename="ipPool")]
    pub ip_pool: Option<String>,      
}


#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    pub tag: Option<String>,
    pub listen: Option<String>
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Observatory {
    #[serde(rename="subjectSelector")]
    pub subject_selector: Option<Vec<String>>,
    #[serde(rename="probeInterval")]
    pub probe_interval: Option<String>,
    #[serde(rename="probeUrl")]
    pub probe_url: Option<String>,
    #[serde(rename="enableConcurrency")]
    pub enable_concurrency: Option<bool>
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct BurstObservatory {
    #[serde(rename="subjectSelector")]
    pub subject_selector: Option<Vec<String>>,
    #[serde(rename="pingConfig")]
    pub ping_config: Option<PingConfig>
}


#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct PingConfig {
    pub destination: Option<String>,
    pub connectivity: Option<String>,
    pub interval: Option<String>,
    pub sampling: Option<u32>,
    pub timeout: Option<String>,
    #[serde(rename="httpMethod")]
    pub http_method: Option<String>

}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Geodata {
    pub cron: Option<String>,
    pub outbound: Option<String>,
    pub assets: Option<AssetObject>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetObject {
    pub url: Option<String>,
    pub file: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub min: Option<String>,
    pub max: Option<String>
}