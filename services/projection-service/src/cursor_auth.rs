//! Signed list cursors for projection read models.

use std::sync::OnceLock;

use getrandom::fill as fill_random;
use im_app_context::is_production_like_im_environment;
use sdkwork_utils_rust::{
    base64url_decode, base64url_encode, hmac_sha256_base64url, verify_hmac_sha256_base64url,
};
use serde_json::Value;

const PROJECTION_CURSOR_HS256_SECRET_ENV: &str = "SDKWORK_IM_PROJECTION_CURSOR_HS256_SECRET";
const CURSOR_VERSION: u32 = 1;

pub(crate) fn encode_signed_projection_cursor(payload: &Value) -> Result<String, String> {
    let header = serde_json::json!({
        "alg": "HS256",
        "typ": "cursor",
        "v": CURSOR_VERSION,
    });
    let header_bytes = serde_json::to_vec(&header).map_err(|error| error.to_string())?;
    let payload_bytes = serde_json::to_vec(payload).map_err(|error| error.to_string())?;
    let header_segment = base64url_encode(&header_bytes);
    let payload_segment = base64url_encode(&payload_bytes);
    let signing_input = format!("{header_segment}.{payload_segment}");
    let signature_segment =
        hmac_sha256_base64url(signing_input.as_bytes(), resolve_cursor_secret()?.as_bytes());
    Ok(format!("{signing_input}.{signature_segment}"))
}

pub(crate) fn decode_signed_projection_cursor(cursor: &str) -> Result<Value, String> {
    let parts: Vec<&str> = cursor.split('.').collect();
    if parts.len() != 3 {
        return Err("projection cursor must contain three segments".into());
    }
    let signing_input = format!("{}.{}", parts[0], parts[1]);
    let signature = base64url_decode(parts[2])
        .ok_or_else(|| "projection cursor signature is invalid".to_string())?;
    if !verify_hmac_sha256_base64url(
        signing_input.as_bytes(),
        resolve_cursor_secret()?.as_bytes(),
        signature.as_slice(),
    ) {
        return Err("projection cursor signature is invalid".to_string());
    }
    let payload_bytes = base64url_decode(parts[1])
        .ok_or_else(|| "projection cursor payload is invalid".to_string())?;
    serde_json::from_slice(&payload_bytes).map_err(|_| "projection cursor payload is invalid".to_string())
}

fn resolve_cursor_secret() -> Result<String, String> {
    if let Some(configured) = std::env::var(PROJECTION_CURSOR_HS256_SECRET_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
    {
        return Ok(configured);
    }
    if requires_configured_projection_cursor_secret() {
        return Err(format!(
            "{PROJECTION_CURSOR_HS256_SECRET_ENV} is required in production-like IM environments"
        ));
    }
    static EPHEMERAL_SECRET: OnceLock<String> = OnceLock::new();
    Ok(EPHEMERAL_SECRET
        .get_or_init(|| {
            let mut bytes = [0u8; 32];
            if fill_random(&mut bytes).is_ok() {
                tracing::warn!(
                    "{PROJECTION_CURSOR_HS256_SECRET_ENV} is unset; using ephemeral in-memory projection cursor signing secret for local development only"
                );
                base64url_encode(&bytes)
            } else {
                "sdkwork-im-dev-projection-cursor-secret".to_owned()
            }
        })
        .clone())
}

fn requires_configured_projection_cursor_secret() -> bool {
    is_production_like_im_environment()
}
