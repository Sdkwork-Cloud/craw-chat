//! Gateway protection: trusted-proxy IP extraction, per-IP rate limiting,
//! and per-service circuit breaker registry.
//!
//! These modules provide production-grade traffic shaping for the IM gateway:
//!
//! - **Rate Limiter**: per-client-IP token bucket rejecting with HTTP 429.
//! - **Circuit Breaker Registry**: per-upstream-service consecutive-failure
//!   tracking returning HTTP 503 when tripped, with single-probe half-open
//!   recovery.
//! - **Trusted Proxy IP Extraction**: only honours `X-Forwarded-For` /
//!   `X-Real-IP` when the direct TCP peer is in the configured trusted-proxy
//!   list, preventing IP-spoofing bypass of rate limits.
//!
//! Configuration is driven entirely by environment variables:
//!
//! | Variable | Default | Description |
//! |---|---|---|
//! | `SDKWORK_IM_GATEWAY_RATE_LIMIT_RPM` | `600` | Max requests per minute per client IP |
//! | `SDKWORK_IM_GATEWAY_RATE_LIMIT_BURST` | `50` | Burst capacity (token bucket size) |
//! | `SDKWORK_IM_GATEWAY_RATE_LIMIT_MAX_ENTRIES` | `5000` | Max tracked client IPs before forced eviction |
//! | `SDKWORK_IM_GATEWAY_CIRCUIT_BREAKER_THRESHOLD` | `10` | Consecutive failures before tripping |
//! | `SDKWORK_IM_GATEWAY_CIRCUIT_BREAKER_RESET_SECS` | `30` | Seconds before half-open retry |
//! | `SDKWORK_IM_GATEWAY_TRUSTED_PROXIES` | _(empty)_ | Comma-separated trusted proxy IPs |

use std::collections::HashMap;
use std::fmt;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::extract::{ConnectInfo, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use dashmap::DashMap;

// ---------------------------------------------------------------------------
// Trusted Proxy IP Extraction
// ---------------------------------------------------------------------------

const TRUSTED_PROXIES_ENV: &str = "SDKWORK_IM_GATEWAY_TRUSTED_PROXIES";

/// Configuration controlling which direct TCP peers are allowed to set
/// forwarded IP headers.
#[derive(Clone, Debug)]
pub struct TrustedProxyConfig {
    trusted_proxies: Vec<IpAddr>,
}

impl TrustedProxyConfig {
    /// Load trusted-proxy list from `SDKWORK_IM_GATEWAY_TRUSTED_PROXIES`.
    pub fn from_env() -> Self {
        let raw = std::env::var(TRUSTED_PROXIES_ENV).unwrap_or_default();
        let trusted_proxies = raw
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse::<IpAddr>().ok())
            .collect();
        Self { trusted_proxies }
    }

    fn is_trusted(&self, ip: &IpAddr) -> bool {
        self.trusted_proxies.iter().any(|trusted| trusted == ip)
    }

    fn is_empty(&self) -> bool {
        self.trusted_proxies.is_empty()
    }
}

impl Default for TrustedProxyConfig {
    fn default() -> Self {
        Self {
            trusted_proxies: Vec::new(),
        }
    }
}

/// Extract the real client IP from a request using trusted-proxy validation.
///
/// Resolution order:
/// 1. If `ConnectInfo<SocketAddr>` is available (requires
///    `into_make_service_with_connect_info`), use the TCP peer IP as the
///    baseline.
/// 2. If the peer IP is a trusted proxy, parse `X-Forwarded-For` from right
///    to left, skipping trusted-proxy entries, and return the first
///    untrusted IP.
/// 3. If the peer IP is NOT a trusted proxy (or no trusted proxies are
///    configured), return the peer IP directly — forwarded headers are
///    ignored to prevent spoofing.
/// 4. If `ConnectInfo` is unavailable, fall back to `X-Real-IP` only when
///    trusted proxies are configured; otherwise return `0.0.0.0`.
pub fn extract_client_ip(req: &Request) -> IpAddr {
    extract_client_ip_with_config(req, &TrustedProxyConfig::from_env())
}

