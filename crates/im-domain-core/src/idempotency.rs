//! Idempotency key management for ensuring exactly-once processing semantics.
//!
//! This module provides idempotency protection following `SECURITY_SPEC.md` requirements
//! for preventing duplicate request processing in distributed systems.
//!
//! ## Design Principles
//!
//! - Unique idempotency keys per tenant+organization
//! - Configurable TTL for automatic cleanup
//! - Atomic check-and-set semantics
//! - Distributed lock integration for multi-instance deployments
//!
//! ## Usage
//!
//! ```rust
//! use im_domain_core::idempotency::{IdempotencyGuard, IdempotencyKey};
//!
//! let guard = IdempotencyGuard::new(Duration::from_secs(300));
//! let key = IdempotencyKey::new("tenant1", "org1", "req-12345");
//! 
//! // Check if request was already processed
//! if guard.check_and_reserve(&key)? {
//!     // Process request
//!     let result = do_work();
//!     guard.complete(&key, result)?;
//! } else {
//!     // Return cached response
//!     return guard.get_cached_response(&key);
//! }
//! ```

use std::collections::HashMap;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Idempotency errors.
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum IdempotencyError {
    #[error("idempotency key already in use: key={key}, locked_at={locked_at}")]
    KeyInUse {
        key: String,
        locked_at: String,
    },
    
    #[error("idempotency key expired: key={key}")]
    KeyExpired {
        key: String,
    },
    
    #[error("idempotency key not found: key={key}")]
    KeyNotFound {
        key: String,
    },
    
    #[error("lock acquisition failed: key={key}, retry_after={retry_after_ms}ms")]
    LockFailed {
        key: String,
        retry_after_ms: u64,
    },
    
    #[error("distributed lock error: {message}")]
    DistributedLockError {
        message: String,
    },
}

/// Unique identifier for an idempotent operation.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdempotencyKey {
    /// Tenant identifier for multi-tenancy isolation.
    pub tenant_id: String,
    /// Organization identifier for organization-level isolation.
    pub organization_id: String,
    /// Unique request identifier (client-provided).
    pub request_id: String,
}

impl IdempotencyKey {
    /// Create a new idempotency key.
    pub fn new(tenant_id: &str, organization_id: &str, request_id: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_string(),
            organization_id: organization_id.to_string(),
            request_id: request_id.to_string(),
        }
    }
    
    /// Convert to a composite string key for storage.
    pub fn to_storage_key(&self) -> String {
        format!("{}:{}:{}", self.tenant_id, self.organization_id, self.request_id)
    }
    
    /// Parse from a composite string key.
    pub fn from_storage_key(key: &str) -> Option<Self> {
        let parts: Vec<&str> = key.splitn(3, ':').collect();
        if parts.len() != 3 {
            return None;
        }
        Some(Self {
            tenant_id: parts[0].to_string(),
            organization_id: parts[1].to_string(),
            request_id: parts[2].to_string(),
        })
    }
}

/// State of an idempotent operation.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum IdempotencyState {
    /// Key is reserved but processing not yet complete.
    Reserved {
        reserved_at: String,
        expires_at: String,
    },
    /// Processing completed successfully.
    Completed {
        completed_at: String,
        response_hash: String,
    },
    /// Processing failed, key can be reused.
    Failed {
        failed_at: String,
        error_message: String,
    },
}

/// Record for tracking idempotent operations.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdempotencyRecord {
    /// The idempotency key.
    pub key: IdempotencyKey,
    /// Current state.
    pub state: IdempotencyState,
    /// Cached response (for completed operations).
    pub cached_response: Option<String>,
    /// Created timestamp.
    pub created_at: String,
    /// Last updated timestamp.
    pub updated_at: String,
}

/// Guard for managing idempotent operations.
///
/// Provides in-memory idempotency tracking with TTL-based cleanup.
/// For distributed deployments, integrate with Redis or database-backed storage.
#[derive(Clone, Debug)]
pub struct IdempotencyGuard {
    records: HashMap<String, IdempotencyRecord>,
    default_ttl: Duration,
    lock_timeout: Duration,
    cleanup_interval: Duration,
    last_cleanup: Instant,
}

