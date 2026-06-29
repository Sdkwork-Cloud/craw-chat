//! Capacity limits and resource management for production safety.
//!
//! This module provides resource tracking and capacity enforcement following
//! `OPERATIONS_SPEC.md` requirements for preventing resource exhaustion.
//!
//! ## Design Principles
//!
//! - Multi-dimensional capacity tracking (connections, memory, CPU, storage)
//! - Per-tenant resource quotas
//! - Graceful degradation under pressure
//! - Real-time monitoring integration
//!
//! ## Usage
//!
//! ```rust
//! use im_domain_core::capacity::{CapacityManager, ResourceQuota, ResourceUsage};
//!
//! let manager = CapacityManager::new(ResourceQuota::default());
//!
//! // Check before accepting new connection
//! if manager.can_allocate("tenant1", ResourceType::Connection) {
//!     manager.allocate("tenant1", ResourceType::Connection, 1);
//! }
//!
//! // Get current usage
//! let usage = manager.get_usage("tenant1");
//! ```

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Capacity management errors.
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum CapacityError {
    #[error("capacity limit exceeded: resource={resource}, current={current}, limit={limit}")]
    LimitExceeded {
        resource: String,
        current: u64,
        limit: u64,
    },
    
    #[error("tenant quota exceeded: tenant={tenant}, resource={resource}, quota={quota}")]
    TenantQuotaExceeded {
        tenant: String,
        resource: String,
        quota: u64,
    },
    
    #[error("resource allocation failed: resource={resource}, reason={reason}")]
    AllocationFailed {
        resource: String,
        reason: String,
    },
    
    #[error("resource release failed: resource={resource}, reason={reason}")]
    ReleaseFailed {
        resource: String,
        reason: String,
    },
}

/// Types of tracked resources.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    /// Active WebSocket connections.
    WebSocketConnection,
    /// Active RTC sessions.
    RtcSession,
    /// In-flight HTTP requests.
    HttpRequest,
    /// Message queue backlog.
    MessageQueue,
    /// Storage usage (bytes).
    StorageBytes,
    /// Memory usage (bytes).
    MemoryBytes,
    /// CPU usage (percentage).
    CpuPercent,
}

impl ResourceType {
    /// Get human-readable name.
    pub fn name(&self) -> &'static str {
        match self {
            ResourceType::WebSocketConnection => "ws_connections",
            ResourceType::RtcSession => "rtc_sessions",
            ResourceType::HttpRequest => "http_requests",
            ResourceType::MessageQueue => "message_queue",
            ResourceType::StorageBytes => "storage_bytes",
            ResourceType::MemoryBytes => "memory_bytes",
            ResourceType::CpuPercent => "cpu_percent",
        }
    }
}

/// Resource quota configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceQuota {
    /// Maximum WebSocket connections per tenant.
    pub max_ws_connections: u64,
    /// Maximum RTC sessions per tenant.
    pub max_rtc_sessions: u64,
    /// Maximum concurrent HTTP requests.
    pub max_http_requests: u64,
    /// Maximum message queue backlog.
    pub max_message_queue: u64,
    /// Maximum storage per tenant (bytes).
    pub max_storage_bytes: u64,
    /// Maximum memory usage (bytes).
    pub max_memory_bytes: u64,
    /// Maximum CPU usage (percentage).
    pub max_cpu_percent: u64,
    /// Global limits (across all tenants).
    pub global_limits: GlobalLimits,
}

impl Default for ResourceQuota {
    fn default() -> Self {
        Self {
            max_ws_connections: 1000,
            max_rtc_sessions: 100,
            max_http_requests: 500,
            max_message_queue: 10000,
            max_storage_bytes: 10_000_000_000, // 10 GB
            max_memory_bytes: 1_000_000_000,   // 1 GB
            max_cpu_percent: 80,
            global_limits: GlobalLimits::default(),
        }
    }
}

/// Global resource limits across all tenants.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GlobalLimits {
    pub max_total_ws_connections: u64,
    pub max_total_rtc_sessions: u64,
    pub max_total_http_requests: u64,
    pub max_total_storage_bytes: u64,
    pub max_total_memory_bytes: u64,
}

impl Default for GlobalLimits {
    fn default() -> Self {
        Self {
            max_total_ws_connections: 50_000,
            max_total_rtc_sessions: 5_000,
            max_total_http_requests: 10_000,
            max_total_storage_bytes: 500_000_000_000, // 500 GB
            max_total_memory_bytes: 32_000_000_000,   // 32 GB
        }
    }
}

