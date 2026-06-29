//! Redis-backed state store for RTC call sessions.
//!
//! This adapter implements [`StateStore`] from `im-domain-core::rtc` using
//! Redis as the backing store. It is the hot-path, low-latency state cache
//! that complements the durable PostgreSQL store in
//! `adapters/postgres-rtc-state`.
//!
//! ## Storage model
//!
//! Each RTC session is stored as two keys under the
//! `rtc:state:{tenant_id}:{rtc_session_id}` and
//! `rtc:epoch:{tenant_id}:{rtc_session_id}` patterns:
//!
//! - **state key**: STRING holding the serialized [`RtcStateRecord`] JSON.
//! - **epoch key**: STRING holding the current epoch as a decimal integer,
//!   used as the CAS guard for `save_state`.
//!
//! Both keys share the same TTL (default 24 hours) so the cache entry is
//! reaped automatically after the longest supported call plus the
//! post-call audit window.
//!
//! ## Epoch fencing
//!
//! `save_state` executes a single Lua script atomically. The script reads
//! the existing epoch, rejects stale writes (incoming epoch strictly less
//! than the persisted epoch), and otherwise updates both keys in the same
//! Redis round trip. Because Redis executes Lua scripts atomically, this
//! is race-free even under concurrent writers.
//!
//! ## Threading bridge
//!
//! [`StateStore`] is a synchronous trait. Redis I/O uses the async
//! `redis::aio::ConnectionManager` (the same `RedisCachePool` used by the
//! rest of this crate) and is bridged onto a blocking scope via
//! `tokio::task::block_in_place` + `Handle::block_on`, matching the
//! pattern in `adapters/postgres-rtc-state` and `adapters/postgres-journal`.
//!
//! ## Spec alignment
//!
//! - DATABASE_SPEC §5.1 (`event_log`) — RTC state is hot cache, not the
//!   source of truth; PostgreSQL remains the durable authority.
//! - SECURITY_SPEC — multi-tenant isolation is enforced by key prefix.
//! - RUST_CODE_SPEC — fail-closed error handling via `RtcContractError`.

use std::sync::Arc;

use im_domain_core::rtc::{RtcStateRecord, StateStore};
use redis::aio::ConnectionManager;
use sdkwork_communication_rtc_service::RtcContractError;
use tokio::runtime::Handle;

/// Default TTL for RTC state keys: 24 hours.
///
/// Covers the longest supported call (a few hours) plus a post-call audit
/// window so recently-ended sessions remain queryable from the hot cache.
const DEFAULT_STATE_TTL_SECONDS: u64 = 86_400;

/// Lua script for atomic epoch-fenced save.
///
/// KEYS[1] = epoch key
/// KEYS[2] = state key
/// ARGV[1] = incoming epoch (decimal string)
/// ARGV[2] = state JSON
/// ARGV[3] = TTL seconds
///
/// Returns the string "OK" on success. On stale epoch, returns
/// `redis.error_reply("STALE_EPOCH:" .. existing_epoch)` so the Rust
/// caller can surface the conflict via [`RtcContractError::Conflict`].
const SAVE_STATE_LUA: &str = r#"
local existing = redis.call('GET', KEYS[1])
if existing then
    local existing_epoch = tonumber(existing)
    local incoming_epoch = tonumber(ARGV[1])
    if incoming_epoch < existing_epoch then
        return redis.error_reply('STALE_EPOCH:' .. tostring(existing_epoch))
    end
end
redis.call('SET', KEYS[1], ARGV[1], 'EX', ARGV[3])
redis.call('SET', KEYS[2], ARGV[2], 'EX', ARGV[3])
return 'OK'
"#;

/// Configuration for the Redis RTC state store.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RedisRtcStateConfig {
    redis_url: String,
    state_ttl_seconds: u64,
}

impl RedisRtcStateConfig {
    pub fn new(redis_url: impl Into<String>) -> Self {
        Self {
            redis_url: redis_url.into(),
            state_ttl_seconds: DEFAULT_STATE_TTL_SECONDS,
        }
    }

    /// Override the default 24-hour state TTL.
    ///
    /// Values below 60 seconds are clamped to 60 to avoid prematurely
    /// evicting an in-flight call's state.
    pub fn with_state_ttl_seconds(mut self, ttl_seconds: u64) -> Self {
        self.state_ttl_seconds = ttl_seconds.max(60);
        self
    }

    pub fn redis_url(&self) -> &str {
        self.redis_url.as_str()
    }

    pub fn state_ttl_seconds(&self) -> u64 {
        self.state_ttl_seconds
    }
}

