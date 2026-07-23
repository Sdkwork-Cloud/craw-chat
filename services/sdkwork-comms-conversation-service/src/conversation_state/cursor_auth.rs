//! Signed list cursors for conversation_state read models.

use std::sync::OnceLock;

use getrandom::fill as fill_random;
use im_app_context::is_production_like_im_environment;
use sdkwork_utils_rust::{
    base64url_decode, base64url_encode, hmac_sha256_base64url, verify_hmac_sha256_base64url,
};
use serde_json::Value;

const CONVERSATION_STATE_CURSOR_HS256_SECRET_ENV: &str =
    "SDKWORK_IM_CONVERSATION_STATE_CURSOR_HS256_SECRET";
const CURSOR_VERSION: u32 = 1;

pub(crate) fn encode_conversation_state_list_cursor(
    payload: &Value,
) -> Result<String, crate::conversation_state::event_apply::ConversationStateError> {
    encode_signed_conversation_state_cursor(payload)
        .map_err(crate::conversation_state::event_apply::ConversationStateError::InvalidEvent)
}

pub(crate) fn encode_signed_conversation_state_cursor(payload: &Value) -> Result<String, String> {
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
    let signature_segment = hmac_sha256_base64url(
        signing_input.as_bytes(),
        resolve_cursor_secret()?.as_bytes(),
    );
    Ok(format!("{signing_input}.{signature_segment}"))
}

pub(crate) fn decode_signed_conversation_state_cursor(cursor: &str) -> Result<Value, String> {
    let parts: Vec<&str> = cursor.split('.').collect();
    if parts.len() != 3 {
        return Err("conversation_state cursor must contain three segments".into());
    }
    let signing_input = format!("{}.{}", parts[0], parts[1]);
    let signature = base64url_decode(parts[2])
        .ok_or_else(|| "conversation_state cursor signature is invalid".to_string())?;
    if !verify_hmac_sha256_base64url(
        signing_input.as_bytes(),
        resolve_cursor_secret()?.as_bytes(),
        signature.as_slice(),
    ) {
        return Err("conversation_state cursor signature is invalid".to_string());
    }
    let payload_bytes = base64url_decode(parts[1])
        .ok_or_else(|| "conversation_state cursor payload is invalid".to_string())?;
    serde_json::from_slice(&payload_bytes)
        .map_err(|_| "conversation_state cursor payload is invalid".to_string())
}

fn resolve_cursor_secret() -> Result<String, String> {
    // 1. Check _FILE variant first (Docker/K8s secret injection pattern)
    let file_env = format!("{CONVERSATION_STATE_CURSOR_HS256_SECRET_ENV}_FILE");
    if let Some(file_path) = std::env::var(&file_env)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
    {
        let secret = std::fs::read_to_string(&file_path)
            .map_err(|e| format!("failed to read {file_env} ({file_path}): {e}"))?;
        let secret = secret.trim().to_owned();
        if secret.is_empty() {
            return Err(format!("{file_env} ({file_path}) contains an empty secret"));
        }
        return Ok(secret);
    }

    // 2. Check direct env var
    if let Some(configured) = std::env::var(CONVERSATION_STATE_CURSOR_HS256_SECRET_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
    {
        return Ok(configured);
    }

    // 3. Production fail-closed: no secret configured
    if requires_configured_conversation_state_cursor_secret() {
        return Err(format!(
            "{CONVERSATION_STATE_CURSOR_HS256_SECRET_ENV} (or {CONVERSATION_STATE_CURSOR_HS256_SECRET_ENV}_FILE) is required in production-like IM environments"
        ));
    }

    // 4. Non-production: generate ephemeral secret (no hardcoded fallback)
    static EPHEMERAL_SECRET: OnceLock<String> = OnceLock::new();
    EPHEMERAL_SECRET
        .get_or_init(|| {
            let mut bytes = [0u8; 32];
            match fill_random(&mut bytes) {
                Ok(()) => {
                    tracing::warn!(
                        "{CONVERSATION_STATE_CURSOR_HS256_SECRET_ENV} is unset; using ephemeral in-memory conversation_state cursor signing secret for local development only"
                    );
                    base64url_encode(&bytes)
                }
                Err(e) => {
                    tracing::error!(
                        error = %e,
                        "{CONVERSATION_STATE_CURSOR_HS256_SECRET_ENV} is unset and ephemeral secret generation failed; cursor signing will fail"
                    );
                    String::new()
                }
            }
        });
    EPHEMERAL_SECRET
        .get()
        .cloned()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            format!(
                "{CONVERSATION_STATE_CURSOR_HS256_SECRET_ENV} is unset and ephemeral secret generation failed; set the env var or ensure system randomness is available"
            )
        })
}

fn requires_configured_conversation_state_cursor_secret() -> bool {
    is_production_like_im_environment()
}
