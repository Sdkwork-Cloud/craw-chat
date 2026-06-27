//! Redis cache for presence/online status.
//!
//! Key pattern: presence:{tenant_id}:{org_id}:{principal_kind}:{principal_id}
//! Type: HASH
//! Fields: status, device_id, last_seen_at, session_id, custom_status
//! TTL: 300 seconds (auto-expire stale presence)

use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};

use crate::redis_unavailable;

/// Presence data cached in Redis.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CachedPresence {
    pub status: String,
    pub device_id: String,
    pub last_seen_at: String,
    pub session_id: Option<String>,
    pub custom_status: Option<String>,
}

/// Trait for presence cache operations.
pub trait PresenceCache: Send + Sync {
    fn set_presence(
        &self,
        tenant_id: &str,
        org_id: &str,
        principal_kind: &str,
        principal_id: &str,
        presence: &CachedPresence,
        ttl_seconds: u64,
    ) -> impl std::future::Future<Output = Result<(), im_platform_contracts::ContractError>> + Send;

    fn get_presence(
        &self,
        tenant_id: &str,
        org_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> impl std::future::Future<
        Output = Result<Option<CachedPresence>, im_platform_contracts::ContractError>,
    > + Send;

    fn delete_presence(
        &self,
        tenant_id: &str,
        org_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> impl std::future::Future<Output = Result<(), im_platform_contracts::ContractError>> + Send;

    fn get_batch_presence(
        &self,
        tenant_id: &str,
        org_id: &str,
        principal_kind: &str,
        principal_ids: &[String],
    ) -> impl std::future::Future<
        Output = Result<Vec<Option<CachedPresence>>, im_platform_contracts::ContractError>,
    > + Send;
}

fn presence_key(tenant_id: &str, org_id: &str, principal_kind: &str, principal_id: &str) -> String {
    format!("presence:{tenant_id}:{org_id}:{principal_kind}:{principal_id}")
}

/// Redis-backed presence cache.
#[derive(Clone)]
pub struct RedisPresenceCache {
    manager: ConnectionManager,
}

impl RedisPresenceCache {
    pub fn new(manager: ConnectionManager) -> Self {
        Self { manager }
    }
}

impl PresenceCache for RedisPresenceCache {
    async fn set_presence(
        &self,
        tenant_id: &str,
        org_id: &str,
        principal_kind: &str,
        principal_id: &str,
        presence: &CachedPresence,
        ttl_seconds: u64,
    ) -> Result<(), im_platform_contracts::ContractError> {
        let key = presence_key(tenant_id, org_id, principal_kind, principal_id);
        let mut conn = self.manager.clone();
        let data = serde_json::to_string(presence).map_err(|e| {
            im_platform_contracts::ContractError::Unavailable(format!(
                "serialize presence failed: {e}"
            ))
        })?;

        // Use SET with EX for TTL
        redis::cmd("SET")
            .arg(&key)
            .arg(&data)
            .arg("EX")
            .arg(ttl_seconds)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| redis_unavailable("set_presence", e))?;

        Ok(())
    }

    async fn get_presence(
        &self,
        tenant_id: &str,
        org_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Result<Option<CachedPresence>, im_platform_contracts::ContractError> {
        let key = presence_key(tenant_id, org_id, principal_kind, principal_id);
        let mut conn = self.manager.clone();

        let data: Option<String> = conn
            .get(&key)
            .await
            .map_err(|e| redis_unavailable("get_presence", e))?;

        match data {
            Some(json) => {
                let presence: CachedPresence = serde_json::from_str(&json).map_err(|e| {
                    im_platform_contracts::ContractError::Unavailable(format!(
                        "deserialize presence failed: {e}"
                    ))
                })?;
                Ok(Some(presence))
            }
            None => Ok(None),
        }
    }

    async fn delete_presence(
        &self,
        tenant_id: &str,
        org_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Result<(), im_platform_contracts::ContractError> {
        let key = presence_key(tenant_id, org_id, principal_kind, principal_id);
        let mut conn = self.manager.clone();

        redis::cmd("DEL")
            .arg(&key)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| redis_unavailable("delete_presence", e))?;

        Ok(())
    }

    async fn get_batch_presence(
        &self,
        tenant_id: &str,
        org_id: &str,
        principal_kind: &str,
        principal_ids: &[String],
    ) -> Result<Vec<Option<CachedPresence>>, im_platform_contracts::ContractError> {
        let keys: Vec<String> = principal_ids
            .iter()
            .map(|pid| presence_key(tenant_id, org_id, principal_kind, pid))
            .collect();

        let mut conn = self.manager.clone();

        let data: Vec<Option<String>> = conn
            .mget(&keys)
            .await
            .map_err(|e| redis_unavailable("get_batch_presence", e))?;

        let mut results = Vec::with_capacity(data.len());
        for item in data {
            match item {
                Some(json) => {
                    let presence: CachedPresence = serde_json::from_str(&json).map_err(|e| {
                        im_platform_contracts::ContractError::Unavailable(format!(
                            "deserialize presence failed: {e}"
                        ))
                    })?;
                    results.push(Some(presence));
                }
                None => results.push(None),
            }
        }

        Ok(results)
    }
}
