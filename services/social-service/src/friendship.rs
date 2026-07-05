use std::cmp::Ordering as CmpOrdering;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::{Extension, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, response::Response};
use sdkwork_routes_web_framework_backend_api::response::ApiProblem;
use sdkwork_utils_rust::{
    base64url_decode, base64url_encode, cursor_list_page_data, hmac_sha256_base64url,
    verify_hmac_sha256_base64url,
};
use sdkwork_web_core::{
    problem_response, ProblemCorrelation, WebFrameworkError, WebFrameworkErrorKind,
    WebRequestContext,
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

use crate::api_payload::resource_item;
use crate::direct_chat_binder::BindDirectChatConversationInput;
use crate::runtime::{
    SocialRuntime, SocialWritePersistence, StoredDirectChat, StoredFriendRequest, StoredFriendship,
    accepted_friend_request_record_for_pair, active_direct_chat_record_for_pair,
    active_friendship_record_for_pair, active_friendship_records_for_user,
    active_friendship_scoped_user_block,
    active_friend_request_block_for_pair,
    archive_active_direct_chats_for_pair, deterministic_social_id,
    friend_request_records_for_user, friendship_pair_has_materialized_record,
    open_friend_request_record_for_pair, organization_id_from_commits,
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
const CONTROL_ACTIVATE_FRIENDSHIP_ENV: &str = "SDKWORK_IM_SOCIAL_CONTROL_ACTIVATE_FRIENDSHIP";

fn control_plane_activate_friendship_allowed(
    state: &crate::runtime::SocialControlState,
    tenant_id: &str,
    organization_id: &str,
    user_low_id: &str,
    user_high_id: &str,
) -> bool {
    if std::env::var(CONTROL_ACTIVATE_FRIENDSHIP_ENV)
        .ok()
        .map(|value| {
            let trimmed = value.trim();
            trimmed == "true" || trimmed == "1"
        })
        .unwrap_or(false)
    {
        return true;
    }
    accepted_friend_request_record_for_pair(
        state,
        tenant_id,
        organization_id,
        user_low_id,
        user_high_id,
    )
    .is_some()
}

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
    retry_after_seconds: Option<u64>,
}

impl SocialServiceError {
    pub(crate) fn invalid(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code,
            message: message.into(),
            details: None,
            retry_after_seconds: None,
        }
    }

    pub(crate) fn not_found(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code,
            message: message.into(),
            details: None,
            retry_after_seconds: None,
        }
    }

    pub(crate) fn conflict(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::CONFLICT,
            code,
            message: message.into(),
            details: None,
            retry_after_seconds: None,
        }
    }

    pub(crate) fn forbidden(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code,
            message: message.into(),
            details: None,
            retry_after_seconds: None,
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
            retry_after_seconds: None,
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
            retry_after_seconds: None,
        }
    }

    pub(crate) fn rate_limited(message: impl Into<String>, retry_after_seconds: u64) -> Self {
        Self {
            status: StatusCode::TOO_MANY_REQUESTS,
            code: "friend_request_rate_limited",
            message: message.into(),
            details: None,
            retry_after_seconds: Some(retry_after_seconds),
        }
    }

    pub(crate) fn dependency_unavailable(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code,
            message: message.into(),
            details: None,
            retry_after_seconds: None,
        }
    }

    fn from_string(error: String) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "social_service_error",
            message: error,
            details: None,
            retry_after_seconds: None,
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
        StatusCode::TOO_MANY_REQUESTS => WebFrameworkErrorKind::RateLimitExceeded,
        StatusCode::SERVICE_UNAVAILABLE => WebFrameworkErrorKind::DependencyUnavailable,
        _ => WebFrameworkErrorKind::InternalServerError,
    }
}

impl From<SocialServiceError> for ApiProblem {
    fn from(error: SocialServiceError) -> Self {
        let framework_error = WebFrameworkError {
            kind: social_service_error_kind(&error.status),
            message: error.message,
            retry_after_seconds: error.retry_after_seconds,
        };
        ApiProblem::from_web_framework(framework_error)
    }
}

