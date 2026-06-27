//! Configuration for the Redis cache adapter.

use redis::RedisResult;

use crate::RedisCachePool;

/// Configuration for connecting to Redis.
#[derive(Clone, Debug)]
pub struct RedisCacheConfig {
    url: String,
}

impl RedisCacheConfig {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// Create a connection pool from this configuration.
    pub async fn connect_pool(&self) -> RedisResult<RedisCachePool> {
        RedisCachePool::new(&self.url).await
    }
}
