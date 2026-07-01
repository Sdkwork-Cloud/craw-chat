use std::collections::BTreeMap;
use std::sync::{Mutex, MutexGuard};

use im_app_context::AppContext;
use im_domain_core::message::Sender;
use im_domain_core::stream::{StreamDurabilityClass, StreamFrame, StreamSession, StreamSessionState};

use crate::dto::{
    AbortStreamRequest, AppendStreamFrameRequest, CheckpointStreamRequest, CompleteStreamRequest,
    OpenStreamRequest,
};
use crate::error::StreamingError;
use crate::state::StreamingRuntime;

const STREAM_MAX_STREAM_ID_BYTES: usize = 256;
const STREAM_MAX_STREAM_TYPE_BYTES: usize = 128;
const STREAM_MAX_SCOPE_KIND_BYTES: usize = 64;
const STREAM_MAX_SCOPE_ID_BYTES: usize = 512;
const STREAM_MAX_DURABILITY_CLASS_BYTES: usize = 64;
const STREAM_MAX_SCHEMA_REF_BYTES: usize = 256;
const STREAM_MAX_FRAME_TYPE_BYTES: usize = 64;
const STREAM_MAX_FRAME_ENCODING_BYTES: usize = 32;
const STREAM_MAX_FRAME_PAYLOAD_BYTES: usize = 256 * 1024;
const STREAM_MAX_FRAME_ATTRIBUTES_BYTES: usize = 64 * 1024;
const STREAM_MAX_RESULT_MESSAGE_ID_BYTES: usize = 256;
const STREAM_MAX_ABORT_REASON_BYTES: usize = 8 * 1024;
const STREAMING_MAX_IN_FLIGHT_REQUESTS_ENV: &str = "SDKWORK_IM_STREAMING_MAX_IN_FLIGHT_REQUESTS";
const STREAMING_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 1_000;
const STREAMING_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 20_000;
const STREAMING_MAX_REQUEST_BODY_BYTES_ENV: &str = "SDKWORK_IM_STREAMING_MAX_REQUEST_BODY_BYTES";
const STREAMING_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
const STREAMING_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;

