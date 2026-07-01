use std::collections::BTreeMap;
use std::fs;
use std::path::Path as FsPath;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::rejection::JsonRejection;
use axum::extract::{DefaultBodyLimit, Extension, FromRequest, Path, Query, State};
use axum::http::{HeaderMap, Request};
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use im_app_context::{AppContext, resolve_app_context};
use im_domain_core::conversation::{
    ConversationMember, ConversationReadCursorView, MembershipRole,
};
use im_domain_core::message::{ContentPart, MessageBody, MessageType};
use sdkwork_im_api_registry::HttpMethod;
use sdkwork_im_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};
use sdkwork_im_web_bootstrap::{
    im_service_router_config, mount_im_infra_routes,
};
use sdkwork_routes_web_framework_backend_api::response::{ApiProblem, ApiResult, finish_api_json};
use sdkwork_web_core::{
    ProblemCorrelation, WebFrameworkError, WebFrameworkErrorKind, WebRequestContext,
    problem_response,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Semaphore;

use super::*;

const CONVERSATION_RUNTIME_MAX_IN_FLIGHT_REQUESTS_ENV: &str =
    "SDKWORK_IM_CONVERSATION_RUNTIME_MAX_IN_FLIGHT_REQUESTS";
const CONVERSATION_RUNTIME_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 1_000;
const CONVERSATION_RUNTIME_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 50_000;
const CONVERSATION_RUNTIME_MAX_REQUEST_BODY_BYTES_ENV: &str =
    "SDKWORK_IM_CONVERSATION_RUNTIME_MAX_REQUEST_BODY_BYTES";
const CONVERSATION_RUNTIME_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
const CONVERSATION_RUNTIME_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;
pub const PRINCIPAL_DIRECTORY_CATALOG_PATH_ENV: &str = "SDKWORK_IM_PRINCIPAL_DIRECTORY_CATALOG_PATH";
pub const ALLOW_ALL_PRINCIPALS_ENV: &str = "SDKWORK_IM_ALLOW_ALL_PRINCIPALS";

#[derive(Clone)]
pub struct AppState {
    runtime: Arc<ConversationRuntime<ConversationCommitJournal>>,
    principal_directory: Arc<dyn PrincipalDirectory>,
    shared_channel_sync_rate_limiter: SharedChannelSyncRateLimiter,
}

impl AppState {
    pub(crate) fn rpc_runtime(&self) -> &ConversationRuntime<ConversationCommitJournal> {
        self.runtime.as_ref()
    }
}

#[derive(Clone)]
struct PublicAppGuardrails {
    request_gate: Arc<Semaphore>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrincipalDirectoryError {
    PrincipalNotFound {
        tenant_id: String,
        principal_id: String,
        principal_kind: String,
    },
    PrincipalDisabled {
        tenant_id: String,
        principal_id: String,
        principal_kind: String,
    },
    Unavailable(String),
}

pub trait PrincipalDirectory: Send + Sync {
    fn ensure_active_principal(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Result<(), PrincipalDirectoryError>;
}

#[derive(Default)]
struct AllowAllPrincipalDirectory;

impl PrincipalDirectory for AllowAllPrincipalDirectory {
    fn ensure_active_principal(
        &self,
        _tenant_id: &str,
        _principal_id: &str,
        _principal_kind: &str,
    ) -> Result<(), PrincipalDirectoryError> {
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct StaticPrincipalDirectory {
    principals: BTreeMap<(String, String, String), StaticPrincipalDirectoryRecord>,
}

#[derive(Clone, Debug)]
struct StaticPrincipalDirectoryRecord {
    disabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StaticPrincipalDirectoryCatalog {
    #[serde(default)]
    principals: Vec<StaticPrincipalDirectoryEntry>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StaticPrincipalDirectoryEntry {
    tenant_id: String,
    principal_id: String,
    principal_kind: String,
    #[serde(default)]
    disabled: bool,
}

impl StaticPrincipalDirectory {
    pub fn from_json_file(path: &FsPath) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|error| {
            format!(
                "principal directory catalog unreadable: {} ({error})",
                path.display()
            )
        })?;
        let catalog: StaticPrincipalDirectoryCatalog =
            serde_json::from_str(&content).map_err(|error| {
                format!(
                    "principal directory catalog invalid json: {} ({error})",
                    path.display()
                )
            })?;
        let mut principals = BTreeMap::new();
        for entry in catalog.principals {
            if entry.tenant_id.trim().is_empty() {
                return Err("principal directory catalog contains empty tenantId".into());
            }
            if entry.principal_id.trim().is_empty() {
                return Err("principal directory catalog contains empty principalId".into());
            }
            if entry.principal_kind.trim().is_empty() {
                return Err("principal directory catalog contains empty principalKind".into());
            }
            principals.insert(
                (entry.tenant_id, entry.principal_kind, entry.principal_id),
                StaticPrincipalDirectoryRecord {
                    disabled: entry.disabled,
                },
            );
        }
        Ok(Self { principals })
    }
}

impl PrincipalDirectory for StaticPrincipalDirectory {
    fn ensure_active_principal(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Result<(), PrincipalDirectoryError> {
        if principal_kind != "user" {
            return Ok(());
        }

        match self.principals.get(&(
            tenant_id.to_owned(),
            principal_kind.to_owned(),
            principal_id.to_owned(),
        )) {
            Some(record) if record.disabled => Err(PrincipalDirectoryError::PrincipalDisabled {
                tenant_id: tenant_id.into(),
                principal_id: principal_id.into(),
                principal_kind: principal_kind.into(),
            }),
            Some(_) => Ok(()),
            None => Err(PrincipalDirectoryError::PrincipalNotFound {
                tenant_id: tenant_id.into(),
                principal_id: principal_id.into(),
                principal_kind: principal_kind.into(),
            }),
        }
    }
}

const SHARED_CHANNEL_SYNC_PERMISSION: &str = "conversation.shared_channel.sync";
const SHARED_CHANNEL_SYNC_ACTOR_ID: &str = "control-plane-sync";
const SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS_ENV: &str =
    "SDKWORK_IM_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS";
const SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS_ENV: &str =
    "SDKWORK_IM_SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS";
const SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_BUCKETS_ENV: &str =
    "SDKWORK_IM_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_BUCKETS";
const SHARED_CHANNEL_SYNC_RATE_LIMIT_DEFAULT_MAX_REQUESTS: u32 = 120;
const SHARED_CHANNEL_SYNC_RATE_LIMIT_DEFAULT_WINDOW_SECONDS: u64 = 60;
const SHARED_CHANNEL_SYNC_RATE_LIMIT_DEFAULT_MAX_BUCKETS: usize = 10_000;
const SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_MAX_REQUESTS: u32 = 10_000;
const SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_WINDOW_SECONDS: u64 = 3_600;
const SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_BUCKETS: usize = 200_000;
const SHARED_CHANNEL_SYNC_RATE_LIMIT_SWEEP_THRESHOLD: usize = 1024;
#[derive(Clone)]
struct SharedChannelSyncRateLimiter {
    max_requests: u32,
    window_millis: u128,
    max_buckets: usize,
    buckets: Arc<Mutex<BTreeMap<String, SharedChannelSyncRateLimitBucket>>>,
}

#[derive(Clone, Debug)]
struct SharedChannelSyncRateLimitBucket {
    window_started_at_millis: u128,
    request_count: u32,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct MessageHistoryQuery {
    after_seq: Option<u64>,
    limit: Option<usize>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct MemberListQuery {
    limit: Option<usize>,
    cursor: Option<String>,
}

impl SharedChannelSyncRateLimiter {
    fn from_env() -> Self {
        let max_requests = resolve_positive_env_u32_with_upper_bound(
            SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS_ENV,
            SHARED_CHANNEL_SYNC_RATE_LIMIT_DEFAULT_MAX_REQUESTS,
            SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_MAX_REQUESTS,
        );
        let window_seconds = resolve_positive_env_u64_with_upper_bound(
            SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS_ENV,
            SHARED_CHANNEL_SYNC_RATE_LIMIT_DEFAULT_WINDOW_SECONDS,
            SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_WINDOW_SECONDS,
        );
        let max_buckets = resolve_positive_env_usize_with_upper_bound(
            SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_BUCKETS_ENV,
            SHARED_CHANNEL_SYNC_RATE_LIMIT_DEFAULT_MAX_BUCKETS,
            SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_BUCKETS,
        );
        Self {
            max_requests,
            window_millis: (window_seconds as u128) * 1000,
            max_buckets,
            buckets: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    fn try_acquire(&self, tenant_id: &str) -> bool {
        let now = current_unix_epoch_millis();
        let mut buckets =
            lock_shared_channel_rate_limit_mutex(&self.buckets, "shared-channel-sync-rate-limit");

        if buckets.len() > SHARED_CHANNEL_SYNC_RATE_LIMIT_SWEEP_THRESHOLD
            || buckets.len() >= self.max_buckets
        {
            let window_millis = self.window_millis;
            buckets.retain(|_, bucket| {
                now.saturating_sub(bucket.window_started_at_millis) < window_millis
            });
        }
        if !buckets.contains_key(tenant_id) && buckets.len() >= self.max_buckets {
            return false;
        }

        let bucket =
            buckets
                .entry(tenant_id.to_owned())
                .or_insert(SharedChannelSyncRateLimitBucket {
                    window_started_at_millis: now,
                    request_count: 0,
                });

        if now.saturating_sub(bucket.window_started_at_millis) >= self.window_millis {
            bucket.window_started_at_millis = now;
            bucket.request_count = 0;
        }

        if bucket.request_count >= self.max_requests {
            return false;
        }

        bucket.request_count = bucket.request_count.saturating_add(1);
        true
    }
}

fn lock_shared_channel_rate_limit_mutex<'a, T>(
    mutex: &'a Mutex<T>,
    lock_name: &'static str,
) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovered poisoned conversation-runtime mutex lock={lock_name}");
            poisoned.into_inner()
        }
    }
}

fn resolve_positive_env_u32_with_upper_bound(name: &str, default: u32, max: u32) -> u32 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
        .clamp(1, max)
}

fn resolve_positive_env_u64_with_upper_bound(name: &str, default: u64, max: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
        .clamp(1, max)
}

fn resolve_positive_env_usize_with_upper_bound(name: &str, default: usize, max: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
        .clamp(1, max)
}

fn unix_epoch_millis(now: SystemTime) -> u128 {
    now.duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn current_unix_epoch_millis() -> u128 {
    unix_epoch_millis(SystemTime::now())
}

fn shared_channel_sync_request_key(
    tenant_id: &str,
    request: &SyncSharedChannelLinkedMemberRequest,
) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}",
        tenant_id,
        request.conversation_id,
        request.shared_channel_policy_id,
        request.external_connection_id,
        request.local_actor_id,
        request.local_actor_kind,
        request.external_member_id
    )
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PostMessageRequest {
    client_msg_id: Option<String>,
    summary: Option<String>,
    text: Option<String>,
    reply_to: Option<im_domain_core::message::MessageReplyReference>,
    #[serde(default)]
    parts: Vec<ContentPart>,
    #[serde(default)]
    render_hints: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EditMessageRequest {
    summary: Option<String>,
    text: Option<String>,
    reply_to: Option<im_domain_core::message::MessageReplyReference>,
    #[serde(default)]
    parts: Vec<ContentPart>,
    #[serde(default)]
    render_hints: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageReactionRequest {
    reaction_key: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateConversationRequest {
    conversation_id: String,
    conversation_type: String,
    policy_version: Option<String>,
    capability_flags: Option<Vec<String>>,
    history_visibility: Option<String>,
    retention_policy_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateAgentDialogRequest {
    #[serde(default)]
    conversation_id: Option<String>,
    agent_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateAgentHandoffRequest {
    conversation_id: String,
    target_id: String,
    target_kind: String,
    handoff_session_id: String,
    handoff_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateSystemChannelRequest {
    conversation_id: String,
    subscriber_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateRoomRequest {
    conversation_id: String,
    room_id: String,
    room_kind: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct EnterRoomResponse {
    member: ConversationMember,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateThreadConversationRequest {
    conversation_id: String,
    parent_conversation_id: String,
    root_message_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BindDirectChatConversationRequest {
    #[serde(default)]
    conversation_id: Option<String>,
    #[serde(default)]
    direct_chat_id: Option<String>,
    left_actor_id: String,
    left_actor_kind: String,
    right_actor_id: String,
    right_actor_kind: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SyncSharedChannelLinkedMemberRequest {
    conversation_id: String,
    shared_channel_policy_id: String,
    external_connection_id: String,
    local_actor_id: String,
    local_actor_kind: String,
    external_member_id: String,
    #[serde(default)]
    request_key: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddConversationMemberRequest {
    principal_id: String,
    principal_kind: String,
    role: MembershipRole,
    #[serde(default)]
    attributes: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RemoveConversationMemberRequest {
    member_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransferConversationOwnerRequest {
    member_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChangeConversationMemberRoleRequest {
    member_id: String,
    role: MembershipRole,
}

type ListMembersResponse = ListMembersResult;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SyncSharedChannelLinkedMemberResponse {
    proof_version: &'static str,
    request_key: String,
    status: SyncSharedChannelLinkedMemberStatus,
    #[serde(flatten)]
    member: ConversationMember,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConversationBindingResponse {
    conversation_id: String,
    business_type: String,
    business_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateReadCursorRequest {
    read_seq: u64,
    last_read_message_id: Option<String>,
}

impl CreateConversationRequest {
    fn conversation_policy(&self) -> Result<Option<ConversationPolicy>, ApiError> {
        validate_optional_payload_size(
            "policyVersion",
            self.policy_version.as_deref(),
            CONVERSATION_MAX_POLICY_VERSION_BYTES,
        )
        .map_err(ApiError::from)?;
        if let Some(capability_flags) = &self.capability_flags {
            validate_string_vec_payload_size(
                "capabilityFlags",
                capability_flags,
                CONVERSATION_MAX_CAPABILITY_FLAG_BYTES,
                CONVERSATION_MAX_CAPABILITY_FLAGS_TOTAL_BYTES,
            )
            .map_err(ApiError::from)?;
        }
        validate_optional_payload_size(
            "historyVisibility",
            self.history_visibility.as_deref(),
            CONVERSATION_MAX_HISTORY_VISIBILITY_BYTES,
        )
        .map_err(ApiError::from)?;
        validate_optional_payload_size(
            "retentionPolicyRef",
            self.retention_policy_ref.as_deref(),
            CONVERSATION_MAX_RETENTION_POLICY_REF_BYTES,
        )
        .map_err(ApiError::from)?;
        if self.policy_version.is_none()
            && self.capability_flags.is_none()
            && self.history_visibility.is_none()
            && self.retention_policy_ref.is_none()
        {
            return Ok(None);
        }

        let mut policy = ConversationPolicy::default();
        if let Some(policy_version) = &self.policy_version {
            policy.policy_version = policy_version.clone();
        }
        if let Some(capability_flags) = &self.capability_flags {
            policy.capability_flags = Some(capability_flags.clone());
        }
        if let Some(history_visibility) = &self.history_visibility {
            policy.history_visibility = history_visibility.clone();
        }
        if let Some(retention_policy_ref) = &self.retention_policy_ref {
            policy.retention_policy_ref = retention_policy_ref.clone();
        }

        policy
            .normalize()
            .map(Some)
            .map_err(|message| ApiError::bad_request("conversation_policy_invalid", message))
    }
}

#[derive(Debug)]
pub(crate) struct ApiError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

struct AppJson<T>(T);

impl ApiError {
    fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code,
            message: message.into(),
        }
    }

    fn bad_request(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::BAD_REQUEST,
            code,
            message: message.into(),
        }
    }

    fn forbidden(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::FORBIDDEN,
            code,
            message: message.into(),
        }
    }

    fn too_many_requests(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::TOO_MANY_REQUESTS,
            code,
            message: message.into(),
        }
    }
}

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        Self {
            status: rejection.status(),
            code: "invalid_json",
            message: rejection.body_text(),
        }
    }
}

impl<S, T> FromRequest<S> for AppJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = ApiError;

    async fn from_request(
        request: Request<axum::body::Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(request, state)
            .await
            .map_err(ApiError::from)?;
        Ok(Self(value))
    }
}

/// Map [`ApiError::status`] to the canonical [`WebFrameworkErrorKind`].
fn api_error_kind(status: &axum::http::StatusCode) -> WebFrameworkErrorKind {
    use axum::http::StatusCode;
    match *status {
        StatusCode::BAD_REQUEST => WebFrameworkErrorKind::BadRequest,
        StatusCode::UNAUTHORIZED => WebFrameworkErrorKind::MissingCredentials,
        StatusCode::FORBIDDEN => WebFrameworkErrorKind::Forbidden,
        StatusCode::NOT_FOUND => WebFrameworkErrorKind::NotFound,
        StatusCode::CONFLICT => WebFrameworkErrorKind::Conflict,
        StatusCode::PAYLOAD_TOO_LARGE => WebFrameworkErrorKind::PayloadTooLarge,
        StatusCode::SERVICE_UNAVAILABLE => WebFrameworkErrorKind::DependencyUnavailable,
        StatusCode::NOT_IMPLEMENTED => WebFrameworkErrorKind::NotImplemented,
        _ => WebFrameworkErrorKind::InternalServerError,
    }
}

impl From<ApiError> for ApiProblem {
    fn from(error: ApiError) -> Self {
        let framework_error = WebFrameworkError {
            kind: api_error_kind(&error.status),
            message: error.message,
            retry_after_seconds: None,
        };
        ApiProblem::from_web_framework(framework_error)
    }
}

impl From<RuntimeError> for ApiProblem {
    fn from(error: RuntimeError) -> Self {
        ApiProblem::from(ApiError::from(error))
    }
}

impl From<RuntimeError> for ApiError {
    fn from(value: RuntimeError) -> Self {
        match value {
            RuntimeError::ConversationAlreadyExists(message) => {
                Self::bad_request("conversation_exists", message)
            }
            RuntimeError::ConversationTypeInvalid(message) => {
                Self::bad_request("conversation_type_invalid", message)
            }
            RuntimeError::AgentIdInvalid(message) => Self::bad_request("agent_id_invalid", message),
            RuntimeError::InvalidInput(message) => {
                Self::bad_request("conversation_request_invalid", message)
            }
            RuntimeError::PayloadTooLarge(message) => Self {
                status: axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                code: "payload_too_large",
                message,
            },
            RuntimeError::ConversationNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "conversation_not_found",
                message,
            },
            RuntimeError::ConversationBindingNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "conversation_binding_not_found",
                message,
            },
            RuntimeError::MessageNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "message_not_found",
                message,
            },
            RuntimeError::MessageAlreadyRecalled(message) => Self::bad_request(
                "message_already_recalled",
                format!("message already recalled: {message}"),
            ),
            RuntimeError::MemberAlreadyExists(message) => {
                Self::bad_request("conversation_member_exists", message)
            }
            RuntimeError::MemberNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "conversation_member_not_found",
                message,
            },
            RuntimeError::PermissionDenied(message) => {
                Self::forbidden("conversation_permission_denied", message)
            }
            RuntimeError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                code: "conversation_conflict",
                message,
            },
            RuntimeError::ReadCursorInvalid(message) => {
                Self::bad_request("read_cursor_invalid", message)
            }
            RuntimeError::Contract(error) => match error {
                sdkwork_im_contract_core::ContractError::Unavailable(message) => Self {
                    status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                    code: "journal_unavailable",
                    message,
                },
                sdkwork_im_contract_core::ContractError::Conflict(message) => Self {
                    status: axum::http::StatusCode::CONFLICT,
                    code: "journal_conflict",
                    message,
                },
                sdkwork_im_contract_core::ContractError::UnsupportedCapability(message) => Self {
                    status: axum::http::StatusCode::NOT_IMPLEMENTED,
                    code: "journal_capability_unsupported",
                    message,
                },
                sdkwork_im_contract_core::ContractError::Invalid(message) => Self {
                    status: axum::http::StatusCode::BAD_REQUEST,
                    code: "journal_invalid",
                    message,
                },
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let error = WebFrameworkError {
            kind: api_error_kind(&self.status),
            message: self.message,
            retry_after_seconds: None,
        };
        problem_response(&error, ProblemCorrelation::from(None))
    }
}

pub(crate) fn map_api_error_to_im_rpc(error: ApiError) -> sdkwork_im_rpc_service_rust::ImRpcError {
    match error.status {
        axum::http::StatusCode::BAD_REQUEST | axum::http::StatusCode::PAYLOAD_TOO_LARGE => {
            sdkwork_im_rpc_service_rust::ImRpcError::invalid_argument(error.message)
        }
        axum::http::StatusCode::UNAUTHORIZED => {
            sdkwork_im_rpc_service_rust::ImRpcError::unauthenticated(error.message)
        }
        axum::http::StatusCode::FORBIDDEN => {
            sdkwork_im_rpc_service_rust::ImRpcError::permission_denied(error.message)
        }
        axum::http::StatusCode::NOT_FOUND => {
            sdkwork_im_rpc_service_rust::ImRpcError::not_found(error.message)
        }
        axum::http::StatusCode::CONFLICT => {
            sdkwork_im_rpc_service_rust::ImRpcError::already_exists(error.message)
        }
        axum::http::StatusCode::SERVICE_UNAVAILABLE => {
            sdkwork_im_rpc_service_rust::ImRpcError::unavailable(error.message)
        }
        axum::http::StatusCode::TOO_MANY_REQUESTS => {
            sdkwork_im_rpc_service_rust::ImRpcError::resource_exhausted(error.message)
        }
        _ => sdkwork_im_rpc_service_rust::ImRpcError::internal(error.message),
    }
}

fn build_runtime_for_app_state() -> ConversationRuntime<ConversationCommitJournal> {
    super::journal_bootstrap::build_conversation_runtime_from_env().unwrap_or_else(|error| {
        if im_app_context::allows_header_only_app_context_fallback() {
            tracing::warn!(
                "conversation-runtime journal bootstrap failed ({error}); \
                 falling back to in-memory journal for local development"
            );
            ConversationRuntime::new(ConversationCommitJournal::Memory(
                InMemoryJournal::default(),
            ))
        } else {
            panic!(
                "conversation-runtime journal bootstrap failed in production: {error}"
            );
        }
    })
}

pub fn default_app_state() -> AppState {
    AppState {
        runtime: Arc::new(build_runtime_for_app_state()),
        principal_directory: Arc::new(AllowAllPrincipalDirectory),
        shared_channel_sync_rate_limiter: SharedChannelSyncRateLimiter::from_env(),
    }
}

pub fn app_state_with_principal_directory(
    principal_directory: Arc<dyn PrincipalDirectory>,
) -> AppState {
    AppState {
        runtime: Arc::new(build_runtime_for_app_state()),
        principal_directory,
        shared_channel_sync_rate_limiter: SharedChannelSyncRateLimiter::from_env(),
    }
}

/// Resolve conversation HTTP [`AppState`] from process environment.
///
/// Production requires a principal directory catalog. Development and test
/// environments may omit the catalog and fall back to allow-all principals.
pub fn bootstrap_conversation_app_state_from_env() -> Result<AppState, String> {
    if let Some(catalog_path) = std::env::var(PRINCIPAL_DIRECTORY_CATALOG_PATH_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
    {
        let directory =
            StaticPrincipalDirectory::from_json_file(FsPath::new(catalog_path.as_str()))?;
        return Ok(app_state_with_principal_directory(Arc::new(directory)));
    }

    let allow_all_explicit = std::env::var(ALLOW_ALL_PRINCIPALS_ENV)
        .ok()
        .and_then(|value| sdkwork_utils_rust::parse_bool(value.as_str()));

    let dev_or_test = im_app_context::allows_header_only_app_context_fallback();
    let allow_all = match allow_all_explicit {
        Some(true) => {
            if !dev_or_test {
                return Err(format!(
                    "{ALLOW_ALL_PRINCIPALS_ENV}=true is forbidden in production"
                ));
            }
            true
        }
        Some(false) => false,
        None => dev_or_test,
    };

    if allow_all {
        if !dev_or_test {
            return Err(format!(
                "principal directory is required in production: set {PRINCIPAL_DIRECTORY_CATALOG_PATH_ENV} \
                 to a JSON catalog file path"
            ));
        }
        tracing::warn!(
            env = %ALLOW_ALL_PRINCIPALS_ENV,
            "conversation-runtime using allow-all principal directory (development/test only)"
        );
        return Ok(default_app_state());
    }

    Err(format!(
        "principal directory is required: set {PRINCIPAL_DIRECTORY_CATALOG_PATH_ENV} to a JSON catalog file path, \
         or set {ALLOW_ALL_PRINCIPALS_ENV}=true for development-only mode"
    ))
}

pub fn build_default_app() -> Router {
    build_app(default_app_state())
}

pub fn build_default_app_with_principal_directory(
    principal_directory: Arc<dyn PrincipalDirectory>,
) -> Router {
    build_app(app_state_with_principal_directory(principal_directory))
}

pub fn apply_public_http_guardrails(router: Router) -> Router {
    let guardrails = PublicAppGuardrails {
        request_gate: Arc::new(Semaphore::new(resolve_max_in_flight_requests())),
    };
    router
        .layer(DefaultBodyLimit::max(resolve_max_http_request_body_bytes()))
        .layer(middleware::from_fn_with_state(
            guardrails,
            enforce_in_flight_gate,
        ))
}

pub fn build_public_app() -> Router {
    mount_im_infra_routes(
        apply_public_http_guardrails(build_business_router(default_app_state())),
        im_service_router_config(),
    )
}

pub fn build_public_app_with_allow_all_principals() -> Router {
    build_public_app()
}

pub fn build_public_app_with_principal_directory(
    principal_directory: Arc<dyn PrincipalDirectory>,
) -> Router {
    mount_im_infra_routes(
        apply_public_http_guardrails(build_business_router(
            app_state_with_principal_directory(principal_directory),
        )),
        im_service_router_config(),
    )
}

pub fn build_domain_api_router(state: AppState) -> Router {
    Router::new()
        .route("/im/v3/api/chat/rooms", post(create_room))
        .route("/im/v3/api/chat/rooms/{room_id}", get(get_room))
        .route("/im/v3/api/chat/rooms/{room_id}/enter", post(enter_room))
        .route("/im/v3/api/chat/rooms/{room_id}/leave", post(leave_room))
        .route("/im/v3/api/chat/conversations", post(create_conversation))
        .route(
            "/im/v3/api/chat/conversations/threads",
            post(create_thread_conversation),
        )
        .route(
            "/im/v3/api/chat/conversations/direct_chats/bindings",
            post(bind_direct_chat_conversation),
        )
        .route(
            "/im/v3/api/chat/conversations/shared_channel_links/sync",
            post(sync_shared_channel_linked_member),
        )
        .route(
            "/im/v3/api/chat/conversations/agent_dialogs",
            post(create_agent_dialog),
        )
        .route(
            "/im/v3/api/chat/conversations/agent_handoffs",
            post(create_agent_handoff),
        )
        .route(
            "/im/v3/api/chat/conversations/system_channels",
            post(create_system_channel),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/agent_handoff",
            get(get_agent_handoff_state),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/agent_handoff/accept",
            post(accept_agent_handoff),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/agent_handoff/resolve",
            post(resolve_agent_handoff),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/agent_handoff/close",
            post(close_agent_handoff),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/members",
            get(list_members),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/binding",
            get(get_conversation_binding),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/members/add",
            post(add_member),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/members/remove",
            post(remove_member),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/members/transfer_owner",
            post(transfer_conversation_owner),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/members/change_role",
            post(change_conversation_member_role),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/members/leave",
            post(leave_conversation),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/read_cursor",
            get(get_read_cursor).post(update_read_cursor),
        )
        .route(
            "/im/v3/api/chat/messages/{message_id}/edit",
            post(edit_message),
        )
        .route(
            "/im/v3/api/chat/messages/{message_id}/recall",
            post(recall_message),
        )
        .route(
            "/im/v3/api/chat/messages/{message_id}/reactions",
            post(add_message_reaction),
        )
        .route(
            "/im/v3/api/chat/messages/{message_id}/reactions/remove",
            post(remove_message_reaction),
        )
        .route(
            "/im/v3/api/chat/messages/{message_id}/pin",
            post(pin_message),
        )
        .route(
            "/im/v3/api/chat/messages/{message_id}/unpin",
            post(unpin_message),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/messages",
            get(list_messages).post(post_message),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/system_channel/publish",
            post(publish_system_channel_message),
        )
        .with_state(state)
}

fn build_business_router(state: AppState) -> Router {
    Router::new()
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs))
        .merge(build_domain_api_router(state))
}

fn build_app(state: AppState) -> Router {
    mount_im_infra_routes(
        build_business_router(state),
        im_service_router_config(),
    )
}

async fn enforce_in_flight_gate(
    State(guardrails): State<PublicAppGuardrails>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    if matches!(
        request.uri().path(),
        "/healthz" | "/readyz" | "/livez" | "/metrics" | "/openapi.json" | "/docs"
    ) {
        return next.run(request).await;
    }
    let permit = match guardrails.request_gate.clone().try_acquire_owned() {
        Ok(permit) => permit,
        Err(_) => {
            let problem = ApiProblem::dependency_unavailable(
                "server is at maximum in-flight request capacity, please retry later",
            );
            if let Some(ctx) = request.extensions().get::<WebRequestContext>() {
                return problem.into_response_for(ctx);
            }
            return ApiError {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "http_overloaded",
                message: "server is at maximum in-flight request capacity, please retry later"
                    .to_owned(),
            }
            .into_response();
        }
    };
    let response = next.run(request).await;
    drop(permit);
    response
}

fn resolve_max_in_flight_requests() -> usize {
    std::env::var(CONVERSATION_RUNTIME_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(CONVERSATION_RUNTIME_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(CONVERSATION_RUNTIME_MAX_IN_FLIGHT_REQUESTS_MAX)
}

fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(CONVERSATION_RUNTIME_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(CONVERSATION_RUNTIME_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(CONVERSATION_RUNTIME_MAX_REQUEST_BODY_BYTES_MAX)
}

async fn openapi_json() -> Result<Json<serde_json::Value>, ApiError> {
    Ok(Json(
        build_conversation_runtime_openapi_document()
            .map_err(|message| ApiError::internal("openapi_export_failed", message))?,
    ))
}

async fn docs() -> Html<String> {
    Html(render_docs_html(&conversation_runtime_openapi_spec()))
}

fn build_conversation_runtime_openapi_document() -> Result<serde_json::Value, String> {
    let http_source = include_str!("http.rs");
    let mut routes = extract_routes_from_function(
        http_source,
        "build_app",
        &[],
        &["/openapi.json", "/docs"],
    )?;
    routes.extend(extract_routes_from_function(
        http_source,
        "build_domain_api_router",
        &[],
        &[],
    )?);

    Ok(build_openapi_document(
        &conversation_runtime_openapi_spec(),
        &routes,
        conversation_runtime_tag,
        conversation_runtime_requires_app_context,
        conversation_runtime_summary,
    ))
}

fn conversation_runtime_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Sdkwork IM Conversation Runtime API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the conversation-runtime router for conversation creation, membership changes, messaging, read cursor updates, and shared_channel sync commands.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn conversation_runtime_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" | "/readyz" => "system".to_owned(),
        path if path.starts_with("/im/v3/api/chat/messages/") => "messages".to_owned(),
        path if path.contains("/members") => "members".to_owned(),
        path if path.contains("shared_channel_links") => "shared_channel".to_owned(),
        path if path.contains("/chat/rooms") => "room".to_owned(),
        path if path.contains("agent_handoff") => "agent_handoff".to_owned(),
        _ => "conversations".to_owned(),
    }
}

fn conversation_runtime_requires_app_context(path: &str, _method: HttpMethod) -> bool {
    !matches!(path, "/healthz" | "/readyz")
}

fn conversation_runtime_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check conversation runtime health".to_owned(),
        ("/readyz", HttpMethod::Get) => "Check conversation runtime readiness".to_owned(),
        _ => format!(
            "{} {}",
            conversation_runtime_method_display(method),
            path.trim_matches('/').replace('/', " ")
        ),
    }
}

fn conversation_runtime_method_display(method: HttpMethod) -> &'static str {
    match method {
        HttpMethod::Delete => "Delete",
        HttpMethod::Get => "Get",
        HttpMethod::Head => "Head",
        HttpMethod::Options => "Options",
        HttpMethod::Patch => "Patch",
        HttpMethod::Post => "Post",
        HttpMethod::Put => "Put",
    }
}

async fn create_room(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    AppJson(request): AppJson<CreateRoomRequest>,
) -> Response {
    let result: ApiResult<CreateConversationResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state.runtime.create_room_from_auth_context(
            &auth,
            request.conversation_id,
            request.room_id,
            request.room_kind,
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn get_room(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(room_id): Path<String>,
) -> Response {
    let result: ApiResult<RoomView> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state.runtime.room_view_from_auth_context(&auth, room_id)?)
    })();
    finish_api_json(&ctx, result)
}

async fn enter_room(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(room_id): Path<String>,
) -> Response {
    let result: ApiResult<EnterRoomResponse> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        let member = state
            .runtime
            .enter_room_from_auth_context(&auth, room_id)?;
        Ok(EnterRoomResponse { member })
    })();
    finish_api_json(&ctx, result)
}

async fn leave_room(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(room_id): Path<String>,
) -> Response {
    let result: ApiResult<EnterRoomResponse> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        let member = state
            .runtime
            .leave_room_from_auth_context(&auth, room_id)?;
        Ok(EnterRoomResponse { member })
    })();
    finish_api_json(&ctx, result)
}

async fn create_conversation(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    AppJson(request): AppJson<CreateConversationRequest>,
) -> Response {
    let result: ApiResult<CreateConversationResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        let organization_id = organization_id_from_auth_context(&auth);
        let policy = request.conversation_policy()?;
        let result = state.runtime.create_conversation_from_auth_context(
            &auth,
            request.conversation_id,
            request.conversation_type,
        )?;
        if let Some(policy) = policy {
            if result.is_applied() {
                state.runtime.apply_conversation_policy_from_auth_context(
                    &auth,
                    result.conversation_id.clone(),
                    policy,
                )?;
            } else {
                match state.runtime.conversation_policy_snapshot(
                    auth.tenant_id.as_str(),
                    organization_id.as_str(),
                    result.conversation_id.as_str(),
                )? {
                    Some(existing) if existing == policy => {}
                    Some(_) => {
                        return Err(RuntimeError::Conflict(format!(
                            "conversation create request conflicts with existing policy for conversation {}",
                            result.conversation_id
                        )).into());
                    }
                    None => {
                        state.runtime.apply_conversation_policy_from_auth_context(
                            &auth,
                            result.conversation_id.clone(),
                            policy,
                        )?;
                    }
                }
            }
        }
        Ok(result)
    })();
    finish_api_json(&ctx, result)
}

