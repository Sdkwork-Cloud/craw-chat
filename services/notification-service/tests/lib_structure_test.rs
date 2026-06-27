/// Concatenates all notification-service source modules so structural
/// regression assertions stay accurate after the lib.rs split.
fn notification_service_source() -> String {
    format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        include_str!("../src/lib.rs"),
        include_str!("../src/app.rs"),
        include_str!("../src/dto.rs"),
        include_str!("../src/error.rs"),
        include_str!("../src/handlers.rs"),
        include_str!("../src/helpers.rs"),
        include_str!("../src/openapi.rs"),
        include_str!("../src/state.rs"),
        include_str!("../src/tests.rs"),
    )
    .replace("\r\n", "\n")
}

#[test]
fn test_notification_runtime_exposes_public_request_access_owner() {
    let source = notification_service_source();

    assert!(
        source.contains("pub fn request_notification_from_app_context("),
        "notification-service should expose a runtime-owned AppContext notification request seam so service entrypoints do not each reimplement cross-recipient access control"
    );

    assert!(
        source.contains(".request_notification_from_app_context(&auth, request)"),
        "notification-service HTTP request path should consume the runtime-owned AppContext notification request seam"
    );

    assert!(
        !source
            .contains("ensure_notification_request_access(&auth, request.recipient_id.as_str())?;"),
        "notification-service HTTP request path should not inline cross-recipient notification access control once NotificationRuntime owns that AppContext boundary"
    );
}

#[test]
fn test_notification_runtime_exposes_notification_fanout_owner_seam() {
    let source = notification_service_source();

    assert!(
        source.contains("pub fn request_notification_fanout("),
        "notification-service should expose a runtime-owned notification fanout seam so service-edge side-effect paths do not each reimplement per-recipient notification orchestration"
    );
}

#[test]
fn test_notification_runtime_exposes_automation_result_notification_owner_seam() {
    let source = notification_service_source();

    assert!(
        source.contains("pub fn request_automation_result_notification("),
        "notification-service should expose a runtime-owned automation result notification seam so service-edge automation paths do not each hand-assemble notification ids, source events, and recipient routing"
    );
}

#[test]
fn test_notification_runtime_exposes_message_posted_notification_owner_seam() {
    let source = notification_service_source();

    assert!(
        source.contains("pub fn request_message_posted_notifications("),
        "notification-service should expose a runtime-owned message-posted notification seam so service-edge message side-effect paths do not each hand-assemble category, source-event type, and payload defaults"
    );

    assert!(
        source.contains(".message_posted_notification_recipients_from_auth_context("),
        "notification-service should resolve message-posted recipients through projection-service's message-posted principal-context owner seam instead of expecting service-edge recipient fanout input"
    );

    assert!(
        !source.contains("recipient_ids: request.recipient_ids"),
        "notification-service should not keep threading message-posted recipient_ids from callers once notification-service owns the projection-backed recipient seam"
    );
}

#[test]
fn test_notification_service_requires_explicit_recipient_kind() {
    let source = notification_service_source();

    assert!(
        source.contains("pub recipient_kind: String"),
        "notification request and task models should require recipient_kind explicitly"
    );
    assert!(
        !source.contains("pub recipient_kind: Option<String>"),
        "notification request and task models must not model recipient_kind as optional"
    );
    assert!(
        !source.contains("resolved_request_recipient_kind("),
        "notification-service must not infer recipient_kind from auth or a default user kind"
    );
    assert!(
        !source.contains("unwrap_or_else(|| {\n        if request.recipient_id == auth.actor_id"),
        "notification-service must not default missing recipient_kind to auth.actor_kind/user"
    );
}

#[test]
fn test_notification_service_cache_keys_are_length_prefixed() {
    let source = notification_service_source();

    assert!(
        source.contains("fn scope_key_parts(parts: &[&str]) -> String"),
        "notification-service should use length-prefixed cache keys for internal notification indexes"
    );
    assert!(
        source.contains("scope_key_parts(&[tenant_id, recipient_kind, recipient_id])"),
        "notification recipient cache key should encode tenant_id, recipient_kind, and recipient_id as separate length-prefixed segments"
    );
    assert!(
        !source.contains("format!(\"{tenant_id}:{notification_id}\")"),
        "notification id cache keys must not be colon-concatenated"
    );
    assert!(
        !source.contains("format!(\"{tenant_id}:{recipient_kind}:{recipient_id}\")"),
        "recipient cache keys must not be colon-concatenated"
    );
}
