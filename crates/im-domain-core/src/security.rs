//! Security validation utilities for SDKWork IM service.
//!
//! This module provides security validation following `SECURITY_SPEC.md` and
//! `IAM_SPEC.md` requirements for enterprise-grade compliance.
//!
//! ## Tenant Isolation
//!
//! All data access MUST validate tenant context consistency:
//!
//! ```rust
//! use im_domain_core::security::TenantIsolationValidator;
//!
//! let validator = TenantIsolationValidator::new();
//! validator.validate_access(&request_context, resource_tenant_id)?;
//! ```
//!
//! ## Security Event Emission
//!
//! All security-sensitive operations MUST emit audit events:
//!
//! - Login attempts (success/failure)
//! - Token validation failures
//! - Permission denied events
//! - Cross-tenant access attempts

use std::collections::HashMap;
use std::time::{Duration, Instant};

use dashmap::DashMap;
use serde::{Deserialize, Serialize};

/// Tenant isolation validation error.
#[derive(Clone, Debug, thiserror::Error)]
pub enum TenantIsolationError {
    #[error("Tenant mismatch: request tenant '{request}' does not match resource tenant '{resource}'")]
    TenantMismatch { request: String, resource: String },
    
    #[error("Missing tenant context in request")]
    MissingTenantContext,
    
    #[error("Cross-tenant access denied without platform permission")]
    CrossTenantAccessDenied,
    
    #[error("Organization mismatch: request org '{request}' does not match resource org '{resource}'")]
    OrganizationMismatch { request: String, resource: String },
}

/// Permission validation error.
#[derive(Clone, Debug, thiserror::Error)]
pub enum PermissionError {
    #[error("Permission denied: required '{permission}' not granted")]
    PermissionDenied { permission: String },
    
    #[error("Role '{role}' does not have permission '{permission}'")]
    RolePermissionMissing { role: String, permission: String },
    
    #[error("Data scope mismatch: request scope '{request}' exceeds allowed scope '{allowed}'")]
    DataScopeMismatch { request: String, allowed: String },
}

/// Request context for security validation.
#[derive(Clone, Debug)]
pub struct SecurityContext {
    /// Tenant ID from verified token.
    pub tenant_id: String,
    /// Organization ID from verified token.
    pub organization_id: String,
    /// Login scope (TENANT or ORGANIZATION).
    pub login_scope: String,
    /// User ID from verified token.
    pub user_id: Option<String>,
    /// Session ID from verified token.
    pub session_id: Option<String>,
    /// Actor ID (user or service).
    pub actor_id: String,
    /// Actor kind (USER, SYSTEM, SERVICE).
    pub actor_kind: String,
    /// Permission scope from token.
    pub permission_scope: Vec<String>,
    /// Data scope from token.
    pub data_scope: String,
    /// Auth level (password, mfa, etc).
    pub auth_level: String,
    /// Request ID for correlation.
    pub request_id: Option<String>,
    /// Trace ID for distributed tracing.
    pub trace_id: Option<String>,
}

impl SecurityContext {
    /// Check if this is a tenant-level session.
    pub fn is_tenant_scope(&self) -> bool {
        self.login_scope == "TENANT" || self.organization_id == "0"
    }
    
    /// Check if this is an organization-level session.
    pub fn is_organization_scope(&self) -> bool {
        self.login_scope == "ORGANIZATION" && self.organization_id != "0"
    }
    
    /// Check if user has a specific permission.
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permission_scope.iter().any(|p| p == permission || p == "*")
    }
    
    /// Check if this is a platform/admin context.
    pub fn is_platform_admin(&self) -> bool {
        self.has_permission("platform.*") || self.has_permission("platform.admin")
    }
}

/// Tenant isolation validator.
#[derive(Clone, Debug)]
pub struct TenantIsolationValidator {
    /// Enable strict mode (reject all cross-tenant access).
    strict_mode: bool,
    /// Allowed cross-tenant permissions.
    cross_tenant_permissions: Vec<String>,
}

impl TenantIsolationValidator {
    pub fn new() -> Self {
        Self {
            strict_mode: true,
            cross_tenant_permissions: vec![
                "platform.tenants.read".to_string(),
                "platform.tenants.write".to_string(),
                "platform.admin".to_string(),
            ],
        }
    }
    
    /// Create validator with relaxed mode for specific scenarios.
    pub fn relaxed() -> Self {
        Self {
            strict_mode: false,
            cross_tenant_permissions: vec![],
        }
    }
    
