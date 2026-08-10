use std::collections::BTreeMap;
use std::fs;
use std::path::Path as FsPath;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::{DefaultBodyLimit, Extension, FromRequest, Path, Query, State};
use axum::http::{HeaderMap, HeaderName, HeaderValue, Request, Uri, header};
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use im_app_context::{AppContext, app_context_from_web_request, resolve_app_context};
use im_domain_core::conversation::{
    ConversationMember, ConversationReadCursorView, MembershipRole,
};
use im_domain_core::message::{ContentPart, Message, MessageBody, MessageType, Sender};
use sdkwork_im_api_registry::HttpMethod;
use sdkwork_im_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};
use sdkwork_im_web_bootstrap::{im_service_router_config, mount_im_infra_routes};
use sdkwork_routes_web_framework_backend_api::response::{
    ApiProblem, ApiResult, created_json, finish_api_json, finish_api_response,
};
use sdkwork_utils_rust::{
    SDKWORK_TRACE_ID_HEADER, SdkWorkCommandData, SdkWorkCursorListQuery, SdkWorkPageData,
    SdkWorkProblemDetail, SdkWorkResourceData, SdkWorkResultCode,
};
use sdkwork_web_core::{
    ProblemCorrelation, WebFrameworkError, WebFrameworkErrorKind, WebRequestContext,
    problem_response,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
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
pub const PRINCIPAL_DIRECTORY_CATALOG_PATH_ENV: &str =
    "SDKWORK_IM_PRINCIPAL_DIRECTORY_CATALOG_PATH";
pub const ALLOW_ALL_PRINCIPALS_ENV: &str = "SDKWORK_IM_ALLOW_ALL_PRINCIPALS";

#[derive(Clone)]
pub struct AppState {
    runtime: Arc<ConversationRuntime<ConversationCommitJournal>>,
    principal_directory: Arc<dyn PrincipalDirectory>,
    group_knowledgebase: Arc<GroupKnowledgebaseCoordinator>,
    group_knowledgebase_outbox_relay_owner: Arc<GroupKnowledgebaseOutboxRelayOwner>,
    group_knowledgebase_launch_rate_limiter: GroupKnowledgebaseLaunchRateLimiter,
    shared_channel_sync_rate_limiter: SharedChannelSyncRateLimiter,
}

/// Owns the one group-Knowledgebase relay for an authoritative Conversation
/// state. Clones of [`AppState`] share this owner, while separate production
/// processes coordinate through the PostgreSQL outbox claim lease and the
/// stable remote idempotency keys.
struct GroupKnowledgebaseOutboxRelayOwner {
    startup_gate: tokio::sync::Mutex<()>,
    handle: Mutex<Option<GroupKnowledgebaseOutboxRelayHandle>>,
}

impl GroupKnowledgebaseOutboxRelayOwner {
    fn new() -> Self {
        Self {
            startup_gate: tokio::sync::Mutex::new(()),
            handle: Mutex::new(None),
        }
    }

    async fn ensure_started(
        &self,
        coordinator: Arc<GroupKnowledgebaseCoordinator>,
        runtime: Arc<ConversationRuntime<ConversationCommitJournal>>,
    ) -> Result<bool, RuntimeError> {
        let _startup_guard = self.startup_gate.lock().await;
        if self
            .handle
            .lock()
            .map_err(|_| {
                RuntimeError::Contract(im_platform_contracts::ContractError::Unavailable(
                    "group knowledgebase outbox relay owner lock is unavailable".into(),
                ))
            })?
            .is_some()
        {
            return Ok(false);
        }

        let handle = spawn_group_knowledgebase_outbox_relay(coordinator, runtime).await?;
        let mut retained_handle = self.handle.lock().map_err(|_| {
            RuntimeError::Contract(im_platform_contracts::ContractError::Unavailable(
                "group knowledgebase outbox relay owner lock is unavailable".into(),
            ))
        })?;
        *retained_handle = Some(handle);
        Ok(true)
    }
}

impl AppState {
    pub fn runtime(&self) -> Arc<ConversationRuntime<ConversationCommitJournal>> {
        self.runtime.clone()
    }

    pub(crate) fn rpc_runtime(&self) -> &ConversationRuntime<ConversationCommitJournal> {
        self.runtime.as_ref()
    }

    pub(crate) fn group_knowledgebase(&self) -> Arc<GroupKnowledgebaseCoordinator> {
        self.group_knowledgebase.clone()
    }

    /// Verifies delivery dependencies and starts the single durable
    /// Knowledgebase relay retained by this authoritative state. Production
    /// callers must invoke this before they listen for group mutations.
    pub async fn ensure_group_knowledgebase_outbox_relay_started(
        &self,
    ) -> Result<(), RuntimeError> {
        if super::knowledgebase_rpc_config::resolve_group_knowledgebase_rpc_port_from_env()?
            .is_none()
        {
            tracing::info!(
                "group knowledgebase outbox relay is disabled because the development/test RPC client is not configured"
            );
            return Ok(());
        }

        match self
            .group_knowledgebase_outbox_relay_owner
            .ensure_started(self.group_knowledgebase.clone(), self.runtime.clone())
            .await
        {
            Ok(started) => {
                if started {
                    tracing::info!(
                        "group knowledgebase outbox relay readiness completed for conversation runtime"
                    );
                }
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    fn register_for_embedded_wiring(&self) {
        crate::embedded_wiring::register_embedded_conversation_runtime(self.runtime.clone());
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
const GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_MAX_REQUESTS_ENV: &str =
    "SDKWORK_IM_GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_MAX_REQUESTS";
const GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_WINDOW_SECONDS_ENV: &str =
    "SDKWORK_IM_GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_WINDOW_SECONDS";
const GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_MAX_BUCKETS_ENV: &str =
    "SDKWORK_IM_GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_MAX_BUCKETS";
const GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_DEFAULT_MAX_REQUESTS: u32 = 12;
const GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_DEFAULT_WINDOW_SECONDS: u64 = 60;
const GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_DEFAULT_MAX_BUCKETS: usize = 100_000;
const GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_MAX_ALLOWED_MAX_REQUESTS: u32 = 600;
const GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_MAX_ALLOWED_WINDOW_SECONDS: u64 = 3_600;
const GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_MAX_ALLOWED_BUCKETS: usize = 200_000;
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

#[derive(Clone, Copy)]
struct RateLimiterEnvironmentConfig<'a> {
    max_requests_env: &'a str,
    default_max_requests: u32,
    max_allowed_max_requests: u32,
    window_seconds_env: &'a str,
    default_window_seconds: u64,
    max_allowed_window_seconds: u64,
    max_buckets_env: &'a str,
    default_max_buckets: usize,
    max_allowed_max_buckets: usize,
}

/// Launch tickets are capability credentials, so their issuance has an
/// independent, per-principal rate limit. The fixed-window implementation is
/// shared with the existing control-plane sync limiter to keep its memory
/// bounds and poisoned-lock recovery behavior consistent.
#[derive(Clone)]
struct GroupKnowledgebaseLaunchRateLimiter {
    inner: SharedChannelSyncRateLimiter,
}

#[derive(Debug, Deserialize, Default)]
struct MessageHistoryQuery {
    #[serde(flatten)]
    paging: SdkWorkCursorListQuery,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConversationMessageEntry {
    tenant_id: String,
    conversation_id: String,
    message_id: String,
    message_seq: u64,
    summary: Option<String>,
    sender: Sender,
    body: MessageBody,
    message_type: MessageType,
    delivery_mode: String,
    client_msg_id: Option<String>,
    stream_session_id: Option<String>,
    rtc_session_id: Option<String>,
    occurred_at: String,
    committed_at: Option<String>,
}

impl From<&Message> for ConversationMessageEntry {
    fn from(message: &Message) -> Self {
        Self {
            tenant_id: message.tenant_id.clone(),
            conversation_id: message.conversation_id.clone(),
            message_id: message.message_id.clone(),
            message_seq: message.message_seq,
            summary: message.body.summary_or_derived(),
            sender: message.sender.clone(),
            body: message.body.clone(),
            message_type: message.message_type.clone(),
            delivery_mode: message.delivery_mode.clone(),
            client_msg_id: message.client_msg_id.clone(),
            stream_session_id: message.stream_session_id.clone(),
            rtc_session_id: message.rtc_session_id.clone(),
            occurred_at: message.occurred_at.clone(),
            committed_at: message.committed_at.clone(),
        }
    }
}

impl From<&im_domain_core::message::StoredMessage> for ConversationMessageEntry {
    fn from(stored: &im_domain_core::message::StoredMessage) -> Self {
        Self::from(&stored.message)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConversationMessageListResponse {
    #[serde(flatten)]
    page: SdkWorkPageData<ConversationMessageEntry>,
    high_watermark: u64,
}

impl ConversationMessageListResponse {
    fn from_history(history: MessageHistoryResult, next_cursor: Option<String>) -> Self {
        let MessageHistoryResult {
            page,
            high_watermark,
            ..
        } = history;
        let SdkWorkPageData {
            items,
            mut page_info,
        } = page;
        page_info.next_cursor = next_cursor;
        Self {
            page: SdkWorkPageData {
                items: items.iter().map(ConversationMessageEntry::from).collect(),
                page_info,
            },
            high_watermark,
        }
    }
}

fn created_resource_response<T: Serialize>(
    ctx: &WebRequestContext,
    result: ApiResult<T>,
) -> Response {
    finish_api_response(
        ctx,
        result.and_then(|item| created_json(ctx, SdkWorkResourceData { item })),
    )
}

fn resource_response<T: Serialize>(ctx: &WebRequestContext, result: ApiResult<T>) -> Response {
    finish_api_json(ctx, result.map(|item| SdkWorkResourceData { item }))
}

fn no_store_resource_response<T: Serialize>(
    ctx: &WebRequestContext,
    result: ApiResult<T>,
) -> Response {
    let mut response = resource_response(ctx, result);
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

impl SharedChannelSyncRateLimiter {
    fn from_env() -> Self {
        Self::from_env_config(RateLimiterEnvironmentConfig {
            max_requests_env: SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS_ENV,
            default_max_requests: SHARED_CHANNEL_SYNC_RATE_LIMIT_DEFAULT_MAX_REQUESTS,
            max_allowed_max_requests: SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_MAX_REQUESTS,
            window_seconds_env: SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS_ENV,
            default_window_seconds: SHARED_CHANNEL_SYNC_RATE_LIMIT_DEFAULT_WINDOW_SECONDS,
            max_allowed_window_seconds: SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_WINDOW_SECONDS,
            max_buckets_env: SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_BUCKETS_ENV,
            default_max_buckets: SHARED_CHANNEL_SYNC_RATE_LIMIT_DEFAULT_MAX_BUCKETS,
            max_allowed_max_buckets: SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_BUCKETS,
        })
    }

    fn from_env_config(config: RateLimiterEnvironmentConfig<'_>) -> Self {
        let max_requests = resolve_positive_env_u32_with_upper_bound(
            config.max_requests_env,
            config.default_max_requests,
            config.max_allowed_max_requests,
        );
        let window_seconds = resolve_positive_env_u64_with_upper_bound(
            config.window_seconds_env,
            config.default_window_seconds,
            config.max_allowed_window_seconds,
        );
        let max_buckets = resolve_positive_env_usize_with_upper_bound(
            config.max_buckets_env,
            config.default_max_buckets,
            config.max_allowed_max_buckets,
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

impl GroupKnowledgebaseLaunchRateLimiter {
    fn from_env() -> Self {
        Self {
            inner: SharedChannelSyncRateLimiter::from_env_config(RateLimiterEnvironmentConfig {
                max_requests_env: GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_MAX_REQUESTS_ENV,
                default_max_requests: GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_DEFAULT_MAX_REQUESTS,
                max_allowed_max_requests:
                    GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_MAX_ALLOWED_MAX_REQUESTS,
                window_seconds_env: GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_WINDOW_SECONDS_ENV,
                default_window_seconds:
                    GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_DEFAULT_WINDOW_SECONDS,
                max_allowed_window_seconds:
                    GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_MAX_ALLOWED_WINDOW_SECONDS,
                max_buckets_env: GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_MAX_BUCKETS_ENV,
                default_max_buckets: GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_DEFAULT_MAX_BUCKETS,
                max_allowed_max_buckets: GROUP_KNOWLEDGEBASE_LAUNCH_RATE_LIMIT_MAX_ALLOWED_BUCKETS,
            }),
        }
    }

    fn try_acquire(&self, auth: &AppContext) -> bool {
        let scope = format!(
            "{}:{}:{}:{}",
            auth.tenant_id,
            organization_id_from_auth_context(auth),
            auth.actor_kind,
            auth.actor_id,
        );
        self.inner.try_acquire(scope.as_str())
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
    idempotency_key: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct RecallMessageRequest {
    idempotency_key: Option<String>,
}

/// The command has no caller-controlled business fields. Group scope,
/// membership, and initial space metadata remain authoritative Conversation
/// state; retaining an explicit empty object prevents undocumented empty POST
/// semantics from leaking into generated SDKs.
#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct CreateGroupKnowledgebaseCommandRequest {}

/// Ticket issuance is deliberately parameterless: the authenticated member,
/// active group binding, and synchronized membership epoch are all resolved
/// server-side before a capability ticket is minted.
#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct LaunchGroupKnowledgebaseCommandRequest {}

/// The group archive command has no caller-controlled business fields. Its
/// target, owner, and lifecycle source event are derived from the trusted
/// request context and path.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ArchiveGroupConversationCommandRequest {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
struct ArchiveGroupConversationResponse {
    #[serde(flatten)]
    command: SdkWorkCommandData,
    archive_event_id: String,
    archived_at: String,
    knowledgebase_archive_scheduled: bool,
}

impl ArchiveGroupConversationResponse {
    fn accepted(
        resource_id: String,
        archive_event_id: String,
        archived_at: String,
        knowledgebase_archive_scheduled: bool,
    ) -> Self {
        Self {
            command: SdkWorkCommandData {
                accepted: true,
                resource_id: Some(resource_id),
                status: Some("archived".into()),
            },
            archive_event_id,
            archived_at,
            knowledgebase_archive_scheduled,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageReactionRequest {
    reaction_key: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateConversationRequest {
    /// Client-supplied conversation id. Required for `direct` and other generic
    /// conversation types; ignored for `group` conversations where the server
    /// derives a canonical `g_` id from creator + group name + client request
    /// key.
    #[serde(default)]
    conversation_id: Option<String>,
    conversation_type: String,
    /// Group display name. Required when `conversationType` is `group`; ignored
    /// otherwise.
    #[serde(default)]
    group_name: Option<String>,
    /// Client-supplied idempotency seed for group creation. Required when
    /// `conversationType` is `group`; ignored otherwise.
    #[serde(default)]
    client_request_key: Option<String>,
    /// Explicit opt-in for one post-create group Knowledgebase provisioning
    /// attempt. Omitted or false must leave the Knowledgebase lifecycle
    /// absent, which keeps ordinary group creation lazy.
    #[serde(default)]
    initialize_knowledgebase: bool,
    /// Optional initial group agent set committed in the same creation batch.
    #[serde(default)]
    agent_assignments: Option<Vec<ConversationAgentAssignment>>,
    /// Optional initial user members committed in the same creation batch.
    #[serde(default)]
    member_user_ids: Option<Vec<String>>,
    policy_version: Option<String>,
    capability_flags: Option<Vec<String>>,
    history_visibility: Option<String>,
    retention_policy_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReplaceConversationAgentsRequest {
    expected_generation: u64,
    agent_assignments: Vec<ConversationAgentAssignment>,
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
    #[serde(default)]
    conversation_id: Option<String>,
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
    message: String,
}

struct AppJson<T>(T);

impl ApiError {
    fn internal(_code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
        }
    }

    fn bad_request(_code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }

    fn unauthorized(_code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::UNAUTHORIZED,
            message: message.into(),
        }
    }

    fn forbidden(_code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::FORBIDDEN,
            message: message.into(),
        }
    }

    fn too_many_requests(_code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::TOO_MANY_REQUESTS,
            message: message.into(),
        }
    }
}

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        Self {
            status: rejection.status(),
            message: rejection.body_text(),
        }
    }
}

impl From<QueryRejection> for ApiError {
    fn from(rejection: QueryRejection) -> Self {
        Self {
            status: rejection.status(),
            message: rejection.body_text(),
        }
    }
}

impl From<crate::conversation_state::ConversationStateAccessError> for ApiError {
    fn from(error: crate::conversation_state::ConversationStateAccessError) -> Self {
        Self {
            status: error.status(),
            message: error.message().to_owned(),
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
        StatusCode::UNPROCESSABLE_ENTITY => WebFrameworkErrorKind::UnprocessableEntity,
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
            auth_profile: None,
            failed_stage: None,
            reason: None,
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
                message,
            },
            RuntimeError::ConversationNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
                message,
            },
            RuntimeError::ConversationBindingNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
                message,
            },
            RuntimeError::MessageNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
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
                message,
            },
            RuntimeError::PermissionDenied(message) => {
                Self::forbidden("conversation_permission_denied", message)
            }
            RuntimeError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                message,
            },
            RuntimeError::ReadCursorInvalid(message) => {
                Self::bad_request("read_cursor_invalid", message)
            }
            RuntimeError::Contract(error) => match error {
                sdkwork_im_contract_core::ContractError::Unavailable(message) => Self {
                    status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                    message,
                },
                sdkwork_im_contract_core::ContractError::Conflict(message) => Self {
                    status: axum::http::StatusCode::CONFLICT,
                    message,
                },
                sdkwork_im_contract_core::ContractError::UnsupportedCapability(message) => Self {
                    status: axum::http::StatusCode::NOT_IMPLEMENTED,
                    message,
                },
                sdkwork_im_contract_core::ContractError::Invalid(message) => Self {
                    status: axum::http::StatusCode::BAD_REQUEST,
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
            auth_profile: None,
            failed_stage: None,
            reason: None,
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

fn build_server_runtime_for_app_state()
-> Result<ConversationRuntime<ConversationCommitJournal>, RuntimeError> {
    super::journal_bootstrap::build_conversation_runtime_from_env().map_err(|error| {
        RuntimeError::Contract(im_platform_contracts::ContractError::Unavailable(error))
    })
}

fn build_test_runtime_for_app_state() -> ConversationRuntime<ConversationCommitJournal> {
    ConversationRuntime::new(ConversationCommitJournal::Memory(InMemoryJournal::default()))
        .with_welcome_state_store(Arc::new(InMemoryWelcomeStateStore::default()))
}

fn build_server_group_knowledgebase_for_app_state()
-> Result<Arc<GroupKnowledgebaseCoordinator>, RuntimeError> {
    let port = group_knowledgebase_port_for_server(
        super::knowledgebase_rpc_config::resolve_group_knowledgebase_rpc_port_from_env()?,
    );
    let id_generator =
        sdkwork_im_runtime_id::build_runtime_id_generator_blocking("conversation-knowledgebase");
    Ok(Arc::new(
        GroupKnowledgebaseCoordinator::with_production_store(port, id_generator)?,
    ))
}

fn group_knowledgebase_port_for_server(
    port: Option<Arc<dyn GroupKnowledgebasePort>>,
) -> Arc<dyn GroupKnowledgebasePort> {
    port.unwrap_or_else(|| Arc::new(UnavailableGroupKnowledgebasePort))
}

fn build_test_group_knowledgebase_for_app_state()
-> Result<Arc<GroupKnowledgebaseCoordinator>, RuntimeError> {
    let port = super::knowledgebase_rpc_config::resolve_group_knowledgebase_rpc_port_from_env()?
        .unwrap_or_else(|| Arc::new(UnavailableGroupKnowledgebasePort));
    let id_generator =
        sdkwork_im_runtime_id::build_runtime_id_generator_blocking("conversation-knowledgebase");
    Ok(Arc::new(
        GroupKnowledgebaseCoordinator::with_development_memory_store(port, id_generator)?,
    ))
}

/// Explicit local/test fixture. Server processes must call
/// [`bootstrap_conversation_app_state_from_env`] instead.
pub fn default_app_state() -> AppState {
    if !im_app_context::allows_header_only_app_context_fallback() {
        panic!(
            "default conversation app state with allow-all principals is forbidden in production; \
             call bootstrap_conversation_app_state_from_env() instead"
        );
    }
    let state = AppState {
        runtime: Arc::new(build_test_runtime_for_app_state()),
        principal_directory: Arc::new(AllowAllPrincipalDirectory),
        group_knowledgebase: build_test_group_knowledgebase_for_app_state()
            .expect("development group knowledgebase coordinator should initialize"),
        group_knowledgebase_outbox_relay_owner: Arc::new(GroupKnowledgebaseOutboxRelayOwner::new()),
        group_knowledgebase_launch_rate_limiter: GroupKnowledgebaseLaunchRateLimiter::from_env(),
        shared_channel_sync_rate_limiter: SharedChannelSyncRateLimiter::from_env(),
    };
    state.register_for_embedded_wiring();
    state
}

fn try_server_app_state_with_principal_directory(
    principal_directory: Arc<dyn PrincipalDirectory>,
) -> Result<AppState, RuntimeError> {
    let state = AppState {
        group_knowledgebase: build_server_group_knowledgebase_for_app_state()?,
        runtime: Arc::new(build_server_runtime_for_app_state()?),
        principal_directory,
        group_knowledgebase_outbox_relay_owner: Arc::new(GroupKnowledgebaseOutboxRelayOwner::new()),
        group_knowledgebase_launch_rate_limiter: GroupKnowledgebaseLaunchRateLimiter::from_env(),
        shared_channel_sync_rate_limiter: SharedChannelSyncRateLimiter::from_env(),
    };
    state.register_for_embedded_wiring();
    Ok(state)
}

pub fn app_state_with_principal_directory(
    principal_directory: Arc<dyn PrincipalDirectory>,
) -> AppState {
    if !im_app_context::allows_header_only_app_context_fallback() {
        panic!(
            "local conversation app state is forbidden in production; call bootstrap_conversation_app_state_from_env()"
        );
    }
    let state = AppState {
        group_knowledgebase: build_test_group_knowledgebase_for_app_state()
            .expect("development group knowledgebase coordinator should initialize"),
        runtime: Arc::new(build_test_runtime_for_app_state()),
        principal_directory,
        group_knowledgebase_outbox_relay_owner: Arc::new(GroupKnowledgebaseOutboxRelayOwner::new()),
        group_knowledgebase_launch_rate_limiter: GroupKnowledgebaseLaunchRateLimiter::from_env(),
        shared_channel_sync_rate_limiter: SharedChannelSyncRateLimiter::from_env(),
    };
    state.register_for_embedded_wiring();
    state
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
        return try_server_app_state_with_principal_directory(Arc::new(directory))
            .map_err(|error| error.to_string());
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
        return try_server_app_state_with_principal_directory(Arc::new(AllowAllPrincipalDirectory))
            .map_err(|error| error.to_string());
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
        apply_public_http_guardrails(build_business_router(app_state_with_principal_directory(
            principal_directory,
        ))),
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
            "/im/v3/api/chat/conversations/{conversation_id}/members/current",
            get(get_current_conversation_member),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/agents",
            get(get_conversation_agents).put(replace_conversation_agents),
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
            "/im/v3/api/chat/conversations/{conversation_id}/members/accept_invitation",
            post(accept_conversation_invitation),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/read_cursor",
            get(get_read_cursor).patch(update_read_cursor),
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
        .route(
            "/im/v3/api/chat/me/welcome/ensure",
            post(ensure_my_welcome_message),
        )
        .with_state(state)
}

/// App-API surface for the group knowledgebase capability. The caller-facing
/// app SDK owns these routes; IM's open API remains free of app-business
/// launch-ticket semantics.
pub fn build_group_knowledgebase_app_api_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/app/v3/api/chat/conversations/{conversation_id}/knowledgebase",
            get(get_group_knowledgebase).post(ensure_group_knowledgebase),
        )
        .route(
            "/app/v3/api/chat/conversations/{conversation_id}/knowledgebase/launch",
            post(launch_group_knowledgebase),
        )
        .route(
            "/app/v3/api/chat/conversations/{conversation_id}/archive",
            post(archive_group_conversation),
        )
        .with_state(state)
}

fn build_business_router(state: AppState) -> Router {
    Router::new()
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs))
        .merge(build_domain_api_router(state.clone()))
        .merge(build_group_knowledgebase_app_api_router(state))
}

fn build_app(state: AppState) -> Router {
    mount_im_infra_routes(build_business_router(state), im_service_router_config())
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
    let mut routes =
        extract_routes_from_function(http_source, "build_app", &[], &["/openapi.json", "/docs"])?;
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
            request.conversation_id.unwrap_or_default(),
            request.room_id,
            request.room_kind,
        )?)
    })();
    created_resource_response(&ctx, result)
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
    resource_response(&ctx, result)
}

async fn enter_room(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(room_id): Path<String>,
) -> Response {
    let result: ApiResult<EnterRoomResponse> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        let member = state.runtime.enter_room_from_auth_context(&auth, room_id)?;
        Ok(EnterRoomResponse { member })
    })();
    resource_response(&ctx, result)
}

async fn leave_room(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(room_id): Path<String>,
) -> Response {
    let result: ApiResult<EnterRoomResponse> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        let member = state.runtime.leave_room_from_auth_context(&auth, room_id)?;
        Ok(EnterRoomResponse { member })
    })();
    resource_response(&ctx, result)
}

async fn create_conversation(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    AppJson(request): AppJson<CreateConversationRequest>,
) -> Response {
    let initialize_knowledgebase = request.initialize_knowledgebase;
    let result: ApiResult<CreateConversationResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        let organization_id = organization_id_from_auth_context(&auth);
        let policy = request.conversation_policy()?;
        let conversation_type = request.conversation_type.trim();
        let requested_agent_assignments = request.agent_assignments.clone();
        let requested_member_user_ids = request.member_user_ids.clone().unwrap_or_default();
        if conversation_type.is_empty() {
            return Err(ApiProblem::from(ApiError::bad_request(
                "conversation_type_required",
                "conversationType is required",
            )));
        }
        if initialize_knowledgebase && !conversation_type.eq_ignore_ascii_case("group") {
            return Err(ApiProblem::from(ApiError::bad_request(
                "conversation_initialize_knowledgebase_group_only",
                "initializeKnowledgebase is only supported for group conversations",
            )));
        }
        // Group conversations use a server-derived canonical `g_` id seeded
        // from creator + group name + client request key. Direct and other
        // generic conversation types continue to accept a client-supplied id.
        let result = if conversation_type.eq_ignore_ascii_case("group") {
            let group_name = request
                .group_name
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .ok_or_else(|| {
                    ApiProblem::from(ApiError::bad_request(
                        "conversation_group_name_required",
                        "groupName is required for group conversations",
                    ))
                })?;
            let client_request_key = request
                .client_request_key
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .ok_or_else(|| {
                    ApiProblem::from(ApiError::bad_request(
                        "conversation_client_request_key_required",
                        "clientRequestKey is required for group conversations",
                    ))
                })?;
            let normalized_member_user_ids = super::creation::normalize_initial_member_user_ids(
                requested_member_user_ids,
                auth.actor_id.as_str(),
            )?;
            for member_user_id in &normalized_member_user_ids {
                if member_user_id.as_str() != auth.actor_id.as_str() {
                    ensure_active_http_principal(
                        &state,
                        auth.tenant_id.as_str(),
                        member_user_id,
                        "user",
                    )?;
                }
            }
            match requested_agent_assignments.clone() {
                Some(agent_assignments) => state
                    .runtime
                    .create_group_conversation_from_auth_context_with_members_and_agent_assignments_and_knowledgebase_initialization(
                        &auth,
                        group_name.to_owned(),
                        client_request_key.to_owned(),
                        normalized_member_user_ids,
                        agent_assignments,
                        initialize_knowledgebase,
                    )?,
                None => state
                    .runtime
                    .create_group_conversation_from_auth_context_with_members_and_knowledgebase_initialization(
                        &auth,
                        group_name.to_owned(),
                        client_request_key.to_owned(),
                        normalized_member_user_ids,
                        initialize_knowledgebase,
                    )?,
            }
        } else {
            if requested_agent_assignments.is_some() {
                return Err(ApiProblem::from(ApiError::bad_request(
                    "conversation_agent_assignments_group_only",
                    "agentAssignments are only supported for group conversations",
                )));
            }
            if !requested_member_user_ids.is_empty() {
                return Err(ApiProblem::from(ApiError::bad_request(
                    "conversation_member_user_ids_group_only",
                    "memberUserIds are only supported for group conversations",
                )));
            }
            let conversation_id = request
                .conversation_id
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .ok_or_else(|| {
                    ApiProblem::from(ApiError::bad_request(
                        "conversation_id_required",
                        "conversationId is required",
                    ))
                })?;
            state.runtime.create_conversation_from_auth_context(
                &auth,
                conversation_id.to_owned(),
                conversation_type.to_owned(),
            )?
        };
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
    let result = match result {
        Ok(mut created) if initialize_knowledgebase => {
            let conversation_id = created.conversation_id.clone();
            let initialization_status = match state
                .group_knowledgebase
                .ensure(state.runtime(), auth.clone(), conversation_id.clone())
                .await
            {
                Ok(GroupKnowledgebaseEnsureResult::Created(_))
                | Ok(GroupKnowledgebaseEnsureResult::Existing(_)) => {
                    GroupKnowledgebaseInitializationStatus::Active
                }
                Ok(GroupKnowledgebaseEnsureResult::Provisioning(_)) => {
                    GroupKnowledgebaseInitializationStatus::Provisioning
                }
                Err(error) => {
                    tracing::warn!(
                        conversation_id,
                        error = ?error,
                        "group was created but explicit knowledgebase initialization did not complete"
                    );
                    GroupKnowledgebaseInitializationStatus::Failed
                }
            };
            created.knowledgebase_initialization = Some(initialization_status);
            Ok(created)
        }
        result => result,
    };
    created_resource_response(&ctx, result)
}

async fn get_conversation_agents(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
) -> Response {
    let result: ApiResult<ConversationAgentAssignmentSet> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state
            .runtime
            .conversation_agent_assignments_snapshot_from_auth_context(
                &auth,
                conversation_id.as_str(),
            )?)
    })();
    resource_response(&ctx, result)
}

