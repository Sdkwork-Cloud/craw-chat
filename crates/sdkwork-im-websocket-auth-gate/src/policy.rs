use axum::http::HeaderMap;
use im_app_context::has_websocket_upgrade_auth_headers;
use sdkwork_im_realtime_api_paths::REALTIME_WS;

pub const SENSITIVE_WEBSOCKET_QUERY_KEYS: &[&str] = &[
    "authToken",
    "auth_token",
    "accessToken",
    "access_token",
    "token",
    "authorization",
    "refreshToken",
    "refresh_token",
];

/// Browser-like websocket upgrades require the post-upgrade `auth.init` frame.
pub fn should_require_auth_init_frame(
    headers: &HeaderMap,
    query_token_headers_available: bool,
) -> bool {
    if has_websocket_upgrade_auth_headers(headers) {
        return false;
    }
    !query_token_headers_available
}

pub fn is_sensitive_websocket_query_key(key: &str) -> bool {
    SENSITIVE_WEBSOCKET_QUERY_KEYS
        .iter()
        .any(|candidate| key.eq_ignore_ascii_case(candidate))
}

pub fn sanitized_realtime_websocket_path_and_query(path: &str, query: Option<&str>) -> String {
    let Some(query) = query else {
        return path.to_owned();
    };
    if path == REALTIME_WS {
        let safe_query = query
            .split('&')
            .filter(|pair| {
                let key = pair.split_once('=').map_or(*pair, |(key, _)| key).trim();
                key.eq_ignore_ascii_case("deviceId")
            })
            .collect::<Vec<_>>();
        if safe_query.is_empty() {
            return path.to_owned();
        }
        return format!("{path}?{}", safe_query.join("&"));
    }
    let safe_query = query
        .split('&')
        .filter(|pair| {
            let key = pair.split_once('=').map_or(*pair, |(key, _)| key).trim();
            !is_sensitive_websocket_query_key(key)
        })
        .collect::<Vec<_>>();
    if safe_query.is_empty() {
        return path.to_owned();
    }
    format!("{path}?{}", safe_query.join("&"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn realtime_query_sanitization_keeps_only_device_id() {
        assert_eq!(
            sanitized_realtime_websocket_path_and_query(
                REALTIME_WS,
                Some("deviceId=d1&accessToken=secret"),
            ),
            format!("{REALTIME_WS}?deviceId=d1")
        );
    }

    #[test]
    fn non_realtime_query_sanitization_strips_sensitive_keys() {
        assert_eq!(
            sanitized_realtime_websocket_path_and_query(
                "/im/v3/api/custom/ws",
                Some("deviceId=d1&authToken=secret&scope=chat"),
            ),
            "/im/v3/api/custom/ws?deviceId=d1&scope=chat"
        );
    }
}
