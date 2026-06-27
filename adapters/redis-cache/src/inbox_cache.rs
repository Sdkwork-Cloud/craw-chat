//! Redis cache for inbox/conversation list.
//!
//! Key pattern: inbox:{tenant_id}:{org_id}:{user_id}
//! Type: SORTED SET (score = last_activity_at epoch)
//! Members: conversation_id
//! TTL: 3600 seconds

use redis::AsyncCommands;
use redis::aio::ConnectionManager;

use crate::redis_unavailable;

/// Trait for inbox cache operations.
pub trait InboxCache: Send + Sync {
    fn update_conversation_activity(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        conversation_id: &str,
        activity_timestamp: f64,
    ) -> impl std::future::Future<Output = Result<(), im_platform_contracts::ContractError>> + Send;

    fn get_top_conversations(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        count: usize,
    ) -> impl std::future::Future<Output = Result<Vec<String>, im_platform_contracts::ContractError>>
    + Send;

    fn remove_conversation(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        conversation_id: &str,
    ) -> impl std::future::Future<Output = Result<(), im_platform_contracts::ContractError>> + Send;

    fn clear_inbox(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
    ) -> impl std::future::Future<Output = Result<(), im_platform_contracts::ContractError>> + Send;
}

fn inbox_key(tenant_id: &str, org_id: &str, user_id: &str) -> String {
    format!("inbox:{tenant_id}:{org_id}:{user_id}")
}

/// Redis-backed inbox cache.
#[derive(Clone)]
pub struct RedisInboxCache {
    manager: ConnectionManager,
    ttl_seconds: u64,
}

impl RedisInboxCache {
    pub fn new(manager: ConnectionManager, ttl_seconds: u64) -> Self {
        Self {
            manager,
            ttl_seconds,
        }
    }
}

impl InboxCache for RedisInboxCache {
    async fn update_conversation_activity(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        conversation_id: &str,
        activity_timestamp: f64,
    ) -> Result<(), im_platform_contracts::ContractError> {
        let key = inbox_key(tenant_id, org_id, user_id);
        let mut conn = self.manager.clone();

        // ZADD updates score if member exists, adds if not
        let _: () = conn
            .zadd(&key, conversation_id, activity_timestamp)
            .await
            .map_err(|e| redis_unavailable("update_conversation_activity", e))?;

        // Refresh TTL
        let _: () = conn
            .expire(&key, self.ttl_seconds as i64)
            .await
            .map_err(|e| redis_unavailable("update_conversation_activity_ttl", e))?;

        Ok(())
    }

    async fn get_top_conversations(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        count: usize,
    ) -> Result<Vec<String>, im_platform_contracts::ContractError> {
        let key = inbox_key(tenant_id, org_id, user_id);
        let mut conn = self.manager.clone();

        // ZREVRANGE returns members sorted by score descending (most recent first)
        let conversations: Vec<String> = conn
            .zrevrange(&key, 0, count as isize - 1)
            .await
            .map_err(|e| redis_unavailable("get_top_conversations", e))?;

        Ok(conversations)
    }

    async fn remove_conversation(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        conversation_id: &str,
    ) -> Result<(), im_platform_contracts::ContractError> {
        let key = inbox_key(tenant_id, org_id, user_id);
        let mut conn = self.manager.clone();

        let _: () = conn
            .zrem(&key, conversation_id)
            .await
            .map_err(|e| redis_unavailable("remove_conversation", e))?;

        Ok(())
    }

    async fn clear_inbox(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
    ) -> Result<(), im_platform_contracts::ContractError> {
        let key = inbox_key(tenant_id, org_id, user_id);
        let mut conn = self.manager.clone();

        redis::cmd("DEL")
            .arg(&key)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| redis_unavailable("clear_inbox", e))?;

        Ok(())
    }
}
