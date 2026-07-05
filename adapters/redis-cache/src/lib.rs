//! Redis cache adapter for hot data caching.
//!
//! Provides caching for presence, unread counts, recent messages,
//! conversation lists, typing indicators, and session state.

pub mod fixed_window_rate_limit;
pub mod ip_block_store;
pub mod route_store;
pub mod cluster_bus;
pub mod config;
pub mod inbox_cache;
pub mod presence_cache;
pub mod realtime_checkpoint_store;
pub mod realtime_event_store;
pub mod rtc_state_store;
pub mod seq_allocator;
pub mod signal_rate_limit;
pub mod session_cache;
pub mod timeline_cache;
pub mod typing_cache;
pub mod unread_cache;

pub use fixed_window_rate_limit::{
    RedisFixedWindowRateLimiter, gateway_rate_limit_redis_fail_closed_from_env,
    resolve_gateway_rate_limit_redis_url_from_env,
};
pub use ip_block_store::RedisIpBlockStore;
pub use route_store::RedisBackedRouteStore;
pub use cluster_bus::{ClusterRouteEvent, RedisClusterBus};
pub use config::RedisCacheConfig;
pub use realtime_checkpoint_store::RedisRealtimeCheckpointStore;
pub use realtime_event_store::RedisRealtimeEventWindowStore;
pub use rtc_state_store::{RedisRtcStateConfig, RedisRtcStateStore};
pub use seq_allocator::RedisSeqAllocator;
pub use signal_rate_limit::RedisSignalRateLimiter;

use redis::RedisResult;
use redis::aio::ConnectionManager;

/// Shared Redis connection manager wrapper.
#[derive(Clone)]
pub struct RedisCachePool {
    manager: ConnectionManager,
}

impl RedisCachePool {
    pub async fn new(url: &str) -> RedisResult<Self> {
        let client = redis::Client::open(url)?;
        let manager = ConnectionManager::new(client).await?;
        Ok(Self { manager })
    }

    pub fn inner(&self) -> &ConnectionManager {
        &self.manager
    }
}

/// Map a Redis error to ContractError.
pub(crate) fn redis_unavailable(
    operation: &str,
    error: redis::RedisError,
) -> im_platform_contracts::ContractError {
    im_platform_contracts::ContractError::Unavailable(format!("redis {operation} failed: {error}"))
}
