use im_app_context::AppContext;
use im_domain_core::retention::retention_until_from_envelope;
use im_platform_contracts::{
    AgentAssignmentSource, AgentDispatchReplyCompletion, AgentIntegrationStore,
    AgentReplyCommitResult, CONVERSATION_AGGREGATE_PAGE_SIZE_MAX, ConversationAgentAssignmentItem,
    ConversationAggregateStore, ConversationMemberRecord, ConversationSeqAllocator, IdGenerator,
    MessageStore, NormalizedConversationBusinessBindingRecord, NormalizedConversationCommit,
    NormalizedConversationCurrentState, NormalizedConversationHandoffRecord,
    NormalizedConversationPolicyRecord, NormalizedConversationRecord, OutboxStore,
    ReadCursorRecord, RealtimeEventPublisher, ReplaceConversationAgentAssignments,
    RetentionScopeStore, StoredMessageMutation, StoredMessageMutationTarget,
    StoredMessagePinRecord, StoredMessageReactionRecord, StoredMessageRecord,
};
use sdkwork_im_contract_core::ContractError;
use sdkwork_im_contract_message::{
    CommitJournal, CommitJournalAggregateEventTypeQuery, CommitJournalAggregateScope,
    CommitJournalReplayCursor, CommitJournalReplayPage, CommitPosition,
};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::io::Write;
use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub use im_domain_core::conversation::{
    AgentHandoffStateView, ChangeAgentHandoffStatusView, ConversationBusinessBinding,
};
use im_domain_core::conversation::{
    ConversationAgentAssignment, ConversationAgentAssignmentError, ConversationAgentAssignmentSet,
    ConversationAgentAssignmentSource, ConversationAggregateState, ConversationLifecycleState,
    ConversationMember, ConversationPolicy, ConversationReadCursor, ConversationReadCursorView,
    ConversationRoster, LEGACY_GROUP_AGENT_DEFAULT_POLICY_ID,
    LEGACY_GROUP_AGENT_DEFAULT_POLICY_VERSION, MembershipRole, MembershipState,
    build_conversation_member_with_attributes, build_default_read_cursor,
    legacy_group_agent_assignment_set, member_episode_id, member_id,
};
use im_domain_core::media::{DriveReference, MediaResource, MediaSource};
use im_domain_core::message::{
    ContentPart, ConversationMessageLog, Message, MessageBody, MessageEdited, MessageLocatorIndex,
    MessagePinned, MessageReactionAdded, MessageReactionRemoved, MessageRecalled, MessageType,
    MessageUnpinned, ReactionActorIdentity, Sender,
};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
pub use im_platform_contracts::{
    AGENT_MENTION_DISPATCH_EVENT_TYPE, AGENT_MENTION_DISPATCH_OUTBOX_AGGREGATE_TYPE,
    AGENT_MENTION_DISPATCH_PAYLOAD_SCHEMA, AGENT_MENTION_DISPATCH_SCHEMA_VERSION,
    AgentMentionDispatchRequest, AgentMentionDispatchTarget,
};
use im_time::utc_now_rfc3339_millis;
use sdkwork_utils_rust::SdkWorkPageData;
use sdkwork_utils_rust::sha256_hash;
use serde::{Deserialize, Serialize};
use serde_json::{Value as JsonValue, json};

mod actor_inbox;
mod agent_dispatch;
mod agent_dispatch_worker;
mod agents;
mod binding;
#[cfg(test)]
mod bounded_soak_tests;
mod creation;
mod cursor_signing;
mod direct_message_access;
mod durable_conversation_event;
mod durable_message_mutation;
mod durable_message_post;
mod governance;
mod group_knowledgebase_outbox_relay;
mod group_lifecycle;
mod handoff;
pub mod http;
pub mod internal_rpc_dispatch;
mod journal_bootstrap;
mod knowledgebase;
mod knowledgebase_rpc_adapter;
mod knowledgebase_rpc_config;
mod member_list_cursor;
mod membership;
mod message_history_cursor;
mod message_realtime;
mod policy;
mod postgres_direct_message_gate;
mod room;
pub mod rpc_dispatch;
mod rpc_state_dispatch;
mod runtime_metrics;
mod support;

use self::group_lifecycle::ensure_conversation_write_allowed;
use self::message_realtime::ConversationRealtimeEvent;
use self::policy::MessagePostPolicy;
use self::runtime_metrics::ConversationRuntimeMetrics;
pub use self::runtime_metrics::ConversationRuntimeMetricsSnapshot;
use self::support::{
    build_agent_handoff_status_changed_envelope, build_conversation_policy_applied_envelope,
    build_member_envelope, build_member_role_changed_envelope, build_message_edited_envelope,
    build_message_pinned_envelope, build_message_reaction_added_envelope,
    build_message_reaction_removed_envelope, build_message_recalled_envelope,
    build_message_unpinned_envelope, build_owner_transfer_envelope, build_read_cursor_envelope,
    conversation_business_scope_key, conversation_retention_class, conversation_scope_key,
    conversation_timestamp, decode_conversation_scope_key, encode_conversation_key_segments,
    event_id_component,
    next_member_episode, resolve_active_member, resolve_active_member_id,
    resolve_active_member_id_with_kind, resolve_active_member_with_kind, upsert_member,
    upsert_read_cursor, upsert_roster_member,
};
pub use agent_dispatch_worker::{
    AgentDispatchSource, AgentDispatchSourceLoader, AgentDispatchWorker, AgentDispatchWorkerConfig,
    AgentDispatchWorkerHandle, AgentDispatchWorkerOutcome, AgentReplyCommitter,
    ConversationRuntimeAgentReplyCommitter, MessageStoreAgentDispatchSourceLoader,
    resolve_agent_dispatch_worker_id, spawn_agent_dispatch_worker,
};
pub use direct_message_access::DirectMessageAccessGate;
pub use durable_conversation_event::DurableConversationEventWriter;
pub use durable_message_mutation::DurableMessageMutationWriter;
pub use durable_message_post::DurableMessagePostWriter;
pub use group_knowledgebase_outbox_relay::{
    GroupKnowledgebaseOutboxRelayHandle, spawn_group_knowledgebase_outbox_relay,
};
pub use group_lifecycle::{
    ArchiveGroupConversationCommand, ArchiveGroupConversationResult,
    ConversationGroupArchivedPayload,
};
pub use http::{
    PrincipalDirectory, PrincipalDirectoryError, StaticPrincipalDirectory,
    bootstrap_conversation_app_state_from_env, build_default_app,
    build_default_app_with_principal_directory, build_public_app,
    build_public_app_with_allow_all_principals, build_public_app_with_principal_directory,
};
pub use journal_bootstrap::{
    ConversationCommitJournal, build_conversation_runtime_from_env,
    resolve_conversation_commit_journal_from_env,
};
pub use knowledgebase::{
    ArchiveGroupKnowledgebaseRequest, ConsumedGroupKnowledgebaseLaunchTicket,
    EnsureGroupKnowledgebaseRequest, EnsuredGroupKnowledgebase,
    GROUP_KNOWLEDGEBASE_ARCHIVE_EVENT_TYPE, GROUP_KNOWLEDGEBASE_MEMBERSHIP_SYNC_EVENT_TYPE,
    GROUP_KNOWLEDGEBASE_OUTBOX_AGGREGATE_TYPE, GroupKnowledgebaseCoordinator,
    GroupKnowledgebaseEnsureResult, GroupKnowledgebaseLaunchResponse,
    GroupKnowledgebaseLaunchResult, GroupKnowledgebaseLaunchView, GroupKnowledgebaseLinkView,
    GroupKnowledgebaseMembership, GroupKnowledgebaseOutboxOperation,
    GroupKnowledgebaseOutboxPayload, GroupKnowledgebasePort, GroupKnowledgebasePortError,
    GroupKnowledgebaseScope, SynchronizeGroupKnowledgebaseMembersRequest,
    UnavailableGroupKnowledgebasePort,
};
pub use membership::MessageHistoryReadRequest;

const CONVERSATION_MAX_ID_BYTES: usize = 256;
const CONVERSATION_MAX_KIND_BYTES: usize = 64;
const CONVERSATION_MAX_POLICY_VERSION_BYTES: usize = 128;
const CONVERSATION_MAX_HISTORY_VISIBILITY_BYTES: usize = 32;
const CONVERSATION_MAX_RETENTION_POLICY_REF_BYTES: usize = 256;
const CONVERSATION_MAX_CAPABILITY_FLAG_BYTES: usize = 128;
const CONVERSATION_MAX_CAPABILITY_FLAGS_TOTAL_BYTES: usize = 16 * 1024;
const CONVERSATION_MAX_MEMBER_ATTRIBUTES_BYTES: usize = 64 * 1024;
const CONVERSATION_MAX_SENDER_METADATA_BYTES: usize = 64 * 1024;
const MESSAGE_RENDER_HINTS_MAX_BYTES: usize = 64 * 1024;
const CONVERSATION_MAX_REASON_BYTES: usize = 8 * 1024;
const CONVERSATION_MAX_REQUEST_KEY_BYTES: usize = 2048;
const MESSAGE_CLIENT_MSG_ID_MAX_BYTES: usize = 256;
const MESSAGE_MENTION_DISPLAY_TEXT_MAX_CHARACTERS: usize = 512;
const MESSAGE_REACTION_KEY_MAX_BYTES: usize = 128;
const MESSAGE_BODY_MAX_BYTES: usize = 512 * 1024;
const MESSAGE_HISTORY_DEFAULT_LIMIT: usize = 20;
const MESSAGE_HISTORY_MAX_LIMIT: usize = 200;
const CONVERSATION_MEMBER_LIST_DEFAULT_LIMIT: usize = 20;
pub(super) const CONVERSATION_MEMBER_LIST_MAX_LIMIT: usize = CONVERSATION_AGGREGATE_PAGE_SIZE_MAX;
const CONVERSATION_MAX_INITIAL_MEMBER_COUNT: usize = 200;
const CONVERSATION_CREATE_DELIVERY_PROOF_VERSION: &str = "conversation.create.delivery-proof.v1";
const CONVERSATION_MESSAGE_DELIVERY_PROOF_VERSION: &str = "conversation.message.delivery-proof.v1";
const CONVERSATION_MAX_IN_MEMORY_DEFAULT: usize = 10_000;
const CONVERSATION_CACHE_MAX_BYTES_DEFAULT: usize = 512 * 1024 * 1024;
const CONVERSATION_IDLE_EVICTION_TARGET_RATIO: f64 = 0.8;
const CONVERSATION_MAX_IN_MEMORY_ENV: &str = "SDKWORK_IM_CONVERSATION_MAX_IN_MEMORY";
const CONVERSATION_CACHE_MAX_BYTES_ENV: &str = "SDKWORK_IM_CONVERSATION_CACHE_MAX_BYTES";
const CONVERSATION_STATE_FIXED_OVERHEAD_BYTES: usize = 64 * 1024;
const CONVERSATION_IDEMPOTENCY_REPLAY_MAX_ENTRIES: usize = 1_024;
const CONVERSATION_IDEMPOTENCY_REPLAY_MAX_BYTES: usize = 8 * 1024 * 1024;

fn normalize_message_history_limit(limit: Option<usize>) -> Result<usize, String> {
    let limit = limit.unwrap_or(MESSAGE_HISTORY_DEFAULT_LIMIT);
    if limit == 0 || limit > MESSAGE_HISTORY_MAX_LIMIT {
        return Err(format!(
            "message history limit must be between 1 and {MESSAGE_HISTORY_MAX_LIMIT}: {limit}"
        ));
    }
    Ok(limit)
}

fn normalize_member_list_limit(limit: Option<usize>) -> Result<usize, String> {
    let limit = limit.unwrap_or(CONVERSATION_MEMBER_LIST_DEFAULT_LIMIT);
    if limit == 0 || limit > CONVERSATION_MEMBER_LIST_MAX_LIMIT {
        return Err(format!(
            "conversation member list limit must be between 1 and {CONVERSATION_MEMBER_LIST_MAX_LIMIT}: {limit}"
        ));
    }
    Ok(limit)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConversationCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub creator_id: String,
    pub conversation_type: String,
}

/// Command for creating a group conversation with a server-derived canonical
/// `g_` id. Groups have no natural deterministic pair key, so the canonical id
/// seeds from the creator identity, the group display name at creation, and a
/// client-supplied request key that guarantees idempotent retries.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGroupConversationCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub creator_id: String,
    pub group_name: String,
    pub client_request_key: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceConversationAgentsCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub replaced_by: String,
    pub expected_generation: u64,
    pub agents: Vec<ConversationAgentAssignment>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceConversationAgentsResult {
    pub event_id: String,
    pub previous_generation: u64,
    pub assignments: ConversationAgentAssignmentSet,
    pub replaced_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAgentDialogCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub requester_id: String,
    pub agent_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAgentHandoffCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub source_id: String,
    pub target_id: String,
    pub target_kind: String,
    pub handoff_session_id: String,
    pub handoff_reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSystemChannelCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub requester_id: String,
    pub subscriber_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateThreadConversationCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub parent_conversation_id: String,
    pub root_message_id: String,
    pub creator_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoomCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub room_id: String,
    pub room_kind: String,
    pub creator_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnterRoomCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub room_id: String,
    pub principal_id: String,
    pub principal_kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaveRoomCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub room_id: String,
    pub principal_id: String,
    pub principal_kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomView {
    pub room_id: String,
    pub room_kind: String,
    pub conversation_id: String,
    pub active_member_count: usize,
    pub max_members: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BindDirectChatConversationCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub direct_chat_id: String,
    pub left_actor_id: String,
    pub left_actor_kind: String,
    pub right_actor_id: String,
    pub right_actor_kind: String,
    pub bound_by: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncSharedChannelLinkedMemberCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub shared_channel_policy_id: String,
    pub external_connection_id: String,
    pub local_actor_id: String,
    pub local_actor_kind: String,
    pub external_member_id: String,
    pub synced_by: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncSharedChannelLinkedMemberStatus {
    Applied,
    AlreadyLinked,
    Replayed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncSharedChannelLinkedMemberResult {
    pub status: SyncSharedChannelLinkedMemberStatus,
    pub member: ConversationMember,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptAgentHandoffCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub accepted_by: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolveAgentHandoffCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub resolved_by: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseAgentHandoffCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub closed_by: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateConversationDeliveryStatus {
    Applied,
    Replayed,
}

impl CreateConversationDeliveryStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Applied => "applied",
            Self::Replayed => "replayed",
        }
    }
}

/// The outcome of an explicitly requested group Knowledgebase initialization.
///
/// This field is intentionally absent from ordinary conversation creation
/// responses. A group is durably created before an optional remote
/// Knowledgebase provisioning attempt, so a transient provider failure must
/// not turn the successful group creation into a misleading HTTP error.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupKnowledgebaseInitializationStatus {
    Active,
    Provisioning,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConversationResult {
    pub conversation_id: String,
    pub event_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<CreateConversationDeliveryStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledgebase_initialization: Option<GroupKnowledgebaseInitializationStatus>,
}

impl CreateConversationResult {
    pub fn new(conversation_id: String, event_id: String) -> Self {
        Self {
            conversation_id,
            event_id,
            request_key: None,
            delivery_status: None,
            proof_version: None,
            knowledgebase_initialization: None,
        }
    }

    pub fn applied_with_request_key(
        conversation_id: String,
        event_id: String,
        request_key: String,
    ) -> Self {
        Self {
            conversation_id,
            event_id,
            request_key: Some(request_key),
            delivery_status: Some(CreateConversationDeliveryStatus::Applied),
            proof_version: Some(CONVERSATION_CREATE_DELIVERY_PROOF_VERSION.into()),
            knowledgebase_initialization: None,
        }
    }

    pub fn replayed_with_request_key(
        conversation_id: String,
        event_id: String,
        request_key: String,
    ) -> Self {
        Self {
            conversation_id,
            event_id,
            request_key: Some(request_key),
            delivery_status: Some(CreateConversationDeliveryStatus::Replayed),
            proof_version: Some(CONVERSATION_CREATE_DELIVERY_PROOF_VERSION.into()),
            knowledgebase_initialization: None,
        }
    }

    pub fn is_applied(&self) -> bool {
        !matches!(
            self.delivery_status,
            Some(CreateConversationDeliveryStatus::Replayed)
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct GenericConversationCreateReplayRecord {
    creator_id: String,
    creator_kind: String,
    requested_kind: String,
    initial_member_user_ids: Vec<String>,
    initial_agent_assignments: Option<Vec<ConversationAgentAssignment>>,
    knowledgebase_initialization_requested: bool,
    event_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AgentDialogCreateReplayRecord {
    requester_id: String,
    requester_kind: String,
    agent_id: String,
    event_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SystemChannelCreateReplayRecord {
    requester_id: String,
    requester_kind: String,
    subscriber_id: String,
    event_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AgentHandoffCreateReplayRecord {
    source_id: String,
    source_kind: String,
    target_id: String,
    target_kind: String,
    handoff_session_id: String,
    handoff_reason: Option<String>,
    event_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ThreadConversationCreateReplayRecord {
    creator_id: String,
    creator_kind: String,
    parent_conversation_id: String,
    root_message_id: String,
    event_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RoomCreateReplayRecord {
    creator_id: String,
    creator_kind: String,
    room_id: String,
    room_kind: String,
    event_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DirectChatBindingReplayRecord {
    bound_by: String,
    binder_kind: String,
    direct_chat_id: String,
    anchor_actor_id: String,
    anchor_actor_kind: String,
    peer_actor_id: String,
    peer_actor_kind: String,
    event_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentHandoffStatusChangedPayload {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub previous_status: String,
    pub current_status: String,
    pub changed_by: ChangeAgentHandoffStatusView,
    pub changed_at: String,
    pub state: AgentHandoffStateView,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddConversationMemberCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub principal_id: String,
    pub principal_kind: String,
    pub role: MembershipRole,
    pub invited_by: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveConversationMemberCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub member_id: String,
    pub removed_by: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaveConversationCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub principal_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferConversationOwnerCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub target_member_id: String,
    pub transferred_by: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferConversationOwnerPayload {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub previous_owner: ConversationMember,
    pub new_owner: ConversationMember,
    pub transferred_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferConversationOwnerResult {
    pub event_id: String,
    pub transferred_at: String,
    pub previous_owner: ConversationMember,
    pub new_owner: ConversationMember,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConversationMemberRoleCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub target_member_id: String,
    pub new_role: MembershipRole,
    pub changed_by: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConversationMemberRolePayload {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub previous_member: ConversationMember,
    pub updated_member: ConversationMember,
    pub changed_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConversationMemberRoleResult {
    pub event_id: String,
    pub changed_at: String,
    pub previous_member: ConversationMember,
    pub updated_member: ConversationMember,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReadCursorCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub principal_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    pub read_seq: u64,
    pub last_read_message_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyConversationPolicyCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub applied_by: String,
    pub policy: ConversationPolicy,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMessageCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub sender: Sender,
    pub client_msg_id: Option<String>,
    pub message_type: MessageType,
    pub body: MessageBody,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishSystemChannelMessageCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub conversation_id: String,
    pub publisher: Sender,
    pub client_msg_id: Option<String>,
    pub body: MessageBody,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostMessageDeliveryStatus {
    Applied,
    Replayed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMessageResult {
    pub message_id: String,
    pub message_seq: u64,
    pub event_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_key: Option<String>,
    pub delivery_status: PostMessageDeliveryStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_version: Option<String>,
}

impl PostMessageResult {
    fn applied(
        message_id: String,
        message_seq: u64,
        event_id: String,
        request_key: Option<String>,
    ) -> Self {
        Self {
            message_id,
            message_seq,
            event_id,
            proof_version: request_key
                .as_ref()
                .map(|_| CONVERSATION_MESSAGE_DELIVERY_PROOF_VERSION.into()),
            request_key,
            delivery_status: PostMessageDeliveryStatus::Applied,
        }
    }

    fn replayed(
        message_id: String,
        message_seq: u64,
        event_id: String,
        request_key: Option<String>,
    ) -> Self {
        Self {
            message_id,
            message_seq,
            event_id,
            proof_version: request_key
                .as_ref()
                .map(|_| CONVERSATION_MESSAGE_DELIVERY_PROOF_VERSION.into()),
            request_key,
            delivery_status: PostMessageDeliveryStatus::Replayed,
        }
    }

    pub fn is_applied(&self) -> bool {
        self.delivery_status == PostMessageDeliveryStatus::Applied
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PostedMessageReplayRecord {
    sender_id: String,
    sender_kind: String,
    message_type: MessageType,
    body: MessageBody,
    message_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MessageMutationReplayRecord {
    result: MessageMutationResult,
}

trait ReplayCacheEntrySize {
    fn estimated_cache_bytes(&self) -> usize;
}

impl ReplayCacheEntrySize for PostedMessageReplayRecord {
    fn estimated_cache_bytes(&self) -> usize {
        self.sender_id
            .len()
            .saturating_add(self.sender_kind.len())
            .saturating_add(self.message_id.len())
            .saturating_add(estimated_json_bytes(&self.body))
            .saturating_add(std::mem::size_of::<MessageType>())
    }
}

impl ReplayCacheEntrySize for MessageMutationReplayRecord {
    fn estimated_cache_bytes(&self) -> usize {
        self.result
            .conversation_id
            .len()
            .saturating_add(self.result.message_id.len())
            .saturating_add(self.result.event_id.len())
            .saturating_add(std::mem::size_of::<u64>())
    }
}

#[derive(Default)]
struct SerializedSizeCounter {
    bytes: usize,
}

impl Write for SerializedSizeCounter {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.bytes = self.bytes.saturating_add(buffer.len());
        Ok(buffer.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn estimated_json_bytes(value: &impl serde::Serialize) -> usize {
    let mut counter = SerializedSizeCounter::default();
    serde_json::to_writer(&mut counter, value)
        .map(|()| counter.bytes)
        .unwrap_or(MESSAGE_BODY_MAX_BYTES)
}

#[derive(Clone)]
struct BoundedReplayCache<V> {
    entries: HashMap<String, V>,
    insertion_order: VecDeque<String>,
    entry_bytes: HashMap<String, usize>,
    cached_bytes: usize,
}

impl<V> Default for BoundedReplayCache<V> {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
            insertion_order: VecDeque::new(),
            entry_bytes: HashMap::new(),
            cached_bytes: 0,
        }
    }
}

impl<V> BoundedReplayCache<V> {
    fn get(&self, key: &str) -> Option<&V> {
        self.entries.get(key)
    }

    fn estimated_heap_bytes(&self) -> usize {
        const REPLAY_CACHE_INDEX_OVERHEAD_BYTES: usize = 192;
        std::mem::size_of::<Self>()
            .saturating_add(self.cached_bytes)
            .saturating_add(
                self.entries
                    .len()
                    .saturating_mul(REPLAY_CACHE_INDEX_OVERHEAD_BYTES),
            )
    }

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn cached_bytes(&self) -> usize {
        self.cached_bytes
    }
}

impl<V> BoundedReplayCache<V>
where
    V: ReplayCacheEntrySize,
{
    fn insert(&mut self, key: String, value: V) -> Option<V> {
        let estimated_bytes = key.len().saturating_add(value.estimated_cache_bytes());
        let previous = self.entries.remove(key.as_str());
        if previous.is_some() {
            self.insertion_order.retain(|queued| queued != &key);
            if let Some(previous_bytes) = self.entry_bytes.remove(key.as_str()) {
                self.cached_bytes = self.cached_bytes.saturating_sub(previous_bytes);
            }
        }
        if estimated_bytes > CONVERSATION_IDEMPOTENCY_REPLAY_MAX_BYTES {
            return previous;
        }
        while self.entries.len() >= CONVERSATION_IDEMPOTENCY_REPLAY_MAX_ENTRIES
            || self.cached_bytes.saturating_add(estimated_bytes)
                > CONVERSATION_IDEMPOTENCY_REPLAY_MAX_BYTES
        {
            let Some(oldest_key) = self.insertion_order.pop_front() else {
                break;
            };
            self.entries.remove(oldest_key.as_str());
            if let Some(oldest_bytes) = self.entry_bytes.remove(oldest_key.as_str()) {
                self.cached_bytes = self.cached_bytes.saturating_sub(oldest_bytes);
            }
        }
        self.insertion_order.push_back(key.clone());
        self.cached_bytes = self.cached_bytes.saturating_add(estimated_bytes);
        self.entry_bytes.insert(key.clone(), estimated_bytes);
        self.entries.insert(key, value);
        previous
    }
}

fn message_mutation_request_key(sender: &Sender, idempotency_key: &str) -> String {
    format!(
        "{}:{}:{}",
        sender.kind.as_str(),
        sender.id.as_str(),
        idempotency_key.trim()
    )
}

#[allow(clippy::large_enum_variant)]
enum PostMessageMutation {
    Applied {
        result: PostMessageResult,
        message: Message,
        evicted_message_ids: Vec<String>,
    },
    Replayed(PostMessageResult),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditMessageCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub message_id: String,
    pub editor: Sender,
    pub body: MessageBody,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecallMessageCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub message_id: String,
    pub recalled_by: Sender,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMessageReactionCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub message_id: String,
    pub reaction_key: String,
    pub reacted_by: Sender,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveMessageReactionCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub message_id: String,
    pub reaction_key: String,
    pub removed_by: Sender,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PinMessageCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub message_id: String,
    pub pinned_by: Sender,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnpinMessageCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub message_id: String,
    pub unpinned_by: Sender,
}

pub(super) fn organization_id_from_auth_context(auth: &AppContext) -> String {
    im_domain_events::normalize_commit_organization_id(auth.organization_id.as_str())
}

pub(super) fn default_organization_id() -> String {
    "0".to_owned()
}

pub fn default_post_message_organization_id() -> String {
    default_organization_id()
}

impl CreateConversationCommand {
    pub fn from_auth_context(
        auth: &AppContext,
        conversation_id: String,
        conversation_type: String,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            creator_id: auth.actor_id.clone(),
            conversation_type,
        }
    }
}

impl CreateGroupConversationCommand {
    pub fn from_auth_context(
        auth: &AppContext,
        group_name: String,
        client_request_key: String,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            creator_id: auth.actor_id.clone(),
            group_name,
            client_request_key,
        }
    }
}

impl CreateAgentDialogCommand {
    pub fn from_auth_context(auth: &AppContext, conversation_id: String, agent_id: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            requester_id: auth.actor_id.clone(),
            agent_id,
        }
    }
}

impl CreateAgentHandoffCommand {
    pub fn from_auth_context(
        auth: &AppContext,
        conversation_id: String,
        target_id: String,
        target_kind: String,
        handoff_session_id: String,
        handoff_reason: Option<String>,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            source_id: auth.actor_id.clone(),
            target_id,
            target_kind,
            handoff_session_id,
            handoff_reason,
        }
    }
}

impl CreateSystemChannelCommand {
    pub fn from_auth_context(
        auth: &AppContext,
        conversation_id: String,
        subscriber_id: String,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            requester_id: auth.actor_id.clone(),
            subscriber_id,
        }
    }
}

impl CreateThreadConversationCommand {
    pub fn from_auth_context(
        auth: &AppContext,
        conversation_id: String,
        parent_conversation_id: String,
        root_message_id: String,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            parent_conversation_id,
            root_message_id,
            creator_id: auth.actor_id.clone(),
        }
    }
}

impl CreateRoomCommand {
    pub fn from_auth_context(
        auth: &AppContext,
        conversation_id: String,
        room_id: String,
        room_kind: String,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            room_id,
            room_kind,
            creator_id: auth.actor_id.clone(),
        }
    }
}

impl EnterRoomCommand {
    pub fn from_auth_context(auth: &AppContext, room_id: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            room_id,
            principal_id: auth.actor_id.clone(),
            principal_kind: auth.actor_kind.clone(),
        }
    }
}

impl LeaveRoomCommand {
    pub fn from_auth_context(auth: &AppContext, room_id: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            room_id,
            principal_id: auth.actor_id.clone(),
            principal_kind: auth.actor_kind.clone(),
        }
    }
}

impl BindDirectChatConversationCommand {
    pub fn from_auth_context(
        auth: &AppContext,
        conversation_id: String,
        direct_chat_id: String,
        left_actor_id: String,
        left_actor_kind: String,
        right_actor_id: String,
        right_actor_kind: String,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            direct_chat_id,
            left_actor_id,
            left_actor_kind,
            right_actor_id,
            right_actor_kind,
            bound_by: auth.actor_id.clone(),
        }
    }
}

impl SyncSharedChannelLinkedMemberCommand {
    pub fn from_auth_context(
        auth: &AppContext,
        conversation_id: String,
        shared_channel_policy_id: String,
        external_connection_id: String,
        local_actor_id: String,
        local_actor_kind: String,
        external_member_id: String,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            shared_channel_policy_id,
            external_connection_id,
            local_actor_id,
            local_actor_kind,
            external_member_id,
            synced_by: auth.actor_id.clone(),
        }
    }
}

impl AcceptAgentHandoffCommand {
    pub fn from_auth_context(auth: &AppContext, conversation_id: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            accepted_by: auth.actor_id.clone(),
        }
    }
}

impl ResolveAgentHandoffCommand {
    pub fn from_auth_context(auth: &AppContext, conversation_id: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            resolved_by: auth.actor_id.clone(),
        }
    }
}

impl CloseAgentHandoffCommand {
    pub fn from_auth_context(auth: &AppContext, conversation_id: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            closed_by: auth.actor_id.clone(),
        }
    }
}

impl AddConversationMemberCommand {
    pub fn from_auth_context(
        auth: &AppContext,
        conversation_id: String,
        principal_id: String,
        principal_kind: String,
        role: MembershipRole,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            principal_id,
            principal_kind,
            role,
            invited_by: auth.actor_id.clone(),
        }
    }
}

impl RemoveConversationMemberCommand {
    pub fn from_auth_context(
        auth: &AppContext,
        conversation_id: String,
        member_id: String,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            member_id,
            removed_by: auth.actor_id.clone(),
        }
    }
}

impl LeaveConversationCommand {
    pub fn from_auth_context(auth: &AppContext, conversation_id: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            principal_id: auth.actor_id.clone(),
        }
    }
}

impl TransferConversationOwnerCommand {
    pub fn from_auth_context(
        auth: &AppContext,
        conversation_id: String,
        target_member_id: String,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            target_member_id,
            transferred_by: auth.actor_id.clone(),
        }
    }
}

impl ChangeConversationMemberRoleCommand {
    pub fn from_auth_context(
        auth: &AppContext,
        conversation_id: String,
        target_member_id: String,
        new_role: MembershipRole,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            target_member_id,
            new_role,
            changed_by: auth.actor_id.clone(),
        }
    }
}

impl UpdateReadCursorCommand {
    pub fn from_auth_context(
        auth: &AppContext,
        conversation_id: String,
        read_seq: u64,
        last_read_message_id: Option<String>,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            principal_id: auth.actor_id.clone(),
            device_id: auth.device_id.clone(),
            read_seq,
            last_read_message_id,
        }
    }
}

