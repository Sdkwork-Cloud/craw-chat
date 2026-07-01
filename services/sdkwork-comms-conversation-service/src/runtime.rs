use im_app_context::AppContext;
use im_platform_contracts::{
    ConversationAggregateStore, ConversationMemberRecord, ConversationSeqAllocator, IdGenerator,
    MessageStore, OutboxStore, ReadCursorRecord, RetentionScopeStore, StoredMessageRecord,
};
use im_domain_core::retention::retention_until_from_envelope;
use sdkwork_im_contract_core::ContractError;
use sdkwork_im_contract_message::{CommitJournal, CommitPosition};
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub use im_domain_core::conversation::{
    AgentHandoffStateView, ChangeAgentHandoffStatusView, ConversationBusinessBinding,
};
use im_domain_core::conversation::{
    ConversationAggregateState, ConversationMember, ConversationPolicy, ConversationReadCursor,
    ConversationReadCursorView, ConversationRoster, MembershipRole, MembershipState,
    build_conversation_member_with_attributes, build_default_read_cursor, member_episode_id,
    member_id,
};
use im_domain_core::media::{DriveReference, MediaResource, MediaSource};
use im_domain_core::message::{
    ContentPart, ConversationMessageLog, Message, MessageBody, MessageEdited, MessageLocatorIndex,
    MessagePinned, MessageReactionAdded, MessageReactionRemoved, MessageRecalled, MessageType,
    MessageUnpinned, ReactionActorIdentity, Sender,
};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_time::utc_now_rfc3339_millis;
use serde::{Deserialize, Serialize};
use serde_json::{Value as JsonValue, json};
use sdkwork_utils_rust::sha256_hash;

mod binding;
mod creation;
mod governance;
mod handoff;
pub mod http;
mod journal_bootstrap;
pub mod internal_rpc_dispatch;
pub mod rpc_dispatch;
mod membership;
mod policy;
mod recovery;
mod room;
mod support;

use self::governance::ConversationPolicyAppliedPayload;
use self::policy::MessagePostPolicy;
use self::support::{
    build_agent_handoff_status_changed_envelope, build_conversation_policy_applied_envelope,
    build_member_envelope, build_member_role_changed_envelope, build_message_edited_envelope,
    build_message_pinned_envelope, build_message_reaction_added_envelope,
    build_message_reaction_removed_envelope, build_message_recalled_envelope,
    build_message_unpinned_envelope, build_owner_transfer_envelope, build_read_cursor_envelope,
    conversation_business_scope_key, conversation_retention_class, conversation_scope_key,
    conversation_scope_key_for_envelope, conversation_timestamp, encode_conversation_key_segments,
    event_id_component, next_member_episode, resolve_active_member, resolve_active_member_id,
    resolve_active_member_id_with_kind, resolve_active_member_with_kind,
    resolve_agent_dialog_conversation_id, resolve_direct_chat_binding_ids, upsert_member,
    upsert_read_cursor,
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
const MESSAGE_REACTION_KEY_MAX_BYTES: usize = 128;
const MESSAGE_BODY_MAX_BYTES: usize = 512 * 1024;
const MESSAGE_HISTORY_DEFAULT_LIMIT: usize = 100;
const MESSAGE_HISTORY_MAX_LIMIT: usize = 1000;
const CONVERSATION_MEMBER_LIST_DEFAULT_LIMIT: usize = 100;
const CONVERSATION_MEMBER_LIST_MAX_LIMIT: usize = 1000;
const CONVERSATION_CREATE_DELIVERY_PROOF_VERSION: &str = "conversation.create.delivery-proof.v1";
const CONVERSATION_MESSAGE_DELIVERY_PROOF_VERSION: &str = "conversation.message.delivery-proof.v1";
const CONVERSATION_MAX_IN_MEMORY_DEFAULT: usize = 10_000;
const CONVERSATION_IDLE_EVICTION_TARGET_RATIO: f64 = 0.8;
const CONVERSATION_MAX_IN_MEMORY_ENV: &str = "SDKWORK_IM_CONVERSATION_MAX_IN_MEMORY";

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
}

