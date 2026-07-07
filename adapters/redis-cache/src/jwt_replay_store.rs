//! Distributed JWT `jti` replay protection shared across IM service replicas.

use crate::redis_blocking::{RedisBlockingTimeouts, run_bounded_redis_command};

const DEFAULT_KEY_PREFIX: &str = "im:jwt:replay:";

/// Redis-backed JWT replay cache for multi-instance deployments.
#[derive(Clone, Debug)]
pub struct RedisJwtReplayStore {
    client: redis::Client,
    key_prefix: String,
    timeouts: RedisBlockingTimeouts,
}

impl RedisJwtReplayStore {
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

    pub fn try_from_env() -> Option<Self> {
        std::env::var("SDKWORK_IM_JWT_REPLAY_REDIS_URL")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .and_then(|url| Self::try_from_url(url.as_str(), DEFAULT_KEY_PREFIX))
            .or_else(|| {
                crate::fixed_window_rate_limit::resolve_gateway_rate_limit_redis_url_from_env()
                    .and_then(|url| Self::try_from_url(url.as_str(), DEFAULT_KEY_PREFIX))
            })
    }

    fn key(&self, jti: &str) -> String {
        format!("{}{jti}", self.key_prefix)
    }

    /// Returns `true` when the `jti` was claimed for the first time within `ttl_secs`.
    pub fn try_claim_jti(
        &self,
        jti: &str,
        ttl_secs: u64,
    ) -> Result<bool, im_platform_contracts::ContractError> {
        let key = self.key(jti);
        run_bounded_redis_command(
            &self.client,
            self.timeouts,
            "jwt_replay_set_nx",
            move |mut connection| async move {
                redis::cmd("SET")
                    .arg(key)
                    .arg("1")
                    .arg("NX")
                    .arg("EX")
                    .arg(ttl_secs.max(1))
                    .query_async(&mut connection)
                    .await
            },
        )
    }
}