async fn replace_conversation_agents(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    AppJson(request): AppJson<ReplaceConversationAgentsRequest>,
) -> Response {
    let result: ApiResult<ConversationAgentAssignmentSet> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state
            .runtime
            .replace_conversation_agents_from_auth_context(
                &auth,
                conversation_id,
                request.expected_generation,
                request.agent_assignments,
            )?
            .assignments)
    })();
    resource_response(&ctx, result)
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
    created_resource_response(&ctx, result)
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
    created_resource_response(&ctx, result)
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
    created_resource_response(&ctx, result)
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
    created_resource_response(&ctx, result)
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
    created_resource_response(&ctx, result)
}

pub(crate) fn resolve_active_rpc_auth_context(
    state: &AppState,
    headers: &HeaderMap,
) -> Result<AppContext, ApiError> {
    let auth = resolve_app_context(headers).map_err(|value| ApiError {
        status: axum::http::StatusCode::UNAUTHORIZED,
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

fn require_im_app_context(ctx: &WebRequestContext) -> Result<AppContext, ApiError> {
    app_context_from_web_request(ctx).ok_or_else(|| {
        ApiError::unauthorized(
            "web_request_principal_required",
            "an authenticated WebRequestContext principal is required",
        )
    })
}

fn require_group_knowledgebase_http_context(
    ctx: &WebRequestContext,
) -> Result<AppContext, ApiError> {
    require_im_app_context(ctx)
}

fn require_normalized_idempotency_key(ctx: &WebRequestContext) -> Result<String, ApiError> {
    let value = ctx
        .idempotency_key()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            ApiError::bad_request(
                "idempotency_key_required",
                "a framework-normalized Idempotency-Key is required",
            )
        })?;
    let valid = (8..=128).contains(&value.len())
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-'));
    if !valid {
        return Err(ApiError::bad_request(
            "idempotency_key_invalid",
            "Idempotency-Key must contain 8 to 128 ASCII letters, digits, dots, underscores, colons, or hyphens",
        ));
    }
    Ok(value.to_owned())
}

fn record_group_knowledgebase_membership_change(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
    source_event_id: &str,
) -> Result<(), ApiError> {
    state
        .group_knowledgebase
        .record_membership_change(
            state.runtime.as_ref(),
            auth,
            conversation_id,
            source_event_id,
        )
        .map(|_| ())
        .map_err(ApiError::from)
}

fn map_blocking_join_error(error: tokio::task::JoinError) -> ApiProblem {
    ApiProblem::internal_server_error(format!(
        "conversation_runtime_blocking_join_failed: {error}"
    ))
}

/// Run journal/Redis/Postgres-backed runtime work off the Tokio async worker pool.
async fn run_blocking_conversation<F, T>(
    state: AppState,
    auth: AppContext,
    operation: F,
) -> ApiResult<T>
where
    F: FnOnce(AppState, AppContext) -> ApiResult<T> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(move || operation(state, auth))
        .await
        .map_err(map_blocking_join_error)?
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
            message,
        },
    }
}

