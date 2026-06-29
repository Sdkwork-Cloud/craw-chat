//! Security and operational audit events for SDKWork IM service.
//!
//! This module provides audit trail functionality following `SECURITY_SPEC.md`
//! requirements for L3 enterprise-grade compliance.
//!
//! ## Audit Event Categories
//!
//! - **Security events**: Authentication failures, permission changes, key creation/revocation
//! - **Access events**: Resource access, cross-tenant operations
//! - **Mutation events**: Call session lifecycle, signaling events
//! - **Administrative events**: Configuration changes, tenant operations
//!
//! ## Event Structure
//!
//! All audit events follow the standard schema from `SECURITY_SPEC.md`:
//!
//! ```json
//! {
//!   "event_id": "snowflake_id",
//!   "event_type": "security.login_failure",
//!   "timestamp": "2026-01-01T00:00:00Z",
//!   "tenant_id": "100001",
//!   "organization_id": "0",
//!   "user_id": "user_1",
//!   "session_id": "session_1",
//!   "actor_type": "USER",
//!   "actor_id": "user_1",
//!   "action": "login",
//!   "target_type": "session",
//!   "target_id": "session_1",
//!   "outcome": "FAILURE",
//!   "reason": "invalid_credentials",
//!   "request_id": "req_123",
//!   "trace_id": "trace_456",
//!   "client_ip": "192.168.1.1",
//!   "metadata": {}
//! }
//! ```

use serde::{Deserialize, Serialize};

/// Audit event type classification.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    // Security events (L3 required)
    /// Login attempt (success or failure).
    SecurityLoginAttempt,
    /// Token validation failure.
    SecurityTokenValidationFailure,
    /// Permission denied event.
    SecurityPermissionDenied,
    /// Cross-tenant access attempt (blocked or allowed).
    SecurityCrossTenantAccess,
    /// API key usage event.
    SecurityApiKeyUsage,
    /// Session creation/termination.
    SecuritySessionLifecycle,
    
    // Access events (L2 required)
    /// Resource read access.
    AccessResourceRead,
    /// Resource mutation access.
    AccessResourceMutation,
    /// List/query access.
    AccessResourceList,
    
    // Mutation events (L2 required)
    /// Call session created.
    MutationCallSessionCreated,
    /// Call session state changed.
    MutationCallSessionStateChanged,
    /// Call signaling event posted.
    MutationCallSignalPosted,
    /// Participant invited/accepted/rejected.
    MutationCallParticipantChanged,
    
    // Administrative events (L3 required)
    /// Configuration changed.
    AdminConfigurationChanged,
    /// Rate limit threshold changed.
    AdminRateLimitChanged,
    /// Feature flag toggled.
    AdminFeatureFlagChanged,
}

impl AuditEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditEventType::SecurityLoginAttempt => "security.login_attempt",
            AuditEventType::SecurityTokenValidationFailure => "security.token_validation_failure",
            AuditEventType::SecurityPermissionDenied => "security.permission_denied",
            AuditEventType::SecurityCrossTenantAccess => "security.cross_tenant_access",
            AuditEventType::SecurityApiKeyUsage => "security.api_key_usage",
            AuditEventType::SecuritySessionLifecycle => "security.session_lifecycle",
            AuditEventType::AccessResourceRead => "access.resource_read",
            AuditEventType::AccessResourceMutation => "access.resource_mutation",
            AuditEventType::AccessResourceList => "access.resource_list",
            AuditEventType::MutationCallSessionCreated => "mutation.call_session_created",
            AuditEventType::MutationCallSessionStateChanged => "mutation.call_session_state_changed",
            AuditEventType::MutationCallSignalPosted => "mutation.call_signal_posted",
            AuditEventType::MutationCallParticipantChanged => "mutation.call_participant_changed",
            AuditEventType::AdminConfigurationChanged => "admin.configuration_changed",
            AuditEventType::AdminRateLimitChanged => "admin.rate_limit_changed",
            AuditEventType::AdminFeatureFlagChanged => "admin.feature_flag_changed",
        }
    }
    
    /// Check if this event type requires L3 audit compliance.
    pub fn requires_l3_audit(&self) -> bool {
        matches!(self, 
            AuditEventType::SecurityLoginAttempt |
            AuditEventType::SecurityTokenValidationFailure |
            AuditEventType::SecurityPermissionDenied |
            AuditEventType::SecurityCrossTenantAccess |
            AuditEventType::SecurityApiKeyUsage |
            AuditEventType::SecuritySessionLifecycle |
            AuditEventType::AdminConfigurationChanged |
            AuditEventType::AdminRateLimitChanged |
            AuditEventType::AdminFeatureFlagChanged
        )
    }
}

