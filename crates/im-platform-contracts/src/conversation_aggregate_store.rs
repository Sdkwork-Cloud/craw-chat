use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};

use crate::{CommitEnvelope, OutboxEventRecord, ReplaceConversationAgentAssignments};

pub const CONVERSATION_AGGREGATE_PAGE_SIZE_DEFAULT: usize = 20;
pub const CONVERSATION_AGGREGATE_PAGE_SIZE_MAX: usize = 200;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationMemberRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub principal_kind: String,
    pub principal_id: String,
    pub member_id: i64,
    pub membership_role: String,
    pub membership_state: String,
    pub invited_by: Option<String>,
    pub joined_at: String,
    pub removed_at: Option<String>,
    pub attributes_json: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadCursorRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub member_id: i64,
    #[serde(default)]
    pub device_id: String,
    pub principal_kind: String,
    pub principal_id: String,
    pub read_seq: u64,
    pub last_read_message_id: Option<i64>,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationMemberPageCursor {
    pub principal_kind: String,
    pub principal_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationMemberPage {
    pub items: Vec<ConversationMemberRecord>,
    pub next_cursor: Option<ConversationMemberPageCursor>,
    pub has_more: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadCursorPageCursor {
    pub member_id: i64,
    pub device_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadCursorPage {
    pub items: Vec<ReadCursorRecord>,
    pub next_cursor: Option<ReadCursorPageCursor>,
    pub has_more: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationAggregateState {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub members: Vec<ConversationMemberRecord>,
    pub read_cursors: Vec<ReadCursorRecord>,
    pub high_watermark: u64,
}

/// Typed current-state row for the normalized `im_conversations` authority.
/// Journal payloads are deliberately absent: callers must provide current
/// state directly instead of asking persistence to derive it from events.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedConversationRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub conversation_type: String,
    pub lifecycle_state: String,
    pub commit_seq: u64,
    pub member_epoch: u64,
    pub last_activity_at: String,
    pub retention_until: Option<String>,
}

/// One command-side PostgreSQL unit of work. Every collection contains only
/// facts produced by the command being committed; this is not a replay or
/// materialization contract.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedConversationCommit {
    pub conversation: NormalizedConversationRecord,
    pub members: Vec<ConversationMemberRecord>,
    pub read_cursors: Vec<ReadCursorRecord>,
    #[serde(default)]
    pub agent_assignments: Option<ReplaceConversationAgentAssignments>,
    pub envelopes: Vec<CommitEnvelope>,
    pub outboxes: Vec<OutboxEventRecord>,
}

/// Durable repository boundary for conversation membership, read cursors, and
/// message high-watermarks.
///
/// Collection reads are keyset-paged at the authoritative store. Consumers
/// must either request one bounded page or use targeted member/cursor lookups;
/// the contract intentionally exposes no load-all aggregate operation.
pub trait ConversationAggregateStore: Send + Sync {
    fn load_members_page(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        cursor: Option<&ConversationMemberPageCursor>,
        page_size: usize,
    ) -> Result<ConversationMemberPage, ContractError>;

    fn load_member(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Result<Option<ConversationMemberRecord>, ContractError>;

    fn load_member_by_id(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        member_id: i64,
    ) -> Result<Option<ConversationMemberRecord>, ContractError>;

    /// Loads active realtime recipients who had already joined when a durable
    /// conversation event was created.
    fn load_event_recipients_page(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        joined_before_or_at: &str,
        cursor: Option<&ConversationMemberPageCursor>,
        page_size: usize,
    ) -> Result<ConversationMemberPage, ContractError>;

    fn upsert_member(&self, member: ConversationMemberRecord) -> Result<(), ContractError>;

    fn remove_member(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_kind: &str,
        principal_id: &str,
        removed_at: &str,
    ) -> Result<(), ContractError>;

    fn load_read_cursors_page(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        cursor: Option<&ReadCursorPageCursor>,
        page_size: usize,
    ) -> Result<ReadCursorPage, ContractError>;

    fn load_read_cursor(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        member_id: i64,
    ) -> Result<Option<ReadCursorRecord>, ContractError>;

    fn upsert_read_cursor(&self, cursor: ReadCursorRecord) -> Result<(), ContractError>;

    fn load_high_watermark(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<u64, ContractError>;

    fn allocate_member_id(&self) -> Result<i64, ContractError>;

    fn conversation_exists(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<bool, ContractError>;
}
