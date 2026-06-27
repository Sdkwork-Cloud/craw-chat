use axum::http::{HeaderMap, header};
use sdkwork_utils_rust::hmac_sha256_base64url;

use crate::context::AppContextSignatureConfig;
use crate::env::{
    APP_CONTEXT_REQUIRE_SIGNATURE_ENV, APP_CONTEXT_SIGNATURE_SECRET_ENV,
    SDKWORK_CONTEXT_SIGNATURE_HEADER, SIGNED_APP_CONTEXT_HEADER_NAMES,
};
use crate::error::AppContextError;
use crate::jwt::constant_time_eq;

pub(crate) fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .and_then(|value| {
            let (scheme, token) = value.split_once(' ')?;
            if scheme.eq_ignore_ascii_case("bearer") && !token.trim().is_empty() {
                return Some(token.trim().to_owned());
            }
            None
        })
}

pub(crate) fn extract_access_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("access-token")
        .or_else(|| headers.get("Access-Token"))
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| {
            value
                .strip_prefix("Bearer ")
                .or_else(|| value.strip_prefix("bearer "))
                .unwrap_or(value)
                .trim()
                .to_owned()
        })
}

pub(crate) fn canonical_app_context_signature_payload(headers: &HeaderMap) -> String {
    SIGNED_APP_CONTEXT_HEADER_NAMES
        .iter()
        .map(|name| {
            let value = headers
                .get(*name)
                .and_then(|value| value.to_str().ok())
                .map(str::trim)
                .unwrap_or("");
            format!("{name}:{value}")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn sign_app_context_headers(
    headers: &HeaderMap,
    shared_secret: &str,
) -> Result<String, AppContextError> {
    let shared_secret = shared_secret.trim();
    if shared_secret.is_empty() {
        return Err(AppContextError::invalid(
            "AppContext signature shared secret must not be empty",
        ));
    }

    let payload = canonical_app_context_signature_payload(headers);
    Ok(hmac_sha256_base64url(
        payload.as_bytes(),
        shared_secret.as_bytes(),
    ))
}

pub fn require_app_context_signature(
    headers: &HeaderMap,
    signature_config: &AppContextSignatureConfig,
) -> Result<(), AppContextError> {
    if !signature_config.require_signature {
        return Ok(());
    }

    let shared_secret = signature_config
        .shared_secret
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            AppContextError::invalid(format!(
                "{APP_CONTEXT_SIGNATURE_SECRET_ENV} is required when {APP_CONTEXT_REQUIRE_SIGNATURE_ENV}=true"
            ))
        })?;
    let actual_signature = headers
        .get(SDKWORK_CONTEXT_SIGNATURE_HEADER)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            AppContextError::invalid(format!(
                "{SDKWORK_CONTEXT_SIGNATURE_HEADER} header is required when signed AppContext projection is required"
            ))
        })?;
    let expected_signature = sign_app_context_headers(headers, shared_secret)?;
    if !constant_time_eq(actual_signature.as_bytes(), expected_signature.as_bytes()) {
        return Err(AppContextError::invalid(format!(
            "{SDKWORK_CONTEXT_SIGNATURE_HEADER} signature validation failed"
        )));
    }

    Ok(())
}

/// Returns true when the upgrade request already carries dual-token credentials in headers.
///
/// Browser clients cannot set these headers; native/mobile/Node runtimes use this path and
/// skip the post-upgrade `auth.init` frame.
pub fn has_websocket_upgrade_auth_headers(headers: &HeaderMap) -> bool {
    headers.contains_key(header::AUTHORIZATION)
        || headers.contains_key("access-token")
        || headers.contains_key("Access-Token")
}

/// Extracts `deviceId` from a websocket path-and-query string (`/path?deviceId=...`).
pub fn websocket_query_device_id_from_path_and_query(path_and_query: &str) -> Option<String> {
    let (_path, query) = path_and_query.split_once('?')?;
    query.split('&').find_map(|pair| {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        if key.trim().eq_ignore_ascii_case("deviceId") {
            Some(value.trim().to_owned()).filter(|value| !value.is_empty())
        } else {
            None
        }
    })
}

/// Prefers the `auth.init` frame `deviceId`, then falls back to the upgrade query value.
pub fn coalesce_websocket_device_id(
    frame_device_id: Option<String>,
    query_device_id: Option<String>,
) -> Option<String> {
    frame_device_id
        .or(query_device_id)
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

pub(crate) fn has_any_dual_token_header(headers: &HeaderMap) -> bool {
    has_websocket_upgrade_auth_headers(headers)
}