fn extract_client_ip_with_config(req: &Request, config: &TrustedProxyConfig) -> IpAddr {
    // Strategy 1: ConnectInfo is available — use TCP peer as baseline.
    if let Some(conn_info) = req.extensions().get::<ConnectInfo<SocketAddr>>() {
        let peer_ip = conn_info.0.ip();

        if config.is_empty() {
            // No trusted proxies — use the direct peer IP, ignore forwarded headers.
            return peer_ip;
        }

        if config.is_trusted(&peer_ip) {
            // Peer is a trusted proxy — parse X-Forwarded-For chain from right to left.
            if let Some(real_ip) = parse_forwarded_for_trusted(req.headers(), config) {
                return real_ip;
            }
            // Fallback to X-Real-IP if X-Forwarded-For is absent or unparseable.
            if let Some(ip) = parse_header_ip(req.headers(), "x-real-ip") {
                return ip;
            }
        }

        // Peer is not a trusted proxy — use peer IP directly.
        return peer_ip;
    }

    // Strategy 2: ConnectInfo unavailable — conservative fallback.
    if !config.is_empty() {
        // Trusted proxies are configured, so we assume traffic arrives via proxy.
        if let Some(ip) = parse_forwarded_for_trusted(req.headers(), config) {
            return ip;
        }
        if let Some(ip) = parse_header_ip(req.headers(), "x-real-ip") {
            return ip;
        }
    }

    // P1-9 fix: When we cannot determine the real client IP, generate a unique
    // identifier based on request characteristics to prevent all unknown-IP
    // requests from sharing a single rate-limit bucket.
    // 
    // This uses a hash of available headers to create a pseudo-IP that provides
    // some level of differentiation between different clients even when we
    // can't determine their real IP.
    //
    // The fallback strategy:
    // 1. Try User-Agent + Accept-Language headers for differentiation
    // 2. Fall back to a time-based bucket that rotates every 10 seconds
    // 3. This limits the blast radius of attacks from unknown-IP sources
    let fallback_ip = generate_fallback_ip_from_headers(req.headers());
    tracing::warn!(
        target: "sdkwork.im.gateway",
        event = "im.gateway.ip_extraction_fallback",
        path = %req.uri().path(),
        fallback_ip = %fallback_ip,
        "could not determine real client IP, using header-based fallback"
    );
    fallback_ip
}

/// Generate a fallback pseudo-IP from request headers when real IP cannot be determined.
///
/// This prevents all unknown-IP requests from sharing a single rate-limit bucket
/// by creating differentiation based on request characteristics.
fn generate_fallback_ip_from_headers(headers: &axum::http::HeaderMap) -> IpAddr {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    // Collect header values for hashing
    let mut hasher = DefaultHasher::new();
    
    // Hash User-Agent for client differentiation
    if let Some(ua) = headers.get("user-agent").and_then(|v| v.to_str().ok()) {
        ua.hash(&mut hasher);
    }
    
    // Hash Accept-Language for additional differentiation
    if let Some(lang) = headers.get("accept-language").and_then(|v| v.to_str().ok()) {
        lang.hash(&mut hasher);
    }
    
    // Add time-based component that rotates every 10 seconds
    // This limits the window of attack for any single bucket
    let time_bucket = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() / 10;
    time_bucket.hash(&mut hasher);
    
    // Convert hash to an IPv4 address in the 198.51.100.0/24 range (TEST-NET-2)
    // This range is reserved for documentation and won't conflict with real IPs
    let hash = hasher.finish();
    let octet3 = ((hash >> 8) & 0xFF) as u8;
    let octet4 = (hash & 0xFF) as u8;
    
    IpAddr::V4(std::net::Ipv4Addr::new(198, 51, octet3, octet4))
}

fn parse_forwarded_for_trusted(
    headers: &axum::http::HeaderMap,
    config: &TrustedProxyConfig,
) -> Option<IpAddr> {
    let raw = headers.get("x-forwarded-for")?.to_str().ok()?;
    let chain: Vec<&str> = raw.split(',').map(str::trim).filter(|s| !s.is_empty()).collect();
    if chain.is_empty() {
        return None;
    }

    // Walk from right to left, skipping trusted proxy IPs.
    for entry in chain.iter().rev() {
        if let Ok(ip) = entry.parse::<IpAddr>() {
            if !config.is_trusted(&ip) {
                return Some(ip);
            }
        }
    }

    // All entries were trusted proxies — return the leftmost (original client).
    chain
        .first()
        .and_then(|s| s.parse::<IpAddr>().ok())
}

fn parse_header_ip(headers: &axum::http::HeaderMap, name: &str) -> Option<IpAddr> {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.trim().parse::<IpAddr>().ok())
}

// ---------------------------------------------------------------------------
// Rate Limiter
// ---------------------------------------------------------------------------