/// Actor type classification.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditActorType {
    /// Regular user.
    User,
    /// System service account.
    System,
    /// Administrator.
    Admin,
    /// Background job/task.
    Job,
    /// API key authentication.
    Service,
    /// Anonymous/public request.
    Anonymous,
}

impl AuditActorType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditActorType::User => "USER",
            AuditActorType::System => "SYSTEM",
            AuditActorType::Admin => "ADMIN",
            AuditActorType::Job => "JOB",
            AuditActorType::Service => "SERVICE",
            AuditActorType::Anonymous => "ANONYMOUS",
        }
    }
}

/// Audit event outcome.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditOutcome {
    /// Operation succeeded.
    Success,
    /// Operation failed due to business error.
    Failure,
    /// Operation denied due to authorization.
    Denied,
    /// Operation blocked due to security policy.
    Blocked,
    /// Operation throttled due to rate limit.
    Throttled,
}

impl AuditOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditOutcome::Success => "SUCCESS",
            AuditOutcome::Failure => "FAILURE",
            AuditOutcome::Denied => "DENIED",
            AuditOutcome::Blocked => "BLOCKED",
            AuditOutcome::Throttled => "THROTTLED",
        }
    }
    
    pub fn is_success(&self) -> bool {
        matches!(self, AuditOutcome::Success)
    }
}

/// Standard audit event structure following `SECURITY_SPEC.md`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Unique event ID (Snowflake ID).
    pub event_id: i64,
    /// Event type classification.
    pub event_type: AuditEventType,
    /// Event timestamp (UTC ISO 8601).
    pub timestamp: String,
    /// Tenant ID where event occurred.
    pub tenant_id: String,
    /// Organization ID (0 for tenant-level).
    pub organization_id: String,
    /// User ID who performed the action.
    pub user_id: Option<String>,
    /// Session ID of the actor.
    pub session_id: Option<String>,
    /// Actor type classification.
    pub actor_type: AuditActorType,
    /// Actor ID (user_id, service_id, or system).
    pub actor_id: String,
    /// Action performed.
    pub action: String,
    /// Target resource type.
    pub target_type: String,
    /// Target resource ID.
    pub target_id: String,
    /// Outcome of the action.
    pub outcome: AuditOutcome,
    /// Reason for outcome (error code or message).
    pub reason: Option<String>,
    /// Request correlation ID.
    pub request_id: Option<String>,
    /// Distributed trace ID.
    pub trace_id: Option<String>,
    /// Client IP address (will be redacted in output).
    pub client_ip: Option<String>,
    /// User agent string.
    pub user_agent: Option<String>,
    /// Additional metadata (redacted in output).
    pub metadata: serde_json::Value,
    /// Event processing duration in milliseconds.
    pub duration_ms: Option<u64>,
}

impl AuditEvent {
    /// Create a new audit event builder.
    pub fn builder() -> AuditEventBuilder {
        AuditEventBuilder::default()
    }
    