impl IdempotencyGuard {
    /// Create a new idempotency guard with specified TTL.
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            records: HashMap::new(),
            default_ttl,
            lock_timeout: Duration::from_secs(30),
            cleanup_interval: Duration::from_secs(60),
            last_cleanup: Instant::now(),
        }
    }
    
    /// Create with custom lock timeout.
    pub fn with_lock_timeout(default_ttl: Duration, lock_timeout: Duration) -> Self {
        Self {
            records: HashMap::new(),
            default_ttl,
            lock_timeout,
            cleanup_interval: Duration::from_secs(60),
            last_cleanup: Instant::now(),
        }
    }
    
    /// Check if a key is available and reserve it atomically.
    ///
    /// Returns `true` if the key was successfully reserved (processing should proceed).
    /// Returns `false` if the key is already in use or completed.
    pub fn check_and_reserve(&mut self, key: &IdempotencyKey) -> Result<bool, IdempotencyError> {
        self.cleanup_expired();
        
        let storage_key = key.to_storage_key();
        let now = chrono::Utc::now().to_rfc3339();
        let expires_at = (chrono::Utc::now() + chrono::Duration::from_std(self.default_ttl).unwrap()).to_rfc3339();
        
        if let Some(record) = self.records.get(&storage_key) {
            match &record.state {
                IdempotencyState::Reserved { reserved_at, .. } => {
                    // Key is locked - check if lock has expired based on lock_timeout
                    if let Ok(reserved_time) = chrono::DateTime::parse_from_rfc3339(reserved_at) {
                        let elapsed = chrono::Utc::now() - reserved_time.with_timezone(&chrono::Utc);
                        if elapsed > chrono::Duration::from_std(self.lock_timeout).unwrap_or_else(|_| chrono::Duration::seconds(30)) {
                            // Lock has expired - allow retry by removing the stale reservation
                            self.records.remove(&storage_key);
                        } else {
                            // Lock is still valid
                            return Err(IdempotencyError::KeyInUse {
                                key: storage_key,
                                locked_at: reserved_at.clone(),
                            });
                        }
                    } else {
                        // Invalid timestamp - allow retry
                        self.records.remove(&storage_key);
                    }
                }
                IdempotencyState::Completed { .. } => {
                    // Already processed - return cached result
                    return Ok(false);
                }
                IdempotencyState::Failed { .. } => {
                    // Previous attempt failed - allow retry
                }
            }
        }
        
        // Reserve the key
        let record = IdempotencyRecord {
            key: key.clone(),
            state: IdempotencyState::Reserved {
                reserved_at: now.clone(),
                expires_at,
            },
            cached_response: None,
            created_at: now.clone(),
            updated_at: now,
        };
        
        self.records.insert(storage_key, record);
        Ok(true)
    }
    
    /// Mark an operation as completed with a cached response.
    pub fn complete(&mut self, key: &IdempotencyKey, response: Option<String>) -> Result<(), IdempotencyError> {
        let storage_key = key.to_storage_key();
        let now = chrono::Utc::now().to_rfc3339();
        
        let record = self.records.get_mut(&storage_key).ok_or_else(|| IdempotencyError::KeyNotFound {
            key: storage_key.clone(),
        })?;
        
        // Compute response hash for verification
        let response_hash = response.as_ref().map(|r| {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            r.hash(&mut hasher);
            format!("{:x}", hasher.finish())
        }).unwrap_or_default();
        
        record.state = IdempotencyState::Completed {
            completed_at: now.clone(),
            response_hash,
        };
        record.cached_response = response;
        record.updated_at = now;
        
        Ok(())
    }
    
    /// Mark an operation as failed, allowing retry.
    pub fn fail(&mut self, key: &IdempotencyKey, error_message: &str) -> Result<(), IdempotencyError> {
        let storage_key = key.to_storage_key();
        let now = chrono::Utc::now().to_rfc3339();
        
        let record = self.records.get_mut(&storage_key).ok_or_else(|| IdempotencyError::KeyNotFound {
            key: storage_key.clone(),
        })?;
        
        record.state = IdempotencyState::Failed {
            failed_at: now.clone(),
            error_message: error_message.to_string(),
        };
        record.updated_at = now;
        
        Ok(())
    }
    
    /// Get cached response for a completed operation.
    pub fn get_cached_response(&self, key: &IdempotencyKey) -> Option<String> {
        let storage_key = key.to_storage_key();
        self.records.get(&storage_key).and_then(|r| r.cached_response.clone())
    }
    
    /// Check if a key exists and is completed.
    pub fn is_completed(&self, key: &IdempotencyKey) -> bool {
        let storage_key = key.to_storage_key();
        matches!(
            self.records.get(&storage_key).map(|r| &r.state),
            Some(IdempotencyState::Completed { .. })
        )
    }
    
    /// Release a reserved key (for cleanup after timeout).
    pub fn release(&mut self, key: &IdempotencyKey) -> Result<(), IdempotencyError> {
        let storage_key = key.to_storage_key();
        if self.records.remove(&storage_key).is_some() {
            Ok(())
        } else {
            Err(IdempotencyError::KeyNotFound { key: storage_key })
        }
    }
    
    /// Clean up expired records.
    fn cleanup_expired(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_cleanup) > self.cleanup_interval {
            let now_str = chrono::Utc::now().to_rfc3339();
            self.records.retain(|_, record| {
                match &record.state {
                    IdempotencyState::Reserved { expires_at, .. } => {
                        // Keep if not expired
                        expires_at > &now_str
                    }
                    IdempotencyState::Completed { completed_at, .. } => {
                        // Keep completed records for their TTL
                        completed_at > &now_str
                    }
                    IdempotencyState::Failed { .. } => {
                        // Remove failed records quickly
                        false
                    }
                }
            });
            self.last_cleanup = now;
        }
    }
    
    /// Get the count of tracked keys (for monitoring).
    pub fn key_count(&self) -> usize {
        self.records.len()
    }
    
    /// Get statistics about idempotency tracking.
    pub fn stats(&self) -> IdempotencyStats {
        let mut reserved = 0;
        let mut completed = 0;
        let mut failed = 0;
        
        for record in self.records.values() {
            match &record.state {
                IdempotencyState::Reserved { .. } => reserved += 1,
                IdempotencyState::Completed { .. } => completed += 1,
                IdempotencyState::Failed { .. } => failed += 1,
            }
        }
        
        IdempotencyStats {
            total_keys: self.records.len(),
            reserved,
            completed,
            failed,
        }
    }
}