impl IntoResponse for SocialServiceError {
    fn into_response(self) -> Response {
        let error = WebFrameworkError {
            kind: social_service_error_kind(&self.status),
            message: self.message,
            retry_after_seconds: self.retry_after_seconds,
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

pub(crate) const FRIENDSHIP_LIST_MAX_LIMIT: usize = 200;
const FRIENDSHIP_CURSOR_VERSION: u64 = 1;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FriendshipInventoryCursor {
    v: u64,
    updated_at: String,
    friendship_id: String,
}

#[derive(Debug)]
pub(crate) struct FriendshipInventoryPage {
    pub(crate) items: Vec<Friendship>,
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

pub(crate) fn ensure_auth_user_matches(
    auth: &AppContext,
    user_id: &str,
    field: &'static str,
) -> Result<(), SocialServiceError> {
    if auth.social_principal_user_id() == user_id {
        return Ok(());
    }
    Err(SocialServiceError::forbidden(
        "social_actor_mismatch",
        format!("{field} must match authenticated user"),
    ))
}

fn ensure_social_record_organization_scope(
    auth: &AppContext,
    commits: &[CommitEnvelope],
    not_found_code: &'static str,
) -> Result<(), SocialServiceError> {
    let record_org = organization_id_from_commits(commits);
    let auth_org =
        im_domain_events::normalize_commit_organization_id(auth.organization_id.as_str());
    if record_org != auth_org {
        return Err(SocialServiceError::not_found(
            not_found_code,
            "resource was not found in the current organization scope",
        ));
    }
    Ok(())
}

fn ensure_friend_request_not_expired(
    request_id: &str,
    stored: &StoredFriendRequest,
    existing_ordering_seq: Option<u64>,
) -> Result<(), SocialServiceError> {
    if !matches!(stored.friend_request.status, FriendRequestStatus::Pending)
        || existing_ordering_seq.is_some()
    {
        return Ok(());
    }
    if crate::friend_request_expiration::friend_request_is_expired(
        stored.friend_request.expired_at.as_deref(),
        stored.friend_request.created_at.as_str(),
    ) {
        return Err(SocialServiceError::conflict(
            "friend_request_expired",
            format!("friend request {request_id} has expired"),
        ));
    }
    Ok(())
}

fn ensure_friend_request_participant(
    auth: &AppContext,
    friend_request: &FriendRequest,
) -> Result<(), SocialServiceError> {
    let principal = auth.social_principal_user_id();
    if principal == friend_request.requester_user_id.as_str()
        || principal == friend_request.target_user_id.as_str()
    {
        return Ok(());
    }
    Err(SocialServiceError::forbidden(
        "social_actor_mismatch",
        "authenticated user must be a friend request participant",
    ))
}

fn ensure_friendship_participant(
    auth: &AppContext,
    friendship: &Friendship,
) -> Result<(), SocialServiceError> {
    let principal = auth.social_principal_user_id();
    if principal == friendship.user_low_id.as_str()
        || principal == friendship.user_high_id.as_str()
    {
        return Ok(());
    }
    Err(SocialServiceError::forbidden(
        "social_actor_mismatch",
        "authenticated user must be a friendship participant",
    ))
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

fn compare_friendship_inventory_order(left: &Friendship, right: &Friendship) -> CmpOrdering {
    compare_friendship_inventory_sort_key(
        left.updated_at.as_str(),
        left.friendship_id.as_str(),
        right.updated_at.as_str(),
        right.friendship_id.as_str(),
    )
}

fn compare_friendship_inventory_with_cursor(
    friendship: &Friendship,
    cursor: &FriendshipInventoryCursor,
) -> CmpOrdering {
    compare_friendship_inventory_sort_key(
        friendship.updated_at.as_str(),
        friendship.friendship_id.as_str(),
        cursor.updated_at.as_str(),
        cursor.friendship_id.as_str(),
    )
}

fn compare_friendship_inventory_sort_key(
    left_updated_at: &str,
    left_friendship_id: &str,
    right_updated_at: &str,
    right_friendship_id: &str,
) -> CmpOrdering {
    right_updated_at
        .cmp(left_updated_at)
        .then_with(|| left_friendship_id.cmp(right_friendship_id))
}

fn friendship_inventory_cursor_for(friendship: &Friendship) -> Result<String, SocialServiceError> {
    let cursor = FriendshipInventoryCursor {
        v: FRIENDSHIP_CURSOR_VERSION,
        updated_at: friendship.updated_at.clone(),
        friendship_id: friendship.friendship_id.clone(),
    };
    let payload = serde_json::to_value(&cursor).map_err(|_| {
        SocialServiceError::invalid(
            "cursor_encoding_failed",
            "friendship inventory cursor could not be encoded",
        )
    })?;
    let secret = resolve_friend_request_cursor_signing_secret()?;
    encode_signed_cursor_payload(&payload, secret.as_str())
}

pub(crate) fn parse_friendship_inventory_cursor(
    cursor: &str,
) -> Result<FriendshipInventoryCursor, SocialServiceError> {
    let payload = decode_signed_friend_request_cursor_payload(cursor)?;
    let cursor: FriendshipInventoryCursor = serde_json::from_value(payload).map_err(|_| {
        SocialServiceError::invalid("cursor_invalid", "friendship inventory cursor is invalid")
    })?;
    if cursor.v != FRIENDSHIP_CURSOR_VERSION {
        return Err(SocialServiceError::invalid(
            "cursor_invalid",
            "friendship inventory cursor version is unsupported",
        ));
    }
    Ok(cursor)
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
    im_app_context::is_production_like_im_environment()
}

fn resolve_friend_request_cursor_signing_secret() -> Result<String, SocialServiceError> {
    if let Some(configured) = resolve_non_empty_env_secret(FRIEND_REQUEST_CURSOR_HS256_SECRET_ENV) {
        return Ok(configured);
    }

    if requires_configured_friend_request_cursor_signing_secret() {
        return Err(SocialServiceError::invalid(
            "cursor_signing_secret_required",
            format!(
                "{FRIEND_REQUEST_CURSOR_HS256_SECRET_ENV} is required in production-like IM environments"
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

fn friendship_from_record(record: im_adapters_social_postgres::friendship_store::FriendshipRecord) -> Option<Friendship> {
    let status = match record.status.as_str() {
        "active" => FriendshipStatus::Active,
        "removed" => FriendshipStatus::Removed,
        other => {
            tracing::warn!(
                friendship_id = record.friendship_id,
                status = other,
                "unknown friendship status from postgres supplemental store; skipping record"
            );
            return None;
        }
    };
    Some(Friendship {
        tenant_id: record.tenant_id,
        friendship_id: record.friendship_id.to_string(),
        user_low_id: record.user_low_id,
        user_high_id: record.user_high_id,
        initiator_user_id: record.initiator_user_id,
        status,
        established_at: record.established_at,
        updated_at: record.updated_at,
    })
}

fn friend_request_from_record(
    record: im_adapters_social_postgres::friend_request_store::FriendRequestRecord,
) -> Option<FriendRequest> {
    let status = match record.status.as_str() {
        "pending" => FriendRequestStatus::Pending,
        "accepted" => FriendRequestStatus::Accepted,
        "declined" => FriendRequestStatus::Declined,
        "canceled" => FriendRequestStatus::Canceled,
        "expired" => FriendRequestStatus::Expired,
        other => {
            tracing::warn!(
                request_id = record.request_id,
                status = other,
                "unknown friend request status from postgres supplemental store; skipping record"
            );
            return None;
        }
    };
    Some(FriendRequest {
        tenant_id: record.tenant_id,
        request_id: record.request_id.to_string(),
        requester_user_id: record.requester_user_id,
        target_user_id: record.target_user_id,
        request_message: record.request_message,
        status,
        expired_at: record.expired_at,
        created_at: record.created_at,
        updated_at: record.updated_at,
    })
}

fn friend_request_inventory_status_filter(
    status: FriendRequestInventoryStatusQuery,
) -> Option<&'static str> {
    match status {
        FriendRequestInventoryStatusQuery::Pending => Some("pending"),
        FriendRequestInventoryStatusQuery::Accepted => Some("accepted"),
        FriendRequestInventoryStatusQuery::Declined => Some("declined"),
        FriendRequestInventoryStatusQuery::Canceled => Some("canceled"),
        FriendRequestInventoryStatusQuery::Expired => Some("expired"),
        FriendRequestInventoryStatusQuery::All => None,
    }
}

fn friend_request_inventory_direction(direction: FriendRequestInventoryDirectionQuery) -> &'static str {
    match direction {
        FriendRequestInventoryDirectionQuery::Incoming => "incoming",
        FriendRequestInventoryDirectionQuery::Outgoing => "outgoing",
    }
}

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
        ensure_auth_user_matches(auth, request.requester_user_id.as_str(), "requesterUserId")?;
        let postgres_rate_store = self.friend_request_rate_limit_store();
        crate::friend_request_rate_limit::check_friend_request_rate_allowed(
            tenant_id,
            auth.organization_id.as_str(),
            request.requester_user_id.as_str(),
            postgres_rate_store
                .as_ref()
                .map(|store| store.as_ref() as &dyn im_adapters_social_postgres::friend_request_store::FriendRequestStore),
        )
        .map_err(|error| {
            SocialServiceError::rate_limited(error.message, error.retry_after_seconds)
        })?;
        normalize_user_pair(
            request.requester_user_id.as_str(),
            request.target_user_id.as_str(),
        )
        .map_err(|error| {
            SocialServiceError::invalid("invalid_friend_request", error.to_string())
        })?;
        self.validate_friend_request_target(
            tenant_id,
            auth.organization_id.as_str(),
            request.target_user_id.as_str(),
        )?;

        let expires_at =
            crate::friend_request_expiration::resolve_friend_request_expires_at(
                request.requested_at.as_str(),
            );
        let payload = FriendRequestSubmittedPayload {
            request_id: request.request_id.clone(),
            requester_user_id: request.requester_user_id.clone(),
            target_user_id: request.target_user_id.clone(),
            request_message: request.request_message.clone(),
            requested_at: request.requested_at.clone(),
            expires_at: expires_at.clone(),
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
            expired_at: expires_at,
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
        if let Some(user_block) = active_friend_request_block_for_pair(
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
        crate::friend_request_rate_limit::record_friend_request_submitted(
            tenant_id,
            friend_request.requester_user_id.as_str(),
            self.friend_request_rate_limit_store().is_some(),
        );

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
        if let Some(store) = self.friend_request_rate_limit_store() {
            let fetch_limit = i64::try_from(limit.saturating_add(1)).unwrap_or(i64::MAX);
            let records = store
                .list_inventory(
                    tenant_id,
                    organization_id,
                    user_id,
                    friend_request_inventory_direction(direction),
                    friend_request_inventory_status_filter(status),
                    cursor.map(|value| value.updated_at.as_str()),
                    cursor.map(|value| value.created_at.as_str()),
                    cursor.and_then(|value| value.request_id.parse().ok()),
                    fetch_limit,
                )
                .map_err(|error| {
                    SocialServiceError::dependency_unavailable(
                        "friend_request_inventory_unavailable",
                        format!("postgres friend request inventory query failed: {error:?}"),
                    )
                })?;
            let mut items = records
                .into_iter()
                .filter_map(friend_request_from_record)
                .collect::<Vec<_>>();
            let next_cursor = if items.len() > limit {
                items
                    .get(limit - 1)
                    .map(friend_request_inventory_cursor_for)
                    .transpose()?
            } else {
                None
            };
            items.truncate(limit);
            return Ok(FriendRequestInventoryPage { items, next_cursor });
        }

        if crate::friend_request_rate_limit::is_production_like_environment() {
            return Err(SocialServiceError::dependency_unavailable(
                "friend_request_inventory_unavailable",
                "postgres friend request inventory is required in production-like environments",
            ));
        }

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

    pub(crate) fn list_friendships(
        &self,
        tenant_id: &str,
        organization_id: &str,
        user_id: &str,
        limit: usize,
        cursor: Option<&FriendshipInventoryCursor>,
    ) -> Result<FriendshipInventoryPage, SocialServiceError> {
        if let Some(store) = self.friendship_inventory_store() {
            let fetch_limit = i64::try_from(limit.saturating_add(1)).unwrap_or(i64::MAX);
            let cursor_updated_at = cursor.map(|value| value.updated_at.as_str());
            let cursor_friendship_id = cursor.and_then(|value| value.friendship_id.parse().ok());
            let records = store
                .list_by_user_inventory(
                    tenant_id,
                    organization_id,
                    user_id,
                    "active",
                    cursor_updated_at,
                    cursor_friendship_id,
                    fetch_limit,
                )
                .map_err(|error| {
                    SocialServiceError::dependency_unavailable(
                        "friendship_inventory_unavailable",
                        format!("postgres friendship inventory query failed: {error:?}"),
                    )
                })?;
            let mut items = records
                .into_iter()
                .filter_map(friendship_from_record)
                .collect::<Vec<_>>();
            let next_cursor = if items.len() > limit {
                items
                    .get(limit - 1)
                    .map(friendship_inventory_cursor_for)
                    .transpose()?
            } else {
                None
            };
            items.truncate(limit);
            return Ok(FriendshipInventoryPage { items, next_cursor });
        }

        if crate::friend_request_rate_limit::is_production_like_environment() {
            return Err(SocialServiceError::dependency_unavailable(
                "friendship_inventory_unavailable",
                "postgres friendship inventory is required in production-like environments",
            ));
        }

        let state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let mut items = active_friendship_records_for_user(
            &state,
            tenant_id,
            organization_id,
            user_id,
        )
        .into_iter()
        .map(|record| record.friendship)
        .collect::<Vec<_>>();
        items.sort_by(compare_friendship_inventory_order);
        if let Some(cursor) = cursor {
            items.retain(|item| compare_friendship_inventory_with_cursor(item, cursor).is_gt());
        }
        let next_cursor = if items.len() > limit {
            items
                .get(limit - 1)
                .map(friendship_inventory_cursor_for)
                .transpose()?
        } else {
            None
        };
        items.truncate(limit);
        Ok(FriendshipInventoryPage { items, next_cursor })
    }

    pub(crate) fn count_pending_incoming_friend_requests(
        &self,
        tenant_id: &str,
        organization_id: &str,
        user_id: &str,
    ) -> usize {
        if let Some(store) = self.friend_request_rate_limit_store() {
            match store.count_pending_incoming_by_target(tenant_id, organization_id, user_id) {
                Ok(count) if count >= 0 => return count as usize,
                Ok(_) => return 0,
                Err(error) => {
                    if crate::friend_request_rate_limit::is_production_like_environment() {
                        tracing::error!(
                            ?error,
                            "postgres pending friend request count failed; denying new friend requests (fail-closed)"
                        );
                        return usize::MAX;
                    }
                    tracing::warn!(
                        ?error,
                        "postgres pending friend request count failed; falling back to in-memory index (development/test only)"
                    );
                }
            }
        }
        let state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        friend_request_records_for_user(&state, tenant_id, organization_id, user_id)
            .into_iter()
            .filter(|record| {
                friend_request_matches_inventory_direction(
                    &record.friend_request,
                    user_id,
                    FriendRequestInventoryDirectionQuery::Incoming,
                )
            })
            .filter(|record| {
                friend_request_matches_inventory_status(
                    &record.friend_request,
                    FriendRequestInventoryStatusQuery::Pending,
                )
            })
            .filter(|record| {
                record
                    .commits
                    .first()
                    .map(|commit| commit.organization_id.as_str() == organization_id)
                    .unwrap_or(false)
            })
            .count()
    }

    fn replay_terminal_accepted_friend_request(
        &self,
        state: &crate::runtime::SocialControlState,
        tenant_id: &str,
        organization_id: &str,
        request_id: &str,
        stored: &crate::runtime::StoredFriendRequest,
    ) -> Result<AcceptedFriendRequest, SocialServiceError> {
        let accept_commit = stored
            .commits
            .iter()
            .find(|commit| commit.event_type == "friend_request.accepted")
            .cloned()
            .ok_or_else(|| {
                SocialServiceError::conflict(
                    "friend_request_not_pending",
                    format!("friend request {request_id} is accepted but missing accept commit"),
                )
            })?;
        let user_pair = stored
            .friend_request
            .user_pair()
            .map_err(|error| {
                SocialServiceError::conflict(
                    "friend_request_not_pending",
                    format!(
                        "friend request {request_id} is missing normalized user pair: {error}"
                    ),
                )
            })?;
        let actor_pair = normalize_actor_pair(
            stored.friend_request.requester_user_id.as_str(),
            stored.friend_request.target_user_id.as_str(),
        )
        .map_err(|error| {
            SocialServiceError::conflict(
                "friend_request_not_pending",
                format!(
                    "friend request {request_id} is missing normalized actor pair: {error}"
                ),
            )
        })?;
        let friendship = active_friendship_record_for_pair(
            state,
            tenant_id,
            organization_id,
            user_pair.user_low_id.as_str(),
            user_pair.user_high_id.as_str(),
        )
        .map(|record| record.friendship);
        let direct_chat = active_direct_chat_record_for_pair(
            state,
            tenant_id,
            organization_id,
            actor_pair.left_actor_id.as_str(),
            actor_pair.right_actor_id.as_str(),
        )
        .map(|record| record.direct_chat);
        Ok(AcceptedFriendRequest {
            friend_request: stored.friend_request.clone(),
            latest_commit: accept_commit,
            persistence: self.repair_derived_snapshot_best_effort(state),
            friendship,
            friendship_materialized_commit: None,
            direct_chat,
            direct_chat_materialized_commit: None,
        })
    }

    fn replay_terminal_declined_friend_request(
        &self,
        state: &crate::runtime::SocialControlState,
        request_id: &str,
        stored: &crate::runtime::StoredFriendRequest,
    ) -> Result<DeclinedFriendRequest, SocialServiceError> {
        let decline_commit = stored
            .commits
            .iter()
            .find(|commit| commit.event_type == "friend_request.declined")
            .cloned()
            .ok_or_else(|| {
                SocialServiceError::conflict(
                    "friend_request_not_pending",
                    format!("friend request {request_id} is declined but missing decline commit"),
                )
            })?;
        Ok(DeclinedFriendRequest {
            friend_request: stored.friend_request.clone(),
            latest_commit: decline_commit,
            persistence: self.repair_derived_snapshot_best_effort(state),
        })
    }

    fn replay_terminal_canceled_friend_request(
        &self,
        state: &crate::runtime::SocialControlState,
        request_id: &str,
        stored: &crate::runtime::StoredFriendRequest,
    ) -> Result<CanceledFriendRequest, SocialServiceError> {
        let cancel_commit = stored
            .commits
            .iter()
            .find(|commit| commit.event_type == "friend_request.canceled")
            .cloned()
            .ok_or_else(|| {
                SocialServiceError::conflict(
                    "friend_request_not_pending",
                    format!("friend request {request_id} is canceled but missing cancel commit"),
                )
            })?;
        Ok(CanceledFriendRequest {
            friend_request: stored.friend_request.clone(),
            latest_commit: cancel_commit,
            persistence: self.repair_derived_snapshot_best_effort(state),
        })
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
        ensure_social_record_organization_scope(
            auth,
            &stored.commits,
            "friend_request_not_found",
        )?;
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
        ensure_auth_user_matches(auth, request.accepted_by_user_id.as_str(), "acceptedByUserId")?;
        if !matches!(stored.friend_request.status, FriendRequestStatus::Pending)
            && existing_ordering_seq.is_none()
        {
            if matches!(stored.friend_request.status, FriendRequestStatus::Accepted)
                && stored.friend_request.target_user_id == request.accepted_by_user_id
            {
                return Ok(self.replay_terminal_accepted_friend_request(
                    &state,
                    tenant_id,
                    auth.organization_id.as_str(),
                    request_id,
                    &stored,
                )?);
            }
            return Err(SocialServiceError::conflict(
                "friend_request_not_pending",
                format!("friend request {request_id} is not pending"),
            ));
        }
        if matches!(stored.friend_request.status, FriendRequestStatus::Pending)
            && existing_ordering_seq.is_none()
        {
            ensure_friend_request_not_expired(request_id, &stored, existing_ordering_seq)?;
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
            requester_user_id: stored.friend_request.requester_user_id.clone(),
            target_user_id: stored.friend_request.target_user_id.clone(),
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
        if let Some(user_block) = active_friend_request_block_for_pair(
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
                self.bind_direct_chat_conversation_if_configured(BindDirectChatConversationInput {
                    tenant_id: tenant_id.to_owned(),
                    organization_id: auth.organization_id.clone(),
                    conversation_id: conversation_id.clone(),
                    direct_chat_id: planned_direct_chat_id.clone(),
                    left_actor_id: actor_pair.left_actor_id.clone(),
                    left_actor_kind: "user".to_owned(),
                    right_actor_id: actor_pair.right_actor_id.clone(),
                    right_actor_kind: "user".to_owned(),
                    bound_by: auth.actor_id.clone(),
                })?;

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
        ensure_social_record_organization_scope(
            auth,
            &stored.commits,
            "friend_request_not_found",
        )?;
        let existing_ordering_seq = state
            .committed_event(tenant_id, request.event_id.as_str())
            .map(|existing| existing.commit().ordering_seq);
        if !matches!(stored.friend_request.status, FriendRequestStatus::Pending)
            && existing_ordering_seq.is_none()
        {
            if matches!(stored.friend_request.status, FriendRequestStatus::Declined)
                && stored.friend_request.target_user_id == request.declined_by_user_id
            {
                return Ok(self.replay_terminal_declined_friend_request(
                    &state,
                    request_id,
                    &stored,
                )?);
            }
            return Err(SocialServiceError::conflict(
                "friend_request_not_pending",
                format!("friend request {request_id} is not pending"),
            ));
        }
        ensure_friend_request_not_expired(request_id, &stored, existing_ordering_seq)?;
        if stored.friend_request.target_user_id != request.declined_by_user_id {
            return Err(SocialServiceError::invalid(
                "invalid_friend_request",
                format!("declinedByUserId must match target user for {request_id}"),
            ));
        }
        ensure_auth_user_matches(auth, request.declined_by_user_id.as_str(), "declinedByUserId")?;

        let payload = FriendRequestDeclinedPayload {
            request_id: request_id.into(),
            requester_user_id: stored.friend_request.requester_user_id.clone(),
            target_user_id: stored.friend_request.target_user_id.clone(),
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
        ensure_social_record_organization_scope(
            auth,
            &stored.commits,
            "friend_request_not_found",
        )?;
        let existing_ordering_seq = state
            .committed_event(tenant_id, request.event_id.as_str())
            .map(|existing| existing.commit().ordering_seq);
        if !matches!(stored.friend_request.status, FriendRequestStatus::Pending)
            && existing_ordering_seq.is_none()
        {
            if matches!(stored.friend_request.status, FriendRequestStatus::Canceled)
                && stored.friend_request.requester_user_id == request.canceled_by_user_id
            {
                return Ok(self.replay_terminal_canceled_friend_request(
                    &state,
                    request_id,
                    &stored,
                )?);
            }
            return Err(SocialServiceError::conflict(
                "friend_request_not_pending",
                format!("friend request {request_id} is not pending"),
            ));
        }
        ensure_friend_request_not_expired(request_id, &stored, existing_ordering_seq)?;
        if stored.friend_request.requester_user_id != request.canceled_by_user_id {
            return Err(SocialServiceError::invalid(
                "invalid_friend_request",
                format!("canceledByUserId must match requester user for {request_id}"),
            ));
        }
        ensure_auth_user_matches(auth, request.canceled_by_user_id.as_str(), "canceledByUserId")?;

        let payload = FriendRequestCanceledPayload {
            request_id: request_id.into(),
            requester_user_id: stored.friend_request.requester_user_id.clone(),
            target_user_id: stored.friend_request.target_user_id.clone(),
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
        ensure_auth_user_matches(auth, request.initiator_user_id.as_str(), "initiatorUserId")?;
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
        if !control_plane_activate_friendship_allowed(
            &next_state,
            tenant_id,
            auth.organization_id.as_str(),
            pair.user_low_id.as_str(),
            pair.user_high_id.as_str(),
        ) {
            return Err(SocialServiceError::forbidden(
                "activate_friendship_forbidden",
                "activate_friendship requires SDKWORK_IM_SOCIAL_CONTROL_ACTIVATE_FRIENDSHIP=true or an accepted friend request for the pair",
            ));
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
        ensure_social_record_organization_scope(
            auth,
            &stored.commits,
            "friendship_not_found",
        )?;
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
        ensure_auth_user_matches(auth, request.removed_by_user_id.as_str(), "removedByUserId")?;

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
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Query(query): Query<FriendRequestInventoryQuery>,
    State(state): State<AppState>,
) -> Response {
    let result = (|| {
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

        ensure_auth_user_matches(&auth, query.user_id.as_str(), "userId")?;
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
        let has_more = page.next_cursor.is_some();
        Ok(cursor_list_page_data(
            page.items,
            limit,
            page.next_cursor,
            has_more,
        ))
    })();
    crate::envelope::finish_enveloped_json(&ctx, result)
}

pub(crate) async fn submit_friend_request(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(wire): Json<SubmitFriendRequestWireRequest>,
) -> Response {
    let result = (|| {
        let tenant_id = auth.tenant_id.as_str();
        let request = SubmitFriendRequestRequest::from_wire(
            crate::openapi::next_open_api_id()?,
            wire,
        );

        let submitted = state
            .social_runtime
            .submit_friend_request(tenant_id, &auth, request)?;

        Ok(resource_item(SocialFriendRequestCommitResponse {
            status: SocialFriendRequestWriteStatus::Submitted,
            friend_request: submitted.friend_request,
            latest_commit: submitted.latest_commit.into(),
            persistence: submitted.persistence,
            friendship: None,
            friendship_latest_commit: None,
            direct_chat: None,
            direct_chat_latest_commit: None,
        }))
    })();
    crate::envelope::finish_enveloped_json(&ctx, result)
}

pub(crate) async fn accept_friend_request(
    Path(request_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<AcceptFriendRequestRequest>,
) -> Response {
    let result = (|| {
        let tenant_id = auth.tenant_id.clone();

        let accepted = state.social_runtime.accept_friend_request(
            tenant_id.as_str(),
            &auth,
            request_id.as_str(),
            request,
        )?;

        Ok(resource_item(SocialFriendRequestCommitResponse {
            status: SocialFriendRequestWriteStatus::Accepted,
            friend_request: accepted.friend_request,
            latest_commit: accepted.latest_commit.into(),
            persistence: accepted.persistence,
            friendship: accepted.friendship,
            friendship_latest_commit: accepted.friendship_materialized_commit.map(Into::into),
            direct_chat: accepted.direct_chat,
            direct_chat_latest_commit: accepted.direct_chat_materialized_commit.map(Into::into),
        }))
    })();
    crate::envelope::finish_enveloped_json(&ctx, result)
}

pub(crate) async fn decline_friend_request(
    Path(request_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<DeclineFriendRequestRequest>,
) -> Response {
    let result = (|| {
        let tenant_id = auth.tenant_id.clone();

        let declined = state.social_runtime.decline_friend_request(
            tenant_id.as_str(),
            &auth,
            request_id.as_str(),
            request,
        )?;

        Ok(resource_item(SocialFriendRequestCommitResponse {
            status: SocialFriendRequestWriteStatus::Declined,
            friend_request: declined.friend_request,
            latest_commit: declined.latest_commit.into(),
            persistence: declined.persistence,
            friendship: None,
            friendship_latest_commit: None,
            direct_chat: None,
            direct_chat_latest_commit: None,
        }))
    })();
    crate::envelope::finish_enveloped_json(&ctx, result)
}

pub(crate) async fn cancel_friend_request(
    Path(request_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<CancelFriendRequestRequest>,
) -> Response {
    let result = (|| {
        let tenant_id = auth.tenant_id.clone();

        let canceled = state.social_runtime.cancel_friend_request(
            tenant_id.as_str(),
            &auth,
            request_id.as_str(),
            request,
        )?;

        Ok(resource_item(SocialFriendRequestCommitResponse {
            status: SocialFriendRequestWriteStatus::Canceled,
            friend_request: canceled.friend_request,
            latest_commit: canceled.latest_commit.into(),
            persistence: canceled.persistence,
            friendship: None,
            friendship_latest_commit: None,
            direct_chat: None,
            direct_chat_latest_commit: None,
        }))
    })();
    crate::envelope::finish_enveloped_json(&ctx, result)
}

pub(crate) async fn friend_request_snapshot(
    Path(request_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = (|| {
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
        ensure_friend_request_participant(&auth, &snapshot.friend_request)?;

        Ok(resource_item(SocialFriendRequestSnapshotResponse {
            status: SocialFriendRequestReadStatus::Snapshot,
            friend_request: snapshot.friend_request,
            commits: snapshot.commits.into_iter().map(Into::into).collect(),
        }))
    })();
    crate::envelope::finish_enveloped_json(&ctx, result)
}

pub(crate) async fn activate_friendship(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<ActivateFriendshipRequest>,
) -> Response {
    let result = (|| {
        let tenant_id = auth.tenant_id.as_str();

        let activated = state
            .social_runtime
            .activate_friendship(tenant_id, &auth, request)?;

        Ok(resource_item(SocialFriendshipCommitResponse {
            status: SocialFriendshipWriteStatus::Activated,
            friendship: activated.friendship,
            latest_commit: activated.latest_commit.into(),
            persistence: activated.persistence,
        }))
    })();
    crate::envelope::finish_enveloped_json(&ctx, result)
}

pub(crate) async fn remove_friendship(
    Path(friendship_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<RemoveFriendshipRequest>,
) -> Response {
    let result = (|| {
        let tenant_id = auth.tenant_id.clone();

        let removed = state.social_runtime.remove_friendship(
            tenant_id.as_str(),
            &auth,
            friendship_id.as_str(),
            request,
        )?;

        Ok(resource_item(SocialFriendshipCommitResponse {
            status: SocialFriendshipWriteStatus::Removed,
            friendship: removed.friendship,
            latest_commit: removed.latest_commit.into(),
            persistence: removed.persistence,
        }))
    })();
    crate::envelope::finish_enveloped_json(&ctx, result)
}

pub(crate) async fn friendship_snapshot(
    Path(friendship_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = (|| {
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
        ensure_friendship_participant(&auth, &snapshot.friendship)?;

        Ok(resource_item(SocialFriendshipSnapshotResponse {
            status: SocialFriendshipReadStatus::Snapshot,
            friendship: snapshot.friendship,
            commits: snapshot.commits.into_iter().map(Into::into).collect(),
        }))
    })();
    crate::envelope::finish_enveloped_json(&ctx, result)
}

#[cfg(test)]
mod control_plane_policy_tests {
    use super::control_plane_activate_friendship_allowed;
    use crate::runtime::SocialControlState;
    use im_domain_core::social::{FriendRequest, FriendRequestStatus};

    #[test]
    fn activate_friendship_requires_control_flag_or_accepted_request() {
        let state = SocialControlState::default();
        assert!(!control_plane_activate_friendship_allowed(
            &state,
            "100001",
            "default",
            "1",
            "2",
        ));
    }

    #[test]
    fn activate_friendship_allows_accepted_request_evidence() {
        let mut state = SocialControlState::default();
        let friend_request = FriendRequest {
            tenant_id: "100001".into(),
            request_id: "fr_1".into(),
            requester_user_id: "1".into(),
            target_user_id: "2".into(),
            status: FriendRequestStatus::Accepted,
            request_message: None,
            expired_at: None,
            created_at: "2026-01-01T00:00:00Z".into(),
            updated_at: "2026-01-01T00:00:00Z".into(),
        };
        state.insert_friend_request_record(
            "fr_1".into(),
            crate::runtime::StoredFriendRequest {
                friend_request,
                commits: Vec::new(),
            },
        );
        assert!(control_plane_activate_friendship_allowed(
            &state,
            "100001",
            "default",
            "1",
            "2",
        ));
    }
}

#[cfg(test)]
mod friendship_lifecycle_tests {
    use std::sync::Arc;

    use im_app_context::local_service_app_context;
    use im_domain_core::social::FriendshipStatus;

    use crate::runtime::SocialRuntime;
    use crate::user_directory::PermissiveSocialUserDirectory;

    use super::{
        AcceptFriendRequestRequest, RemoveFriendshipRequest, SubmitFriendRequestRequest,
    };

    fn auth_for(user_id: &str) -> im_app_context::AppContext {
        let mut auth = local_service_app_context("100001", user_id, "user", None, ["*"]);
        auth.organization_id = "0".into();
        auth
    }

    fn sample_submit_request(
        request_id: &str,
        requester: &str,
        target: &str,
    ) -> SubmitFriendRequestRequest {
        SubmitFriendRequestRequest {
            request_id: request_id.into(),
            event_id: format!("evt_{request_id}"),
            requester_user_id: requester.into(),
            target_user_id: target.into(),
            request_message: None,
            requested_at: "2026-07-05T00:00:00.000Z".into(),
        }
    }

    #[test]
    fn friend_request_submit_accept_list_and_remove_lifecycle() {
        let _env_guard = crate::friend_request_rate_limit::social_service_test_env_lock();
        crate::friend_request_rate_limit::reset_friend_request_rate_limiter_for_tests();
        let previous_im_env = std::env::var("SDKWORK_IM_ENVIRONMENT").ok();
        unsafe {
            std::env::set_var("SDKWORK_IM_ENVIRONMENT", "test");
        }

        struct RestoreImEnv(Option<String>);
        impl Drop for RestoreImEnv {
            fn drop(&mut self) {
                unsafe {
                    match self.0.as_ref() {
                        Some(value) => std::env::set_var("SDKWORK_IM_ENVIRONMENT", value),
                        None => std::env::remove_var("SDKWORK_IM_ENVIRONMENT"),
                    }
                }
            }
        }
        let _restore_im_env = RestoreImEnv(previous_im_env);

        let runtime = SocialRuntime::default()
            .with_user_directory(Arc::new(PermissiveSocialUserDirectory));
        runtime.set_realtime_fanout(Arc::new(crate::LoggingSocialRealtimeFanout));
        let tenant_id = "100001";
        let organization_id = "0";
        let requester_auth = auth_for("user_a");
        let target_auth = auth_for("user_b");

        runtime
            .submit_friend_request(
                tenant_id,
                &requester_auth,
                sample_submit_request("fr_lifecycle_1", "user_a", "user_b"),
            )
            .expect("submit friend request");

        runtime
            .accept_friend_request(
                tenant_id,
                &target_auth,
                "fr_lifecycle_1",
                AcceptFriendRequestRequest {
                    event_id: "evt_accept_fr_lifecycle_1".into(),
                    accepted_by_user_id: "user_b".into(),
                    accepted_at: "2026-07-05T00:00:01.000Z".into(),
                },
            )
            .expect("accept friend request");

        let friendships = runtime
            .list_friendships(
                tenant_id,
                organization_id,
                requester_auth.actor_id.as_str(),
                20,
                None,
            )
            .expect("list friendships");
        assert!(
            friendships
                .items
                .iter()
                .any(|item| item.status == FriendshipStatus::Active),
            "accepted friendship should appear in list"
        );

        let friendship_id = friendships
            .items
            .first()
            .map(|item| item.friendship_id.clone())
            .expect("friendship id");

        runtime
            .remove_friendship(
                tenant_id,
                &requester_auth,
                friendship_id.as_str(),
                RemoveFriendshipRequest {
                    event_id: "evt_remove_fr_lifecycle_1".into(),
                    removed_by_user_id: "user_a".into(),
                    removed_at: "2026-07-05T00:00:02.000Z".into(),
                },
            )
            .expect("remove friendship");

        let after_remove = runtime
            .list_friendships(
                tenant_id,
                organization_id,
                requester_auth.actor_id.as_str(),
                20,
                None,
            )
            .expect("list friendships after remove");
        assert!(
            after_remove
                .items
                .iter()
                .all(|item| item.status != FriendshipStatus::Active),
            "removed friendship should no longer be active"
        );
    }
}