//! Edge client-IP rate limiting applied after IM, IAM, and embedded dependency
//! routers are merged.
//!
//! Token-bucket per client IP with bounded key cardinality:
//! - `SDKWORK_IM_GATEWAY_RATE_LIMIT_RPM` (default `600`): sustained requests
//!   per minute per client IP.
//! - `SDKWORK_IM_GATEWAY_RATE_LIMIT_BURST` (default `50`): burst capacity.
//! - `SDKWORK_IM_GATEWAY_RATE_LIMIT_MAX_ENTRIES` (default `5000`): max tracked
//!   client IPs before eviction.
//!
//! Canonical infrastructure probes (`/healthz`, `/livez`, `/readyz`,
//! `/metrics`) and the `/openapi` documentation surface are exempt so
//! readiness and contract discovery never trip the edge limiter.
//!
//! This is the pre-auth edge layer; authenticated request throttling remains
//! the framework tier/tenant rate limit policy (Redis-backed when
//! `SDKWORK_IM_GATEWAY_RATE_LIMIT_REDIS_URL` / `SDKWORK_IM_REDIS_URL` is
//! configured, see `wire_redis_http_stores`).

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::extract::{ConnectInfo, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use im_domain_core::rate_limiter::DomainRateLimiter;

const RATE_LIMIT_RPM_ENV: &str = "SDKWORK_IM_GATEWAY_RATE_LIMIT_RPM";
const RATE_LIMIT_BURST_ENV: &str = "SDKWORK_IM_GATEWAY_RATE_LIMIT_BURST";
const RATE_LIMIT_MAX_ENTRIES_ENV: &str = "SDKWORK_IM_GATEWAY_RATE_LIMIT_MAX_ENTRIES";

const DEFAULT_RATE_LIMIT_RPM: u32 = 600;
const DEFAULT_RATE_LIMIT_BURST: u32 = 50;
const DEFAULT_RATE_LIMIT_MAX_ENTRIES: usize = 5_000;

const RATE_LIMIT_EXEMPT_PREFIXES: [&str; 5] =
    ["/healthz", "/livez", "/readyz", "/metrics", "/openapi"];

#[derive(Clone)]
pub struct EdgeIpRateLimiter {
    inner: Arc<Mutex<DomainRateLimiter>>,
}

impl EdgeIpRateLimiter {
    pub fn from_env() -> Self {
        let rpm = read_u32_env(RATE_LIMIT_RPM_ENV, DEFAULT_RATE_LIMIT_RPM).max(1);
        let burst = read_u32_env(RATE_LIMIT_BURST_ENV, DEFAULT_RATE_LIMIT_BURST).max(1);
        let max_entries =
            read_usize_env(RATE_LIMIT_MAX_ENTRIES_ENV, DEFAULT_RATE_LIMIT_MAX_ENTRIES).max(1);
        // Sustained tokens refill per second; a 1-request minimum keeps the
        // bucket moving even for very low RPM configurations.
        let refill_per_sec = (rpm / 60).max(1);
        Self {
            inner: Arc::new(Mutex::new(DomainRateLimiter::with_burst_and_capacity(
                rpm,
                refill_per_sec,
                burst,
                max_entries,
            ))),
        }
    }

    /// Consumes one token for the client IP. Returns `false` when the IP
    /// exceeds its window (or when the poisoned lock cannot be recovered).
    fn check(&self, client_ip: &str) -> bool {
        match self.inner.lock() {
            Ok(mut limiter) => limiter.check_rate(client_ip, "edge").is_ok(),
            Err(poisoned) => {
                tracing::warn!("recovering poisoned edge IP rate limiter lock");
                poisoned.into_inner().check_rate(client_ip, "edge").is_ok()
            }
        }
    }
}

pub async fn edge_ip_rate_limit(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(limiter): State<EdgeIpRateLimiter>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let path = request.uri().path();
    if RATE_LIMIT_EXEMPT_PREFIXES
        .iter()
        .any(|prefix| path.starts_with(prefix))
    {
        return next.run(request).await;
    }
    if !limiter.check(&addr.ip().to_string()) {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            [(axum::http::header::CONTENT_TYPE, "application/problem+json")],
            r#"{"type":"about:blank","title":"Too Many Requests","status":429,"code":"gateway_rate_limited","detail":"edge request rate limit exceeded"}"#,
        )
            .into_response();
    }
    next.run(request).await
}

fn read_u32_env(name: &str, default: u32) -> u32 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<u32>().ok())
        .unwrap_or(default)
}

fn read_usize_env(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_ip_limiter_tracks_ips_independently() {
        let limiter = EdgeIpRateLimiter {
            inner: Arc::new(Mutex::new(DomainRateLimiter::with_burst_and_capacity(
                60, 1, 3, 16,
            ))),
        };
        assert!(limiter.check("203.0.113.7"));
        assert!(limiter.check("203.0.113.7"));
        assert!(limiter.check("203.0.113.7"));
        // Exhausted for this IP only.
        assert!(!limiter.check("203.0.113.7"));
        assert!(limiter.check("203.0.113.8"));
    }

    #[test]
    fn edge_ip_limiter_recovers_after_refill() {
        let limiter = EdgeIpRateLimiter {
            inner: Arc::new(Mutex::new(DomainRateLimiter::with_burst_and_capacity(
                60, 10, 3, 16,
            ))),
        };
        assert!(limiter.check("203.0.113.9"));
        assert!(limiter.check("203.0.113.9"));
        assert!(limiter.check("203.0.113.9"));
        assert!(!limiter.check("203.0.113.9"));
        // Refill is 10 tokens/second; one refill tick restores capacity.
        std::thread::sleep(std::time::Duration::from_millis(1100));
        assert!(limiter.check("203.0.113.9"));
    }

    #[test]
    fn exempt_prefixes_cover_infrastructure_probes() {
        for prefix in RATE_LIMIT_EXEMPT_PREFIXES {
            assert!(
                prefix.starts_with('/'),
                "exempt prefix must start with '/': {prefix}"
            );
        }
        assert!(RATE_LIMIT_EXEMPT_PREFIXES.contains(&"/healthz"));
        assert!(RATE_LIMIT_EXEMPT_PREFIXES.contains(&"/metrics"));
    }
}
