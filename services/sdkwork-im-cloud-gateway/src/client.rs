//! Gateway upstream HTTP client construction and env-driven body/timeout limits.

use std::time::Duration;

use reqwest::Client;

use crate::constants::{
    GATEWAY_MAX_REQUEST_BODY_BYTES_DEFAULT, GATEWAY_MAX_REQUEST_BODY_BYTES_ENV,
    GATEWAY_MAX_REQUEST_BODY_BYTES_MAX, GATEWAY_MAX_UPSTREAM_RESPONSE_BODY_BYTES_DEFAULT,
    GATEWAY_MAX_UPSTREAM_RESPONSE_BODY_BYTES_ENV, GATEWAY_MAX_UPSTREAM_RESPONSE_BODY_BYTES_MAX,
    GATEWAY_UPSTREAM_TIMEOUT_SECONDS_DEFAULT, GATEWAY_UPSTREAM_TIMEOUT_SECONDS_ENV,
    GATEWAY_UPSTREAM_TIMEOUT_SECONDS_MAX,
};

// Connection pool configuration constants (P1-10 fix)
const GATEWAY_POOL_MAX_IDLE_PER_HOST_DEFAULT: usize = 50;
const GATEWAY_POOL_MAX_IDLE_PER_HOST_ENV: &str = "SDKWORK_IM_GATEWAY_POOL_MAX_IDLE_PER_HOST";
const GATEWAY_POOL_IDLE_TIMEOUT_SECS_DEFAULT: u64 = 90;
const GATEWAY_POOL_IDLE_TIMEOUT_SECS_ENV: &str = "SDKWORK_IM_GATEWAY_POOL_IDLE_TIMEOUT_SECS";

pub(crate) fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(GATEWAY_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(GATEWAY_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(GATEWAY_MAX_REQUEST_BODY_BYTES_MAX)
}

fn resolve_upstream_timeout_seconds() -> u64 {
    std::env::var(GATEWAY_UPSTREAM_TIMEOUT_SECONDS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(GATEWAY_UPSTREAM_TIMEOUT_SECONDS_DEFAULT)
        .min(GATEWAY_UPSTREAM_TIMEOUT_SECONDS_MAX)
}

pub(crate) fn resolve_max_upstream_response_body_bytes() -> usize {
    std::env::var(GATEWAY_MAX_UPSTREAM_RESPONSE_BODY_BYTES_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(GATEWAY_MAX_UPSTREAM_RESPONSE_BODY_BYTES_DEFAULT)
        .min(GATEWAY_MAX_UPSTREAM_RESPONSE_BODY_BYTES_MAX)
}

fn resolve_pool_max_idle_per_host() -> usize {
    std::env::var(GATEWAY_POOL_MAX_IDLE_PER_HOST_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(GATEWAY_POOL_MAX_IDLE_PER_HOST_DEFAULT)
}

fn resolve_pool_idle_timeout_secs() -> u64 {
    std::env::var(GATEWAY_POOL_IDLE_TIMEOUT_SECS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(GATEWAY_POOL_IDLE_TIMEOUT_SECS_DEFAULT)
}

/// Build the gateway upstream HTTP client with connection pool configuration.
///
/// Connection pool settings are critical for high-throughput gateway scenarios:
/// - `pool_max_idle_per_host`: Maximum idle connections kept alive per upstream host
/// - `pool_idle_timeout`: How long to keep idle connections before closing
/// - `tcp_nodelay`: Disable Nagle's algorithm for lower latency
///
/// These settings prevent connection pool exhaustion under high load (P1-10 fix).
pub(crate) fn build_gateway_upstream_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(resolve_upstream_timeout_seconds()))
        // Connection pool configuration for high throughput
        .pool_max_idle_per_host(resolve_pool_max_idle_per_host())
        .pool_idle_timeout(Some(Duration::from_secs(resolve_pool_idle_timeout_secs())))
        // Enable TCP_NODELAY for lower latency on small packets
        .tcp_nodelay(true)
        .build()
        .expect("gateway upstream HTTP client should build")
}
