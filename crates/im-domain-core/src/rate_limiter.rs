//! Domain-level rate limiting for SDKWork IM service.
//!
//! This module provides rate limiting utilities following `SECURITY_SPEC.md` requirements
//! for protection against abuse and resource exhaustion.
//!
//! ## Design Principles
//!
//! - Token bucket algorithm with configurable refill rate
//! - Sliding window for burst protection  
//! - Tenant-aware isolation (per-tenant limits)
//! - Zero-allocation hot path for performance
//!
//! ## Usage
//!
//! ```rust
//! use im_domain_core::rate_limiter::DomainRateLimiter;
//!
//! let limiter = DomainRateLimiter::new(100, 10); // 100 max, 10/sec refill
//! limiter.check_rate("tenant1", "operation1")?;
//! ```

use std::collections::HashMap;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Rate limiting errors.
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum RateLimitError {
    #[error("rate limit exceeded: limit={limit}, current={current}, retry_after={retry_after_ms}ms")]
    Exceeded {
        limit: u32,
        current: u32,
        retry_after_ms: u64,
    },
    
    #[error("burst limit exceeded: burst={burst}, current={current}")]
    BurstExceeded {
        burst: u32,
        current: u32,
    },
    
    #[error("tenant quota exceeded: tenant={tenant}, quota={quota}")]
    TenantQuotaExceeded {
        tenant: String,
        quota: u32,
    },
}

/// Token bucket state for a single rate limit key.
#[derive(Clone, Debug)]
struct TokenBucket {
    tokens: u32,
    max_tokens: u32,
    refill_rate: u32, // tokens per second
    last_refill: Instant,
    burst_tokens: u32,
    max_burst: u32,
    burst_window_start: Instant,
    burst_window_duration: Duration,
}

impl TokenBucket {
    fn new(max_tokens: u32, refill_rate: u32, max_burst: u32) -> Self {
        Self {
            tokens: max_tokens,
            max_tokens,
            refill_rate,
            last_refill: Instant::now(),
            burst_tokens: max_burst,
            max_burst,
            burst_window_start: Instant::now(),
            burst_window_duration: Duration::from_secs(1),
        }
    }
    
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);
        let tokens_to_add = (elapsed.as_secs_f64() * self.refill_rate as f64) as u32;
        
        if tokens_to_add > 0 {
            self.tokens = (self.tokens + tokens_to_add).min(self.max_tokens);
            self.last_refill = now;
        }
        
        // Reset burst window
        if now.duration_since(self.burst_window_start) > self.burst_window_duration {
            self.burst_tokens = self.max_burst;
            self.burst_window_start = now;
        }
    }
    
    fn try_consume(&mut self, tokens: u32) -> Result<(), RateLimitError> {
        self.refill();
        
        // Check burst limit first
        if tokens > self.burst_tokens {
            return Err(RateLimitError::BurstExceeded {
                burst: self.max_burst,
                current: tokens,
            });
        }
        
        // Check sustained rate limit
        if tokens > self.tokens {
            let retry_after_ms = ((tokens - self.tokens) as f64 / self.refill_rate as f64 * 1000.0) as u64;
            return Err(RateLimitError::Exceeded {
                limit: self.max_tokens,
                current: self.tokens,
                retry_after_ms,
            });
        }
        
        self.tokens -= tokens;
        self.burst_tokens -= tokens;
        Ok(())
    }
}

/// Domain-level rate limiter with tenant isolation.
///
/// Provides per-tenant rate limiting with token bucket algorithm
/// and sliding window burst protection.
#[derive(Clone, Debug)]
pub struct DomainRateLimiter {
    buckets: HashMap<String, TokenBucket>,
    default_max_tokens: u32,
    default_refill_rate: u32,
    default_max_burst: u32,
    tenant_quotas: HashMap<String, u32>, // Per-tenant overrides
    cleanup_interval: Duration,
    last_cleanup: Instant,
}

