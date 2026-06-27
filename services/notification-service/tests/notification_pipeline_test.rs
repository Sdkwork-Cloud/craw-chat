use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use im_app_context::AppContext;
use im_domain_events::{AggregateType, CommitEnvelope};
use im_platform_contracts::{CommitJournal, CommitPosition, ContractError};
use projection_service::TimelineProjectionService;

#[derive(Clone, Default)]
struct RecordingJournal {
    events: Arc<Mutex<Vec<CommitEnvelope>>>,
}

impl RecordingJournal {
    fn recorded(&self) -> Vec<CommitEnvelope> {
        self.events.lock().expect("journal should lock").clone()
    }
}

impl CommitJournal for RecordingJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let mut events = self.events.lock().expect("journal should lock");
        events.push(envelope);
        Ok(CommitPosition::new("p0", events.len() as u64))
    }
}

fn auth_context(actor_id: &str, actor_kind: &str, session_id: &str) -> AppContext {
    AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: actor_id.into(),
        actor_id: actor_id.into(),
        actor_kind: actor_kind.into(),
        session_id: Some(session_id.into()),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        permission_scope: Default::default(),
        device_id: None,
    }
}

#[test]
fn test_request_notification_appends_requested_and_dispatched_events() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = notification_service::NotificationRuntime::with_journal(journal.clone());
    let auth = auth_context("1", "user", "s_demo");

    let task = runtime
        .request_notification(
            &auth,
            notification_service::RequestNotification {
                notification_id: "ntf_demo".into(),
                source_event_id: "evt_message_1".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "1105".into(),
                recipient_kind: "user".into(),
                title: Some("New message".into()),
                body: Some("hello".into()),
                payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("notification request should succeed");

    assert_eq!(task.notification_id, "ntf_demo");
    assert_eq!(task.status.as_str(), "dispatched");

    let events = journal.recorded();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].event_type, "notification.requested");
    assert_eq!(events[1].event_type, "notification.dispatched");
    assert_eq!(
        events[0].idempotency_key.as_deref(),
        Some("ntf_demo:notification.requested:1")
    );
    assert_eq!(
        events[1].idempotency_key.as_deref(),
        Some("ntf_demo:notification.dispatched:2")
    );
    assert_eq!(events[0].aggregate_type, AggregateType::Notification);
    assert_eq!(events[0].aggregate_id, "ntf_demo");
    assert_eq!(events[0].actor.actor_id, "1");
    assert_eq!(events[0].actor.actor_session_id.as_deref(), Some("s_demo"));

    let payload: serde_json::Value =
        serde_json::from_str(&events[1].payload).expect("payload should be valid json");
    assert_eq!(payload["notificationId"], "ntf_demo");
    assert_eq!(payload["recipientId"], "1105");
    assert_eq!(payload["status"], "dispatched");
}

