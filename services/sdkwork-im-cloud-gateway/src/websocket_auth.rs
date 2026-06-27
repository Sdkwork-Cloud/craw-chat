//! WebSocket authentication helpers: query-token parsing, auth.init frame
//! gating, dual-token header resolution, and path/query sanitization.

use axum::http::{HeaderMap, HeaderValue, header};
use im_app_context::build_dual_token_headers_for_context;
use sdkwork_im_realtime_api_paths::REALTIME_WS;
use sdkwork_im_websocket_auth_gate::{
    normalize_websocket_auth_token, sanitized_realtime_websocket_path_and_query,
    should_require_auth_init_frame,
};
use session_gateway::RealtimeAuthContextResolver;

use crate::constants::GATEWAY_WEBSOCKET_ALLOW_QUERY_TOKENS_ENV;

fn parse_truthy_env_flag(raw: Option<String>) -> bool {
    raw.is_some_and(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    })
}

fn gateway_allows_websocket_query_tokens() -> bool {
    let env = std::env::var(GATEWAY_WEBSOCKET_ALLOW_QUERY_TOKENS_ENV).ok();
    parse_truthy_env_flag(env)
}

fn websocket_query_params(uri: &axum::http::Uri) -> Vec<(String, String)> {
    let Some(query) = uri.query() else {
        return Vec::new();
    };
    query
        .split('&')
        .filter_map(|pair| {
            let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
            let key = key.trim();
            if key.is_empty() {
                return None;
            }
            Some((key.to_owned(), value.trim().to_owned()))
        })
        .collect()
}

pub(crate) fn websocket_auth_headers_from_query(uri: &axum::http::Uri) -> Option<HeaderMap> {
    if uri.path() == REALTIME_WS {
        return None;
    }
    if !gateway_allows_websocket_query_tokens() {
        return None;
    }
    let params = websocket_query_params(uri);
    if params.is_empty() {
        return None;
    }
    let access_token = params.iter().find_map(|(key, value)| {
        if key.eq_ignore_ascii_case("accessToken")
            || key.eq_ignore_ascii_case("access_token")
            || key.eq_ignore_ascii_case("access-token")
        {
            Some(value.as_str())
        } else {
            None
        }
    })?;
    let auth_token = params.iter().find_map(|(key, value)| {
        if key.eq_ignore_ascii_case("authToken")
            || key.eq_ignore_ascii_case("auth_token")
            || key.eq_ignore_ascii_case("authorization")
            || key.eq_ignore_ascii_case("token")
        {
            Some(value.as_str())
        } else {
            None
        }
    });
    let auth_token = auth_token.unwrap_or(access_token);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(normalize_websocket_auth_token(auth_token).as_str()).ok()?,
    );
    headers.insert("access-token", HeaderValue::from_str(access_token).ok()?);
    Some(headers)
}

pub(crate) fn should_authenticate_gateway_websocket_with_init_frame(
    headers: &HeaderMap,
    uri: &axum::http::Uri,
) -> bool {
    should_require_auth_init_frame(headers, websocket_auth_headers_from_query(uri).is_some())
}

pub(crate) fn sanitized_gateway_websocket_path_and_query(uri: &axum::http::Uri) -> String {
    sanitized_realtime_websocket_path_and_query(uri.path(), uri.query())
}

pub(crate) async fn websocket_dual_token_headers_for_auth_init(
    resolver: &RealtimeAuthContextResolver,
    auth_headers: &HeaderMap,
    auth_init_device_id: Option<&str>,
) -> Result<HeaderMap, ()> {
    let mut context = resolver
        .resolve_from_headers(auth_headers)
        .await
        .map_err(|_| ())?;
    let Some(device_id) = auth_init_device_id
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        return Ok(auth_headers.clone());
    };
    if context.device_id.is_some() {
        return Ok(auth_headers.clone());
    }
    context.device_id = Some(device_id.to_owned());
    Ok(build_dual_token_headers_for_context(
        &context,
        context.permission_scope.iter(),
    ))
}
