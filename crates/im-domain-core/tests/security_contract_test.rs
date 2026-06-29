//! Authentication, tenant isolation, audit, and replay-protection contract tests.
//!
//! These tests verify enforcement of `SECURITY_SPEC.md` and `IAM_SPEC.md` rules
//! at the domain layer. They test the building blocks (validators, protectors,
//! emitters) that the services depend on.

use im_domain_core::audit::*;
use im_domain_core::security::*;

// =============================================================================
// Tenant Isolation Contract
// =============================================================================

#[test]
fn test_tenant_isolation_same_tenant_allows_access() {
    let validator = TenantIsolationValidator::new();
    let ctx = SecurityContext {
        tenant_id: "t1".into(),
        organization_id: "0".into(),
        login_scope: "TENANT".into(),
        user_id: Some("u1".into()),
        session_id: None,
        actor_id: "u1".into(),
        actor_kind: "USER".into(),
        permission_scope: vec![],
        data_scope: "tenant".into(),
        auth_level: "password".into(),
        request_id: None,
        trace_id: None,
    };
    assert!(validator.validate_access(&ctx, "t1").is_ok());
}

#[test]
fn test_tenant_isolation_different_tenant_denied() {
    let validator = TenantIsolationValidator::new();
    let ctx = SecurityContext {
        tenant_id: "t1".into(),
        ..create_minimal_context()
    };
    assert!(validator.validate_access(&ctx, "t2").is_err());
}

#[test]
fn test_tenant_isolation_platform_admin_crosses_tenant_boundary() {
    let validator = TenantIsolationValidator::new();
    let mut ctx = create_minimal_context();
    ctx.permission_scope.push("platform.admin".into());
    assert!(validator.validate_access(&ctx, "t2").is_ok());
}

// =============================================================================
// Permission Contract
// =============================================================================

#[test]
fn test_permission_required_for_operation() {
    let validator = PermissionValidator::new();
    let ctx = SecurityContext {
        permission_scope: vec!["rtc.sessions.write".into()],
        ..create_minimal_context()
    };
    assert!(validator.validate_operation(&ctx, "rtc.session.create").is_ok());
}

#[test]
fn test_permission_missing_rejected() {
    let validator = PermissionValidator::new();
    let ctx = SecurityContext {
        permission_scope: vec!["rtc.sessions.read".into()],
        ..create_minimal_context()
    };
    let err = validator.validate_operation(&ctx, "rtc.session.create");
    assert!(err.is_err());
}

// =============================================================================
// Signal Replay Protection Contract
// =============================================================================

#[test]
fn test_signal_replay_accepts_new_sequences() {
    let protector = SignalReplayProtector::new();
    assert!(protector.validate_signal_sequence("s1", 1, "h1").is_ok());
    assert!(protector.validate_signal_sequence("s1", 2, "h2").is_ok());
}

#[test]
fn test_signal_replay_rejects_stale_sequences() {
    let protector = SignalReplayProtector::new();
    assert!(protector.validate_signal_sequence("s1", 5, "h1").is_ok());
    let err = protector.validate_signal_sequence("s1", 3, "h2");
    assert!(matches!(err, Err(SignalReplayError::StaleSequence { .. })));
}

#[test]
fn test_signal_replay_rejects_duplicate_hash() {
    let protector = SignalReplayProtector::new();
    assert!(protector.validate_signal_sequence("s1", 1, "same_hash").is_ok());
    let err = protector.validate_signal_sequence("s1", 2, "same_hash");
    assert!(matches!(err, Err(SignalReplayError::DuplicateSignal { .. })));
}

#[test]
fn test_signal_replay_accepts_after_session_cleanup() {
    let protector = SignalReplayProtector::new();
    protector.validate_signal_sequence("s1", 1, "h1").unwrap();
    protector.clear_session("s1");
    assert!(protector.validate_signal_sequence("s1", 1, "h2").is_ok());
}

// =============================================================================
// Audit Event Contract
// =============================================================================

#[test]
fn test_audit_event_requires_mandatory_fields() {
    let err = AuditEvent::builder().event_id(1).build();
    assert!(err.is_err());
    assert!(err.unwrap_err().to_string().contains("event_type"));
}

#[test]
fn test_audit_event_complete_build_succeeds() {
    let event = AuditEvent::builder()
        .event_id(1)
        .event_type(AuditEventType::SecurityLoginAttempt)
        .timestamp("2026-06-27T00:00:00Z".into())
        .tenant_id("t1".into())
        .actor_type(AuditActorType::User)
        .actor_id("u1".into())
        .action("login".into())
        .target_type("session".into())
        .target_id("s1".into())
        .outcome(AuditOutcome::Success)
        .build();
    assert!(event.is_ok());
}

#[test]
fn test_audit_event_redaction_hides_sensitive_fields() {
    let event = AuditEventBuilder::default()
        .event_id(1)
        .event_type(AuditEventType::SecurityLoginAttempt)
        .timestamp("2026-01-01T00:00:00Z".into())
        .tenant_id("t1".into())
        .actor_type(AuditActorType::User)
        .actor_id("u1".into())
        .action("login".into())
        .target_type("session".into())
        .target_id("s1".into())
        .outcome(AuditOutcome::Success)
        .session_id(Some("supersecret".into()))
        .client_ip(Some("10.0.0.1".into()))
        .metadata(serde_json::json!({"token": "abc"}))
        .build()
        .unwrap();
    let redacted = event.to_redacted_json();
    assert_eq!(redacted["session_id"], "[REDACTED]");
    assert_eq!(redacted["client_ip"], "[IP_REDACTED]");
    assert_eq!(redacted["metadata"], "[REDACTED]");
}

// =============================================================================
// Helpers
// =============================================================================

fn create_minimal_context() -> SecurityContext {
    SecurityContext {
        tenant_id: "t1".into(),
        organization_id: "0".into(),
        login_scope: "TENANT".into(),
        user_id: None,
        session_id: None,
        actor_id: "u1".into(),
        actor_kind: "USER".into(),
        permission_scope: vec![],
        data_scope: "tenant".into(),
        auth_level: "password".into(),
        request_id: None,
        trace_id: None,
    }
}