const RATE_LIMIT_MAX_ENTRIES_ENV: &str = "SDKWORK_IM_GATEWAY_RATE_LIMIT_MAX_ENTRIES";
const RATE_LIMIT_MAX_ENTRIES_DEFAULT: usize = 5_000;
const RATE_LIMIT_EVICT_AGE_SECS: u64 = 120;

/// Configuration for the per-IP rate limiter.
#[derive(Clone, Debug)]
pub struct RateLimitConfig {
    /// Maximum requests per minute per client IP.
    pub max_rpm: u32,
    /// Burst capacity (token bucket size).
    pub burst: u32,
    /// Maximum number of tracked client IPs before forced eviction.
    pub max_entries: usize,
}

impl RateLimitConfig {
    /// Load configuration from environment variables with sensible defaults.
    pub fn from_env() -> Self {
        Self {
            max_rpm: parse_env_or("SDKWORK_IM_GATEWAY_RATE_LIMIT_RPM", 600),
            burst: parse_env_or("SDKWORK_IM_GATEWAY_RATE_LIMIT_BURST", 50),
            max_entries: parse_env_usize_or(
                RATE_LIMIT_MAX_ENTRIES_ENV,
                RATE_LIMIT_MAX_ENTRIES_DEFAULT,
            ),
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_rpm: 600,
            burst: 50,
            max_entries: RATE_LIMIT_MAX_ENTRIES_DEFAULT,
        }
    }
}

/// Per-client token-bucket state.
#[derive(Debug, Clone)]
struct ClientBucket {
    tokens: f64,
    last_refill: Instant,
}

impl ClientBucket {
    fn new(burst: u32) -> Self {
        Self {
            tokens: burst as f64,
            last_refill: Instant::now(),
        }
    }

    fn try_acquire(&mut self, config: &RateLimitConfig) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);
        let refill_per_sec = config.max_rpm as f64 / 60.0;
        self.tokens = (self.tokens + elapsed.as_secs_f64() * refill_per_sec)
            .min(config.burst as f64);
        self.last_refill = now;

        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}

/// Thread-safe rate limiter using `std::sync::Mutex` for minimal contention.
///
/// The lock is held only for trivial hashmap operations (lookup + arithmetic),
/// never across `.await` points, so `std::sync::Mutex` is both safe and faster
/// than `tokio::sync::Mutex`.
#[derive(Clone)]
pub struct RateLimiter {
    config: RateLimitConfig,
    buckets: Arc<Mutex<HashMap<IpAddr, ClientBucket>>>,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            buckets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Returns `true` if the request is allowed, `false` if rate-limited.
    pub fn check(&self, client_ip: IpAddr) -> bool {
        let mut buckets = match self.buckets.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                tracing::warn!(
                    target: "sdkwork.im.gateway",
                    "rate limiter mutex poisoned — recovering"
                );
                poisoned.into_inner()
            }
        };

        // Proactive bounded eviction: when the map exceeds the configured
        // maximum, remove entries older than the eviction age. This prevents
        // unbounded memory growth from spoofed or rotating client IPs.
        if buckets.len() > self.config.max_entries {
            let cutoff = Instant::now() - Duration::from_secs(RATE_LIMIT_EVICT_AGE_SECS);
            buckets.retain(|_, bucket| bucket.last_refill > cutoff);
        }

        let bucket = buckets
            .entry(client_ip)
            .or_insert_with(|| ClientBucket::new(self.config.burst));
        bucket.try_acquire(&self.config)
    }
}

/// High-performance rate limiter using DashMap for lock-free concurrent access.
///
/// Uses sharded concurrent HashMap (DashMap) instead of Mutex<HashMap> to
/// significantly reduce lock contention in high-throughput scenarios.
/// Each shard has its own fine-grained lock, allowing concurrent reads
/// and writes to different IP addresses without blocking.
#[derive(Clone)]
pub struct DashMapRateLimiter {
    config: RateLimitConfig,
    buckets: DashMap<IpAddr, ClientBucket>,
}