    /// Validate tenant access for a resource.
    pub fn validate_access(
        &self,
        context: &SecurityContext,
        resource_tenant_id: &str,
    ) -> Result<(), TenantIsolationError> {
        // Check tenant match
        if context.tenant_id != resource_tenant_id {
            // Allow cross-tenant only with explicit permission
            if !self.is_cross_tenant_allowed(context) {
                return Err(TenantIsolationError::TenantMismatch {
                    request: context.tenant_id.clone(),
                    resource: resource_tenant_id.to_string(),
                });
            }
        }
        
        Ok(())
    }
    
    /// Validate organization access for a resource.
    pub fn validate_organization_access(
        &self,
        context: &SecurityContext,
        resource_organization_id: &str,
    ) -> Result<(), TenantIsolationError> {
        // Tenant-level sessions can access tenant-level resources (org = 0)
        if context.is_tenant_scope() && resource_organization_id == "0" {
            return Ok(());
        }
        
        // Organization-level sessions must match
        if context.is_organization_scope() {
            if context.organization_id != resource_organization_id {
                return Err(TenantIsolationError::OrganizationMismatch {
                    request: context.organization_id.clone(),
                    resource: resource_organization_id.to_string(),
                });
            }
        }
        
        Ok(())
    }
    
    /// Validate combined tenant and organization access.
    pub fn validate_full_access(
        &self,
        context: &SecurityContext,
        resource_tenant_id: &str,
        resource_organization_id: &str,
    ) -> Result<(), TenantIsolationError> {
        self.validate_access(context, resource_tenant_id)?;
        self.validate_organization_access(context, resource_organization_id)?;
        Ok(())
    }
    
    /// Check if cross-tenant access is allowed for this context.
    fn is_cross_tenant_allowed(&self, context: &SecurityContext) -> bool {
        if self.strict_mode {
            // In strict mode, only allow with explicit platform permission
            context.is_platform_admin()
        } else {
            // In relaxed mode, check against allowed permissions list
            self.cross_tenant_permissions
                .iter()
                .any(|p| context.has_permission(p))
        }
    }
}

impl Default for TenantIsolationValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Permission validator.
#[derive(Clone, Debug)]
pub struct PermissionValidator {
    /// Required permissions for operations.
    operation_permissions: HashMap<String, Vec<String>>,
}

impl PermissionValidator {
    pub fn new() -> Self {
        Self {
            operation_permissions: Self::default_operation_permissions(),
        }
    }
    
    /// Get default operation permission mapping.
    fn default_operation_permissions() -> HashMap<String, Vec<String>> {
        HashMap::from([
            ("rtc.session.create".to_string(), vec!["rtc.sessions.write".to_string()]),
            ("rtc.session.read".to_string(), vec!["rtc.sessions.read".to_string()]),
            ("rtc.session.update".to_string(), vec!["rtc.sessions.write".to_string()]),
            ("rtc.session.delete".to_string(), vec!["rtc.sessions.write".to_string()]),
            ("rtc.signal.post".to_string(), vec!["rtc.signals.write".to_string()]),
            ("rtc.signal.list".to_string(), vec!["rtc.signals.read".to_string()]),
            ("rtc.participant.invite".to_string(), vec!["rtc.participants.write".to_string()]),
            ("rtc.participant.accept".to_string(), vec!["rtc.participants.write".to_string()]),
            ("rtc.participant.reject".to_string(), vec!["rtc.participants.write".to_string()]),
        ])
    }
    
    /// Validate permission for an operation.
    pub fn validate_operation(
        &self,
        context: &SecurityContext,
        operation: &str,
    ) -> Result<(), PermissionError> {
        let required = self.operation_permissions.get(operation);
        
        if let Some(permissions) = required {
            for permission in permissions {
                if !context.has_permission(permission) {
                    return Err(PermissionError::PermissionDenied {
                        permission: permission.to_string(),
                    });
                }
            }
        }
        
        Ok(())
    }
    
    /// Check if context has permission for operation (without error).
    pub fn has_operation_permission(&self, context: &SecurityContext, operation: &str) -> bool {
        self.validate_operation(context, operation).is_ok()
    }
}

impl Default for PermissionValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Signal replay attack protection.
/// 
/// Prevents acceptance of signals with stale or duplicate sequence numbers.
#[derive(Clone, Debug)]
pub struct SignalReplayProtector {
    /// Last signal sequence per session.
    last_sequences: DashMap<String, u64>,
    /// Signal hash cache for deduplication.
    signal_hashes: DashMap<String, Instant>,
    /// Maximum age for signal hash entries.
    max_hash_age: Duration,
}