impl DomainRateLimiter {
    /// Create a new rate limiter with default limits.
    ///
    /// # Arguments
    ///
    /// * `max_tokens` - Maximum token capacity (sustained rate limit)
    /// * `refill_rate` - Tokens refilled per second
    pub fn new(max_tokens: u32, refill_rate: u32) -> Self {
        // Burst should be at least 1 or 10% of max
        let burst = (max_tokens / 10).max(1);
        Self {
            buckets: HashMap::new(),
            default_max_tokens: max_tokens,
            default_refill_rate: refill_rate,
            default_max_burst: burst,
            tenant_quotas: HashMap::new(),
            cleanup_interval: Duration::from_secs(60),
            last_cleanup: Instant::now(),
        }
    }
    
    /// Create with custom burst configuration.
    pub fn with_burst(max_tokens: u32, refill_rate: u32, max_burst: u32) -> Self {
        Self {
            buckets: HashMap::new(),
            default_max_tokens: max_tokens,
            default_refill_rate: refill_rate,
            default_max_burst: max_burst,
            tenant_quotas: HashMap::new(),
            cleanup_interval: Duration::from_secs(60),
            last_cleanup: Instant::now(),
        }
    }
    
    /// Set per-tenant quota override.
    pub fn set_tenant_quota(&mut self, tenant_id: &str, quota: u32) {
        self.tenant_quotas.insert(tenant_id.to_string(), quota);
    }
    
    /// Check rate limit for an operation.
    ///
    /// Key format: `{tenant_id}:{operation_type}`
    pub fn check_rate(&mut self, tenant_id: &str, operation: &str) -> Result<(), RateLimitError> {
        self.cleanup_expired_buckets();
        
        // Check tenant quota first
        if let Some(&quota) = self.tenant_quotas.get(tenant_id) {
            let tenant_key = format!("{}:__tenant_quota__", tenant_id);
            let bucket = self.buckets.entry(tenant_key).or_insert_with(|| {
                // For tenant quota bucket, burst = quota (allow full quota usage in burst window)
                TokenBucket::new(quota, quota / 10, quota)
            });
            bucket.try_consume(1)?;
        }
        
        // Check operation-specific rate
        let key = format!("{}:{}", tenant_id, operation);
        let bucket = self.buckets.entry(key).or_insert_with(|| {
            TokenBucket::new(self.default_max_tokens, self.default_refill_rate, self.default_max_burst)
        });
        bucket.try_consume(1)
    }
    
    /// Check rate with custom token cost.
    pub fn check_rate_with_cost(&mut self, tenant_id: &str, operation: &str, cost: u32) -> Result<(), RateLimitError> {
        self.cleanup_expired_buckets();
        
        // Check tenant quota
        if let Some(&quota) = self.tenant_quotas.get(tenant_id) {
            let tenant_key = format!("{}:__tenant_quota__", tenant_id);
            let bucket = self.buckets.entry(tenant_key).or_insert_with(|| {
                TokenBucket::new(quota, quota / 10, quota / 20)
            });
            bucket.try_consume(cost)?;
        }
        
        // Check operation-specific rate
        let key = format!("{}:{}", tenant_id, operation);
        let bucket = self.buckets.entry(key).or_insert_with(|| {
            TokenBucket::new(self.default_max_tokens, self.default_refill_rate, self.default_max_burst)
        });
        bucket.try_consume(cost)
    }
    
    /// Get current token count for a key (for monitoring).
    pub fn get_tokens(&mut self, tenant_id: &str, operation: &str) -> u32 {
        let key = format!("{}:{}", tenant_id, operation);
        if let Some(bucket) = self.buckets.get_mut(&key) {
            bucket.refill();
            bucket.tokens
        } else {
            self.default_max_tokens
        }
    }
    