fn validate_message_history_limit(limit: Option<usize>) -> Result<usize, ApiError> {
    normalize_message_history_limit(limit)
        .map_err(|message| ApiError::bad_request("limit_invalid", message))
}

fn validate_message_history_page_size(query: &SdkWorkCursorListQuery) -> Result<usize, ApiError> {
    let Some(page_size) = query.page_size else {
        return validate_message_history_limit(None);
    };
    if page_size < 1 {
        return Err(ApiError::bad_request(
            "limit_invalid",
            format!(
                "message history limit must be between 1 and {MESSAGE_HISTORY_MAX_LIMIT}: {page_size}"
            ),
        ));
    }
    validate_message_history_limit(Some(page_size as usize))
}

fn validate_member_list_page_size(query: &SdkWorkCursorListQuery) -> Result<usize, ApiError> {
    let Some(page_size) = query.page_size else {
        return normalize_member_list_limit(None)
            .map_err(|message| ApiError::bad_request("limit_invalid", message));
    };
    if page_size < 1 {
        return Err(ApiError::bad_request(
            "limit_invalid",
            format!(
                "conversation member list limit must be between 1 and {CONVERSATION_MEMBER_LIST_MAX_LIMIT}: {page_size}"
            ),
        ));
    }
    normalize_member_list_limit(Some(page_size as usize))
        .map_err(|message| ApiError::bad_request("limit_invalid", message))
}

