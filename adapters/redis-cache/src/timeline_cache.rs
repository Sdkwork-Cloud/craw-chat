//! Redis cache for recent messages timeline.
//!
//! Key pattern: timeline:{tenant_id}:{org_id}:{conversation_id}
//! Type: LIST (capped at N entries)
//! Value: JSON-serialized message summaries
//! TTL: 3600 seconds

use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};

use crate::redis_unavailable;

/// Message summary cached in Redis.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CachedMessageSummary {
    pub message_id: i64,
    pub message_seq: i64,
    pub sender_principal_kind: String,
    pub sender_principal_id: String,
    pub message_type: String,
    pub created_at: String,
    pub summary: Option<String>,
}

/// Trait for timeline cache operations.
pub trait TimelineCache: Send + Sync {
    fn push_message(
        &self,
        tenant_id: &str,
        org_id: &str,
        conversation_id: &str,
        message: &CachedMessageSummary,
        max_entries: usize,
    ) -> impl std::future::Future<Output = Result<(), im_platform_contracts::ContractError>> + Send;

    fn get_recent_messages(
        &self,
        tenant_id: &str,
        org_id: &str,
        conversation_id: &str,
        count: usize,
    ) -> impl std::future::Future<
        Output = Result<Vec<CachedMessageSummary>, im_platform_contracts::ContractError>,
    > + Send;

    fn clear_timeline(
        &self,
        tenant_id: &str,
        org_id: &str,
        conversation_id: &str,
    ) -> impl std::future::Future<Output = Result<(), im_platform_contracts::ContractError>> + Send;
}

fn timeline_key(tenant_id: &str, org_id: &str, conversation_id: &str) -> String {
    format!("timeline:{tenant_id}:{org_id}:{conversation_id}")
}

/// Redis-backed timeline cache.
#[derive(Clone)]
pub struct RedisTimelineCache {
    manager: ConnectionManager,
    ttl_seconds: u64,
}

impl RedisTimelineCache {
    pub fn new(manager: ConnectionManager, ttl_seconds: u64) -> Self {
        Self {
            manager,
            ttl_seconds,
        }
    }
}

impl TimelineCache for RedisTimelineCache {
    async fn push_message(
        &self,
        tenant_id: &str,
        org_id: &str,
        conversation_id: &str,
        message: &CachedMessageSummary,
        max_entries: usize,
    ) -> Result<(), im_platform_contracts::ContractError> {
        let key = timeline_key(tenant_id, org_id, conversation_id);
        let mut conn = self.manager.clone();

        let data = serde_json::to_string(message).map_err(|e| {
            im_platform_contracts::ContractError::Unavailable(format!(
                "serialize message failed: {e}"
            ))
        })?;

        // LPUSH to add to front, LTRIM to cap at max_entries
        redis::cmd("LPUSH")
            .arg(&key)
            .arg(&data)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| redis_unavailable("push_message", e))?;

        redis::cmd("LTRIM")
            .arg(&key)
            .arg(0)
            .arg(max_entries - 1)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| redis_unavailable("push_message_trim", e))?;

        // Refresh TTL
        let _: () = conn
            .expire(&key, self.ttl_seconds as i64)
            .await
            .map_err(|e| redis_unavailable("push_message_ttl", e))?;

        Ok(())
    }

    async fn get_recent_messages(
        &self,
        tenant_id: &str,
        org_id: &str,
        conversation_id: &str,
        count: usize,
    ) -> Result<Vec<CachedMessageSummary>, im_platform_contracts::ContractError> {
        let key = timeline_key(tenant_id, org_id, conversation_id);
        let mut conn = self.manager.clone();

        let data: Vec<String> = conn
            .lrange(&key, 0, count as isize - 1)
            .await
            .map_err(|e| redis_unavailable("get_recent_messages", e))?;

        let mut messages = Vec::with_capacity(data.len());
        for item in data {
            let msg: CachedMessageSummary = serde_json::from_str(&item).map_err(|e| {
                im_platform_contracts::ContractError::Unavailable(format!(
                    "deserialize message failed: {e}"
                ))
            })?;
            messages.push(msg);
        }

        Ok(messages)
    }

    async fn clear_timeline(
        &self,
        tenant_id: &str,
        org_id: &str,
        conversation_id: &str,
    ) -> Result<(), im_platform_contracts::ContractError> {
        let key = timeline_key(tenant_id, org_id, conversation_id);
        let mut conn = self.manager.clone();

        redis::cmd("DEL")
            .arg(&key)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| redis_unavailable("clear_timeline", e))?;

        Ok(())
    }
}
