// Message Store Contract - 消息真值存储契约
// 所有方法强制携带 organization_id，确保租户+组织双重隔离

use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};

/// 消息记录（存储层表示）
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoredMessageRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub message_id: i64,  // Snowflake ID
    pub message_seq: u64, // 会话内序号
    pub sender_principal_kind: String,
    pub sender_principal_id: String,
    pub sender_device_id: Option<String>,
    pub client_msg_id: Option<String>,
    pub message_type: String,
    pub payload_json: String, // JSON 字符串
    pub payload_hash: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention_until: Option<String>,
}

/// 消息窗口（查询结果）
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageWindow {
    pub items: Vec<StoredMessageRecord>,
    pub high_watermark: u64,
    pub next_after_seq: Option<u64>,
    pub has_more: bool,
}

/// 消息存储契约
///
/// 设计原则：
/// 1. 所有方法首参强制 (tenant_id, organization_id)
/// 2. message_id 为 Snowflake i64，全局唯一
/// 3. message_seq 由 allocate_message_seq 原子分配
/// 4. 支持客户端幂等（client_msg_id 唯一约束）
pub trait MessageStore: Send + Sync {
    /// 原子分配会话内消息序号
    ///
    /// 使用 UPDATE ... RETURNING 实现：
    /// UPDATE im_conversation_seq_counters
    /// SET next_seq = next_seq + 1
    /// WHERE tenant_id=$1 AND organization_id=$2 AND conversation_id=$3
    /// RETURNING next_seq
    fn allocate_message_seq(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<u64, ContractError>;

    /// 插入消息（真值写入）
    ///
    /// 唯一约束：
    /// - uk_im_conversation_messages_id (tenant_id, message_id)
    /// - uk_im_conversation_messages_client (tenant_id, org, conv, sender, client_msg_id)
    ///
    /// 冲突时返回 ContractError::Conflict
    fn insert_message(&self, message: StoredMessageRecord) -> Result<(), ContractError>;

    /// 读取消息窗口（分页查询）
    ///
    /// SELECT ... FROM im_conversation_messages
    /// WHERE tenant_id=$1 AND organization_id=$2 AND conversation_id=$3 AND message_seq > $4
    /// AND (retention_until IS NULL OR retention_until > NOW())
    /// ORDER BY message_seq ASC LIMIT $5
    fn read_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        after_seq: u64,
        limit: usize,
    ) -> Result<MessageWindow, ContractError>;

    /// 按 Snowflake ID 读取单条消息
    ///
    /// SELECT ... FROM im_conversation_messages
    /// WHERE tenant_id=$1 AND message_id=$2
    fn read_message_by_id(
        &self,
        tenant_id: &str,
        message_id: i64,
    ) -> Result<Option<StoredMessageRecord>, ContractError>;

    /// 按 client_msg_id 读取（幂等检查）
    fn read_message_by_client_id(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        sender_principal_kind: &str,
        sender_principal_id: &str,
        client_msg_id: &str,
    ) -> Result<Option<StoredMessageRecord>, ContractError>;

    /// 读取会话最新消息序号（高水位）
    fn read_high_watermark(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<u64, ContractError>;
}