    /// Convert to redacted JSON for logging/output.
    pub fn to_redacted_json(&self) -> serde_json::Value {
        serde_json::json!({
            "event_id": self.event_id.to_string(),
            "event_type": self.event_type.as_str(),
            "timestamp": self.timestamp,
            "tenant_id": self.tenant_id,
            "organization_id": self.organization_id,
            "user_id": self.user_id,
            "session_id": self.session_id.as_ref().map(|_| "[REDACTED]"),
            "actor_type": self.actor_type.as_str(),
            "actor_id": self.actor_id,
            "action": self.action,
            "target_type": self.target_type,
            "target_id": self.target_id,
            "outcome": self.outcome.as_str(),
            "reason": self.reason,
            "request_id": self.request_id,
            "trace_id": self.trace_id,
            "client_ip": "[IP_REDACTED]",
            "user_agent": self.user_agent,
            "metadata": "[REDACTED]",
            "duration_ms": self.duration_ms,
        })
    }
}

/// Builder for audit events.
#[derive(Clone, Debug, Default)]
pub struct AuditEventBuilder {
    event_id: Option<i64>,
    event_type: Option<AuditEventType>,
    timestamp: Option<String>,
    tenant_id: Option<String>,
    organization_id: Option<String>,
    user_id: Option<String>,
    session_id: Option<String>,
    actor_type: Option<AuditActorType>,
    actor_id: Option<String>,
    action: Option<String>,
    target_type: Option<String>,
    target_id: Option<String>,
    outcome: Option<AuditOutcome>,
    reason: Option<String>,
    request_id: Option<String>,
    trace_id: Option<String>,
    client_ip: Option<String>,
    user_agent: Option<String>,
    metadata: serde_json::Value,
    duration_ms: Option<u64>,
}

impl AuditEventBuilder {
    pub fn event_id(mut self, id: i64) -> Self {
        self.event_id = Some(id);
        self
    }
    
    pub fn event_type(mut self, type_: AuditEventType) -> Self {
        self.event_type = Some(type_);
        self
    }
    
    pub fn timestamp(mut self, ts: String) -> Self {
        self.timestamp = Some(ts);
        self
    }
    
    pub fn tenant_id(mut self, id: String) -> Self {
        self.tenant_id = Some(id);
        self
    }
    
    pub fn organization_id(mut self, id: String) -> Self {
        self.organization_id = Some(id);
        self
    }
    
    pub fn user_id(mut self, id: Option<String>) -> Self {
        self.user_id = id;
        self
    }
    
    pub fn session_id(mut self, id: Option<String>) -> Self {
        self.session_id = id;
        self
    }
    
    pub fn actor_type(mut self, type_: AuditActorType) -> Self {
        self.actor_type = Some(type_);
        self
    }
    
    pub fn actor_id(mut self, id: String) -> Self {
        self.actor_id = Some(id);
        self
    }
    
    pub fn action(mut self, action: String) -> Self {
        self.action = Some(action);
        self
    }
    
    pub fn target_type(mut self, type_: String) -> Self {
        self.target_type = Some(type_);
        self
    }
    
    pub fn target_id(mut self, id: String) -> Self {
        self.target_id = Some(id);
        self
    }
    
    pub fn outcome(mut self, outcome: AuditOutcome) -> Self {
        self.outcome = Some(outcome);
        self
    }
    
    pub fn reason(mut self, reason: String) -> Self {
        self.reason = Some(reason);
        self
    }

    pub fn reason_opt(mut self, reason: Option<String>) -> Self {
        self.reason = reason;
        self
    }

    pub fn request_id(mut self, id: Option<String>) -> Self {
        self.request_id = id;
        self
    }
    
    pub fn trace_id(mut self, id: Option<String>) -> Self {
        self.trace_id = id;
        self
    }
    
    pub fn client_ip(mut self, ip: Option<String>) -> Self {
        self.client_ip = ip;
        self
    }
    
    pub fn user_agent(mut self, ua: Option<String>) -> Self {
        self.user_agent = ua;
        self
    }
    
