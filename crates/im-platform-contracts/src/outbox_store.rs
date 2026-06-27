// Outbox Store Contract - Outbox 事件存储契约
// 支持分布式 outbox 模式，实现可靠的事件投递

#![allow(clippy::should_implement_trait)]

use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};

/// Outbox 事件记录
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutboxEventRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub outbox_id: String, // Snowflake ID
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub event_id: String,
    pub event_type: String,
    pub payload_json: String,
    pub payload_hash: String,
    pub publish_status: OutboxPublishStatus,
    pub attempt_count: u32,
    pub available_at: String,
    pub published_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// 发布状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutboxPublishStatus {
    Pending,
    Published,
    Failed,
}

impl OutboxPublishStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Published => "published",
            Self::Failed => "failed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "published" => Some(Self::Published),
            "failed" => Some(Self::Failed),
            _ => None,
        }
    }
}

/// Outbox 存储契约
///
/// 设计原则：
/// 1. 支持 FOR UPDATE SKIP LOCKED，多 worker 并发安全
/// 2. 幂等投递（event_id 唯一约束）
/// 3. 失败重试与死信处理
pub trait OutboxStore: Send + Sync {
    /// 入队事件
    ///
    /// INSERT INTO im_outbox_events (...)
    /// 唯一约束：uk_im_outbox_events_event (tenant_id, organization_id, event_id)
    fn enqueue(&self, event: OutboxEventRecord) -> Result<(), ContractError>;

    /// 拉取待投递事件（批量）
    ///
    /// 使用 FOR UPDATE SKIP LOCKED 实现多 worker 并发安全：
    ///
    /// SELECT * FROM im_outbox_events
    /// WHERE tenant_id=$1 AND organization_id=$2
    ///   AND publish_status='pending'
    ///   AND available_at <= NOW()
    /// ORDER BY available_at, outbox_id
    /// FOR UPDATE SKIP LOCKED
    /// LIMIT $3
    ///
    /// 返回的事件已被当前连接锁定，其他 worker 无法获取
    fn drain_pending(
        &self,
        tenant_id: &str,
        organization_id: &str,
        batch_size: usize,
    ) -> Result<Vec<OutboxEventRecord>, ContractError>;

    /// 标记已发布
    ///
    /// UPDATE im_outbox_events
    /// SET publish_status='published', published_at=NOW(), updated_at=NOW()
    /// WHERE tenant_id=$1 AND organization_id=$2 AND outbox_id=$3
    fn mark_published(
        &self,
        tenant_id: &str,
        organization_id: &str,
        outbox_id: &str,
    ) -> Result<(), ContractError>;

    /// 标记失败
    ///
    /// UPDATE im_outbox_events
    /// SET publish_status='failed', attempt_count=attempt_count+1, updated_at=NOW()
    /// WHERE tenant_id=$1 AND organization_id=$2 AND outbox_id=$3
    fn mark_failed(
        &self,
        tenant_id: &str,
        organization_id: &str,
        outbox_id: &str,
        reason: &str,
    ) -> Result<(), ContractError>;

    /// 重试失败事件（将 failed 状态重置为 pending）
    fn retry_failed(
        &self,
        tenant_id: &str,
        organization_id: &str,
        outbox_id: &str,
    ) -> Result<(), ContractError>;

    /// 按事件 ID 查询（幂等检查）
    fn read_by_event_id(
        &self,
        tenant_id: &str,
        organization_id: &str,
        event_id: &str,
    ) -> Result<Option<OutboxEventRecord>, ContractError>;

    /// 统计待投递事件数量（监控用）
    fn count_pending(&self, tenant_id: &str, organization_id: &str) -> Result<u64, ContractError>;
}