#[test]
fn test_duplicate_request_notification_is_idempotent_when_payload_matches() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = notification_service::NotificationRuntime::with_journal(journal.clone());
    let auth = auth_context("1", "user", "s_demo");

    let first = runtime
        .request_notification_with_outcome(
            &auth,
            notification_service::RequestNotification {
                notification_id: "ntf_idempotent".into(),
                source_event_id: "evt_notification_1".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "1105".into(),
                recipient_kind: "user".into(),
                title: Some("New message".into()),
                body: Some("hello".into()),
                payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("first notification request should succeed");
    let second = runtime
        .request_notification_with_outcome(
            &auth,
            notification_service::RequestNotification {
                notification_id: "ntf_idempotent".into(),
                source_event_id: "evt_notification_1".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "1105".into(),
                recipient_kind: "user".into(),
                title: Some("New message".into()),
                body: Some("hello".into()),
                payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("duplicate notification request should be idempotent");

    assert!(first.is_new);
    assert!(!second.is_new);
    assert_eq!(first.delivery_status.as_str(), "applied");
    assert_eq!(second.delivery_status.as_str(), "replayed");
    assert_eq!(first.request_key, second.request_key);
    assert!(!first.request_key.is_empty());
    assert_eq!(second.task, first.task);

    let events = journal.recorded();
    assert_eq!(events.len(), 2);
}

#[test]
fn test_duplicate_request_notification_across_principals_keeps_stable_request_key() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = notification_service::NotificationRuntime::with_journal(journal);
    let first_auth = auth_context("1109", "user", "s_first");
    let second_auth = auth_context("1107", "user", "s_second");

    let first = runtime
        .request_notification_with_outcome(
            &first_auth,
            notification_service::RequestNotification {
                notification_id: "ntf_stable_key".into(),
                source_event_id: "evt_stable_key".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "1105".into(),
                recipient_kind: "user".into(),
                title: Some("New message".into()),
                body: Some("hello".into()),
                payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("first notification request should succeed");
    let replayed = runtime
        .request_notification_with_outcome(
            &second_auth,
            notification_service::RequestNotification {
                notification_id: "ntf_stable_key".into(),
                source_event_id: "evt_stable_key".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "1105".into(),
                recipient_kind: "user".into(),
                title: Some("New message".into()),
                body: Some("hello".into()),
                payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("cross-principal duplicate should replay");

    assert!(first.is_new);
    assert!(!replayed.is_new);
    assert_eq!(first.delivery_status.as_str(), "applied");
    assert_eq!(replayed.delivery_status.as_str(), "replayed");
    assert_eq!(replayed.request_key, first.request_key);
}

#[test]
fn test_duplicate_request_notification_rejects_conflicting_payload() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = notification_service::NotificationRuntime::with_journal(journal.clone());
    let auth = auth_context("1", "user", "s_demo");

    runtime
        .request_notification(
            &auth,
            notification_service::RequestNotification {
                notification_id: "ntf_conflict".into(),
                source_event_id: "evt_notification_1".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "1105".into(),
                recipient_kind: "user".into(),
                title: Some("New message".into()),
                body: Some("hello".into()),
                payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("first notification request should succeed");

    let error = runtime
        .request_notification(
            &auth,
            notification_service::RequestNotification {
                notification_id: "ntf_conflict".into(),
                source_event_id: "evt_notification_2".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "1110".into(),
                recipient_kind: "user".into(),
                title: Some("Changed message".into()),
                body: Some("different".into()),
                payload: Some(r#"{"conversationId":"c_other"}"#.into()),
            },
        )
        .expect_err("conflicting duplicate should be rejected");
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::CONFLICT);

    let events = journal.recorded();
    assert_eq!(events.len(), 2);
}

#[test]
fn test_request_notification_rejects_oversized_payload() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = notification_service::NotificationRuntime::with_journal(journal);
    let auth = auth_context("1", "user", "s_demo");

    let oversized_payload = "x".repeat(262145);
    let error = runtime
        .request_notification(
            &auth,
            notification_service::RequestNotification {
                notification_id: "ntf_oversized_payload".into(),
                source_event_id: "evt_notification_oversized".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "1105".into(),
                recipient_kind: "user".into(),
                title: Some("New message".into()),
                body: Some("hello".into()),
                payload: Some(oversized_payload),
            },
        )
        .expect_err("oversized notification payload should be rejected");
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
}

#[test]
fn test_request_notification_rejects_oversized_notification_id() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = notification_service::NotificationRuntime::with_journal(journal);
    let auth = auth_context("1", "user", "s_demo");

    let oversized_notification_id = "n".repeat(513);
    let error = runtime
        .request_notification(
            &auth,
            notification_service::RequestNotification {
                notification_id: oversized_notification_id,
                source_event_id: "evt_notification_oversized_id".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "1105".into(),
                recipient_kind: "user".into(),
                title: Some("New message".into()),
                body: Some("hello".into()),
                payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect_err("oversized notification id should be rejected");
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
}

#[test]
fn test_request_notification_fanout_skips_actor_and_creates_notifications_for_other_recipients() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = notification_service::NotificationRuntime::with_journal(journal.clone());
    let auth = auth_context("1", "user", "s_owner");

    let tasks = runtime
        .request_notification_fanout(
            &auth,
            notification_service::RequestNotificationFanout {
                notification_id_seed: "msg_c_demo_1".into(),
                source_event_id: "evt_message_1".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipients: BTreeSet::from([
                    notification_service::NotificationRecipient {
                        recipient_id: "1".into(),
                        recipient_kind: "user".into(),
                    },
                    notification_service::NotificationRecipient {
                        recipient_id: "1102".into(),
                        recipient_kind: "user".into(),
                    },
                    notification_service::NotificationRecipient {
                        recipient_id: "1103".into(),
                        recipient_kind: "user".into(),
                    },
                ]),
                title: Some("hello".into()),
                body: Some("hello".into()),
                payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("notification fanout should succeed");

    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].recipient_id, "1102");
    assert_eq!(tasks[1].recipient_id, "1103");

    let owner_notifications = runtime
        .list_notifications(&auth)
        .expect("owner notifications should list");
    assert!(owner_notifications.is_empty());

    let member_a_auth = AppContext {
        user_id: "1102".into(),
        actor_id: "1102".into(),
        ..auth.clone()
    };
    let member_a_notifications = runtime
        .list_notifications(&member_a_auth)
        .expect("member a notifications should list");
    assert_eq!(member_a_notifications.len(), 1);
    assert_eq!(
        member_a_notifications[0].notification_id,
        "ntf_msg_c_demo_1_user_u_member_a"
    );

    let member_b_auth = AppContext {
        user_id: "1103".into(),
        actor_id: "1103".into(),
        ..auth
    };
    let member_b_notifications = runtime
        .list_notifications(&member_b_auth)
        .expect("member b notifications should list");
    assert_eq!(member_b_notifications.len(), 1);
    assert_eq!(
        member_b_notifications[0].notification_id,
        "ntf_msg_c_demo_1_user_u_member_b"
    );

    let events = journal.recorded();
    assert_eq!(events.len(), 4);
    assert_eq!(events[0].event_type, "notification.requested");
    assert_eq!(events[1].event_type, "notification.dispatched");
    assert_eq!(events[2].event_type, "notification.requested");
    assert_eq!(events[3].event_type, "notification.dispatched");
}

#[test]
fn test_request_message_posted_notifications_resolves_current_active_recipients_from_projection_auth_context()
 {
    let journal = Arc::new(RecordingJournal::default());
    let projection_service = Arc::new(TimelineProjectionService::default());
    let runtime = notification_service::NotificationRuntime::with_journal_and_projection(
        journal.clone(),
        projection_service.clone(),
    );
    let owner_joined = CommitEnvelope::minimal(
        "evt_notification_owner_joined",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_demo",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_demo",
            "memberId":"cm_demo_owner",
            "principalId":"1",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-07T10:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let member_joined = CommitEnvelope::minimal(
        "evt_notification_member_joined",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_demo",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_demo",
            "memberId":"cm_demo_member",
            "principalId":"1108",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"1",
            "joinedAt":"2026-04-07T10:01:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let removed_joined = CommitEnvelope::minimal(
        "evt_notification_removed_joined",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_demo",
        3,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_demo",
            "memberId":"cm_demo_removed",
            "principalId":"1104",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"1",
            "joinedAt":"2026-04-07T10:02:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let removed_member = CommitEnvelope::minimal(
        "evt_notification_member_removed",
        "100001",
        "conversation.member_removed",
        "conversation",
        "c_demo",
        4,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_demo",
            "memberId":"cm_demo_removed",
            "principalId":"1104",
            "principalKind":"user",
            "role":"member",
            "state":"removed",
            "invitedBy":"1",
            "joinedAt":"2026-04-07T10:02:00Z",
            "removedAt":"2026-04-07T10:03:00Z",
            "attributes":{}
        }"#,
    );
    for event in [owner_joined, member_joined, removed_joined, removed_member] {
        projection_service
            .apply(&event)
            .expect("projection should accept conversation membership event");
    }
    let auth = auth_context("1", "user", "s_owner");

    let tasks = runtime
        .request_message_posted_notifications(
            &auth,
            notification_service::RequestMessagePostedNotifications {
                source_event_id: "evt_message_1".into(),
                conversation_id: "c_demo".into(),
                message_id: "msg_c_demo_1".into(),
                message_seq: 1,
                message_type: "text".into(),
                summary: Some("hello member".into()),
            },
        )
        .expect("message-posted notifications should succeed");

    assert_eq!(tasks.len(), 1);
    let task = &tasks[0];
    assert_eq!(task.notification_id, "ntf_msg_c_demo_1_user_u_member");
    assert_eq!(task.source_event_id, "evt_message_1");
    assert_eq!(task.source_event_type, "message.posted");
    assert_eq!(task.category, "message.new");
    assert_eq!(task.channel, "inapp");
    assert_eq!(task.recipient_id, "1108");
    assert_eq!(task.recipient_kind, "user");
    assert_eq!(task.title.as_deref(), Some("hello member"));
    assert_eq!(task.body.as_deref(), Some("hello member"));
    assert_eq!(
        task.payload.as_deref(),
        Some(
            r#"{"conversationId":"c_demo","messageId":"msg_c_demo_1","messageSeq":1,"messageType":"text"}"#
        )
    );

    let events = journal.recorded();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].event_type, "notification.requested");
    assert_eq!(events[1].event_type, "notification.dispatched");
}

#[test]
fn test_request_message_posted_notifications_includes_shared_linked_recipients_from_projection() {
    let journal = Arc::new(RecordingJournal::default());
    let projection_service = Arc::new(TimelineProjectionService::default());
    let runtime = notification_service::NotificationRuntime::with_journal_and_projection(
        journal.clone(),
        projection_service.clone(),
    );
    let owner_joined = CommitEnvelope::minimal(
        "evt_notification_shared_owner_joined",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_shared_notification",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_shared_notification",
            "memberId":"cm_shared_notification_owner",
            "principalId":"1",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-07T10:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let member_joined = CommitEnvelope::minimal(
        "evt_notification_shared_member_joined",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_shared_notification",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_shared_notification",
            "memberId":"cm_shared_notification_member",
            "principalId":"1108",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"1",
            "joinedAt":"2026-04-07T10:01:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let shared_linked = CommitEnvelope::minimal(
        "evt_notification_shared_linked",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_shared_notification",
        3,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_shared_notification",
            "memberId":"cm_shared_notification_external",
            "principalId":"1100",
            "principalKind":"external_user",
            "role":"member",
            "state":"linked",
            "invitedBy":"1",
            "joinedAt":"2026-04-07T10:02:00Z",
            "removedAt":null,
            "attributes":{
                "sharedChannelPolicyId":"scp_demo",
                "externalConnectionId":"conn_demo",
                "externalMemberId":"ext_demo"
            }
        }"#,
    );
    for event in [owner_joined, member_joined, shared_linked] {
        projection_service
            .apply(&event)
            .expect("projection should accept shared notification membership event");
    }
    let auth = auth_context("1", "user", "s_owner");

    let tasks = runtime
        .request_message_posted_notifications(
            &auth,
            notification_service::RequestMessagePostedNotifications {
                source_event_id: "evt_shared_message_1".into(),
                conversation_id: "c_shared_notification".into(),
                message_id: "msg_c_shared_notification_1".into(),
                message_seq: 1,
                message_type: "text".into(),
                summary: Some("hello shared member".into()),
            },
        )
        .expect("message-posted notifications should include shared linked members");

    assert_eq!(tasks.len(), 2);
    let recipient_ids = tasks
        .iter()
        .map(|task| task.recipient_id.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        recipient_ids,
        BTreeSet::from(["1108", "1100"])
    );

    let events = journal.recorded();
    assert_eq!(events.len(), 4);
    assert_eq!(events[0].event_type, "notification.requested");
    assert_eq!(events[1].event_type, "notification.dispatched");
    assert_eq!(events[2].event_type, "notification.requested");
    assert_eq!(events[3].event_type, "notification.dispatched");
}

#[test]
fn test_request_automation_result_notification_targets_requesting_actor_idempotently() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = notification_service::NotificationRuntime::with_journal(journal.clone());
    let auth = auth_context("1", "user", "s_demo");

    let first = runtime
        .request_automation_result_notification(
            &auth,
            notification_service::RequestAutomationResultNotification {
                execution_id: "ae_demo".into(),
                target_ref: "wf_demo".into(),
                output_payload: Some(r#"{"status":"ok"}"#.into()),
            },
        )
        .expect("automation result notification should succeed");
    let second = runtime
        .request_automation_result_notification(
            &auth,
            notification_service::RequestAutomationResultNotification {
                execution_id: "ae_demo".into(),
                target_ref: "wf_demo".into(),
                output_payload: Some(r#"{"status":"ok"}"#.into()),
            },
        )
        .expect("duplicate automation result notification should be idempotent");

    assert_eq!(second, first);
    assert_eq!(first.notification_id, "ntf_automation_user_ae_demo");
    assert_eq!(first.source_event_type, "automation.execution_completed");
    assert_eq!(first.category, "automation.result");
    assert_eq!(first.channel, "inapp");
    assert_eq!(first.recipient_id, "1");
    assert_eq!(first.title.as_deref(), Some("Automation completed"));
    assert_eq!(first.body.as_deref(), Some("wf_demo"));
    assert_eq!(first.payload.as_deref(), Some(r#"{"status":"ok"}"#));

    let notifications = runtime
        .list_notifications(&auth)
        .expect("automation notifications should list");
    assert_eq!(notifications.len(), 1);
    assert_eq!(notifications[0], first);

    let events = journal.recorded();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].event_type, "notification.requested");
    assert_eq!(events[1].event_type, "notification.dispatched");
}

#[test]
fn test_request_automation_result_notification_isolated_by_actor_kind_for_same_execution_id() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = notification_service::NotificationRuntime::with_journal(journal.clone());
    let user_auth = auth_context("1", "user", "s_user");
    let system_auth = auth_context("1", "system", "s_system");

    let user_task = runtime
        .request_automation_result_notification(
            &user_auth,
            notification_service::RequestAutomationResultNotification {
                execution_id: "ae_shared".into(),
                target_ref: "wf_demo".into(),
                output_payload: Some(r#"{"status":"ok"}"#.into()),
            },
        )
        .expect("user automation notification should succeed");
    let system_task = runtime
        .request_automation_result_notification(
            &system_auth,
            notification_service::RequestAutomationResultNotification {
                execution_id: "ae_shared".into(),
                target_ref: "wf_demo".into(),
                output_payload: Some(r#"{"status":"ok"}"#.into()),
            },
        )
        .expect("system automation notification should succeed");

    assert_eq!(user_task.notification_id, "ntf_automation_user_ae_shared");
    assert_eq!(
        user_task.source_event_id,
        "evt_user_ae_shared_automation_execution_completed"
    );
    assert_eq!(user_task.recipient_kind, "user");
    assert_eq!(
        system_task.notification_id,
        "ntf_automation_system_ae_shared"
    );
    assert_eq!(
        system_task.source_event_id,
        "evt_system_ae_shared_automation_execution_completed"
    );
    assert_eq!(system_task.recipient_kind, "system");

    let user_notifications = runtime
        .list_notifications(&user_auth)
        .expect("user notifications should list");
    assert_eq!(user_notifications, vec![user_task.clone()]);

    let system_notifications = runtime
        .list_notifications(&system_auth)
        .expect("system notifications should list");
    assert_eq!(system_notifications, vec![system_task.clone()]);

    let events = journal.recorded();
    assert_eq!(events.len(), 4);
    assert_eq!(events[0].aggregate_id, "ntf_automation_user_ae_shared");
    assert_eq!(events[2].aggregate_id, "ntf_automation_system_ae_shared");
}

#[test]
fn test_notification_queries_are_isolated_by_actor_kind() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = notification_service::NotificationRuntime::with_journal(journal);
    let sender_auth = auth_context("1106", "user", "s_sender");
    let user_auth = auth_context("1", "user", "s_user");
    let system_auth = auth_context("1", "system", "s_system");

    runtime
        .request_notification(
            &sender_auth,
            notification_service::RequestNotification {
                notification_id: "ntf_actor_kind_isolation".into(),
                source_event_id: "evt_actor_kind_isolation".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "1".into(),
                recipient_kind: "user".into(),
                title: Some("New message".into()),
                body: Some("hello".into()),
                payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("notification request should succeed");

    let user_notifications = runtime
        .list_notifications(&user_auth)
        .expect("user notifications should list");
    assert_eq!(user_notifications.len(), 1);
    assert_eq!(
        user_notifications[0].notification_id,
        "ntf_actor_kind_isolation"
    );

    let system_notifications = runtime
        .list_notifications(&system_auth)
        .expect("system notifications should list");
    assert!(
        system_notifications.is_empty(),
        "a different actor_kind with the same actor_id must not share the inbox"
    );

    let error = runtime
        .get_notification(&system_auth, "ntf_actor_kind_isolation")
        .expect_err(
            "a different actor_kind with the same actor_id must not fetch the notification",
        );
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
}