async fn create_agent_dialog(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    AppJson(request): AppJson<CreateAgentDialogRequest>,
) -> Response {
    let result: ApiResult<CreateConversationResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state.runtime.create_agent_dialog_from_auth_context(
            &auth,
            request.conversation_id.unwrap_or_default(),
            request.agent_id,
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn create_agent_handoff(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    AppJson(request): AppJson<CreateAgentHandoffRequest>,
) -> Response {
    let result: ApiResult<CreateConversationResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        ensure_active_http_principal(
            &state,
            auth.tenant_id.as_str(),
            request.target_id.as_str(),
            request.target_kind.as_str(),
        )?;
        Ok(state.runtime.create_agent_handoff_from_auth_context(
            &auth,
            request.conversation_id,
            request.target_id,
            request.target_kind,
            request.handoff_session_id,
            request.handoff_reason,
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn create_system_channel(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    AppJson(request): AppJson<CreateSystemChannelRequest>,
) -> Response {
    let result: ApiResult<CreateConversationResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        ensure_active_http_principal(
            &state,
            auth.tenant_id.as_str(),
            request.subscriber_id.as_str(),
            "user",
        )?;
        Ok(state.runtime.create_system_channel_from_auth_context(
            &auth,
            request.conversation_id,
            request.subscriber_id,
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn create_thread_conversation(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    AppJson(request): AppJson<CreateThreadConversationRequest>,
) -> Response {
    let result: ApiResult<CreateConversationResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state.runtime.create_thread_conversation_from_auth_context(
            &auth,
            request.conversation_id,
            request.parent_conversation_id,
            request.root_message_id,
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn bind_direct_chat_conversation(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    AppJson(request): AppJson<BindDirectChatConversationRequest>,
) -> Response {
    let result: ApiResult<CreateConversationResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        ensure_active_http_principal(
            &state,
            auth.tenant_id.as_str(),
            request.left_actor_id.as_str(),
            request.left_actor_kind.as_str(),
        )?;
        ensure_active_http_principal(
            &state,
            auth.tenant_id.as_str(),
            request.right_actor_id.as_str(),
            request.right_actor_kind.as_str(),
        )?;
        Ok(state
            .runtime
            .bind_direct_chat_conversation_from_auth_context(
                &auth,
                request.conversation_id.unwrap_or_default(),
                request.direct_chat_id.unwrap_or_default(),
                request.left_actor_id,
                request.left_actor_kind,
                request.right_actor_id,
                request.right_actor_kind,
            )?)
    })();
    finish_api_json(&ctx, result)
}

pub(crate) fn resolve_active_rpc_auth_context(
    state: &AppState,
    headers: &HeaderMap,
) -> Result<AppContext, ApiError> {
    let auth = resolve_app_context(headers).map_err(|value| ApiError {
        status: axum::http::StatusCode::UNAUTHORIZED,
        code: value.code(),
        message: value.message().to_owned(),
    })?;
    ensure_active_http_auth_principal(state, &auth)?;
    Ok(auth)
}

pub(crate) fn ensure_active_rpc_principal(
    state: &AppState,
    tenant_id: &str,
    principal_id: &str,
    principal_kind: &str,
) -> Result<(), ApiError> {
    ensure_active_http_principal(state, tenant_id, principal_id, principal_kind)
}

pub(crate) fn build_rpc_message_body(
    parts: Vec<ContentPart>,
    reply_to: Option<im_domain_core::message::MessageReplyReference>,
) -> Result<MessageBody, ApiError> {
    build_message_body(None, None, reply_to, parts, BTreeMap::new())
}

fn ensure_active_http_auth_principal(state: &AppState, auth: &AppContext) -> Result<(), ApiError> {
    ensure_active_http_principal(
        state,
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        auth.actor_kind.as_str(),
    )
}

fn ensure_active_http_principal(
    state: &AppState,
    tenant_id: &str,
    principal_id: &str,
    principal_kind: &str,
) -> Result<(), ApiError> {
    state
        .principal_directory
        .ensure_active_principal(tenant_id, principal_id, principal_kind)
        .map_err(map_principal_directory_error)
}

fn map_principal_directory_error(error: PrincipalDirectoryError) -> ApiError {
    match error {
        PrincipalDirectoryError::PrincipalNotFound {
            tenant_id,
            principal_id,
            principal_kind,
        } => ApiError::bad_request(
            "conversation_principal_not_found",
            format!(
                "principal not found in directory: tenant={tenant_id} principal={principal_kind}:{principal_id}"
            ),
        ),
        PrincipalDirectoryError::PrincipalDisabled {
            tenant_id,
            principal_id,
            principal_kind,
        } => ApiError::forbidden(
            "conversation_principal_disabled",
            format!(
                "principal disabled in directory: tenant={tenant_id} principal={principal_kind}:{principal_id}"
            ),
        ),
        PrincipalDirectoryError::Unavailable(message) => ApiError {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code: "principal_directory_unavailable",
            message,
        },
    }
}

fn validate_message_history_limit(limit: Option<usize>) -> Result<usize, ApiError> {
    normalize_message_history_limit(limit)
        .map_err(|message| ApiError::bad_request("limit_invalid", message))
}

async fn sync_shared_channel_linked_member(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    AppJson(request): AppJson<SyncSharedChannelLinkedMemberRequest>,
) -> Response {
    let result: ApiResult<SyncSharedChannelLinkedMemberResponse> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        ensure_active_http_principal(
            &state,
            auth.tenant_id.as_str(),
            request.local_actor_id.as_str(),
            request.local_actor_kind.as_str(),
        )?;
        if !auth.has_permission(SHARED_CHANNEL_SYNC_PERMISSION) {
            return Err(ApiError::forbidden(
                "shared_channel_sync_permission_denied",
                format!(
                    "shared channel linked-member sync requires permission {SHARED_CHANNEL_SYNC_PERMISSION}"
                ),
            )
            .into());
        }
        if auth.actor_id != SHARED_CHANNEL_SYNC_ACTOR_ID {
            return Err(ApiError::forbidden(
                "shared_channel_sync_actor_invalid",
                format!(
                    "shared channel linked-member sync requires actor {}",
                    SHARED_CHANNEL_SYNC_ACTOR_ID
                ),
            )
            .into());
        }
        if !state
            .shared_channel_sync_rate_limiter
            .try_acquire(auth.tenant_id.as_str())
        {
            return Err(ApiError::too_many_requests(
                "shared_channel_sync_rate_limited",
                "shared channel linked-member sync exceeded per-tenant rate limit",
            )
            .into());
        }
        let expected_request_key = shared_channel_sync_request_key(auth.tenant_id.as_str(), &request);
        if let Some(request_key) = request.request_key.as_deref() {
            validate_payload_size(
                "requestKey",
                request_key,
                CONVERSATION_MAX_REQUEST_KEY_BYTES,
            )
            .map_err(ApiError::from)?;
            if request_key.trim().is_empty() {
                return Err(ApiError::bad_request(
                    "shared_channel_sync_request_key_invalid",
                    "shared channel linked-member sync requestKey cannot be empty when provided",
                )
                .into());
            }
            if request_key != expected_request_key.as_str() {
                return Err(ApiError::bad_request(
                    "shared_channel_sync_request_key_mismatch",
                    format!(
                        "shared channel linked-member sync requestKey mismatch: expected {expected_request_key}, got {request_key}"
                    ),
                )
                .into());
            }
        }
        let request_key = request.request_key.clone().unwrap_or(expected_request_key);
        let sync_result = state
            .runtime
            .sync_shared_channel_linked_member_from_auth_context_with_result(
                &auth,
                request.conversation_id,
                request.shared_channel_policy_id,
                request.external_connection_id,
                request.local_actor_id,
                request.local_actor_kind,
                request.external_member_id,
            )?;
        Ok(SyncSharedChannelLinkedMemberResponse {
            proof_version: "shared_channel_sync_ack.v1",
            request_key,
            status: sync_result.status,
            member: sync_result.member,
        })
    })();
    finish_api_json(&ctx, result)
}

async fn get_agent_handoff_state(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
) -> Response {
    let result: ApiResult<AgentHandoffStateView> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state
            .runtime
            .get_agent_handoff_state_from_auth_context(&auth, conversation_id.as_str())?)
    })();
    finish_api_json(&ctx, result)
}

async fn get_conversation_binding(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
) -> Response {
    let result: ApiResult<ConversationBindingResponse> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        let binding = state
            .runtime
            .conversation_business_binding_from_auth_context(&auth, conversation_id.as_str())?;
        Ok(ConversationBindingResponse {
            conversation_id,
            business_type: binding.business_type,
            business_id: binding.business_id,
        })
    })();
    finish_api_json(&ctx, result)
}

async fn accept_agent_handoff(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
) -> Response {
    let result: ApiResult<AgentHandoffStateView> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state.runtime.accept_agent_handoff_from_auth_context(
            &auth,
            conversation_id,
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn resolve_agent_handoff(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
) -> Response {
    let result: ApiResult<AgentHandoffStateView> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state
            .runtime
            .resolve_agent_handoff_from_auth_context(&auth, conversation_id)?)
    })();
    finish_api_json(&ctx, result)
}