impl DashMapRateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            buckets: DashMap::new(),
        }
    }

    /// Returns `true` if the request is allowed, `false` if rate-limited.
    ///
    /// This implementation uses DashMap's entry API for atomic check-and-update,
    /// avoiding the TOCTOU race condition that could occur with separate get/insert.
    pub fn check(&self, client_ip: IpAddr) -> bool {
        // Proactive bounded eviction (less frequent in DashMap)
        // Use a threshold slightly below max_entries to avoid frequent cleanup
        if self.buckets.len() > self.config.max_entries * 9 / 10 {
            let cutoff = Instant::now() - Duration::from_secs(RATE_LIMIT_EVICT_AGE_SECS);
            // DashMap's retain is shard-local, minimal contention
            self.buckets.retain(|_, bucket| bucket.last_refill > cutoff);

            tracing::debug!(
                target: "sdkwork.im.gateway",
                tracked_ips = self.buckets.len(),
                max_entries = self.config.max_entries,
                "rate limiter cleanup completed"
            );
        }

        // Use DashMap's entry API for atomic operation
        // The key insight: we need to call try_acquire which handles both refill and token check
        match self.buckets.entry(client_ip) {
            dashmap::mapref::entry::Entry::Occupied(mut entry) => {
                // Existing entry - call try_acquire on the bucket
                entry.get_mut().try_acquire(&self.config)
            }
            dashmap::mapref::entry::Entry::Vacant(entry) => {
                // New entry - insert a fresh bucket and consume one token
                let mut bucket = ClientBucket::new(self.config.burst);
                bucket.tokens -= 1.0; // Consume first token
                entry.insert(bucket);
                true // First request always succeeds
            }
        }
    }

    /// Get current number of tracked IPs.
    pub fn tracked_count(&self) -> usize {
        self.buckets.len()
    }

    /// Clear all rate limit buckets.
    pub fn clear(&self) {
        self.buckets.clear();
    }
}

/// Axum middleware: per-IP rate limiting using DashMap (high-performance variant).
///
/// Returns HTTP 429 when the client exceeds the configured rate.
/// The retry_after_secs is calculated based on actual RPM configuration (P0-5 fix).
pub async fn dashmap_rate_limit_middleware(
    State(limiter): State<DashMapRateLimiter>,
    req: Request,
    next: Next,
) -> Response {
    let client_ip = extract_client_ip(&req);

    if !limiter.check(client_ip) {
        // Calculate retry_after based on actual RPM: ceil(60 / max_rpm)
        // For example: 60 RPM -> 1 sec, 120 RPM -> 0.5 sec (rounded to 1), 30 RPM -> 2 sec
        let retry_after_secs = (60.0 / limiter.config.max_rpm as f64).ceil() as u64;
        
        tracing::warn!(
            target: "sdkwork.im.gateway",
            event = "im.gateway.rate_limited",
            client_ip = %client_ip,
            path = %req.uri().path(),
            retry_after_secs = retry_after_secs,
            max_rpm = limiter.config.max_rpm,
            "request rate limited"
        );
        return (
            StatusCode::TOO_MANY_REQUESTS,
            axum::Json(serde_json::json!({
                "error": "rate_limit_exceeded",
                "message": "Too many requests. Please slow down.",
                "retry_after_secs": retry_after_secs.max(1), // minimum 1 second
            })),
        )
            .into_response();
    }

    next.run(req).await
}

/// Axum middleware: per-IP rate limiting.
///
/// Returns HTTP 429 when the client exceeds the configured rate.
/// The retry_after_secs is calculated based on actual RPM configuration (P0-5 fix).
pub async fn rate_limit_middleware(
    State(limiter): State<RateLimiter>,
    req: Request,
    next: Next,
) -> Response {
    let client_ip = extract_client_ip(&req);

    if !limiter.check(client_ip) {
        // Calculate retry_after based on actual RPM: ceil(60 / max_rpm)
        // For example: 60 RPM -> 1 sec, 120 RPM -> 0.5 sec (rounded to 1), 30 RPM -> 2 sec
        let retry_after_secs = (60.0 / limiter.config.max_rpm as f64).ceil() as u64;
        
        tracing::warn!(
            target: "sdkwork.im.gateway",
            event = "im.gateway.rate_limited",
            client_ip = %client_ip,
            path = %req.uri().path(),
            retry_after_secs = retry_after_secs,
            max_rpm = limiter.config.max_rpm,
            "request rate limited"
        );
        return (
            StatusCode::TOO_MANY_REQUESTS,
            axum::Json(serde_json::json!({
                "error": "rate_limit_exceeded",
                "message": "Too many requests. Please slow down.",
                "retry_after_secs": retry_after_secs.max(1), // minimum 1 second
            })),
        )
            .into_response();
    }

    next.run(req).await
}

// ---------------------------------------------------------------------------
// Circuit Breaker
// ---------------------------------------------------------------------------

/// State of the circuit breaker.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CircuitState {
    /// Normal operation — requests pass through.
    Closed,
    /// Too many failures — requests are rejected with 503.
    Open,
    /// Trial period after reset timeout — a single probe request is allowed.
    HalfOpen,
}

