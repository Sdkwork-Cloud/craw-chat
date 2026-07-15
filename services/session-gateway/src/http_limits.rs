pub const REALTIME_MAX_PREAUTH_WEBSOCKET_CONNECTIONS_ENV: &str =
    "SDKWORK_IM_REALTIME_MAX_PREAUTH_WEBSOCKET_CONNECTIONS";
pub const REALTIME_MAX_PREAUTH_WEBSOCKET_CONNECTIONS_DEFAULT: usize = 512;
pub const REALTIME_MAX_PREAUTH_WEBSOCKET_CONNECTIONS_MAX: usize = 10_000;
pub const REALTIME_MAX_WEBSOCKET_CONNECTIONS_ENV: &str =
    "SDKWORK_IM_REALTIME_MAX_WEBSOCKET_CONNECTIONS";
pub const REALTIME_MAX_WEBSOCKET_CONNECTIONS_DEFAULT: usize = 10_000;
pub const REALTIME_MAX_WEBSOCKET_CONNECTIONS_MAX: usize = 100_000;
pub const SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS_ENV: &str =
    "SDKWORK_IM_SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS";
pub const SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 2_000;
pub const SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 50_000;
pub const REALTIME_NODE_ID_ENV: &str = "SDKWORK_IM_REALTIME_NODE_ID";
/// Opt-in compatibility for deprecated plain-JSON websocket mode without `sdkwork-im.ccp.ws.v1`.
pub const REALTIME_ACCEPT_LEGACY_WEBSOCKET_JSON_ENV: &str =
    "SDKWORK_IM_REALTIME_ACCEPT_LEGACY_WEBSOCKET_JSON";
pub const SESSION_GATEWAY_MAX_REQUEST_BODY_BYTES_ENV: &str =
    "SDKWORK_IM_SESSION_GATEWAY_MAX_REQUEST_BODY_BYTES";
pub const SESSION_GATEWAY_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
pub const SESSION_GATEWAY_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;
pub const SESSION_GATEWAY_WS_UPGRADE_RATE_RPM_ENV: &str =
    "SDKWORK_IM_SESSION_GATEWAY_WS_UPGRADE_RATE_RPM";
pub const SESSION_GATEWAY_WS_UPGRADE_RATE_BURST_ENV: &str =
    "SDKWORK_IM_SESSION_GATEWAY_WS_UPGRADE_RATE_BURST";
const SESSION_GATEWAY_WS_FRAME_RATE_RPM_ENV: &str = "SDKWORK_IM_SESSION_GATEWAY_WS_FRAME_RATE_RPM";
const SESSION_GATEWAY_WS_FRAME_RATE_BURST_ENV: &str =
    "SDKWORK_IM_SESSION_GATEWAY_WS_FRAME_RATE_BURST";
pub const SESSION_GATEWAY_WS_RATE_MAX_BUCKETS_ENV: &str =
    "SDKWORK_IM_SESSION_GATEWAY_WS_RATE_MAX_BUCKETS";
const SESSION_GATEWAY_WS_UPGRADE_RATE_RPM_DEFAULT: u32 = 120;
const SESSION_GATEWAY_WS_UPGRADE_RATE_BURST_DEFAULT: u32 = 20;
const SESSION_GATEWAY_WS_FRAME_RATE_RPM_DEFAULT: u32 = 600;
const SESSION_GATEWAY_WS_FRAME_RATE_BURST_DEFAULT: u32 = 60;
const SESSION_GATEWAY_WS_RATE_MAX_BUCKETS_DEFAULT: usize = 50_000;
const SESSION_GATEWAY_WS_RATE_MAX_BUCKETS_MAX: usize = 500_000;

pub fn resolve_realtime_node_id_from_env() -> String {
    std::env::var(REALTIME_NODE_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "session_gateway_local_1".to_owned())
}

pub fn realtime_accepts_legacy_websocket_json() -> bool {
    parse_env_truthy(std::env::var(REALTIME_ACCEPT_LEGACY_WEBSOCKET_JSON_ENV).ok())
}

fn parse_env_truthy(value: Option<String>) -> bool {
    value.is_some_and(|value| {
        matches!(
            value.trim(),
            "1" | "true" | "TRUE" | "True" | "yes" | "YES" | "Yes"
        )
    })
}

pub fn resolve_max_websocket_connections() -> usize {
    std::env::var(REALTIME_MAX_WEBSOCKET_CONNECTIONS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(REALTIME_MAX_WEBSOCKET_CONNECTIONS_DEFAULT)
        .min(REALTIME_MAX_WEBSOCKET_CONNECTIONS_MAX)
}

/// Capacity budget for websocket upgrades awaiting `auth.init` (does not consume authenticated slots).
pub fn resolve_max_preauth_websocket_connections(max_authenticated: usize) -> usize {
    std::env::var(REALTIME_MAX_PREAUTH_WEBSOCKET_CONNECTIONS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or_else(|| {
            REALTIME_MAX_PREAUTH_WEBSOCKET_CONNECTIONS_DEFAULT
                .min(max_authenticated.max(1) / 10)
                .max(64)
        })
        .min(REALTIME_MAX_PREAUTH_WEBSOCKET_CONNECTIONS_MAX)
        .min(max_authenticated)
}

pub fn resolve_max_in_flight_requests() -> usize {
    std::env::var(SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS_MAX)
}

pub fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(SESSION_GATEWAY_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(SESSION_GATEWAY_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(SESSION_GATEWAY_MAX_REQUEST_BODY_BYTES_MAX)
}

pub fn resolve_websocket_upgrade_rate_rpm() -> u32 {
    std::env::var(SESSION_GATEWAY_WS_UPGRADE_RATE_RPM_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(SESSION_GATEWAY_WS_UPGRADE_RATE_RPM_DEFAULT)
}

pub fn resolve_websocket_upgrade_rate_burst() -> u32 {
    std::env::var(SESSION_GATEWAY_WS_UPGRADE_RATE_BURST_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(SESSION_GATEWAY_WS_UPGRADE_RATE_BURST_DEFAULT)
}

pub fn resolve_websocket_frame_rate_rpm() -> u32 {
    std::env::var(SESSION_GATEWAY_WS_FRAME_RATE_RPM_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(SESSION_GATEWAY_WS_FRAME_RATE_RPM_DEFAULT)
}

pub fn resolve_websocket_frame_rate_burst() -> u32 {
    std::env::var(SESSION_GATEWAY_WS_FRAME_RATE_BURST_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(SESSION_GATEWAY_WS_FRAME_RATE_BURST_DEFAULT)
}

pub fn resolve_websocket_rate_max_buckets() -> usize {
    std::env::var(SESSION_GATEWAY_WS_RATE_MAX_BUCKETS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(SESSION_GATEWAY_WS_RATE_MAX_BUCKETS_DEFAULT)
        .min(SESSION_GATEWAY_WS_RATE_MAX_BUCKETS_MAX)
}
