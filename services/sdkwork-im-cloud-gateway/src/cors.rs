//! Browser CORS layer assembly driven by the `SDKWORK_IM_BROWSER_ORIGINS` env var.

use axum::http::Method;
use tower_http::cors::{AllowMethods, CorsLayer};

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
        configured.extend([
            "tauri://localhost".to_owned(),
            "http://tauri.localhost".to_owned(),
            "https://tauri.localhost".to_owned(),
        ]);
    }
    let mut policy =
        sdkwork_web_bootstrap::security_policy_for_environment(&environment, configured);
    for header_name in browser_request_headers() {
        if !policy
            .cors
            .allowed_headers
            .iter()
            .any(|allowed| allowed.eq_ignore_ascii_case(&header_name))
        {
            policy.cors.allowed_headers.push(header_name);
        }
    }
    sdkwork_web_axum::cors_layer_from_policy(policy.cors).allow_methods(AllowMethods::list([
        Method::DELETE,
        Method::GET,
        Method::HEAD,
        Method::OPTIONS,
        Method::PATCH,
        Method::POST,
        Method::PUT,
    ]))
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

fn browser_request_headers() -> Vec<String> {
    [
        "authorization",
        "access-token",
        "content-type",
        "idempotency-key",
        "x-api-key",
        "x-request-id",
        "x-trace-id",
        "x-sdkwork-trace-id",
        "x-sdkwork-client-version",
        "x-device-id",
        "x-sdkwork-app-id",
        "x-sdkwork-tenant-id",
        "x-sdkwork-organization-id",
        "x-sdkwork-user-id",
        "x-sdkwork-session-id",
        "x-sdkwork-environment",
        "x-sdkwork-deployment-mode",
        "x-sdkwork-auth-level",
        "x-sdkwork-data-scope",
        "x-sdkwork-permission-scope",
        "x-sdkwork-actor-id",
        "x-sdkwork-actor-kind",
        "x-sdkwork-device-id",
        "x-sdkwork-context-signature",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
