//! Distributed RTC call-signal rate limiting via Redis fixed-window counters.

use crate::fixed_window_rate_limit::RedisFixedWindowRateLimiter;

const KEY_PREFIX: &str = "rtc:signal_rate:";

/// Redis-backed per-sender signal rate limiter shared across call-service instances.
#[derive(Clone)]
pub struct RedisSignalRateLimiter {
    inner: RedisFixedWindowRateLimiter,
}

impl RedisSignalRateLimiter {
    pub fn new(client: redis::Client) -> Self {
        Self {
            inner: RedisFixedWindowRateLimiter::new(client, KEY_PREFIX),
        }
    }

    /// Build from a Redis URL; returns `None` when the URL is empty or invalid.
    pub fn try_from_url(url: &str) -> Option<Self> {
        RedisFixedWindowRateLimiter::try_from_url(url, KEY_PREFIX).map(|inner| Self { inner })
    }

    /// Increment the sender counter and return whether the signal is allowed.
    pub fn allow_signal(
        &self,
        sender_key: &str,
        max_signals: u32,
        window_secs: u64,
    ) -> Result<bool, im_platform_contracts::ContractError> {
        self.inner.allow(sender_key, max_signals, window_secs)
    }
}
