//! Redis fixed-window rate limiting shared across IM gateway and realtime services.
//!
//! Keys: `{prefix}{bucket_key}` with TTL = window seconds on first increment.

use crate::redis_blocking::{RedisBlockingTimeouts, run_bounded_redis_command};

/// Redis-backed fixed-window counter limiter for horizontally scaled services.
#[derive(Clone, Debug)]
pub struct RedisFixedWindowRateLimiter {
    client: redis::Client,
    key_prefix: String,
    timeouts: RedisBlockingTimeouts,
}

impl RedisFixedWindowRateLimiter {
    pub fn new(client: redis::Client, key_prefix: impl Into<String>) -> Self {
        Self {
            client,
            key_prefix: key_prefix.into(),
            timeouts: RedisBlockingTimeouts::gateway_rate_limit_from_env(),
        }
    }

    /// Build from a Redis URL and key prefix; returns `None` when URL is empty/invalid.
    pub fn try_from_url(url: &str, key_prefix: impl Into<String>) -> Option<Self> {
        let trimmed = url.trim();
        if trimmed.is_empty() {
            return None;
        }
        redis::Client::open(trimmed)
            .ok()
            .map(|client| Self::new(client, key_prefix))
    }

    fn key(&self, bucket_key: &str) -> String {
        format!("{}{bucket_key}", self.key_prefix)
    }

    /// Increment the bucket counter and return whether the request is allowed.
    pub fn allow(
        &self,
        bucket_key: &str,
        max_count: u32,
        window_secs: u64,
    ) -> Result<bool, im_platform_contracts::ContractError> {
        let key = self.key(bucket_key);
        let count: u32 = run_bounded_redis_command(
            &self.client,
            self.timeouts,
            "fixed_window_rate",
            move |mut connection| async move {
                let count: u32 = redis::cmd("INCR")
                    .arg(&key)
                    .query_async(&mut connection)
                    .await?;
                if count == 1 {
                    redis::cmd("EXPIRE")
                        .arg(&key)
                        .arg(window_secs as i64)
                        .query_async::<()>(&mut connection)
                        .await?;
                }
                Ok(count)
            },
        )?;
        Ok(count <= max_count)
    }

    /// Increment the bucket counter and return the current count (for threshold checks).
    pub fn increment(
        &self,
        bucket_key: &str,
        window_secs: u64,
    ) -> Result<u32, im_platform_contracts::ContractError> {
        let key = self.key(bucket_key);
        run_bounded_redis_command(
            &self.client,
            self.timeouts,
            "fixed_window_rate",
            move |mut connection| async move {
                let count: u32 = redis::cmd("INCR")
                    .arg(&key)
                    .query_async(&mut connection)
                    .await?;
                if count == 1 {
                    redis::cmd("EXPIRE")
                        .arg(&key)
                        .arg(window_secs as i64)
                        .query_async::<()>(&mut connection)
                        .await?;
                }
                Ok(count)
            },
        )
    }
}

/// Resolve Redis URL for gateway/session distributed rate limiting.
///
/// Priority: `SDKWORK_IM_GATEWAY_RATE_LIMIT_REDIS_URL` → `SDKWORK_IM_REDIS_URL` when
/// `SDKWORK_IM_REDIS_ENABLED` is truthy.
pub fn resolve_gateway_rate_limit_redis_url_from_env() -> Option<String> {
    if let Ok(override_url) = std::env::var("SDKWORK_IM_GATEWAY_RATE_LIMIT_REDIS_URL") {
        let trimmed = override_url.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_owned());
        }
    }

    let enabled = std::env::var("SDKWORK_IM_REDIS_ENABLED")
        .ok()
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false);
    if !enabled {
        return None;
    }

    std::env::var("SDKWORK_IM_REDIS_URL")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

/// Returns true when Redis-backed rate limiting should deny on backend outage.
pub fn gateway_rate_limit_redis_fail_closed_from_env() -> bool {
    matches!(
        std::env::var("SDKWORK_IM_ENVIRONMENT")
            .ok()
            .as_deref()
            .map(str::trim),
        Some("production") | Some("prod") | Some("staging")
    ) && resolve_gateway_rate_limit_redis_url_from_env().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::thread;
    use std::time::{Duration, Instant};

    #[test]
    fn resolve_gateway_rate_limit_redis_url_prefers_override() {
        let _guard = TestEnvGuard::set(
            "SDKWORK_IM_GATEWAY_RATE_LIMIT_REDIS_URL",
            "redis://override:6379",
        );
        let _enabled = TestEnvGuard::remove("SDKWORK_IM_REDIS_ENABLED");
        assert_eq!(
            resolve_gateway_rate_limit_redis_url_from_env().as_deref(),
            Some("redis://override:6379")
        );
    }

    #[test]
    fn fixed_window_rate_limiter_fails_fast_when_redis_stops_responding() {
        let _connect_timeout = TestEnvGuard::set(
            "SDKWORK_IM_GATEWAY_RATE_LIMIT_REDIS_CONNECT_TIMEOUT_MS",
            "50",
        );
        let _command_timeout = TestEnvGuard::set(
            "SDKWORK_IM_GATEWAY_RATE_LIMIT_REDIS_COMMAND_TIMEOUT_MS",
            "50",
        );
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind local redis stub");
        let address = listener.local_addr().expect("local redis stub addr");
        let handle = thread::spawn(move || {
            let Ok((_socket, _peer)) = listener.accept() else {
                return;
            };
            thread::sleep(Duration::from_millis(400));
        });

        let limiter =
            RedisFixedWindowRateLimiter::try_from_url(&format!("redis://{address}/"), "test:")
                .expect("local redis stub client");

        let started = Instant::now();
        let result = limiter.allow("bucket", 1, 60);

        assert!(result.is_err());
        assert!(
            started.elapsed() < Duration::from_millis(250),
            "redis rate limiter must fail fast instead of leaving API calls pending"
        );
        handle.join().expect("redis stub thread should finish");
    }

    struct TestEnvGuard {
        name: &'static str,
        previous: Option<String>,
    }

    impl TestEnvGuard {
        fn set(name: &'static str, value: &str) -> Self {
            let previous = std::env::var(name).ok();
            unsafe {
                std::env::set_var(name, value);
            }
            Self { name, previous }
        }

        fn remove(name: &'static str) -> Self {
            let previous = std::env::var(name).ok();
            unsafe {
                std::env::remove_var(name);
            }
            Self { name, previous }
        }
    }

    impl Drop for TestEnvGuard {
        fn drop(&mut self) {
            match &self.previous {
                Some(value) => unsafe {
                    std::env::set_var(self.name, value);
                },
                None => unsafe {
                    std::env::remove_var(self.name);
                },
            }
        }
    }
}