/// Redis-backed implementation of [`StateStore`].
///
/// Stores the full [`RtcStateRecord`] as JSON in a STRING key, with a
/// separate epoch key for atomic CAS fencing. Both keys share the same
/// TTL so the cache entry is reaped automatically after the configured
/// `state_ttl_seconds` window.
#[derive(Clone)]
pub struct RedisRtcStateStore {
    manager: ConnectionManager,
    state_ttl_seconds: u64,
    /// Pre-compiled Lua script; `redis::Script` is cheap to invoke
    /// repeatedly via `invoke_async`.
    save_script: redis::Script,
}

impl RedisRtcStateStore {
    pub fn new(manager: ConnectionManager, state_ttl_seconds: u64) -> Self {
        Self {
            manager,
            state_ttl_seconds: state_ttl_seconds.max(60),
            save_script: redis::Script::new(SAVE_STATE_LUA),
        }
    }

    /// Connect to Redis and build the store.
    ///
    /// The `ConnectionManager` is created asynchronously. This helper
    /// handles both contexts:
    /// - Inside a tokio runtime: uses `block_in_place` + `Handle::block_on`
    /// - Outside a runtime: creates a temporary runtime via
    ///   `tokio::runtime::Runtime::new()` to establish the connection
    pub fn from_config(config: &RedisRtcStateConfig) -> Result<Self, RtcContractError> {
        let client = redis::Client::open(config.redis_url.as_str()).map_err(|err| {
            RtcContractError::Unavailable(format!("invalid redis_url: {err}"))
        })?;
        let manager = match Handle::try_current() {
            Ok(handle) => {
                // Inside a runtime: use block_in_place to avoid deadlock.
                tokio::task::block_in_place(|| {
                    handle.block_on(ConnectionManager::new(client))
                })
            }
            Err(_) => {
                // Outside a runtime: create a temporary one-shot runtime.
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .map_err(|err| {
                        RtcContractError::Unavailable(format!(
                            "redis runtime build failed: {err}"
                        ))
                    })?;
                rt.block_on(ConnectionManager::new(client))
            }
        }
        .map_err(|err| {
            RtcContractError::Unavailable(format!("redis connection manager failed: {err}"))
        })?;
        Ok(Self::new(manager, config.state_ttl_seconds))
    }

    /// Build the store from an existing [`ConnectionManager`], suitable
    /// for use inside a tokio runtime.
    pub fn from_manager(
        manager: ConnectionManager,
        state_ttl_seconds: u64,
    ) -> Self {
        Self::new(manager, state_ttl_seconds)
    }

    pub fn state_ttl_seconds(&self) -> u64 {
        self.state_ttl_seconds
    }

    /// Bridge a sync trait call onto the async Redis runtime.
    ///
    /// `block_in_place` moves the current worker thread into the blocking
    /// pool so `block_on` will not deadlock the multi-threaded runtime
    /// (the calls-service uses `rt-multi-thread`).
    fn run_blocking_async<F, Fut, T>(&self, f: F) -> Result<T, RtcContractError>
    where
        F: FnOnce(ConnectionManager) -> Fut + Send,
        Fut: std::future::Future<Output = Result<T, RtcContractError>> + Send,
        T: Send,
    {
        let manager = self.manager.clone();
        let handle = Handle::current();
        tokio::task::block_in_place(|| handle.block_on(f(manager)))
    }

    fn epoch_key(tenant_id: &str, rtc_session_id: &str) -> String {
        format!("rtc:epoch:{tenant_id}:{rtc_session_id}")
    }

    fn state_key(tenant_id: &str, rtc_session_id: &str) -> String {
        format!("rtc:state:{tenant_id}:{rtc_session_id}")
    }
}