/// Current resource usage for a tenant.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub ws_connections: u64,
    pub rtc_sessions: u64,
    pub http_requests: u64,
    pub message_queue: u64,
    pub storage_bytes: u64,
    pub memory_bytes: u64,
    pub cpu_percent: u64,
    pub last_updated: String,
}

/// Capacity manager for multi-tenant resource tracking.
pub struct CapacityManager {
    /// Per-tenant resource usage.
    tenant_usage: HashMap<String, ResourceUsage>,
    /// Global resource usage.
    global_usage: Arc<GlobalResourceUsage>,
    /// Quota configuration.
    quota: ResourceQuota,
    /// Cleanup interval.
    cleanup_interval: Duration,
    /// Last cleanup timestamp.
    last_cleanup: Instant,
}

/// Thread-safe global resource counters.
struct GlobalResourceUsage {
    ws_connections: AtomicU64,
    rtc_sessions: AtomicU64,
    http_requests: AtomicU64,
    storage_bytes: AtomicU64,
    memory_bytes: AtomicU64,
}

impl CapacityManager {
    /// Create new capacity manager with specified quotas.
    pub fn new(quota: ResourceQuota) -> Self {
        Self {
            tenant_usage: HashMap::new(),
            global_usage: Arc::new(GlobalResourceUsage {
                ws_connections: AtomicU64::new(0),
                rtc_sessions: AtomicU64::new(0),
                http_requests: AtomicU64::new(0),
                storage_bytes: AtomicU64::new(0),
                memory_bytes: AtomicU64::new(0),
            }),
            quota,
            cleanup_interval: Duration::from_secs(60),
            last_cleanup: Instant::now(),
        }
    }
    
    /// Create with default quotas.
    pub fn default_quotas() -> Self {
        Self::new(ResourceQuota::default())
    }
    
    /// Check if a resource can be allocated.
    pub fn can_allocate(&self, tenant_id: &str, resource_type: ResourceType, amount: u64) -> bool {
        // Check tenant quota
        let usage = self.tenant_usage.get(tenant_id).cloned().unwrap_or_default();
        if !self.check_tenant_quota(&usage, resource_type, amount) {
            return false;
        }
        
        // Check global limit
        if !self.check_global_limit(resource_type, amount) {
            return false;
        }
        
        true
    }
    
    /// Allocate resources.
    pub fn allocate(&mut self, tenant_id: &str, resource_type: ResourceType, amount: u64) -> Result<(), CapacityError> {
        self.cleanup_inactive_tenants();
        
        // Check capacity first (without borrowing)
        if !self.can_allocate(tenant_id, resource_type.clone(), amount) {
            let limit = self.get_quota_limit(&resource_type);
            
            return Err(CapacityError::TenantQuotaExceeded {
                tenant: tenant_id.to_string(),
                resource: resource_type.name().to_string(),
                quota: limit,
            });
        }
        
        // Get or create tenant usage and update in-place
        let now = chrono::Utc::now().to_rfc3339();
        self.tenant_usage.entry(tenant_id.to_string()).and_modify(|usage| {
            match &resource_type {
                ResourceType::WebSocketConnection => usage.ws_connections += amount,
                ResourceType::RtcSession => usage.rtc_sessions += amount,
                ResourceType::HttpRequest => usage.http_requests += amount,
                ResourceType::MessageQueue => usage.message_queue += amount,
                ResourceType::StorageBytes => usage.storage_bytes += amount,
                ResourceType::MemoryBytes => usage.memory_bytes += amount,
                ResourceType::CpuPercent => usage.cpu_percent = (usage.cpu_percent + amount).min(100),
            }
            usage.last_updated = now.clone();
        }).or_insert(ResourceUsage {
            ws_connections: if resource_type == ResourceType::WebSocketConnection { amount } else { 0 },
            rtc_sessions: if resource_type == ResourceType::RtcSession { amount } else { 0 },
            http_requests: if resource_type == ResourceType::HttpRequest { amount } else { 0 },
            message_queue: if resource_type == ResourceType::MessageQueue { amount } else { 0 },
            storage_bytes: if resource_type == ResourceType::StorageBytes { amount } else { 0 },
            memory_bytes: if resource_type == ResourceType::MemoryBytes { amount } else { 0 },
            cpu_percent: if resource_type == ResourceType::CpuPercent { amount.min(100) } else { 0 },
            last_updated: now.clone(),
        });
        
        // Increment global counters (after releasing tenant_usage borrow)
        self.increment_global(&resource_type, amount);
        
        Ok(())
    }
    