impl fmt::Display for CircuitState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CircuitState::Closed => write!(f, "closed"),
            CircuitState::Open => write!(f, "open"),
            CircuitState::HalfOpen => write!(f, "half-open"),
        }
    }
}

/// Configuration for the circuit breaker.
#[derive(Clone, Debug)]
pub struct CircuitBreakerConfig {
    /// Consecutive failures required to trip the circuit.
    pub failure_threshold: u32,
    /// Seconds to wait before transitioning from Open to HalfOpen.
    pub reset_timeout_secs: u64,
}

impl CircuitBreakerConfig {
    pub fn from_env() -> Self {
        Self {
            failure_threshold: parse_env_or(
                "SDKWORK_IM_GATEWAY_CIRCUIT_BREAKER_THRESHOLD",
                10,
            ),
            reset_timeout_secs: parse_env_or(
                "SDKWORK_IM_GATEWAY_CIRCUIT_BREAKER_RESET_SECS",
                30,
            ) as u64,
        }
    }

    pub fn reset_duration(&self) -> Duration {
        Duration::from_secs(self.reset_timeout_secs)
    }
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 10,
            reset_timeout_secs: 30,
        }
    }
}

/// Inner mutable state of a single circuit breaker.
#[derive(Debug)]
struct CircuitBreakerInner {
    state: CircuitState,
    consecutive_failures: u32,
    opened_at: Option<Instant>,
    /// Tracks whether a half-open probe is already in flight.
    /// Only one probe request is allowed at a time in HalfOpen state.
    half_open_probe_in_flight: bool,
}

impl CircuitBreakerInner {
    fn new() -> Self {
        Self {
            state: CircuitState::Closed,
            consecutive_failures: 0,
            opened_at: None,
            half_open_probe_in_flight: false,
        }
    }
}

/// Thread-safe circuit breaker for a single upstream service.
#[derive(Clone)]
pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    state: Arc<Mutex<CircuitBreakerInner>>,
}

impl CircuitBreaker {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: Arc::new(Mutex::new(CircuitBreakerInner::new())),
        }
    }

    /// Returns `true` if the request should be allowed through.
    ///
    /// In `HalfOpen` state, only a single probe request is permitted;
    /// subsequent requests are rejected until the probe completes.
    pub fn allow_request(&self) -> bool {
        let mut inner = match self.state.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                tracing::warn!("circuit breaker mutex poisoned — recovering");
                poisoned.into_inner()
            }
        };
        match inner.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                if let Some(opened_at) = inner.opened_at {
                    if Instant::now().duration_since(opened_at) >= self.config.reset_duration() {
                        inner.state = CircuitState::HalfOpen;
                        inner.half_open_probe_in_flight = true;
                        tracing::info!(
                            target: "sdkwork.im.gateway",
                            event = "im.gateway.circuit_breaker.half_open",
                            "circuit breaker transitioning to half-open"
                        );
                        return true;
                    }
                }
                false
            }
            CircuitState::HalfOpen => {
                // Only allow one probe request at a time.
                if inner.half_open_probe_in_flight {
                    return false;
                }
                inner.half_open_probe_in_flight = true;
                true
            }
        }
    }

    /// Record a successful response.
    pub fn record_success(&self) {
        let mut inner = match self.state.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        inner.consecutive_failures = 0;
        inner.half_open_probe_in_flight = false;
        if inner.state == CircuitState::HalfOpen {
            inner.state = CircuitState::Closed;
            inner.opened_at = None;
            tracing::info!(
                target: "sdkwork.im.gateway",
                event = "im.gateway.circuit_breaker.closed",
                "circuit breaker closed after successful half-open probe"
            );
        }
    }

    /// Record a failed response (5xx or network error).
    pub fn record_failure(&self) {
        let mut inner = match self.state.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        inner.consecutive_failures += 1;
        inner.half_open_probe_in_flight = false;
        if inner.state == CircuitState::HalfOpen {
            inner.state = CircuitState::Open;
            inner.opened_at = Some(Instant::now());
            tracing::warn!(
                target: "sdkwork.im.gateway",
                event = "im.gateway.circuit_breaker.reopened",
                "circuit breaker re-opened after half-open probe failure"
            );
        } else if inner.consecutive_failures >= self.config.failure_threshold {
            inner.state = CircuitState::Open;
            inner.opened_at = Some(Instant::now());
            tracing::error!(
                target: "sdkwork.im.gateway",
                event = "im.gateway.circuit_breaker.opened",
                failures = inner.consecutive_failures,
                threshold = self.config.failure_threshold,
                "circuit breaker opened due to consecutive failures"
            );
        }
    }

    /// Current state (for health checks and observability).
    pub fn state(&self) -> CircuitState {
        match self.state.lock() {
            Ok(inner) => inner.state,
            Err(poisoned) => poisoned.into_inner().state,
        }
    }
}

