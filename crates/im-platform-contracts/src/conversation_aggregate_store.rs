// Conversation Aggregate Store Contract - 会话聚合存储契约
// 管理会话成员、已读游标等聚合状态

use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};

/// 会话成员记录
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationMemberRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub principal_kind: String,
    pub principal_id: String,
    pub member_id: i64, // Snowflake ID
    pub membership_role: String,
    pub membership_state: String,
    pub invited_by: Option<String>,
    pub joined_at: String,
    pub removed_at: Option<String>,
    pub attributes_json: String,
}

/// 已读游标记录
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadCursorRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub member_id: i64, // Snowflake ID
    pub principal_kind: String,
    pub principal_id: String,
    pub read_seq: u64,
    pub last_read_message_id: Option<i64>,
    pub updated_at: String,
}

/// 会话聚合状态（加载结果）
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationAggregateState {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub members: Vec<ConversationMemberRecord>,
    pub read_cursors: Vec<ReadCursorRecord>,
    pub high_watermark: u64,
}

/// 会话聚合存储契约
///
/// 设计原则：
/// 1. 替代 conversation-runtime 的内存 HashMap 状态
/// 2. 支持按会话加载（O(1)，而非扫全量 journal）
/// 3. member_id 为 Snowflake i64
pub trait ConversationAggregateStore: Send + Sync {
    /// 加载会话成员列表
    ///
    /// SELECT * FROM im_projection_conversation_members
    /// WHERE tenant_id=$1 AND organization_id=$2 AND conversation_id=$3
    ///   AND membership_state IN ('joined', 'linked')
    fn load_members(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<Vec<ConversationMemberRecord>, ContractError>;

    /// 加载单个成员
    fn load_member(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Result<Option<ConversationMemberRecord>, ContractError>;

    /// 插入或更新成员
    ///
    /// INSERT INTO im_projection_conversation_members (...)
    /// ON CONFLICT (tenant_id, organization_id, conversation_id, principal_kind, principal_id)
    /// DO UPDATE SET ...
    fn upsert_member(&self, member: ConversationMemberRecord) -> Result<(), ContractError>;

    /// 移除成员（更新 membership_state 为 'removed'）
    fn remove_member(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_kind: &str,
        principal_id: &str,
        removed_at: &str,
    ) -> Result<(), ContractError>;

    /// 加载会话所有已读游标
    ///
    /// SELECT * FROM im_projection_read_cursors
    /// WHERE tenant_id=$1 AND organization_id=$2 AND conversation_id=$3
    fn load_read_cursors(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<Vec<ReadCursorRecord>, ContractError>;

    /// 加载单个成员的已读游标
    fn load_read_cursor(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        member_id: i64,
    ) -> Result<Option<ReadCursorRecord>, ContractError>;

    /// 插入或更新已读游标
    ///
    /// INSERT INTO im_projection_read_cursors (...)
    /// ON CONFLICT (tenant_id, organization_id, conversation_id, member_id)
    /// DO UPDATE SET read_seq=EXCLUDED.read_seq, ...
    fn upsert_read_cursor(&self, cursor: ReadCursorRecord) -> Result<(), ContractError>;

    /// 加载完整会话聚合状态（成员 + 游标 + 高水位）
    ///
    /// 一次调用返回完整状态，减少 DB 往返
    fn load_aggregate_state(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<ConversationAggregateState, ContractError>;

    /// 分配成员 ID（Snowflake）
    fn allocate_member_id(&self) -> Result<i64, ContractError>;

    /// 检查会话是否存在（有活跃成员）
    fn conversation_exists(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<bool, ContractError>;
}
