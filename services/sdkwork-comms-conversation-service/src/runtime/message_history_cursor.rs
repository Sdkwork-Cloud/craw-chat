use std::fmt;
use std::sync::OnceLock;

use im_app_context::is_production_like_im_environment;
use sdkwork_utils_rust::base64url_encode;
use serde::{Deserialize, Serialize};

use super::cursor_signing::{SignedCursorError, decode_signed_cursor, encode_signed_cursor};

const MESSAGE_HISTORY_CURSOR_HS256_SECRET_ENV: &str =
    "SDKWORK_IM_MESSAGE_HISTORY_CURSOR_HS256_SECRET";
const MESSAGE_HISTORY_CURSOR_VERSION: u8 = 1;
const MESSAGE_HISTORY_CURSOR_TYPE: &str = "sdkwork-im-message-history-cursor";
const MESSAGE_HISTORY_CURSOR_DIRECTION: &str = "backward";
const MESSAGE_HISTORY_CURSOR_SORT: &str = "message_seq_desc";
const MESSAGE_HISTORY_CURSOR_MIN_SECRET_BYTES: usize = 32;

#[derive(Clone, Copy, Debug)]
pub(crate) struct MessageHistoryCursorScope<'a> {
    pub tenant_id: &'a str,
    pub organization_id: &'a str,
    pub conversation_id: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum MessageHistoryCursorError {
    Invalid,
    Configuration(String),
}

impl fmt::Display for MessageHistoryCursorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid => formatter.write_str("message history cursor is invalid"),
            Self::Configuration(message) => formatter.write_str(message),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct MessageHistoryCursorPayload {
    #[serde(rename = "b")]
    before_seq: u64,
    #[serde(rename = "c")]
    conversation_id: String,
    #[serde(rename = "d")]
    direction: String,
    #[serde(rename = "o")]
    organization_id: String,
    #[serde(rename = "s")]
    sort: String,
    #[serde(rename = "t")]
    tenant_id: String,
    #[serde(rename = "v")]
    version: u8,
}

pub(crate) fn encode_message_history_cursor(
    scope: MessageHistoryCursorScope<'_>,
    before_seq: u64,
) -> Result<String, MessageHistoryCursorError> {
    let payload = MessageHistoryCursorPayload {
        before_seq,
        conversation_id: scope.conversation_id.to_owned(),
        direction: MESSAGE_HISTORY_CURSOR_DIRECTION.to_owned(),
        organization_id: scope.organization_id.to_owned(),
        sort: MESSAGE_HISTORY_CURSOR_SORT.to_owned(),
        tenant_id: scope.tenant_id.to_owned(),
        version: MESSAGE_HISTORY_CURSOR_VERSION,
    };
    encode_signed_cursor(
        MESSAGE_HISTORY_CURSOR_TYPE,
        MESSAGE_HISTORY_CURSOR_VERSION,
        &payload,
        resolve_message_history_cursor_secret()?,
    )
    .map_err(map_signed_cursor_error)
}

pub(crate) fn decode_message_history_cursor(
    cursor: &str,
    scope: MessageHistoryCursorScope<'_>,
) -> Result<u64, MessageHistoryCursorError> {
    let payload: MessageHistoryCursorPayload = decode_signed_cursor(
        cursor,
        MESSAGE_HISTORY_CURSOR_TYPE,
        MESSAGE_HISTORY_CURSOR_VERSION,
        resolve_message_history_cursor_secret()?,
    )
    .map_err(map_signed_cursor_error)?;
    if payload.version != MESSAGE_HISTORY_CURSOR_VERSION
        || payload.direction != MESSAGE_HISTORY_CURSOR_DIRECTION
        || payload.sort != MESSAGE_HISTORY_CURSOR_SORT
        || payload.tenant_id != scope.tenant_id
        || payload.organization_id != scope.organization_id
        || payload.conversation_id != scope.conversation_id
        || payload.before_seq == 0
    {
        return Err(MessageHistoryCursorError::Invalid);
    }
    Ok(payload.before_seq)
}

fn map_signed_cursor_error(error: SignedCursorError) -> MessageHistoryCursorError {
    match error {
        SignedCursorError::Invalid => MessageHistoryCursorError::Invalid,
        SignedCursorError::Serialization(message) => MessageHistoryCursorError::Configuration(
            format!("message history cursor serialization failed: {message}"),
        ),
    }
}