impl ApplyConversationPolicyCommand {
    pub fn from_auth_context(
        auth: &AppContext,
        conversation_id: String,
        policy: ConversationPolicy,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            applied_by: auth.actor_id.clone(),
            policy,
        }
    }
}

fn sender_from_auth_context(auth: &AppContext) -> Sender {
    Sender {
        id: auth.actor_id.clone(),
        kind: auth.actor_kind.clone(),
        member_id: None,
        device_id: auth.device_id.clone(),
        session_id: auth.session_id.clone(),
        metadata: BTreeMap::new(),
    }
}

impl PostMessageCommand {
    pub fn from_auth_context(
        auth: &AppContext,
        conversation_id: String,
        client_msg_id: Option<String>,
        message_type: MessageType,
        body: MessageBody,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            sender: sender_from_auth_context(auth),
            client_msg_id,
            message_type,
            body,
        }
    }

    pub fn new(
        tenant_id: impl Into<String>,
        conversation_id: impl Into<String>,
        sender: Sender,
        client_msg_id: Option<String>,
        message_type: MessageType,
        body: MessageBody,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            organization_id: default_organization_id(),
            conversation_id: conversation_id.into(),
            sender,
            client_msg_id,
            message_type,
            body,
        }
    }
}

impl PublishSystemChannelMessageCommand {
    pub fn from_auth_context(
        auth: &AppContext,
        conversation_id: String,
        client_msg_id: Option<String>,
        body: MessageBody,
    ) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            conversation_id,
            publisher: sender_from_auth_context(auth),
            client_msg_id,
            body,
        }
    }
}

impl EditMessageCommand {
    pub fn from_auth_context(auth: &AppContext, message_id: String, body: MessageBody) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            message_id,
            editor: sender_from_auth_context(auth),
            body,
            idempotency_key: None,
        }
    }
}

impl RecallMessageCommand {
    pub fn from_auth_context(auth: &AppContext, message_id: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            message_id,
            recalled_by: sender_from_auth_context(auth),
            idempotency_key: None,
        }
    }
}

impl AddMessageReactionCommand {
    pub fn from_auth_context(auth: &AppContext, message_id: String, reaction_key: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            message_id,
            reaction_key,
            reacted_by: sender_from_auth_context(auth),
        }
    }
}

impl RemoveMessageReactionCommand {
    pub fn from_auth_context(auth: &AppContext, message_id: String, reaction_key: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            message_id,
            reaction_key,
            removed_by: sender_from_auth_context(auth),
        }
    }
}

impl PinMessageCommand {
    pub fn from_auth_context(auth: &AppContext, message_id: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            message_id,
            pinned_by: sender_from_auth_context(auth),
        }
    }
}

impl UnpinMessageCommand {
    pub fn from_auth_context(auth: &AppContext, message_id: String) -> Self {
        Self {
            tenant_id: auth.tenant_id.clone(),
            organization_id: organization_id_from_auth_context(auth),
            message_id,
            unpinned_by: sender_from_auth_context(auth),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageMutationResult {
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub event_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageHistoryResult {
    #[serde(flatten)]
    pub page: SdkWorkPageData<im_domain_core::message::StoredMessage>,
    pub high_watermark: u64,
    pub next_before_seq: Option<u64>,
}

pub type ListMembersResult = SdkWorkPageData<ConversationMember>;

pub type ListPinnedMessagesResult = SdkWorkPageData<String>;

pub type InboxListResult = SdkWorkPageData<String>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageReactionMutationResult {
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub reaction_key: String,
    pub event_id: Option<String>,
    pub changed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagePinMutationResult {
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub event_id: Option<String>,
    pub changed: bool,
}

#[derive(Debug)]
pub enum RuntimeError {
    ConversationAlreadyExists(String),
    ConversationTypeInvalid(String),
    AgentIdInvalid(String),
    InvalidInput(String),
    PayloadTooLarge(String),
    ConversationNotFound(String),
    ConversationBindingNotFound(String),
    MessageNotFound(String),
    MessageAlreadyRecalled(String),
    MemberAlreadyExists(String),
    MemberNotFound(String),
    PermissionDenied(String),
    Conflict(String),
    ReadCursorInvalid(String),
    Contract(ContractError),
}

impl From<ContractError> for RuntimeError {
    fn from(value: ContractError) -> Self {
        Self::Contract(value)
    }
}

impl RuntimeError {
    fn payload_too_large(field: &str, max_bytes: usize, actual_bytes: usize) -> Self {
        Self::PayloadTooLarge(format!(
            "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
        ))
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::ConversationAlreadyExists(id) => {
                write!(f, "conversation already exists: {id}")
            }
            RuntimeError::ConversationTypeInvalid(value) => {
                write!(f, "conversation type invalid: {value}")
            }
            RuntimeError::AgentIdInvalid(value) => write!(f, "agent id invalid: {value}"),
            RuntimeError::InvalidInput(message) => write!(f, "invalid input: {message}"),
            RuntimeError::PayloadTooLarge(message) => write!(f, "{message}"),
            RuntimeError::ConversationNotFound(id) => write!(f, "conversation not found: {id}"),
            RuntimeError::ConversationBindingNotFound(id) => {
                write!(f, "conversation binding not found: {id}")
            }
            RuntimeError::MessageNotFound(id) => write!(f, "message not found: {id}"),
            RuntimeError::MessageAlreadyRecalled(id) => {
                write!(f, "message already recalled: {id}")
            }
            RuntimeError::MemberAlreadyExists(id) => write!(f, "member already exists: {id}"),
            RuntimeError::MemberNotFound(id) => write!(f, "member not found: {id}"),
            RuntimeError::PermissionDenied(message) => write!(f, "permission denied: {message}"),
            RuntimeError::Conflict(message) => write!(f, "conflict: {message}"),
            RuntimeError::ReadCursorInvalid(message) => {
                write!(f, "read cursor invalid: {message}")
            }
            RuntimeError::Contract(error) => write!(f, "contract error: {error:?}"),
        }
    }
}

impl std::error::Error for RuntimeError {}

fn runtime_json_string<T: serde::Serialize>(value: &T) -> Result<String, RuntimeError> {
    serde_json::to_string(value).map_err(|error| {
        RuntimeError::InvalidInput(format!("failed to serialize runtime payload: {error}"))
    })
}

#[derive(Clone, Default)]
struct ConversationState {
    aggregate: ConversationAggregateState,
    roster: ConversationRoster,
    message_log: ConversationMessageLog,
    generic_create_request: Option<GenericConversationCreateReplayRecord>,
    agent_dialog_create_request: Option<AgentDialogCreateReplayRecord>,
    system_channel_create_request: Option<SystemChannelCreateReplayRecord>,
    agent_handoff_create_request: Option<AgentHandoffCreateReplayRecord>,
    thread_create_request: Option<ThreadConversationCreateReplayRecord>,
    room_create_request: Option<RoomCreateReplayRecord>,
    direct_chat_binding_request: Option<DirectChatBindingReplayRecord>,
    posted_message_requests: BoundedReplayCache<PostedMessageReplayRecord>,
    message_mutation_requests: BoundedReplayCache<MessageMutationReplayRecord>,
    last_accessed_at_ms: u64,
}

impl ConversationState {
    fn estimated_heap_bytes(&self) -> usize {
        CONVERSATION_STATE_FIXED_OVERHEAD_BYTES
            .saturating_add(self.roster.estimated_heap_bytes())
            .saturating_add(self.message_log.estimated_heap_bytes())
            .saturating_add(self.posted_message_requests.estimated_heap_bytes())
            .saturating_add(self.message_mutation_requests.estimated_heap_bytes())
    }
}

#[derive(Default)]
struct RuntimeState {
    conversations: HashMap<String, ConversationState>,
    conversation_weights: HashMap<String, usize>,
    dirty_conversation_scopes: HashSet<String>,
    estimated_conversation_bytes: usize,
    business_index: HashMap<String, String>,
    message_locator: MessageLocatorIndex,
    actor_inbox: actor_inbox::ActorInboxRuntimeStore,
}

pub(super) fn lock_runtime_mutex<'a, T>(
    mutex: &'a Mutex<T>,
    label: &'static str,
) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovering poisoned mutex in conversation-runtime: {label}");
            poisoned.into_inner()
        }
    }
}

fn read_runtime_state<'a>(
    state: &'a RwLock<RuntimeState>,
    label: &'static str,
) -> RwLockReadGuard<'a, RuntimeState> {
    match state.read() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!(
                "recovering poisoned runtime read lock in conversation-runtime: {label}"
            );
            poisoned.into_inner()
        }
    }
}

fn write_runtime_state<'a>(
    state: &'a RwLock<RuntimeState>,
    label: &'static str,
) -> RwLockWriteGuard<'a, RuntimeState> {
    match state.write() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!(
                "recovering poisoned runtime write lock in conversation-runtime: {label}"
            );
            poisoned.into_inner()
        }
    }
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn resolve_max_conversations_in_memory() -> usize {
    std::env::var(CONVERSATION_MAX_IN_MEMORY_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(CONVERSATION_MAX_IN_MEMORY_DEFAULT)
}

fn resolve_conversation_cache_max_bytes() -> usize {
    std::env::var(CONVERSATION_CACHE_MAX_BYTES_ENV)
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(CONVERSATION_CACHE_MAX_BYTES_DEFAULT)
}

impl RuntimeState {
    fn insert_conversation(&mut self, scope_key: String, mut conversation: ConversationState) {
        if let Some(previous_binding) = self
            .conversations
            .get(scope_key.as_str())
            .and_then(|previous| previous.aggregate.business_binding())
            .cloned()
            && let Some((tenant_id, _, conversation_id)) =
                decode_conversation_scope_key(scope_key.as_str())
        {
            let previous_business_scope = conversation_business_scope_key(
                tenant_id.as_str(),
                previous_binding.business_type.as_str(),
                previous_binding.business_id.as_str(),
            );
            if self
                .business_index
                .get(previous_business_scope.as_str())
                .is_some_and(|mapped_id| mapped_id == conversation_id.as_str())
            {
                self.business_index.remove(previous_business_scope.as_str());
            }
        }
        if conversation.last_accessed_at_ms == 0 {
            conversation.last_accessed_at_ms = now_ms();
        }
        if let Some(previous_weight) = self.conversation_weights.remove(scope_key.as_str()) {
            self.estimated_conversation_bytes = self
                .estimated_conversation_bytes
                .saturating_sub(previous_weight);
        }
        let weight = conversation.estimated_heap_bytes();
        self.estimated_conversation_bytes =
            self.estimated_conversation_bytes.saturating_add(weight);
        self.conversation_weights.insert(scope_key.clone(), weight);
        self.dirty_conversation_scopes.remove(scope_key.as_str());
        if let Some(binding) = conversation.aggregate.business_binding()
            && let Some((tenant_id, _, conversation_id)) =
                decode_conversation_scope_key(scope_key.as_str())
        {
            self.business_index.insert(
                conversation_business_scope_key(
                    tenant_id.as_str(),
                    binding.business_type.as_str(),
                    binding.business_id.as_str(),
                ),
                conversation_id,
            );
        }
        self.conversations.insert(scope_key, conversation);
    }

    fn touch_conversation(&mut self, scope_key: &str) {
        if let Some(conv) = self.conversations.get_mut(scope_key) {
            conv.last_accessed_at_ms = now_ms();
            self.dirty_conversation_scopes.insert(scope_key.to_owned());
        }
    }

    fn refresh_dirty_conversation_weights(&mut self) {
        if self.conversation_weights.len() != self.conversations.len() {
            self.dirty_conversation_scopes.extend(
                self.conversations
                    .keys()
                    .filter(|scope| !self.conversation_weights.contains_key(scope.as_str()))
                    .cloned(),
            );
        }
        let dirty_scopes = std::mem::take(&mut self.dirty_conversation_scopes);
        for scope_key in dirty_scopes {
            let Some(conversation) = self.conversations.get(scope_key.as_str()) else {
                if let Some(previous_weight) = self.conversation_weights.remove(scope_key.as_str())
                {
                    self.estimated_conversation_bytes = self
                        .estimated_conversation_bytes
                        .saturating_sub(previous_weight);
                }
                continue;
            };
            let weight = conversation.estimated_heap_bytes();
            let previous_weight = self
                .conversation_weights
                .insert(scope_key, weight)
                .unwrap_or_default();
            self.estimated_conversation_bytes = self
                .estimated_conversation_bytes
                .saturating_sub(previous_weight)
                .saturating_add(weight);
        }
    }

    fn evict_idle_conversations(&mut self, max_conversations: usize, max_bytes: usize) -> usize {
        self.refresh_dirty_conversation_weights();
        let count = self.conversations.len();
        let over_count = count > max_conversations;
        let over_bytes = self.estimated_conversation_bytes > max_bytes;
        if !over_count && !over_bytes {
            return 0;
        }
        let target_count = if over_count {
            ((max_conversations as f64 * CONVERSATION_IDLE_EVICTION_TARGET_RATIO) as usize).max(1)
        } else {
            count
        };
        let target_bytes = if over_bytes {
            ((max_bytes as f64 * CONVERSATION_IDLE_EVICTION_TARGET_RATIO) as usize).max(1)
        } else {
            self.estimated_conversation_bytes
        };
        let mut entries: Vec<(String, u64)> = self
            .conversations
            .iter()
            .map(|(key, conversation)| (key.clone(), conversation.last_accessed_at_ms))
            .collect();
        entries.sort_unstable_by_key(|(_, last_accessed_at_ms)| *last_accessed_at_ms);
        let mut evicted = 0usize;
        for (key, _) in entries {
            if self.conversations.len() <= target_count
                && self.estimated_conversation_bytes <= target_bytes
            {
                break;
            }
            let Some(conversation) = self.conversations.remove(key.as_str()) else {
                continue;
            };
            if let Some(weight) = self.conversation_weights.remove(key.as_str()) {
                self.estimated_conversation_bytes =
                    self.estimated_conversation_bytes.saturating_sub(weight);
            }
            self.dirty_conversation_scopes.remove(key.as_str());
            if let Some((tenant_id, organization_id, conversation_id)) =
                decode_conversation_scope_key(key.as_str())
            {
                self.message_locator
                    .remove_conversation(tenant_id.as_str(), conversation_id.as_str());
                self.actor_inbox.remove_conversation(
                    organization_id.as_str(),
                    conversation_id.as_str(),
                    &conversation,
                );
                if let Some(binding) = conversation.aggregate.business_binding() {
                    let business_scope_key = conversation_business_scope_key(
                        tenant_id.as_str(),
                        binding.business_type.as_str(),
                        binding.business_id.as_str(),
                    );
                    if self
                        .business_index
                        .get(business_scope_key.as_str())
                        .is_some_and(|mapped_id| mapped_id == conversation_id.as_str())
                    {
                        self.business_index.remove(business_scope_key.as_str());
                    }
                }
            }
            evicted = evicted.saturating_add(1);
        }
        evicted
    }
}

fn validate_payload_size(field: &str, value: &str, max_bytes: usize) -> Result<(), RuntimeError> {
    let actual_bytes = value.len();
    if actual_bytes > max_bytes {
        return Err(RuntimeError::payload_too_large(
            field,
            max_bytes,
            actual_bytes,
        ));
    }
    Ok(())
}

fn validate_optional_payload_size(
    field: &str,
    value: Option<&str>,
    max_bytes: usize,
) -> Result<(), RuntimeError> {
    if let Some(value) = value {
        validate_payload_size(field, value, max_bytes)?;
    }
    Ok(())
}

fn validate_standard_agent_id(agent_id: &str) -> Result<(), RuntimeError> {
    if agent_id.trim().is_empty() {
        return Err(RuntimeError::AgentIdInvalid("agentId is required".into()));
    }
    if agent_id.trim() != agent_id {
        return Err(RuntimeError::AgentIdInvalid(
            "agentId must not contain leading or trailing whitespace".into(),
        ));
    }
    if agent_id.chars().count() > 128 {
        return Err(RuntimeError::AgentIdInvalid(
            "agentId must be at most 128 characters".into(),
        ));
    }
    if !agent_id.chars().all(is_standard_agent_id_character) {
        return Err(RuntimeError::AgentIdInvalid(
            "agentId must use lowercase standard id characters".into(),
        ));
    }
    if !agent_id.split('.').all(|segment| !segment.is_empty()) {
        return Err(RuntimeError::AgentIdInvalid(
            "agentId must use non-empty dot-delimited segments".into(),
        ));
    }
    if !agent_id.starts_with("agent.") {
        return Err(RuntimeError::AgentIdInvalid(
            "agentId must start with agent.".into(),
        ));
    }
    Ok(())
}

fn is_standard_agent_id_character(ch: char) -> bool {
    ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '.' | '_' | '-')
}