impl CreateConversationResult {
    pub fn new(conversation_id: String, event_id: String) -> Self {
        Self {
            conversation_id,
            event_id,
            request_key: None,
            delivery_status: None,
            proof_version: None,
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

#[allow(clippy::large_enum_variant)]
enum PostMessageMutation {
    Applied {
        result: PostMessageResult,
        message: Message,
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
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecallMessageCommand {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub message_id: String,
    pub recalled_by: Sender,
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
    pub items: Vec<im_domain_core::message::StoredMessage>,
    pub high_watermark: u64,
    pub next_after_seq: Option<u64>,
    pub has_more: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListMembersResult {
    pub items: Vec<ConversationMember>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListPinnedMessagesResult {
    pub message_ids: Vec<String>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InboxRetrieveResult {
    pub conversation_ids: Vec<String>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

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

fn runtime_json_string<T: serde::Serialize>(value: &T) -> Result<String, RuntimeError> {
    serde_json::to_string(value).map_err(|error| {
        RuntimeError::InvalidInput(format!("failed to serialize runtime payload: {error}"))
    })
}

#[derive(Default)]
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
    posted_message_requests: HashMap<String, PostedMessageReplayRecord>,
    last_accessed_at_ms: u64,
}

#[derive(Default)]
struct RuntimeState {
    conversations: HashMap<String, ConversationState>,
    business_index: HashMap<String, String>,
    message_locator: MessageLocatorIndex,
}

fn lock_runtime_mutex<'a, T>(mutex: &'a Mutex<T>, label: &'static str) -> MutexGuard<'a, T> {
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

impl RuntimeState {
    fn touch_conversation(&mut self, scope_key: &str) {
        if let Some(conv) = self.conversations.get_mut(scope_key) {
            conv.last_accessed_at_ms = now_ms();
        }
    }

    fn evict_idle_conversations(&mut self, max_conversations: usize) -> usize {
        let count = self.conversations.len();
        if count <= max_conversations {
            return 0;
        }
        let target = (max_conversations as f64 * CONVERSATION_IDLE_EVICTION_TARGET_RATIO) as usize;
        let evict_count = count.saturating_sub(target.max(1));
        let mut entries: Vec<(String, u64)> = self
            .conversations
            .iter()
            .map(|(k, v)| {
                let ts = if v.last_accessed_at_ms == 0 {
                    u64::MAX
                } else {
                    v.last_accessed_at_ms
                };
                (k.clone(), ts)
            })
            .collect();
        entries.sort_by_key(|(_, ts)| *ts);
        for (key, _) in entries.iter().take(evict_count) {
            self.conversations.remove(key.as_str());
        }
        evict_count
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
        if let ContentPart::Media(media_part) = part {
            let drive = &media_part.drive;
            validate_media_drive_reference(index, drive)?;
            validate_media_resource_drive_snapshot(index, &media_part.resource, drive)?;
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
) -> bool {
    existing.creator_id == command.creator_id
        && existing.creator_kind == creator_kind
        && existing.requested_kind == command.conversation_type
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

fn direct_chat_binding_replay_matches(
    existing: &DirectChatBindingReplayRecord,
    command: &BindDirectChatConversationCommand,
    binder_kind: &str,
    pair: &im_domain_core::social::NormalizedActorPair,
    direct_chat_id: &str,
) -> bool {
    existing.bound_by == command.bound_by
        && existing.binder_kind == binder_kind
        && existing.direct_chat_id == direct_chat_id
        && existing.anchor_actor_id == pair.left_actor_id
        && existing.anchor_actor_kind == command.left_actor_kind
        && existing.peer_actor_id == pair.right_actor_id
        && existing.peer_actor_kind == command.right_actor_kind
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

fn post_message_request_key_from_message(message: &Message) -> Option<String> {
    message.client_msg_id.as_ref().map(|client_msg_id| {
        encode_conversation_key_segments([
            message.tenant_id.as_str(),
            message.sender.kind.as_str(),
            message.sender.id.as_str(),
            "message",
            message.conversation_id.as_str(),
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

// This internal transition result keeps the full idempotent view and mutation
// payload inline because the runtime immediately pattern-matches and forwards it
// within a single command path; boxing would add indirection without changing
// release behavior.
#[allow(clippy::large_enum_variant)]
enum AgentHandoffStatusTransitionOutcome {
    Idempotent(AgentHandoffStateView),
    Mutated {
        payload: AgentHandoffStatusChangedPayload,
        ordering_seq: u64,
        actor_id: String,
        actor_kind: String,
        retention_class: String,
    },
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
}

pub struct ConversationRuntime<J> {
    journal: J,
    state: RwLock<RuntimeState>,
    /// 可选的消息真值存储。注入后 post_message 走 DB seq 分配 + 真值写入路径。
    message_store: Option<Arc<dyn MessageStore>>,
    /// 可选的 Outbox 存储。注入后事件通过 outbox 异步投递。
    outbox_store: Option<Arc<dyn OutboxStore>>,
    /// 可选的 ID 生成器。注入后 message_id/event_id 使用 Snowflake。
    id_generator: Option<Arc<dyn IdGenerator>>,
    /// 可选的会话聚合存储。注入后成员/已读游标从 DB 加载和持久化，
    /// 替代纯内存状态，使多实例部署共享会话聚合视图。
    aggregate_store: Option<Arc<dyn ConversationAggregateStore>>,
    /// 可选的序列号分配器。注入后 message_seq 走 Redis INCRBY 批量预取，
    /// 消除 im_conversation_seq_counters 单行热点。
    seq_allocator: Option<Arc<dyn ConversationSeqAllocator>>,
    /// 可选的保留期协调存储。注入后在 indefinite retention 策略下清除过期标记。
    retention_scope_store: Option<Arc<dyn RetentionScopeStore>>,
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn new(journal: J) -> Self {
        Self {
            journal,
            state: RwLock::new(RuntimeState::default()),
            message_store: None,
            outbox_store: None,
            id_generator: None,
            aggregate_store: None,
            seq_allocator: None,
            retention_scope_store: None,
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

    /// 注入序列号分配器，启用 Redis 批量预取的消息序号分配。
    pub fn with_seq_allocator(mut self, allocator: Arc<dyn ConversationSeqAllocator>) -> Self {
        self.seq_allocator = Some(allocator);
        self
    }

    pub fn with_retention_scope_store(mut self, store: Arc<dyn RetentionScopeStore>) -> Self {
        self.retention_scope_store = Some(store);
        self
    }

    /// 运行时是否已配置 DB 真值存储路径。
    pub fn has_message_store(&self) -> bool {
        self.message_store.is_some()
    }

    pub fn reset_for_recovery(&self) {
        *write_runtime_state(&self.state, "runtime state") = RuntimeState::default();
    }

    pub fn recover_from_journal(&self) -> Result<usize, RuntimeError> {
        let snapshot = self.journal.recorded().map_err(RuntimeError::from)?;
        let mut recovered_count = 0usize;
        for envelope in &snapshot {
            self.apply_recovered_envelope(envelope)?;
            recovered_count = recovered_count.saturating_add(1);
        }
        Ok(recovered_count)
    }

    pub fn evict_idle_conversations(&self) -> usize {
        let max = resolve_max_conversations_in_memory();
        let mut state = write_runtime_state(&self.state, "runtime.state.evict_idle");
        state.evict_idle_conversations(max)
    }

    fn recover_single_conversation(
        &self,
        tenant_id: &str,
        conversation_id: &str,
    ) -> Result<usize, RuntimeError> {
        let snapshot = self.journal.recorded().map_err(RuntimeError::from)?;
        let mut recovered = 0usize;
        for envelope in &snapshot {
            if envelope.tenant_id == tenant_id && envelope.scope_id == conversation_id {
                self.apply_recovered_envelope(envelope)?;
                recovered += 1;
            }
        }
        if recovered == 0 {
            return Err(RuntimeError::ConversationNotFound(
                conversation_id.to_owned(),
            ));
        }
        Ok(recovered)
    }

    fn ensure_conversation_loaded(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<(), RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        {
            let state = read_runtime_state(&self.state, "runtime.state.ensure_conversation_loaded");
            if state.conversations.contains_key(scope_key.as_str()) {
                return Ok(());
            }
        }
        // 优先路径 1：有 AggregateStore 时从 DB 加载聚合状态。
        if let Some(ref aggregate_store) = self.aggregate_store {
            let organization_id =
                im_domain_events::normalize_commit_organization_id(organization_id);
            let mut state =
                write_runtime_state(&self.state, "ensure_conversation_loaded.aggregate");
            let mut conversation_state = ConversationState {
                last_accessed_at_ms: now_ms(),
                ..Default::default()
            };
            if let Ok(db_state) =
                aggregate_store.load_aggregate_state(tenant_id, organization_id.as_str(), conversation_id)
            {
                for member_record in &db_state.members {
                    let member = conversation_member_from_record(member_record);
                    conversation_state.roster.upsert_member(member);
                }
                for cursor_record in &db_state.read_cursors {
                    let cursor = read_cursor_from_record(cursor_record);
                    conversation_state.roster.upsert_read_cursor(cursor);
                }
            }
            state.conversations.insert(scope_key, conversation_state);
            return Ok(());
        }
        // 优先路径 2：如果有 MessageStore 但从 journal 重放太昂贵，
        // 初始化空状态（消息真值已在 im_conversation_messages 中）。
        if self.message_store.is_some() {
            let mut state =
                write_runtime_state(&self.state, "ensure_conversation_loaded.store_path");
            let conversation_state = ConversationState {
                last_accessed_at_ms: now_ms(),
                ..Default::default()
            };
            state.conversations.insert(scope_key, conversation_state);
            return Ok(());
        }
        // Fallback 路径：无 store 时从 journal 重放（仅用于测试/单机模式）
        self.recover_single_conversation(tenant_id, conversation_id)?;
        Ok(())
    }

    fn maybe_evict_after_write(&self) {
        let max = resolve_max_conversations_in_memory();
        let mut state = write_runtime_state(&self.state, "runtime.state.maybe_evict");
        state.evict_idle_conversations(max);
    }

    /// 将会话的当前内存聚合状态（成员 + 已读游标）持久化到 AggregateStore。
    /// 多实例部署时在成员变更/已读游标更新后调用，使其他实例可见。
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
        for member in conversation.roster.members().values() {
            let record = member_to_record(tenant_id, organization_id, conversation_id, member);
            store.upsert_member(record).map_err(RuntimeError::from)?;
        }
        for cursor in conversation.roster.read_cursors().values() {
            let record = cursor_to_record(tenant_id, organization_id, conversation_id, cursor);
            store
                .upsert_read_cursor(record)
                .map_err(RuntimeError::from)?;
        }
        Ok(())
    }

    pub fn post_message(
        &self,
        command: PostMessageCommand,
    ) -> Result<PostMessageResult, RuntimeError> {
        self.post_message_with_policy(command, MessagePostPolicy::GenericPost)
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
        )
    }

    fn post_message_with_policy(
        &self,
        command: PostMessageCommand,
        policy: MessagePostPolicy,
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
        self.ensure_conversation_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.conversation_id.as_str(),
        )?;
        let request_key = post_message_request_key(&command);
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), command.conversation_id.as_str());
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
                    let sender_member = resolve_active_member_with_kind(
                        conversation,
                        command.sender.id.as_str(),
                        command.sender.kind.as_str(),
                    )?;
                    policy::ensure_actor_kind_matches_member(
                        &sender_member,
                        command.sender.kind.as_str(),
                    )?;
                    match policy {
                        MessagePostPolicy::GenericPost => {
                            policy::ensure_message_post_allowed(conversation, &sender_member)?;
                            policy::ensure_room_message_post_allowed(conversation, &sender_member)?;
                        }
                        MessagePostPolicy::SystemChannelPublish => {
                            policy::ensure_system_channel_publish_command_allowed(
                                conversation,
                                &sender_member,
                            )?
                        }
                    }
                    // 序号分配：优先使用 DB 原子分配器，fallback 到内存 high_watermark
                    let message_seq = if let Some(store) = &self.message_store {
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
                    if sender.member_id.is_none() {
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
                        ordering_seq: message.message_seq,
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
                        retention_class,
                        audit_class: "default".into(),
                    };

                    self.journal.append(envelope)?;

                    // 真值写入：如果有 MessageStore，写入 im_conversation_messages（消息真值表）
                    if let Some(store) = &self.message_store {
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
                        };
                        store
                            .insert_message(stored_record)
                            .map_err(RuntimeError::from)?;
                    }

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
                    }
                }
            };

            if let PostMessageMutation::Applied { message, .. } = &mutation {
                state.message_locator.register_message(message);
            }

            mutation
        };

        match mutation {
            PostMessageMutation::Replayed(result) => Ok(result),
            PostMessageMutation::Applied { result, .. } => {
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
        let conversation_id = {
            let state = read_runtime_state(&self.state, "runtime.state.edit_message.locate");
            state
                .message_locator
                .conversation_id(command.tenant_id.as_str(), command.message_id.as_str())
                .map(str::to_owned)
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?
        };
        self.ensure_conversation_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            conversation_id.as_str(),
        )?;
        let edited = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.edit_message");
            let scope_key =
                conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), conversation_id.as_str());
            state.touch_conversation(scope_key.as_str());
            let conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
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
            self.journal.append(build_message_edited_envelope(
                &edited,
                command.organization_id.as_str(),
                format!("evt_{}_edited", edited.message_id).as_str(),
                retention_class.as_str(),
            ))?;
            conversation
                .message_log
                .apply_edited(&edited)
                .ok_or_else(|| RuntimeError::MessageNotFound(edited.message_id.clone()))?;
            edited
        };

        let event_id = format!("evt_{}_edited", edited.message_id);

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
        let conversation_id = {
            let state = read_runtime_state(&self.state, "runtime.state.recall_message.locate");
            state
                .message_locator
                .conversation_id(command.tenant_id.as_str(), command.message_id.as_str())
                .map(str::to_owned)
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?
        };
        self.ensure_conversation_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            conversation_id.as_str(),
        )?;
        let recalled = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.recall_message");
            let scope_key =
                conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), conversation_id.as_str());
            state.touch_conversation(scope_key.as_str());
            let conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
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
            self.journal.append(build_message_recalled_envelope(
                &recalled,
                command.organization_id.as_str(),
                format!("evt_{}_recalled", recalled.message_id).as_str(),
                retention_class.as_str(),
            ))?;
            conversation
                .message_log
                .apply_recalled(&recalled)
                .ok_or_else(|| RuntimeError::MessageNotFound(recalled.message_id.clone()))?;
            recalled
        };

        let event_id = format!("evt_{}_recalled", recalled.message_id);

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
        let conversation_id = {
            let state =
                read_runtime_state(&self.state, "runtime.state.add_message_reaction.locate");
            state
                .message_locator
                .conversation_id(command.tenant_id.as_str(), command.message_id.as_str())
                .map(str::to_owned)
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?
        };
        self.ensure_conversation_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            conversation_id.as_str(),
        )?;
        let (reaction, changed) = {
            let mut state = write_runtime_state(
                &self.state,
                "conversation-runtime.state.add_message_reaction",
            );
            let scope_key =
                conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), conversation_id.as_str());
            state.touch_conversation(scope_key.as_str());
            let conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
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
            if changed {
                let retention_class = conversation_retention_class(conversation);
                self.journal.append(build_message_reaction_added_envelope(
                    &reaction,
                    command.organization_id.as_str(),
                    format!(
                        "evt_{}_reaction_added_{}_{}_{}",
                        reaction.message_id,
                        event_id_component(reaction.reaction_key.as_str()),
                        event_id_component(reaction.reacted_by.id.as_str()),
                        event_id_component(reaction.reacted_at.as_str())
                    )
                    .as_str(),
                    retention_class.as_str(),
                ))?;
                conversation
                    .message_log
                    .apply_reaction_added(&reaction)
                    .ok_or_else(|| RuntimeError::MessageNotFound(reaction.message_id.clone()))?;
            }
            (reaction, changed)
        };

        let event_id = if changed {
            Some(format!(
                "evt_{}_reaction_added_{}_{}_{}",
                reaction.message_id,
                event_id_component(reaction.reaction_key.as_str()),
                event_id_component(reaction.reacted_by.id.as_str()),
                event_id_component(reaction.reacted_at.as_str())
            ))
        } else {
            None
        };

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
        let conversation_id = {
            let state =
                read_runtime_state(&self.state, "runtime.state.remove_message_reaction.locate");
            state
                .message_locator
                .conversation_id(command.tenant_id.as_str(), command.message_id.as_str())
                .map(str::to_owned)
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?
        };
        self.ensure_conversation_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            conversation_id.as_str(),
        )?;
        let (reaction, changed) = {
            let mut state = write_runtime_state(
                &self.state,
                "conversation-runtime.state.remove_message_reaction",
            );
            let scope_key =
                conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), conversation_id.as_str());
            state.touch_conversation(scope_key.as_str());
            let conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
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
            if changed {
                let retention_class = conversation_retention_class(conversation);
                self.journal
                    .append(build_message_reaction_removed_envelope(
                        &reaction,
                        command.organization_id.as_str(),
                        format!(
                            "evt_{}_reaction_removed_{}_{}_{}",
                            reaction.message_id,
                            event_id_component(reaction.reaction_key.as_str()),
                            event_id_component(reaction.removed_by.id.as_str()),
                            event_id_component(reaction.removed_at.as_str())
                        )
                        .as_str(),
                        retention_class.as_str(),
                    ))?;
                conversation
                    .message_log
                    .apply_reaction_removed(&reaction)
                    .ok_or_else(|| RuntimeError::MessageNotFound(reaction.message_id.clone()))?;
            }
            (reaction, changed)
        };

