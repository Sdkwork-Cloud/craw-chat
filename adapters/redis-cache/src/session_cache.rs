//! Redis cache for session/device registry.
//!
//! Key pattern: session:{tenant_id}:{org_id}:{device_id}
//! Type: HASH
//! Fields: principal_id, principal_kind, connection_kind, node_id, connected_at
//! TTL: 86400 seconds

use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};

use crate::redis_unavailable;

/// Session data cached in Redis.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CachedSession {
    pub principal_id: String,
    pub principal_kind: String,
    pub connection_kind: String,
    pub node_id: String,
    pub connected_at: String,
}

/// Trait for session cache operations.
pub trait SessionCache: Send + Sync {
    fn set_session(
        &self,
        tenant_id: &str,
        org_id: &str,
        device_id: &str,
        session: &CachedSession,
        ttl_seconds: u64,
    ) -> impl std::future::Future<Output = Result<(), im_platform_contracts::ContractError>> + Send;

    fn get_session(
        &self,
        tenant_id: &str,
        org_id: &str,
        device_id: &str,
    ) -> impl std::future::Future<
        Output = Result<Option<CachedSession>, im_platform_contracts::ContractError>,
    > + Send;

    fn delete_session(
        &self,
        tenant_id: &str,
        org_id: &str,
        device_id: &str,
    ) -> impl std::future::Future<Output = Result<(), im_platform_contracts::ContractError>> + Send;
}

fn session_key(tenant_id: &str, org_id: &str, device_id: &str) -> String {
    format!("session:{tenant_id}:{org_id}:{device_id}")
}

/// Redis-backed session cache.
#[derive(Clone)]
pub struct RedisSessionCache {
    manager: ConnectionManager,
}

impl RedisSessionCache {
    pub fn new(manager: ConnectionManager) -> Self {
        Self { manager }
    }
}

impl SessionCache for RedisSessionCache {
    async fn set_session(
        &self,
        tenant_id: &str,
        org_id: &str,
        device_id: &str,
        session: &CachedSession,
        ttl_seconds: u64,
    ) -> Result<(), im_platform_contracts::ContractError> {
        let key = session_key(tenant_id, org_id, device_id);
        let mut conn = self.manager.clone();

        let data = serde_json::to_string(session).map_err(|e| {
            im_platform_contracts::ContractError::Unavailable(format!(
                "serialize session failed: {e}"
            ))
        })?;

        redis::cmd("SET")
            .arg(&key)
            .arg(&data)
            .arg("EX")
            .arg(ttl_seconds)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| redis_unavailable("set_session", e))?;

        Ok(())
    }

    async fn get_session(
        &self,
        tenant_id: &str,
        org_id: &str,
        device_id: &str,
    ) -> Result<Option<CachedSession>, im_platform_contracts::ContractError> {
        let key = session_key(tenant_id, org_id, device_id);
        let mut conn = self.manager.clone();

        let data: Option<String> = conn
            .get(&key)
            .await
            .map_err(|e| redis_unavailable("get_session", e))?;

        match data {
            Some(json) => {
                let session: CachedSession = serde_json::from_str(&json).map_err(|e| {
                    im_platform_contracts::ContractError::Unavailable(format!(
                        "deserialize session failed: {e}"
                    ))
                })?;
                Ok(Some(session))
            }
            None => Ok(None),
        }
    }

    async fn delete_session(
        &self,
        tenant_id: &str,
        org_id: &str,
        device_id: &str,
    ) -> Result<(), im_platform_contracts::ContractError> {
        let key = session_key(tenant_id, org_id, device_id);
        let mut conn = self.manager.clone();

        redis::cmd("DEL")
            .arg(&key)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| redis_unavailable("delete_session", e))?;

        Ok(())
    }
}