impl SignalReplayProtector {
    pub fn new() -> Self {
        Self {
            last_sequences: DashMap::new(),
            signal_hashes: DashMap::new(),
            max_hash_age: Duration::from_secs(300), // 5 minutes
        }
    }
    
    /// Validate signal sequence is not stale or duplicate.
    pub fn validate_signal_sequence(
        &self,
        rtc_session_id: &str,
        signal_seq: u64,
        signal_hash: &str,
    ) -> Result<(), SignalReplayError> {
        // Check sequence monotonicity
        let last_seq = self.last_sequences
            .get(rtc_session_id)
            .map(|entry| *entry.value())
            .unwrap_or(0);
        
        if signal_seq <= last_seq {
            return Err(SignalReplayError::StaleSequence {
                provided: signal_seq,
                expected_min: last_seq + 1,
            });
        }
        
        // Check for duplicate signal hash (replay detection)
        let hash_key = format!("{}:{}", rtc_session_id, signal_hash);
        if let Some(entry) = self.signal_hashes.get(&hash_key) {
            let age = Instant::now().duration_since(*entry.value());
            if age < self.max_hash_age {
                return Err(SignalReplayError::DuplicateSignal {
                    hash: signal_hash.to_string(),
                });
            }
        }
        
        // Update sequence and hash cache
        self.last_sequences.insert(rtc_session_id.to_string(), signal_seq);
        self.signal_hashes.insert(hash_key, Instant::now());
        
        Ok(())
    }
    
    /// Update sequence after successful signal processing.
    pub fn record_signal_processed(&self, rtc_session_id: &str, signal_seq: u64) {
        self.last_sequences.insert(rtc_session_id.to_string(), signal_seq);
    }
    
    /// Clear session state after session ends.
    pub fn clear_session(&self, rtc_session_id: &str) {
        self.last_sequences.remove(rtc_session_id);
        // Remove all signal hashes for this session
        self.signal_hashes.retain(|key, _| !key.starts_with(rtc_session_id));
    }
    
    /// Cleanup expired signal hash entries.
    pub fn cleanup_expired(&self) {
        let now = Instant::now();
        self.signal_hashes.retain(|_, instant| {
            now.duration_since(*instant) < self.max_hash_age
        });
    }
}

impl Default for SignalReplayProtector {
    fn default() -> Self {
        Self::new()
    }
}

/// Signal replay error.
#[derive(Clone, Debug, thiserror::Error)]
pub enum SignalReplayError {
    #[error("Stale signal sequence: provided {provided}, expected at least {expected_min}")]
    StaleSequence { provided: u64, expected_min: u64 },
    
    #[error("Duplicate signal detected with hash '{hash}'")]
    DuplicateSignal { hash: String },
}

/// Rate limit key generator for RTC operations.
#[derive(Clone, Debug)]
pub struct RateLimitKeyGenerator;

impl RateLimitKeyGenerator {
    /// Generate rate limit key for session creation.
    pub fn session_create(tenant_id: &str, user_id: &str) -> String {
        format!("rtc:create:{}:{}", tenant_id, user_id)
    }
    
    /// Generate rate limit key for signal posting.
    pub fn signal_post(tenant_id: &str, session_id: &str) -> String {
        format!("rtc:signal:{}:{}", tenant_id, session_id)
    }
    
    /// Generate rate limit key for participant invite.
    pub fn participant_invite(tenant_id: &str, session_id: &str) -> String {
        format!("rtc:invite:{}:{}", tenant_id, session_id)
    }
    
    /// Generate rate limit key for tenant-level operations.
    pub fn tenant_level(tenant_id: &str, operation: &str) -> String {
        format!("rtc:{}:{}", operation, tenant_id)
    }
}

/// Security validation result.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecurityValidationResult {
    /// Whether validation passed.
    pub passed: bool,
    /// Tenant ID validated.
    pub tenant_id: String,
    /// Organization ID validated.
    pub organization_id: String,
    /// User ID validated.
    pub user_id: Option<String>,
    /// Validation errors (if any).
    pub errors: Vec<String>,
    /// Audit event type for this validation.
    pub audit_event_type: String,
}

impl SecurityValidationResult {
    pub fn success(tenant_id: String, organization_id: String, user_id: Option<String>) -> Self {
        Self {
            passed: true,
            tenant_id,
            organization_id,
            user_id,
            errors: vec![],
            audit_event_type: "validation.success".to_string(),
        }
    }
    
