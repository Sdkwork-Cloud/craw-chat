//! Browser CORS layer assembly driven by the `SDKWORK_IM_BROWSER_ORIGINS` env var.

use axum::http::{Method, header};
use tower_http::cors::{AllowHeaders, AllowMethods, CorsLayer};

use crate::constants::BROWSER_ORIGINS_ENV;

pub(crate) fn build_browser_cors_layer() -> CorsLayer {
    let environment = sdkwork_web_bootstrap::web_environment_from_env(&[
        "SDKWORK_IM_ENVIRONMENT",
        "IM_ENVIRONMENT",
        "SDKWORK_ENVIRONMENT",
    ]);
    let mut configured = resolve_browser_origins();
    if matches!(
        environment,
        sdkwork_web_core::WebEnvironment::Dev | sdkwork_web_core::WebEnvironment::Test
    ) {
        configured.push("tauri://localhost".to_owned());
    }
    let policy = sdkwork_web_bootstrap::security_policy_for_environment(&environment, configured);
    sdkwork_web_axum::cors_layer_from_policy(policy.cors)
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

fn resolve_browser_origins() -> Vec<String> {
    let configured = std::env::var(BROWSER_ORIGINS_ENV).ok();
    let origins = configured
        .as_deref()
        .map(parse_browser_origin_list)
        .filter(|origins| !origins.is_empty())
        .unwrap_or_else(default_browser_origins);

    origins
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
    Vec::new()
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