fn query_key(raw_pair: &str) -> &str {
    raw_pair
        .split_once('=')
        .map(|(key, _)| key)
        .unwrap_or(raw_pair)
}

fn invalid_message_history_query(uri: &Uri) -> Option<String> {
    let query = uri.query()?;
    let mut seen = std::collections::BTreeSet::new();
    for raw_pair in query.split('&').filter(|pair| !pair.is_empty()) {
        let key = query_key(raw_pair);
        if !matches!(key, "cursor" | "page_size") {
            return Some(format!(
                "query parameter `{key}` is not supported; message history accepts only `cursor` and `page_size`"
            ));
        }
        if !seen.insert(key) {
            return Some(format!(
                "query parameter `{key}` must not be supplied more than once"
            ));
        }
    }
    None
}

fn invalid_parameter_response(ctx: &WebRequestContext, detail: impl Into<String>) -> Response {
    let trace_id = ctx.resolved_trace_id();
    let problem = SdkWorkProblemDetail::platform(
        SdkWorkResultCode::InvalidParameter,
        detail,
        trace_id.clone(),
    );
    let status = axum::http::StatusCode::from_u16(problem.status)
        .unwrap_or(axum::http::StatusCode::BAD_REQUEST);
    let mut response = (status, Json(problem)).into_response();
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/problem+json"),
    );
    if let Ok(value) = HeaderValue::from_str(trace_id.as_str())
        && let Ok(name) = HeaderName::from_bytes(SDKWORK_TRACE_ID_HEADER.as_bytes())
    {
        response.headers_mut().insert(name, value);
    }
    response
}

