//! Redis cache for typing indicators.
//!
//! Key pattern: typing:{tenant_id}:{org_id}:{conversation_id}:{user_id}
//! Type: STRING
//! TTL: 5 seconds (auto-expire)

use redis::AsyncCommands;
use redis::aio::ConnectionManager;

use crate::redis_unavailable;

/// Trait for typing indicator cache operations.
pub trait TypingCache: Send + Sync {
    fn set_typing(
        &self,
        tenant_id: &str,
        org_id: &str,
        conversation_id: &str,
        user_id: &str,
        ttl_seconds: u64,
    ) -> impl std::future::Future<Output = Result<(), im_platform_contracts::ContractError>> + Send;

    fn is_typing(
        &self,
        tenant_id: &str,
        org_id: &str,
        conversation_id: &str,
        user_id: &str,
    ) -> impl std::future::Future<Output = Result<bool, im_platform_contracts::ContractError>> + Send;
}

fn typing_key(tenant_id: &str, org_id: &str, conversation_id: &str, user_id: &str) -> String {
    format!("typing:{tenant_id}:{org_id}:{conversation_id}:{user_id}")
}

/// Redis-backed typing indicator cache.
#[derive(Clone)]
pub struct RedisTypingCache {
    manager: ConnectionManager,
}

impl RedisTypingCache {
    pub fn new(manager: ConnectionManager) -> Self {
        Self { manager }
    }
}

impl TypingCache for RedisTypingCache {
    async fn set_typing(
        &self,
        tenant_id: &str,
        org_id: &str,
        conversation_id: &str,
        user_id: &str,
        ttl_seconds: u64,
    ) -> Result<(), im_platform_contracts::ContractError> {
        let key = typing_key(tenant_id, org_id, conversation_id, user_id);
        let mut conn = self.manager.clone();

        let _: () = conn
            .set_ex(&key, "1", ttl_seconds)
            .await
            .map_err(|e| redis_unavailable("set_typing", e))?;

        Ok(())
    }

    async fn is_typing(
        &self,
        tenant_id: &str,
        org_id: &str,
        conversation_id: &str,
        user_id: &str,
    ) -> Result<bool, im_platform_contracts::ContractError> {
        let key = typing_key(tenant_id, org_id, conversation_id, user_id);
        let mut conn = self.manager.clone();

        let exists: bool = conn
            .exists(&key)
            .await
            .map_err(|e| redis_unavailable("is_typing", e))?;

        Ok(exists)
    }
}