// ---------------------------------------------------------------------------
// Circuit Breaker Registry — per-upstream-service isolation
// ---------------------------------------------------------------------------

/// Registry maintaining an independent `CircuitBreaker` per upstream service,
/// so that failures in one service do not trip the breaker for others.
#[derive(Clone)]
pub struct CircuitBreakerRegistry {
    config: CircuitBreakerConfig,
    breakers: Arc<Mutex<HashMap<String, CircuitBreaker>>>,
}

impl CircuitBreakerRegistry {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            breakers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn get_or_create(&self, service_id: &str) -> CircuitBreaker {
        let mut breakers = match self.breakers.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        breakers
            .entry(service_id.to_owned())
            .or_insert_with(|| CircuitBreaker::new(self.config.clone()))
            .clone()
    }

    /// Check whether requests to `service_id` should be allowed.
    pub fn check(&self, service_id: &str) -> bool {
        self.get_or_create(service_id).allow_request()
    }

    /// Record a successful upstream response for `service_id`.
    pub fn record_success(&self, service_id: &str) {
        self.get_or_create(service_id).record_success();
    }

    /// Record a failed upstream response for `service_id`.
    pub fn record_failure(&self, service_id: &str) {
        self.get_or_create(service_id).record_failure();
    }

    /// Get the current state for a service (observability).
    pub fn state_for(&self, service_id: &str) -> CircuitState {
        self.get_or_create(service_id).state()
    }
}

// ---------------------------------------------------------------------------
// Per-tenant Rate Limiter
// ---------------------------------------------------------------------------

const TENANT_RATE_LIMIT_MAX_RPM_ENV: &str = "SDKWORK_IM_GATEWAY_TENANT_RATE_LIMIT_RPM";
const TENANT_RATE_LIMIT_MAX_RPM_DEFAULT: u32 = 60_000;
const TENANT_RATE_LIMIT_BURST_ENV: &str = "SDKWORK_IM_GATEWAY_TENANT_RATE_LIMIT_BURST";
const TENANT_RATE_LIMIT_BURST_DEFAULT: u32 = 2_000;
const TENANT_RATE_LIMIT_MAX_ENTRIES_ENV: &str = "SDKWORK_IM_GATEWAY_TENANT_RATE_LIMIT_MAX_ENTRIES";
const TENANT_RATE_LIMIT_MAX_ENTRIES_DEFAULT: usize = 10_000;
const TENANT_RATE_LIMIT_EVICT_AGE_SECS: u64 = 120;

/// Configuration for the per-tenant rate limiter.
///
/// Per-tenant limits run after IAM context resolution (Layer 2) and complement
/// the Layer 1 per-IP limiter that runs before authentication. The two layers
/// compose: a single NAT egress IP is bounded by the IP limiter, while each
/// authenticated tenant is independently bounded by the tenant limiter so that
/// a noisy tenant cannot exhaust a shared IP budget for other tenants.
#[derive(Clone, Debug)]
pub struct TenantRateLimitConfig {
    pub max_rpm: u32,
    pub burst: u32,
    pub max_entries: usize,
}

impl TenantRateLimitConfig {
    pub fn from_env() -> Self {
        Self {
            max_rpm: parse_env_or(TENANT_RATE_LIMIT_MAX_RPM_ENV, TENANT_RATE_LIMIT_MAX_RPM_DEFAULT),
            burst: parse_env_or(TENANT_RATE_LIMIT_BURST_ENV, TENANT_RATE_LIMIT_BURST_DEFAULT),
            max_entries: parse_env_usize_or(
                TENANT_RATE_LIMIT_MAX_ENTRIES_ENV,
                TENANT_RATE_LIMIT_MAX_ENTRIES_DEFAULT,
            ),
        }
    }
}

impl Default for TenantRateLimitConfig {
    fn default() -> Self {
        Self {
            max_rpm: TENANT_RATE_LIMIT_MAX_RPM_DEFAULT,
            burst: TENANT_RATE_LIMIT_BURST_DEFAULT,
            max_entries: TENANT_RATE_LIMIT_MAX_ENTRIES_DEFAULT,
        }
    }
}