async fn close_agent_handoff(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
) -> Response {
    let result: ApiResult<AgentHandoffStateView> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state.runtime.close_agent_handoff_from_auth_context(
            &auth,
            conversation_id,
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn list_members(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    Query(query): Query<MemberListQuery>,
) -> Response {
    let result: ApiResult<ListMembersResponse> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        state
            .runtime
            .list_members_window_from_auth_context(
                &auth,
                conversation_id.as_str(),
                query.limit,
                query.cursor.as_deref(),
            )
            .map_err(|error| match error {
                RuntimeError::InvalidInput(message)
                    if message.contains("member list limit")
                        || message.contains("member list cursor") =>
                {
                    ApiError::bad_request(
                        if message.contains("cursor") {
                            "cursor_invalid"
                        } else {
                            "limit_invalid"
                        },
                        message,
                    )
                    .into()
                }
                other => ApiError::from(other).into(),
            })
    })();
    finish_api_json(&ctx, result)
}

async fn add_member(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    AppJson(request): AppJson<AddConversationMemberRequest>,
) -> Response {
    let result: ApiResult<ConversationMember> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        ensure_active_http_principal(
            &state,
            auth.tenant_id.as_str(),
            request.principal_id.as_str(),
            request.principal_kind.as_str(),
        )?;
        Ok(state.runtime.add_member_from_auth_context(
            &auth,
            conversation_id,
            request.principal_id,
            request.principal_kind,
            request.role,
            request.attributes,
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn remove_member(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    AppJson(request): AppJson<RemoveConversationMemberRequest>,
) -> Response {
    let result: ApiResult<ConversationMember> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state.runtime.remove_member_from_auth_context(
            &auth,
            conversation_id,
            request.member_id,
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn transfer_conversation_owner(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    AppJson(request): AppJson<TransferConversationOwnerRequest>,
) -> Response {
    let result: ApiResult<TransferConversationOwnerResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state
            .runtime
            .transfer_conversation_owner_from_auth_context(
                &auth,
                conversation_id,
                request.member_id,
            )?)
    })();
    finish_api_json(&ctx, result)
}

async fn change_conversation_member_role(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    AppJson(request): AppJson<ChangeConversationMemberRoleRequest>,
) -> Response {
    let result: ApiResult<ChangeConversationMemberRoleResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state
            .runtime
            .change_conversation_member_role_from_auth_context(
                &auth,
                conversation_id,
                request.member_id,
                request.role,
            )?)
    })();
    finish_api_json(&ctx, result)
}