    pub fn metadata(mut self, meta: serde_json::Value) -> Self {
        self.metadata = meta;
        self
    }
    
    pub fn duration_ms(mut self, ms: u64) -> Self {
        self.duration_ms = Some(ms);
        self
    }
    
    /// Build the audit event, validating required fields.
    pub fn build(self) -> Result<AuditEvent, AuditEventError> {
        Ok(AuditEvent {
            event_id: self.event_id.ok_or(AuditEventError::MissingField("event_id"))?,
            event_type: self.event_type.ok_or(AuditEventError::MissingField("event_type"))?,
            timestamp: self.timestamp.ok_or(AuditEventError::MissingField("timestamp"))?,
            tenant_id: self.tenant_id.ok_or(AuditEventError::MissingField("tenant_id"))?,
            organization_id: self.organization_id.unwrap_or_else(|| "0".to_string()),
            user_id: self.user_id,
            session_id: self.session_id,
            actor_type: self.actor_type.ok_or(AuditEventError::MissingField("actor_type"))?,
            actor_id: self.actor_id.ok_or(AuditEventError::MissingField("actor_id"))?,
            action: self.action.ok_or(AuditEventError::MissingField("action"))?,
            target_type: self.target_type.ok_or(AuditEventError::MissingField("target_type"))?,
            target_id: self.target_id.ok_or(AuditEventError::MissingField("target_id"))?,
            outcome: self.outcome.ok_or(AuditEventError::MissingField("outcome"))?,
            reason: self.reason,
            request_id: self.request_id,
            trace_id: self.trace_id,
            client_ip: self.client_ip,
            user_agent: self.user_agent,
            metadata: self.metadata,
            duration_ms: self.duration_ms,
        })
    }
}

/// Error for audit event construction.
#[derive(Clone, Debug, thiserror::Error)]
pub enum AuditEventError {
    #[error("Missing required field: {0}")]
    MissingField(&'static str),
}

/// Audit event emitter trait.
pub trait AuditEmitter: Send + Sync {
    /// Emit an audit event.
    fn emit(&self, event: AuditEvent) -> Result<(), AuditEventError>;
}

/// In-memory audit emitter for testing.
#[derive(Clone, Debug, Default)]
pub struct MemoryAuditEmitter {
    events: std::sync::Arc<std::sync::Mutex<Vec<AuditEvent>>>,
}

impl MemoryAuditEmitter {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn events(&self) -> Vec<AuditEvent> {
        self.events.lock().unwrap().clone()
    }
    