    /// Release resources.
    pub fn release(&mut self, tenant_id: &str, resource_type: ResourceType, amount: u64) -> Result<(), CapacityError> {
        // Check if tenant exists
        if !self.tenant_usage.contains_key(tenant_id) {
            return Err(CapacityError::ReleaseFailed {
                resource: resource_type.name().to_string(),
                reason: "tenant not found".to_string(),
            });
        }
        
        // Decrement tenant usage
        let now = chrono::Utc::now().to_rfc3339();
        if let Some(usage) = self.tenant_usage.get_mut(tenant_id) {
            match &resource_type {
                ResourceType::WebSocketConnection => usage.ws_connections = usage.ws_connections.saturating_sub(amount),
                ResourceType::RtcSession => usage.rtc_sessions = usage.rtc_sessions.saturating_sub(amount),
                ResourceType::HttpRequest => usage.http_requests = usage.http_requests.saturating_sub(amount),
                ResourceType::MessageQueue => usage.message_queue = usage.message_queue.saturating_sub(amount),
                ResourceType::StorageBytes => usage.storage_bytes = usage.storage_bytes.saturating_sub(amount),
                ResourceType::MemoryBytes => usage.memory_bytes = usage.memory_bytes.saturating_sub(amount),
                ResourceType::CpuPercent => usage.cpu_percent = usage.cpu_percent.saturating_sub(amount),
            }
            usage.last_updated = now;
        }
        
        // Decrement global counters
        self.decrement_global(&resource_type, amount);
        
        Ok(())
    }
    
    /// Get current usage for a tenant.
    pub fn get_usage(&self, tenant_id: &str) -> ResourceUsage {
        self.tenant_usage.get(tenant_id).cloned().unwrap_or_default()
    }
    
    /// Get global usage.
    pub fn get_global_usage(&self) -> HashMap<String, u64> {
        let mut usage = HashMap::new();
        usage.insert("ws_connections".to_string(), self.global_usage.ws_connections.load(Ordering::SeqCst));
        usage.insert("rtc_sessions".to_string(), self.global_usage.rtc_sessions.load(Ordering::SeqCst));
        usage.insert("http_requests".to_string(), self.global_usage.http_requests.load(Ordering::SeqCst));
        usage.insert("storage_bytes".to_string(), self.global_usage.storage_bytes.load(Ordering::SeqCst));
        usage.insert("memory_bytes".to_string(), self.global_usage.memory_bytes.load(Ordering::SeqCst));
        usage
    }
    