fn message_history_cursor_error_response(
    ctx: &WebRequestContext,
    error: super::message_history_cursor::MessageHistoryCursorError,
) -> Response {
    match error {
        super::message_history_cursor::MessageHistoryCursorError::Invalid => {
            invalid_parameter_response(ctx, "message history cursor is invalid")
        }
        super::message_history_cursor::MessageHistoryCursorError::Configuration(message) => {
            finish_api_json(
                ctx,
                Err::<(), ApiProblem>(ApiProblem::dependency_unavailable(message)),
            )
        }
    }
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
        let expected_request_key =
            shared_channel_sync_request_key(auth.tenant_id.as_str(), &request);
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

async fn get_group_knowledgebase(
    ctx: WebRequestContext,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
) -> Response {
    // retrieve 是只读查询，内部已经通过 require_active_member_from_auth_context
    // 完成"是否为该群活跃成员 + 非 Guest"的 ACL 校验。Organization 登录态要求
    // 是为 create/launch 签发 launch ticket 设计的，retrieve 不应被其拦截。
    let auth = match require_im_app_context(&ctx) {
        Ok(auth) => auth,
        Err(error) => {
            return resource_response::<GroupKnowledgebaseLinkView>(&ctx, Err(error.into()));
        }
    };
    let result = run_blocking_conversation(state, auth, move |state, auth| {
        ensure_active_http_auth_principal(&state, &auth)?;
        state
            .group_knowledgebase
            .retrieve(state.rpc_runtime(), &auth, conversation_id.as_str())
            .map_err(ApiProblem::from)
    })
    .await;
    resource_response(&ctx, result)
}

async fn ensure_group_knowledgebase(
    ctx: WebRequestContext,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    AppJson(_request): AppJson<CreateGroupKnowledgebaseCommandRequest>,
) -> Response {
    let auth = match require_group_knowledgebase_http_context(&ctx) {
        Ok(auth) => auth,
        Err(error) => {
            return resource_response::<GroupKnowledgebaseLinkView>(&ctx, Err(error.into()));
        }
    };
    if let Err(error) = ensure_active_http_auth_principal(&state, &auth)
        .and_then(|_| require_normalized_idempotency_key(&ctx).map(|_| ()))
    {
        return resource_response::<GroupKnowledgebaseLinkView>(&ctx, Err(error.into()));
    }
    let result = state
        .group_knowledgebase
        .ensure(state.runtime(), auth, conversation_id)
        .await
        .map_err(ApiProblem::from);
    match result {
        Ok(GroupKnowledgebaseEnsureResult::Created(view)) => {
            created_resource_response(&ctx, Ok(view))
        }
        Ok(GroupKnowledgebaseEnsureResult::Existing(view))
        | Ok(GroupKnowledgebaseEnsureResult::Provisioning(view)) => {
            resource_response(&ctx, Ok(view))
        }
        Err(error) => resource_response::<GroupKnowledgebaseLinkView>(&ctx, Err(error)),
    }
}

async fn launch_group_knowledgebase(
    ctx: WebRequestContext,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    AppJson(_request): AppJson<LaunchGroupKnowledgebaseCommandRequest>,
) -> Response {
    let auth = match require_group_knowledgebase_http_context(&ctx) {
        Ok(auth) => auth,
        Err(error) => {
            return no_store_resource_response::<GroupKnowledgebaseLaunchResponse>(
                &ctx,
                Err(error.into()),
            );
        }
    };
    let idempotency_key = match ensure_active_http_auth_principal(&state, &auth)
        .and_then(|_| require_normalized_idempotency_key(&ctx))
        .and_then(|idempotency_key| {
            if state
                .group_knowledgebase_launch_rate_limiter
                .try_acquire(&auth)
            {
                Ok(idempotency_key)
            } else {
                Err(ApiError::too_many_requests(
                    "group_knowledgebase_launch_rate_limited",
                    "group knowledgebase launch exceeded the per-principal rate limit",
                ))
            }
        }) {
        Ok(idempotency_key) => idempotency_key,
        Err(error) => {
            return no_store_resource_response::<GroupKnowledgebaseLaunchResponse>(
                &ctx,
                Err(error.into()),
            );
        }
    };
    let result = state
        .group_knowledgebase
        .launch(state.runtime(), auth, conversation_id, idempotency_key)
        .await
        .map(GroupKnowledgebaseLaunchResponse::from)
        .map_err(ApiProblem::from);
    no_store_resource_response(&ctx, result)
}

async fn archive_group_conversation(
    ctx: WebRequestContext,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    AppJson(_request): AppJson<ArchiveGroupConversationCommandRequest>,
) -> Response {
    let auth = match require_im_app_context(&ctx) {
        Ok(auth) => auth,
        Err(error) => {
            return finish_api_json::<ArchiveGroupConversationResponse>(&ctx, Err(error.into()));
        }
    };
    let idempotency_key = match ensure_active_http_auth_principal(&state, &auth)
        .and_then(|_| require_normalized_idempotency_key(&ctx))
    {
        Ok(value) => value,
        Err(error) => {
            return finish_api_json::<ArchiveGroupConversationResponse>(&ctx, Err(error.into()));
        }
    };
    let result = run_blocking_conversation(state, auth, move |state, auth| {
        let archived = state.runtime.archive_group_conversation_from_auth_context(
            &auth,
            conversation_id.clone(),
            idempotency_key,
        )?;
        let knowledgebase_archive_scheduled = state
            .group_knowledgebase
            .archive_after_group_conversation_archive(
                &auth,
                archived.conversation_id.as_str(),
                archived.event_id.as_str(),
            )?;
        Ok(ArchiveGroupConversationResponse::accepted(
            archived.conversation_id,
            archived.event_id,
            archived.archived_at,
            knowledgebase_archive_scheduled,
        ))
    })
    .await;
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
        Ok(state
            .runtime
            .accept_agent_handoff_from_auth_context(&auth, conversation_id)?)
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
        Ok(state
            .runtime
            .close_agent_handoff_from_auth_context(&auth, conversation_id)?)
    })();
    finish_api_json(&ctx, result)
}