    pub fn failure(tenant_id: String, organization_id: String, user_id: Option<String>, errors: Vec<String>) -> Self {
        Self {
            passed: false,
            tenant_id,
            organization_id,
            user_id,
            errors,
            audit_event_type: "validation.failure".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_context() -> SecurityContext {
        SecurityContext {
            tenant_id: "tenant_1".to_string(),
            organization_id: "org_1".to_string(),
            login_scope: "ORGANIZATION".to_string(),
            user_id: Some("user_1".to_string()),
            session_id: Some("session_1".to_string()),
            actor_id: "user_1".to_string(),
            actor_kind: "USER".to_string(),
            permission_scope: vec!["rtc.sessions.write".to_string(), "rtc.signals.write".to_string()],
            data_scope: "organization".to_string(),
            auth_level: "password".to_string(),
            request_id: Some("req_1".to_string()),
            trace_id: Some("trace_1".to_string()),
        }
    }
    
    #[test]
    fn tenant_isolation_same_tenant() {
        let validator = TenantIsolationValidator::new();
        let context = create_test_context();
        
        let result = validator.validate_access(&context, "tenant_1");
        assert!(result.is_ok());
    }
    
    #[test]
    fn tenant_isolation_cross_tenant_denied() {
        let validator = TenantIsolationValidator::new();
        let context = create_test_context();
        
        let result = validator.validate_access(&context, "tenant_2");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TenantIsolationError::TenantMismatch { .. }));
    }
    
    #[test]
    fn tenant_isolation_platform_admin_allowed() {
        let validator = TenantIsolationValidator::new();
        let mut context = create_test_context();
        context.permission_scope.push("platform.admin".to_string());
        
        let result = validator.validate_access(&context, "tenant_2");
        assert!(result.is_ok());
    }
    
    #[test]
    fn permission_validation_allowed() {
        let validator = PermissionValidator::new();
        let context = create_test_context();
        
        let result = validator.validate_operation(&context, "rtc.session.create");
        assert!(result.is_ok());
    }
    
    #[test]
    fn permission_validation_denied() {
        let validator = PermissionValidator::new();
        let context = SecurityContext {
            tenant_id: "tenant_1".to_string(),
            organization_id: "org_1".to_string(),
            login_scope: "ORGANIZATION".to_string(),
            user_id: Some("user_1".to_string()),
            session_id: Some("session_1".to_string()),
            actor_id: "user_1".to_string(),
            actor_kind: "USER".to_string(),
            permission_scope: vec!["rtc.sessions.read".to_string()], // Missing write permission
            data_scope: "organization".to_string(),
            auth_level: "password".to_string(),
            request_id: None,
            trace_id: None,
        };
        
        let result = validator.validate_operation(&context, "rtc.session.create");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PermissionError::PermissionDenied { .. }));
    }
    
    #[test]
    fn signal_replay_sequence_validation() {
        let protector = SignalReplayProtector::new();
        
        // First signal should pass
        let result = protector.validate_signal_sequence("session_1", 1, "hash_1");
        assert!(result.is_ok());
        
        // Higher sequence should pass
        let result = protector.validate_signal_sequence("session_1", 2, "hash_2");
        assert!(result.is_ok());
        
        // Stale sequence should fail
        let result = protector.validate_signal_sequence("session_1", 1, "hash_3");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SignalReplayError::StaleSequence { .. }));
        
        // Duplicate hash should fail
        let result = protector.validate_signal_sequence("session_1", 3, "hash_1");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SignalReplayError::DuplicateSignal { .. }));
    }
    
    #[test]
    fn signal_replay_session_cleanup() {
        let protector = SignalReplayProtector::new();
        
        protector.validate_signal_sequence("session_1", 1, "hash_1").unwrap();
        protector.validate_signal_sequence("session_1", 2, "hash_2").unwrap();
        
        protector.clear_session("session_1");
        
        // After cleanup, sequence should restart from 0
        let result = protector.validate_signal_sequence("session_1", 1, "hash_new");
        assert!(result.is_ok());
    }
    
    #[test]
    fn login_scope_detection() {
        let tenant_context = SecurityContext {
            tenant_id: "tenant_1".to_string(),
            organization_id: "0".to_string(),
            login_scope: "TENANT".to_string(),
            user_id: Some("user_1".to_string()),
            session_id: None,
            actor_id: "user_1".to_string(),
            actor_kind: "USER".to_string(),
            permission_scope: vec![],
            data_scope: "tenant".to_string(),
            auth_level: "password".to_string(),
            request_id: None,
            trace_id: None,
        };
        
        assert!(tenant_context.is_tenant_scope());
        assert!(!tenant_context.is_organization_scope());
        
        let org_context = create_test_context();
        assert!(org_context.is_organization_scope());
        assert!(!org_context.is_tenant_scope());
    }
}