    pub fn clear(&self) {
        self.events.lock().unwrap().clear();
    }
}

impl AuditEmitter for MemoryAuditEmitter {
    fn emit(&self, event: AuditEvent) -> Result<(), AuditEventError> {
        self.events.lock().unwrap().push(event);
        Ok(())
    }
}

/// No-op audit emitter for development.
#[derive(Clone, Debug, Default)]
pub struct NoOpAuditEmitter;

impl AuditEmitter for NoOpAuditEmitter {
    fn emit(&self, _event: AuditEvent) -> Result<(), AuditEventError> {
        Ok(())
    }
}

/// Logging audit emitter for production.
///
/// Emits audit events via `tracing::info!` so they are captured by the
/// application's log pipeline (typically shipped to a SIEM such as
/// Elasticsearch, Splunk, or Loki). The redacted JSON form is used so
/// sensitive fields (`session_id`, `client_ip`, `metadata`) are masked
/// before reaching the log sink.
///
/// This is the default production audit emitter for services that do not
/// wire a dedicated audit-store emitter (e.g. `PostgresAuditEmitter`).
/// Audit emission is best-effort: logging failures are swallowed to
/// never block user-facing operations.
#[derive(Clone, Debug, Default)]
pub struct LoggingAuditEmitter;

impl AuditEmitter for LoggingAuditEmitter {
    fn emit(&self, event: AuditEvent) -> Result<(), AuditEventError> {
        let redacted = event.to_redacted_json();
        tracing::info!(
            target: "sdkwork.im.audit",
            event_id = %event.event_id,
            event_type = %event.event_type.as_str(),
            tenant_id = %event.tenant_id,
            organization_id = %event.organization_id,
            actor_id = %event.actor_id,
            action = %event.action,
            target_type = %event.target_type,
            target_id = %event.target_id,
            outcome = %event.outcome.as_str(),
            l3_required = event.event_type.requires_l3_audit(),
            audit_event = %redacted,
            "audit event emitted"
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn audit_event_builder_complete() {
        let event = AuditEvent::builder()
            .event_id(123456789)
            .event_type(AuditEventType::MutationCallSessionCreated)
            .timestamp("2026-01-01T00:00:00Z".to_string())
            .tenant_id("tenant_1".to_string())
            .organization_id("org_1".to_string())
            .user_id(Some("user_1".to_string()))
            .session_id(Some("session_1".to_string()))
            .actor_type(AuditActorType::User)
            .actor_id("user_1".to_string())
            .action("create_call_session".to_string())
            .target_type("rtc_session".to_string())
            .target_id("rtc_session_1".to_string())
            .outcome(AuditOutcome::Success)
            .request_id(Some("req_1".to_string()))
            .trace_id(Some("trace_1".to_string()))
            .client_ip(Some("192.168.1.1".to_string()))
            .duration_ms(50)
            .build()
            .expect("complete audit event should build");
        
        assert_eq!(event.event_id, 123456789);
        assert_eq!(event.tenant_id, "tenant_1");
        assert!(event.outcome.is_success());
    }
    
    #[test]
    fn audit_event_missing_field_error() {
        let result = AuditEvent::builder()
            .event_id(123)
            .build();
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Missing required field: event_type");
    }
    
    #[test]
    fn audit_event_redaction() {
        let event = AuditEvent::builder()
            .event_id(123)
            .event_type(AuditEventType::SecurityLoginAttempt)
            .timestamp("2026-01-01T00:00:00Z".to_string())
            .tenant_id("tenant_1".to_string())
            .actor_type(AuditActorType::User)
            .actor_id("user_1".to_string())
            .action("login".to_string())
            .target_type("session".to_string())
            .target_id("session_1".to_string())
            .outcome(AuditOutcome::Success)
            .session_id(Some("sensitive_session_id".to_string()))
            .client_ip(Some("192.168.1.1".to_string()))
            .metadata(serde_json::json!({"token": "secret"}))
            .build()
            .unwrap();
        
        let redacted = event.to_redacted_json();
        assert_eq!(redacted["session_id"], "[REDACTED]");
        assert_eq!(redacted["client_ip"], "[IP_REDACTED]");
        assert_eq!(redacted["metadata"], "[REDACTED]");
    }
    
    #[test]
    fn memory_audit_emitter() {
        let emitter = MemoryAuditEmitter::new();
        let event = AuditEvent::builder()
            .event_id(1)
            .event_type(AuditEventType::MutationCallSessionCreated)
            .timestamp("2026-01-01T00:00:00Z".to_string())
            .tenant_id("tenant_1".to_string())
            .actor_type(AuditActorType::User)
            .actor_id("user_1".to_string())
            .action("create".to_string())
            .target_type("session".to_string())
            .target_id("session_1".to_string())
            .outcome(AuditOutcome::Success)
            .build()
            .unwrap();
        
        emitter.emit(event.clone()).unwrap();
        
        let events = emitter.events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_id, 1);
    }
    
    #[test]
    fn event_type_l3_classification() {
        assert!(AuditEventType::SecurityLoginAttempt.requires_l3_audit());
        assert!(AuditEventType::SecurityTokenValidationFailure.requires_l3_audit());
        assert!(!AuditEventType::MutationCallSignalPosted.requires_l3_audit());
    }
}