pub(crate) fn resolve_max_in_flight_requests() -> usize {
    std::env::var(STREAMING_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(STREAMING_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(STREAMING_MAX_IN_FLIGHT_REQUESTS_MAX)
}

pub(crate) fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(STREAMING_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(STREAMING_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(STREAMING_MAX_REQUEST_BODY_BYTES_MAX)
}

fn validate_payload_size(
    field: &'static str,
    payload: &str,
    max_bytes: usize,
) -> Result<(), StreamingError> {
    let payload_len = payload.len();
    if payload_len > max_bytes {
        return Err(StreamingError::payload_too_large(
            field,
            max_bytes,
            payload_len,
        ));
    }
    Ok(())
}

pub(crate) fn validate_stream_id(stream_id: &str) -> Result<(), StreamingError> {
    validate_payload_size("streamId", stream_id, STREAM_MAX_STREAM_ID_BYTES)
}

pub(crate) fn validate_open_stream_request_payload_size(
    request: &OpenStreamRequest,
) -> Result<(), StreamingError> {
    validate_stream_id(request.stream_id.as_str())?;
    validate_payload_size(
        "streamType",
        request.stream_type.as_str(),
        STREAM_MAX_STREAM_TYPE_BYTES,
    )?;
    validate_payload_size(
        "scopeKind",
        request.scope_kind.as_str(),
        STREAM_MAX_SCOPE_KIND_BYTES,
    )?;
    validate_payload_size("scopeId", request.scope_id.as_str(), STREAM_MAX_SCOPE_ID_BYTES)?;
    validate_payload_size(
        "durabilityClass",
        request.durability_class.as_str(),
        STREAM_MAX_DURABILITY_CLASS_BYTES,
    )?;
    if let Some(schema_ref) = request.schema_ref.as_deref() {
        validate_payload_size("schemaRef", schema_ref, STREAM_MAX_SCHEMA_REF_BYTES)?;
    }
    Ok(())
}

pub(crate) fn validate_append_frame_request_payload_size(
    request: &AppendStreamFrameRequest,
) -> Result<(), StreamingError> {
    validate_payload_size(
        "frameType",
        request.frame_type.as_str(),
        STREAM_MAX_FRAME_TYPE_BYTES,
    )?;
    validate_payload_size(
        "encoding",
        request.encoding.as_str(),
        STREAM_MAX_FRAME_ENCODING_BYTES,
    )?;
    validate_payload_size(
        "payload",
        request.payload.as_str(),
        STREAM_MAX_FRAME_PAYLOAD_BYTES,
    )?;
    if let Some(schema_ref) = request.schema_ref.as_deref() {
        validate_payload_size("schemaRef", schema_ref, STREAM_MAX_SCHEMA_REF_BYTES)?;
    }
    let attributes_bytes = request
        .attributes
        .iter()
        .map(|(key, value)| key.len() + value.len())
        .sum::<usize>();
    if attributes_bytes > STREAM_MAX_FRAME_ATTRIBUTES_BYTES {
        return Err(StreamingError::payload_too_large(
            "attributes",
            STREAM_MAX_FRAME_ATTRIBUTES_BYTES,
            attributes_bytes,
        ));
    }
    Ok(())
}

pub(crate) fn validate_complete_stream_request_payload_size(
    request: &CompleteStreamRequest,
) -> Result<(), StreamingError> {
    if let Some(result_message_id) = request.result_message_id.as_deref() {
        validate_payload_size(
            "resultMessageId",
            result_message_id,
            STREAM_MAX_RESULT_MESSAGE_ID_BYTES,
        )?;
    }
    Ok(())
}

pub(crate) fn validate_abort_stream_request_payload_size(
    request: &AbortStreamRequest,
) -> Result<(), StreamingError> {
    if let Some(reason) = request.reason.as_deref() {
        validate_payload_size("reason", reason, STREAM_MAX_ABORT_REASON_BYTES)?;
    }
    Ok(())
}

pub(crate) fn lock_stream_mutex<'a, T>(
    mutex: &'a Mutex<T>,
    lock_name: &'static str,
) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovered poisoned stream mutex lock={lock_name}");
            poisoned.into_inner()
        }
    }
}

pub(crate) fn stream_scope_key(tenant_id: &str, stream_id: &str) -> String {
    encode_stream_key_segments([tenant_id, stream_id])
}

fn encode_stream_key_segments<'a>(segments: impl IntoIterator<Item = &'a str>) -> String {
    let mut encoded = String::new();
    for segment in segments {
        encoded.push_str(segment.len().to_string().as_str());
        encoded.push('#');
        encoded.push_str(segment);
    }
    encoded
}

pub(crate) fn stream_frame_index(frames: Vec<StreamFrame>) -> BTreeMap<u64, StreamFrame> {
    frames
        .into_iter()
        .filter(|frame| frame.frame_seq > 0)
        .fold(BTreeMap::new(), |mut index, frame| {
            index.insert(frame.frame_seq, frame);
            index
        })
}

pub fn stream_open_request_key(auth: &AppContext, stream_id: &str) -> String {
    encode_stream_key_segments([
        auth.tenant_id.as_str(),
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
        "open",
        stream_id,
    ])
}

pub fn stream_complete_request_key(auth: &AppContext, stream_id: &str) -> String {
    encode_stream_key_segments([
        auth.tenant_id.as_str(),
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
        "complete",
        stream_id,
    ])
}

pub fn stream_checkpoint_request_key(auth: &AppContext, stream_id: &str, frame_seq: u64) -> String {
    let frame_seq = frame_seq.to_string();
    encode_stream_key_segments([
        auth.tenant_id.as_str(),
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
        "checkpoint",
        stream_id,
        frame_seq.as_str(),
    ])
}

pub fn stream_abort_request_key(auth: &AppContext, stream_id: &str) -> String {
    encode_stream_key_segments([
        auth.tenant_id.as_str(),
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
        "abort",
        stream_id,
    ])
}