    /// Get capacity status for monitoring.
    pub fn get_capacity_status(&self) -> CapacityStatus {
        let total_tenants = self.tenant_usage.len();
        let global = self.get_global_usage();
        
        CapacityStatus {
            total_tenants,
            global_usage: global.clone(),
            global_limits: self.quota.global_limits.clone(),
            utilization: self.calculate_utilization(&global),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
    
    /// Check tenant quota.
    fn check_tenant_quota(&self, usage: &ResourceUsage, resource_type: ResourceType, amount: u64) -> bool {
        let current = self.get_usage_value(usage, &resource_type);
        let limit = self.get_quota_limit(&resource_type);
        current + amount <= limit
    }
    
    /// Check global limit.
    fn check_global_limit(&self, resource_type: ResourceType, amount: u64) -> bool {
        match resource_type {
            ResourceType::WebSocketConnection => {
                self.global_usage.ws_connections.load(Ordering::SeqCst) + amount <= self.quota.global_limits.max_total_ws_connections
            }
            ResourceType::RtcSession => {
                self.global_usage.rtc_sessions.load(Ordering::SeqCst) + amount <= self.quota.global_limits.max_total_rtc_sessions
            }
            ResourceType::HttpRequest => {
                self.global_usage.http_requests.load(Ordering::SeqCst) + amount <= self.quota.global_limits.max_total_http_requests
            }
            ResourceType::StorageBytes => {
                self.global_usage.storage_bytes.load(Ordering::SeqCst) + amount <= self.quota.global_limits.max_total_storage_bytes
            }
            ResourceType::MemoryBytes => {
                self.global_usage.memory_bytes.load(Ordering::SeqCst) + amount <= self.quota.global_limits.max_total_memory_bytes
            }
            _ => true, // Message queue and CPU are tenant-only
        }
    }
    
    /// Get usage value by resource type.
    fn get_usage_value(&self, usage: &ResourceUsage, resource_type: &ResourceType) -> u64 {
        match resource_type {
            ResourceType::WebSocketConnection => usage.ws_connections,
            ResourceType::RtcSession => usage.rtc_sessions,
            ResourceType::HttpRequest => usage.http_requests,
            ResourceType::MessageQueue => usage.message_queue,
            ResourceType::StorageBytes => usage.storage_bytes,
            ResourceType::MemoryBytes => usage.memory_bytes,
            ResourceType::CpuPercent => usage.cpu_percent,
        }
    }
    
    /// Get quota limit by resource type.
    fn get_quota_limit(&self, resource_type: &ResourceType) -> u64 {
        match resource_type {
            ResourceType::WebSocketConnection => self.quota.max_ws_connections,
            ResourceType::RtcSession => self.quota.max_rtc_sessions,
            ResourceType::HttpRequest => self.quota.max_http_requests,
            ResourceType::MessageQueue => self.quota.max_message_queue,
            ResourceType::StorageBytes => self.quota.max_storage_bytes,
            ResourceType::MemoryBytes => self.quota.max_memory_bytes,
            ResourceType::CpuPercent => self.quota.max_cpu_percent,
        }
    }
    
    /// Increment global counter.
    fn increment_global(&self, resource_type: &ResourceType, amount: u64) {
        match resource_type {
            ResourceType::WebSocketConnection => self.global_usage.ws_connections.fetch_add(amount, Ordering::SeqCst),
            ResourceType::RtcSession => self.global_usage.rtc_sessions.fetch_add(amount, Ordering::SeqCst),
            ResourceType::HttpRequest => self.global_usage.http_requests.fetch_add(amount, Ordering::SeqCst),
            ResourceType::StorageBytes => self.global_usage.storage_bytes.fetch_add(amount, Ordering::SeqCst),
            ResourceType::MemoryBytes => self.global_usage.memory_bytes.fetch_add(amount, Ordering::SeqCst),
            _ => 0,
        };
    }
    
    /// Decrement global counter.
    fn decrement_global(&self, resource_type: &ResourceType, amount: u64) {
        match resource_type {
            ResourceType::WebSocketConnection => self.global_usage.ws_connections.fetch_sub(amount, Ordering::SeqCst),
            ResourceType::RtcSession => self.global_usage.rtc_sessions.fetch_sub(amount, Ordering::SeqCst),
            ResourceType::HttpRequest => self.global_usage.http_requests.fetch_sub(amount, Ordering::SeqCst),
            ResourceType::StorageBytes => self.global_usage.storage_bytes.fetch_sub(amount, Ordering::SeqCst),
            ResourceType::MemoryBytes => self.global_usage.memory_bytes.fetch_sub(amount, Ordering::SeqCst),
            _ => 0,
        };
    }
    
    /// Calculate utilization percentages.
    fn calculate_utilization(&self, global: &HashMap<String, u64>) -> HashMap<String, f64> {
        let mut utilization = HashMap::new();
        
        let ws = global.get("ws_connections").unwrap_or(&0);
        utilization.insert("ws_connections".to_string(), (*ws as f64 / self.quota.global_limits.max_total_ws_connections as f64) * 100.0);
        
        let rtc = global.get("rtc_sessions").unwrap_or(&0);
        utilization.insert("rtc_sessions".to_string(), (*rtc as f64 / self.quota.global_limits.max_total_rtc_sessions as f64) * 100.0);
        
        let http = global.get("http_requests").unwrap_or(&0);
        utilization.insert("http_requests".to_string(), (*http as f64 / self.quota.global_limits.max_total_http_requests as f64) * 100.0);
        
        let storage = global.get("storage_bytes").unwrap_or(&0);
        utilization.insert("storage_bytes".to_string(), (*storage as f64 / self.quota.global_limits.max_total_storage_bytes as f64) * 100.0);
        
        let memory = global.get("memory_bytes").unwrap_or(&0);
        utilization.insert("memory_bytes".to_string(), (*memory as f64 / self.quota.global_limits.max_total_memory_bytes as f64) * 100.0);
        
        utilization
    }
    
    /// Clean up inactive tenants.
    fn cleanup_inactive_tenants(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_cleanup) > self.cleanup_interval {
            let cutoff = chrono::Utc::now() - chrono::Duration::seconds(300);
            let cutoff_str = cutoff.to_rfc3339();
            
            self.tenant_usage.retain(|_, usage| {
                usage.last_updated > cutoff_str
            });
            
            self.last_cleanup = now;
        }
    }
}

/// Capacity status for monitoring.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapacityStatus {
    pub total_tenants: usize,
    pub global_usage: HashMap<String, u64>,
    pub global_limits: GlobalLimits,
    pub utilization: HashMap<String, f64>,
    pub timestamp: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn resource_type_names() {
        assert_eq!(ResourceType::WebSocketConnection.name(), "ws_connections");
        assert_eq!(ResourceType::RtcSession.name(), "rtc_sessions");
    }
    
