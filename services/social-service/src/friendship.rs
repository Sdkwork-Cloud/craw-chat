use std::cmp::Ordering as CmpOrdering;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::{Json, response::Response};
use sdkwork_routes_web_framework_backend_api::response::ApiProblem;
use sdkwork_utils_rust::{
    base64url_decode, base64url_encode, hmac_sha256_base64url, verify_hmac_sha256_base64url,
};
use sdkwork_web_core::{
    problem_response, ProblemCorrelation, WebFrameworkError, WebFrameworkErrorKind,
};
use getrandom::fill as fill_random;
use im_app_context::AppContext;
use im_domain_core::social::{
    DirectChat, DirectChatStatus, FriendRequest, FriendRequestStatus, Friendship, FriendshipStatus,
    normalize_actor_pair, normalize_user_pair,
};
use im_domain_events::social::{
    DirectChatBoundPayload, FriendRequestAcceptedPayload, FriendRequestCanceledPayload,
    FriendRequestDeclinedPayload, FriendRequestSubmittedPayload, FriendshipActivatedPayload,
    FriendshipRemovedPayload, SocialCommitEnvelopeInput, SocialEventType, social_commit_envelope,
};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use serde::{Deserialize, Serialize};

use crate::runtime::{
    SocialRuntime, SocialWritePersistence, StoredDirectChat, StoredFriendRequest, StoredFriendship,
    active_direct_chat_record_for_pair, active_friendship_record_for_pair,
    active_friendship_scoped_user_block, archive_active_direct_chats_for_pair,
    deterministic_social_id, friend_request_records_for_user,
    friendship_pair_has_materialized_record, open_friend_request_record_for_pair,
    social_pair_block_conflict_details,
};

const MAX_ID_BYTES: usize = 256;
const MAX_TIMESTAMP_BYTES: usize = 64;
const MAX_REQUEST_MESSAGE_BYTES: usize = 8 * 1024;
const FRIEND_REQUEST_LIST_DEFAULT_LIMIT: usize = 100;
const FRIEND_REQUEST_LIST_MAX_LIMIT: usize = 200;
const FRIEND_REQUEST_LIST_MAX_CURSOR_BYTES: usize = 1024;
const FRIEND_REQUEST_CURSOR_VERSION: u64 = 1;
const FRIEND_REQUEST_CURSOR_HS256_SECRET_ENV: &str =
    "SDKWORK_IM_FRIEND_REQUEST_CURSOR_HS256_SECRET";
const SDKWORK_IM_ENVIRONMENT_ENV: &str = "SDKWORK_IM_ENVIRONMENT";

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct SocialServiceError {
    status: StatusCode,
    code: &'static str,
    message: String,
    details: Option<serde_json::Value>,
}

impl SocialServiceError {
    pub(crate) fn invalid(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code,
            message: message.into(),
            details: None,
        }
    }

    pub(crate) fn not_found(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code,
            message: message.into(),
            details: None,
        }
    }

    pub(crate) fn conflict(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::CONFLICT,
            code,
            message: message.into(),
            details: None,
        }
    }

    pub(crate) fn conflict_with_details(
        code: &'static str,
        message: impl Into<String>,
        details: serde_json::Value,
    ) -> Self {
        Self {
            status: StatusCode::CONFLICT,
            code,
            message: message.into(),
            details: Some(details),
        }
    }

    pub(crate) fn payload_too_large(field: &str, max_bytes: usize, actual_bytes: usize) -> Self {
        Self {
            status: StatusCode::PAYLOAD_TOO_LARGE,
            code: "payload_too_large",
            message: format!(
                "{field} exceeds maximum size of {max_bytes} bytes (got {actual_bytes})"
            ),
            details: None,
        }
    }

    fn from_string(error: String) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "social_service_error",
            message: error,
            details: None,
        }
    }
}

fn social_service_error_kind(status: &StatusCode) -> WebFrameworkErrorKind {
    match *status {
        StatusCode::BAD_REQUEST => WebFrameworkErrorKind::BadRequest,
        StatusCode::UNAUTHORIZED => WebFrameworkErrorKind::MissingCredentials,
        StatusCode::FORBIDDEN => WebFrameworkErrorKind::Forbidden,
        StatusCode::NOT_FOUND => WebFrameworkErrorKind::NotFound,
        StatusCode::CONFLICT => WebFrameworkErrorKind::Conflict,
        StatusCode::PAYLOAD_TOO_LARGE => WebFrameworkErrorKind::PayloadTooLarge,
        StatusCode::SERVICE_UNAVAILABLE => WebFrameworkErrorKind::DependencyUnavailable,
        _ => WebFrameworkErrorKind::InternalServerError,
    }
}

impl From<SocialServiceError> for ApiProblem {
    fn from(error: SocialServiceError) -> Self {
        let framework_error = WebFrameworkError {
            kind: social_service_error_kind(&error.status),
            message: error.message,
            retry_after_seconds: None,
        };
        ApiProblem::from_web_framework(framework_error)
    }
}

impl IntoResponse for SocialServiceError {
    fn into_response(self) -> Response {
        let error = WebFrameworkError {
            kind: social_service_error_kind(&self.status),
            message: self.message,
            retry_after_seconds: None,
        };
        problem_response(&error, ProblemCorrelation::from(None))
    }
}

impl From<String> for SocialServiceError {
    fn from(error: String) -> Self {
        Self::from_string(error)
    }
}

fn social_event_id_conflict_string(
    event_id: &str,
    existing: &crate::runtime::SocialCommittedEvent,
) -> String {
    let committed = existing.commit();
    format!(
        "eventId {} is already committed for {} {}",
        event_id,
        existing.aggregate_label(),
        committed.aggregate_id
    )
}

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SubmitFriendRequestWireRequest {
    pub(crate) event_id: String,
    pub(crate) requester_user_id: String,
    pub(crate) target_user_id: String,
    pub(crate) request_message: Option<String>,
    pub(crate) requested_at: String,
}

#[derive(Debug, Clone)]
pub(crate) struct SubmitFriendRequestRequest {
    pub(crate) request_id: String,
    pub(crate) event_id: String,
    pub(crate) requester_user_id: String,
    pub(crate) target_user_id: String,
    pub(crate) request_message: Option<String>,
    pub(crate) requested_at: String,
}

