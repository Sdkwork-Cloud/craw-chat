use std::sync::OnceLock;

use axum::extract::Extension;
use axum::http::HeaderMap;
use im_app_context::{AppContext, resolve_app_context};
use im_domain_core::rtc::SignalSender;

use crate::dto::{
    CreateRtcSessionRequest, InviteRtcSessionRequest, PostRtcSignalRequest,
    UpdateRtcSessionRequest,
};
use crate::error::CallingError;

const CALL_MAX_RTC_SESSION_ID_BYTES: usize = 256;
const CALL_MAX_CONVERSATION_ID_BYTES: usize = 256;
const CALL_MAX_RTC_MODE_BYTES: usize = 64;
const CALL_MAX_SIGNALING_STREAM_ID_BYTES: usize = 256;
const CALL_MAX_SIGNAL_TYPE_BYTES: usize = 128;
const CALL_MAX_SCHEMA_REF_BYTES: usize = 256;
const CALL_MAX_SIGNAL_PAYLOAD_BYTES: usize = 256 * 1024;
const CALL_MAX_ARTIFACT_MESSAGE_ID_BYTES: usize = 256;
/// Maximum number of participant IDs accepted in a single invite request.
/// Bounds the work done deduplicating and persisting invited_ids.
const CALL_MAX_PARTICIPANT_IDS: usize = 256;
/// Maximum length of a single participant ID in bytes.
const CALL_MAX_PARTICIPANT_ID_BYTES: usize = 256;
const CALLING_MAX_IN_FLIGHT_REQUESTS_ENV: &str = "SDKWORK_IM_CALLING_MAX_IN_FLIGHT_REQUESTS";
const CALLING_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 1_000;
const CALLING_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 20_000;
const CALLING_MAX_REQUEST_BODY_BYTES_ENV: &str = "SDKWORK_IM_CALLING_MAX_REQUEST_BODY_BYTES";
const CALLING_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 1024 * 1024;
const CALLING_MAX_REQUEST_BODY_BYTES_MAX: usize = 10 * 1024 * 1024;

pub(crate) fn resolve_request_app_context(
    auth: Option<Extension<AppContext>>,
    headers: &HeaderMap,
) -> Result<AppContext, CallingError> {
    match auth {
        Some(Extension(auth)) => Ok(auth),
        None => resolve_app_context(headers).map_err(CallingError::from),
    }
}

/// Cached max in-flight requests. Reading `std::env::var` on every request
/// is unnecessary filesystem work under load; the value is process-static
/// after startup.
pub(crate) fn resolve_max_in_flight_requests() -> usize {
    static CACHED: OnceLock<usize> = OnceLock::new();
    *CACHED.get_or_init(|| {
        std::env::var(CALLING_MAX_IN_FLIGHT_REQUESTS_ENV)
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|&parsed| parsed > 0)
            .unwrap_or(CALLING_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
            .min(CALLING_MAX_IN_FLIGHT_REQUESTS_MAX)
    })
}

/// Cached max HTTP request body bytes. Same rationale as
/// `resolve_max_in_flight_requests`.
pub(crate) fn resolve_max_http_request_body_bytes() -> usize {
    static CACHED: OnceLock<usize> = OnceLock::new();
    *CACHED.get_or_init(|| {
        std::env::var(CALLING_MAX_REQUEST_BODY_BYTES_ENV)
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|&parsed| parsed > 0)
            .unwrap_or(CALLING_MAX_REQUEST_BODY_BYTES_DEFAULT)
            .min(CALLING_MAX_REQUEST_BODY_BYTES_MAX)
    })
}

pub(crate) fn rtc_session_scope_key(tenant_id: &str, rtc_session_id: &str) -> String {
    im_domain_core::rtc::encode_im_call_key_segments([tenant_id, rtc_session_id])
}

fn validate_payload_size(
    field: &'static str,
    payload: &str,
    max_bytes: usize,
) -> Result<(), CallingError> {
    let payload_len = payload.len();
    if payload_len > max_bytes {
        return Err(CallingError::payload_too_large(field, max_bytes, payload_len));
    }
    Ok(())
}

pub(crate) fn validate_rtc_session_id(rtc_session_id: &str) -> Result<(), CallingError> {
    validate_payload_size("rtcSessionId", rtc_session_id, CALL_MAX_RTC_SESSION_ID_BYTES)
}

pub(crate) fn validate_create_request_payload_size(
    request: &CreateRtcSessionRequest,
) -> Result<(), CallingError> {
    validate_rtc_session_id(request.rtc_session_id.as_str())?;
    if let Some(conversation_id) = request.conversation_id.as_deref() {
        validate_payload_size(
            "conversationId",
            conversation_id,
            CALL_MAX_CONVERSATION_ID_BYTES,
        )?;
    }
    validate_payload_size("rtcMode", request.rtc_mode.as_str(), CALL_MAX_RTC_MODE_BYTES)
}