pub(super) fn resolve_message_history_cursor_secret()
-> Result<&'static str, MessageHistoryCursorError> {
    static SECRET: OnceLock<Result<String, String>> = OnceLock::new();
    SECRET
        .get_or_init(load_message_history_cursor_secret)
        .as_deref()
        .map_err(|message| MessageHistoryCursorError::Configuration(message.clone()))
}

fn load_message_history_cursor_secret() -> Result<String, String> {
    let file_env = format!("{MESSAGE_HISTORY_CURSOR_HS256_SECRET_ENV}_FILE");
    if let Some(file_path) = non_empty_env(file_env.as_str()) {
        let secret = std::fs::read_to_string(file_path.as_str())
            .map_err(|error| format!("failed to read {file_env} ({file_path}): {error}"))?;
        return validate_message_history_cursor_secret(secret.trim(), file_env.as_str());
    }
    if let Some(secret) = non_empty_env(MESSAGE_HISTORY_CURSOR_HS256_SECRET_ENV) {
        return validate_message_history_cursor_secret(
            secret.as_str(),
            MESSAGE_HISTORY_CURSOR_HS256_SECRET_ENV,
        );
    }
    if is_production_like_im_environment() {
        return Err(format!(
            "{MESSAGE_HISTORY_CURSOR_HS256_SECRET_ENV} (or {MESSAGE_HISTORY_CURSOR_HS256_SECRET_ENV}_FILE) is required in production-like IM environments"
        ));
    }

    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).map_err(|error| {
        format!(
            "{MESSAGE_HISTORY_CURSOR_HS256_SECRET_ENV} is unset and local cursor secret generation failed: {error}"
        )
    })?;
    tracing::warn!(
        "{MESSAGE_HISTORY_CURSOR_HS256_SECRET_ENV} is unset; using an ephemeral message-history cursor secret for local development only"
    );
    Ok(base64url_encode(bytes.as_slice()))
}

fn non_empty_env(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn validate_message_history_cursor_secret(secret: &str, source: &str) -> Result<String, String> {
    if secret.as_bytes().len() < MESSAGE_HISTORY_CURSOR_MIN_SECRET_BYTES {
        return Err(format!(
            "{source} must contain at least {MESSAGE_HISTORY_CURSOR_MIN_SECRET_BYTES} bytes"
        ));
    }
    Ok(secret.to_owned())
}

#[cfg(test)]
mod tests {
    use std::sync::Once;

    use super::{
        MessageHistoryCursorScope, decode_message_history_cursor, encode_message_history_cursor,
    };

    fn scope<'a>(conversation_id: &'a str) -> MessageHistoryCursorScope<'a> {
        static TEST_SECRET: Once = Once::new();
        TEST_SECRET.call_once(|| unsafe {
            std::env::set_var(
                "SDKWORK_IM_MESSAGE_HISTORY_CURSOR_HS256_SECRET",
                "test-message-history-cursor-secret-at-least-32-bytes",
            );
        });
        MessageHistoryCursorScope {
            tenant_id: "100001",
            organization_id: "0",
            conversation_id,
        }
    }

    #[test]
    fn signed_message_history_cursor_round_trips_without_exposing_numeric_wire() {
        let encoded =
            encode_message_history_cursor(scope("c_history"), 42).expect("cursor should encode");

        assert_ne!(encoded, "42");
        assert_eq!(
            decode_message_history_cursor(encoded.as_str(), scope("c_history")),
            Ok(42)
        );
    }

    #[test]
    fn message_history_cursor_rejects_tampering_numeric_tokens_and_cross_scope_reuse() {
        let encoded =
            encode_message_history_cursor(scope("c_history"), 42).expect("cursor should encode");
        let mut tampered = encoded.into_bytes();
        let last = tampered.last_mut().expect("cursor should not be empty");
        *last = if *last == b'A' { b'B' } else { b'A' };
        let tampered = String::from_utf8(tampered).expect("tampered cursor should remain utf8");

        assert!(decode_message_history_cursor("42", scope("c_history")).is_err());
        assert!(decode_message_history_cursor(tampered.as_str(), scope("c_history")).is_err());

        let encoded =
            encode_message_history_cursor(scope("c_history"), 42).expect("cursor should encode");
        assert!(decode_message_history_cursor(encoded.as_str(), scope("c_other")).is_err());
    }
}