        let event_id = if changed {
            Some(format!(
                "evt_{}_reaction_removed_{}_{}_{}",
                reaction.message_id,
                event_id_component(reaction.reaction_key.as_str()),
                event_id_component(reaction.removed_by.id.as_str()),
                event_id_component(reaction.removed_at.as_str())
            ))
        } else {
            None
        };

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
        let conversation_id = {
            let state = read_runtime_state(&self.state, "runtime.state.pin_message.locate");
            state
                .message_locator
                .conversation_id(command.tenant_id.as_str(), command.message_id.as_str())
                .map(str::to_owned)
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?
        };
        self.ensure_conversation_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            conversation_id.as_str(),
        )?;
        let (pin, changed) = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.pin_message");
            let scope_key =
                conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), conversation_id.as_str());
            state.touch_conversation(scope_key.as_str());
            let conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
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
            if changed {
                let retention_class = conversation_retention_class(conversation);
                self.journal.append(build_message_pinned_envelope(
                    &pin,
                    command.organization_id.as_str(),
                    format!(
                        "evt_{}_pin_added_{}_{}",
                        pin.message_id,
                        event_id_component(pin.pinned_by.id.as_str()),
                        event_id_component(pin.pinned_at.as_str())
                    )
                    .as_str(),
                    retention_class.as_str(),
                ))?;
                conversation
                    .message_log
                    .apply_pinned(&pin)
                    .ok_or_else(|| RuntimeError::MessageNotFound(pin.message_id.clone()))?;
            }
            (pin, changed)
        };

        let event_id = if changed {
            Some(format!(
                "evt_{}_pin_added_{}_{}",
                pin.message_id,
                event_id_component(pin.pinned_by.id.as_str()),
                event_id_component(pin.pinned_at.as_str())
            ))
        } else {
            None
        };

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
        let conversation_id = {
            let state = read_runtime_state(&self.state, "runtime.state.unpin_message.locate");
            state
                .message_locator
                .conversation_id(command.tenant_id.as_str(), command.message_id.as_str())
                .map(str::to_owned)
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?
        };
        self.ensure_conversation_loaded(
            command.tenant_id.as_str(),
            command.organization_id.as_str(),
            conversation_id.as_str(),
        )?;
        let (pin, changed) = {
            let mut state =
                write_runtime_state(&self.state, "conversation-runtime.state.unpin_message");
            let scope_key =
                conversation_scope_key(command.tenant_id.as_str(), command.organization_id.as_str(), conversation_id.as_str());
            state.touch_conversation(scope_key.as_str());
            let conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.message_id.clone()))?;
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
            if changed {
                let retention_class = conversation_retention_class(conversation);
                self.journal.append(build_message_unpinned_envelope(
                    &pin,
                    command.organization_id.as_str(),
                    format!(
                        "evt_{}_pin_removed_{}_{}",
                        pin.message_id,
                        event_id_component(pin.unpinned_by.id.as_str()),
                        event_id_component(pin.unpinned_at.as_str())
                    )
                    .as_str(),
                    retention_class.as_str(),
                ))?;
                conversation
                    .message_log
                    .apply_unpinned(&pin)
                    .ok_or_else(|| RuntimeError::MessageNotFound(pin.message_id.clone()))?;
            }
            (pin, changed)
        };

        let event_id = if changed {
            Some(format!(
                "evt_{}_pin_removed_{}_{}",
                pin.message_id,
                event_id_component(pin.unpinned_by.id.as_str()),
                event_id_component(pin.unpinned_at.as_str())
            ))
        } else {
            None
        };

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
        self.require_active_member_with_kind(auth.tenant_id.as_str(), "default", conversation_id,
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
        self.ensure_conversation_bound_write_allowed_with_actor_kind(auth.tenant_id.as_str(), "default", conversation_id,
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
        member_id: member.member_id.parse::<i64>().unwrap_or(0),
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
        member_id: cursor.member_id.parse::<i64>().unwrap_or(0),
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

#[cfg(test)]
mod tests {
    use super::*;
    use im_domain_core::retention::retention_until_from_envelope;
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
}