impl StateStore for RedisRtcStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<Option<RtcStateRecord>, RtcContractError> {
        let tenant_id = tenant_id.to_string();
        let rtc_session_id = rtc_session_id.to_string();
        self.run_blocking_async(move |mut conn| async move {
            let key = Self::state_key(&tenant_id, &rtc_session_id);
            let payload: Option<String> = redis::cmd("GET")
                .arg(&key)
                .query_async(&mut conn)
                .await
                .map_err(|e| {
                    RtcContractError::Unavailable(format!("rtc load_state failed: {e}"))
                })?;
            match payload {
                Some(json) => {
                    let record: RtcStateRecord = serde_json::from_str(&json).map_err(|err| {
                        RtcContractError::Unavailable(format!(
                            "load_state deserialize failed: {err}"
                        ))
                    })?;
                    Ok(Some(record))
                }
                None => Ok(None),
            }
        })
    }

    fn save_state(&self, record: RtcStateRecord) -> Result<(), RtcContractError> {
        let state_ttl = self.state_ttl_seconds;
        let save_script = self.save_script.clone();
        self.run_blocking_async(move |mut conn| async move {
            let epoch_key = Self::epoch_key(&record.tenant_id, &record.rtc_session_id);
            let state_key = Self::state_key(&record.tenant_id, &record.rtc_session_id);
            let payload_json = serde_json::to_string(&record).map_err(|err| {
                RtcContractError::Unavailable(format!("save_state serialize failed: {err}"))
            })?;
            let incoming_epoch = record.session.epoch.to_string();
            let ttl_str = state_ttl.to_string();

            // Atomic Lua execution: stale-epoch rejection + dual-key write
            // happen in a single Redis round trip, race-free under
            // concurrent writers.
            let result: String = save_script
                .key(epoch_key)
                .key(state_key)
                .arg(incoming_epoch)
                .arg(payload_json)
                .arg(ttl_str)
                .invoke_async(&mut conn)
                .await
                .map_err(|err| {
                    let msg = err.to_string();
                    if let Some(existing) = msg.strip_prefix("STALE_EPOCH:") {
                        return RtcContractError::Conflict(format!(
                            "stale epoch rejected: existing={existing} incoming={}",
                            record.session.epoch
                        ));
                    }
                    RtcContractError::Unavailable(format!("save_state lua failed: {err}"))
                })?;
            debug_assert_eq!(result, "OK", "SAVE_STATE_LUA must return OK on success");
            Ok(())
        })
    }

    fn clear_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<bool, RtcContractError> {
        let tenant_id = tenant_id.to_string();
        let rtc_session_id = rtc_session_id.to_string();
        self.run_blocking_async(move |mut conn| async move {
            let epoch_key = Self::epoch_key(&tenant_id, &rtc_session_id);
            let state_key = Self::state_key(&tenant_id, &rtc_session_id);
            // Delete both keys in a single round trip; `DEL` returns the
            // count of keys actually removed.
            let removed: usize = redis::cmd("DEL")
                .arg(&epoch_key)
                .arg(&state_key)
                .query_async(&mut conn)
                .await
                .map_err(|e| {
                    RtcContractError::Unavailable(format!("rtc clear_state failed: {e}"))
                })?;
            Ok(removed > 0)
        })
    }
}

/// Build a [`RedisRtcStateStore`] from a Redis URL.
///
/// Convenience wrapper that connects and constructs the store with the
/// default TTL. Must be called outside a tokio runtime (use
/// [`RedisRtcStateStore::from_manager`] inside a runtime).
pub fn build_redis_rtc_state_store(
    redis_url: &str,
) -> Result<RedisRtcStateStore, RtcContractError> {
    let config = RedisRtcStateConfig::new(redis_url);
    RedisRtcStateStore::from_config(&config)
}

/// Build a [`RedisRtcStateStore`] wrapped in an [`Arc`] for shared use,
/// returning `None` when the Redis URL is empty (Postgres-only mode).
///
/// Production deployments MAY provide a Redis URL for hot-path state
/// caching; when absent, the service falls back to the durable Postgres
/// store or in-memory store as configured in `build_default_app`.
///
/// Must be called from within a tokio runtime context (the connection
/// manager is established asynchronously via `block_in_place`).
pub fn build_redis_rtc_state_store_optional(
    redis_url: Option<&str>,
) -> Option<Arc<RedisRtcStateStore>> {
    let url = redis_url?.trim();
    if url.is_empty() {
        return None;
    }
    match build_redis_rtc_state_store(url) {
        Ok(store) => {
            tracing::info!("RedisRtcStateStore connected successfully");
            Some(Arc::new(store))
        }
        Err(err) => {
            tracing::warn!(
                error = %format!("{err:?}"),
                "RedisRtcStateStore connection failed; falling back to Postgres/in-memory store"
            );
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_validates_redis_url() {
        let config = RedisRtcStateConfig::new("redis://localhost:6379");
        assert_eq!(config.redis_url(), "redis://localhost:6379");
        assert_eq!(config.state_ttl_seconds, DEFAULT_STATE_TTL_SECONDS);
    }

    #[test]
    fn config_with_state_ttl_clamps_to_minimum_60_seconds() {
        let config = RedisRtcStateConfig::new("redis://localhost:6379").with_state_ttl_seconds(0);
        assert_eq!(config.state_ttl_seconds, 60);
    }

    #[test]
    fn build_optional_returns_none_for_empty_url() {
        assert!(build_redis_rtc_state_store_optional(None).is_none());
        assert!(build_redis_rtc_state_store_optional(Some("")).is_none());
        assert!(build_redis_rtc_state_store_optional(Some("   ")).is_none());
    }

    #[test]
    fn key_patterns_are_tenant_scoped() {
        assert_eq!(
            RedisRtcStateStore::epoch_key("t1", "s1"),
            "rtc:epoch:t1:s1"
        );
        assert_eq!(
            RedisRtcStateStore::state_key("t1", "s1"),
            "rtc:state:t1:s1"
        );
    }
}