pub(crate) fn validate_invite_request_payload_size(
    request: &InviteRtcSessionRequest,
) -> Result<(), CallingError> {
    if let Some(signaling_stream_id) = request.signaling_stream_id.as_deref() {
        validate_payload_size(
            "signalingStreamId",
            signaling_stream_id,
            CALL_MAX_SIGNALING_STREAM_ID_BYTES,
        )?;
    }
    Ok(())
}

/// Validate the `participantIds` field of an invite request. Bounds both the
/// number of IDs and each ID's length to prevent unbounded work during
/// deduplication and persistence.
pub(crate) fn validate_participant_ids_payload_size(
    participant_ids: &[String],
) -> Result<(), CallingError> {
    if participant_ids.len() > CALL_MAX_PARTICIPANT_IDS {
        return Err(CallingError::payload_too_large(
            "participantIds",
            CALL_MAX_PARTICIPANT_IDS,
            participant_ids.len(),
        ));
    }
    for participant_id in participant_ids {
        validate_payload_size(
            "participantId",
            participant_id.as_str(),
            CALL_MAX_PARTICIPANT_ID_BYTES,
        )?;
    }
    Ok(())
}

pub(crate) fn validate_update_request_payload_size(
    request: &UpdateRtcSessionRequest,
) -> Result<(), CallingError> {
    if let Some(artifact_message_id) = request.artifact_message_id.as_deref() {
        validate_payload_size(
            "artifactMessageId",
            artifact_message_id,
            CALL_MAX_ARTIFACT_MESSAGE_ID_BYTES,
        )?;
    }
    Ok(())
}

pub(crate) fn validate_post_signal_request_payload_size(
    request: &PostRtcSignalRequest,
) -> Result<(), CallingError> {
    validate_payload_size(
        "signalType",
        request.signal_type.as_str(),
        CALL_MAX_SIGNAL_TYPE_BYTES,
    )?;
    if let Some(schema_ref) = request.schema_ref.as_deref() {
        validate_payload_size("schemaRef", schema_ref, CALL_MAX_SCHEMA_REF_BYTES)?;
    }
    validate_payload_size(
        "payload",
        request.payload.as_str(),
        CALL_MAX_SIGNAL_PAYLOAD_BYTES,
    )?;
    if let Some(signaling_stream_id) = request.signaling_stream_id.as_deref() {
        validate_payload_size(
            "signalingStreamId",
            signaling_stream_id,
            CALL_MAX_SIGNALING_STREAM_ID_BYTES,
        )?;
    }
    Ok(())
}

pub(crate) fn resolve_rtc_signal_sender(auth: &AppContext) -> SignalSender {
    SignalSender {
        id: auth.actor_id.clone(),
        kind: auth.actor_kind.clone(),
        member_id: auth.user_id.clone().into(),
        device_id: auth.device_id.clone(),
        session_id: auth.session_id.clone(),
        metadata: Default::default(),
    }
}

pub fn rtc_session_create_request_key(tenant_id: &str, rtc_session_id: &str) -> String {
    im_domain_core::rtc::encode_im_call_key_segments([
        tenant_id,
        "call.create",
        rtc_session_id,
    ])
}

pub fn rtc_session_invite_request_key(
    tenant_id: &str,
    rtc_session_id: &str,
    signaling_stream_id: Option<&str>,
) -> String {
    im_domain_core::rtc::encode_im_call_key_segments([
        tenant_id,
        "call.invite",
        rtc_session_id,
        signaling_stream_id.unwrap_or(""),
    ])
}

pub fn rtc_session_accept_request_key(tenant_id: &str, rtc_session_id: &str) -> String {
    im_domain_core::rtc::encode_im_call_key_segments([tenant_id, "call.accept", rtc_session_id])
}

pub fn rtc_session_reject_request_key(tenant_id: &str, rtc_session_id: &str) -> String {
    im_domain_core::rtc::encode_im_call_key_segments([tenant_id, "call.reject", rtc_session_id])
}

pub fn rtc_session_end_request_key(tenant_id: &str, rtc_session_id: &str) -> String {
    im_domain_core::rtc::encode_im_call_key_segments([tenant_id, "call.end", rtc_session_id])
}

pub(crate) fn call_session_matches_create_request(
    session: &im_domain_core::rtc::RtcSession,
    auth: &AppContext,
    request: &CreateRtcSessionRequest,
) -> bool {
    session.initiator_id == auth.actor_id
        && session.initiator_kind == auth.actor_kind
        && session.rtc_mode == request.rtc_mode
        && session.conversation_id.as_deref() == request.conversation_id.as_deref()
}