fn validate_string_vec_payload_size(
    field: &str,
    values: &[String],
    item_max_bytes: usize,
    total_max_bytes: usize,
) -> Result<(), RuntimeError> {
    let total_bytes = values
        .iter()
        .fold(0usize, |total, value| total.saturating_add(value.len()));
    if total_bytes > total_max_bytes {
        return Err(RuntimeError::payload_too_large(
            field,
            total_max_bytes,
            total_bytes,
        ));
    }
    for value in values {
        validate_payload_size(field, value.as_str(), item_max_bytes)?;
    }
    Ok(())
}

fn validate_string_map_payload_size(
    field: &str,
    values: &BTreeMap<String, String>,
    max_bytes: usize,
) -> Result<(), RuntimeError> {
    let payload_bytes = values
        .iter()
        .map(|(key, value)| key.len().saturating_add(value.len()))
        .sum::<usize>();
    if payload_bytes > max_bytes {
        return Err(RuntimeError::payload_too_large(
            field,
            max_bytes,
            payload_bytes,
        ));
    }
    Ok(())
}

fn validate_member_attributes_payload_size(
    field: &str,
    attributes: &BTreeMap<String, String>,
) -> Result<(), RuntimeError> {
    validate_string_map_payload_size(field, attributes, CONVERSATION_MAX_MEMBER_ATTRIBUTES_BYTES)
}

fn validate_sender_payload_size(field_prefix: &str, sender: &Sender) -> Result<(), RuntimeError> {
    let id_field = format!("{field_prefix}Id");
    validate_payload_size(
        id_field.as_str(),
        sender.id.as_str(),
        CONVERSATION_MAX_ID_BYTES,
    )?;

    let kind_field = format!("{field_prefix}Kind");
    validate_payload_size(
        kind_field.as_str(),
        sender.kind.as_str(),
        CONVERSATION_MAX_KIND_BYTES,
    )?;

    let member_id_field = format!("{field_prefix}MemberId");
    validate_optional_payload_size(
        member_id_field.as_str(),
        sender.member_id.as_deref(),
        CONVERSATION_MAX_ID_BYTES,
    )?;

    let device_id_field = format!("{field_prefix}DeviceId");
    validate_optional_payload_size(
        device_id_field.as_str(),
        sender.device_id.as_deref(),
        CONVERSATION_MAX_ID_BYTES,
    )?;

    let session_id_field = format!("{field_prefix}SessionId");
    validate_optional_payload_size(
        session_id_field.as_str(),
        sender.session_id.as_deref(),
        CONVERSATION_MAX_ID_BYTES,
    )?;

    let metadata_field = format!("{field_prefix}Metadata");
    validate_string_map_payload_size(
        metadata_field.as_str(),
        &sender.metadata,
        CONVERSATION_MAX_SENDER_METADATA_BYTES,
    )?;

    Ok(())
}

fn validate_message_body_size(body: &MessageBody) -> Result<(), RuntimeError> {
    validate_string_map_payload_size(
        "renderHints",
        &body.render_hints,
        MESSAGE_RENDER_HINTS_MAX_BYTES,
    )?;
    let actual_bytes = serde_json::to_vec(body)
        .map_err(|error| RuntimeError::InvalidInput(format!("message body invalid: {error}")))?
        .len();
    if actual_bytes > MESSAGE_BODY_MAX_BYTES {
        return Err(RuntimeError::payload_too_large(
            "messageBody",
            MESSAGE_BODY_MAX_BYTES,
            actual_bytes,
        ));
    }
    Ok(())
}