async fn list_members(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    query: Result<Query<SdkWorkCursorListQuery>, QueryRejection>,
) -> Response {
    let result: ApiResult<ListMembersResponse> = (|| {
        let Query(query) = query.map_err(ApiError::from)?;
        ensure_active_http_auth_principal(&state, &auth)?;
        let page_size = validate_member_list_page_size(&query)?;
        state
            .runtime
            .list_members_window_from_auth_context(
                &auth,
                conversation_id.as_str(),
                Some(page_size),
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

async fn get_current_conversation_member(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
) -> Response {
    let result: ApiResult<ConversationMember> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state
            .runtime
            .require_active_member_from_auth_context(&auth, conversation_id.as_str())?)
    })();
    resource_response(&ctx, result)
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
        let member = state.runtime.add_member_from_auth_context(
            &auth,
            conversation_id.clone(),
            request.principal_id,
            request.principal_kind,
            request.role,
            request.attributes,
        )?;
        record_group_knowledgebase_membership_change(
            &state,
            &auth,
            conversation_id.as_str(),
            format!(
                "conversation.member.add:{}:{}",
                member.member_id, member.joined_at
            )
            .as_str(),
        )?;
        Ok(member)
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
        let member = state.runtime.remove_member_from_auth_context(
            &auth,
            conversation_id.clone(),
            request.member_id,
        )?;
        record_group_knowledgebase_membership_change(
            &state,
            &auth,
            conversation_id.as_str(),
            format!(
                "conversation.member.remove:{}:{}",
                member.member_id,
                member.removed_at.as_deref().unwrap_or_default()
            )
            .as_str(),
        )?;
        Ok(member)
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
        let transfer = state
            .runtime
            .transfer_conversation_owner_from_auth_context(
                &auth,
                conversation_id.clone(),
                request.member_id,
            )?;
        record_group_knowledgebase_membership_change(
            &state,
            &auth,
            conversation_id.as_str(),
            format!("conversation.member.transfer-owner:{}", transfer.event_id).as_str(),
        )?;
        Ok(transfer)
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
        let change = state
            .runtime
            .change_conversation_member_role_from_auth_context(
                &auth,
                conversation_id.clone(),
                request.member_id,
                request.role,
            )?;
        record_group_knowledgebase_membership_change(
            &state,
            &auth,
            conversation_id.as_str(),
            format!("conversation.member.change-role:{}", change.event_id).as_str(),
        )?;
        Ok(change)
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
        let member = state
            .runtime
            .leave_conversation_from_auth_context(&auth, conversation_id.clone())?;
        record_group_knowledgebase_membership_change(
            &state,
            &auth,
            conversation_id.as_str(),
            format!(
                "conversation.member.leave:{}:{}",
                member.member_id,
                member.removed_at.as_deref().unwrap_or_default()
            )
            .as_str(),
        )?;
        Ok(member)
    })();
    finish_api_json(&ctx, result)
}

async fn accept_conversation_invitation(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
) -> Response {
    let result: ApiResult<ConversationMember> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        let member = state
            .runtime
            .accept_conversation_invitation_from_auth_context(&auth, conversation_id.clone())?;
        record_group_knowledgebase_membership_change(
            &state,
            &auth,
            conversation_id.as_str(),
            format!(
                "conversation.member.accept-invitation:{}:{}",
                member.member_id, member.joined_at
            )
            .as_str(),
        )?;
        Ok(member)
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
        Ok(state
            .runtime
            .read_cursor_view_from_auth_context(&auth, conversation_id.as_str())?)
    })();
    resource_response(&ctx, result)
}

async fn list_messages(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    uri: Uri,
    query: Result<Query<MessageHistoryQuery>, QueryRejection>,
) -> Response {
    if let Some(detail) = invalid_message_history_query(&uri) {
        return invalid_parameter_response(&ctx, detail);
    }
    let Query(query) = match query {
        Ok(query) => query,
        Err(rejection) => {
            return invalid_parameter_response(&ctx, rejection.body_text());
        }
    };
    let limit = match validate_message_history_page_size(&query.paging) {
        Ok(limit) => limit,
        Err(error) => return invalid_parameter_response(&ctx, error.message),
    };
    let cursor_tenant_id = auth.tenant_id.clone();
    let cursor_organization_id = organization_id_from_auth_context(&auth);
    let cursor_conversation_id = conversation_id.clone();
    let cursor_scope = super::message_history_cursor::MessageHistoryCursorScope {
        tenant_id: cursor_tenant_id.as_str(),
        organization_id: cursor_organization_id.as_str(),
        conversation_id: cursor_conversation_id.as_str(),
    };
    let before_seq = match query.paging.cursor.as_deref() {
        Some(cursor) => {
            match super::message_history_cursor::decode_message_history_cursor(cursor, cursor_scope)
            {
                Ok(before_seq) => Some(before_seq),
                Err(error) => return message_history_cursor_error_response(&ctx, error),
            }
        }
        None => None,
    };
    let result = run_blocking_conversation(state, auth, move |state, auth| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state.runtime.list_messages_window_from_auth_context(
            &auth,
            conversation_id.as_str(),
            before_seq,
            limit,
        )?)
    })
    .await;
    let history = match result {
        Ok(history) => history,
        Err(problem) => {
            return finish_api_json::<ConversationMessageListResponse>(&ctx, Err(problem));
        }
    };
    let next_cursor = if history.page.page_info.has_more == Some(true) {
        let Some(next_before_seq) = history.next_before_seq else {
            return finish_api_json::<ConversationMessageListResponse>(
                &ctx,
                Err(ApiProblem::internal_server_error(
                    "message history page reported hasMore without a continuation position",
                )),
            );
        };
        match super::message_history_cursor::encode_message_history_cursor(
            cursor_scope,
            next_before_seq,
        ) {
            Ok(cursor) => Some(cursor),
            Err(error) => return message_history_cursor_error_response(&ctx, error),
        }
    } else {
        None
    };
    finish_api_json(
        &ctx,
        Ok(ConversationMessageListResponse::from_history(
            history,
            next_cursor,
        )),
    )
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

        Ok(state
            .runtime
            .read_cursor_view_from_auth_context(&auth, conversation_id.as_str())?)
    })();
    resource_response(&ctx, result)
}

async fn post_message(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    AppJson(request): AppJson<PostMessageRequest>,
) -> Response {
    let body = match build_message_body(
        request.summary,
        request.text,
        request.reply_to,
        request.parts,
        request.render_hints,
    ) {
        Ok(body) => body,
        Err(error) => {
            return finish_api_json::<SdkWorkResourceData<PostMessageResult>>(
                &ctx,
                Err(error.into()),
            );
        }
    };
    let command = PostMessageCommand::from_auth_context(
        &auth,
        conversation_id,
        request.client_msg_id,
        MessageType::Standard,
        body,
    );
    let result = run_blocking_conversation(state, auth, move |state, auth| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state.runtime.post_message(command)?)
    })
    .await;
    finish_api_response(
        &ctx,
        result.and_then(|item| created_json(&ctx, SdkWorkResourceData { item })),
    )
}