pub fn stream_append_request_key(auth: &AppContext, stream_id: &str, frame_seq: u64) -> String {
    let frame_seq = frame_seq.to_string();
    encode_stream_key_segments([
        auth.tenant_id.as_str(),
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
        "append",
        stream_id,
        frame_seq.as_str(),
    ])
}

pub(crate) fn ensure_standalone_stream_open_allowed(
    request: &OpenStreamRequest,
) -> Result<(), StreamingError> {
    if request.scope_kind != "conversation" {
        return Ok(());
    }

    Err(conversation_gateway_required(
        "conversation-bound streams must be opened through an authorizing IM gateway",
    ))
}

pub(crate) fn ensure_standalone_stream_session_allowed(
    runtime: &StreamingRuntime,
    auth: &AppContext,
    stream_id: &str,
) -> Result<(), StreamingError> {
    let session = runtime.session(auth, stream_id)?;
    if session.scope_kind != "conversation" {
        return Ok(());
    }

    Err(conversation_gateway_required(
        "conversation-bound streams must be accessed through an authorizing IM gateway",
    ))
}

fn conversation_gateway_required(message: &str) -> StreamingError {
    StreamingError {
        status: axum::http::StatusCode::FORBIDDEN,
        code: "conversation_gateway_required",
        message: message.into(),
    }
}

pub(crate) fn stream_session_matches_open_request(
    session: &StreamSession,
    auth: &AppContext,
    request: &OpenStreamRequest,
    durability_class: &StreamDurabilityClass,
) -> bool {
    stream_session_matches_owner_principal(session, auth)
        && session.stream_id == request.stream_id.as_str()
        && session.stream_type == request.stream_type.as_str()
        && session.scope_kind == request.scope_kind.as_str()
        && session.scope_id == request.scope_id.as_str()
        && session.durability_class == *durability_class
        && session.schema_ref.as_ref() == request.schema_ref.as_ref()
}

pub(crate) fn stream_checkpoint_matches_request(
    session: &StreamSession,
    auth: &AppContext,
    request: &CheckpointStreamRequest,
) -> bool {
    stream_session_matches_owner_principal(session, auth)
        && session.last_checkpoint_seq == Some(request.frame_seq)
}

pub(crate) fn stream_completion_matches_request(
    session: &StreamSession,
    auth: &AppContext,
    request: &CompleteStreamRequest,
) -> bool {
    stream_session_matches_owner_principal(session, auth)
        && session.state == StreamSessionState::Completed
        && session.complete_frame_seq == Some(request.frame_seq)
        && session.result_message_id == request.result_message_id
}

pub(crate) fn stream_abort_matches_request(
    session: &StreamSession,
    auth: &AppContext,
    request: &AbortStreamRequest,
) -> bool {
    stream_session_matches_owner_principal(session, auth)
        && session.state == StreamSessionState::Aborted
        && session.abort_frame_seq == request.frame_seq
        && session.abort_reason == request.reason
}

fn stream_session_matches_owner_principal(session: &StreamSession, auth: &AppContext) -> bool {
    session.owner_principal_id == auth.actor_id && session.owner_principal_kind == auth.actor_kind
}

pub(crate) fn ensure_stream_session_actor_access(
    session: &StreamSession,
    auth: &AppContext,
    stream_id: &str,
) -> Result<(), StreamingError> {
    if session.scope_kind != "conversation"
        && !stream_session_matches_owner_principal(session, auth)
    {
        return Err(StreamingError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "stream_not_found",
            message: format!("stream not found: {stream_id}"),
        });
    }

    Ok(())
}

pub(crate) fn resolve_stream_frame_sender(auth: &AppContext) -> Sender {
    Sender {
        id: auth.actor_id.clone(),
        kind: auth.actor_kind.clone(),
        member_id: None,
        device_id: auth.device_id.clone(),
        session_id: auth.session_id.clone(),
        metadata: BTreeMap::new(),
    }
}