fn validate_message_body_semantics(body: &MessageBody) -> Result<(), RuntimeError> {
    for (index, part) in body.parts.iter().enumerate() {
        match part {
            ContentPart::Media(media_part) => {
                let drive = &media_part.drive;
                validate_media_drive_reference(index, drive)?;
                validate_media_resource_drive_snapshot(index, &media_part.resource, drive)?;
            }
            ContentPart::Mention(mention_part) => {
                if mention_part.display_text.trim().is_empty() {
                    return Err(RuntimeError::InvalidInput(format!(
                        "message body parts[{index}].displayText must not be empty"
                    )));
                }
                let character_count = mention_part.display_text.chars().count();
                if character_count > MESSAGE_MENTION_DISPLAY_TEXT_MAX_CHARACTERS {
                    return Err(RuntimeError::InvalidInput(format!(
                        "message body parts[{index}].displayText must not exceed {MESSAGE_MENTION_DISPLAY_TEXT_MAX_CHARACTERS} characters: {character_count}"
                    )));
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn validate_message_body_contract(body: &MessageBody) -> Result<(), RuntimeError> {
    validate_message_body_size(body)?;
    validate_message_body_semantics(body)
}

fn validate_media_drive_reference(
    part_index: usize,
    drive: &DriveReference,
) -> Result<(), RuntimeError> {
    for (field, value) in [
        ("driveUri", drive.drive_uri.as_str()),
        ("spaceId", drive.space_id.as_str()),
        ("nodeId", drive.node_id.as_str()),
    ] {
        if value.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(format!(
                "message body parts[{part_index}].drive.{field} must not be empty"
            )));
        }
    }

    if !drive.is_canonical() {
        return Err(RuntimeError::InvalidInput(format!(
            "message body parts[{part_index}].drive.driveUri must equal drive://spaces/{{spaceId}}/nodes/{{nodeId}}"
        )));
    }
    Ok(())
}

fn validate_media_resource_drive_snapshot(
    part_index: usize,
    resource: &MediaResource,
    drive: &DriveReference,
) -> Result<(), RuntimeError> {
    match resource.source {
        MediaSource::Drive | MediaSource::ProviderAsset | MediaSource::Generated => {}
        MediaSource::ExternalUrl | MediaSource::DataUrl => {
            return Err(RuntimeError::InvalidInput(format!(
                "message body parts[{part_index}].resource.source must be drive, provider_asset, or generated for Drive-backed media parts"
            )));
        }
    }

    match resource.uri.as_deref() {
        Some(uri) if uri == drive.drive_uri => {}
        Some(_) => {
            return Err(RuntimeError::InvalidInput(format!(
                "message body parts[{part_index}].resource.uri must match parts[{part_index}].drive.driveUri"
            )));
        }
        None => {
            return Err(RuntimeError::InvalidInput(format!(
                "message body parts[{part_index}].resource.uri is required for Drive-backed media"
            )));
        }
    }

    if let Some(id) = resource.id.as_deref()
        && id != drive.node_id
    {
        return Err(RuntimeError::InvalidInput(format!(
            "message body parts[{part_index}].resource.id must match parts[{part_index}].drive.nodeId when present"
        )));
    }

    validate_media_resource_delivery_urls(part_index, "resource", resource)?;

    Ok(())
}

fn is_local_preview_url(value: &str) -> bool {
    let trimmed = value.trim();
    trimmed.get(..5).is_some_and(|prefix| {
        prefix.eq_ignore_ascii_case("blob:") || prefix.eq_ignore_ascii_case("data:")
    })
}

fn validate_media_resource_delivery_urls(
    part_index: usize,
    field_prefix: &str,
    resource: &MediaResource,
) -> Result<(), RuntimeError> {
    for (field, value) in [
        ("url", resource.url.as_deref()),
        ("publicUrl", resource.public_url.as_deref()),
    ] {
        if value.is_some_and(is_local_preview_url) {
            return Err(RuntimeError::InvalidInput(format!(
                "message body parts[{part_index}].{field_prefix}.{field} must not be a local preview URL"
            )));
        }
    }

    if let Some(poster) = resource.poster.as_deref() {
        validate_media_resource_delivery_urls(
            part_index,
            format!("{field_prefix}.poster").as_str(),
            poster,
        )?;
    }
    if let Some(thumbnails) = &resource.thumbnails {
        for (index, thumbnail) in thumbnails.iter().enumerate() {
            validate_media_resource_delivery_urls(
                part_index,
                format!("{field_prefix}.thumbnails[{index}]").as_str(),
                thumbnail,
            )?;
        }
    }
    if let Some(variants) = &resource.variants {
        for (index, variant) in variants.iter().enumerate() {
            validate_media_resource_delivery_urls(
                part_index,
                format!("{field_prefix}.variants[{index}]").as_str(),
                variant,
            )?;
        }
    }

    Ok(())
}

fn generic_conversation_create_request_key(
    tenant_id: &str,
    creator_kind: &str,
    creator_id: &str,
    conversation_id: &str,
) -> String {
    encode_conversation_key_segments([
        tenant_id,
        creator_kind,
        creator_id,
        "create-conversation",
        conversation_id,
    ])
}

fn generic_conversation_create_replay_matches(
    existing: &GenericConversationCreateReplayRecord,
    command: &CreateConversationCommand,
    creator_kind: &str,
    initial_member_user_ids: &[String],
    initial_agent_assignments: Option<&[ConversationAgentAssignment]>,
    knowledgebase_initialization_requested: bool,
) -> bool {
    (existing.creator_id.is_empty() || existing.creator_id == command.creator_id)
        && (existing.creator_kind.is_empty() || existing.creator_kind == creator_kind)
        && existing.requested_kind == command.conversation_type
        && existing.initial_member_user_ids == initial_member_user_ids
        && existing.initial_agent_assignments.as_deref() == initial_agent_assignments
        && existing.knowledgebase_initialization_requested == knowledgebase_initialization_requested
}

fn agent_dialog_create_request_key(
    tenant_id: &str,
    requester_kind: &str,
    requester_id: &str,
    conversation_id: &str,
) -> String {
    encode_conversation_key_segments([
        tenant_id,
        requester_kind,
        requester_id,
        "create-agent-dialog",
        conversation_id,
    ])
}

fn agent_dialog_create_replay_matches(
    existing: &AgentDialogCreateReplayRecord,
    command: &CreateAgentDialogCommand,
    requester_kind: &str,
) -> bool {
    existing.requester_id == command.requester_id
        && existing.requester_kind == requester_kind
        && existing.agent_id == command.agent_id
}

fn system_channel_create_request_key(
    tenant_id: &str,
    requester_kind: &str,
    requester_id: &str,
    conversation_id: &str,
) -> String {
    encode_conversation_key_segments([
        tenant_id,
        requester_kind,
        requester_id,
        "create-system_channel",
        conversation_id,
    ])
}

fn system_channel_create_replay_matches(
    existing: &SystemChannelCreateReplayRecord,
    command: &CreateSystemChannelCommand,
    requester_kind: &str,
) -> bool {
    existing.requester_id == command.requester_id
        && existing.requester_kind == requester_kind
        && existing.subscriber_id == command.subscriber_id
}

fn agent_handoff_create_request_key(
    tenant_id: &str,
    source_kind: &str,
    source_id: &str,
    conversation_id: &str,
) -> String {
    encode_conversation_key_segments([
        tenant_id,
        source_kind,
        source_id,
        "create-agent_handoff",
        conversation_id,
    ])
}

fn agent_handoff_create_replay_matches(
    existing: &AgentHandoffCreateReplayRecord,
    command: &CreateAgentHandoffCommand,
    source_kind: &str,
) -> bool {
    existing.source_id == command.source_id
        && existing.source_kind == source_kind
        && existing.target_id == command.target_id
        && existing.target_kind == command.target_kind
        && existing.handoff_session_id == command.handoff_session_id
        && existing.handoff_reason == command.handoff_reason
}

fn thread_conversation_create_request_key(
    tenant_id: &str,
    creator_kind: &str,
    creator_id: &str,
    conversation_id: &str,
) -> String {
    encode_conversation_key_segments([
        tenant_id,
        creator_kind,
        creator_id,
        "create-thread",
        conversation_id,
    ])
}

fn thread_conversation_create_replay_matches(
    existing: &ThreadConversationCreateReplayRecord,
    command: &CreateThreadConversationCommand,
    creator_kind: &str,
) -> bool {
    existing.creator_id == command.creator_id
        && existing.creator_kind == creator_kind
        && existing.parent_conversation_id == command.parent_conversation_id
        && existing.root_message_id == command.root_message_id
}

fn direct_chat_binding_request_key(
    tenant_id: &str,
    binder_kind: &str,
    bound_by: &str,
    conversation_id: &str,
) -> String {
    encode_conversation_key_segments([
        tenant_id,
        binder_kind,
        bound_by,
        "bind-direct-chat",
        conversation_id,
    ])
}

// Authorization is checked before this comparison. The original binder is audit
// provenance, while the canonical participant pair and direct-chat id define state identity.
fn direct_chat_binding_state_matches(
    existing: &DirectChatBindingReplayRecord,
    command: &BindDirectChatConversationCommand,
    pair: &im_domain_core::social::NormalizedActorPair,
    direct_chat_id: &str,
) -> bool {
    existing.direct_chat_id == direct_chat_id
        && existing.anchor_actor_id == pair.left_actor_id
        && existing.anchor_actor_kind == command.left_actor_kind
        && existing.peer_actor_id == pair.right_actor_id
        && existing.peer_actor_kind == command.right_actor_kind
}

fn normalized_direct_chat_binding_state_matches(
    existing: &ConversationState,
    command: &BindDirectChatConversationCommand,
    direct_chat_id: &str,
) -> bool {
    existing.aggregate.conversation_type() == "direct"
        && existing.aggregate.lifecycle_state() == ConversationLifecycleState::Active
        && existing
            .aggregate
            .business_binding()
            .is_some_and(|binding| {
                binding.business_type == "direct_chat" && binding.business_id == direct_chat_id
            })
        && existing
            .roster
            .resolve_active_member_with_kind(
                command.left_actor_id.as_str(),
                command.left_actor_kind.as_str(),
            )
            .is_some()
        && existing
            .roster
            .resolve_active_member_with_kind(
                command.right_actor_id.as_str(),
                command.right_actor_kind.as_str(),
            )
            .is_some()
}

fn post_message_request_key(command: &PostMessageCommand) -> Option<String> {
    command.client_msg_id.as_ref().map(|client_msg_id| {
        encode_conversation_key_segments([
            command.tenant_id.as_str(),
            command.sender.kind.as_str(),
            command.sender.id.as_str(),
            "message",
            command.conversation_id.as_str(),
            client_msg_id.as_str(),
        ])
    })
}

fn posted_message_replay_matches(
    existing: &PostedMessageReplayRecord,
    command: &PostMessageCommand,
) -> bool {
    existing.sender_id == command.sender.id
        && existing.sender_kind == command.sender.kind
        && existing.message_type == command.message_type
        && existing.body == command.body
}

fn durable_posted_message_replay_matches(
    existing: &StoredMessageRecord,
    command: &PostMessageCommand,
) -> Result<bool, RuntimeError> {
    let stored_body = serde_json::from_str::<MessageBody>(existing.payload_json.as_str())
        .map_err(|_| RuntimeError::Conflict("stored message replay payload is invalid".into()))?;
    Ok(existing.tenant_id == command.tenant_id
        && im_domain_events::normalize_commit_organization_id(existing.organization_id.as_str())
            == im_domain_events::normalize_commit_organization_id(command.organization_id.as_str())
        && existing.conversation_id == command.conversation_id
        && existing.sender_principal_kind == command.sender.kind
        && existing.sender_principal_id == command.sender.id
        && existing.client_msg_id == command.client_msg_id
        && existing.message_type == command.message_type.as_wire_value()
        && stored_body == command.body)
}

fn rtc_session_id_from_signal_message(command: &PostMessageCommand) -> Option<String> {
    if command.message_type != MessageType::Signal {
        return None;
    }

    command.body.parts.iter().find_map(|part| {
        let ContentPart::Signal(signal) = part else {
            return None;
        };
        let payload = serde_json::from_str::<JsonValue>(signal.payload.as_str()).ok()?;
        string_json_field(&payload, &["rtcSessionId", "rtc_session_id"]).map(str::to_owned)
    })
}

fn string_json_field<'a>(value: &'a JsonValue, names: &[&str]) -> Option<&'a str> {
    names.iter().find_map(|name| {
        value
            .get(*name)
            .and_then(JsonValue::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
    })
}

fn generated_message_id(conversation_id: &str, message_seq: u64) -> String {
    let raw_message_id = format!("msg_{conversation_id}_{message_seq}");
    if raw_message_id.len() <= CONVERSATION_MAX_ID_BYTES {
        return raw_message_id;
    }

    let digest = sha256_hash(conversation_id.as_bytes());
    let bounded_message_id = format!("msg_{digest}_{message_seq}");
    debug_assert!(bounded_message_id.len() <= CONVERSATION_MAX_ID_BYTES);
    bounded_message_id
}

/// 计算消息 body 的 SHA256 哈希，用于真值表的 payload_hash 字段。
fn sha256_message_hash(body: &MessageBody) -> String {
    let serialized = serde_json::to_vec(body).unwrap_or_default();
    format!("sha256:{}", sha256_hash(&serialized))
}

fn validate_agent_dispatch_reply(
    conversation: &ConversationState,
    command: &PostMessageCommand,
    completion: Option<&AgentDispatchReplyCompletion>,
) -> Result<(), RuntimeError> {
    let completion = completion.ok_or_else(|| {
        RuntimeError::InvalidInput("agent dispatch reply completion is required".into())
    })?;
    let tenant_id = command.tenant_id.parse::<u64>().map_err(|_| {
        RuntimeError::InvalidInput("agent dispatch reply tenant id must be an int64 string".into())
    })?;
    let organization_id = command.organization_id.parse::<u64>().map_err(|_| {
        RuntimeError::InvalidInput(
            "agent dispatch reply organization id must be an int64 string".into(),
        )
    })?;
    if tenant_id != completion.tenant_id
        || organization_id != completion.organization_id
        || command.conversation_id != completion.conversation_id
        || command.sender.kind != "agent"
        || command.sender.id != completion.agent_id
        || command.message_type != MessageType::Standard
        || completion.dispatch_id.trim().is_empty()
        || completion.lease_owner.trim().is_empty()
        || completion.agents_session_id.trim().is_empty()
        || completion.agents_turn_id.trim().is_empty()
    {
        return Err(RuntimeError::InvalidInput(
            "agent dispatch reply identity is invalid".into(),
        ));
    }
    if command.client_msg_id.as_deref()
        != Some(format!("agent-dispatch-reply:{}", completion.dispatch_id).as_str())
    {
        return Err(RuntimeError::InvalidInput(
            "agent dispatch reply client message id is invalid".into(),
        ));
    }
    let assignments = conversation.aggregate.agent_assignments().ok_or_else(|| {
        RuntimeError::Conflict("agent dispatch reply conversation has no agent assignments".into())
    })?;
    let assignment = assignments
        .agents
        .iter()
        .find(|assignment| assignment.agent_id == completion.agent_id)
        .ok_or_else(|| {
            RuntimeError::Conflict(
                "agent dispatch reply target is no longer assigned to the conversation".into(),
            )
        })?;
    if assignments.generation != completion.assignment_generation
        || assignment.revision_id != completion.agent_revision_ref
    {
        return Err(RuntimeError::Conflict(
            "agent dispatch reply assignment generation or revision is stale".into(),
        ));
    }
    Ok(())
}

const IN_MEMORY_JOURNAL_MAX_EVENTS_DEFAULT: usize = 100_000;
const IN_MEMORY_JOURNAL_MAX_EVENTS_ENV: &str = "SDKWORK_IM_JOURNAL_MAX_EVENTS";

#[derive(Clone)]
pub struct InMemoryJournal {
    events: Arc<Mutex<Vec<CommitEnvelope>>>,
    max_events: usize,
}

impl Default for InMemoryJournal {
    fn default() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            max_events: std::env::var(IN_MEMORY_JOURNAL_MAX_EVENTS_ENV)
                .ok()
                .and_then(|v| v.trim().parse::<usize>().ok())
                .filter(|v| *v > 0)
                .unwrap_or(IN_MEMORY_JOURNAL_MAX_EVENTS_DEFAULT),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JournalSnapshot {
    pub events: Vec<CommitEnvelope>,
    pub snapshot_version: String,
    pub exported_at: String,
}

impl JournalSnapshot {
    pub fn new(events: Vec<CommitEnvelope>) -> Self {
        Self {
            events,
            snapshot_version: "conversation.journal.snapshot.v1".into(),
            exported_at: utc_now_rfc3339_millis(),
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl InMemoryJournal {
    pub fn recorded(&self) -> Vec<CommitEnvelope> {
        lock_runtime_mutex(&self.events, "in-memory-journal.events").clone()
    }

    pub fn export_snapshot(&self) -> JournalSnapshot {
        JournalSnapshot::new(self.recorded())
    }

    pub fn load_from_snapshot(snapshot: JournalSnapshot) -> Self {
        let capacity = snapshot.events.len();
        Self {
            events: Arc::new(Mutex::new(snapshot.events)),
            max_events: std::env::var(IN_MEMORY_JOURNAL_MAX_EVENTS_ENV)
                .ok()
                .and_then(|v| v.trim().parse::<usize>().ok())
                .filter(|v| *v > 0)
                .unwrap_or(IN_MEMORY_JOURNAL_MAX_EVENTS_DEFAULT)
                .max(capacity.saturating_add(1024)),
        }
    }
}

impl CommitJournal for InMemoryJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let mut events = lock_runtime_mutex(&self.events, "in-memory-journal.events");
        if events.len() >= self.max_events {
            return Err(ContractError::Unavailable(
                "journal event store is full; snapshot and reset to continue".into(),
            ));
        }
        events.push(envelope);
        Ok(CommitPosition::new("p0", events.len() as u64))
    }

    fn append_batch(
        &self,
        envelopes: Vec<CommitEnvelope>,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        let mut events = lock_runtime_mutex(&self.events, "in-memory-journal.events");
        if events.len().saturating_add(envelopes.len()) > self.max_events {
            return Err(ContractError::Unavailable(
                "journal event store is full; snapshot and reset to continue".into(),
            ));
        }
        let start_offset = events.len() as u64 + 1;
        let batch_len = envelopes.len() as u64;
        events.extend(envelopes);
        Ok((0..batch_len)
            .map(|index| CommitPosition::new("p0", start_offset + index))
            .collect())
    }

    fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        Ok(InMemoryJournal::recorded(self))
    }

    fn recorded_page(
        &self,
        cursor: Option<&CommitJournalReplayCursor>,
        limit: usize,
    ) -> Result<CommitJournalReplayPage, ContractError> {
        let all = InMemoryJournal::recorded(self);
        let limit = limit.max(1);
        let start_index = cursor
            .and_then(|cursor| {
                all.iter().position(|envelope| {
                    envelope.ordering_key == cursor.partition_key
                        && envelope.ordering_seq == cursor.commit_offset
                })
            })
            .map(|index| index.saturating_add(1))
            .unwrap_or(0);
        let page_items: Vec<_> = all.into_iter().skip(start_index).take(limit).collect();
        let next_cursor = if page_items.len() == limit {
            page_items.last().map(|envelope| CommitJournalReplayCursor {
                partition_key: envelope.ordering_key.clone(),
                commit_offset: envelope.ordering_seq,
            })
        } else {
            None
        };
        Ok(CommitJournalReplayPage {
            items: page_items,
            next_cursor,
        })
    }

    fn recorded_page_for_aggregate(
        &self,
        scope: &CommitJournalAggregateScope,
        cursor: Option<&CommitJournalReplayCursor>,
        limit: usize,
    ) -> Result<CommitJournalReplayPage, ContractError> {
        let filtered: Vec<CommitEnvelope> = InMemoryJournal::recorded(self)
            .into_iter()
            .filter(|envelope| {
                envelope.tenant_id == scope.tenant_id
                    && (envelope.aggregate_id == scope.aggregate_id
                        || envelope.scope_id == scope.aggregate_id)
            })
            .collect();
        let limit = limit.max(1);
        let start_index = cursor
            .and_then(|cursor| {
                filtered.iter().position(|envelope| {
                    envelope.ordering_key == cursor.partition_key
                        && envelope.ordering_seq == cursor.commit_offset
                })
            })
            .map(|index| index.saturating_add(1))
            .unwrap_or(0);
        let page_items: Vec<_> = filtered.into_iter().skip(start_index).take(limit).collect();
        let next_cursor = if page_items.len() == limit {
            page_items.last().map(|envelope| CommitJournalReplayCursor {
                partition_key: envelope.ordering_key.clone(),
                commit_offset: envelope.ordering_seq,
            })
        } else {
            None
        };
        Ok(CommitJournalReplayPage {
            items: page_items,
            next_cursor,
        })
    }

    fn recorded_page_for_aggregate_event_types(
        &self,
        query: &CommitJournalAggregateEventTypeQuery,
        cursor: Option<&CommitJournalReplayCursor>,
        limit: usize,
    ) -> Result<CommitJournalReplayPage, ContractError> {
        if query.event_types.is_empty() {
            return Err(ContractError::Invalid(
                "journal aggregate event-type query requires at least one event type".into(),
            ));
        }
        let filtered: Vec<CommitEnvelope> = InMemoryJournal::recorded(self)
            .into_iter()
            .filter(|envelope| {
                envelope.tenant_id == query.tenant_id
                    && envelope.organization_id == query.organization_id
                    && envelope.aggregate_type.as_wire_value() == query.aggregate_type
                    && envelope.aggregate_id == query.aggregate_id
                    && query
                        .event_types
                        .iter()
                        .any(|event_type| event_type == &envelope.event_type)
            })
            .collect();
        let limit = limit.max(1);
        let start_index = cursor
            .and_then(|cursor| {
                filtered.iter().position(|envelope| {
                    envelope.ordering_key == cursor.partition_key
                        && envelope.ordering_seq == cursor.commit_offset
                })
            })
            .map(|index| index.saturating_add(1))
            .unwrap_or(0);
        let page_items: Vec<_> = filtered.into_iter().skip(start_index).take(limit).collect();
        let next_cursor = if page_items.len() == limit {
            page_items.last().map(|envelope| CommitJournalReplayCursor {
                partition_key: envelope.ordering_key.clone(),
                commit_offset: envelope.ordering_seq,
            })
        } else {
            None
        };
        Ok(CommitJournalReplayPage {
            items: page_items,
            next_cursor,
        })
    }
}

pub struct ConversationRuntime<J> {
    journal: J,
    state: RwLock<RuntimeState>,
    metrics: ConversationRuntimeMetrics,
    group_agent_default_policy: agents::GroupAgentDefaultPolicy,
    /// 可选的消息真值存储。注入后 post_message 走 DB seq 分配 + 真值写入路径。
    message_store: Option<Arc<dyn MessageStore>>,
    /// 可选的 Outbox 存储。注入后事件通过 outbox 异步投递。
    outbox_store: Option<Arc<dyn OutboxStore>>,
    /// 可选的 ID 生成器。注入后 message_id/event_id 使用 Snowflake。
    id_generator: Option<Arc<dyn IdGenerator>>,
    /// 可选的会话聚合存储。注入后成员/已读游标从 DB 加载和持久化，
    /// 替代纯内存状态，使多实例部署共享会话聚合视图。
    aggregate_store: Option<Arc<dyn ConversationAggregateStore>>,
    /// Normalized IM-side Agent assignments and dispatch correlations.
    agent_integration_store: Option<Arc<dyn AgentIntegrationStore>>,
    /// 可选的序列号分配器。注入后 message_seq 走 Redis INCRBY 批量预取，
    /// 消除 im_conversation_seq_counters 单行热点。
    seq_allocator: Option<Arc<dyn ConversationSeqAllocator>>,
    /// 可选的保留期协调存储。注入后在 indefinite retention 策略下清除过期标记。
    retention_scope_store: Option<Arc<dyn RetentionScopeStore>>,
    /// 可选的会话范围 durable realtime 发布器（TECH-16 message fanout）。
    realtime_publisher: Option<Arc<dyn RealtimeEventPublisher>>,
    /// 可选的私信访问门禁（social user block enforcement）。
    direct_message_access_gate: Option<Arc<dyn DirectMessageAccessGate>>,
    /// 可选的原子消息写入器（Postgres journal + message + outbox 单事务）。
    durable_message_post_writer: Option<Arc<dyn DurableMessagePostWriter>>,
    durable_message_mutation_writer: Option<Arc<dyn DurableMessageMutationWriter>>,
    durable_conversation_event_writer: Option<Arc<dyn DurableConversationEventWriter>>,
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn new(journal: J) -> Self {
        Self {
            journal,
            state: RwLock::new(RuntimeState::default()),
            metrics: ConversationRuntimeMetrics::default(),
            group_agent_default_policy: agents::GroupAgentDefaultPolicy::default(),
            message_store: None,
            outbox_store: None,
            id_generator: None,
            aggregate_store: None,
            agent_integration_store: None,
            seq_allocator: None,
            retention_scope_store: None,
            realtime_publisher: None,
            direct_message_access_gate: None,
            durable_message_post_writer: None,
            durable_message_mutation_writer: None,
            durable_conversation_event_writer: None,
        }
    }

    /// 注入消息真值存储，启用 DB seq 分配 + 真值写入路径。
    pub fn with_message_store(mut self, store: Arc<dyn MessageStore>) -> Self {
        self.message_store = Some(store);
        self
    }

    /// 注入 Outbox 存储，启用分布式事件投递。
    pub fn with_outbox_store(mut self, store: Arc<dyn OutboxStore>) -> Self {
        self.outbox_store = Some(store);
        self
    }

    /// 注入 ID 生成器，启用 Snowflake ID。
    pub fn with_id_generator(mut self, generator: Arc<dyn IdGenerator>) -> Self {
        self.id_generator = Some(generator);
        self
    }

    /// 注入会话聚合存储，启用 DB 持久化的成员/已读游标管理。
    /// 多实例部署时启用此选项以共享会话聚合视图。
    pub fn with_aggregate_store(mut self, store: Arc<dyn ConversationAggregateStore>) -> Self {
        self.aggregate_store = Some(store);
        self
    }

    pub fn with_agent_integration_store(mut self, store: Arc<dyn AgentIntegrationStore>) -> Self {
        self.agent_integration_store = Some(store);
        self
    }

    /// 注入序列号分配器，启用 Redis 批量预取的消息序号分配。
    pub fn with_seq_allocator(mut self, allocator: Arc<dyn ConversationSeqAllocator>) -> Self {
        self.seq_allocator = Some(allocator);
        self
    }

    pub fn with_retention_scope_store(mut self, store: Arc<dyn RetentionScopeStore>) -> Self {
        self.retention_scope_store = Some(store);
        self
    }

    /// 注入 Postgres 原子消息写入器（journal + message + outbox 单事务）。
    pub fn with_durable_message_post_writer(
        mut self,
        writer: Arc<dyn DurableMessagePostWriter>,
    ) -> Self {
        self.durable_message_post_writer = Some(writer);
        self
    }

    pub fn with_durable_message_mutation_writer(
        mut self,
        writer: Arc<dyn DurableMessageMutationWriter>,
    ) -> Self {
        self.durable_message_mutation_writer = Some(writer);
        self
    }

    pub fn with_durable_conversation_event_writer(
        mut self,
        writer: Arc<dyn DurableConversationEventWriter>,
    ) -> Self {
        self.durable_conversation_event_writer = Some(writer);
        self
    }

    /// 运行时是否已配置 DB 真值存储路径。
    pub fn has_message_store(&self) -> bool {
        self.message_store.is_some()
    }

    pub fn evict_idle_conversations(&self) -> usize {
        let max = resolve_max_conversations_in_memory();
        let max_bytes = resolve_conversation_cache_max_bytes();
        self.evict_idle_conversations_with_limits(max, max_bytes)
    }

    fn evict_idle_conversations_with_limits(
        &self,
        max_conversations: usize,
        max_bytes: usize,
    ) -> usize {
        let started = std::time::Instant::now();
        let mut state = write_runtime_state(&self.state, "runtime.state.evict_idle");
        state.refresh_dirty_conversation_weights();
        let over_count = state.conversations.len() > max_conversations;
        let over_bytes = state.estimated_conversation_bytes > max_bytes;
        let before_bytes = state.estimated_conversation_bytes;
        let evicted = state.evict_idle_conversations(max_conversations, max_bytes);
        let evicted_bytes = before_bytes.saturating_sub(state.estimated_conversation_bytes);
        drop(state);
        self.metrics.record_eviction_check(
            over_count,
            over_bytes,
            evicted,
            evicted_bytes,
            started.elapsed(),
        );
        evicted
    }

    fn load_normalized_conversation(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<NormalizedConversationRecord, RuntimeError> {
        self.load_normalized_conversation_current_state(tenant_id, organization_id, conversation_id)
            .map(|state| state.conversation)
    }

    fn load_normalized_conversation_current_state(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<NormalizedConversationCurrentState, RuntimeError> {
        let aggregate_store = self.aggregate_store.as_ref().ok_or_else(|| {
            RuntimeError::Contract(ContractError::Unavailable(
                "normalized conversation store is required for cold conversation reads".into(),
            ))
        })?;
        aggregate_store
            .load_conversation_current_state(tenant_id, organization_id, conversation_id)?
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.to_owned()))
    }

    fn ensure_conversation_loaded(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<(), RuntimeError> {
        let normalized_organization_id =
            im_domain_events::normalize_commit_organization_id(organization_id);
        let scope_key = conversation_scope_key(
            tenant_id,
            normalized_organization_id.as_str(),
            conversation_id,
        );
        {
            let state = read_runtime_state(&self.state, "runtime.state.ensure_conversation_local");
            if self.durable_conversation_event_writer.is_none()
                && state.conversations.contains_key(scope_key.as_str())
            {
                // In-memory/dev runtimes and focused adapter tests do not
                // claim a normalized Conversation writer. Their locally
                // committed hot state remains authoritative for that process.
                return Ok(());
            }
        }
        if self.aggregate_store.is_none() {
            let state = read_runtime_state(&self.state, "runtime.state.ensure_conversation_loaded");
            if state.conversations.contains_key(scope_key.as_str()) {
                return Ok(());
            }
        }
        let normalized_current_state = self.load_normalized_conversation_current_state(
            tenant_id,
            normalized_organization_id.as_str(),
            conversation_id,
        )?;
        let normalized_conversation = &normalized_current_state.conversation;
        let aggregate_store = self.aggregate_store.as_ref().ok_or_else(|| {
            RuntimeError::Contract(ContractError::Unavailable(
                "normalized conversation store is required for cold conversation reads".into(),
            ))
        })?;
        {
            let state = read_runtime_state(&self.state, "runtime.state.ensure_conversation_fresh");
            if let Some(conversation) = state.conversations.get(scope_key.as_str()) {
                // A local atomic commit can become visible in memory just
                // before this earlier row read is applied. Never regress it.
                if conversation.aggregate.commit_seq() > normalized_conversation.commit_seq {
                    return Ok(());
                }
            }
        }

        // Cold and stale-cache hydration stays bounded: one Conversation row
        // and one high-watermark query. Members and read cursors are loaded by
        // targeted lookup or explicit keyset pages at their call sites.
        let high_watermark = aggregate_store.load_high_watermark(
            tenant_id,
            normalized_organization_id.as_str(),
            conversation_id,
        )?;
        let mut state = write_runtime_state(&self.state, "ensure_conversation_loaded.normalized");
        if let Some(conversation) = state.conversations.get(scope_key.as_str()) {
            if conversation.aggregate.commit_seq() > normalized_conversation.commit_seq {
                return Ok(());
            }
            let mut candidate = conversation.clone();
            hydrate_normalized_conversation_aggregate(
                &mut candidate.aggregate,
                &normalized_current_state,
            )?;
            candidate.message_log.observe_high_watermark(high_watermark);
            let business_binding = candidate.aggregate.business_binding().cloned();
            state.insert_conversation(scope_key, candidate);
            if let Some(binding) = business_binding {
                state.business_index.insert(
                    conversation_business_scope_key(
                        tenant_id,
                        binding.business_type.as_str(),
                        binding.business_id.as_str(),
                    ),
                    conversation_id.to_owned(),
                );
            }
            return Ok(());
        }

        let mut aggregate = ConversationAggregateState::default();
        hydrate_normalized_conversation_aggregate(&mut aggregate, &normalized_current_state)?;
        let mut conversation_state = ConversationState {
            aggregate,
            last_accessed_at_ms: now_ms(),
            ..Default::default()
        };
        conversation_state
            .message_log
            .observe_high_watermark(high_watermark);
        let business_binding = conversation_state.aggregate.business_binding().cloned();
        state.insert_conversation(scope_key, conversation_state);
        if let Some(binding) = business_binding {
            state.business_index.insert(
                conversation_business_scope_key(
                    tenant_id,
                    binding.business_type.as_str(),
                    binding.business_id.as_str(),
                ),
                conversation_id.to_owned(),
            );
        }
        Ok(())
    }

    fn load_cold_conversation_for_creation(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<bool, RuntimeError> {
        if self.aggregate_store.is_none() || self.durable_conversation_event_writer.is_none() {
            return Ok(false);
        }
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        if read_runtime_state(&self.state, "runtime.state.creation_existing_local")
            .conversations
            .contains_key(scope_key.as_str())
        {
            return Ok(false);
        }
        match self.ensure_conversation_loaded(tenant_id, organization_id, conversation_id) {
            Ok(()) => Ok(true),
            Err(RuntimeError::ConversationNotFound(_)) => Ok(false),
            Err(error) => Err(error),
        }
    }

    fn ensure_message_loaded(
        &self,
        tenant_id: &str,
        organization_id: &str,
        message_id: &str,
    ) -> Result<String, RuntimeError> {
        let hot_conversation_id = {
            let state = read_runtime_state(&self.state, "runtime.state.ensure_message_loaded.hot");
            state
                .message_locator
                .conversation_id(tenant_id, message_id)
                .filter(|conversation_id| {
                    let scope_key =
                        conversation_scope_key(tenant_id, organization_id, conversation_id);
                    state
                        .conversations
                        .get(scope_key.as_str())
                        .and_then(|conversation| conversation.message_log.message(message_id))
                        .is_some()
                })
                .map(str::to_owned)
        };
        if let Some(conversation_id) = hot_conversation_id {
            return Ok(conversation_id);
        }

        let numeric_message_id = message_id
            .parse::<i64>()
            .map_err(|_| RuntimeError::MessageNotFound(message_id.to_owned()))?;
        let message_store = self
            .message_store
            .as_ref()
            .ok_or_else(|| RuntimeError::MessageNotFound(message_id.to_owned()))?;
        let record = message_store
            .read_message_by_id(tenant_id, numeric_message_id)?
            .ok_or_else(|| RuntimeError::MessageNotFound(message_id.to_owned()))?;
        let normalized_organization_id =
            im_domain_events::normalize_commit_organization_id(organization_id);
        if record.tenant_id != tenant_id
            || im_domain_events::normalize_commit_organization_id(record.organization_id.as_str())
                != normalized_organization_id
        {
            return Err(RuntimeError::MessageNotFound(message_id.to_owned()));
        }

        let stored = membership::stored_message_from_record(&record)?;
        let conversation_id = record.conversation_id.clone();
        self.ensure_conversation_loaded(
            tenant_id,
            normalized_organization_id.as_str(),
            conversation_id.as_str(),
        )?;
        let mut state = write_runtime_state(
            &self.state,
            "conversation-runtime.state.ensure_message_loaded.hydrate",
        );
        let scope_key = conversation_scope_key(
            tenant_id,
            normalized_organization_id.as_str(),
            conversation_id.as_str(),
        );
        state.touch_conversation(scope_key.as_str());
        let evicted_message_ids = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| RuntimeError::MessageNotFound(message_id.to_owned()))?
            .message_log
            .store_hydrated(stored);
        let evicted_message_count = evicted_message_ids.len();
        for evicted_message_id in &evicted_message_ids {
            state
                .message_locator
                .remove(tenant_id, evicted_message_id.as_str());
        }
        if !evicted_message_ids
            .iter()
            .any(|evicted_message_id| evicted_message_id == message_id)
        {
            state
                .message_locator
                .register(tenant_id, message_id, conversation_id.as_str());
        }
        drop(state);
        self.metrics.record_message_evictions(evicted_message_count);
        self.maybe_evict_after_write();
        Ok(conversation_id)
    }

    fn ensure_member_loaded(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Result<(), RuntimeError> {
        self.ensure_conversation_loaded(tenant_id, organization_id, conversation_id)?;
        let normalized_organization_id =
            im_domain_events::normalize_commit_organization_id(organization_id);
        let scope_key = conversation_scope_key(
            tenant_id,
            normalized_organization_id.as_str(),
            conversation_id,
        );
        // A durable aggregate store is the authority for permission-sensitive
        // membership.  Do not short-circuit on the local roster when it is
        // configured: another runtime instance may have removed or demoted
        // this principal since the conversation was cached here.
        let Some(aggregate_store) = self.aggregate_store.as_ref() else {
            return Ok(());
        };
        let Some(member_record) = aggregate_store.load_member(
            tenant_id,
            normalized_organization_id.as_str(),
            conversation_id,
            principal_kind,
            principal_id,
        )?
        else {
            // Once a durable aggregate store is configured, absence is an
            // authoritative denial.  Falling back to a hot/journal roster
            // here would let a hard-deleted member retain access until cache
            // eviction.  Runtimes without an aggregate store continue to use
            // the in-memory recovery path above.
            return Err(RuntimeError::PermissionDenied(format!(
                "principal is not active conversation member: {principal_kind}:{principal_id}"
            )));
        };
        if member_record.tenant_id != tenant_id
            || member_record.organization_id != normalized_organization_id
            || member_record.conversation_id != conversation_id
            || member_record.principal_kind != principal_kind
            || member_record.principal_id != principal_id
        {
            return Err(RuntimeError::Conflict(
                "normalized conversation member scope is inconsistent".into(),
            ));
        }
        let member = conversation_member_from_record(&member_record);
        {
            let mut state = write_runtime_state(
                &self.state,
                "conversation-runtime.state.ensure_member_loaded.authoritative",
            );
            let conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
            // Always replace the cached copy.  This updates role/state after
            // a cross-instance mutation instead of retaining the old roster
            // entry merely because it already exists in memory.
            conversation.roster.upsert_member(member.clone());
            state.sync_actor_inbox_member(normalized_organization_id.as_str(), &member);
            state.touch_conversation(scope_key.as_str());
        }
        self.maybe_evict_after_write();
        if !member.is_active() {
            return Err(RuntimeError::PermissionDenied(format!(
                "principal is not active conversation member: {principal_kind}:{principal_id}"
            )));
        }
        Ok(())
    }

    fn ensure_member_by_id_loaded(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        member_id: &str,
    ) -> Result<(), RuntimeError> {
        self.ensure_conversation_loaded(tenant_id, organization_id, conversation_id)?;
        let Some(aggregate_store) = self.aggregate_store.as_ref() else {
            return Ok(());
        };
        let numeric_member_id = member_id
            .parse::<i64>()
            .map_err(|_| RuntimeError::MemberNotFound(member_id.to_owned()))?;
        let normalized_organization_id =
            im_domain_events::normalize_commit_organization_id(organization_id);
        let member_record = aggregate_store
            .load_member_by_id(
                tenant_id,
                normalized_organization_id.as_str(),
                conversation_id,
                numeric_member_id,
            )?
            .ok_or_else(|| RuntimeError::MemberNotFound(member_id.to_owned()))?;
        if member_record.tenant_id != tenant_id
            || member_record.organization_id != normalized_organization_id
            || member_record.conversation_id != conversation_id
            || member_record.member_id != numeric_member_id
        {
            return Err(RuntimeError::Conflict(
                "normalized conversation member id scope is inconsistent".into(),
            ));
        }
        let member = conversation_member_from_record(&member_record);
        let scope_key = conversation_scope_key(
            tenant_id,
            normalized_organization_id.as_str(),
            conversation_id,
        );
        let mut state = write_runtime_state(
            &self.state,
            "conversation-runtime.state.ensure_member_by_id_loaded.authoritative",
        );
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        conversation.roster.upsert_member(member.clone());
        state.sync_actor_inbox_member(normalized_organization_id.as_str(), &member);
        state.touch_conversation(scope_key.as_str());
        drop(state);
        self.maybe_evict_after_write();
        Ok(())
    }

    fn ensure_read_cursor_loaded(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: Option<&str>,
    ) -> Result<(), RuntimeError> {
        self.ensure_member_loaded(
            tenant_id,
            organization_id,
            conversation_id,
            principal_kind,
            principal_id,
        )?;
        let Some(aggregate_store) = self.aggregate_store.as_ref() else {
            return Ok(());
        };
        let normalized_organization_id =
            im_domain_events::normalize_commit_organization_id(organization_id);
        let scope_key = conversation_scope_key(
            tenant_id,
            normalized_organization_id.as_str(),
            conversation_id,
        );
        let member = {
            let state = read_runtime_state(
                &self.state,
                "conversation-runtime.state.ensure_read_cursor.member",
            );
            let conversation = state
                .conversations
                .get(scope_key.as_str())
                .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
            resolve_active_member_with_kind(conversation, principal_id, principal_kind)?
        };
        let numeric_member_id = member.member_id.parse::<i64>().map_err(|_| {
            RuntimeError::Conflict(format!(
                "normalized member id is not a signed integer: {}",
                member.member_id
            ))
        })?;
        let requested_device_id = device_id.unwrap_or_default();
        let cursor = aggregate_store
            .load_read_cursor_for_device(
                tenant_id,
                normalized_organization_id.as_str(),
                conversation_id,
                numeric_member_id,
                requested_device_id,
            )?
            .map(|record| {
                if record.tenant_id != tenant_id
                    || record.organization_id != normalized_organization_id
                    || record.conversation_id != conversation_id
                    || record.member_id != numeric_member_id
                    || record.principal_kind != principal_kind
                    || record.principal_id != principal_id
                    || (!record.device_id.is_empty() && record.device_id != requested_device_id)
                {
                    return Err(RuntimeError::Conflict(
                        "normalized read cursor scope is inconsistent".into(),
                    ));
                }
                Ok(read_cursor_from_record(&record))
            })
            .transpose()?
            .unwrap_or_else(|| {
                let mut cursor = build_default_read_cursor(&member);
                cursor.device_id = device_id.map(str::to_owned);
                cursor
            });

        let mut state = write_runtime_state(
            &self.state,
            "conversation-runtime.state.ensure_read_cursor.normalized",
        );
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        conversation.roster.upsert_read_cursor(cursor);
        state.touch_conversation(scope_key.as_str());
        Ok(())
    }

    fn maybe_evict_after_write(&self) {
        let max = resolve_max_conversations_in_memory();
        let max_bytes = resolve_conversation_cache_max_bytes();
        self.evict_idle_conversations_with_limits(max, max_bytes);
    }

    /// Persists cached members and read cursors for explicit local recovery
    /// and test workflows. Production commands use the normalized writer.
    pub fn persist_aggregate_state(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<(), RuntimeError> {
        let store = self
            .aggregate_store
            .as_ref()
            .ok_or_else(|| RuntimeError::InvalidInput("aggregate_store not configured".into()))?;
        let state = read_runtime_state(&self.state, "persist_aggregate_state");
        let conversation = state
            .conversations
            .get(conversation_scope_key(tenant_id, organization_id, conversation_id).as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        let members = conversation
            .roster
            .members()
            .values()
            .map(|member| member_to_record(tenant_id, organization_id, conversation_id, member))
            .collect();
        let read_cursors = conversation
            .roster
            .read_cursors()
            .values()
            .map(|cursor| cursor_to_record(tenant_id, organization_id, conversation_id, cursor))
            .collect();
        persist_aggregate_records(store.as_ref(), members, read_cursors)
    }

    fn persist_normalized_conversation_commit_with_assignments(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        conversation: &ConversationState,
        agent_assignments: Option<ReplaceConversationAgentAssignments>,
        envelopes: Vec<CommitEnvelope>,
    ) -> Result<(), RuntimeError> {
        let members = conversation
            .roster
            .members()
            .values()
            .map(|member| member_to_record(tenant_id, organization_id, conversation_id, member))
            .collect();
        let read_cursors = conversation
            .roster
            .read_cursors()
            .values()
            .map(|cursor| cursor_to_record(tenant_id, organization_id, conversation_id, cursor))
            .collect();
        self.persist_normalized_conversation_changes_with_assignments(
            tenant_id,
            organization_id,
            conversation_id,
            conversation,
            members,
            read_cursors,
            agent_assignments,
            envelopes,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn persist_normalized_conversation_changes(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        conversation: &ConversationState,
        members: Vec<ConversationMemberRecord>,
        read_cursors: Vec<ReadCursorRecord>,
        envelopes: Vec<CommitEnvelope>,
    ) -> Result<(), RuntimeError> {
        self.persist_normalized_conversation_changes_with_assignments(
            tenant_id,
            organization_id,
            conversation_id,
            conversation,
            members,
            read_cursors,
            None,
            envelopes,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn persist_normalized_conversation_changes_with_assignments(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        conversation: &ConversationState,
        members: Vec<ConversationMemberRecord>,
        read_cursors: Vec<ReadCursorRecord>,
        agent_assignments: Option<ReplaceConversationAgentAssignments>,
        envelopes: Vec<CommitEnvelope>,
    ) -> Result<(), RuntimeError> {
        let Some(writer) = self.durable_conversation_event_writer.as_ref() else {
            if self.outbox_store.is_some() {
                return Err(RuntimeError::Conflict(
                    "durable conversation outbox persistence requires an atomic normalized conversation writer"
                        .into(),
                ));
            }
            self.journal.append_batch(envelopes)?;
            if let Some(store) = self.aggregate_store.as_ref() {
                persist_aggregate_records(store.as_ref(), members, read_cursors)?;
            }
            return Ok(());
        };
        let last_activity_at = envelopes
            .last()
            .map(|envelope| envelope.committed_at.clone())
            .ok_or_else(|| RuntimeError::InvalidInput("conversation commit is empty".into()))?;
        let lifecycle_state = match conversation.aggregate.lifecycle_state() {
            ConversationLifecycleState::Active => "active",
            ConversationLifecycleState::Archived => "archived",
        };
        let outboxes = envelopes
            .iter()
            .map(|envelope| {
                self.build_conversation_event_outbox_record(ConversationRealtimeEvent {
                    tenant_id,
                    organization_id,
                    conversation_id,
                    event_type: envelope.event_type.as_str(),
                    journal_event_id: envelope.event_id.as_str(),
                    payload_json: envelope.payload.clone(),
                    occurred_at: envelope.occurred_at.as_str(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let expected_commit_seq = envelopes
            .first()
            .ok_or_else(|| RuntimeError::InvalidInput("conversation commit is empty".into()))?
            .ordering_seq
            .checked_sub(1);
        let policy = normalized_policy_record(
            tenant_id,
            organization_id,
            conversation_id,
            &conversation.aggregate,
        );
        let business_binding = normalized_business_binding_record(
            tenant_id,
            organization_id,
            conversation_id,
            &conversation.aggregate,
        );
        let handoff = normalized_handoff_record(
            tenant_id,
            organization_id,
            conversation_id,
            &conversation.aggregate,
        );
        writer
            .persist_normalized_conversation_commit(NormalizedConversationCommit {
                expected_commit_seq,
                conversation: NormalizedConversationRecord {
                    tenant_id: tenant_id.to_owned(),
                    organization_id: organization_id.to_owned(),
                    conversation_id: conversation_id.to_owned(),
                    conversation_type: conversation.aggregate.conversation_type().to_owned(),
                    lifecycle_state: lifecycle_state.into(),
                    archived_at: conversation.aggregate.archived_at().map(str::to_owned),
                    archive_event_id: conversation.aggregate.archive_event_id().map(str::to_owned),
                    commit_seq: conversation.aggregate.commit_seq(),
                    member_epoch: conversation.aggregate.member_epoch(),
                    last_activity_at,
                    retention_until: None,
                },
                policy,
                business_binding,
                handoff,
                members,
                read_cursors,
                agent_assignments,
                envelopes: envelopes.clone(),
                outboxes,
            })
            .map_err(RuntimeError::from)?;
        for envelope in &envelopes {
            crate::conversation_state::refresh_conversation_cache(envelope);
        }
        Ok(())
    }

    fn persist_message_mutation_commit(
        &self,
        envelope: CommitEnvelope,
        mutation: StoredMessageMutation,
        realtime_payload_json: String,
        local_change: bool,
    ) -> Result<bool, RuntimeError> {
        if let Some(writer) = self.durable_message_mutation_writer.as_ref() {
            let outbox = self
                .build_message_mutation_outbox_record(
                    envelope.tenant_id.as_str(),
                    envelope.organization_id.as_str(),
                    envelope.aggregate_id.as_str(),
                    envelope.event_type.as_str(),
                    envelope.event_id.as_str(),
                    realtime_payload_json,
                )?
                .ok_or_else(|| {
                    RuntimeError::Conflict(
                        "durable message mutation requires transactional outbox configuration"
                            .into(),
                    )
                })?;
            return writer
                .persist_message_mutation(envelope, mutation, outbox)
                .map(|position| position.is_some())
                .map_err(RuntimeError::from);
        }
        if !local_change {
            return Ok(false);
        }
        if self.message_store.is_some() || self.outbox_store.is_some() {
            return Err(RuntimeError::Conflict(
                "durable message mutation requires an atomic normalized message writer".into(),
            ));
        }
        self.journal.append(envelope)?;
        Ok(true)
    }

    pub fn post_message(
        &self,
        command: PostMessageCommand,
    ) -> Result<PostMessageResult, RuntimeError> {
        self.post_message_with_policy(command, MessagePostPolicy::GenericPost, None)
    }

    pub fn post_agent_dispatch_reply(
        &self,
        command: PostMessageCommand,
        completion: AgentDispatchReplyCompletion,
    ) -> Result<AgentReplyCommitResult, RuntimeError> {
        let result = self.post_message_with_policy(
            command,
            MessagePostPolicy::AgentDispatchReply,
            Some(completion),
        )?;
        let reply_message_id = result.message_id.parse::<u64>().map_err(|_| {
            RuntimeError::Conflict("agent reply message id is not a Snowflake integer".into())
        })?;
        Ok(AgentReplyCommitResult {
            reply_message_id,
            reply_message_seq: result.message_seq,
        })
    }

    pub fn publish_system_channel_message(
        &self,
        command: PublishSystemChannelMessageCommand,
    ) -> Result<PostMessageResult, RuntimeError> {
        self.post_message_with_policy(
            PostMessageCommand {
                tenant_id: command.tenant_id.clone(),
                organization_id: command.organization_id.clone(),
                conversation_id: command.conversation_id,
                sender: command.publisher,
                client_msg_id: command.client_msg_id,
                message_type: MessageType::Standard,
                body: command.body,
            },
            MessagePostPolicy::SystemChannelPublish,
            None,
        )
    }

    fn post_message_with_policy(
        &self,
        command: PostMessageCommand,
        policy: MessagePostPolicy,
        dispatch_completion: Option<AgentDispatchReplyCompletion>,
    ) -> Result<PostMessageResult, RuntimeError> {
        validate_payload_size(
            "conversationId",
            command.conversation_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_sender_payload_size("sender", &command.sender)?;
        validate_optional_payload_size(
            "clientMsgId",
            command.client_msg_id.as_deref(),
            MESSAGE_CLIENT_MSG_ID_MAX_BYTES,
        )?;
        validate_message_body_contract(&command.body)?;
        match policy {
            MessagePostPolicy::AgentDispatchReply => {
                self.ensure_conversation_loaded(
                    command.tenant_id.as_str(),
                    command.organization_id.as_str(),
                    command.conversation_id.as_str(),
                )?;
                self.hydrate_conversation_agent_metadata_if_missing(
                    command.tenant_id.as_str(),
                    command.organization_id.as_str(),
                    command.conversation_id.as_str(),
                )?;
            }
            MessagePostPolicy::GenericPost | MessagePostPolicy::SystemChannelPublish => {
                self.ensure_member_loaded(
                    command.tenant_id.as_str(),
                    command.organization_id.as_str(),
                    command.conversation_id.as_str(),
                    command.sender.kind.as_str(),
                    command.sender.id.as_str(),
                )?;
            }
        }
        if agents::message_has_agent_mentions(&command.body) {
            self.hydrate_conversation_agent_metadata_if_missing(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                command.conversation_id.as_str(),
            )?;
        }
        let request_key = post_message_request_key(&command);
        let scope_key = conversation_scope_key(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
        );
        let mutation = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.post_message");
            state.touch_conversation(scope_key.as_str());
            let mutation = {
                let conversation =
                    state
                        .conversations
                        .get_mut(scope_key.as_str())
                        .ok_or_else(|| {
                            RuntimeError::ConversationNotFound(command.conversation_id.clone())
                        })?;
                if let Some(request_key) = request_key.as_ref()
                    && let Some(existing) = conversation.posted_message_requests.get(request_key)
                {
                    if !posted_message_replay_matches(existing, &command) {
                        return Err(RuntimeError::Conflict(format!(
                            "message post request conflicts with existing message idempotency key: {request_key}"
                        )));
                    }
                    let stored = conversation
                        .message_log
                        .message(existing.message_id.as_str())
                        .ok_or_else(|| {
                            RuntimeError::Conflict(format!(
                                "replayed message id missing from message log: {}",
                                existing.message_id
                            ))
                        })?;
                    PostMessageMutation::Replayed(PostMessageResult::replayed(
                        existing.message_id.clone(),
                        stored.message.message_seq,
                        format!("evt_{}_posted", existing.message_id),
                        Some(request_key.clone()),
                    ))
                } else {
                    'post_new: {
                        if let (Some(store), Some(client_msg_id)) = (
                            self.message_store.as_ref(),
                            command
                                .client_msg_id
                                .as_deref()
                                .filter(|value| !value.trim().is_empty()),
                        ) && let Some(stored) = store.read_message_by_client_id(
                            command.tenant_id.as_str(),
                            command.organization_id.as_str(),
                            command.conversation_id.as_str(),
                            command.sender.kind.as_str(),
                            command.sender.id.as_str(),
                            client_msg_id,
                        )? {
                            if !durable_posted_message_replay_matches(&stored, &command)? {
                                return Err(RuntimeError::Conflict(
                                    "message post request conflicts with existing durable client message id"
                                        .into(),
                                ));
                            }
                            break 'post_new PostMessageMutation::Replayed(
                                PostMessageResult::replayed(
                                    stored.message_id.to_string(),
                                    stored.message_seq,
                                    format!("evt_{}_posted", stored.message_id),
                                    request_key.clone(),
                                ),
                            );
                        }
                        ensure_conversation_write_allowed(conversation)?;
                        let sender_member = if policy == MessagePostPolicy::AgentDispatchReply {
                            None
                        } else {
                            let sender_member = resolve_active_member_with_kind(
                                conversation,
                                command.sender.id.as_str(),
                                command.sender.kind.as_str(),
                            )?;
                            policy::ensure_actor_kind_matches_member(
                                &sender_member,
                                command.sender.kind.as_str(),
                            )?;
                            Some(sender_member)
                        };
                        match policy {
                            MessagePostPolicy::GenericPost => {
                                let sender_member = sender_member.as_ref().ok_or_else(|| {
                                    RuntimeError::Conflict(
                                        "generic post is missing an active sender member".into(),
                                    )
                                })?;
                                policy::ensure_message_post_allowed(conversation, sender_member)?;
                                policy::ensure_room_message_post_allowed(
                                    conversation,
                                    sender_member,
                                )?;
                                direct_message_access::ensure_direct_message_post_allowed(
                                    command.tenant_id.as_str(),
                                    command.organization_id.as_str(),
                                    conversation,
                                    sender_member,
                                    self.resolve_direct_message_access_gate().as_deref(),
                                )?;
                            }
                            MessagePostPolicy::SystemChannelPublish => {
                                let sender_member = sender_member.as_ref().ok_or_else(|| {
                                    RuntimeError::Conflict(
                                        "system publish is missing an active sender member".into(),
                                    )
                                })?;
                                policy::ensure_system_channel_publish_command_allowed(
                                    conversation,
                                    sender_member,
                                )?
                            }
                            MessagePostPolicy::AgentDispatchReply => {
                                validate_agent_dispatch_reply(
                                    conversation,
                                    &command,
                                    dispatch_completion.as_ref(),
                                )?;
                            }
                        }
                        let resolved_agent_mentions = if policy
                            == MessagePostPolicy::AgentDispatchReply
                        {
                            if agents::message_has_agent_mentions(&command.body) {
                                return Err(RuntimeError::InvalidInput(
                                    "agent dispatch replies cannot mention another agent".into(),
                                ));
                            }
                            Vec::new()
                        } else {
                            agents::resolve_message_agent_mentions(
                                conversation,
                                sender_member.as_ref().ok_or_else(|| {
                                    RuntimeError::Conflict(
                                        "message post is missing an active sender member".into(),
                                    )
                                })?,
                                &command.body,
                            )?
                        };
                        // Per-conversation ordinal seq: Redis batch prefetch or Postgres counter.
                        let message_seq = if let Some(allocator) = &self.seq_allocator {
                            allocator
                                .allocate_seq(
                                    command.tenant_id.as_str(),
                                    command.organization_id.as_str(),
                                    command.conversation_id.as_str(),
                                )
                                .map_err(RuntimeError::from)?
                        } else if let Some(store) = &self.message_store {
                            store
                                .allocate_message_seq(
                                    command.tenant_id.as_str(),
                                    command.organization_id.as_str(),
                                    command.conversation_id.as_str(),
                                )
                                .map_err(RuntimeError::from)?
                        } else {
                            conversation.message_log.high_watermark() + 1
                        };

                        let mut sender = command.sender.clone();
                        if sender.member_id.is_none()
                            && let Some(sender_member) = sender_member.as_ref()
                        {
                            sender.member_id = Some(sender_member.member_id.clone());
                        }

                        // ID 生成：优先使用 Snowflake，fallback 到确定性字符串拼接
                        let message_id = if let Some(generator) = &self.id_generator {
                            generator.next_id().map_err(RuntimeError::from)?.to_string()
                        } else {
                            generated_message_id(command.conversation_id.as_str(), message_seq)
                        };
                        let message_timestamp = conversation_timestamp();
                        let message = Message {
                            tenant_id: command.tenant_id.clone(),
                            conversation_id: command.conversation_id.clone(),
                            message_id: message_id.clone(),
                            message_seq,
                            sender,
                            message_type: command.message_type.clone(),
                            delivery_mode: "discrete".into(),
                            client_msg_id: command.client_msg_id.clone(),
                            stream_session_id: None,
                            rtc_session_id: rtc_session_id_from_signal_message(&command),
                            body: command.body.clone(),
                            attributes: BTreeMap::new(),
                            metadata: BTreeMap::new(),
                            occurred_at: message_timestamp.clone(),
                            committed_at: Some(message_timestamp),
                        };
                        let event_id = if let Some(generator) = &self.id_generator {
                            generator.next_id().map_err(RuntimeError::from)?.to_string()
                        } else {
                            format!("evt_{}_posted", message.message_id)
                        };
                        let retention_class = conversation_retention_class(conversation);
                        let retention_until = retention_until_from_envelope(
                            retention_class.as_str(),
                            message.occurred_at.as_str(),
                        );
                        // Allocate ordering slots without mutating the live
                        // aggregate. A durable journal/outbox failure must
                        // not leave a phantom commit sequence in memory.
                        let journal_ordering_seq = conversation
                            .aggregate
                            .commit_seq()
                            .checked_add(1)
                            .ok_or_else(|| {
                                RuntimeError::Conflict(
                                    "conversation journal ordering sequence overflow".into(),
                                )
                            })?;
                        let mut last_ordering_seq = journal_ordering_seq;
                        let envelope = CommitEnvelope {
                            event_id: event_id.clone(),
                            tenant_id: command.tenant_id.clone(),
                            organization_id: command.organization_id.clone(),
                            event_type: "message.posted".into(),
                            event_version: 1,
                            aggregate_type: AggregateType::Conversation,
                            aggregate_id: command.conversation_id.clone(),
                            scope_type: "conversation".into(),
                            scope_id: command.conversation_id.clone(),
                            ordering_key: CommitEnvelope::ordering_key(
                                command.tenant_id.as_str(),
                                command.conversation_id.as_str(),
                            ),
                            ordering_seq: journal_ordering_seq,
                            causation_id: None,
                            correlation_id: None,
                            idempotency_key: command.client_msg_id.clone(),
                            actor: EventActor {
                                actor_id: message.sender.id.clone(),
                                actor_kind: message.sender.kind.clone(),
                                actor_session_id: message.sender.session_id.clone(),
                            },
                            occurred_at: message.occurred_at.clone(),
                            committed_at: message
                                .committed_at
                                .clone()
                                .unwrap_or_else(|| message.occurred_at.clone()),
                            payload_schema: Some("message.posted.v1".into()),
                            payload: runtime_json_string(&message)?,
                            retention_class: retention_class.clone(),
                            audit_class: "default".into(),
                        };

                        let mut journal_envelopes = vec![envelope];
                        let mut outboxes = Vec::new();
                        let mut agent_dispatch_request = None;
                        if let Some(outbox) = self.build_message_posted_outbox_record(
                            command.tenant_id.as_str(),
                            command.organization_id.as_str(),
                            &message,
                        )? {
                            outboxes.push(outbox);
                        }
                        if !resolved_agent_mentions.is_empty() {
                            let dispatch_ordering_seq =
                                journal_ordering_seq.checked_add(1).ok_or_else(|| {
                                    RuntimeError::Conflict(
                                        "conversation journal ordering sequence overflow".into(),
                                    )
                                })?;
                            last_ordering_seq = dispatch_ordering_seq;
                            if let Some(dispatch) = self.build_agent_mention_dispatch_artifacts(
                                command.organization_id.as_str(),
                                &message,
                                event_id.as_str(),
                                resolved_agent_mentions.as_slice(),
                                dispatch_ordering_seq,
                                retention_class.as_str(),
                            )? {
                                agent_dispatch_request = Some(dispatch.request);
                                if let Some(outbox) = dispatch.outbox {
                                    outboxes.push(outbox);
                                }
                                journal_envelopes.push(dispatch.envelope);
                            }
                        }

                        let stored_record = StoredMessageRecord {
                            tenant_id: message.tenant_id.clone(),
                            organization_id: command.organization_id.clone(),
                            conversation_id: message.conversation_id.clone(),
                            message_id: message.message_id.parse::<i64>().unwrap_or(0),
                            message_seq: message.message_seq,
                            sender_principal_kind: message.sender.kind.clone(),
                            sender_principal_id: message.sender.id.clone(),
                            sender_device_id: message.sender.device_id.clone(),
                            client_msg_id: message.client_msg_id.clone(),
                            message_type: message.message_type.as_wire_value().to_owned(),
                            payload_json: runtime_json_string(&message.body)?,
                            payload_hash: sha256_message_hash(&message.body),
                            created_at: message.occurred_at.clone(),
                            updated_at: message.occurred_at.clone(),
                            deleted_at: None,
                            retention_until,
                            reactions: Vec::new(),
                            pin: None,
                        };

                        if let Some(writer) = &self.durable_message_post_writer {
                            if let Some(completion) = dispatch_completion.clone() {
                                writer
                                    .persist_agent_reply_and_complete_dispatch(
                                        journal_envelopes,
                                        stored_record,
                                        outboxes,
                                        completion,
                                    )
                                    .map_err(RuntimeError::from)?;
                            } else {
                                writer
                                    .persist_message_post_batch_with_agent_dispatch(
                                        journal_envelopes,
                                        stored_record,
                                        outboxes,
                                        agent_dispatch_request,
                                        10,
                                    )
                                    .map_err(RuntimeError::from)?;
                            }
                        } else {
                            if self.message_store.is_some() || self.outbox_store.is_some() {
                                return Err(RuntimeError::Conflict(
                                    "durable message or outbox persistence requires an atomic durable message writer"
                                        .into(),
                                ));
                            }
                            if dispatch_completion.is_some() {
                                return Err(RuntimeError::Conflict(
                                    "agent dispatch reply requires an atomic durable writer".into(),
                                ));
                            }
                            self.journal.append_batch(journal_envelopes)?;
                        }

                        // Publish the in-memory watermark only after the
                        // journal/message/outbox transaction has committed.
                        conversation.aggregate.observe_commit_seq(last_ordering_seq);

                        let evicted_message_ids =
                            conversation.message_log.store_posted(message.clone());
                        if let Some(request_key) = request_key.as_ref() {
                            conversation.posted_message_requests.insert(
                                request_key.clone(),
                                PostedMessageReplayRecord {
                                    sender_id: command.sender.id.clone(),
                                    sender_kind: command.sender.kind.clone(),
                                    message_type: command.message_type.clone(),
                                    body: command.body.clone(),
                                    message_id: message_id.clone(),
                                },
                            );
                        }
                        PostMessageMutation::Applied {
                            result: PostMessageResult::applied(
                                message_id,
                                message_seq,
                                event_id,
                                request_key.clone(),
                            ),
                            message,
                            evicted_message_ids,
                        }
                    }
                }
            };

            if let PostMessageMutation::Applied {
                message,
                evicted_message_ids,
                ..
            } = &mutation
            {
                self.metrics
                    .record_message_evictions(evicted_message_ids.len());
                for message_id in evicted_message_ids {
                    state
                        .message_locator
                        .remove(message.tenant_id.as_str(), message_id.as_str());
                }
                state.message_locator.register_message(message);
            }

            mutation
        };

        match mutation {
            PostMessageMutation::Replayed(result) => Ok(result),
            PostMessageMutation::Applied {
                result, message, ..
            } => {
                let publish_result = self.publish_message_posted_realtime(
                    command.tenant_id.as_str(),
                    command.organization_id.as_str(),
                    &message,
                );
                if let Err(error) = publish_result {
                    if policy == MessagePostPolicy::AgentDispatchReply
                        && self.durable_message_post_writer.is_some()
                    {
                        tracing::warn!(
                            conversation_id = %command.conversation_id,
                            message_id = %message.message_id,
                            error = ?error,
                            "agent reply realtime publish failed after durable outbox commit"
                        );
                    } else {
                        return Err(error);
                    }
                }
                self.maybe_evict_after_write();
                Ok(result)
            }
        }
    }

    pub fn edit_message(
        &self,
        command: EditMessageCommand,
    ) -> Result<MessageMutationResult, RuntimeError> {
        validate_payload_size(
            "messageId",
            command.message_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_sender_payload_size("editor", &command.editor)?;
        validate_message_body_contract(&command.body)?;
        let conversation_id = self.ensure_message_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.message_id.as_str(),
        )?;
        self.ensure_member_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            conversation_id.as_str(),
            command.editor.kind.as_str(),
            command.editor.id.as_str(),
        )?;
        let edited = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.edit_message");
            let scope_key = conversation_scope_key(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                conversation_id.as_str(),
            );
            let live_conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let mut candidate = live_conversation.clone();
            let conversation = &mut candidate;
            let editor_member = resolve_active_member_with_kind(
                conversation,
                command.editor.id.as_str(),
                command.editor.kind.as_str(),
            )?;
            policy::ensure_actor_kind_matches_member(&editor_member, command.editor.kind.as_str())?;
            let scenario = conversation.aggregate.scenario();
            let handoff_closed = conversation.aggregate.has_closed_handoff();
            let stored = conversation
                .message_log
                .message(command.message_id.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            if stored.recalled {
                return Err(RuntimeError::MessageAlreadyRecalled(command.message_id));
            }
            policy::ensure_message_edit_allowed(
                command.editor.id.as_str(),
                &editor_member,
                scenario,
                handoff_closed,
                &stored.message,
            )?;
            if let Some(idempotency_key) = command
                .idempotency_key
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
            {
                let request_key = message_mutation_request_key(&command.editor, idempotency_key);
                if let Some(existing) = conversation.message_mutation_requests.get(&request_key) {
                    if existing.result.message_id == command.message_id {
                        return Ok(existing.result.clone());
                    }
                    return Err(RuntimeError::Conflict(format!(
                        "message edit request conflicts with existing idempotency key: {request_key}"
                    )));
                }
            }
            let conversation_id = stored.message.conversation_id.clone();
            let message_id = stored.message.message_id.clone();
            let message_seq = stored.message.message_seq;

            let mut editor = command.editor.clone();
            if editor.member_id.is_none() {
                editor.member_id = Some(editor_member.member_id.clone());
            }

            let edited_at = conversation_timestamp();
            let edited = MessageEdited {
                tenant_id: command.tenant_id.clone(),
                conversation_id,
                message_id,
                message_seq,
                body: command.body,
                editor,
                edited_at,
            };
            let retention_class = conversation_retention_class(conversation);
            let ordering_seq = conversation
                .aggregate
                .commit_seq()
                .checked_add(1)
                .ok_or_else(|| {
                    RuntimeError::Conflict("conversation journal ordering sequence overflow".into())
                })?;
            let event_id = format!("evt_{}_edited_{ordering_seq}", edited.message_id);
            let mut envelope = build_message_edited_envelope(
                &edited,
                command.organization_id.as_str(),
                event_id.as_str(),
                ordering_seq,
                retention_class.as_str(),
            );
            envelope.idempotency_key = command.idempotency_key.clone();
            let normalized_body = edited.body.clone().with_derived_summary();
            let mutation = StoredMessageMutation::Edited {
                target: StoredMessageMutationTarget {
                    tenant_id: command.tenant_id.clone(),
                    organization_id: command.organization_id.clone(),
                    conversation_id: edited.conversation_id.clone(),
                    message_id: edited.message_id.clone(),
                    message_seq: edited.message_seq,
                },
                payload_json: runtime_json_string(&normalized_body)?,
                payload_hash: sha256_message_hash(&normalized_body),
                edited_at: edited.edited_at.clone(),
            };
            let realtime_payload = runtime_json_string(&json!({
                "conversationId": edited.conversation_id,
                "messageId": edited.message_id,
                "messageSeq": edited.message_seq,
                "summary": normalized_body
                    .summary_or_derived()
                    .unwrap_or_else(|| "[message]".into()),
            }))?;
            let evicted_message_ids = conversation
                .message_log
                .apply_edited(&edited)
                .ok_or_else(|| RuntimeError::MessageNotFound(edited.message_id.clone()))?
                .evicted_message_ids;
            if !self.persist_message_mutation_commit(
                envelope,
                mutation,
                realtime_payload.clone(),
                true,
            )? {
                return Err(RuntimeError::Conflict(
                    "message edit did not advance normalized state".into(),
                ));
            }
            conversation.aggregate.observe_commit_seq(ordering_seq);
            *live_conversation = candidate;
            self.metrics
                .record_message_evictions(evicted_message_ids.len());
            for message_id in &evicted_message_ids {
                state
                    .message_locator
                    .remove(command.tenant_id.as_str(), message_id.as_str());
            }
            state.touch_conversation(scope_key.as_str());
            (edited, event_id, realtime_payload)
        };

        let (edited, event_id, realtime_payload) = edited;

        self.publish_message_mutation_realtime_after_commit(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            edited.conversation_id.as_str(),
            "message.edited",
            realtime_payload,
        )?;
        if let Some(idempotency_key) = command
            .idempotency_key
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            let mut state = write_runtime_state(
                &self.state,
                "conversation-runtime.state.edit_message.idempotency",
            );
            let scope_key = conversation_scope_key(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                edited.conversation_id.as_str(),
            );
            if let Some(conversation) = state.conversations.get_mut(scope_key.as_str()) {
                conversation.message_mutation_requests.insert(
                    message_mutation_request_key(&command.editor, idempotency_key),
                    MessageMutationReplayRecord {
                        result: MessageMutationResult {
                            conversation_id: edited.conversation_id.clone(),
                            message_id: edited.message_id.clone(),
                            message_seq: edited.message_seq,
                            event_id: event_id.clone(),
                        },
                    },
                );
            }
        }
        self.maybe_evict_after_write();
        Ok(MessageMutationResult {
            conversation_id: edited.conversation_id,
            message_id: edited.message_id,
            message_seq: edited.message_seq,
            event_id,
        })
    }

    pub fn recall_message(
        &self,
        command: RecallMessageCommand,
    ) -> Result<MessageMutationResult, RuntimeError> {
        validate_payload_size(
            "messageId",
            command.message_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_sender_payload_size("recalledBy", &command.recalled_by)?;
        let conversation_id = self.ensure_message_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.message_id.as_str(),
        )?;
        self.ensure_member_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            conversation_id.as_str(),
            command.recalled_by.kind.as_str(),
            command.recalled_by.id.as_str(),
        )?;
        let recalled = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.recall_message");
            let scope_key = conversation_scope_key(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                conversation_id.as_str(),
            );
            let live_conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let mut candidate = live_conversation.clone();
            let conversation = &mut candidate;
            let recalled_member = resolve_active_member_with_kind(
                conversation,
                command.recalled_by.id.as_str(),
                command.recalled_by.kind.as_str(),
            )?;
            policy::ensure_actor_kind_matches_member(
                &recalled_member,
                command.recalled_by.kind.as_str(),
            )?;
            let scenario = conversation.aggregate.scenario();
            let handoff_closed = conversation.aggregate.has_closed_handoff();
            let stored = conversation
                .message_log
                .message(command.message_id.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            if let Some(idempotency_key) = command
                .idempotency_key
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
            {
                let request_key =
                    message_mutation_request_key(&command.recalled_by, idempotency_key);
                if let Some(existing) = conversation.message_mutation_requests.get(&request_key) {
                    if existing.result.message_id == command.message_id {
                        return Ok(existing.result.clone());
                    }
                    return Err(RuntimeError::Conflict(format!(
                        "message recall request conflicts with existing idempotency key: {request_key}"
                    )));
                }
            }
            if stored.recalled {
                return Err(RuntimeError::MessageAlreadyRecalled(command.message_id));
            }
            policy::ensure_message_recall_allowed(
                command.recalled_by.id.as_str(),
                &recalled_member,
                scenario,
                handoff_closed,
                &stored.message,
            )?;
            let conversation_id = stored.message.conversation_id.clone();
            let message_id = stored.message.message_id.clone();
            let message_seq = stored.message.message_seq;

            let mut recalled_by = command.recalled_by.clone();
            if recalled_by.member_id.is_none() {
                recalled_by.member_id = Some(recalled_member.member_id.clone());
            }

            let recalled_at = conversation_timestamp();
            let recalled = MessageRecalled {
                tenant_id: command.tenant_id.clone(),
                conversation_id,
                message_id,
                message_seq,
                recalled_by,
                recalled_at,
            };
            let retention_class = conversation_retention_class(conversation);
            let ordering_seq = conversation
                .aggregate
                .commit_seq()
                .checked_add(1)
                .ok_or_else(|| {
                    RuntimeError::Conflict("conversation journal ordering sequence overflow".into())
                })?;
            let event_id = format!("evt_{}_recalled_{ordering_seq}", recalled.message_id);
            let mut envelope = build_message_recalled_envelope(
                &recalled,
                command.organization_id.as_str(),
                event_id.as_str(),
                ordering_seq,
                retention_class.as_str(),
            );
            envelope.idempotency_key = command.idempotency_key.clone();
            let mutation = StoredMessageMutation::Recalled {
                target: StoredMessageMutationTarget {
                    tenant_id: command.tenant_id.clone(),
                    organization_id: command.organization_id.clone(),
                    conversation_id: recalled.conversation_id.clone(),
                    message_id: recalled.message_id.clone(),
                    message_seq: recalled.message_seq,
                },
                recalled_at: recalled.recalled_at.clone(),
            };
            let realtime_payload = runtime_json_string(&json!({
                "conversationId": recalled.conversation_id.clone(),
                "messageId": recalled.message_id.clone(),
                "messageSeq": recalled.message_seq,
                "summary": "[recalled]",
            }))?;
            let evicted_message_ids = conversation
                .message_log
                .apply_recalled(&recalled)
                .ok_or_else(|| RuntimeError::MessageNotFound(recalled.message_id.clone()))?
                .evicted_message_ids;
            let applied = self.persist_message_mutation_commit(
                envelope,
                mutation,
                realtime_payload.clone(),
                true,
            )?;
            if applied {
                conversation.aggregate.observe_commit_seq(ordering_seq);
            }
            *live_conversation = candidate;
            self.metrics
                .record_message_evictions(evicted_message_ids.len());
            for message_id in &evicted_message_ids {
                state
                    .message_locator
                    .remove(command.tenant_id.as_str(), message_id.as_str());
            }
            state.touch_conversation(scope_key.as_str());
            if !applied {
                return Err(RuntimeError::MessageAlreadyRecalled(
                    recalled.message_id.clone(),
                ));
            }
            (recalled, event_id, realtime_payload)
        };

        let (recalled, event_id, realtime_payload) = recalled;

        self.publish_message_mutation_realtime_after_commit(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            recalled.conversation_id.as_str(),
            "message.recalled",
            realtime_payload,
        )?;
        if let Some(idempotency_key) = command
            .idempotency_key
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            let mut state = write_runtime_state(
                &self.state,
                "conversation-runtime.state.recall_message.idempotency",
            );
            let scope_key = conversation_scope_key(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                recalled.conversation_id.as_str(),
            );
            if let Some(conversation) = state.conversations.get_mut(scope_key.as_str()) {
                conversation.message_mutation_requests.insert(
                    message_mutation_request_key(&command.recalled_by, idempotency_key),
                    MessageMutationReplayRecord {
                        result: MessageMutationResult {
                            conversation_id: recalled.conversation_id.clone(),
                            message_id: recalled.message_id.clone(),
                            message_seq: recalled.message_seq,
                            event_id: event_id.clone(),
                        },
                    },
                );
            }
        }
        self.maybe_evict_after_write();
        Ok(MessageMutationResult {
            conversation_id: recalled.conversation_id,
            message_id: recalled.message_id,
            message_seq: recalled.message_seq,
            event_id,
        })
    }

    pub fn add_message_reaction(
        &self,
        command: AddMessageReactionCommand,
    ) -> Result<MessageReactionMutationResult, RuntimeError> {
        validate_payload_size(
            "messageId",
            command.message_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "reactionKey",
            command.reaction_key.as_str(),
            MESSAGE_REACTION_KEY_MAX_BYTES,
        )?;
        validate_sender_payload_size("reactedBy", &command.reacted_by)?;
        let conversation_id = self.ensure_message_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.message_id.as_str(),
        )?;
        self.ensure_member_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            conversation_id.as_str(),
            command.reacted_by.kind.as_str(),
            command.reacted_by.id.as_str(),
        )?;
        let (reaction, changed, event_id, realtime_payload) = {
            let mut state = write_runtime_state(
                &self.state,
                "conversation-runtime.state.add_message_reaction",
            );
            let scope_key = conversation_scope_key(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                conversation_id.as_str(),
            );
            let live_conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let mut candidate = live_conversation.clone();
            let conversation = &mut candidate;
            let reacted_member = resolve_active_member_with_kind(
                conversation,
                command.reacted_by.id.as_str(),
                command.reacted_by.kind.as_str(),
            )?;
            policy::ensure_actor_kind_matches_member(
                &reacted_member,
                command.reacted_by.kind.as_str(),
            )?;
            policy::ensure_message_reaction_allowed(conversation, &reacted_member)?;
            let stored = conversation
                .message_log
                .message(command.message_id.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            if stored.recalled {
                return Err(RuntimeError::MessageAlreadyRecalled(command.message_id));
            }
            let conversation_id = stored.message.conversation_id.clone();
            let message_id = stored.message.message_id.clone();
            let message_seq = stored.message.message_seq;

            let mut reacted_by = command.reacted_by.clone();
            if reacted_by.member_id.is_none() {
                reacted_by.member_id = Some(reacted_member.member_id.clone());
            }

            let reaction = MessageReactionAdded {
                tenant_id: command.tenant_id.clone(),
                conversation_id,
                message_id,
                message_seq,
                reaction_key: command.reaction_key,
                reacted_by,
                reacted_at: conversation_timestamp(),
            };
            let changed = !stored
                .reactions
                .get(reaction.reaction_key.as_str())
                .is_some_and(|actors| {
                    actors.contains(&ReactionActorIdentity::from_sender(&reaction.reacted_by))
                });
            let retention_class = conversation_retention_class(conversation);
            let ordering_seq = conversation
                .aggregate
                .commit_seq()
                .checked_add(1)
                .ok_or_else(|| {
                    RuntimeError::Conflict("conversation journal ordering sequence overflow".into())
                })?;
            let candidate_event_id = format!(
                "evt_{}_reaction_added_{}_{}_{}_{}",
                reaction.message_id,
                event_id_component(reaction.reaction_key.as_str()),
                event_id_component(reaction.reacted_by.kind.as_str()),
                event_id_component(reaction.reacted_by.id.as_str()),
                event_id_component(reaction.reacted_at.as_str())
            );
            let envelope = build_message_reaction_added_envelope(
                &reaction,
                command.organization_id.as_str(),
                candidate_event_id.as_str(),
                ordering_seq,
                retention_class.as_str(),
            );
            let mutation = StoredMessageMutation::ReactionAdded {
                target: StoredMessageMutationTarget {
                    tenant_id: command.tenant_id.clone(),
                    organization_id: command.organization_id.clone(),
                    conversation_id: reaction.conversation_id.clone(),
                    message_id: reaction.message_id.clone(),
                    message_seq: reaction.message_seq,
                },
                reaction: StoredMessageReactionRecord {
                    actor_principal_kind: reaction.reacted_by.kind.clone(),
                    actor_principal_id: reaction.reacted_by.id.clone(),
                    reaction_key: reaction.reaction_key.clone(),
                    reacted_at: reaction.reacted_at.clone(),
                },
            };
            let realtime_payload = runtime_json_string(&reaction)?;
            let evicted_message_ids = conversation
                .message_log
                .apply_reaction_added(&reaction)
                .ok_or_else(|| RuntimeError::MessageNotFound(reaction.message_id.clone()))?
                .evicted_message_ids;
            let applied = self.persist_message_mutation_commit(
                envelope,
                mutation,
                realtime_payload.clone(),
                changed,
            )?;
            if applied {
                conversation.aggregate.observe_commit_seq(ordering_seq);
            }
            *live_conversation = candidate;
            self.metrics
                .record_message_evictions(evicted_message_ids.len());
            for message_id in &evicted_message_ids {
                state
                    .message_locator
                    .remove(command.tenant_id.as_str(), message_id.as_str());
            }
            state.touch_conversation(scope_key.as_str());
            (
                reaction,
                applied,
                applied.then_some(candidate_event_id),
                realtime_payload,
            )
        };

        if changed {
            self.publish_message_mutation_realtime_after_commit(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                reaction.conversation_id.as_str(),
                "message.reaction_added",
                realtime_payload,
            )?;
        }

        self.maybe_evict_after_write();
        Ok(MessageReactionMutationResult {
            conversation_id: reaction.conversation_id,
            message_id: reaction.message_id,
            message_seq: reaction.message_seq,
            reaction_key: reaction.reaction_key,
            event_id,
            changed,
        })
    }

    pub fn remove_message_reaction(
        &self,
        command: RemoveMessageReactionCommand,
    ) -> Result<MessageReactionMutationResult, RuntimeError> {
        validate_payload_size(
            "messageId",
            command.message_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "reactionKey",
            command.reaction_key.as_str(),
            MESSAGE_REACTION_KEY_MAX_BYTES,
        )?;
        validate_sender_payload_size("removedBy", &command.removed_by)?;
        let conversation_id = self.ensure_message_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.message_id.as_str(),
        )?;
        self.ensure_member_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            conversation_id.as_str(),
            command.removed_by.kind.as_str(),
            command.removed_by.id.as_str(),
        )?;
        let (reaction, changed, event_id, realtime_payload) = {
            let mut state = write_runtime_state(
                &self.state,
                "conversation-runtime.state.remove_message_reaction",
            );
            let scope_key = conversation_scope_key(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                conversation_id.as_str(),
            );
            let live_conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let mut candidate = live_conversation.clone();
            let conversation = &mut candidate;
            let removed_member = resolve_active_member_with_kind(
                conversation,
                command.removed_by.id.as_str(),
                command.removed_by.kind.as_str(),
            )?;
            policy::ensure_actor_kind_matches_member(
                &removed_member,
                command.removed_by.kind.as_str(),
            )?;
            policy::ensure_message_reaction_allowed(conversation, &removed_member)?;
            let stored = conversation
                .message_log
                .message(command.message_id.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            if stored.recalled {
                return Err(RuntimeError::MessageAlreadyRecalled(command.message_id));
            }
            let conversation_id = stored.message.conversation_id.clone();
            let message_id = stored.message.message_id.clone();
            let message_seq = stored.message.message_seq;

            let mut removed_by = command.removed_by.clone();
            if removed_by.member_id.is_none() {
                removed_by.member_id = Some(removed_member.member_id.clone());
            }

            let reaction = MessageReactionRemoved {
                tenant_id: command.tenant_id.clone(),
                conversation_id,
                message_id,
                message_seq,
                reaction_key: command.reaction_key,
                removed_by,
                removed_at: conversation_timestamp(),
            };
            let changed = stored
                .reactions
                .get(reaction.reaction_key.as_str())
                .is_some_and(|actors| {
                    actors.contains(&ReactionActorIdentity::from_sender(&reaction.removed_by))
                });
            let retention_class = conversation_retention_class(conversation);
            let ordering_seq = conversation
                .aggregate
                .commit_seq()
                .checked_add(1)
                .ok_or_else(|| {
                    RuntimeError::Conflict("conversation journal ordering sequence overflow".into())
                })?;
            let candidate_event_id = format!(
                "evt_{}_reaction_removed_{}_{}_{}_{}",
                reaction.message_id,
                event_id_component(reaction.reaction_key.as_str()),
                event_id_component(reaction.removed_by.kind.as_str()),
                event_id_component(reaction.removed_by.id.as_str()),
                event_id_component(reaction.removed_at.as_str())
            );
            let envelope = build_message_reaction_removed_envelope(
                &reaction,
                command.organization_id.as_str(),
                candidate_event_id.as_str(),
                ordering_seq,
                retention_class.as_str(),
            );
            let mutation = StoredMessageMutation::ReactionRemoved {
                target: StoredMessageMutationTarget {
                    tenant_id: command.tenant_id.clone(),
                    organization_id: command.organization_id.clone(),
                    conversation_id: reaction.conversation_id.clone(),
                    message_id: reaction.message_id.clone(),
                    message_seq: reaction.message_seq,
                },
                reaction: StoredMessageReactionRecord {
                    actor_principal_kind: reaction.removed_by.kind.clone(),
                    actor_principal_id: reaction.removed_by.id.clone(),
                    reaction_key: reaction.reaction_key.clone(),
                    reacted_at: reaction.removed_at.clone(),
                },
            };
            let realtime_payload = runtime_json_string(&reaction)?;
            let evicted_message_ids = conversation
                .message_log
                .apply_reaction_removed(&reaction)
                .ok_or_else(|| RuntimeError::MessageNotFound(reaction.message_id.clone()))?
                .evicted_message_ids;
            let applied = self.persist_message_mutation_commit(
                envelope,
                mutation,
                realtime_payload.clone(),
                changed,
            )?;
            if applied {
                conversation.aggregate.observe_commit_seq(ordering_seq);
            }
            *live_conversation = candidate;
            self.metrics
                .record_message_evictions(evicted_message_ids.len());
            for message_id in &evicted_message_ids {
                state
                    .message_locator
                    .remove(command.tenant_id.as_str(), message_id.as_str());
            }
            state.touch_conversation(scope_key.as_str());
            (
                reaction,
                applied,
                applied.then_some(candidate_event_id),
                realtime_payload,
            )
        };

        if changed {
            self.publish_message_mutation_realtime_after_commit(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                reaction.conversation_id.as_str(),
                "message.reaction_removed",
                realtime_payload,
            )?;
        }

        self.maybe_evict_after_write();
        Ok(MessageReactionMutationResult {
            conversation_id: reaction.conversation_id,
            message_id: reaction.message_id,
            message_seq: reaction.message_seq,
            reaction_key: reaction.reaction_key,
            event_id,
            changed,
        })
    }

    pub fn pin_message(
        &self,
        command: PinMessageCommand,
    ) -> Result<MessagePinMutationResult, RuntimeError> {
        validate_payload_size(
            "messageId",
            command.message_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_sender_payload_size("pinnedBy", &command.pinned_by)?;
        let conversation_id = self.ensure_message_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.message_id.as_str(),
        )?;
        self.ensure_member_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            conversation_id.as_str(),
            command.pinned_by.kind.as_str(),
            command.pinned_by.id.as_str(),
        )?;
        let (pin, changed, event_id, realtime_payload) = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.pin_message");
            let scope_key = conversation_scope_key(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                conversation_id.as_str(),
            );
            let live_conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let mut candidate = live_conversation.clone();
            let conversation = &mut candidate;
            let pinned_member = resolve_active_member_with_kind(
                conversation,
                command.pinned_by.id.as_str(),
                command.pinned_by.kind.as_str(),
            )?;
            policy::ensure_actor_kind_matches_member(
                &pinned_member,
                command.pinned_by.kind.as_str(),
            )?;
            policy::ensure_message_pin_allowed(conversation, &pinned_member)?;
            let stored = conversation
                .message_log
                .message(command.message_id.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            if stored.recalled {
                return Err(RuntimeError::MessageAlreadyRecalled(command.message_id));
            }
            let conversation_id = stored.message.conversation_id.clone();
            let message_id = stored.message.message_id.clone();
            let message_seq = stored.message.message_seq;

            let mut pinned_by = command.pinned_by.clone();
            if pinned_by.member_id.is_none() {
                pinned_by.member_id = Some(pinned_member.member_id.clone());
            }

            let pin = MessagePinned {
                tenant_id: command.tenant_id.clone(),
                conversation_id,
                message_id,
                message_seq,
                pinned_by,
                pinned_at: conversation_timestamp(),
            };
            let changed = stored.pin.is_none();
            let retention_class = conversation_retention_class(conversation);
            let ordering_seq = conversation
                .aggregate
                .commit_seq()
                .checked_add(1)
                .ok_or_else(|| {
                    RuntimeError::Conflict("conversation journal ordering sequence overflow".into())
                })?;
            let candidate_event_id = format!(
                "evt_{}_pin_added_{}_{}_{}",
                pin.message_id,
                event_id_component(pin.pinned_by.kind.as_str()),
                event_id_component(pin.pinned_by.id.as_str()),
                event_id_component(pin.pinned_at.as_str())
            );
            let envelope = build_message_pinned_envelope(
                &pin,
                command.organization_id.as_str(),
                candidate_event_id.as_str(),
                ordering_seq,
                retention_class.as_str(),
            );
            let mutation = StoredMessageMutation::Pinned {
                target: StoredMessageMutationTarget {
                    tenant_id: command.tenant_id.clone(),
                    organization_id: command.organization_id.clone(),
                    conversation_id: pin.conversation_id.clone(),
                    message_id: pin.message_id.clone(),
                    message_seq: pin.message_seq,
                },
                pin: StoredMessagePinRecord {
                    pinned_by_principal_kind: pin.pinned_by.kind.clone(),
                    pinned_by_principal_id: pin.pinned_by.id.clone(),
                    pinned_at: pin.pinned_at.clone(),
                },
            };
            let realtime_payload = runtime_json_string(&pin)?;
            let evicted_message_ids = conversation
                .message_log
                .apply_pinned(&pin)
                .ok_or_else(|| RuntimeError::MessageNotFound(pin.message_id.clone()))?
                .evicted_message_ids;
            let applied = self.persist_message_mutation_commit(
                envelope,
                mutation,
                realtime_payload.clone(),
                changed,
            )?;
            if applied {
                conversation.aggregate.observe_commit_seq(ordering_seq);
            }
            *live_conversation = candidate;
            self.metrics
                .record_message_evictions(evicted_message_ids.len());
            for message_id in &evicted_message_ids {
                state
                    .message_locator
                    .remove(command.tenant_id.as_str(), message_id.as_str());
            }
            state.touch_conversation(scope_key.as_str());
            (
                pin,
                applied,
                applied.then_some(candidate_event_id),
                realtime_payload,
            )
        };

        if changed {
            self.publish_message_mutation_realtime_after_commit(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                pin.conversation_id.as_str(),
                "message.pin_added",
                realtime_payload,
            )?;
        }

        self.maybe_evict_after_write();
        Ok(MessagePinMutationResult {
            conversation_id: pin.conversation_id,
            message_id: pin.message_id,
            message_seq: pin.message_seq,
            event_id,
            changed,
        })
    }

    pub fn unpin_message(
        &self,
        command: UnpinMessageCommand,
    ) -> Result<MessagePinMutationResult, RuntimeError> {
        validate_payload_size(
            "messageId",
            command.message_id.as_str(),
            CONVERSATION_MAX_ID_BYTES,
        )?;
        validate_sender_payload_size("unpinnedBy", &command.unpinned_by)?;
        let conversation_id = self.ensure_message_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.message_id.as_str(),
        )?;
        self.ensure_member_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            conversation_id.as_str(),
            command.unpinned_by.kind.as_str(),
            command.unpinned_by.id.as_str(),
        )?;
        let (pin, changed, event_id, realtime_payload) = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.unpin_message");
            let scope_key = conversation_scope_key(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                conversation_id.as_str(),
            );
            let live_conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            let mut candidate = live_conversation.clone();
            let conversation = &mut candidate;
            let unpinned_member = resolve_active_member_with_kind(
                conversation,
                command.unpinned_by.id.as_str(),
                command.unpinned_by.kind.as_str(),
            )?;
            policy::ensure_actor_kind_matches_member(
                &unpinned_member,
                command.unpinned_by.kind.as_str(),
            )?;
            policy::ensure_message_pin_allowed(conversation, &unpinned_member)?;
            let stored = conversation
                .message_log
                .message(command.message_id.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
            if stored.recalled {
                return Err(RuntimeError::MessageAlreadyRecalled(command.message_id));
            }
            let conversation_id = stored.message.conversation_id.clone();
            let message_id = stored.message.message_id.clone();
            let message_seq = stored.message.message_seq;

            let mut unpinned_by = command.unpinned_by.clone();
            if unpinned_by.member_id.is_none() {
                unpinned_by.member_id = Some(unpinned_member.member_id.clone());
            }

            let pin = MessageUnpinned {
                tenant_id: command.tenant_id.clone(),
                conversation_id,
                message_id,
                message_seq,
                unpinned_by,
                unpinned_at: conversation_timestamp(),
            };
            let changed = stored.pin.is_some();
            let retention_class = conversation_retention_class(conversation);
            let ordering_seq = conversation
                .aggregate
                .commit_seq()
                .checked_add(1)
                .ok_or_else(|| {
                    RuntimeError::Conflict("conversation journal ordering sequence overflow".into())
                })?;
            let candidate_event_id = format!(
                "evt_{}_pin_removed_{}_{}_{}",
                pin.message_id,
                event_id_component(pin.unpinned_by.kind.as_str()),
                event_id_component(pin.unpinned_by.id.as_str()),
                event_id_component(pin.unpinned_at.as_str())
            );
            let envelope = build_message_unpinned_envelope(
                &pin,
                command.organization_id.as_str(),
                candidate_event_id.as_str(),
                ordering_seq,
                retention_class.as_str(),
            );
            let mutation = StoredMessageMutation::Unpinned {
                target: StoredMessageMutationTarget {
                    tenant_id: command.tenant_id.clone(),
                    organization_id: command.organization_id.clone(),
                    conversation_id: pin.conversation_id.clone(),
                    message_id: pin.message_id.clone(),
                    message_seq: pin.message_seq,
                },
            };
            let realtime_payload = runtime_json_string(&pin)?;
            let evicted_message_ids = conversation
                .message_log
                .apply_unpinned(&pin)
                .ok_or_else(|| RuntimeError::MessageNotFound(pin.message_id.clone()))?
                .evicted_message_ids;
            let applied = self.persist_message_mutation_commit(
                envelope,
                mutation,
                realtime_payload.clone(),
                changed,
            )?;
            if applied {
                conversation.aggregate.observe_commit_seq(ordering_seq);
            }
            *live_conversation = candidate;
            self.metrics
                .record_message_evictions(evicted_message_ids.len());
            for message_id in &evicted_message_ids {
                state
                    .message_locator
                    .remove(command.tenant_id.as_str(), message_id.as_str());
            }
            state.touch_conversation(scope_key.as_str());
            (
                pin,
                applied,
                applied.then_some(candidate_event_id),
                realtime_payload,
            )
        };

        if changed {
            self.publish_message_mutation_realtime_after_commit(
                command.tenant_id.as_str(),
                command.organization_id.as_str(),
                pin.conversation_id.as_str(),
                "message.pin_removed",
                realtime_payload,
            )?;
        }

        self.maybe_evict_after_write();
        Ok(MessagePinMutationResult {
            conversation_id: pin.conversation_id,
            message_id: pin.message_id,
            message_seq: pin.message_seq,
            event_id,
            changed,
        })
    }
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn require_active_member_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<ConversationMember, RuntimeError> {
        let organization_id = organization_id_from_auth_context(auth);
        self.require_active_member_with_kind(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            conversation_id,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        )
    }

    pub fn ensure_conversation_bound_write_allowed_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
        capability: &str,
    ) -> Result<(), RuntimeError> {
        let organization_id = organization_id_from_auth_context(auth);
        self.ensure_conversation_bound_write_allowed_with_actor_kind(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            conversation_id,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            capability,
        )
    }

    pub fn ensure_conversation_bound_write_allowed(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
        capability: &str,
    ) -> Result<(), RuntimeError> {
        let actor_kind = self
            .require_active_member(tenant_id, organization_id, conversation_id, principal_id)?
            .principal_kind;
        self.ensure_conversation_bound_write_allowed_with_actor_kind(
            tenant_id,
            organization_id,
            conversation_id,
            principal_id,
            actor_kind.as_str(),
            capability,
        )
    }

    pub fn ensure_conversation_bound_write_allowed_with_actor_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
        actor_kind: &str,
        capability: &str,
    ) -> Result<(), RuntimeError> {
        self.ensure_member_loaded(
            tenant_id,
            organization_id,
            conversation_id,
            actor_kind,
            principal_id,
        )?;
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        let state = read_runtime_state(
            &self.state,
            "conversation-runtime.state.ensure_conversation_bound_write_allowed",
        );
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        let actor_member = resolve_active_member_with_kind(conversation, principal_id, actor_kind)?;
        policy::ensure_actor_kind_matches_member(&actor_member, actor_kind)?;
        policy::ensure_conversation_bound_write_allowed(conversation, &actor_member, capability)
    }

    pub fn conversation_id_for_message_from_auth_context(
        &self,
        auth: &AppContext,
        message_id: &str,
    ) -> Result<String, RuntimeError> {
        self.conversation_id_for_message(auth.tenant_id.as_str(), message_id)
    }

    pub fn conversation_id_for_message(
        &self,
        tenant_id: &str,
        message_id: &str,
    ) -> Result<String, RuntimeError> {
        let state = read_runtime_state(
            &self.state,
            "conversation-runtime.state.conversation_id_for_message",
        );
        state
            .message_locator
            .conversation_id(tenant_id, message_id)
            .map(str::to_owned)
            .ok_or_else(|| RuntimeError::MessageNotFound(message_id.into()))
    }

    pub fn require_active_member_with_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Result<ConversationMember, RuntimeError> {
        self.ensure_member_loaded(
            tenant_id,
            organization_id,
            conversation_id,
            principal_kind,
            principal_id,
        )?;
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        let state = read_runtime_state(
            &self.state,
            "conversation-runtime.state.require_active_member_with_kind",
        );
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        resolve_active_member_with_kind(conversation, principal_id, principal_kind)
    }

    pub fn require_active_member(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_id: &str,
    ) -> Result<ConversationMember, RuntimeError> {
        self.ensure_conversation_loaded(tenant_id, organization_id, conversation_id)?;
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        let state = read_runtime_state(
            &self.state,
            "conversation-runtime.state.require_active_member",
        );
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        let member_id = resolve_active_member_id(conversation, principal_id)?;
        conversation
            .roster
            .member(member_id.as_str())
            .cloned()
            .ok_or_else(|| {
                RuntimeError::PermissionDenied(format!(
                    "principal is not active conversation member: {principal_id}"
                ))
            })
    }
}

// ---------------------------------------------------------------------------
// Aggregate store conversion helpers
// ---------------------------------------------------------------------------

fn persist_aggregate_records(
    store: &dyn ConversationAggregateStore,
    members: Vec<ConversationMemberRecord>,
    read_cursors: Vec<ReadCursorRecord>,
) -> Result<(), RuntimeError> {
    for member in members {
        store.upsert_member(member).map_err(RuntimeError::from)?;
    }
    for cursor in read_cursors {
        store
            .upsert_read_cursor(cursor)
            .map_err(RuntimeError::from)?;
    }
    Ok(())
}

fn normalized_record_scope_matches(
    tenant_id: &str,
    organization_id: &str,
    conversation_id: &str,
    record_tenant_id: &str,
    record_organization_id: &str,
    record_conversation_id: &str,
) -> bool {
    record_tenant_id == tenant_id
        && record_organization_id == organization_id
        && record_conversation_id == conversation_id
}

fn normalized_handoff_actor(
    kind: &Option<String>,
    id: &Option<String>,
    field: &str,
) -> Result<Option<ChangeAgentHandoffStatusView>, RuntimeError> {
    match (kind.as_deref(), id.as_deref()) {
        (None, None) => Ok(None),
        (Some(kind), Some(id)) if !kind.trim().is_empty() && !id.trim().is_empty() => {
            Ok(Some(ChangeAgentHandoffStatusView {
                id: id.to_owned(),
                kind: kind.to_owned(),
            }))
        }
        _ => Err(RuntimeError::Conflict(format!(
            "normalized conversation handoff {field} identity is incomplete"
        ))),
    }
}

fn hydrate_normalized_conversation_aggregate(
    aggregate: &mut ConversationAggregateState,
    current_state: &NormalizedConversationCurrentState,
) -> Result<(), RuntimeError> {
    let conversation = &current_state.conversation;
    aggregate
        .synchronize_normalized_current_state(
            conversation.conversation_type.as_str(),
            conversation.lifecycle_state.as_str(),
            conversation.commit_seq,
            conversation.member_epoch,
        )
        .map_err(RuntimeError::Conflict)?;

    let (policy_epoch, policy) = match current_state.policy.as_ref() {
        Some(record) => {
            if !normalized_record_scope_matches(
                conversation.tenant_id.as_str(),
                conversation.organization_id.as_str(),
                conversation.conversation_id.as_str(),
                record.tenant_id.as_str(),
                record.organization_id.as_str(),
                record.conversation_id.as_str(),
            ) {
                return Err(RuntimeError::Conflict(
                    "normalized conversation policy scope is inconsistent".into(),
                ));
            }
            let policy = ConversationPolicy {
                policy_version: record.policy_version.clone(),
                capability_flags: record.capability_flags.clone(),
                history_visibility: record.history_visibility.clone(),
                retention_policy_ref: record.retention_policy_ref.clone(),
                max_members: record.max_members,
            }
            .normalize()
            .map_err(RuntimeError::Conflict)?;
            (record.policy_epoch, Some(policy))
        }
        None => (0, None),
    };

    let business_binding = current_state
        .business_binding
        .as_ref()
        .map(|record| {
            if !normalized_record_scope_matches(
                conversation.tenant_id.as_str(),
                conversation.organization_id.as_str(),
                conversation.conversation_id.as_str(),
                record.tenant_id.as_str(),
                record.organization_id.as_str(),
                record.conversation_id.as_str(),
            ) {
                return Err(RuntimeError::Conflict(
                    "normalized conversation business binding scope is inconsistent".into(),
                ));
            }
            Ok(ConversationBusinessBinding {
                business_type: record.business_type.clone(),
                business_id: record.business_id.clone(),
            })
        })
        .transpose()?;

    let (handoff_status_epoch, handoff_state) = match current_state.handoff.as_ref() {
        Some(record) => {
            if !normalized_record_scope_matches(
                conversation.tenant_id.as_str(),
                conversation.organization_id.as_str(),
                conversation.conversation_id.as_str(),
                record.tenant_id.as_str(),
                record.organization_id.as_str(),
                record.conversation_id.as_str(),
            ) || !matches!(
                record.status.as_str(),
                "open" | "accepted" | "resolved" | "closed"
            ) || record.source_principal_kind.trim().is_empty()
                || record.source_principal_id.trim().is_empty()
                || record.target_principal_kind.trim().is_empty()
                || record.target_principal_id.trim().is_empty()
                || record.handoff_session_id.trim().is_empty()
            {
                return Err(RuntimeError::Conflict(
                    "normalized conversation handoff state is invalid".into(),
                ));
            }
            let handoff = AgentHandoffStateView {
                tenant_id: record.tenant_id.clone(),
                conversation_id: record.conversation_id.clone(),
                status: record.status.clone(),
                source: ChangeAgentHandoffStatusView {
                    id: record.source_principal_id.clone(),
                    kind: record.source_principal_kind.clone(),
                },
                target: ChangeAgentHandoffStatusView {
                    id: record.target_principal_id.clone(),
                    kind: record.target_principal_kind.clone(),
                },
                handoff_session_id: record.handoff_session_id.clone(),
                handoff_reason: record.handoff_reason.clone(),
                accepted_at: record.accepted_at.clone(),
                accepted_by: normalized_handoff_actor(
                    &record.accepted_by_principal_kind,
                    &record.accepted_by_principal_id,
                    "acceptedBy",
                )?,
                resolved_at: record.resolved_at.clone(),
                resolved_by: normalized_handoff_actor(
                    &record.resolved_by_principal_kind,
                    &record.resolved_by_principal_id,
                    "resolvedBy",
                )?,
                closed_at: record.closed_at.clone(),
                closed_by: normalized_handoff_actor(
                    &record.closed_by_principal_kind,
                    &record.closed_by_principal_id,
                    "closedBy",
                )?,
            };
            (record.handoff_status_epoch, Some(handoff))
        }
        None => (0, None),
    };

    aggregate
        .restore_normalized_capability_state(
            conversation.archived_at.clone(),
            conversation.archive_event_id.clone(),
            policy_epoch,
            policy,
            business_binding,
            handoff_status_epoch,
            handoff_state,
        )
        .map_err(RuntimeError::Conflict)
}

fn normalized_policy_record(
    tenant_id: &str,
    organization_id: &str,
    conversation_id: &str,
    aggregate: &ConversationAggregateState,
) -> Option<NormalizedConversationPolicyRecord> {
    aggregate
        .policy()
        .map(|policy| NormalizedConversationPolicyRecord {
            tenant_id: tenant_id.to_owned(),
            organization_id: organization_id.to_owned(),
            conversation_id: conversation_id.to_owned(),
            policy_epoch: aggregate.policy_epoch(),
            policy_version: policy.policy_version.clone(),
            capability_flags: policy.capability_flags.clone(),
            history_visibility: policy.history_visibility.clone(),
            retention_policy_ref: policy.retention_policy_ref.clone(),
            max_members: policy.max_members,
        })
}

fn normalized_business_binding_record(
    tenant_id: &str,
    organization_id: &str,
    conversation_id: &str,
    aggregate: &ConversationAggregateState,
) -> Option<NormalizedConversationBusinessBindingRecord> {
    aggregate
        .business_binding()
        .map(|binding| NormalizedConversationBusinessBindingRecord {
            tenant_id: tenant_id.to_owned(),
            organization_id: organization_id.to_owned(),
            conversation_id: conversation_id.to_owned(),
            business_type: binding.business_type.clone(),
            business_id: binding.business_id.clone(),
        })
}

fn normalized_handoff_record(
    tenant_id: &str,
    organization_id: &str,
    conversation_id: &str,
    aggregate: &ConversationAggregateState,
) -> Option<NormalizedConversationHandoffRecord> {
    aggregate
        .handoff_state()
        .map(|handoff| NormalizedConversationHandoffRecord {
            tenant_id: tenant_id.to_owned(),
            organization_id: organization_id.to_owned(),
            conversation_id: conversation_id.to_owned(),
            handoff_status_epoch: aggregate.handoff_status_epoch(),
            status: handoff.status.clone(),
            source_principal_kind: handoff.source.kind.clone(),
            source_principal_id: handoff.source.id.clone(),
            target_principal_kind: handoff.target.kind.clone(),
            target_principal_id: handoff.target.id.clone(),
            handoff_session_id: handoff.handoff_session_id.clone(),
            handoff_reason: handoff.handoff_reason.clone(),
            accepted_at: handoff.accepted_at.clone(),
            accepted_by_principal_kind: handoff
                .accepted_by
                .as_ref()
                .map(|actor| actor.kind.clone()),
            accepted_by_principal_id: handoff.accepted_by.as_ref().map(|actor| actor.id.clone()),
            resolved_at: handoff.resolved_at.clone(),
            resolved_by_principal_kind: handoff
                .resolved_by
                .as_ref()
                .map(|actor| actor.kind.clone()),
            resolved_by_principal_id: handoff.resolved_by.as_ref().map(|actor| actor.id.clone()),
            closed_at: handoff.closed_at.clone(),
            closed_by_principal_kind: handoff.closed_by.as_ref().map(|actor| actor.kind.clone()),
            closed_by_principal_id: handoff.closed_by.as_ref().map(|actor| actor.id.clone()),
        })
}

fn conversation_member_from_record(record: &ConversationMemberRecord) -> ConversationMember {
    use im_domain_core::conversation::{MembershipRole, MembershipState};
    let role = match record.membership_role.as_str() {
        "owner" => MembershipRole::Owner,
        "admin" => MembershipRole::Admin,
        "member" => MembershipRole::Member,
        "guest" => MembershipRole::Guest,
        _ => MembershipRole::Member,
    };
    let state = match record.membership_state.as_str() {
        "joined" => MembershipState::Joined,
        "linked" => MembershipState::Linked,
        "invited" => MembershipState::Invited,
        "removed" => MembershipState::Removed,
        "left" => MembershipState::Left,
        _ => MembershipState::Joined,
    };
    let attributes: BTreeMap<String, String> =
        serde_json::from_str(&record.attributes_json).unwrap_or_default();
    ConversationMember {
        tenant_id: record.tenant_id.clone(),
        conversation_id: record.conversation_id.clone(),
        member_id: record.member_id.to_string(),
        principal_id: record.principal_id.clone(),
        principal_kind: record.principal_kind.clone(),
        role,
        state,
        invited_by: record.invited_by.clone(),
        joined_at: record.joined_at.clone(),
        removed_at: record.removed_at.clone(),
        attributes,
    }
}

fn read_cursor_from_record(record: &ReadCursorRecord) -> ConversationReadCursor {
    ConversationReadCursor {
        tenant_id: record.tenant_id.clone(),
        conversation_id: record.conversation_id.clone(),
        member_id: record.member_id.to_string(),
        principal_id: record.principal_id.clone(),
        principal_kind: record.principal_kind.clone(),
        device_id: (!record.device_id.is_empty()).then(|| record.device_id.clone()),
        read_seq: record.read_seq,
        last_read_message_id: record.last_read_message_id.map(|id| id.to_string()),
        updated_at: record.updated_at.clone(),
    }
}

fn member_to_record(
    tenant_id: &str,
    organization_id: &str,
    conversation_id: &str,
    member: &ConversationMember,
) -> ConversationMemberRecord {
    let membership_role = match member.role {
        MembershipRole::Owner => "owner",
        MembershipRole::Admin => "admin",
        MembershipRole::Member => "member",
        MembershipRole::Guest => "guest",
    };
    let membership_state = match member.state {
        MembershipState::Joined => "joined",
        MembershipState::Linked => "linked",
        MembershipState::Invited => "invited",
        MembershipState::Removed => "removed",
        MembershipState::Left => "left",
    };
    ConversationMemberRecord {
        tenant_id: tenant_id.to_owned(),
        organization_id: organization_id.to_owned(),
        conversation_id: conversation_id.to_owned(),
        principal_kind: member.principal_kind.clone(),
        principal_id: member.principal_id.clone(),
        member_id: normalized_member_storage_id(member.member_id.as_str()),
        membership_role: membership_role.into(),
        membership_state: membership_state.into(),
        invited_by: member.invited_by.clone(),
        joined_at: member.joined_at.clone(),
        removed_at: member.removed_at.clone(),
        attributes_json: serde_json::to_string(&member.attributes).unwrap_or_default(),
    }
}

fn cursor_to_record(
    tenant_id: &str,
    organization_id: &str,
    conversation_id: &str,
    cursor: &ConversationReadCursor,
) -> ReadCursorRecord {
    ReadCursorRecord {
        tenant_id: tenant_id.to_owned(),
        organization_id: organization_id.to_owned(),
        conversation_id: conversation_id.to_owned(),
        member_id: normalized_member_storage_id(cursor.member_id.as_str()),
        device_id: cursor.device_id.clone().unwrap_or_default(),
        principal_kind: cursor.principal_kind.clone(),
        principal_id: cursor.principal_id.clone(),
        read_seq: cursor.read_seq,
        last_read_message_id: cursor
            .last_read_message_id
            .clone()
            .map(|id| id.parse::<i64>().unwrap_or(0)),
        updated_at: cursor.updated_at.clone(),
    }
}

fn normalized_member_storage_id(member_id: &str) -> i64 {
    let digest = sha256_hash(member_id.as_bytes());
    let value = u64::from_str_radix(&digest[..16], 16)
        .expect("sha256_hash must return at least sixteen hexadecimal characters")
        & i64::MAX as u64;
    if value == 0 { 1 } else { value as i64 }
}

fn build_normalized_agent_assignment_change(
    envelope: &CommitEnvelope,
    assignments: &ConversationAgentAssignmentSet,
) -> Result<ReplaceConversationAgentAssignments, RuntimeError> {
    let tenant_id =
        parse_normalized_assignment_scope_id(envelope.tenant_id.as_str(), "tenantId", false)?;
    let organization_id = parse_normalized_assignment_scope_id(
        envelope.normalized_organization_id().as_str(),
        "organizationId",
        true,
    )?;
    let assigned_by = if envelope.actor.actor_kind == "system" {
        0
    } else {
        parse_normalized_assignment_scope_id(envelope.actor.actor_id.as_str(), "assignedBy", false)?
    };
    let assignment_source = match assignments.source {
        ConversationAgentAssignmentSource::DefaultPolicy => AgentAssignmentSource::DefaultPolicy,
        ConversationAgentAssignmentSource::ConversationOverride => {
            AgentAssignmentSource::ConversationOverride
        }
    };
    let items = assignments
        .agents
        .iter()
        .enumerate()
        .map(|(position, assignment)| ConversationAgentAssignmentItem {
            agent_id: assignment.agent_id.clone(),
            agent_revision_ref: assignment.revision_id.clone(),
            position: position as i32,
        })
        .collect();
    Ok(ReplaceConversationAgentAssignments {
        tenant_id,
        organization_id,
        conversation_id: envelope.aggregate_id.clone(),
        assignment_source,
        assignment_generation: assignments.generation,
        assigned_by,
        assigned_at: envelope.occurred_at.clone(),
        source_event_id: envelope.event_id.clone(),
        source_aggregate_version: envelope.ordering_seq,
        payload_hash: sha256_hash(envelope.payload.as_bytes()),
        items,
    })
}

fn parse_normalized_assignment_scope_id(
    value: &str,
    field: &str,
    allow_zero: bool,
) -> Result<u64, RuntimeError> {
    let parsed = value.parse::<u64>().map_err(|_| {
        RuntimeError::InvalidInput(format!("{field} must be a signed int64 string"))
    })?;
    if parsed > i64::MAX as u64 || (!allow_zero && parsed == 0) {
        return Err(RuntimeError::InvalidInput(format!(
            "{field} is outside the signed int64 range"
        )));
    }
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{self, AssertUnwindSafe};

    fn drive_reference_for_test() -> DriveReference {
        DriveReference {
            drive_uri: "drive://spaces/space-im/nodes/node-image-1".into(),
            space_id: "space-im".into(),
            node_id: "node-image-1".into(),
            node_version: None,
        }
    }

    fn drive_media_resource_for_test(drive: &DriveReference) -> MediaResource {
        MediaResource {
            id: Some(drive.node_id.clone()),
            kind: im_domain_core::media::MediaKind::Image,
            source: MediaSource::Drive,
            url: None,
            public_url: None,
            uri: Some(drive.drive_uri.clone()),
            object_blob_id: None,
            file_name: Some("image.png".into()),
            mime_type: Some("image/png".into()),
            size_bytes: Some("42".into()),
            checksum: None,
            width: None,
            height: None,
            duration_seconds: None,
            alt_text: None,
            title: None,
            poster: None,
            thumbnails: None,
            variants: None,
            access: None,
            ai: None,
            metadata: None,
        }
    }

    fn media_message_body_for_test(resource: MediaResource, drive: DriveReference) -> MessageBody {
        MessageBody {
            summary: Some("image".into()),
            parts: vec![ContentPart::media(im_domain_core::message::MediaPart {
                resource,
                drive,
                media_role: Some("attachment".into()),
            })],
            render_hints: BTreeMap::new(),
            reply_to: None,
        }
    }

    fn mention_message_body_for_test(display_text: impl Into<String>) -> MessageBody {
        MessageBody {
            summary: None,
            parts: vec![ContentPart::Mention(im_domain_core::message::MentionPart {
                target_kind: im_domain_core::message::MentionTargetKind::Agent,
                target_id: "agent.im.default".into(),
                display_text: display_text.into(),
                assignment_generation: 1,
            })],
            render_hints: BTreeMap::new(),
            reply_to: None,
        }
    }

    #[test]
    fn test_message_body_rejects_blank_agent_mention_display_text() {
        let body = mention_message_body_for_test(" \t\r\n ");

        let result = validate_message_body_contract(&body);

        assert!(matches!(
            result,
            Err(RuntimeError::InvalidInput(message))
                if message.contains("parts[0].displayText")
                    && message.contains("must not be empty")
        ));
    }

    #[test]
    fn test_message_body_limits_agent_mention_display_text_by_characters() {
        let maximum_length = mention_message_body_for_test(
            "\u{667a}".repeat(MESSAGE_MENTION_DISPLAY_TEXT_MAX_CHARACTERS),
        );
        validate_message_body_contract(&maximum_length)
            .expect("a 512-character Unicode mention label should remain valid");

        let oversized = mention_message_body_for_test(
            "\u{667a}".repeat(MESSAGE_MENTION_DISPLAY_TEXT_MAX_CHARACTERS + 1),
        );
        let result = validate_message_body_contract(&oversized);

        assert!(matches!(
            result,
            Err(RuntimeError::InvalidInput(message))
                if message.contains("parts[0].displayText")
                    && message.contains("must not exceed 512 characters")
        ));
    }

    #[test]
    fn test_message_body_rejects_local_preview_urls_in_drive_media_resource() {
        let drive = drive_reference_for_test();
        let mut resource = drive_media_resource_for_test(&drive);
        resource.url = Some("blob://local-image".into());
        let body = media_message_body_for_test(resource, drive);

        let result = validate_message_body_contract(&body);

        assert!(
            matches!(
                result,
                Err(RuntimeError::InvalidInput(message))
                    if message.contains("resource.url")
                        && message.contains("local preview URL")
            ),
            "local preview URLs must be rejected before IM message persistence"
        );
    }

    #[test]
    fn test_message_body_rejects_nested_local_preview_urls_in_drive_media_resource() {
        let drive = drive_reference_for_test();
        let mut resource = drive_media_resource_for_test(&drive);
        let mut poster = drive_media_resource_for_test(&drive);
        poster.url = Some("data:image/png;base64,local-preview".into());
        resource.poster = Some(Box::new(poster));
        let body = media_message_body_for_test(resource, drive);

        let result = validate_message_body_contract(&body);

        assert!(
            matches!(
                result,
                Err(RuntimeError::InvalidInput(message))
                    if message.contains("resource.poster.url")
                        && message.contains("local preview URL")
            ),
            "nested local preview URLs must be rejected before IM message persistence"
        );
    }

    fn poison_mutex<T>(mutex: &Mutex<T>) {
        let _ = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = mutex.lock().expect("test poison lock should succeed");
            panic!("intentional poison for regression coverage");
        }));
    }

    fn poison_rwlock_write<T>(lock: &RwLock<T>) {
        let _ = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = lock.write().expect("test poison lock should succeed");
            panic!("intentional poison for regression coverage");
        }));
    }

    #[test]
    fn test_in_memory_journal_recorded_recovers_from_poisoned_lock() {
        let journal = InMemoryJournal::default();
        poison_mutex(&journal.events);

        let result = panic::catch_unwind(AssertUnwindSafe(|| journal.recorded()));
        assert!(
            result.is_ok(),
            "journal.recorded should not panic when journal lock is poisoned"
        );
    }

    #[test]
    fn test_require_active_member_recovers_from_poisoned_runtime_state_lock() {
        let runtime = ConversationRuntime::new(InMemoryJournal::default());
        poison_rwlock_write(&runtime.state);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            runtime.require_active_member("100001", "default", "c_demo", "1")
        }));
        assert!(
            result.is_ok(),
            "require_active_member should not panic when runtime state lock is poisoned"
        );
        let member_result = result.expect("panic status should be captured");
        assert!(member_result.is_err());
    }

    #[test]
    fn test_post_message_recovers_from_poisoned_runtime_state_lock() {
        let runtime = ConversationRuntime::new(InMemoryJournal::default());
        poison_rwlock_write(&runtime.state);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            runtime.post_message(PostMessageCommand {
                tenant_id: "100001".into(),
                organization_id: "default".into(),
                conversation_id: "c_demo".into(),
                sender: Sender {
                    id: "1".into(),
                    kind: "user".into(),
                    member_id: None,
                    device_id: None,
                    session_id: None,
                    metadata: BTreeMap::new(),
                },
                client_msg_id: None,
                message_type: MessageType::Standard,
                body: MessageBody {
                    summary: Some("hello".into()),
                    parts: vec![im_domain_core::message::ContentPart::text("hello")],
                    render_hints: BTreeMap::new(),
                    reply_to: None,
                },
            })
        }));
        assert!(
            result.is_ok(),
            "post_message should not panic when runtime state lock is poisoned"
        );
        let post_result = result.expect("panic status should be captured");
        assert!(post_result.is_err());
    }

    #[test]
    fn evict_idle_conversations_removes_all_evicted_companion_indexes() {
        let mut state = RuntimeState::default();
        let old_scope = conversation_scope_key("100001", "default", "c_old");
        let current_scope = conversation_scope_key("100001", "default", "c_current");
        let old_member = ConversationMember {
            tenant_id: "100001".into(),
            conversation_id: "c_old".into(),
            member_id: "m_old".into(),
            principal_id: "alice".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            state: MembershipState::Joined,
            invited_by: None,
            joined_at: "2026-07-10T00:00:00.000Z".into(),
            removed_at: None,
            attributes: BTreeMap::new(),
        };
        let mut old_conversation = ConversationState {
            last_accessed_at_ms: 1,
            ..Default::default()
        };
        old_conversation
            .aggregate
            .replace_business_binding(Some(ConversationBusinessBinding {
                business_type: "thread".into(),
                business_id: "root_old".into(),
            }));
        old_conversation.roster.upsert_member(old_member.clone());
        state.conversations.insert(old_scope, old_conversation);
        state.conversations.insert(
            current_scope,
            ConversationState {
                last_accessed_at_ms: 2,
                ..Default::default()
            },
        );
        state.message_locator.register("100001", "101", "c_old");
        state.message_locator.register("100001", "102", "c_current");
        let business_scope = conversation_business_scope_key("100001", "thread", "root_old");
        state
            .business_index
            .insert(business_scope.clone(), "c_old".into());
        state.sync_actor_inbox_member("0", &old_member);

        state.refresh_dirty_conversation_weights();
        let byte_budget = state.estimated_conversation_bytes;
        assert_eq!(state.evict_idle_conversations(1, byte_budget), 1);

        assert_eq!(state.message_locator.conversation_id("100001", "101"), None);
        assert_eq!(
            state.message_locator.conversation_id("100001", "102"),
            Some("c_current")
        );
        assert!(!state.business_index.contains_key(business_scope.as_str()));
        assert!(
            state
                .actor_inbox_page("100001", "0", "user", "alice", 0, 20)
                .items
                .is_empty()
        );
    }

    #[test]
    fn global_runtime_byte_budget_evicts_oldest_conversations_before_oom() {
        let mut state = RuntimeState::default();
        for (index, conversation_id) in ["c_old", "c_middle", "c_current"].into_iter().enumerate() {
            let scope = conversation_scope_key("100001", "0", conversation_id);
            let mut conversation = ConversationState {
                last_accessed_at_ms: (index + 1) as u64,
                ..Default::default()
            };
            conversation.message_log.store_posted(Message {
                tenant_id: "100001".into(),
                conversation_id: conversation_id.into(),
                message_id: format!("message_{index}"),
                message_seq: 1,
                sender: Sender {
                    id: "1".into(),
                    kind: "user".into(),
                    member_id: None,
                    device_id: None,
                    session_id: None,
                    metadata: BTreeMap::new(),
                },
                message_type: MessageType::Standard,
                delivery_mode: "discrete".into(),
                client_msg_id: None,
                stream_session_id: None,
                rtc_session_id: None,
                body: MessageBody {
                    summary: None,
                    parts: vec![ContentPart::text("x".repeat(512 * 1024))],
                    render_hints: BTreeMap::new(),
                    reply_to: None,
                },
                attributes: BTreeMap::new(),
                metadata: BTreeMap::new(),
                occurred_at: "2026-07-10T00:00:00.000Z".into(),
                committed_at: Some("2026-07-10T00:00:00.000Z".into()),
            });
            state.insert_conversation(scope, conversation);
            state.message_locator.register(
                "100001",
                format!("message_{index}").as_str(),
                conversation_id,
            );
        }
        let byte_budget = state
            .conversation_weights
            .values()
            .copied()
            .take(2)
            .sum::<usize>();

        let evicted = state.evict_idle_conversations(100, byte_budget);

        assert!(evicted >= 1);
        assert!(state.estimated_conversation_bytes <= byte_budget);
        assert!(
            !state
                .conversations
                .contains_key(conversation_scope_key("100001", "0", "c_old").as_str())
        );
        assert_eq!(
            state.message_locator.conversation_id("100001", "message_0"),
            None
        );
    }

    #[test]
    fn conversation_idempotency_replay_caches_are_bounded_under_high_volume() {
        let mut conversation = ConversationState::default();
        for index in 0..2_048 {
            conversation.posted_message_requests.insert(
                format!("post-{index}"),
                PostedMessageReplayRecord {
                    sender_id: "1".into(),
                    sender_kind: "user".into(),
                    message_type: MessageType::Standard,
                    body: MessageBody {
                        summary: Some("bounded".into()),
                        parts: vec![ContentPart::text("bounded")],
                        render_hints: BTreeMap::new(),
                        reply_to: None,
                    },
                    message_id: index.to_string(),
                },
            );
            conversation.message_mutation_requests.insert(
                format!("mutation-{index}"),
                MessageMutationReplayRecord {
                    result: MessageMutationResult {
                        conversation_id: "c_bound".into(),
                        message_id: index.to_string(),
                        message_seq: index,
                        event_id: format!("evt-{index}"),
                    },
                },
            );
        }

        assert!(conversation.posted_message_requests.len() <= 1_024);
        assert!(conversation.message_mutation_requests.len() <= 1_024);
        assert!(
            conversation
                .posted_message_requests
                .get("post-2047")
                .is_some()
        );
        assert!(
            conversation
                .message_mutation_requests
                .get("mutation-2047")
                .is_some()
        );
    }

    #[test]
    fn posted_message_replay_cache_is_bounded_by_estimated_bytes() {
        let mut conversation = ConversationState::default();
        let body_text = "x".repeat(256 * 1024);
        for index in 0..64 {
            conversation.posted_message_requests.insert(
                format!("large-post-{index}"),
                PostedMessageReplayRecord {
                    sender_id: "1".into(),
                    sender_kind: "user".into(),
                    message_type: MessageType::Standard,
                    body: MessageBody {
                        summary: None,
                        parts: vec![ContentPart::text(body_text.clone())],
                        render_hints: BTreeMap::new(),
                        reply_to: None,
                    },
                    message_id: index.to_string(),
                },
            );
        }

        let estimated_bytes = conversation
            .posted_message_requests
            .entries
            .iter()
            .map(|(key, value)| {
                key.len()
                    + value.sender_id.len()
                    + value.sender_kind.len()
                    + value.message_id.len()
                    + serde_json::to_vec(&value.body)
                        .expect("message body should serialize")
                        .len()
            })
            .sum::<usize>();
        assert!(estimated_bytes <= 8 * 1024 * 1024);
    }

    #[test]
    fn runtime_metrics_report_bounded_cache_pressure_and_byte_evictions() {
        let runtime = ConversationRuntime::new(InMemoryJournal::default());
        {
            let mut state = write_runtime_state(&runtime.state, "runtime metrics test setup");
            for (index, conversation_id) in ["c_metrics_old", "c_metrics_current"]
                .into_iter()
                .enumerate()
            {
                let mut conversation = ConversationState {
                    last_accessed_at_ms: (index + 1) as u64,
                    ..Default::default()
                };
                let binding = ConversationBusinessBinding {
                    business_type: "thread".into(),
                    business_id: format!("metrics_root_{index}"),
                };
                conversation
                    .aggregate
                    .replace_business_binding(Some(binding.clone()));
                let member = ConversationMember {
                    tenant_id: "100001".into(),
                    conversation_id: conversation_id.into(),
                    member_id: format!("metrics_member_{index}"),
                    principal_id: "metrics_actor".into(),
                    principal_kind: "user".into(),
                    role: MembershipRole::Member,
                    state: MembershipState::Joined,
                    invited_by: None,
                    joined_at: "2026-07-10T00:00:00.000Z".into(),
                    removed_at: None,
                    attributes: BTreeMap::new(),
                };
                conversation.roster.upsert_member(member.clone());
                conversation.message_log.store_posted(Message {
                    tenant_id: "100001".into(),
                    conversation_id: conversation_id.into(),
                    message_id: format!("metrics_message_{index}"),
                    message_seq: 1,
                    sender: Sender {
                        id: "1".into(),
                        kind: "user".into(),
                        member_id: None,
                        device_id: None,
                        session_id: None,
                        metadata: BTreeMap::new(),
                    },
                    message_type: MessageType::Standard,
                    delivery_mode: "discrete".into(),
                    client_msg_id: None,
                    stream_session_id: None,
                    rtc_session_id: None,
                    body: MessageBody {
                        summary: None,
                        parts: vec![ContentPart::text("x".repeat(256 * 1024))],
                        render_hints: BTreeMap::new(),
                        reply_to: None,
                    },
                    attributes: BTreeMap::new(),
                    metadata: BTreeMap::new(),
                    occurred_at: "2026-07-10T00:00:00.000Z".into(),
                    committed_at: Some("2026-07-10T00:00:00.000Z".into()),
                });
                let scope = conversation_scope_key("100001", "0", conversation_id);
                state.insert_conversation(scope, conversation);
                state.business_index.insert(
                    conversation_business_scope_key(
                        "100001",
                        binding.business_type.as_str(),
                        binding.business_id.as_str(),
                    ),
                    conversation_id.into(),
                );
                state.sync_actor_inbox_member("0", &member);
                state.message_locator.register(
                    "100001",
                    format!("metrics_message_{index}").as_str(),
                    conversation_id,
                );
            }
        }

        let before = runtime.runtime_metrics_snapshot();
        assert_eq!(before.conversation_entries, 2);
        assert_eq!(before.message_cache_entries, 2);
        assert!(before.message_cache_bytes >= 512 * 1024);
        assert_eq!(before.message_locator_entries, 2);
        assert_eq!(before.business_binding_entries, 2);
        assert_eq!(before.actor_inbox_actor_entries, 1);
        assert_eq!(before.actor_inbox_conversation_entries, 2);
        assert_eq!(before.conversation_evictions_bytes_total, 0);

        let byte_budget = before.estimated_conversation_bytes.saturating_sub(1);
        assert!(runtime.evict_idle_conversations_with_limits(100, byte_budget) >= 1);

        let after = runtime.runtime_metrics_snapshot();
        assert!(after.estimated_conversation_bytes <= byte_budget);
        assert!(after.conversation_evictions_bytes_total >= 1);
        assert!(after.conversation_evicted_bytes_total > 0);
        assert!(after.eviction_operations_total >= 1);
        assert!(after.eviction_checks_total >= 1);
        assert_eq!(after.business_binding_entries, 1);
        assert_eq!(after.actor_inbox_actor_entries, 1);
        assert_eq!(after.actor_inbox_conversation_entries, 1);

        let rendered = runtime.render_runtime_metrics_prometheus(
            "conversation-service",
            "test",
            "standalone",
            "server",
        );
        assert!(rendered.contains("im_conversation_runtime_entries"));
        assert!(rendered.contains("im_conversation_runtime_estimated_bytes"));
        assert!(rendered.contains("im_conversation_runtime_message_cache_bytes"));
        assert!(rendered.contains("im_conversation_runtime_business_binding_entries"));
        assert!(rendered.contains("im_conversation_runtime_actor_inbox_actor_entries"));
        assert!(rendered.contains("im_conversation_runtime_actor_inbox_conversation_entries"));
        assert!(rendered.contains(
            "im_conversation_runtime_evictions_total{service=\"conversation-service\",environment=\"test\",deployment_profile=\"standalone\",runtime_target=\"server\",reason=\"bytes\"}"
        ));
        assert!(!rendered.contains("100001"));
        assert!(!rendered.contains("c_metrics_old"));
    }

    #[test]
    fn runtime_metrics_snapshot_includes_the_current_scan() {
        let runtime = ConversationRuntime::new(InMemoryJournal::default());

        let first = runtime.runtime_metrics_snapshot();
        let second = runtime.runtime_metrics_snapshot();

        assert_eq!(first.metrics_scans_total, 1);
        assert_eq!(second.metrics_scans_total, 2);
    }
}