async fn publish_system_channel_message(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    AppJson(request): AppJson<PostMessageRequest>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<PostMessageResult>> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        let body = build_message_body(
            request.summary,
            request.text,
            request.reply_to,
            request.parts,
            request.render_hints,
        )?;

        let item = state.runtime.publish_system_channel_message(
            PublishSystemChannelMessageCommand::from_auth_context(
                &auth,
                conversation_id,
                request.client_msg_id,
                body,
            ),
        )?;
        Ok(SdkWorkResourceData { item })
    })();
    finish_api_json(&ctx, result)
}

async fn ensure_my_welcome_message(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<WelcomeEnsureView>> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        let outcome = state.runtime.ensure_user_welcome(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            auth.user_id.as_str(),
            None,
        )?;
        Ok(SdkWorkResourceData {
            item: WelcomeEnsureView::from(&outcome),
        })
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
    let result: ApiResult<SdkWorkResourceData<MessageMutationResult>> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        let body = build_message_body(
            request.summary,
            request.text,
            request.reply_to,
            request.parts,
            request.render_hints,
        )?;
        let item = state.runtime.edit_message(EditMessageCommand {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(&auth),
            message_id,
            editor: sender_from_auth_context(&auth),
            body,
            idempotency_key: request.idempotency_key,
        })?;
        Ok(SdkWorkResourceData { item })
    })();
    finish_api_json(&ctx, result)
}

async fn recall_message(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(message_id): Path<String>,
    AppJson(request): AppJson<RecallMessageRequest>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<MessageMutationResult>> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        let item = state.runtime.recall_message(RecallMessageCommand {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(&auth),
            message_id,
            recalled_by: sender_from_auth_context(&auth),
            idempotency_key: request.idempotency_key,
        })?;
        Ok(SdkWorkResourceData { item })
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

        Ok(state
            .runtime
            .add_message_reaction(AddMessageReactionCommand::from_auth_context(
                &auth,
                message_id,
                request.reaction_key,
            ))?)
    })();
    created_resource_response(&ctx, result)
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
            RemoveMessageReactionCommand::from_auth_context(
                &auth,
                message_id,
                request.reaction_key,
            ),
        )?)
    })();
    resource_response(&ctx, result)
}

async fn pin_message(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(message_id): Path<String>,
) -> Response {
    let result: ApiResult<MessagePinMutationResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state
            .runtime
            .pin_message(PinMessageCommand::from_auth_context(&auth, message_id))?)
    })();
    resource_response(&ctx, result)
}