async fn leave_conversation(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
) -> Response {
    let result: ApiResult<ConversationMember> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state.runtime.leave_conversation_from_auth_context(
            &auth,
            conversation_id,
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn get_read_cursor(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
) -> Response {
    let result: ApiResult<ConversationReadCursorView> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state.runtime.read_cursor_view_from_auth_context(
            &auth,
            conversation_id.as_str(),
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn list_messages(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    Query(query): Query<MessageHistoryQuery>,
) -> Response {
    let result: ApiResult<MessageHistoryResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        let limit = validate_message_history_limit(query.limit)?;
        Ok(state.runtime.list_messages_window_from_auth_context(
            &auth,
            conversation_id.as_str(),
            query.after_seq,
            limit,
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn update_read_cursor(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    AppJson(request): AppJson<UpdateReadCursorRequest>,
) -> Response {
    let result: ApiResult<ConversationReadCursorView> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        state.runtime.update_read_cursor_from_auth_context(
            &auth,
            conversation_id.clone(),
            request.read_seq,
            request.last_read_message_id,
        )?;

        Ok(state.runtime.read_cursor_view_from_auth_context(
            &auth,
            conversation_id.as_str(),
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn post_message(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    AppJson(request): AppJson<PostMessageRequest>,
) -> Response {
    let result: ApiResult<PostMessageResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        let body = build_message_body(
            request.summary,
            request.text,
            request.reply_to,
            request.parts,
            request.render_hints,
        )?;

        Ok(state
            .runtime
            .post_message(PostMessageCommand::from_auth_context(
                &auth,
                conversation_id,
                request.client_msg_id,
                MessageType::Standard,
                body,
            ))?)
    })();
    finish_api_json(&ctx, result)
}

async fn publish_system_channel_message(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    AppJson(request): AppJson<PostMessageRequest>,
) -> Response {
    let result: ApiResult<PostMessageResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        let body = build_message_body(
            request.summary,
            request.text,
            request.reply_to,
            request.parts,
            request.render_hints,
        )?;

        Ok(state.runtime.publish_system_channel_message(
            PublishSystemChannelMessageCommand::from_auth_context(
                &auth,
                conversation_id,
                request.client_msg_id,
                body,
            ),
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn edit_message(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(message_id): Path<String>,
    AppJson(request): AppJson<EditMessageRequest>,
) -> Response {
    let result: ApiResult<MessageMutationResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        let body = build_message_body(
            request.summary,
            request.text,
            request.reply_to,
            request.parts,
            request.render_hints,
        )?;
        Ok(state.runtime.edit_message(
            EditMessageCommand::from_auth_context(&auth, message_id, body),
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn recall_message(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(message_id): Path<String>,
) -> Response {
    let result: ApiResult<MessageMutationResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state.runtime.recall_message(
            RecallMessageCommand::from_auth_context(&auth, message_id),
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn add_message_reaction(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(message_id): Path<String>,
    AppJson(request): AppJson<MessageReactionRequest>,
) -> Response {
    let result: ApiResult<MessageReactionMutationResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        if request.reaction_key.trim().is_empty() {
            return Err(ApiError::bad_request(
                "reaction_key_invalid",
                "reaction key must not be empty",
            )
            .into());
        }

        Ok(state.runtime.add_message_reaction(
            AddMessageReactionCommand::from_auth_context(&auth, message_id, request.reaction_key),
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn remove_message_reaction(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(message_id): Path<String>,
    AppJson(request): AppJson<MessageReactionRequest>,
) -> Response {
    let result: ApiResult<MessageReactionMutationResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        if request.reaction_key.trim().is_empty() {
            return Err(ApiError::bad_request(
                "reaction_key_invalid",
                "reaction key must not be empty",
            )
            .into());
        }

        Ok(state.runtime.remove_message_reaction(
            RemoveMessageReactionCommand::from_auth_context(&auth, message_id, request.reaction_key),
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn pin_message(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(message_id): Path<String>,
) -> Response {
    let result: ApiResult<MessagePinMutationResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state.runtime.pin_message(
            PinMessageCommand::from_auth_context(&auth, message_id),
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn unpin_message(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(message_id): Path<String>,
) -> Response {
    let result: ApiResult<MessagePinMutationResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state.runtime.unpin_message(
            UnpinMessageCommand::from_auth_context(&auth, message_id),
        )?)
    })();
    finish_api_json(&ctx, result)
}

fn build_message_body(
    summary: Option<String>,
    text: Option<String>,
    reply_to: Option<im_domain_core::message::MessageReplyReference>,
    parts: Vec<ContentPart>,
    render_hints: BTreeMap<String, String>,
) -> Result<MessageBody, ApiError> {
    let mut resolved_parts = Vec::new();
    if let Some(text) = text
        && !text.trim().is_empty()
    {
        resolved_parts.push(ContentPart::text(text));
    }
    resolved_parts.extend(parts);

    if resolved_parts.is_empty() {
        return Err(ApiError::bad_request(
            "message_body_empty",
            "message body must contain text or parts",
        ));
    }

    Ok(MessageBody {
        summary,
        parts: resolved_parts,
        render_hints,
        reply_to,
    }
    .with_derived_summary())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{HeaderMap, HeaderValue, Request, StatusCode};
    use http_body_util::BodyExt;
    use im_app_context::DualTokenRequestBuilderExt;
    use std::collections::BTreeSet;
    use std::sync::{Mutex, OnceLock};
    use std::time::Duration;
    use tower::ServiceExt;

    #[derive(Clone)]
    struct StrictKnownPrincipalDirectory {
        known_user_ids: Vec<&'static str>,
    }

    impl StrictKnownPrincipalDirectory {
        fn new(known_user_ids: &[&'static str]) -> Self {
            Self {
                known_user_ids: known_user_ids.to_vec(),
            }
        }
    }

    impl PrincipalDirectory for StrictKnownPrincipalDirectory {
        fn ensure_active_principal(
            &self,
            _tenant_id: &str,
            principal_id: &str,
            principal_kind: &str,
        ) -> Result<(), PrincipalDirectoryError> {
            if principal_kind != "user" {
                return Ok(());
            }
            if self.known_user_ids.contains(&principal_id) {
                return Ok(());
            }

            Err(PrincipalDirectoryError::PrincipalNotFound {
                tenant_id: "100001".into(),
                principal_id: principal_id.into(),
                principal_kind: principal_kind.into(),
            })
        }
    }

    struct ScopedEnvVar {
        name: &'static str,
        previous: Option<String>,
    }

    impl ScopedEnvVar {
        fn set(name: &'static str, value: &str) -> Self {
            let previous = std::env::var(name).ok();
            unsafe {
                std::env::set_var(name, value);
            }
            Self { name, previous }
        }

        fn remove(name: &'static str) -> Self {
            let previous = std::env::var(name).ok();
            unsafe {
                std::env::remove_var(name);
            }
            Self { name, previous }
        }
    }

    impl Drop for ScopedEnvVar {
        fn drop(&mut self) {
            if let Some(previous) = &self.previous {
                unsafe {
                    std::env::set_var(self.name, previous);
                }
                return;
            }
            unsafe {
                std::env::remove_var(self.name);
            }
        }
    }

    fn rate_limit_env_guard<'a>() -> std::sync::MutexGuard<'a, ()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("env lock")
    }

    fn build_test_app_with_runtime_and_directory(
        runtime: Arc<ConversationRuntime<ConversationCommitJournal>>,
        principal_directory: Arc<dyn PrincipalDirectory>,
    ) -> Router {
        build_app(AppState {
            runtime,
            principal_directory,
            shared_channel_sync_rate_limiter: SharedChannelSyncRateLimiter::from_env(),
        })
    }

    fn seed_group_conversation_with_ghost_member(
        runtime: &ConversationRuntime<ConversationCommitJournal>,
        conversation_id: &str,
    ) -> String {
        let owner_auth = AppContext {
            tenant_id: "100001".into(),
            organization_id: "0".to_owned(),
            user_id: "1".into(),
            actor_id: "1".into(),
            actor_kind: "user".into(),
            session_id: None,
            app_id: None,
            environment: None,
            deployment_mode: None,
            auth_level: None,
            data_scope: BTreeSet::new(),
            permission_scope: BTreeSet::new(),
            device_id: None,
        };
        runtime
            .create_conversation(CreateConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: conversation_id.into(),
                creator_id: "1".into(),
                conversation_type: "group".into(),
            })
            .expect("seed create conversation should succeed");
        runtime
            .add_member(AddConversationMemberCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: conversation_id.into(),
                principal_id: "1044".into(),
                principal_kind: "user".into(),
                role: MembershipRole::Member,
                invited_by: "1".into(),
            })
            .expect("seed add ghost member should succeed");

        runtime
            .post_message(PostMessageCommand::from_auth_context(
                &owner_auth,
                conversation_id.into(),
                Some(format!("seed_{conversation_id}")),
                MessageType::Standard,
                build_message_body(
                    Some("seed root".into()),
                    Some("seed root".into()),
                    None,
                    Vec::new(),
                    BTreeMap::new(),
                )
                .expect("seed message body should build"),
            ))
            .expect("seed root message should succeed")
            .message_id
    }

    #[test]
    fn test_unix_epoch_millis_clamps_pre_epoch_time_to_zero() {
        let before_epoch = UNIX_EPOCH
            .checked_sub(Duration::from_millis(1))
            .expect("test pre-epoch timestamp should construct");
        assert_eq!(unix_epoch_millis(before_epoch), 0);
    }

    #[test]
    fn test_unix_epoch_millis_preserves_post_epoch_time() {
        let after_epoch = UNIX_EPOCH + Duration::from_millis(1_234);
        assert_eq!(unix_epoch_millis(after_epoch), 1_234);
    }

    #[test]
    fn test_shared_channel_sync_rate_limiter_clamps_env_values_to_safe_bounds() {
        let _guard = rate_limit_env_guard();
        let _max_requests =
            ScopedEnvVar::set(SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS_ENV, "999999");
        let _window_seconds =
            ScopedEnvVar::set(SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS_ENV, "999999");
        let _max_buckets =
            ScopedEnvVar::set(SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_BUCKETS_ENV, "999999");

        let limiter = SharedChannelSyncRateLimiter::from_env();
        assert_eq!(
            limiter.max_requests,
            SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_MAX_REQUESTS
        );
        assert_eq!(
            limiter.window_millis,
            (SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_WINDOW_SECONDS as u128) * 1000
        );
        assert_eq!(
            limiter.max_buckets,
            SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_BUCKETS
        );
    }

    #[test]
    fn test_shared_channel_sync_rate_limiter_rejects_new_tenant_when_bucket_cap_is_reached() {
        let limiter = SharedChannelSyncRateLimiter {
            max_requests: 2,
            window_millis: 60_000,
            max_buckets: 2,
            buckets: Arc::new(Mutex::new(BTreeMap::new())),
        };

        assert!(limiter.try_acquire("tenant_a"));
        assert!(limiter.try_acquire("tenant_b"));
        assert!(
            !limiter.try_acquire("tenant_c"),
            "new tenant should be rejected when rate-limit bucket cap is reached"
        );
        assert!(
            limiter.try_acquire("tenant_a"),
            "existing tenant should still be serviceable when cap is reached"
        );
    }

    #[test]
    fn test_shared_channel_sync_rate_limiter_prunes_expired_buckets_before_rejecting_new_tenant() {
        let limiter = SharedChannelSyncRateLimiter {
            max_requests: 1,
            window_millis: 1,
            max_buckets: 2,
            buckets: Arc::new(Mutex::new(BTreeMap::new())),
        };
        {
            let mut buckets = lock_shared_channel_rate_limit_mutex(
                &limiter.buckets,
                "shared-channel-sync-rate-limit",
            );
            buckets.insert(
                "tenant_expired_a".into(),
                SharedChannelSyncRateLimitBucket {
                    window_started_at_millis: 0,
                    request_count: 1,
                },
            );
            buckets.insert(
                "tenant_expired_b".into(),
                SharedChannelSyncRateLimitBucket {
                    window_started_at_millis: 0,
                    request_count: 1,
                },
            );
        }

        assert!(
            limiter.try_acquire("tenant_new"),
            "expired buckets should be swept before enforcing max bucket cap"
        );
    }

    #[test]
    fn test_build_message_body_derives_summary_for_structured_message_when_missing() {
        let body = build_message_body(
            None,
            None,
            None,
            vec![ContentPart::Data(im_domain_core::message::DataPart {
                schema_ref: im_domain_core::message::SDKWORK_IM_MESSAGE_SCHEMA_LOCATION.into(),
                encoding: "application/json".into(),
                payload: serde_json::json!({
                    "name": "The Bund",
                    "latitude": 31.2400,
                    "longitude": 121.4900
                })
                .to_string(),
            })],
            BTreeMap::new(),
        )
        .expect("rich message body should build");

        assert_eq!(body.summary.as_deref(), Some("Location: The Bund"));
    }

    #[test]
    fn test_build_message_body_preserves_explicit_summary_over_derived_summary() {
        let body = build_message_body(
            Some("Pinned location".into()),
            Some("caption".into()),
            None,
            vec![ContentPart::Data(im_domain_core::message::DataPart {
                schema_ref: im_domain_core::message::SDKWORK_IM_MESSAGE_SCHEMA_LOCATION.into(),
                encoding: "application/json".into(),
                payload: serde_json::json!({
                    "name": "West Lake",
                    "latitude": 30.2528,
                    "longitude": 120.1551
                })
                .to_string(),
            })],
            BTreeMap::new(),
        )
        .expect("rich message body should build");

        assert_eq!(body.summary.as_deref(), Some("Pinned location"));
    }

    #[tokio::test]
    async fn test_post_message_rejects_unknown_user_member_with_strict_principal_directory() {
        let runtime = Arc::new(ConversationRuntime::new(ConversationCommitJournal::Memory(
            InMemoryJournal::default(),
        )));
        seed_group_conversation_with_ghost_member(runtime.as_ref(), "c_ghost_post_http");
        let app = build_test_app_with_runtime_and_directory(
            runtime,
            Arc::new(StrictKnownPrincipalDirectory::new(&["1"])),
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/chat/conversations/c_ghost_post_http/messages")
                    .with_dual_token_context("100001", "1044", "user", None, ["*"])
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{
                            "clientMsgId":"ghost_http_post",
                            "summary":"ghost",
                            "text":"ghost"
                        }"#,
                    ))
                    .unwrap(),
            )
            .await
            .expect("ghost member post request should return response");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = response
            .into_body()
            .collect()
            .await
            .expect("body should collect")
            .to_bytes();
        let value: serde_json::Value =
            serde_json::from_slice(&body).expect("response should be valid json");
        assert_eq!(value["code"], 40001);
        assert!(value["detail"]
            .as_str()
            .expect("detail should be string")
            .contains("principal not found in directory"));
    }

    #[tokio::test]
    async fn test_list_messages_rejects_unknown_user_member_with_strict_principal_directory() {
        let runtime = Arc::new(ConversationRuntime::new(ConversationCommitJournal::Memory(
            InMemoryJournal::default(),
        )));
        seed_group_conversation_with_ghost_member(runtime.as_ref(), "c_ghost_history_http");
        let app = build_test_app_with_runtime_and_directory(
            runtime,
            Arc::new(StrictKnownPrincipalDirectory::new(&["1"])),
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/im/v3/api/chat/conversations/c_ghost_history_http/messages")
                    .with_dual_token_context("100001", "1044", "user", None, ["*"])
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("ghost member history request should return response");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = response
            .into_body()
            .collect()
            .await
            .expect("body should collect")
            .to_bytes();
        let value: serde_json::Value =
            serde_json::from_slice(&body).expect("response should be valid json");
        assert_eq!(value["code"], 40001);
        assert!(value["detail"]
            .as_str()
            .expect("detail should be string")
            .contains("principal not found in directory"));
    }
}