impl SubmitFriendRequestRequest {
    pub(crate) fn from_wire(
        request_id: String,
        wire: SubmitFriendRequestWireRequest,
    ) -> Self {
        Self {
            request_id,
            event_id: wire.event_id,
            requester_user_id: wire.requester_user_id,
            target_user_id: wire.target_user_id,
            request_message: wire.request_message,
            requested_at: wire.requested_at,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AcceptFriendRequestRequest {
    pub(crate) event_id: String,
    pub(crate) accepted_by_user_id: String,
    pub(crate) accepted_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DeclineFriendRequestRequest {
    pub(crate) event_id: String,
    pub(crate) declined_by_user_id: String,
    pub(crate) declined_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CancelFriendRequestRequest {
    pub(crate) event_id: String,
    pub(crate) canceled_by_user_id: String,
    pub(crate) canceled_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ActivateFriendshipRequest {
    pub(crate) friendship_id: String,
    pub(crate) event_id: String,
    pub(crate) initiator_user_id: String,
    pub(crate) peer_user_id: String,
    pub(crate) direct_chat_id: Option<String>,
    pub(crate) established_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RemoveFriendshipRequest {
    pub(crate) event_id: String,
    pub(crate) removed_by_user_id: String,
    pub(crate) removed_at: String,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FriendRequestInventoryDirectionQuery {
    Incoming,
    Outgoing,
}

#[derive(Clone, Copy, Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FriendRequestInventoryStatusQuery {
    #[default]
    Pending,
    Accepted,
    Declined,
    Canceled,
    Expired,
    All,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FriendRequestInventoryQuery {
    pub(crate) user_id: String,
    pub(crate) direction: FriendRequestInventoryDirectionQuery,
    #[serde(default)]
    pub(crate) status: FriendRequestInventoryStatusQuery,
    pub(crate) limit: Option<usize>,
    pub(crate) cursor: Option<String>,
}

// ---------------------------------------------------------------------------
// Result types (business logic layer)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct SubmittedFriendRequest {
    pub(crate) friend_request: FriendRequest,
    pub(crate) latest_commit: CommitEnvelope,
    pub(crate) persistence: SocialWritePersistence,
}

#[derive(Clone, Debug)]
pub(crate) struct AcceptedFriendRequest {
    pub(crate) friend_request: FriendRequest,
    pub(crate) latest_commit: CommitEnvelope,
    pub(crate) persistence: SocialWritePersistence,
    pub(crate) friendship: Option<Friendship>,
    pub(crate) friendship_materialized_commit: Option<CommitEnvelope>,
    pub(crate) direct_chat: Option<DirectChat>,
    pub(crate) direct_chat_materialized_commit: Option<CommitEnvelope>,
}

#[derive(Clone, Debug)]
pub(crate) struct DeclinedFriendRequest {
    pub(crate) friend_request: FriendRequest,
    pub(crate) latest_commit: CommitEnvelope,
    pub(crate) persistence: SocialWritePersistence,
}

#[derive(Clone, Debug)]
pub(crate) struct CanceledFriendRequest {
    pub(crate) friend_request: FriendRequest,
    pub(crate) latest_commit: CommitEnvelope,
    pub(crate) persistence: SocialWritePersistence,
}

#[derive(Clone, Debug)]
pub(crate) struct ActivatedFriendship {
    pub(crate) friendship: Friendship,
    pub(crate) latest_commit: CommitEnvelope,
    pub(crate) persistence: SocialWritePersistence,
}

#[derive(Clone, Debug)]
pub(crate) struct RemovedFriendship {
    pub(crate) friendship: Friendship,
    pub(crate) latest_commit: CommitEnvelope,
    pub(crate) persistence: SocialWritePersistence,
}

// ---------------------------------------------------------------------------
// HTTP response types
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SocialFriendRequestWriteStatus {
    Submitted,
    Accepted,
    Declined,
    Canceled,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SocialFriendRequestReadStatus {
    Snapshot,
    Inventory,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SocialFriendshipWriteStatus {
    Activated,
    Removed,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SocialFriendshipReadStatus {
    Snapshot,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CommitEnvelopeResponse {
    event_id: String,
    tenant_id: String,
    event_type: String,
    event_version: u16,
    aggregate_type: String,
    aggregate_id: String,
    scope_type: String,
    scope_id: String,
    ordering_key: String,
    ordering_seq: u64,
    causation_id: Option<String>,
    correlation_id: Option<String>,
    idempotency_key: Option<String>,
    actor: EventActorResponse,
    occurred_at: String,
    committed_at: String,
    payload_schema: Option<String>,
    payload: String,
    retention_class: String,
    audit_class: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct EventActorResponse {
    actor_id: String,
    actor_kind: String,
    actor_session_id: Option<String>,
}

impl From<CommitEnvelope> for CommitEnvelopeResponse {
    fn from(value: CommitEnvelope) -> Self {
        Self {
            event_id: value.event_id,
            tenant_id: value.tenant_id,
            event_type: value.event_type,
            event_version: value.event_version,
            aggregate_type: value.aggregate_type.as_wire_value().into(),
            aggregate_id: value.aggregate_id,
            scope_type: value.scope_type,
            scope_id: value.scope_id,
            ordering_key: value.ordering_key,
            ordering_seq: value.ordering_seq,
            causation_id: value.causation_id,
            correlation_id: value.correlation_id,
            idempotency_key: value.idempotency_key,
            actor: EventActorResponse {
                actor_id: value.actor.actor_id,
                actor_kind: value.actor.actor_kind,
                actor_session_id: value.actor.actor_session_id,
            },
            occurred_at: value.occurred_at,
            committed_at: value.committed_at,
            payload_schema: value.payload_schema,
            payload: value.payload,
            retention_class: value.retention_class,
            audit_class: value.audit_class,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocialFriendRequestCommitResponse {
    status: SocialFriendRequestWriteStatus,
    friend_request: FriendRequest,
    latest_commit: CommitEnvelopeResponse,
    persistence: SocialWritePersistence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    friendship: Option<Friendship>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    friendship_latest_commit: Option<CommitEnvelopeResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    direct_chat: Option<DirectChat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    direct_chat_latest_commit: Option<CommitEnvelopeResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocialFriendRequestSnapshotResponse {
    status: SocialFriendRequestReadStatus,
    friend_request: FriendRequest,
    commits: Vec<CommitEnvelopeResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocialFriendRequestInventoryResponse {
    status: SocialFriendRequestReadStatus,
    items: Vec<FriendRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    next_cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FriendRequestInventoryCursor {
    v: u64,
    updated_at: String,
    created_at: String,
    request_id: String,
}

#[derive(Debug)]
pub(crate) struct FriendRequestInventoryPage {
    pub(crate) items: Vec<FriendRequest>,
    pub(crate) next_cursor: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocialFriendshipCommitResponse {
    status: SocialFriendshipWriteStatus,
    friendship: Friendship,
    latest_commit: CommitEnvelopeResponse,
    persistence: SocialWritePersistence,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SocialFriendshipSnapshotResponse {
    status: SocialFriendshipReadStatus,
    friendship: Friendship,
    commits: Vec<CommitEnvelopeResponse>,
}

// ---------------------------------------------------------------------------
// AppState
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct AppState {
    pub social_runtime: std::sync::Arc<SocialRuntime>,
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn validate_payload_size(
    field: &'static str,
    value: &str,
    max_bytes: usize,
) -> Result<(), SocialServiceError> {
    if value.len() > max_bytes {
        return Err(SocialServiceError::payload_too_large(
            field,
            max_bytes,
            value.len(),
        ));
    }
    Ok(())
}

fn validate_optional_payload_size(
    field: &'static str,
    value: Option<&str>,
    max_bytes: usize,
) -> Result<(), SocialServiceError> {
    if let Some(value) = value {
        validate_payload_size(field, value, max_bytes)?;
    }
    Ok(())
}

fn validate_required(field: &'static str, value: &str) -> Result<(), SocialServiceError> {
    validate_required_with_code(field, value, "invalid_friend_request")
}

fn validate_required_with_code(
    field: &'static str,
    value: &str,
    code: &'static str,
) -> Result<(), SocialServiceError> {
    if value.trim().is_empty() {
        return Err(SocialServiceError::invalid(
            code,
            format!("{field} cannot be empty"),
        ));
    }
    Ok(())
}

fn social_event_id_conflict(
    event_id: &str,
    existing: &crate::runtime::SocialCommittedEvent,
) -> SocialServiceError {
    let committed = existing.commit();
    SocialServiceError::conflict(
        "social_event_id_conflict",
        format!(
            "eventId {} is already committed for {} {}",
            event_id,
            existing.aggregate_label(),
            committed.aggregate_id
        ),
    )
}

// ---------------------------------------------------------------------------
// Friend request inventory helpers
// ---------------------------------------------------------------------------

fn friend_request_matches_inventory_direction(
    friend_request: &FriendRequest,
    user_id: &str,
    direction: FriendRequestInventoryDirectionQuery,
) -> bool {
    match direction {
        FriendRequestInventoryDirectionQuery::Incoming => friend_request.target_user_id == user_id,
        FriendRequestInventoryDirectionQuery::Outgoing => {
            friend_request.requester_user_id == user_id
        }
    }
}

fn friend_request_matches_inventory_status(
    friend_request: &FriendRequest,
    status: FriendRequestInventoryStatusQuery,
) -> bool {
    match status {
        FriendRequestInventoryStatusQuery::Pending => {
            friend_request.status == FriendRequestStatus::Pending
        }
        FriendRequestInventoryStatusQuery::Accepted => {
            friend_request.status == FriendRequestStatus::Accepted
        }
        FriendRequestInventoryStatusQuery::Declined => {
            friend_request.status == FriendRequestStatus::Declined
        }
        FriendRequestInventoryStatusQuery::Canceled => {
            friend_request.status == FriendRequestStatus::Canceled
        }
        FriendRequestInventoryStatusQuery::Expired => {
            friend_request.status == FriendRequestStatus::Expired
        }
        FriendRequestInventoryStatusQuery::All => true,
    }
}

fn compare_friend_request_inventory_order(
    left: &FriendRequest,
    right: &FriendRequest,
) -> CmpOrdering {
    compare_friend_request_inventory_sort_key(
        left.updated_at.as_str(),
        left.created_at.as_str(),
        left.request_id.as_str(),
        right.updated_at.as_str(),
        right.created_at.as_str(),
        right.request_id.as_str(),
    )
}

fn compare_friend_request_inventory_with_cursor(
    friend_request: &FriendRequest,
    cursor: &FriendRequestInventoryCursor,
) -> CmpOrdering {
    compare_friend_request_inventory_sort_key(
        friend_request.updated_at.as_str(),
        friend_request.created_at.as_str(),
        friend_request.request_id.as_str(),
        cursor.updated_at.as_str(),
        cursor.created_at.as_str(),
        cursor.request_id.as_str(),
    )
}

fn compare_friend_request_inventory_sort_key(
    left_updated_at: &str,
    left_created_at: &str,
    left_request_id: &str,
    right_updated_at: &str,
    right_created_at: &str,
    right_request_id: &str,
) -> CmpOrdering {
    right_updated_at
        .cmp(left_updated_at)
        .then_with(|| right_created_at.cmp(left_created_at))
        .then_with(|| left_request_id.cmp(right_request_id))
}

fn friend_request_inventory_cursor_for(
    friend_request: &FriendRequest,
) -> Result<String, SocialServiceError> {
    let cursor = FriendRequestInventoryCursor {
        v: FRIEND_REQUEST_CURSOR_VERSION,
        updated_at: friend_request.updated_at.clone(),
        created_at: friend_request.created_at.clone(),
        request_id: friend_request.request_id.clone(),
    };
    let payload = serde_json::to_value(&cursor).map_err(|_| {
        SocialServiceError::invalid(
            "cursor_encoding_failed",
            "friend request inventory cursor could not be encoded",
        )
    })?;
    let secret = resolve_friend_request_cursor_signing_secret()?;
    encode_signed_cursor_payload(&payload, secret.as_str())
}

fn encode_signed_cursor_payload(
    payload: &serde_json::Value,
    secret: &str,
) -> Result<String, SocialServiceError> {
    let header = serde_json::json!({
        "alg": "HS256",
        "typ": "cursor"
    });
    let header_bytes = serde_json::to_vec(&header).map_err(|_| {
        SocialServiceError::invalid(
            "cursor_encoding_failed",
            "cursor header could not be encoded",
        )
    })?;
    let payload_bytes = serde_json::to_vec(payload).map_err(|_| {
        SocialServiceError::invalid(
            "cursor_encoding_failed",
            "cursor payload could not be encoded",
        )
    })?;
    let header_segment = base64url_encode(&header_bytes);
    let payload_segment = base64url_encode(&payload_bytes);
    let signing_input = format!("{header_segment}.{payload_segment}");
    let signature_segment = hmac_sha256_base64url(signing_input.as_bytes(), secret.as_bytes());
    Ok(format!("{signing_input}.{signature_segment}"))
}

pub(crate) fn parse_friend_request_inventory_cursor(
    cursor: &str,
) -> Result<FriendRequestInventoryCursor, SocialServiceError> {
    let payload = decode_signed_friend_request_cursor_payload(cursor)?;
    let cursor: FriendRequestInventoryCursor = serde_json::from_value(payload).map_err(|_| {
        SocialServiceError::invalid(
            "cursor_invalid",
            "friend request cursor payload is not valid",
        )
    })?;
    if cursor.v != FRIEND_REQUEST_CURSOR_VERSION {
        return Err(SocialServiceError::invalid(
            "cursor_invalid",
            format!(
                "friend request cursor version {} is not supported",
                cursor.v
            ),
        ));
    }
    Ok(cursor)
}

fn decode_signed_friend_request_cursor_payload(
    cursor: &str,
) -> Result<serde_json::Value, SocialServiceError> {
    let segments = cursor.split('.').collect::<Vec<_>>();
    if segments.len() != 3 {
        return Err(SocialServiceError::invalid(
            "cursor_invalid",
            "friend request cursor must be a signed compact token",
        ));
    }
    let header_bytes = base64url_decode(segments[0]).ok_or_else(|| {
        SocialServiceError::invalid(
            "cursor_invalid",
            "friend request cursor header must be valid base64url",
        )
    })?;
    let header: serde_json::Value = serde_json::from_slice(&header_bytes).map_err(|_| {
        SocialServiceError::invalid(
            "cursor_invalid",
            "friend request cursor header must be valid json",
        )
    })?;
    let algorithm = header
        .get("alg")
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            SocialServiceError::invalid(
                "cursor_invalid",
                "friend request cursor algorithm must be HS256",
            )
        })?;
    if algorithm != "HS256" {
        return Err(SocialServiceError::invalid(
            "cursor_invalid",
            "friend request cursor algorithm must be HS256",
        ));
    }

    let signature = base64url_decode(segments[2]).ok_or_else(|| {
        SocialServiceError::invalid(
            "cursor_invalid",
            "friend request cursor signature must be valid base64url",
        )
    })?;
    let secret = resolve_friend_request_cursor_signing_secret()?;
    let signing_input = format!("{}.{}", segments[0], segments[1]);
    if !verify_hmac_sha256_base64url(signing_input.as_bytes(), secret.as_bytes(), &signature) {
        return Err(SocialServiceError::invalid(
            "cursor_invalid",
            "friend request cursor signature is invalid",
        ));
    }

    let payload_bytes = base64url_decode(segments[1]).ok_or_else(|| {
        SocialServiceError::invalid(
            "cursor_invalid",
            "friend request cursor payload must be valid base64url",
        )
    })?;
    serde_json::from_slice(&payload_bytes).map_err(|_| {
        SocialServiceError::invalid(
            "cursor_invalid",
            "friend request cursor payload must be valid json",
        )
    })
}

fn requires_configured_friend_request_cursor_signing_secret() -> bool {
    matches!(
        resolve_non_empty_env_secret(SDKWORK_IM_ENVIRONMENT_ENV).as_deref(),
        Some("production") | Some("staging")
    )
}

fn resolve_friend_request_cursor_signing_secret() -> Result<String, SocialServiceError> {
    if let Some(configured) = resolve_non_empty_env_secret(FRIEND_REQUEST_CURSOR_HS256_SECRET_ENV) {
        return Ok(configured);
    }

    if requires_configured_friend_request_cursor_signing_secret() {
        return Err(SocialServiceError::invalid(
            "cursor_signing_secret_required",
            format!(
                "{FRIEND_REQUEST_CURSOR_HS256_SECRET_ENV} is required when {SDKWORK_IM_ENVIRONMENT_ENV} is production or staging"
            ),
        ));
    }

    static EPHEMERAL_SECRET: OnceLock<String> = OnceLock::new();
    Ok(EPHEMERAL_SECRET
        .get_or_init(|| {
            let mut bytes = [0u8; 32];
            if fill_random(&mut bytes).is_ok() {
                tracing::warn!(
                    "{} is unset; using ephemeral in-memory friend request cursor signing secret for local development only",
                    FRIEND_REQUEST_CURSOR_HS256_SECRET_ENV
                );
                return base64url_encode(&bytes);
            }
            let fallback = format!(
                "ephemeral-friend-request-cursor-secret-{}",
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos()
            );
            tracing::warn!(
                "failed to generate random friend request cursor signing secret; using process-local time-derived fallback for local development only"
            );
            fallback
        })
        .clone())
}

fn resolve_non_empty_env_secret(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

// ---------------------------------------------------------------------------
// Business logic: SocialRuntime friendship methods
// ---------------------------------------------------------------------------

impl SocialRuntime {
    pub(crate) fn submit_friend_request(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request: SubmitFriendRequestRequest,
    ) -> Result<SubmittedFriendRequest, SocialServiceError> {
        validate_payload_size("requestId", request.request_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size("eventId", request.event_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size(
            "requesterUserId",
            request.requester_user_id.as_str(),
            MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "targetUserId",
            request.target_user_id.as_str(),
            MAX_ID_BYTES,
        )?;
        validate_optional_payload_size(
            "requestMessage",
            request.request_message.as_deref(),
            MAX_REQUEST_MESSAGE_BYTES,
        )?;
        validate_payload_size(
            "requestedAt",
            request.requested_at.as_str(),
            MAX_TIMESTAMP_BYTES,
        )?;
        validate_required("requestId", request.request_id.as_str())?;
        validate_required("eventId", request.event_id.as_str())?;
        validate_required("requesterUserId", request.requester_user_id.as_str())?;
        validate_required("targetUserId", request.target_user_id.as_str())?;
        validate_required("requestedAt", request.requested_at.as_str())?;
        normalize_user_pair(
            request.requester_user_id.as_str(),
            request.target_user_id.as_str(),
        )
        .map_err(|error| {
            SocialServiceError::invalid("invalid_friend_request", error.to_string())
        })?;

        let payload = FriendRequestSubmittedPayload {
            request_id: request.request_id.clone(),
            requester_user_id: request.requester_user_id.clone(),
            target_user_id: request.target_user_id.clone(),
            request_message: request.request_message.clone(),
            requested_at: request.requested_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("friend request payload should serialize into json");
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            organization_id: auth.organization_id.as_str(),
            aggregate_type: AggregateType::FriendRequest,
            aggregate_id: request.request_id.as_str(),
            event_type: SocialEventType::FriendRequestSubmitted,
            ordering_seq: 1,
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.requested_at.as_str(),
            committed_at: request.requested_at.as_str(),
            payload: payload_json.as_str(),
        });
        let friend_request = FriendRequest {
            tenant_id: tenant_id.into(),
            request_id: request.request_id.clone(),
            requester_user_id: request.requester_user_id,
            target_user_id: request.target_user_id,
            status: FriendRequestStatus::Pending,
            request_message: request.request_message,
            expired_at: None,
            created_at: request.requested_at.clone(),
            updated_at: request.requested_at,
        };

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let mut next_state = state.clone();
        if let Some(replayed) =
            self.replay_committed_social_event(&state, &commit, |existing, persistence| {
                match existing {
                    crate::runtime::SocialCommittedEvent::FriendRequest { record, commit } => {
                        Ok(SubmittedFriendRequest {
                            friend_request: record.friend_request,
                            latest_commit: commit,
                            persistence,
                        })
                    }
                    other => Err(social_event_id_conflict_string(
                        request.event_id.as_str(),
                        &other,
                    )),
                }
            })?
        {
            return Ok(replayed);
        }
        if next_state
            .friend_requests
            .contains_key(friend_request.request_id.as_str())
        {
            return Err(SocialServiceError::conflict_with_details(
                "friend_request_conflict",
                format!(
                    "friend request {} already exists",
                    friend_request.request_id
                ),
                serde_json::json!({
                    "existingRequestId": friend_request.request_id,
                    "existingStatus": FriendRequestStatus::Pending,
                    "existingRequesterUserId": friend_request.requester_user_id,
                    "existingTargetUserId": friend_request.target_user_id
                }),
            ));
        }
        let requested_pair = friend_request
            .user_pair()
            .expect("validated friend request should expose normalized user pair");
        if let Some(user_block) = active_friendship_scoped_user_block(
            &next_state,
            tenant_id,
            auth.organization_id.as_str(),
            friend_request.requester_user_id.as_str(),
            friend_request.target_user_id.as_str(),
        ) {
            return Err(SocialServiceError::conflict_with_details(
                "friend_request_blocked",
                format!(
                    "friend request pair {} is blocked by {}",
                    requested_pair.pair_key(),
                    user_block.block_id
                ),
                social_pair_block_conflict_details(&user_block),
            ));
        }
        if let Some(existing_friendship) = active_friendship_record_for_pair(
            &next_state,
            tenant_id,
            auth.organization_id.as_str(),
            requested_pair.user_low_id.as_str(),
            requested_pair.user_high_id.as_str(),
        ) {
            return Err(SocialServiceError::conflict_with_details(
                "friendship_pair_conflict",
                format!(
                    "active friendship already exists for pair {}",
                    requested_pair.pair_key()
                ),
                serde_json::json!({
                    "existingFriendshipId": existing_friendship.friendship.friendship_id,
                    "existingStatus": existing_friendship.friendship.status,
                    "userLowId": existing_friendship.friendship.user_low_id,
                    "userHighId": existing_friendship.friendship.user_high_id
                }),
            ));
        }
        let pair_has_materialized_friendship = friendship_pair_has_materialized_record(
            &next_state,
            tenant_id,
            auth.organization_id.as_str(),
            requested_pair.user_low_id.as_str(),
            requested_pair.user_high_id.as_str(),
        );
        if let Some(existing) = open_friend_request_record_for_pair(
            &next_state,
            tenant_id,
            auth.organization_id.as_str(),
            requested_pair.user_low_id.as_str(),
            requested_pair.user_high_id.as_str(),
            pair_has_materialized_friendship,
        ) {
            return Err(SocialServiceError::conflict_with_details(
                "friend_request_pair_conflict",
                format!(
                    "open friend request already exists for pair {}",
                    requested_pair.pair_key()
                ),
                serde_json::json!({
                    "existingRequestId": existing.friend_request.request_id,
                    "existingStatus": existing.friend_request.status,
                    "existingRequesterUserId": existing.friend_request.requester_user_id,
                    "existingTargetUserId": existing.friend_request.target_user_id
                }),
            ));
        }

        next_state.insert_friend_request_record(
            friend_request.request_id.clone(),
            StoredFriendRequest {
                friend_request: friend_request.clone(),
                commits: vec![commit.clone()],
            },
        );
        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(SubmittedFriendRequest {
            friend_request,
            latest_commit: commit,
            persistence,
        })
    }

    pub(crate) fn friend_request_snapshot(
        &self,
        tenant_id: &str,
        request_id: &str,
    ) -> Option<StoredFriendRequest> {
        self.state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .friend_requests
            .get(request_id)
            .filter(|record| record.friend_request.tenant_id == tenant_id)
            .cloned()
    }

    pub(crate) fn list_friend_requests(
        &self,
        tenant_id: &str,
        organization_id: &str,
        user_id: &str,
        direction: FriendRequestInventoryDirectionQuery,
        status: FriendRequestInventoryStatusQuery,
        limit: usize,
        cursor: Option<&FriendRequestInventoryCursor>,
    ) -> Result<FriendRequestInventoryPage, SocialServiceError> {
        let state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let mut items = friend_request_records_for_user(&state, tenant_id, organization_id, user_id)
            .into_iter()
            .filter(|record| {
                friend_request_matches_inventory_direction(
                    &record.friend_request,
                    user_id,
                    direction,
                )
            })
            .filter(|record| {
                friend_request_matches_inventory_status(&record.friend_request, status)
            })
            .map(|record| record.friend_request.clone())
            .collect::<Vec<_>>();
        items.sort_by(compare_friend_request_inventory_order);
        if let Some(cursor) = cursor {
            items.retain(|item| compare_friend_request_inventory_with_cursor(item, cursor).is_gt());
        }
        let next_cursor = if items.len() > limit {
            items
                .get(limit - 1)
                .map(friend_request_inventory_cursor_for)
                .transpose()?
        } else {
            None
        };
        items.truncate(limit);
        Ok(FriendRequestInventoryPage { items, next_cursor })
    }

    pub(crate) fn accept_friend_request(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request_id: &str,
        request: AcceptFriendRequestRequest,
    ) -> Result<AcceptedFriendRequest, SocialServiceError> {
        validate_payload_size("requestId", request_id, MAX_ID_BYTES)?;
        validate_payload_size("eventId", request.event_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size(
            "acceptedByUserId",
            request.accepted_by_user_id.as_str(),
            MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "acceptedAt",
            request.accepted_at.as_str(),
            MAX_TIMESTAMP_BYTES,
        )?;
        validate_required_with_code("requestId", request_id, "invalid_friend_request")?;
        validate_required_with_code(
            "eventId",
            request.event_id.as_str(),
            "invalid_friend_request",
        )?;
        validate_required_with_code(
            "acceptedByUserId",
            request.accepted_by_user_id.as_str(),
            "invalid_friend_request",
        )?;
        validate_required_with_code(
            "acceptedAt",
            request.accepted_at.as_str(),
            "invalid_friend_request",
        )?;

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let stored = state
            .friend_requests
            .get(request_id)
            .filter(|record| record.friend_request.tenant_id == tenant_id)
            .cloned()
            .ok_or_else(|| {
                SocialServiceError::not_found(
                    "friend_request_not_found",
                    format!("friend request {request_id} was not found"),
                )
            })?;
        let existing_committed_event = state.committed_event(tenant_id, request.event_id.as_str());
        let existing_ordering_seq = existing_committed_event
            .as_ref()
            .map(|existing| existing.commit().ordering_seq);
        if stored.friend_request.target_user_id != request.accepted_by_user_id {
            return Err(SocialServiceError::invalid(
                "invalid_friend_request",
                format!("acceptedByUserId must match target user for {request_id}"),
            ));
        }
        if !matches!(stored.friend_request.status, FriendRequestStatus::Pending)
            && existing_ordering_seq.is_none()
        {
            return Err(SocialServiceError::conflict(
                "friend_request_not_pending",
                format!("friend request {request_id} is not pending"),
            ));
        }

        let user_pair = stored
            .friend_request
            .user_pair()
            .expect("validated friend request should expose normalized user pair");
        let actor_pair = normalize_actor_pair(
            stored.friend_request.requester_user_id.as_str(),
            stored.friend_request.target_user_id.as_str(),
        )
        .expect("validated friend request participants should normalize into direct chat pair");
        let accepted_at = request.accepted_at.clone();
        let friendship_id = deterministic_social_id("fs_", request_id);
        let friendship_event_id = deterministic_social_id("evt_fs_activate_", request_id);
        let direct_chat_id = deterministic_social_id("dc_", request_id);
        let direct_chat_event_id = deterministic_social_id("evt_dc_bind_", request_id);
        let conversation_id = deterministic_social_id("c_direct_", request_id);
        let payload = FriendRequestAcceptedPayload {
            request_id: request_id.into(),
            accepted_by_user_id: request.accepted_by_user_id.clone(),
            accepted_at: accepted_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("friend request accept payload should serialize into json");
        let accept_commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            organization_id: auth.organization_id.as_str(),
            aggregate_type: AggregateType::FriendRequest,
            aggregate_id: request_id,
            event_type: SocialEventType::FriendRequestAccepted,
            ordering_seq: existing_ordering_seq.unwrap_or(stored.commits.len() as u64 + 1),
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: accepted_at.as_str(),
            committed_at: accepted_at.as_str(),
            payload: payload_json.as_str(),
        });
        let accept_commit_already_committed = if let Some(existing) = existing_committed_event {
            if existing.commit() != &accept_commit {
                return Err(social_event_id_conflict(
                    request.event_id.as_str(),
                    &existing,
                ));
            }
            true
        } else {
            false
        };
        if let Some(user_block) = active_friendship_scoped_user_block(
            &state,
            tenant_id,
            auth.organization_id.as_str(),
            stored.friend_request.requester_user_id.as_str(),
            stored.friend_request.target_user_id.as_str(),
        ) {
            let pair = stored
                .friend_request
                .user_pair()
                .expect("validated friend request should expose normalized user pair");
            return Err(SocialServiceError::conflict_with_details(
                "friend_request_blocked",
                format!(
                    "friend request pair {} is blocked by {}",
                    pair.pair_key(),
                    user_block.block_id
                ),
                social_pair_block_conflict_details(&user_block),
            ));
        }

        let mut next_state = state.clone();
        let mut commits_to_persist = Vec::new();
        let friend_request = if accept_commit_already_committed {
            next_state
                .friend_requests
                .get(request_id)
                .expect("friend request should exist after replay validation")
                .friend_request
                .clone()
        } else {
            let mut record = next_state
                .friend_requests
                .get(request_id)
                .cloned()
                .expect("friend request should exist after validation");
            record.friend_request.status = FriendRequestStatus::Accepted;
            record.friend_request.updated_at = accepted_at.clone();
            record.commits.push(accept_commit.clone());
            commits_to_persist.push(accept_commit.clone());
            let friend_request = record.friend_request.clone();
            next_state.insert_friend_request_record(request_id.to_owned(), record);
            friend_request
        };

        let existing_friendship = active_friendship_record_for_pair(
            &next_state,
            tenant_id,
            auth.organization_id.as_str(),
            &user_pair.user_low_id,
            &user_pair.user_high_id,
        );
        let existing_direct_chat = active_direct_chat_record_for_pair(
            &next_state,
            tenant_id,
            auth.organization_id.as_str(),
            actor_pair.left_actor_id.as_str(),
            actor_pair.right_actor_id.as_str(),
        );

        let planned_direct_chat_id = existing_direct_chat
            .as_ref()
            .map(|record| record.direct_chat.direct_chat_id.clone())
            .unwrap_or_else(|| direct_chat_id.clone());

        let (friendship, friendship_materialized_commit) = if let Some(record) = existing_friendship
        {
            (Some(record.friendship), None)
        } else {
            let friendship_payload = FriendshipActivatedPayload {
                friendship_id: friendship_id.clone(),
                user_low_id: user_pair.user_low_id.clone(),
                user_high_id: user_pair.user_high_id.clone(),
                initiator_user_id: stored.friend_request.requester_user_id.clone(),
                direct_chat_id: Some(planned_direct_chat_id.clone()),
                established_at: accepted_at.clone(),
            };
            let friendship_payload_json = serde_json::to_string(&friendship_payload)
                .expect("friendship payload should serialize into json");
            let friendship_commit = social_commit_envelope(SocialCommitEnvelopeInput {
                event_id: friendship_event_id.as_str(),
                tenant_id,
                organization_id: auth.organization_id.as_str(),
                aggregate_type: AggregateType::Friendship,
                aggregate_id: friendship_id.as_str(),
                event_type: SocialEventType::FriendshipActivated,
                ordering_seq: 1,
                actor: EventActor {
                    actor_id: auth.actor_id.clone(),
                    actor_kind: auth.actor_kind.clone(),
                    actor_session_id: auth.session_id.clone(),
                },
                occurred_at: accepted_at.as_str(),
                committed_at: accepted_at.as_str(),
                payload: friendship_payload_json.as_str(),
            });
            if let Some(existing) =
                next_state.committed_event(tenant_id, friendship_event_id.as_str())
            {
                if existing.commit() != &friendship_commit {
                    return Err(social_event_id_conflict(
                        friendship_event_id.as_str(),
                        &existing,
                    ));
                }
                match existing {
                    crate::runtime::SocialCommittedEvent::Friendship { record, .. }
                        if record.friendship.status.is_active() =>
                    {
                        (Some(record.friendship), None)
                    }
                    crate::runtime::SocialCommittedEvent::Friendship { .. } => (None, None),
                    other => {
                        return Err(social_event_id_conflict(
                            friendship_event_id.as_str(),
                            &other,
                        ));
                    }
                }
            } else {
                if next_state.friendships.contains_key(friendship_id.as_str()) {
                    return Err(SocialServiceError::conflict(
                        "friendship_conflict",
                        format!("friendship {friendship_id} already exists"),
                    ));
                }
                let friendship = Friendship {
                    tenant_id: tenant_id.into(),
                    friendship_id: friendship_id.clone(),
                    user_low_id: user_pair.user_low_id.clone(),
                    user_high_id: user_pair.user_high_id.clone(),
                    initiator_user_id: stored.friend_request.requester_user_id.clone(),
                    status: FriendshipStatus::Active,
                    established_at: Some(accepted_at.clone()),
                    updated_at: accepted_at.clone(),
                };
                next_state.insert_friendship_record(
                    friendship.friendship_id.clone(),
                    StoredFriendship {
                        friendship: friendship.clone(),
                        commits: vec![friendship_commit.clone()],
                    },
                );
                commits_to_persist.push(friendship_commit.clone());
                (Some(friendship), Some(friendship_commit))
            }
        };

        let (direct_chat, direct_chat_materialized_commit) =
            if let Some(record) = existing_direct_chat {
                (Some(record.direct_chat), None)
            } else {
                let direct_chat_payload = DirectChatBoundPayload {
                    direct_chat_id: direct_chat_id.clone(),
                    conversation_id: conversation_id.clone(),
                    left_actor_id: actor_pair.left_actor_id.clone(),
                    right_actor_id: actor_pair.right_actor_id.clone(),
                    pair_hash: actor_pair.pair_hash.clone(),
                    bound_at: accepted_at.clone(),
                };
                let direct_chat_payload_json = serde_json::to_string(&direct_chat_payload)
                    .expect("direct chat payload should serialize into json");
                let direct_chat_commit = social_commit_envelope(SocialCommitEnvelopeInput {
                    event_id: direct_chat_event_id.as_str(),
                    tenant_id,
                    organization_id: auth.organization_id.as_str(),
                    aggregate_type: AggregateType::DirectChat,
                    aggregate_id: direct_chat_id.as_str(),
                    event_type: SocialEventType::DirectChatBound,
                    ordering_seq: 1,
                    actor: EventActor {
                        actor_id: auth.actor_id.clone(),
                        actor_kind: auth.actor_kind.clone(),
                        actor_session_id: auth.session_id.clone(),
                    },
                    occurred_at: accepted_at.as_str(),
                    committed_at: accepted_at.as_str(),
                    payload: direct_chat_payload_json.as_str(),
                });
                if let Some(existing) =
                    next_state.committed_event(tenant_id, direct_chat_event_id.as_str())
                {
                    if existing.commit() != &direct_chat_commit {
                        return Err(social_event_id_conflict(
                            direct_chat_event_id.as_str(),
                            &existing,
                        ));
                    }
                    match existing {
                        crate::runtime::SocialCommittedEvent::DirectChat { record, .. }
                            if record.direct_chat.status.is_active() =>
                        {
                            (Some(record.direct_chat), None)
                        }
                        crate::runtime::SocialCommittedEvent::DirectChat { .. } => (None, None),
                        other => {
                            return Err(social_event_id_conflict(
                                direct_chat_event_id.as_str(),
                                &other,
                            ));
                        }
                    }
                } else {
                    if next_state
                        .direct_chats
                        .contains_key(direct_chat_id.as_str())
                    {
                        return Err(SocialServiceError::conflict(
                            "direct_chat_conflict",
                            format!("direct chat {direct_chat_id} already exists"),
                        ));
                    }
                    let direct_chat = DirectChat {
                        tenant_id: tenant_id.into(),
                        direct_chat_id: direct_chat_id.clone(),
                        left_actor_id: actor_pair.left_actor_id.clone(),
                        right_actor_id: actor_pair.right_actor_id.clone(),
                        pair_hash: actor_pair.pair_hash.clone(),
                        status: DirectChatStatus::Active,
                        conversation_id: Some(conversation_id.clone()),
                        created_at: accepted_at.clone(),
                        updated_at: accepted_at.clone(),
                    };
                    next_state.insert_direct_chat_record(
                        direct_chat.direct_chat_id.clone(),
                        StoredDirectChat {
                            direct_chat: direct_chat.clone(),
                            commits: vec![direct_chat_commit.clone()],
                        },
                    );
                    commits_to_persist.push(direct_chat_commit.clone());
                    (Some(direct_chat), Some(direct_chat_commit))
                }
            };

        let persistence = if commits_to_persist.is_empty() {
            self.repair_derived_snapshot_best_effort(&next_state)
        } else {
            self.persist_state_transition_batch(&next_state, commits_to_persist.as_slice())?
        };
        *state = next_state;

        Ok(AcceptedFriendRequest {
            friend_request,
            latest_commit: accept_commit,
            persistence,
            friendship,
            friendship_materialized_commit,
            direct_chat,
            direct_chat_materialized_commit,
        })
    }

    pub(crate) fn decline_friend_request(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request_id: &str,
        request: DeclineFriendRequestRequest,
    ) -> Result<DeclinedFriendRequest, SocialServiceError> {
        validate_payload_size("requestId", request_id, MAX_ID_BYTES)?;
        validate_payload_size("eventId", request.event_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size(
            "declinedByUserId",
            request.declined_by_user_id.as_str(),
            MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "declinedAt",
            request.declined_at.as_str(),
            MAX_TIMESTAMP_BYTES,
        )?;
        validate_required_with_code("requestId", request_id, "invalid_friend_request")?;
        validate_required_with_code(
            "eventId",
            request.event_id.as_str(),
            "invalid_friend_request",
        )?;
        validate_required_with_code(
            "declinedByUserId",
            request.declined_by_user_id.as_str(),
            "invalid_friend_request",
        )?;
        validate_required_with_code(
            "declinedAt",
            request.declined_at.as_str(),
            "invalid_friend_request",
        )?;

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let stored = state
            .friend_requests
            .get(request_id)
            .filter(|record| record.friend_request.tenant_id == tenant_id)
            .cloned()
            .ok_or_else(|| {
                SocialServiceError::not_found(
                    "friend_request_not_found",
                    format!("friend request {request_id} was not found"),
                )
            })?;
        let existing_ordering_seq = state
            .committed_event(tenant_id, request.event_id.as_str())
            .map(|existing| existing.commit().ordering_seq);
        if !matches!(stored.friend_request.status, FriendRequestStatus::Pending)
            && existing_ordering_seq.is_none()
        {
            return Err(SocialServiceError::conflict(
                "friend_request_not_pending",
                format!("friend request {request_id} is not pending"),
            ));
        }
        if stored.friend_request.target_user_id != request.declined_by_user_id {
            return Err(SocialServiceError::invalid(
                "invalid_friend_request",
                format!("declinedByUserId must match target user for {request_id}"),
            ));
        }

        let payload = FriendRequestDeclinedPayload {
            request_id: request_id.into(),
            declined_by_user_id: request.declined_by_user_id.clone(),
            declined_at: request.declined_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("friend request decline payload should serialize into json");
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            organization_id: auth.organization_id.as_str(),
            aggregate_type: AggregateType::FriendRequest,
            aggregate_id: request_id,
            event_type: SocialEventType::FriendRequestDeclined,
            ordering_seq: existing_ordering_seq.unwrap_or(stored.commits.len() as u64 + 1),
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.declined_at.as_str(),
            committed_at: request.declined_at.as_str(),
            payload: payload_json.as_str(),
        });
        if let Some(replayed) =
            self.replay_committed_social_event(&state, &commit, |existing, persistence| {
                match existing {
                    crate::runtime::SocialCommittedEvent::FriendRequest { record, commit } => {
                        Ok(DeclinedFriendRequest {
                            friend_request: record.friend_request,
                            latest_commit: commit,
                            persistence,
                        })
                    }
                    other => Err(social_event_id_conflict_string(
                        request.event_id.as_str(),
                        &other,
                    )),
                }
            })?
        {
            return Ok(replayed);
        }

        let mut next_state = state.clone();
        let mut record = next_state
            .friend_requests
            .get(request_id)
            .cloned()
            .expect("friend request should exist after validation");
        record.friend_request.status = FriendRequestStatus::Declined;
        record.friend_request.updated_at = request.declined_at;
        let friend_request = record.friend_request.clone();
        record.commits.push(commit.clone());
        next_state.insert_friend_request_record(request_id.to_owned(), record);

        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(DeclinedFriendRequest {
            friend_request,
            latest_commit: commit,
            persistence,
        })
    }

    pub(crate) fn cancel_friend_request(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request_id: &str,
        request: CancelFriendRequestRequest,
    ) -> Result<CanceledFriendRequest, SocialServiceError> {
        validate_payload_size("requestId", request_id, MAX_ID_BYTES)?;
        validate_payload_size("eventId", request.event_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size(
            "canceledByUserId",
            request.canceled_by_user_id.as_str(),
            MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "canceledAt",
            request.canceled_at.as_str(),
            MAX_TIMESTAMP_BYTES,
        )?;
        validate_required_with_code("requestId", request_id, "invalid_friend_request")?;
        validate_required_with_code(
            "eventId",
            request.event_id.as_str(),
            "invalid_friend_request",
        )?;
        validate_required_with_code(
            "canceledByUserId",
            request.canceled_by_user_id.as_str(),
            "invalid_friend_request",
        )?;
        validate_required_with_code(
            "canceledAt",
            request.canceled_at.as_str(),
            "invalid_friend_request",
        )?;

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let stored = state
            .friend_requests
            .get(request_id)
            .filter(|record| record.friend_request.tenant_id == tenant_id)
            .cloned()
            .ok_or_else(|| {
                SocialServiceError::not_found(
                    "friend_request_not_found",
                    format!("friend request {request_id} was not found"),
                )
            })?;
        let existing_ordering_seq = state
            .committed_event(tenant_id, request.event_id.as_str())
            .map(|existing| existing.commit().ordering_seq);
        if !matches!(stored.friend_request.status, FriendRequestStatus::Pending)
            && existing_ordering_seq.is_none()
        {
            return Err(SocialServiceError::conflict(
                "friend_request_not_pending",
                format!("friend request {request_id} is not pending"),
            ));
        }
        if stored.friend_request.requester_user_id != request.canceled_by_user_id {
            return Err(SocialServiceError::invalid(
                "invalid_friend_request",
                format!("canceledByUserId must match requester user for {request_id}"),
            ));
        }

        let payload = FriendRequestCanceledPayload {
            request_id: request_id.into(),
            canceled_by_user_id: request.canceled_by_user_id.clone(),
            canceled_at: request.canceled_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("friend request cancel payload should serialize into json");
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            organization_id: auth.organization_id.as_str(),
            aggregate_type: AggregateType::FriendRequest,
            aggregate_id: request_id,
            event_type: SocialEventType::FriendRequestCanceled,
            ordering_seq: existing_ordering_seq.unwrap_or(stored.commits.len() as u64 + 1),
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.canceled_at.as_str(),
            committed_at: request.canceled_at.as_str(),
            payload: payload_json.as_str(),
        });
        if let Some(replayed) =
            self.replay_committed_social_event(&state, &commit, |existing, persistence| {
                match existing {
                    crate::runtime::SocialCommittedEvent::FriendRequest { record, commit } => {
                        Ok(CanceledFriendRequest {
                            friend_request: record.friend_request,
                            latest_commit: commit,
                            persistence,
                        })
                    }
                    other => Err(social_event_id_conflict_string(
                        request.event_id.as_str(),
                        &other,
                    )),
                }
            })?
        {
            return Ok(replayed);
        }

        let mut next_state = state.clone();
        let mut record = next_state
            .friend_requests
            .get(request_id)
            .cloned()
            .expect("friend request should exist after validation");
        record.friend_request.status = FriendRequestStatus::Canceled;
        record.friend_request.updated_at = request.canceled_at;
        let friend_request = record.friend_request.clone();
        record.commits.push(commit.clone());
        next_state.insert_friend_request_record(request_id.to_owned(), record);

        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(CanceledFriendRequest {
            friend_request,
            latest_commit: commit,
            persistence,
        })
    }

    pub(crate) fn activate_friendship(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request: ActivateFriendshipRequest,
    ) -> Result<ActivatedFriendship, SocialServiceError> {
        validate_payload_size("friendshipId", request.friendship_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size("eventId", request.event_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size(
            "initiatorUserId",
            request.initiator_user_id.as_str(),
            MAX_ID_BYTES,
        )?;
        validate_payload_size("peerUserId", request.peer_user_id.as_str(), MAX_ID_BYTES)?;
        validate_optional_payload_size(
            "directChatId",
            request.direct_chat_id.as_deref(),
            MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "establishedAt",
            request.established_at.as_str(),
            MAX_TIMESTAMP_BYTES,
        )?;
        validate_required_with_code(
            "friendshipId",
            request.friendship_id.as_str(),
            "invalid_friendship",
        )?;
        validate_required_with_code("eventId", request.event_id.as_str(), "invalid_friendship")?;
        validate_required_with_code(
            "initiatorUserId",
            request.initiator_user_id.as_str(),
            "invalid_friendship",
        )?;
        validate_required_with_code(
            "peerUserId",
            request.peer_user_id.as_str(),
            "invalid_friendship",
        )?;
        validate_required_with_code(
            "establishedAt",
            request.established_at.as_str(),
            "invalid_friendship",
        )?;
        let pair = normalize_user_pair(
            request.initiator_user_id.as_str(),
            request.peer_user_id.as_str(),
        )
        .map_err(|error| SocialServiceError::invalid("invalid_friendship", error.to_string()))?;

        let payload = FriendshipActivatedPayload {
            friendship_id: request.friendship_id.clone(),
            user_low_id: pair.user_low_id.clone(),
            user_high_id: pair.user_high_id.clone(),
            initiator_user_id: request.initiator_user_id.clone(),
            direct_chat_id: request.direct_chat_id.clone(),
            established_at: request.established_at.clone(),
        };
        let payload_json =
            serde_json::to_string(&payload).expect("friendship payload should serialize into json");
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            organization_id: auth.organization_id.as_str(),
            aggregate_type: AggregateType::Friendship,
            aggregate_id: request.friendship_id.as_str(),
            event_type: SocialEventType::FriendshipActivated,
            ordering_seq: 1,
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.established_at.as_str(),
            committed_at: request.established_at.as_str(),
            payload: payload_json.as_str(),
        });
        let friendship = Friendship {
            tenant_id: tenant_id.into(),
            friendship_id: request.friendship_id.clone(),
            user_low_id: pair.user_low_id.clone(),
            user_high_id: pair.user_high_id.clone(),
            initiator_user_id: request.initiator_user_id,
            status: FriendshipStatus::Active,
            established_at: Some(request.established_at.clone()),
            updated_at: request.established_at,
        };

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let mut next_state = state.clone();
        if let Some(replayed) =
            self.replay_committed_social_event(&state, &commit, |existing, persistence| {
                match existing {
                    crate::runtime::SocialCommittedEvent::Friendship { record, commit } => {
                        Ok(ActivatedFriendship {
                            friendship: record.friendship,
                            latest_commit: commit,
                            persistence,
                        })
                    }
                    other => Err(social_event_id_conflict_string(
                        request.event_id.as_str(),
                        &other,
                    )),
                }
            })?
        {
            return Ok(replayed);
        }
        if next_state
            .friendships
            .contains_key(friendship.friendship_id.as_str())
        {
            return Err(SocialServiceError::conflict(
                "friendship_conflict",
                format!("friendship {} already exists", friendship.friendship_id),
            ));
        }
        if let Some(user_block) = active_friendship_scoped_user_block(
            &next_state,
            tenant_id,
            auth.organization_id.as_str(),
            friendship.user_low_id.as_str(),
            friendship.user_high_id.as_str(),
        ) {
            return Err(SocialServiceError::conflict_with_details(
                "friendship_blocked",
                format!(
                    "friendship pair {}:{} is blocked by {}",
                    pair.user_low_id, pair.user_high_id, user_block.block_id
                ),
                social_pair_block_conflict_details(&user_block),
            ));
        }
        if let Some(existing_friendship) = active_friendship_record_for_pair(
            &next_state,
            tenant_id,
            auth.organization_id.as_str(),
            pair.user_low_id.as_str(),
            pair.user_high_id.as_str(),
        ) {
            return Err(SocialServiceError::conflict_with_details(
                "friendship_pair_conflict",
                format!(
                    "active friendship already exists for pair {}:{}",
                    pair.user_low_id, pair.user_high_id
                ),
                serde_json::json!({
                    "existingFriendshipId": existing_friendship.friendship.friendship_id,
                    "existingStatus": existing_friendship.friendship.status,
                    "userLowId": existing_friendship.friendship.user_low_id,
                    "userHighId": existing_friendship.friendship.user_high_id
                }),
            ));
        }

        next_state.insert_friendship_record(
            friendship.friendship_id.clone(),
            StoredFriendship {
                friendship: friendship.clone(),
                commits: vec![commit.clone()],
            },
        );
        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(ActivatedFriendship {
            friendship,
            latest_commit: commit,
            persistence,
        })
    }

    pub(crate) fn friendship_snapshot(
        &self,
        tenant_id: &str,
        friendship_id: &str,
    ) -> Option<StoredFriendship> {
        self.state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .friendships
            .get(friendship_id)
            .filter(|record| record.friendship.tenant_id == tenant_id)
            .cloned()
    }

    pub(crate) fn remove_friendship(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        friendship_id: &str,
        request: RemoveFriendshipRequest,
    ) -> Result<RemovedFriendship, SocialServiceError> {
        validate_payload_size("friendshipId", friendship_id, MAX_ID_BYTES)?;
        validate_payload_size("eventId", request.event_id.as_str(), MAX_ID_BYTES)?;
        validate_payload_size(
            "removedByUserId",
            request.removed_by_user_id.as_str(),
            MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "removedAt",
            request.removed_at.as_str(),
            MAX_TIMESTAMP_BYTES,
        )?;
        validate_required_with_code("friendshipId", friendship_id, "invalid_friendship")?;
        validate_required_with_code("eventId", request.event_id.as_str(), "invalid_friendship")?;
        validate_required_with_code(
            "removedByUserId",
            request.removed_by_user_id.as_str(),
            "invalid_friendship",
        )?;
        validate_required_with_code(
            "removedAt",
            request.removed_at.as_str(),
            "invalid_friendship",
        )?;

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let stored = state
            .friendships
            .get(friendship_id)
            .filter(|record| record.friendship.tenant_id == tenant_id)
            .cloned()
            .ok_or_else(|| {
                SocialServiceError::not_found(
                    "friendship_not_found",
                    format!("friendship {friendship_id} was not found"),
                )
            })?;
        let existing_ordering_seq = state
            .committed_event(tenant_id, request.event_id.as_str())
            .map(|existing| existing.commit().ordering_seq);
        if !stored.friendship.status.is_active() && existing_ordering_seq.is_none() {
            return Err(SocialServiceError::conflict(
                "friendship_not_active",
                format!("friendship {friendship_id} is not active"),
            ));
        }
        if request.removed_by_user_id != stored.friendship.user_low_id
            && request.removed_by_user_id != stored.friendship.user_high_id
        {
            return Err(SocialServiceError::invalid(
                "invalid_friendship",
                format!("removedByUserId must be a friendship participant for {friendship_id}"),
            ));
        }

        let payload = FriendshipRemovedPayload {
            friendship_id: stored.friendship.friendship_id.clone(),
            user_low_id: stored.friendship.user_low_id.clone(),
            user_high_id: stored.friendship.user_high_id.clone(),
            removed_by_user_id: request.removed_by_user_id.clone(),
            removed_at: request.removed_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("friendship removal payload should serialize into json");
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            organization_id: auth.organization_id.as_str(),
            aggregate_type: AggregateType::Friendship,
            aggregate_id: friendship_id,
            event_type: SocialEventType::FriendshipRemoved,
            ordering_seq: existing_ordering_seq.unwrap_or(stored.commits.len() as u64 + 1),
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.removed_at.as_str(),
            committed_at: request.removed_at.as_str(),
            payload: payload_json.as_str(),
        });
        if let Some(replayed) =
            self.replay_committed_social_event(&state, &commit, |existing, persistence| {
                match existing {
                    crate::runtime::SocialCommittedEvent::Friendship { record, commit } => {
                        Ok(RemovedFriendship {
                            friendship: record.friendship,
                            latest_commit: commit,
                            persistence,
                        })
                    }
                    other => Err(social_event_id_conflict_string(
                        request.event_id.as_str(),
                        &other,
                    )),
                }
            })?
        {
            return Ok(replayed);
        }

        let mut next_state = state.clone();
        let mut record = next_state
            .friendships
            .get(friendship_id)
            .cloned()
            .expect("friendship should exist after validation");
        record.friendship.status = FriendshipStatus::Removed;
        record.friendship.updated_at = request.removed_at;
        record.commits.push(commit.clone());
        let friendship = record.friendship.clone();
        next_state.insert_friendship_record(friendship_id.to_owned(), record);
        archive_active_direct_chats_for_pair(
            &mut next_state,
            tenant_id,
            auth.organization_id.as_str(),
            friendship.user_low_id.as_str(),
            friendship.user_high_id.as_str(),
            friendship.updated_at.as_str(),
        );

        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(RemovedFriendship {
            friendship,
            latest_commit: commit,
            persistence,
        })
    }
}

// ---------------------------------------------------------------------------
// HTTP handler functions
// ---------------------------------------------------------------------------

pub(crate) async fn list_friend_requests(
    headers: HeaderMap,
    Query(query): Query<FriendRequestInventoryQuery>,
    State(state): State<AppState>,
) -> Result<Json<SocialFriendRequestInventoryResponse>, SocialServiceError> {
    validate_payload_size("userId", query.user_id.as_str(), MAX_ID_BYTES)?;
    validate_required_with_code(
        "userId",
        query.user_id.as_str(),
        "invalid_friend_request_query",
    )?;
    let limit = query.limit.unwrap_or(FRIEND_REQUEST_LIST_DEFAULT_LIMIT);
    if limit == 0 || limit > FRIEND_REQUEST_LIST_MAX_LIMIT {
        return Err(SocialServiceError::invalid(
            "limit_invalid",
            format!("limit must be between 1 and {FRIEND_REQUEST_LIST_MAX_LIMIT}"),
        ));
    }
    let cursor = if let Some(cursor) = query.cursor.as_deref() {
        validate_payload_size("cursor", cursor, FRIEND_REQUEST_LIST_MAX_CURSOR_BYTES)?;
        Some(parse_friend_request_inventory_cursor(cursor)?)
    } else {
        None
    };

    let auth = resolve_auth_from_headers(&headers)?;
    let tenant_id = auth.tenant_id.as_str();
    let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
    state
        .social_runtime
        .refresh_state_from_authority_for_write()?;
    let page = state.social_runtime.list_friend_requests(
        tenant_id,
        auth.organization_id.as_str(),
        query.user_id.as_str(),
        query.direction,
        query.status,
        limit,
        cursor.as_ref(),
    )?;

    Ok(Json(SocialFriendRequestInventoryResponse {
        status: SocialFriendRequestReadStatus::Inventory,
        items: page.items,
        next_cursor: page.next_cursor,
    }))
}

pub(crate) async fn submit_friend_request(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(wire): Json<SubmitFriendRequestWireRequest>,
) -> Result<Json<SocialFriendRequestCommitResponse>, SocialServiceError> {
    let auth = resolve_auth_from_headers(&headers)?;
    let tenant_id = auth.tenant_id.as_str();
    let request = SubmitFriendRequestRequest::from_wire(
        crate::openapi::next_open_api_id()?,
        wire,
    );

    let submitted = state
        .social_runtime
        .submit_friend_request(tenant_id, &auth, request)?;

    Ok(Json(SocialFriendRequestCommitResponse {
        status: SocialFriendRequestWriteStatus::Submitted,
        friend_request: submitted.friend_request,
        latest_commit: submitted.latest_commit.into(),
        persistence: submitted.persistence,
        friendship: None,
        friendship_latest_commit: None,
        direct_chat: None,
        direct_chat_latest_commit: None,
    }))
}

pub(crate) async fn accept_friend_request(
    Path(request_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AcceptFriendRequestRequest>,
) -> Result<Json<SocialFriendRequestCommitResponse>, SocialServiceError> {
    let auth = resolve_auth_from_headers(&headers)?;
    let tenant_id = auth.tenant_id.clone();

    let accepted = state.social_runtime.accept_friend_request(
        tenant_id.as_str(),
        &auth,
        request_id.as_str(),
        request,
    )?;

    Ok(Json(SocialFriendRequestCommitResponse {
        status: SocialFriendRequestWriteStatus::Accepted,
        friend_request: accepted.friend_request,
        latest_commit: accepted.latest_commit.into(),
        persistence: accepted.persistence,
        friendship: accepted.friendship,
        friendship_latest_commit: accepted.friendship_materialized_commit.map(Into::into),
        direct_chat: accepted.direct_chat,
        direct_chat_latest_commit: accepted.direct_chat_materialized_commit.map(Into::into),
    }))
}

pub(crate) async fn decline_friend_request(
    Path(request_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<DeclineFriendRequestRequest>,
) -> Result<Json<SocialFriendRequestCommitResponse>, SocialServiceError> {
    let auth = resolve_auth_from_headers(&headers)?;
    let tenant_id = auth.tenant_id.clone();

    let declined = state.social_runtime.decline_friend_request(
        tenant_id.as_str(),
        &auth,
        request_id.as_str(),
        request,
    )?;

    Ok(Json(SocialFriendRequestCommitResponse {
        status: SocialFriendRequestWriteStatus::Declined,
        friend_request: declined.friend_request,
        latest_commit: declined.latest_commit.into(),
        persistence: declined.persistence,
        friendship: None,
        friendship_latest_commit: None,
        direct_chat: None,
        direct_chat_latest_commit: None,
    }))
}

pub(crate) async fn cancel_friend_request(
    Path(request_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CancelFriendRequestRequest>,
) -> Result<Json<SocialFriendRequestCommitResponse>, SocialServiceError> {
    let auth = resolve_auth_from_headers(&headers)?;
    let tenant_id = auth.tenant_id.clone();

    let canceled = state.social_runtime.cancel_friend_request(
        tenant_id.as_str(),
        &auth,
        request_id.as_str(),
        request,
    )?;

    Ok(Json(SocialFriendRequestCommitResponse {
        status: SocialFriendRequestWriteStatus::Canceled,
        friend_request: canceled.friend_request,
        latest_commit: canceled.latest_commit.into(),
        persistence: canceled.persistence,
        friendship: None,
        friendship_latest_commit: None,
        direct_chat: None,
        direct_chat_latest_commit: None,
    }))
}

pub(crate) async fn friend_request_snapshot(
    Path(request_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialFriendRequestSnapshotResponse>, SocialServiceError> {
    let auth = resolve_auth_from_headers(&headers)?;
    let tenant_id = auth.tenant_id.as_str();

    let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
    state
        .social_runtime
        .refresh_state_from_authority_for_write()?;
    let snapshot = state
        .social_runtime
        .friend_request_snapshot(tenant_id, request_id.as_str())
        .ok_or_else(|| {
            SocialServiceError::not_found(
                "friend_request_not_found",
                format!("friend request {request_id} was not found"),
            )
        })?;

    Ok(Json(SocialFriendRequestSnapshotResponse {
        status: SocialFriendRequestReadStatus::Snapshot,
        friend_request: snapshot.friend_request,
        commits: snapshot.commits.into_iter().map(Into::into).collect(),
    }))
}

pub(crate) async fn activate_friendship(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<ActivateFriendshipRequest>,
) -> Result<Json<SocialFriendshipCommitResponse>, SocialServiceError> {
    let auth = resolve_auth_from_headers(&headers)?;
    let tenant_id = auth.tenant_id.as_str();

    let activated = state
        .social_runtime
        .activate_friendship(tenant_id, &auth, request)?;

    Ok(Json(SocialFriendshipCommitResponse {
        status: SocialFriendshipWriteStatus::Activated,
        friendship: activated.friendship,
        latest_commit: activated.latest_commit.into(),
        persistence: activated.persistence,
    }))
}

pub(crate) async fn remove_friendship(
    Path(friendship_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RemoveFriendshipRequest>,
) -> Result<Json<SocialFriendshipCommitResponse>, SocialServiceError> {
    let auth = resolve_auth_from_headers(&headers)?;
    let tenant_id = auth.tenant_id.clone();

    let removed = state.social_runtime.remove_friendship(
        tenant_id.as_str(),
        &auth,
        friendship_id.as_str(),
        request,
    )?;

    Ok(Json(SocialFriendshipCommitResponse {
        status: SocialFriendshipWriteStatus::Removed,
        friendship: removed.friendship,
        latest_commit: removed.latest_commit.into(),
        persistence: removed.persistence,
    }))
}

pub(crate) async fn friendship_snapshot(
    Path(friendship_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialFriendshipSnapshotResponse>, SocialServiceError> {
    let auth = resolve_auth_from_headers(&headers)?;
    let tenant_id = auth.tenant_id.as_str();

    let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
    state
        .social_runtime
        .refresh_state_from_authority_for_write()?;
    let snapshot = state
        .social_runtime
        .friendship_snapshot(tenant_id, friendship_id.as_str())
        .ok_or_else(|| {
            SocialServiceError::not_found(
                "friendship_not_found",
                format!("friendship {friendship_id} was not found"),
            )
        })?;

    Ok(Json(SocialFriendshipSnapshotResponse {
        status: SocialFriendshipReadStatus::Snapshot,
        friendship: snapshot.friendship,
        commits: snapshot.commits.into_iter().map(Into::into).collect(),
    }))
}

pub(crate) fn resolve_auth_from_headers(
    headers: &HeaderMap,
) -> Result<AppContext, SocialServiceError> {
    im_app_context::resolve_app_context(headers)
        .map_err(|error| SocialServiceError::invalid("unauthorized", error.message().to_owned()))
}
