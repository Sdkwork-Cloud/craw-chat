pub const REALTIME_MAX_WEBSOCKET_CONNECTIONS_ENV: &str =    "SDKWORK_IM_REALTIME_MAX_WEBSOCKET_CONNECTIONS";
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