    #[test]
    fn capacity_manager_initial_state() {
        let manager = CapacityManager::default_quotas();
        let usage = manager.get_usage("tenant1");
        assert_eq!(usage.ws_connections, 0);
        assert_eq!(usage.rtc_sessions, 0);
    }
    
    #[test]
    fn capacity_allocate_success() {
        let mut manager = CapacityManager::default_quotas();
        
        let result = manager.allocate("t1", ResourceType::WebSocketConnection, 10);
        assert!(result.is_ok());
        
        let usage = manager.get_usage("t1");
        assert_eq!(usage.ws_connections, 10);
    }
    
    #[test]
    fn capacity_allocate_exceeds_quota() {
        let quota = ResourceQuota {
            max_ws_connections: 5,
            ..ResourceQuota::default()
        };
        let mut manager = CapacityManager::new(quota);
        
        // Allocate up to limit
        manager.allocate("t1", ResourceType::WebSocketConnection, 5).unwrap();
        
        // Should fail
        let result = manager.allocate("t1", ResourceType::WebSocketConnection, 1);
        assert!(matches!(result, Err(CapacityError::TenantQuotaExceeded { .. })));
    }
    
    #[test]
    fn capacity_release() {
        let mut manager = CapacityManager::default_quotas();
        
        manager.allocate("t1", ResourceType::WebSocketConnection, 10).unwrap();
        manager.release("t1", ResourceType::WebSocketConnection, 5).unwrap();
        
        let usage = manager.get_usage("t1");
        assert_eq!(usage.ws_connections, 5);
    }
    
    #[test]
    fn capacity_tenant_isolation() {
        let mut manager = CapacityManager::default_quotas();
        
        // Tenant 1 uses resources
        manager.allocate("t1", ResourceType::WebSocketConnection, 50).unwrap();
        
        // Tenant 2 should still have quota
        assert!(manager.can_allocate("t2", ResourceType::WebSocketConnection, 50));
    }
    
    #[test]
    fn capacity_global_limit() {
        let quota = ResourceQuota {
            global_limits: GlobalLimits {
                max_total_ws_connections: 100,
                ..GlobalLimits::default()
            },
            max_ws_connections: 1000, // Per-tenant limit higher than global
            ..ResourceQuota::default()
        };
        let mut manager = CapacityManager::new(quota);
        
        // Exhaust global limit
        manager.allocate("t1", ResourceType::WebSocketConnection, 50).unwrap();
        manager.allocate("t2", ResourceType::WebSocketConnection, 50).unwrap();
        
        // Should fail at global limit
        let result = manager.allocate("t3", ResourceType::WebSocketConnection, 1);
        assert!(matches!(result, Err(CapacityError::TenantQuotaExceeded { .. })));
    }
    
    #[test]
    fn capacity_status() {
        let mut manager = CapacityManager::default_quotas();
        
        manager.allocate("t1", ResourceType::WebSocketConnection, 10).unwrap();
        manager.allocate("t2", ResourceType::RtcSession, 5).unwrap();
        
        let status = manager.get_capacity_status();
        assert_eq!(status.total_tenants, 2);
        assert!(status.utilization.contains_key("ws_connections"));
    }
    
    #[test]
    fn capacity_can_allocate_check() {
        let quota = ResourceQuota {
            max_ws_connections: 10,
            ..ResourceQuota::default()
        };
        let mut manager = CapacityManager::new(quota);
        
        // Check before allocation
        assert!(manager.can_allocate("t1", ResourceType::WebSocketConnection, 10));
        
        // Allocate
        manager.allocate("t1", ResourceType::WebSocketConnection, 10).unwrap();
        
        // Check should fail now
        assert!(!manager.can_allocate("t1", ResourceType::WebSocketConnection, 1));
    }
}