async fn unpin_message(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(message_id): Path<String>,
) -> Response {
    let result: ApiResult<MessagePinMutationResult> = (|| {
        ensure_active_http_auth_principal(&state, &auth)?;
        Ok(state
            .runtime
            .unpin_message(UnpinMessageCommand::from_auth_context(&auth, message_id))?)
    })();
    resource_response(&ctx, result)
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
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use im_app_context::DualTokenRequestBuilderExt;
    use sdkwork_web_core::{
        ServerRequestId, WebApiSurface, WebAuthLevel, WebAuthMode, WebDeploymentMode,
        WebEnvironment, WebLoginScope, WebRequestPrincipal, WebSubjectType, WebTransportFacts,
    };
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

    fn request_context_with_normalized_idempotency_key(
        idempotency_key: Option<&str>,
    ) -> WebRequestContext {
        WebRequestContext {
            request_id: ServerRequestId("00000000-0000-4000-8000-000000000001".into()),
            api_surface: WebApiSurface::AppApi,
            auth_mode: WebAuthMode::DualToken,
            transport: WebTransportFacts {
                path: "/app/v3/api/chat/conversations/g-1/knowledgebase/launch".into(),
                method: "POST".into(),
                auth_token_present: true,
                access_token_present: true,
                api_key_present: false,
                ingress_token_present: false,
                oauth_bearer_present: false,
                agent_token_present: false,
            },
            principal: None,
            locale: None,
            client_kind: None,
            operation: None,
            trace_id: None,
            idempotency_key: idempotency_key.map(ToOwned::to_owned),
        }
    }

    fn organization_request_context(organization_id: Option<&str>) -> WebRequestContext {
        let mut context = request_context_with_normalized_idempotency_key(None);
        context.principal = Some(
            WebRequestPrincipal::builder()
                .tenant_id("100001")
                .organization_id(organization_id.map(ToOwned::to_owned))
                .login_scope(if organization_id.is_some_and(|value| value != "0") {
                    WebLoginScope::Organization
                } else {
                    WebLoginScope::Tenant
                })
                .user_id("42")
                .session_id(Some("session-42".to_owned()))
                .app_id("sdkwork-im-pc")
                .environment(WebEnvironment::Test)
                .deployment_mode(WebDeploymentMode::Saas)
                .auth_level(WebAuthLevel::Password)
                .data_scope(vec!["organization".to_owned()])
                .permission_scope(vec!["conversation.read".to_owned()])
                .subject_type(WebSubjectType::User)
                .build(),
        );
        context
    }

    #[tokio::test]
    async fn server_uses_unavailable_knowledgebase_port_when_dev_rpc_config_is_absent() {
        let port = group_knowledgebase_port_for_server(None);
        assert!(matches!(
            port.ensure_delivery_ready().await,
            Err(GroupKnowledgebasePortError::Unavailable)
        ));
    }

    #[tokio::test]
    async fn archive_group_response_uses_command_data_without_resource_item_wrapper() {
        let response = finish_api_json(
            &request_context_with_normalized_idempotency_key(Some("archive-command-1")),
            Ok(ArchiveGroupConversationResponse::accepted(
                "g-archive-1".into(),
                "evt_group_archived_1".into(),
                "2026-07-13T00:00:00Z".into(),
                true,
            )),
        );

        let status = response.status();
        let body = response
            .into_body()
            .collect()
            .await
            .expect("archive command response should be readable")
            .to_bytes();
        assert_eq!(
            status,
            StatusCode::OK,
            "archive command response: {}",
            String::from_utf8_lossy(&body)
        );
        let value: serde_json::Value =
            serde_json::from_slice(&body).expect("archive command response should be valid json");

        assert_eq!(value["code"], 0);
        assert_eq!(value["data"]["accepted"], true);
        assert_eq!(value["data"]["resourceId"], "g-archive-1");
        assert_eq!(value["data"]["status"], "archived");
        assert_eq!(value["data"]["archiveEventId"], "evt_group_archived_1");
        assert_eq!(value["data"]["archivedAt"], "2026-07-13T00:00:00Z");
        assert_eq!(value["data"]["knowledgebaseArchiveScheduled"], true);
        assert!(value["data"]["item"].is_null());
    }

    fn build_test_app_with_runtime_and_directory(
        runtime: Arc<ConversationRuntime<ConversationCommitJournal>>,
        principal_directory: Arc<dyn PrincipalDirectory>,
    ) -> Router {
        use axum::extract::Request;
        use axum::middleware::{Next, from_fn};

        async fn inject_test_auth_context(request: Request, next: Next) -> Response {
            let path = request.uri().path().to_owned();
            let method = request.method().as_str().to_owned();
            if let Ok(resolved) = im_app_context::resolve_app_context_for_request(
                request.headers(),
                path.as_str(),
                method.as_str(),
            ) {
                let mut request_context = resolved.web_request_context;
                request_context.idempotency_key = request
                    .headers()
                    .get("idempotency-key")
                    .and_then(|value| value.to_str().ok())
                    .map(ToOwned::to_owned);
                let mut request = request;
                request.extensions_mut().insert(request_context);
                request.extensions_mut().insert(resolved.app_context);
                return next.run(request).await;
            }
            next.run(request).await
        }

        let state = AppState {
            runtime,
            principal_directory,
            // Tests opt into the in-memory conversation_state explicitly. Production
            // composition remains PostgreSQL-backed and fail-closed.
            group_knowledgebase: Arc::new(GroupKnowledgebaseCoordinator::with_memory_store(
                Arc::new(UnavailableGroupKnowledgebasePort),
            )),
            group_knowledgebase_outbox_relay_owner: Arc::new(
                GroupKnowledgebaseOutboxRelayOwner::new(),
            ),
            group_knowledgebase_launch_rate_limiter: GroupKnowledgebaseLaunchRateLimiter::from_env(
            ),
            shared_channel_sync_rate_limiter: SharedChannelSyncRateLimiter::from_env(),
        };
        build_app(state).layer(from_fn(inject_test_auth_context))
    }

    fn seed_group_conversation_with_ghost_member(
        runtime: &ConversationRuntime<ConversationCommitJournal>,
        conversation_id: &str,
    ) -> String {
        let owner_auth = AppContext {
            tenant_id: "100001".into(),
            organization_id: "200001".to_owned(),
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
                organization_id: "200001".into(),
                conversation_id: conversation_id.into(),
                creator_id: "1".into(),
                conversation_type: "group".into(),
            })
            .expect("seed create conversation should succeed");
        runtime
            .add_member(AddConversationMemberCommand {
                tenant_id: "100001".into(),
                organization_id: "200001".into(),
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

    #[tokio::test]
    async fn archive_group_conversation_returns_a_command_envelope_over_app_api() {
        let runtime = Arc::new(ConversationRuntime::new(ConversationCommitJournal::Memory(
            InMemoryJournal::default(),
        )));
        let conversation_id = "g_archive_command_http";
        seed_group_conversation_with_ghost_member(runtime.as_ref(), conversation_id);
        let app = build_test_app_with_runtime_and_directory(
            runtime,
            Arc::new(StrictKnownPrincipalDirectory::new(&["1"])),
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!(
                        "/app/v3/api/chat/conversations/{conversation_id}/archive"
                    ))
                    .header("content-type", "application/json")
                    .header("idempotency-key", "archive-command-http-1")
                    .with_dual_token_context("100001", "1", "user", None, ["*"])
                    .with_dual_token_organization("200001")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .expect("archive command request should return a response");

        let status = response.status();
        let body = response
            .into_body()
            .collect()
            .await
            .expect("archive command response should be readable")
            .to_bytes();
        assert_eq!(
            status,
            StatusCode::OK,
            "archive command response: {}",
            String::from_utf8_lossy(&body)
        );
        let value: serde_json::Value =
            serde_json::from_slice(&body).expect("archive command response should be valid json");

        assert_eq!(value["code"], 0);
        assert_eq!(value["data"]["accepted"], true);
        assert_eq!(value["data"]["resourceId"], conversation_id);
        assert_eq!(value["data"]["status"], "archived");
        assert!(value["data"]["archiveEventId"].as_str().is_some());
        assert_eq!(value["data"]["knowledgebaseArchiveScheduled"], false);
        assert!(value["data"]["item"].is_null());
    }

    #[test]
    fn test_unix_epoch_millis_clamps_pre_epoch_time_to_zero() {
        let before_epoch = UNIX_EPOCH
            .checked_sub(Duration::from_millis(1))
            .expect("test pre-epoch timestamp should construct");
        assert_eq!(unix_epoch_millis(before_epoch), 0);
    }

    #[test]
    fn group_knowledgebase_uses_only_the_framework_normalized_idempotency_key() {
        let context =
            request_context_with_normalized_idempotency_key(Some("normalized-launch-key-1"));
        let mut ambiguous_headers = HeaderMap::new();
        ambiguous_headers.append("idempotency-key", HeaderValue::from_static("raw-key-a"));
        ambiguous_headers.append("idempotency-key", HeaderValue::from_static("raw-key-b"));

        assert_eq!(
            require_normalized_idempotency_key(&context).expect("normalized key"),
            "normalized-launch-key-1"
        );
        assert_eq!(
            ambiguous_headers.get_all("idempotency-key").iter().count(),
            2,
            "the test deliberately carries ambiguous raw headers, which the handler cannot read"
        );
    }

    #[test]
    fn group_knowledgebase_rejects_missing_or_malformed_normalized_idempotency_key() {
        assert!(
            require_normalized_idempotency_key(&request_context_with_normalized_idempotency_key(
                None,
            ))
            .is_err()
        );
        assert!(
            require_normalized_idempotency_key(&request_context_with_normalized_idempotency_key(
                Some("invalid key with spaces"),
            ))
            .is_err()
        );
    }

    #[test]
    fn group_knowledgebase_http_requires_only_an_authenticated_framework_context() {
        let tenant_auth =
            require_group_knowledgebase_http_context(&organization_request_context(Some("0")))
                .expect("tenant login context should project into AppContext");
        assert_eq!(tenant_auth.organization_id, "0");

        let organization_auth =
            require_group_knowledgebase_http_context(&organization_request_context(Some("200001")))
                .expect("organization login context should project into AppContext");
        assert_eq!(organization_auth.organization_id, "200001");

        assert!(
            require_group_knowledgebase_http_context(
                &request_context_with_normalized_idempotency_key(None)
            )
            .is_err()
        );
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
        assert!(
            value["detail"]
                .as_str()
                .expect("detail should be string")
                .contains("principal not found in directory")
        );
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
        assert!(
            value["detail"]
                .as_str()
                .expect("detail should be string")
                .contains("principal not found in directory")
        );
    }

    #[tokio::test]
    async fn test_list_messages_rejects_page_size_above_contract_over_http() {
        let runtime = Arc::new(ConversationRuntime::new(ConversationCommitJournal::Memory(
            InMemoryJournal::default(),
        )));
        let app = build_test_app_with_runtime_and_directory(
            runtime,
            Arc::new(StrictKnownPrincipalDirectory::new(&["1"])),
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(
                        "/im/v3/api/chat/conversations/c_history_limit_http/messages?page_size=201",
                    )
                    .with_dual_token_context("100001", "1", "user", None, ["*"])
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("oversized history page request should return response");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = response
            .into_body()
            .collect()
            .await
            .expect("body should collect")
            .to_bytes();
        let value: serde_json::Value =
            serde_json::from_slice(&body).expect("response should be valid json");
        assert!(
            value["detail"]
                .as_str()
                .expect("detail should be string")
                .contains("message history limit must be between 1 and 200: 201")
        );
    }

    #[tokio::test]
    async fn test_list_messages_rejects_non_standard_pagination_aliases_over_http() {
        for alias in ["pageSize", "limit", "page_no", "pageNo", "per_page", "size"] {
            let runtime = Arc::new(ConversationRuntime::new(ConversationCommitJournal::Memory(
                InMemoryJournal::default(),
            )));
            let app = build_test_app_with_runtime_and_directory(
                runtime,
                Arc::new(StrictKnownPrincipalDirectory::new(&["1"])),
            );

            let response = app
                .oneshot(
                    Request::builder()
                        .method("GET")
                        .uri(format!(
                            "/im/v3/api/chat/conversations/c_history_alias_http/messages?{alias}=20"
                        ))
                        .with_dual_token_context("100001", "1", "user", None, ["*"])
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .expect("non-standard history pagination alias request should return response");

            assert_eq!(
                response.status(),
                StatusCode::BAD_REQUEST,
                "alias {alias} should be rejected"
            );
            let body = response
                .into_body()
                .collect()
                .await
                .expect("body should collect")
                .to_bytes();
            let value: serde_json::Value =
                serde_json::from_slice(&body).expect("response should be valid json");
            assert_eq!(value["code"], 40003);
            assert!(
                value["detail"]
                    .as_str()
                    .expect("detail should be string")
                    .contains("accepts only `cursor` and `page_size`"),
                "alias {alias} should identify the canonical history parameters"
            );
        }
    }

    #[tokio::test]
    async fn test_create_agent_dialog_returns_created_resource_envelope_over_http() {
        let runtime = Arc::new(ConversationRuntime::new(ConversationCommitJournal::Memory(
            InMemoryJournal::default(),
        )));
        let app = build_test_app_with_runtime_and_directory(
            runtime,
            Arc::new(StrictKnownPrincipalDirectory::new(&["1"])),
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/chat/conversations/agent_dialogs")
                    .header("content-type", "application/json")
                    .with_dual_token_context("100001", "1", "user", None, ["*"])
                    .body(Body::from(r#"{"agentId":"agent.support"}"#))
                    .unwrap(),
            )
            .await
            .expect("create agent dialog request should return response");

        assert_eq!(response.status(), StatusCode::CREATED);
        let body = response
            .into_body()
            .collect()
            .await
            .expect("body should collect")
            .to_bytes();
        let value: serde_json::Value =
            serde_json::from_slice(&body).expect("response should be valid json");
        assert_eq!(value["code"], 0);
        assert!(
            value["data"]["item"]["conversationId"]
                .as_str()
                .expect("conversation id should be in data.item")
                .starts_with("a_")
        );
        assert!(
            value["data"]["conversationId"].is_null(),
            "create response must be nested under data.item, not flattened under data"
        );
    }
}
