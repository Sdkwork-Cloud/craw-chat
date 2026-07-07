//! Per-principal WebSocket business-frame rate limiting after upgrade.
//!
//! Uses optional Redis fixed-window counters (`SDKWORK_IM_REDIS_URL` /
//! `SDKWORK_IM_GATEWAY_RATE_LIMIT_REDIS_URL`) for horizontal scaling; falls back
//! to in-process token bucket when Redis is unavailable (production fail-closed
//! when `SDKWORK_IM_REDIS_ENABLED=true`).

use std::sync::{Arc, Mutex};

use im_adapters_redis_cache::{
    RedisFixedWindowRateLimiter, gateway_rate_limit_redis_fail_closed_from_env,
    resolve_gateway_rate_limit_redis_url_from_env,
};
use im_domain_core::rate_limiter::DomainRateLimiter;

use crate::ApiError;
use crate::http_limits::{resolve_websocket_frame_rate_burst, resolve_websocket_frame_rate_rpm};

const WS_FRAME_RATE_SCOPE: &str = "session.ws_frame";
const WS_FRAME_RATE_WINDOW_SECS: u64 = 60;
const WS_FRAME_REDIS_KEY_PREFIX: &str = "session:ws_frame:";

#[derive(Clone)]
pub struct WebsocketFrameRateLimiter {
    rpm: u32,
    local: Arc<Mutex<DomainRateLimiter>>,
    redis: Option<RedisFixedWindowRateLimiter>,
    redis_fail_closed: bool,
}

impl WebsocketFrameRateLimiter {
    pub fn from_env() -> Self {
        let rpm = resolve_websocket_frame_rate_rpm();
        let burst = resolve_websocket_frame_rate_burst();
        let refill_per_sec = (rpm / 60).max(1);
        let redis = resolve_gateway_rate_limit_redis_url_from_env().and_then(|url| {
            RedisFixedWindowRateLimiter::try_from_url(url.as_str(), WS_FRAME_REDIS_KEY_PREFIX)
        });
        Self {
            rpm,
            local: Arc::new(Mutex::new(DomainRateLimiter::with_burst(
                rpm,
                refill_per_sec,
                burst,
            ))),
            redis,
            redis_fail_closed: gateway_rate_limit_redis_fail_closed_from_env(),
        }
    }

    pub fn check_frame(&self, principal_key: &str) -> Result<(), ApiError> {
        if let Some(redis) = &self.redis {
            let bucket = format!("{WS_FRAME_RATE_SCOPE}:{principal_key}");
            match redis.allow(bucket.as_str(), self.rpm, WS_FRAME_RATE_WINDOW_SECS) {
                Ok(true) => return Ok(()),
                Ok(false) => {
                    return Err(ApiError {
                        status: axum::http::StatusCode::TOO_MANY_REQUESTS,
                        code: "websocket_frame_rate_limited",
                        message: "websocket frame rate limit exceeded, please retry later"
                            .to_owned(),
                    });
                }
                Err(error) => {
                    tracing::warn!(
                        target: "sdkwork.im.session_gateway",
                        event = "im.session_gateway.ws_frame_rate_redis_unavailable",
                        ?error,
                        principal_key,
                        fail_closed = self.redis_fail_closed,
                        "redis websocket frame rate limit unavailable"
                    );
                    if self.redis_fail_closed {
                        return Err(ApiError {
                            status: axum::http::StatusCode::TOO_MANY_REQUESTS,
                            code: "websocket_frame_rate_limited",
                            message: "websocket frame rate limit backend unavailable".to_owned(),
                        });
                    }
                }
            }
        }

        let mut limiter = self.local.lock().map_err(|_| ApiError {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code: "websocket_frame_rate_limiter_unavailable",
            message: "websocket frame rate limiter lock poisoned".to_owned(),
        })?;
        limiter
            .check_rate(principal_key, "websocket.frame")
            .map_err(|_| ApiError {
                status: axum::http::StatusCode::TOO_MANY_REQUESTS,
                code: "websocket_frame_rate_limited",
                message: "websocket frame rate limit exceeded, please retry later".to_owned(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_rate_limiter_from_env_allows_first_check() {
        let limiter = WebsocketFrameRateLimiter::from_env();
        assert!(limiter.check_frame("tenant:1:user:42").is_ok());
    }
}
