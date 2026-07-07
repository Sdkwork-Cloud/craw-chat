//! Per-IP WebSocket upgrade rate limiting with optional Redis fixed-window backend.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::net::IpAddr;
use std::sync::{Arc, Mutex};

use axum::http::HeaderMap;
use im_adapters_redis_cache::{
    RedisFixedWindowRateLimiter, gateway_rate_limit_redis_fail_closed_from_env,
    resolve_gateway_rate_limit_redis_url_from_env,
};
use im_domain_core::rate_limiter::DomainRateLimiter;
use sdkwork_utils_rust::trusted_proxy::TrustedProxyConfig;

use crate::ApiError;
use crate::http_limits::{
    resolve_websocket_upgrade_rate_burst, resolve_websocket_upgrade_rate_rpm,
};

const WS_UPGRADE_RATE_SCOPE: &str = "session.ws_upgrade";
const WS_UPGRADE_RATE_WINDOW_SECS: u64 = 60;
const WS_UPGRADE_REDIS_KEY_PREFIX: &str = "session:ws_upgrade:";

#[derive(Clone)]
pub struct WebsocketUpgradeRateLimiter {
    rpm: u32,
    local: Arc<Mutex<DomainRateLimiter>>,
    redis: Option<RedisFixedWindowRateLimiter>,
    redis_fail_closed: bool,
}

impl WebsocketUpgradeRateLimiter {
    pub fn from_env() -> Self {
        let rpm = resolve_websocket_upgrade_rate_rpm();
        let burst = resolve_websocket_upgrade_rate_burst();
        let refill_per_sec = (rpm / 60).max(1);
        let redis = resolve_gateway_rate_limit_redis_url_from_env().and_then(|url| {
            RedisFixedWindowRateLimiter::try_from_url(url.as_str(), WS_UPGRADE_REDIS_KEY_PREFIX)
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

    pub fn check_upgrade(&self, client_ip: IpAddr) -> Result<(), ApiError> {
        if let Some(redis) = &self.redis {
            let bucket = format!("{WS_UPGRADE_RATE_SCOPE}:{client_ip}");
            match redis.allow(bucket.as_str(), self.rpm, WS_UPGRADE_RATE_WINDOW_SECS) {
                Ok(true) => return Ok(()),
                Ok(false) => {
                    return Err(ApiError {
                        status: axum::http::StatusCode::TOO_MANY_REQUESTS,
                        code: "websocket_upgrade_rate_limited",
                        message: "websocket upgrade rate limit exceeded, please retry later"
                            .to_owned(),
                    });
                }
                Err(error) => {
                    tracing::warn!(
                        target: "sdkwork.im.session_gateway",
                        event = "im.session_gateway.ws_upgrade_rate_redis_unavailable",
                        ?error,
                        client_ip = %client_ip,
                        fail_closed = self.redis_fail_closed,
                        "redis websocket upgrade rate limit unavailable"
                    );
                    if self.redis_fail_closed {
                        return Err(ApiError {
                            status: axum::http::StatusCode::TOO_MANY_REQUESTS,
                            code: "websocket_upgrade_rate_limited",
                            message: "websocket upgrade rate limit backend unavailable".to_owned(),
                        });
                    }
                }
            }
        }

        let mut limiter = self.local.lock().map_err(|_| ApiError {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code: "websocket_rate_limiter_unavailable",
            message: "websocket upgrade rate limiter lock poisoned".to_owned(),
        })?;
        limiter
            .check_rate(client_ip.to_string().as_str(), "websocket.upgrade")
            .map_err(|_| ApiError {
                status: axum::http::StatusCode::TOO_MANY_REQUESTS,
                code: "websocket_upgrade_rate_limited",
                message: "websocket upgrade rate limit exceeded, please retry later".to_owned(),
            })
    }
}

pub fn extract_client_ip_from_headers(headers: &HeaderMap) -> IpAddr {
    let config = TrustedProxyConfig::from_env();
    if !config.is_empty() {
        if let Some(raw) = header_value(headers, "x-forwarded-for") {
            let chain = raw
                .split(',')
                .map(str::trim)
                .filter(|segment| !segment.is_empty())
                .collect::<Vec<_>>();
            for entry in chain.iter().rev() {
                if let Ok(ip) = entry.parse::<IpAddr>()
                    && !config.is_trusted(&ip)
                {
                    return ip;
                }
            }
            if let Some(entry) = chain.first()
                && let Ok(ip) = entry.parse::<IpAddr>()
            {
                return ip;
            }
        }
        if let Some(raw) = header_value(headers, "x-real-ip")
            && let Ok(ip) = raw.trim().parse::<IpAddr>()
        {
            return ip;
        }
    }
    fallback_ip_from_headers(headers)
}

fn header_value(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .iter()
        .find(|(header_name, _)| header_name.as_str().eq_ignore_ascii_case(name))
        .and_then(|(_, value)| value.to_str().ok().map(str::to_owned))
}

fn fallback_ip_from_headers(headers: &HeaderMap) -> IpAddr {
    let mut hasher = DefaultHasher::new();
    if let Some(user_agent) = header_value(headers, "user-agent") {
        user_agent.hash(&mut hasher);
    }
    if let Some(language) = header_value(headers, "accept-language") {
        language.hash(&mut hasher);
    }
    let time_bucket = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        / 10;
    time_bucket.hash(&mut hasher);
    let hash = hasher.finish();
    let octet3 = ((hash >> 8) & 0xFF) as u8;
    let octet4 = (hash & 0xFF) as u8;
    IpAddr::V4(std::net::Ipv4Addr::new(198, 51, octet3, octet4))
}
