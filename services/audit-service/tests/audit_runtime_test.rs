use std::collections::BTreeSet;
use std::thread::sleep;
use std::time::Duration;

use im_app_context::AppContext;

#[test]
fn test_record_anchor_and_export_bundle() {
    let runtime = audit_service::AuditRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        permission_scope: BTreeSet::from(["audit.read".to_owned()]),
        device_id: None,
    };

    let record = runtime
        .record_anchor(
            &auth,
            audit_service::RecordAuditAnchor {
                record_id: "audit_demo".into(),
                aggregate_type: "automation_execution".into(),
                aggregate_id: "ae_demo".into(),
                action: "automation.execution_requested".into(),
                payload: Some(r#"{"targetRef":"wf_demo"}"#.into()),
            },
        )
        .expect("record anchor should succeed");

    assert_eq!(record.record_id, "audit_demo");
    assert_eq!(record.actor_id, "1");
    assert_eq!(record.actor_session_id.as_deref(), Some("s_demo"));

    let export = runtime
        .export_bundle(&auth)
        .expect("export bundle should succeed");
    assert_eq!(export.total, 1);
    assert_eq!(export.items[0].aggregate_id, "ae_demo");
    assert_eq!(export.items[0].action, "automation.execution_requested");
}

#[test]
fn test_recorded_at_advances_between_distinct_records() {
    let runtime = audit_service::AuditRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        permission_scope: BTreeSet::from(["audit.read".to_owned()]),
        device_id: None,
    };

    let first = runtime
        .record_anchor(
            &auth,
            audit_service::RecordAuditAnchor {
                record_id: "audit_time_first".into(),
                aggregate_type: "notification".into(),
                aggregate_id: "ntf_time_first".into(),
                action: "notification.requested".into(),
                payload: None,
            },
        )
        .expect("first record should succeed");

    sleep(Duration::from_millis(5));

    let second = runtime
        .record_anchor(
            &auth,
            audit_service::RecordAuditAnchor {
                record_id: "audit_time_second".into(),
                aggregate_type: "notification".into(),
                aggregate_id: "ntf_time_second".into(),
                action: "notification.dispatched".into(),
                payload: None,
            },
        )
        .expect("second record should succeed");

    assert_ne!(
        first.recorded_at, second.recorded_at,
        "distinct audit records must not reuse a fixed recorded_at timestamp"
    );
}

#[test]
fn test_export_bundle_includes_verifiable_chain_and_detects_tampering() {
    let runtime = audit_service::AuditRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        permission_scope: BTreeSet::from(["audit.read".to_owned()]),
        device_id: None,
    };

    runtime
        .record_anchor(
            &auth,
            audit_service::RecordAuditAnchor {
                record_id: "audit_chain_first".into(),
                aggregate_type: "notification".into(),
                aggregate_id: "ntf_chain_first".into(),
                action: "notification.requested".into(),
                payload: Some(r#"{"step":"first"}"#.into()),
            },
        )
        .expect("first chain record should succeed");
    runtime
        .record_anchor(
            &auth,
            audit_service::RecordAuditAnchor {
                record_id: "audit_chain_second".into(),
                aggregate_type: "notification".into(),
                aggregate_id: "ntf_chain_second".into(),
                action: "notification.dispatched".into(),
                payload: Some(r#"{"step":"second"}"#.into()),
            },
        )
        .expect("second chain record should succeed");

    let export = runtime
        .export_bundle(&auth)
        .expect("export bundle should succeed");
    assert_eq!(export.total, 2);
    assert!(
        export.chain_valid,
        "freshly exported bundle should report chain_valid=true"
    );
    assert!(
        export.chain_head_hash.is_some(),
        "freshly exported bundle should expose non-empty chain head hash"
    );
    assert!(
        audit_service::verify_audit_export_bundle_integrity(&export),
        "freshly exported bundle should pass integrity verification"
    );

    let mut tampered = export.clone();
    tampered.items[1].action = "notification.tampered".into();
    tampered.chain_valid = true;
    assert!(
        !audit_service::verify_audit_export_bundle_integrity(&tampered),
        "tampered bundle should fail integrity verification"
    );
}

#[test]
fn test_runtime_record_anchor_rejects_oversized_payload_consistently_with_http_contract() {
    let runtime = audit_service::AuditRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        permission_scope: BTreeSet::from(["audit.read".to_owned()]),
        device_id: None,
    };

    let error = runtime
        .record_anchor(
            &auth,
            audit_service::RecordAuditAnchor {
                record_id: "audit_runtime_oversized_payload".into(),
                aggregate_type: "notification".into(),
                aggregate_id: "ntf_runtime_oversized_payload".into(),
                action: "notification.requested".into(),
                payload: Some("x".repeat(200_000)),
            },
        )
        .expect_err("runtime API should reject oversized audit payloads");

    let debug = format!("{error:?}");
    assert!(
        debug.contains("payload_too_large"),
        "runtime API should preserve the audit payload size contract"
    );
    assert_eq!(
        runtime
            .export_bundle(&auth)
            .expect("export bundle should succeed")
            .total,
        0,
        "rejected oversized payload must not append a partial audit record"
    );
}

#[test]
fn test_runtime_reads_require_permission_and_isolate_organizations() {
    let runtime = audit_service::AuditRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "organization-a".to_owned(),
        user_id: "1".into(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        permission_scope: BTreeSet::from(["audit.read".to_owned()]),
        device_id: None,
    };

    runtime
        .record_anchor(
            &auth,
            audit_service::RecordAuditAnchor {
                record_id: "audit_organization_a".into(),
                aggregate_type: "security_event".into(),
                aggregate_id: "event_1".into(),
                action: "security.permission_denied".into(),
                payload: None,
            },
        )
        .expect("audit record should be stored in organization-a");
    runtime
        .record_anchor(
            &auth,
            audit_service::RecordAuditAnchor {
                record_id: "audit_organization_a_second".into(),
                aggregate_type: "security_event".into(),
                aggregate_id: "event_2".into(),
                action: "security.warning".into(),
                payload: None,
            },
        )
        .expect("second audit record should be stored in organization-a");

    let recent = runtime
        .recent_records(&auth, 1)
        .expect("recent audit sample should succeed");
    assert_eq!(recent.items.len(), 1);
    assert_eq!(recent.items[0].record_id, "audit_organization_a_second");
    assert!(recent.has_more);

    let mut no_permission = auth.clone();
    no_permission.permission_scope.clear();
    let error = runtime
        .export_bundle(&no_permission)
        .expect_err("runtime reads must enforce audit.read");
    assert!(format!("{error:?}").contains("permission_denied"));

    let mut other_organization = auth;
    other_organization.organization_id = "organization-b".to_owned();
    let export = runtime
        .export_bundle(&other_organization)
        .expect("authorized organization-b read should succeed");
    assert_eq!(
        export.total, 0,
        "organizations must not share audit records"
    );
}