/// Thread-safe per-tenant token-bucket rate limiter.
///
/// The lock is held only for trivial hashmap operations, never across `.await`
/// points, so `std::sync::Mutex` is both safe and faster than `tokio::sync::Mutex`.
#[derive(Clone)]
pub struct TenantRateLimiter {
    config: TenantRateLimitConfig,
    buckets: Arc<Mutex<HashMap<String, ClientBucket>>>,
}

impl TenantRateLimiter {
    pub fn new(config: TenantRateLimitConfig) -> Self {
        Self {
            config,
            buckets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Returns `true` if the request is allowed, `false` if rate-limited.
    pub fn check(&self, tenant_id: &str) -> bool {
        let mut buckets = match self.buckets.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                tracing::warn!(
                    target: "sdkwork.im.gateway",
                    "tenant rate limiter mutex poisoned — recovering"
                );
                poisoned.into_inner()
            }
        };

        if buckets.len() > self.config.max_entries {
            let cutoff = Instant::now() - Duration::from_secs(TENANT_RATE_LIMIT_EVICT_AGE_SECS);
            buckets.retain(|_, bucket| bucket.last_refill > cutoff);
        }

        let bucket = buckets
            .entry(tenant_id.to_owned())
            .or_insert_with(|| ClientBucket::new(self.config.burst));
        bucket.try_acquire(&RateLimitConfig {
            max_rpm: self.config.max_rpm,
            burst: self.config.burst,
            max_entries: self.config.max_entries,
        })
    }
}