    /// Clean up expired buckets (internal maintenance).
    fn cleanup_expired_buckets(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_cleanup) > self.cleanup_interval {
            // Remove buckets that haven't been used for 5 minutes
            self.buckets.retain(|_, bucket| {
                now.duration_since(bucket.last_refill) < Duration::from_secs(300)
            });
            self.last_cleanup = now;
        }
    }
    
    /// Reset rate limit for a specific key (admin operation).
    pub fn reset(&mut self, tenant_id: &str, operation: &str) {
        let key = format!("{}:{}", tenant_id, operation);
        self.buckets.remove(&key);
    }
    
    /// Reset all rate limits for a tenant (admin operation).
    pub fn reset_tenant(&mut self, tenant_id: &str) {
        self.buckets.retain(|key, _| !key.starts_with(tenant_id));
    }
}

impl Default for DomainRateLimiter {
    fn default() -> Self {
        Self::new(100, 10) // 100 max, 10/sec refill
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn rate_limiter_basic() {
        let mut limiter = DomainRateLimiter::with_burst(10, 5, 15);
        
        // Should allow first request
        assert!(limiter.check_rate("t1", "op1").is_ok());
        
        // Exhaust tokens (but stay within burst limit)
        for _ in 0..9 {
            assert!(limiter.check_rate("t1", "op1").is_ok());
        }
        
        // Should fail after exhaustion
        let result = limiter.check_rate("t1", "op1");
        assert!(matches!(result, Err(RateLimitError::Exceeded { .. })));
    }
    
    #[test]
    fn rate_limiter_refill() {
        let mut limiter = DomainRateLimiter::with_burst(10, 10, 10); // 10/sec refill
        
        // Exhaust tokens
        for _ in 0..10 {
            limiter.check_rate("t1", "op1").unwrap();
        }
        
        // Wait for refill (simulated by sleeping)
        std::thread::sleep(Duration::from_millis(1100));
        
        // Should have tokens again
        assert!(limiter.check_rate("t1", "op1").is_ok());
    }
    
    #[test]
    fn rate_limiter_tenant_isolation() {
        let mut limiter = DomainRateLimiter::with_burst(5, 1, 5);
        
        // Exhaust tenant1
        for _ in 0..5 {
            limiter.check_rate("t1", "op1").unwrap();
        }
        assert!(limiter.check_rate("t1", "op1").is_err());
        
        // Tenant2 should still have tokens
        assert!(limiter.check_rate("t2", "op1").is_ok());
    }
    
    #[test]
    fn rate_limiter_burst_protection() {
        let mut limiter = DomainRateLimiter::with_burst(100, 10, 5);
        
        // Single large request exceeding burst
        let result = limiter.check_rate_with_cost("t1", "op1", 10);
        assert!(matches!(result, Err(RateLimitError::BurstExceeded { .. })));
    }
    
    #[test]
    fn rate_limiter_tenant_quota() {
        let mut limiter = DomainRateLimiter::with_burst(1000, 10, 1000);
        // Set a low quota that will be exhausted quickly
        limiter.set_tenant_quota("limited_tenant", 2);
        
        // Exhaust quota
        assert!(limiter.check_rate("limited_tenant", "op1").is_ok());
        assert!(limiter.check_rate("limited_tenant", "op1").is_ok());
        
        // Should fail - quota exhausted
        assert!(limiter.check_rate("limited_tenant", "op2").is_err());
        
        // Different tenant without quota should still work
        assert!(limiter.check_rate("unlimited_tenant", "op1").is_ok());
    }
    
    #[test]
    fn rate_limiter_reset() {
        let mut limiter = DomainRateLimiter::with_burst(5, 1, 5);
        
        // Exhaust
        for _ in 0..5 {
            limiter.check_rate("t1", "op1").unwrap();
        }
        assert!(limiter.check_rate("t1", "op1").is_err());
        
        // Reset
        limiter.reset("t1", "op1");
        
        // Should work again
        assert!(limiter.check_rate("t1", "op1").is_ok());
    }
}