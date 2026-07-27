use sdkwork_web_contract::{HttpMethod, HttpRoute};
use sdkwork_web_core::HttpRouteManifest;

use crate::paths;

/// API surface: open-api
pub const API_SURFACE: &str = "open-api";

pub const ROUTES: &[HttpRoute] = &[
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Get,
        paths::CALL_SESSION,
        "calls",
        "calls.sessions.retrieve",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::CALL_SESSIONS,
        "calls",
        "calls.sessions.create",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::CALL_SESSION_INVITE,
        "calls",
        "calls.sessions.invite",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::CALL_SESSION_ACCEPT,
        "calls",
        "calls.sessions.accept",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::CALL_SESSION_REJECT,
        "calls",
        "calls.sessions.reject",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::CALL_SESSION_END,
        "calls",
        "calls.sessions.end",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Get,
        paths::CALL_SESSION_SIGNALS,
        "calls",
        "calls.sessions.signals.list",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::CALL_SESSION_SIGNALS,
        "calls",
        "calls.sessions.signals.create",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::CALL_SESSION_CREDENTIALS,
        "calls",
        "calls.sessions.credentials.create",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::CALL_SESSION_CREDENTIALS_REFRESH,
        "calls",
        "calls.sessions.credentials.refresh",
    ),
];

pub fn route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(ROUTES)
}