impl Default for IdempotencyGuard {
    fn default() -> Self {
        Self::new(Duration::from_secs(300)) // 5 minutes default TTL
    }
}

/// Statistics about idempotency tracking.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdempotencyStats {
    pub total_keys: usize,
    pub reserved: usize,
    pub completed: usize,
    pub failed: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn idempotency_key_serialization() {
        let key = IdempotencyKey::new("tenant1", "org1", "req-123");
        let storage = key.to_storage_key();
        assert_eq!(storage, "tenant1:org1:req-123");
        
        let parsed = IdempotencyKey::from_storage_key(&storage).unwrap();
        assert_eq!(parsed, key);
    }
    
    #[test]
    fn idempotency_check_and_reserve() {
        let mut guard = IdempotencyGuard::new(Duration::from_secs(60));
        let key = IdempotencyKey::new("t1", "o1", "req-1");
        
        // First call should succeed
        assert!(guard.check_and_reserve(&key).unwrap());
        
        // Second call should fail (key in use)
        assert!(matches!(
            guard.check_and_reserve(&key),
            Err(IdempotencyError::KeyInUse { .. })
        ));
    }
    
    #[test]
    fn idempotency_complete() {
        let mut guard = IdempotencyGuard::new(Duration::from_secs(60));
        let key = IdempotencyKey::new("t1", "o1", "req-2");
        
        // Reserve and complete
        guard.check_and_reserve(&key).unwrap();
        guard.complete(&key, Some("response".to_string())).unwrap();
        
        // Check cached response
        assert_eq!(guard.get_cached_response(&key), Some("response".to_string()));
        
        // Should be completed
        assert!(guard.is_completed(&key));
    }
    
    #[test]
    fn idempotency_completed_blocks_retry() {
        let mut guard = IdempotencyGuard::new(Duration::from_secs(60));
        let key = IdempotencyKey::new("t1", "o1", "req-3");
        
        // Reserve and complete
        guard.check_and_reserve(&key).unwrap();
        guard.complete(&key, Some("response".to_string())).unwrap();
        
        // Check should return false (already completed)
        assert!(!guard.check_and_reserve(&key).unwrap());
    }
    
    #[test]
    fn idempotency_fail_allows_retry() {
        let mut guard = IdempotencyGuard::new(Duration::from_secs(60));
        let key = IdempotencyKey::new("t1", "o1", "req-4");
        
        // Reserve and fail
        guard.check_and_reserve(&key).unwrap();
        guard.fail(&key, "something went wrong").unwrap();
        
        // Should allow retry (cleanup will remove failed records)
        // For immediate retry, we need to remove the failed record first
        guard.release(&key).ok();
        assert!(guard.check_and_reserve(&key).unwrap());
    }
    
    #[test]
    fn idempotency_tenant_isolation() {
        let mut guard = IdempotencyGuard::new(Duration::from_secs(60));
        let key1 = IdempotencyKey::new("t1", "o1", "req-5");
        let key2 = IdempotencyKey::new("t2", "o1", "req-5"); // Different tenant
        
        // Both should succeed (different tenants)
        assert!(guard.check_and_reserve(&key1).unwrap());
        assert!(guard.check_and_reserve(&key2).unwrap());
    }
    
    #[test]
    fn idempotency_stats() {
        let mut guard = IdempotencyGuard::new(Duration::from_secs(60));
        
        let key1 = IdempotencyKey::new("t1", "o1", "req-6");
        let key2 = IdempotencyKey::new("t1", "o1", "req-7");
        let key3 = IdempotencyKey::new("t1", "o1", "req-8");
        
        guard.check_and_reserve(&key1).unwrap();
        guard.check_and_reserve(&key2).unwrap();
        guard.check_and_reserve(&key3).unwrap();
        
        guard.complete(&key1, None).unwrap();
        guard.fail(&key2, "error").unwrap();
        // key3 remains reserved
        
        let stats = guard.stats();
        assert_eq!(stats.total_keys, 3);
        assert_eq!(stats.reserved, 1);
        assert_eq!(stats.completed, 1);
        assert_eq!(stats.failed, 1);
    }
}