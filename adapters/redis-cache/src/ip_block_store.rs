//! Distributed temporary IP blocks for gateway anomaly enforcement.

use std::net::IpAddr;

use crate::redis_blocking::{RedisBlockingTimeouts, run_bounded_redis_command};

/// Redis-backed temporary IP block store shared across gateway replicas.
#[derive(Clone, Debug)]
pub struct RedisIpBlockStore {
    client: redis::Client,
    key_prefix: String,
    timeouts: RedisBlockingTimeouts,
}

impl RedisIpBlockStore {
    pub fn new(client: redis::Client, key_prefix: impl Into<String>) -> Self {
        Self {
            client,
            key_prefix: key_prefix.into(),
            timeouts: RedisBlockingTimeouts::gateway_rate_limit_from_env(),
        }
    }

    pub fn try_from_url(url: &str, key_prefix: impl Into<String>) -> Option<Self> {
        let trimmed = url.trim();
        if trimmed.is_empty() {
            return None;
        }
        redis::Client::open(trimmed)
            .ok()
            .map(|client| Self::new(client, key_prefix))
    }

    pub fn try_from_gateway_env() -> Option<Self> {
        crate::fixed_window_rate_limit::resolve_gateway_rate_limit_redis_url_from_env()
            .and_then(|url| Self::try_from_url(url.as_str(), "gateway:ip_block:"))
    }

    fn key(&self, client_ip: &IpAddr) -> String {
        format!("{}{client_ip}", self.key_prefix)
    }

    pub fn is_blocked(
        &self,
        client_ip: &IpAddr,
    ) -> Result<bool, im_platform_contracts::ContractError> {
        let key = self.key(client_ip);
        run_bounded_redis_command(
            &self.client,
            self.timeouts,
            "ip_block_exists",
            move |mut connection| async move {
                redis::cmd("EXISTS")
                    .arg(key)
                    .query_async(&mut connection)
                    .await
            },
        )
    }

    pub fn block_for_secs(
        &self,
        client_ip: &IpAddr,
        duration_secs: u64,
    ) -> Result<(), im_platform_contracts::ContractError> {
        if duration_secs == 0 {
            return Ok(());
        }
        let key = self.key(client_ip);
        run_bounded_redis_command(
            &self.client,
            self.timeouts,
            "ip_block_set",
            move |mut connection| async move {
                redis::cmd("SETEX")
                    .arg(key)
                    .arg(duration_secs)
                    .arg("1")
                    .query_async(&mut connection)
                    .await
            },
        )
    }
}