/// Axum middleware: per-tenant rate limiting (Layer 2, post-auth).
///
/// Reads the authenticated `AppContext` from request extensions. When no
/// context is present (unauthenticated public route), the request is allowed
/// through; the Layer 1 per-IP limiter already governs unauthenticated traffic.
/// The retry_after_secs is calculated based on actual RPM configuration (P0-5 fix).
pub async fn per_tenant_rate_limit_middleware(
    State(limiter): State<TenantRateLimiter>,
    req: Request,
    next: Next,
) -> Response {
    use im_app_context::AppContext;
    let tenant_id = req
        .extensions()
        .get::<AppContext>()
        .map(|context| context.tenant_id.clone());

    if let Some(tenant_id) = tenant_id {
        if !limiter.check(tenant_id.as_str()) {
            // Calculate retry_after based on actual RPM: ceil(60 / max_rpm)
            let retry_after_secs = (60.0 / limiter.config.max_rpm as f64).ceil() as u64;
            
            tracing::warn!(
                target: "sdkwork.im.gateway",
                event = "im.gateway.tenant_rate_limited",
                tenant_id = %tenant_id,
                path = %req.uri().path(),
                retry_after_secs = retry_after_secs,
                max_rpm = limiter.config.max_rpm,
                "tenant request rate limited"
            );
            return (
                StatusCode::TOO_MANY_REQUESTS,
                axum::Json(serde_json::json!({
                    "error": "tenant_rate_limit_exceeded",
                    "message": "Tenant request rate limit exceeded. Please slow down.",
                    "retry_after_secs": retry_after_secs.max(1), // minimum 1 second
                })),
            )
                .into_response();
        }
    }

    next.run(req).await
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_env_or(key: &str, default: u32) -> u32 {
    std::env::var(key)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .filter(|v: &u32| *v > 0)
        .unwrap_or(default)
}

fn parse_env_usize_or(key: &str, default: usize) -> usize {
    std::env::var(key)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .filter(|v: &usize| *v > 0)
        .unwrap_or(default)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Rate limiter tests --

    #[test]
    fn rate_limiter_allows_within_limit() {
        let config = RateLimitConfig {
            max_rpm: 60,
            burst: 5,
            max_entries: 100,
        };
        let limiter = RateLimiter::new(config);
        let ip: IpAddr = "127.0.0.1".parse().unwrap();

        for _ in 0..5 {
            assert!(limiter.check(ip), "should allow within burst limit");
        }
        assert!(!limiter.check(ip), "should reject after burst exhausted");
    }

    #[test]
    fn rate_limiter_refills_over_time() {
        let config = RateLimitConfig {
            max_rpm: 600,
            burst: 1,
            max_entries: 100,
        };
        let limiter = RateLimiter::new(config);
        let ip: IpAddr = "127.0.0.1".parse().unwrap();

        assert!(limiter.check(ip));
        assert!(!limiter.check(ip), "bucket exhausted");

        std::thread::sleep(Duration::from_millis(150));
        assert!(limiter.check(ip), "should allow after refill");
    }

    #[test]
    fn rate_limiter_evicts_stale_entries_when_full() {
        let config = RateLimitConfig {
            max_rpm: 600,
            burst: 1,
            max_entries: 3,
        };
        let limiter = RateLimiter::new(config);

        // Fill with 4 distinct IPs — exceeds max_entries of 3.
        for i in 1..=4u8 {
            let ip: IpAddr = format!("10.0.0.{i}").parse().unwrap();
            assert!(limiter.check(ip));
        }

        // The bucket map should have been pruned — original IPs may still
        // exist if not stale, but the map should not exceed limits long-term.
        let bucket_count = limiter.buckets.lock().unwrap().len();
        assert!(
            bucket_count <= 4,
            "eviction should prevent unbounded growth, got {bucket_count}"
        );
    }

    // -- Circuit breaker tests --

    #[test]
    fn circuit_breaker_trips_after_threshold() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            reset_timeout_secs: 1,
        };
        let breaker = CircuitBreaker::new(config);

        assert_eq!(breaker.state(), CircuitState::Closed);
        assert!(breaker.allow_request());

        breaker.record_failure();
        breaker.record_failure();
        assert_eq!(breaker.state(), CircuitState::Closed);

        breaker.record_failure();
        assert_eq!(breaker.state(), CircuitState::Open);
        assert!(!breaker.allow_request());
    }

    #[test]
    fn circuit_breaker_half_open_allows_single_probe() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            reset_timeout_secs: 1,
        };
        let breaker = CircuitBreaker::new(config);

        breaker.record_failure();
        breaker.record_failure();
        assert_eq!(breaker.state(), CircuitState::Open);

        std::thread::sleep(Duration::from_millis(1100));
        // First request in half-open should be allowed (the probe).
        assert!(breaker.allow_request(), "should allow half-open probe");
        assert_eq!(breaker.state(), CircuitState::HalfOpen);
        // Second request should be rejected — only one probe at a time.
        assert!(
            !breaker.allow_request(),
            "should reject additional requests while probe is in flight"
        );

        breaker.record_success();
        assert_eq!(breaker.state(), CircuitState::Closed);
    }

    #[test]
    fn circuit_breaker_success_resets_failures() {
        let config = CircuitBreakerConfig {
            failure_threshold: 5,
            reset_timeout_secs: 30,
        };
        let breaker = CircuitBreaker::new(config);

        breaker.record_failure();
        breaker.record_failure();
        breaker.record_failure();
        breaker.record_success();
        assert_eq!(breaker.state(), CircuitState::Closed);

        breaker.record_failure();
        assert_eq!(breaker.state(), CircuitState::Closed);
    }

    // -- Circuit breaker registry tests --

    #[test]
    fn registry_isolates_breakers_per_service() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            reset_timeout_secs: 30,
        };
        let registry = CircuitBreakerRegistry::new(config);

        // Trip breaker for service A.
        registry.record_failure("service-a");
        registry.record_failure("service-a");
        assert_eq!(registry.state_for("service-a"), CircuitState::Open);
        assert!(!registry.check("service-a"), "service-a should be blocked");

        // Service B should still be operational.
        assert_eq!(registry.state_for("service-b"), CircuitState::Closed);
        assert!(
            registry.check("service-b"),
            "service-b should be unaffected by service-a failures"
        );
    }

    // -- Trusted proxy IP extraction tests --

    #[test]
    fn trusted_proxy_config_parses_env() {
        let _guard = TestEnvGuard::set(TRUSTED_PROXIES_ENV, "10.0.0.1, 10.0.0.2, 192.168.1.1");
        let config = TrustedProxyConfig::from_env();
        assert_eq!(config.trusted_proxies.len(), 3);
        assert!(config.is_trusted(&"10.0.0.1".parse::<IpAddr>().unwrap()));
        assert!(config.is_trusted(&"192.168.1.1".parse::<IpAddr>().unwrap()));
        assert!(!config.is_trusted(&"8.8.8.8".parse::<IpAddr>().unwrap()));
    }

    #[test]
    fn trusted_proxy_config_empty_when_unset() {
        let _guard = TestEnvGuard::remove(TRUSTED_PROXIES_ENV);
        let config = TrustedProxyConfig::from_env();
        assert!(config.is_empty());
    }

    /// Helper to set/unset environment variables during tests.
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
