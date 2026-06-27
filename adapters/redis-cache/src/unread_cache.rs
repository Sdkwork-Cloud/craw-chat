//! Redis cache for unread message counts.
//!
//! Key pattern: unread:{tenant_id}:{org_id}:{user_id}:{conversation_id}
//! Type: STRING (integer)
//! TTL: 86400 seconds (refreshed on read)
//! Write: on message delivery (INCR), on read cursor update (DEL/SET)

use redis::AsyncCommands;
use redis::aio::ConnectionManager;

use crate::redis_unavailable;

/// Trait for unread count cache operations.
pub trait UnreadCache: Send + Sync {
    fn increment_unread(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        conversation_id: &str,
    ) -> impl std::future::Future<Output = Result<i64, im_platform_contracts::ContractError>> + Send;

    fn set_unread(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        conversation_id: &str,
        count: i64,
    ) -> impl std::future::Future<Output = Result<(), im_platform_contracts::ContractError>> + Send;

    fn get_unread(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        conversation_id: &str,
    ) -> impl std::future::Future<Output = Result<i64, im_platform_contracts::ContractError>> + Send;

    fn get_batch_unread(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        conversation_ids: &[String],
    ) -> impl std::future::Future<Output = Result<Vec<i64>, im_platform_contracts::ContractError>> + Send;

    fn clear_unread(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        conversation_id: &str,
    ) -> impl std::future::Future<Output = Result<(), im_platform_contracts::ContractError>> + Send;
}

fn unread_key(tenant_id: &str, org_id: &str, user_id: &str, conversation_id: &str) -> String {
    format!("unread:{tenant_id}:{org_id}:{user_id}:{conversation_id}")
}

/// Redis-backed unread count cache.
#[derive(Clone)]
pub struct RedisUnreadCache {
    manager: ConnectionManager,
    ttl_seconds: u64,
}

impl RedisUnreadCache {
    pub fn new(manager: ConnectionManager, ttl_seconds: u64) -> Self {
        Self {
            manager,
            ttl_seconds,
        }
    }
}

impl UnreadCache for RedisUnreadCache {
    async fn increment_unread(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        conversation_id: &str,
    ) -> Result<i64, im_platform_contracts::ContractError> {
        let key = unread_key(tenant_id, org_id, user_id, conversation_id);
        let mut conn = self.manager.clone();

        let count: i64 = conn
            .incr(&key, 1)
            .await
            .map_err(|e| redis_unavailable("increment_unread", e))?;

        // Refresh TTL
        let _: () = conn
            .expire(&key, self.ttl_seconds as i64)
            .await
            .map_err(|e| redis_unavailable("increment_unread_ttl", e))?;

        Ok(count)
    }

    async fn set_unread(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        conversation_id: &str,
        count: i64,
    ) -> Result<(), im_platform_contracts::ContractError> {
        let key = unread_key(tenant_id, org_id, user_id, conversation_id);
        let mut conn = self.manager.clone();

        let _: () = conn
            .set_ex(&key, count, self.ttl_seconds)
            .await
            .map_err(|e| redis_unavailable("set_unread", e))?;

        Ok(())
    }

    async fn get_unread(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        conversation_id: &str,
    ) -> Result<i64, im_platform_contracts::ContractError> {
        let key = unread_key(tenant_id, org_id, user_id, conversation_id);
        let mut conn = self.manager.clone();

        let count: Option<i64> = conn
            .get(&key)
            .await
            .map_err(|e| redis_unavailable("get_unread", e))?;

        Ok(count.unwrap_or(0))
    }

    async fn get_batch_unread(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        conversation_ids: &[String],
    ) -> Result<Vec<i64>, im_platform_contracts::ContractError> {
        let keys: Vec<String> = conversation_ids
            .iter()
            .map(|cid| unread_key(tenant_id, org_id, user_id, cid))
            .collect();

        let mut conn = self.manager.clone();

        let counts: Vec<Option<i64>> = conn
            .mget(&keys)
            .await
            .map_err(|e| redis_unavailable("get_batch_unread", e))?;

        Ok(counts.into_iter().map(|c| c.unwrap_or(0)).collect())
    }

    async fn clear_unread(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        conversation_id: &str,
    ) -> Result<(), im_platform_contracts::ContractError> {
        let key = unread_key(tenant_id, org_id, user_id, conversation_id);
        let mut conn = self.manager.clone();

        redis::cmd("DEL")
            .arg(&key)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| redis_unavailable("clear_unread", e))?;

        Ok(())
    }
}
