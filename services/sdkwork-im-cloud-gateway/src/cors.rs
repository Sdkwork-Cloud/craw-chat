//! Browser CORS layer assembly driven by the `SDKWORK_IM_BROWSER_ORIGINS` env var.

use axum::http::{Method, header};
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

use crate::constants::BROWSER_ORIGINS_ENV;

pub(crate) fn build_browser_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(AllowOrigin::list(resolve_browser_origins()))
        .allow_methods(AllowMethods::list([
            Method::DELETE,
            Method::GET,
            Method::HEAD,
            Method::OPTIONS,
            Method::PATCH,
            Method::POST,
            Method::PUT,
        ]))
        .allow_headers(AllowHeaders::list(resolve_browser_headers()))
}

fn resolve_browser_origins() -> Vec<header::HeaderValue> {
    let configured = std::env::var(BROWSER_ORIGINS_ENV).ok();
    let origins = configured
        .as_deref()
        .map(parse_browser_origin_list)
        .filter(|origins| !origins.is_empty())
        .unwrap_or_else(default_browser_origins);

    origins
        .into_iter()
        .map(|origin| {
            origin
                .parse::<header::HeaderValue>()
                .expect("configured browser origin should be a valid header value")
        })
        .collect()
}

fn parse_browser_origin_list(raw: &str) -> Vec<String> {
    let mut origins = Vec::new();
    for value in raw
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        let normalized = value.trim_end_matches('/').to_owned();
        if !origins.contains(&normalized) {
            origins.push(normalized);
        }
    }
    origins
}

fn default_browser_origins() -> Vec<String> {
    [
        "http://127.0.0.1:1620",
        "http://localhost:1620",
        "http://127.0.0.1:4176",
        "http://localhost:4176",
        "tauri://localhost",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn resolve_browser_headers() -> Vec<header::HeaderName> {
    let mut headers = Vec::new();
    for header_name in [
        header::AUTHORIZATION.as_str(),
        header::CONTENT_TYPE.as_str(),
        "access-token",
    ] {
        if let Ok(parsed) = header_name.parse::<header::HeaderName>()
            && !headers.contains(&parsed)
        {
            headers.push(parsed);
        }
    }
    headers
}
