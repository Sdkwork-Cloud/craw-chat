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

pub(crate) fn build_gateway_upstream_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(resolve_upstream_timeout_seconds()))
        .build()
        .expect("gateway upstream HTTP client should build")
}
