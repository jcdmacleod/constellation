// Constellation
//
// Pluggable authoritative DNS server
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::net::SocketAddr;
use std::collections::BTreeMap;

use super::config::ConfigDNSZone;

pub fn server_log_level() -> String {
    "warn".to_string()
}

pub fn dns_inets() -> Vec<SocketAddr> {
    vec!["0.0.0.0:53".parse().unwrap(), "[::]:53".parse().unwrap()]
}

pub fn dns_tcp_timeout() -> u64 {
    2
}

pub fn dns_soa_refresh() -> i32 {
    10000
}

pub fn dns_soa_retry() -> i32 {
    2400
}

pub fn dns_soa_expire() -> i32 {
    604800
}

pub fn dns_soa_ttl() -> u32 {
    3600
}

pub fn dns_record_ttl() -> u32 {
    3600
}

pub fn dns_zone() -> BTreeMap<String, ConfigDNSZone> {
    BTreeMap::new()
}

pub fn http_inet() -> SocketAddr {
    "[::1]:8080".parse().unwrap()
}

pub fn http_workers() -> u16 {
    2
}

pub fn redis_host() -> String {
    "localhost".to_string()
}

pub fn redis_port() -> u16 {
    6379
}

pub fn redis_database() -> u8 {
    0
}

pub fn redis_pool_size() -> u32 {
    8
}

pub fn redis_max_lifetime_seconds() -> u64 {
    20
}

pub fn redis_idle_timeout_seconds() -> u64 {
    600
}

pub fn redis_connection_timeout_seconds() -> u64 {
    5
}
