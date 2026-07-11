// Outbox Store Contract - Outbox 事件存储契约
// 支持分布式 outbox 模式，实现可靠的事件投递

#![allow(clippy::should_implement_trait)]

use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

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

/// A bounded claim on one pending outbox event.
///
/// `lease_expires_at` is also the fencing token used by state transitions. A
/// worker whose lease has expired cannot overwrite a newer worker's result.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutboxEventClaim {
    pub event: OutboxEventRecord,
    pub lease_expires_at: String,
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

    /// 原子领取待投递事件（批量）
    ///
    /// The store filters by aggregate type, selects rows with
    /// `FOR UPDATE SKIP LOCKED`, and moves `available_at` to the lease expiry
    /// in the same database statement. Returning a row without atomically
    /// leasing it is not a valid implementation because an auto-commit query
    /// releases row locks before the worker publishes.
    ///
    /// `lease_expires_at` is a fencing token. `mark_published` and
    /// `mark_failed` must update only a pending row whose current
    /// `available_at` still equals that token.
    fn claim_pending(
        &self,
        tenant_id: &str,
        organization_id: &str,
        aggregate_type: &str,
        batch_size: usize,
        lease_duration: Duration,
    ) -> Result<Vec<OutboxEventClaim>, ContractError>;

    /// 标记已发布
    ///
    /// UPDATE im_outbox_events
    /// SET publish_status='published', published_at=NOW(), updated_at=NOW()
    /// WHERE tenant_id=$1 AND organization_id=$2 AND outbox_id=$3
    ///   AND publish_status='pending' AND available_at=$4
    fn mark_published(&self, claim: &OutboxEventClaim) -> Result<(), ContractError>;

    /// 标记失败
    ///
    /// UPDATE im_outbox_events
    /// SET publish_status='failed', attempt_count=attempt_count+1, updated_at=NOW()
    /// WHERE tenant_id=$1 AND organization_id=$2 AND outbox_id=$3
    ///   AND publish_status='pending' AND available_at=$4
    fn mark_failed(&self, claim: &OutboxEventClaim, reason: &str) -> Result<(), ContractError>;

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

    /// 列出存在待投递事件的租户/组织作用域（relay worker 多租户轮询）
    fn list_pending_scopes(
        &self,
        aggregate_type: &str,
        limit: usize,
    ) -> Result<Vec<(String, String)>, ContractError>;
}
