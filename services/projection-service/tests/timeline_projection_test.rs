use im_app_context::AppContext;
use im_domain_core::conversation::MembershipRole;
use projection_service::{
    MessageReactionCountView, NotificationRecipientView, RealtimeFanoutTarget,
    TimelineProjectionService, TimelineViewEntry,
};
use std::thread::sleep;
use std::time::Duration;

fn app_context(
    tenant_id: &str,
    actor_id: &str,
    actor_kind: &str,
    session_id: Option<&str>,
    device_id: Option<&str>,
) -> AppContext {
    AppContext {
        tenant_id: tenant_id.into(),
        organization_id: "0".to_owned(),
        user_id: actor_id.into(),
        session_id: session_id.map(str::to_owned),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        permission_scope: Default::default(),
        actor_id: actor_id.into(),
        actor_kind: actor_kind.into(),
        device_id: device_id.map(str::to_owned),
    }
}

#[test]
fn test_message_posted_event_projects_into_timeline_view() {
    let service = TimelineProjectionService::default();

    let event = im_domain_events::CommitEnvelope::minimal(
        "evt_demo",
        "100001",
        "message.posted",
        "conversation",
        "c_demo",
        1,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_demo",
            "messageId":"m_demo",
            "messageSeq":1,
            "sender":{"id":"1","kind":"user","memberId":"cm_demo","deviceId":"d_demo","sessionId":"s_demo","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_demo",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"hello","parts":[{"kind":"text","text":"hello"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:01Z",
            "committedAt":"2026-04-05T10:00:01Z"
        }"#,
    );

    service.apply(&event).expect("projection should succeed");

    assert_eq!(
        service.timeline("100001", "default", "c_demo"),
        vec![TimelineViewEntry {
            tenant_id: "100001".into(),
            conversation_id: "c_demo".into(),
            message_id: "m_demo".into(),
            message_seq: 1,
            summary: Some("hello".into()),
            sender: im_domain_core::message::Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: Some("cm_demo".into()),
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            body: im_domain_core::message::MessageBody {
                summary: Some("hello".into()),
                parts: vec![im_domain_core::message::ContentPart::text("hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
            message_type: im_domain_core::message::MessageType::Standard,
            delivery_mode: "discrete".into(),
            client_msg_id: Some("client_demo".into()),
            stream_session_id: None,
            rtc_session_id: None,
            occurred_at: "2026-04-05T10:00:01Z".into(),
            committed_at: Some("2026-04-05T10:00:01Z".into()),
            retention_until: Some("2027-04-05T10:00:01.000Z".into()),
        }]
    );
}

#[test]
fn test_timeline_window_returns_cursor_metadata_and_rejects_oversized_limit() {
    let service = TimelineProjectionService::default();

    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_page_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_page",
        0,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_page",
            "memberId":"cm_demo",
            "principalId":"1",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-05T10:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    service
        .apply(&member_joined)
        .expect("member projection should succeed");

    for seq in 1..=3 {
        let event = im_domain_events::CommitEnvelope::minimal(
            &format!("evt_page_{seq}"),
            "100001",
            "message.posted",
            "conversation",
            "c_page",
            seq,
        )
        .with_payload(
            "message.posted.v1",
            &serde_json::json!({
                "tenantId":"100001",
                "conversationId":"c_page",
                "messageId":format!("m_page_{seq}"),
                "messageSeq":seq,
                "sender":{"id":"1","kind":"user","memberId":"cm_demo","deviceId":"d_demo","sessionId":"s_demo","metadata":{}},
                "messageType":"standard",
                "deliveryMode":"discrete",
                "clientMsgId":format!("client_page_{seq}"),
                "streamSessionId":null,
                "rtcSessionId":null,
                "body":{"summary":format!("message {seq}"),"parts":[{"kind":"text","text":format!("message {seq}")}],"renderHints":{}},
                "attributes":{},
                "metadata":{},
                "occurredAt":format!("2026-04-05T10:00:0{seq}Z"),
                "committedAt":format!("2026-04-05T10:00:0{seq}Z")
            })
            .to_string(),
        );
        service.apply(&event).expect("projection should succeed");
    }

    let first = service.timeline_window("100001", "default", "c_page", Some(0), 2);
    assert_eq!(
        first
            .items
            .iter()
            .map(|entry| entry.message_seq)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(first.next_after_seq, Some(2));
    assert!(first.has_more);

    let second = service.timeline_window("100001", "default", "c_page", Some(2), 2);
    assert_eq!(
        second
            .items
            .iter()
            .map(|entry| entry.message_seq)
            .collect::<Vec<_>>(),
        vec![3]
    );
    assert_eq!(second.next_after_seq, Some(3));
    assert!(!second.has_more);

    let auth = app_context("100001", "1", "user", None, None);
    let invalid = service
        .timeline_window_from_auth_context(&auth, "c_page", Some(0), Some(1001))
        .expect_err("oversized limit should be rejected");
    assert_eq!(invalid.code(), "limit_invalid");
}

#[test]
fn test_timeline_projection_stores_entries_in_message_sequence_order() {
    let service = TimelineProjectionService::default();

    for seq in [2_u64, 1_u64] {
        let event = im_domain_events::CommitEnvelope::minimal(
            &format!("evt_ordered_{seq}"),
            "100001",
            "message.posted",
            "conversation",
            "c_ordered",
            seq,
        )
        .with_payload(
            "message.posted.v1",
            &serde_json::json!({
                "tenantId":"100001",
                "conversationId":"c_ordered",
                "messageId":format!("m_ordered_{seq}"),
                "messageSeq":seq,
                "sender":{"id":"1","kind":"user","memberId":"cm_demo","deviceId":"d_demo","sessionId":"s_demo","metadata":{}},
                "messageType":"standard",
                "deliveryMode":"discrete",
                "clientMsgId":format!("client_ordered_{seq}"),
                "streamSessionId":null,
                "rtcSessionId":null,
                "body":{"summary":format!("message {seq}"),"parts":[{"kind":"text","text":format!("message {seq}")}],"renderHints":{}},
                "attributes":{},
                "metadata":{},
                "occurredAt":format!("2026-04-05T10:00:0{seq}Z"),
                "committedAt":format!("2026-04-05T10:00:0{seq}Z")
            })
            .to_string(),
        );
        service.apply(&event).expect("projection should succeed");
    }

    assert_eq!(
        service
            .timeline("100001", "default", "c_ordered")
            .iter()
            .map(|entry| entry.message_seq)
            .collect::<Vec<_>>(),
        vec![1, 2],
        "projection store should keep timeline entries ordered so cursor reads do not sort full histories"
    );
}

#[test]
fn test_same_conversation_id_is_isolated_per_tenant_in_projection() {
    let service = TimelineProjectionService::default();

    let alpha_event = im_domain_events::CommitEnvelope::minimal(
        "evt_alpha",
        "t_alpha",
        "message.posted",
        "conversation",
        "c_shared",
        1,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"t_alpha",
            "conversationId":"c_shared",
            "messageId":"m_alpha",
            "messageSeq":1,
            "sender":{"id":"1010","kind":"user","memberId":"cm_alpha","deviceId":"d_alpha","sessionId":"s_alpha","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_alpha",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"alpha","parts":[{"kind":"text","text":"alpha"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:01Z",
            "committedAt":"2026-04-05T10:00:01Z"
        }"#,
    );
    let beta_event = im_domain_events::CommitEnvelope::minimal(
        "evt_beta",
        "t_beta",
        "message.posted",
        "conversation",
        "c_shared",
        1,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"t_beta",
            "conversationId":"c_shared",
            "messageId":"m_beta",
            "messageSeq":1,
            "sender":{"id":"agent_beta","kind":"agent","memberId":"cm_beta","deviceId":null,"sessionId":"s_beta","metadata":{"agentMode":"handoff"}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_beta",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"beta","parts":[{"kind":"text","text":"beta"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:01Z",
            "committedAt":"2026-04-05T10:00:01Z"
        }"#,
    );

    service
        .apply(&alpha_event)
        .expect("alpha projection should succeed");
    service
        .apply(&beta_event)
        .expect("beta projection should succeed");

    assert_eq!(service.timeline("t_alpha", "default", "c_shared").len(), 1);
    assert_eq!(
        service.timeline("t_alpha", "default", "c_shared")[0]
            .summary
            .as_deref(),
        Some("alpha")
    );
    assert_eq!(service.timeline("t_beta", "default", "c_shared").len(), 1);
    assert_eq!(
        service.timeline("t_beta", "default", "c_shared")[0].summary.as_deref(),
        Some("beta")
    );
}

#[test]
fn test_projection_scope_key_is_segment_safe_for_delimiter_bearing_ids() {
    let service = TimelineProjectionService::default();

    let left_event = im_domain_events::CommitEnvelope::minimal(
        "evt_segment_left",
        "tenant:a",
        "message.posted",
        "conversation",
        "b",
        1,
    )
    .with_payload(
        "message.posted.v1",
        &serde_json::json!({
            "tenantId":"tenant:a",
            "conversationId":"b",
            "messageId":"m_left",
            "messageSeq":1,
            "sender":{"id":"1011","kind":"user","memberId":"cm_left","deviceId":"d_left","sessionId":"s_left","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_left",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"left","parts":[{"kind":"text","text":"left"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:01Z",
            "committedAt":"2026-04-05T10:00:01Z"
        })
        .to_string(),
    );
    let right_event = im_domain_events::CommitEnvelope::minimal(
        "evt_segment_right",
        "tenant",
        "message.posted",
        "conversation",
        "a:b",
        1,
    )
    .with_payload(
        "message.posted.v1",
        &serde_json::json!({
            "tenantId":"tenant",
            "conversationId":"a:b",
            "messageId":"m_right",
            "messageSeq":1,
            "sender":{"id":"1012","kind":"user","memberId":"cm_right","deviceId":"d_right","sessionId":"s_right","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_right",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"right","parts":[{"kind":"text","text":"right"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:02Z",
            "committedAt":"2026-04-05T10:00:02Z"
        })
        .to_string(),
    );

    service
        .apply(&left_event)
        .expect("left projection should succeed");
    service
        .apply(&right_event)
        .expect("right projection should succeed");

    assert_eq!(
        service
            .timeline("tenant:a", "default", "b")
            .iter()
            .map(|entry| entry.message_id.as_str())
            .collect::<Vec<_>>(),
        vec!["m_left"]
    );
    assert_eq!(
        service
            .timeline("tenant", "default", "a:b")
            .iter()
            .map(|entry| entry.message_id.as_str())
            .collect::<Vec<_>>(),
        vec!["m_right"]
    );
}

#[test]
fn test_message_posted_event_projects_into_conversation_summary_view() {
    let service = TimelineProjectionService::default();

    let first_event = im_domain_events::CommitEnvelope::minimal(
        "evt_first",
        "100001",
        "message.posted",
        "conversation",
        "c_summary",
        1,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_summary",
            "messageId":"m_first",
            "messageSeq":1,
            "sender":{"id":"1","kind":"user","memberId":"cm_demo","deviceId":"d_demo","sessionId":"s_demo","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_first",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"first","parts":[{"kind":"text","text":"hello"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:01Z",
            "committedAt":"2026-04-05T10:00:01Z"
        }"#,
    );
    let second_event = im_domain_events::CommitEnvelope::minimal(
        "evt_second",
        "100001",
        "message.posted",
        "conversation",
        "c_summary",
        2,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_summary",
            "messageId":"m_second",
            "messageSeq":2,
            "sender":{"id":"agent_demo","kind":"agent","memberId":null,"deviceId":null,"sessionId":"s_agent","metadata":{"agentId":"ag_demo"}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_second",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"second","parts":[{"kind":"text","text":"world"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:02Z",
            "committedAt":"2026-04-05T10:00:02Z"
        }"#,
    );

    service
        .apply(&first_event)
        .expect("first projection should succeed");
    service
        .apply(&second_event)
        .expect("second projection should succeed");

    let summary = service
        .conversation_summary("100001", "default", "c_summary")
        .expect("summary should exist");

    assert_eq!(summary.message_count, 2);
    assert_eq!(summary.last_message_id.as_deref(), Some("m_second"));
    assert_eq!(summary.last_message_seq, 2);
    assert_eq!(summary.last_sender_id.as_deref(), Some("agent_demo"));
    assert_eq!(summary.last_sender_kind.as_deref(), Some("agent"));
    assert_eq!(summary.last_summary.as_deref(), Some("second"));
}

#[test]
fn test_read_cursor_event_projects_into_cursor_view_with_unread_count() {
    let service = TimelineProjectionService::default();

    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_cursor",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_cursor",
            "memberId":"cm_demo",
            "principalId":"1",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-05T10:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let message_posted = im_domain_events::CommitEnvelope::minimal(
        "evt_message",
        "100001",
        "message.posted",
        "conversation",
        "c_cursor",
        2,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_cursor",
            "messageId":"m_cursor_2",
            "messageSeq":2,
            "sender":{"id":"1013","kind":"user","memberId":"cm_other","deviceId":null,"sessionId":"s_other","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_2",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"second","parts":[{"kind":"text","text":"second"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:02Z",
            "committedAt":"2026-04-05T10:00:02Z"
        }"#,
    );
    let read_cursor_updated = im_domain_events::CommitEnvelope::minimal(
        "evt_cursor",
        "100001",
        "conversation.read_cursor_updated",
        "conversation",
        "c_cursor",
        1,
    )
    .with_payload(
        "conversation.read_cursor.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_cursor",
            "memberId":"cm_demo",
            "principalId":"1",
            "principalKind":"user",
            "readSeq":1,
            "lastReadMessageId":"m_cursor_1",
            "updatedAt":"2026-04-05T10:00:10Z"
        }"#,
    );

    service
        .apply(&member_joined)
        .expect("member projection should succeed");
    service
        .apply(&message_posted)
        .expect("message projection should succeed");
    service
        .apply(&read_cursor_updated)
        .expect("read cursor projection should succeed");

    let cursor = service
        .read_cursor_for_principal_kind("100001", "default", "c_cursor", "1", "user")
        .expect("cursor should exist");
    assert_eq!(cursor.member_id, "cm_demo");
    assert_eq!(cursor.read_seq, 1);
    assert_eq!(cursor.last_read_message_id.as_deref(), Some("m_cursor_1"));
    assert_eq!(cursor.unread_count, 1);
}

#[test]
fn test_member_role_changed_event_updates_member_snapshot() {
    let service = TimelineProjectionService::default();

    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_member_joined",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_role_projection",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_role_projection",
            "memberId":"cm_c_role_projection_user_u_member",
            "principalId":"1014",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"1",
            "joinedAt":"2026-04-06T10:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let role_changed = im_domain_events::CommitEnvelope::minimal(
        "evt_member_role_changed",
        "100001",
        "conversation.member_role_changed",
        "conversation",
        "c_role_projection",
        2,
    )
    .with_payload(
        "conversation.member_role_changed.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_role_projection",
            "previousMember":{
                "tenantId":"100001",
                "conversationId":"c_role_projection",
                "memberId":"cm_c_role_projection_user_u_member",
                "principalId":"1014",
                "principalKind":"user",
                "role":"member",
                "state":"joined",
                "invitedBy":"1",
                "joinedAt":"2026-04-06T10:00:00Z",
                "removedAt":null,
                "attributes":{}
            },
            "updatedMember":{
                "tenantId":"100001",
                "conversationId":"c_role_projection",
                "memberId":"cm_c_role_projection_user_u_member",
                "principalId":"1014",
                "principalKind":"user",
                "role":"admin",
                "state":"joined",
                "invitedBy":"1",
                "joinedAt":"2026-04-06T10:00:00Z",
                "removedAt":null,
                "attributes":{}
            },
            "changedAt":"2026-04-06T10:01:00Z"
        }"#,
    );

    service
        .apply(&member_joined)
        .expect("member joined projection should succeed");
    service
        .apply(&role_changed)
        .expect("role changed projection should succeed");

    let member = service
        .member_snapshot_for_principal_kind("100001", "default", "c_role_projection", "1014", "user")
        .expect("member snapshot should exist");
    assert_eq!(member.role, MembershipRole::Admin);
}

#[test]
fn test_inbox_view_projects_member_summary_and_unread_count() {
    let service = TimelineProjectionService::default();

    let conversation_created = im_domain_events::CommitEnvelope::minimal(
        "evt_conversation",
        "100001",
        "conversation.created",
        "conversation",
        "c_inbox",
        0,
    )
    .with_payload(
        "conversation.created.v1",
        r#"{
            "conversationId":"c_inbox",
            "conversationType":"group"
        }"#,
    );
    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_inbox",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_inbox",
            "memberId":"cm_inbox_demo",
            "principalId":"1",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-05T10:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let message_posted = im_domain_events::CommitEnvelope::minimal(
        "evt_message",
        "100001",
        "message.posted",
        "conversation",
        "c_inbox",
        2,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_inbox",
            "messageId":"m_inbox_2",
            "messageSeq":2,
            "sender":{"id":"1013","kind":"user","memberId":"cm_other","deviceId":null,"sessionId":"s_other","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_2",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"second","parts":[{"kind":"text","text":"second"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:02Z",
            "committedAt":"2026-04-05T10:00:02Z"
        }"#,
    );
    let cursor_updated = im_domain_events::CommitEnvelope::minimal(
        "evt_cursor",
        "100001",
        "conversation.read_cursor_updated",
        "conversation",
        "c_inbox",
        1,
    )
    .with_payload(
        "conversation.read_cursor.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_inbox",
            "memberId":"cm_inbox_demo",
            "principalId":"1",
            "principalKind":"user",
            "readSeq":1,
            "lastReadMessageId":"m_inbox_1",
            "updatedAt":"2026-04-05T10:00:10Z"
        }"#,
    );

    service
        .apply(&conversation_created)
        .expect("conversation projection should succeed");
    service
        .apply(&member_joined)
        .expect("member projection should succeed");
    service
        .apply(&message_posted)
        .expect("message projection should succeed");
    service
        .apply(&cursor_updated)
        .expect("cursor projection should succeed");

    let inbox = service.inbox_for_principal_kind("100001", "1", "user");
    assert_eq!(inbox.len(), 1);
    assert_eq!(inbox[0].conversation_id, "c_inbox");
    assert_eq!(inbox[0].conversation_type, "group");
    assert_eq!(inbox[0].message_count, 2);
    assert_eq!(inbox[0].last_message_id.as_deref(), Some("m_inbox_2"));
    assert_eq!(inbox[0].last_sender_id.as_deref(), Some("1013"));
    assert_eq!(inbox[0].unread_count, 1);
}

#[test]
fn test_inbox_view_projects_direct_peer_display_from_member_attributes() {
    let service = TimelineProjectionService::default();

    let conversation_created = im_domain_events::CommitEnvelope::minimal(
        "evt_direct_display_conversation",
        "100001",
        "conversation.created",
        "conversation",
        "c_direct_display",
        0,
    )
    .with_payload(
        "conversation.created.v1",
        r#"{
            "conversationId":"c_direct_display",
            "conversationType":"single"
        }"#,
    );
    let current_member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_direct_display_current",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_direct_display",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_direct_display",
            "memberId":"cm_direct_display_current",
            "principalId":"1015",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-05T10:00:00Z",
            "removedAt":null,
            "attributes":{"directChatRole":"initiator","peerActorId":"1016","peerActorKind":"user"}
        }"#,
    );
    let peer_member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_direct_display_peer",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_direct_display",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_direct_display",
            "memberId":"cm_direct_display_alice",
            "principalId":"1016",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"1015",
            "joinedAt":"2026-04-05T10:00:01Z",
            "removedAt":null,
            "attributes":{
                "directChatRole":"peer",
                "peerActorId":"1015",
                "peerActorKind":"user",
                "chatId":"alice-chat-id",
                "displayName":"Alice Chen",
                "avatarUrl":"https://cdn.example.test/alice.png",
                "relationshipState":"active"
            }
        }"#,
    );

    for event in [
        &conversation_created,
        &current_member_joined,
        &peer_member_joined,
    ] {
        service.apply(event).expect("projection event should apply");
    }

    let inbox = service.inbox_for_principal_kind("100001", "1015", "user");
    assert_eq!(inbox.len(), 1);
    assert_eq!(inbox[0].conversation_id, "c_direct_display");
    assert_eq!(inbox[0].conversation_type, "single");
    assert_eq!(inbox[0].display_name.as_deref(), Some("Alice Chen"));
    assert_eq!(
        inbox[0].avatar_url.as_deref(),
        Some("https://cdn.example.test/alice.png")
    );
    assert_eq!(
        inbox[0].display_source.as_deref(),
        Some("member_projection")
    );

    let peer = inbox[0]
        .peer
        .as_ref()
        .expect("direct inbox should expose peer");
    assert_eq!(peer.principal_kind, "user");
    assert_eq!(peer.principal_id, "1016");
    assert_eq!(peer.user_id.as_deref(), Some("1016"));
    assert_eq!(peer.chat_id.as_deref(), Some("alice-chat-id"));
    assert_eq!(peer.display_name.as_deref(), Some("Alice Chen"));
    assert_eq!(
        peer.avatar_url.as_deref(),
        Some("https://cdn.example.test/alice.png")
    );
    assert_eq!(peer.relationship_state.as_deref(), Some("active"));
}

#[test]
fn test_inbox_unread_count_excludes_messages_sent_by_current_principal() {
    let service = TimelineProjectionService::default();

    let conversation_created = im_domain_events::CommitEnvelope::minimal(
        "evt_received_unread_conversation",
        "100001",
        "conversation.created",
        "conversation",
        "c_received_unread_projection",
        0,
    )
    .with_payload(
        "conversation.created.v1",
        r#"{
            "conversationId":"c_received_unread_projection",
            "conversationType":"group"
        }"#,
    );
    let owner_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_received_unread_owner",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_received_unread_projection",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_received_unread_projection",
            "memberId":"cm_received_unread_owner",
            "principalId":"1",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-05T10:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let friend_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_received_unread_friend",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_received_unread_projection",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_received_unread_projection",
            "memberId":"cm_received_unread_friend",
            "principalId":"1017",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"1",
            "joinedAt":"2026-04-05T10:00:01Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let owner_message = im_domain_events::CommitEnvelope::minimal(
        "evt_received_unread_owner_message",
        "100001",
        "message.posted",
        "conversation",
        "c_received_unread_projection",
        1,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_received_unread_projection",
            "messageId":"m_received_unread_1",
            "messageSeq":1,
            "sender":{"id":"1","kind":"user","memberId":"cm_received_unread_owner","deviceId":null,"sessionId":"s_owner","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_received_unread_owner",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"owner note","parts":[{"kind":"text","text":"owner note"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:02Z",
            "committedAt":"2026-04-05T10:00:02Z"
        }"#,
    );
    let friend_message = im_domain_events::CommitEnvelope::minimal(
        "evt_received_unread_friend_message",
        "100001",
        "message.posted",
        "conversation",
        "c_received_unread_projection",
        2,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_received_unread_projection",
            "messageId":"m_received_unread_2",
            "messageSeq":2,
            "sender":{"id":"1017","kind":"user","memberId":"cm_received_unread_friend","deviceId":null,"sessionId":"s_friend","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_received_unread_friend",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"friend reply","parts":[{"kind":"text","text":"friend reply"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:03Z",
            "committedAt":"2026-04-05T10:00:03Z"
        }"#,
    );

    for event in [
        &conversation_created,
        &owner_joined,
        &friend_joined,
        &owner_message,
        &friend_message,
    ] {
        service.apply(event).expect("projection event should apply");
    }

    let owner_inbox = service.inbox_for_principal_kind("100001", "1", "user");
    assert_eq!(owner_inbox.len(), 1);
    assert_eq!(
        owner_inbox[0].unread_count, 1,
        "owner inbox should count only the friend's received message as unread"
    );

    let friend_inbox = service.inbox_for_principal_kind("100001", "1017", "user");
    assert_eq!(friend_inbox.len(), 1);
    assert_eq!(
        friend_inbox[0].unread_count, 1,
        "friend inbox should count only the owner's received message as unread"
    );

    let owner_cursor = service
        .read_cursor_for_principal_kind("100001", "default", "c_received_unread_projection", "1", "user")
        .expect("owner cursor should exist");
    assert_eq!(
        owner_cursor.unread_count, 1,
        "read cursor unreadCount should share the same received-message semantics"
    );
}

#[test]
fn test_inbox_from_auth_context_isolates_same_actor_id_by_principal_kind() {
    let service = TimelineProjectionService::default();

    for (conversation_id, conversation_type, event_id) in [
        (
            "c_typed_inbox_user",
            "group",
            "evt_typed_inbox_user_created",
        ),
        (
            "c_typed_inbox_agent",
            "support",
            "evt_typed_inbox_agent_created",
        ),
    ] {
        let payload = format!(
            r#"{{
                "conversationId":"{conversation_id}",
                "conversationType":"{conversation_type}"
            }}"#
        );
        service
            .apply(
                &im_domain_events::CommitEnvelope::minimal(
                    event_id,
                    "100001",
                    "conversation.created",
                    "conversation",
                    conversation_id,
                    0,
                )
                .with_payload("conversation.created.v1", payload.as_str()),
            )
            .expect("typed inbox conversation projection should succeed");
    }

    for (event_id, conversation_id, member_id, principal_kind, role) in [
        (
            "evt_typed_inbox_user_member",
            "c_typed_inbox_user",
            "cm_typed_inbox_user",
            "user",
            "owner",
        ),
        (
            "evt_typed_inbox_agent_member",
            "c_typed_inbox_agent",
            "cm_typed_inbox_agent",
            "agent",
            "member",
        ),
    ] {
        let payload = format!(
            r#"{{
                "tenantId":"100001",
                "conversationId":"{conversation_id}",
                "memberId":"{member_id}",
                "principalId":"1018",
                "principalKind":"{principal_kind}",
                "role":"{role}",
                "state":"joined",
                "invitedBy":null,
                "joinedAt":"2026-04-05T10:00:00Z",
                "removedAt":null,
                "attributes":{{}}
            }}"#
        );
        service
            .apply(
                &im_domain_events::CommitEnvelope::minimal(
                    event_id,
                    "100001",
                    "conversation.member_joined",
                    "conversation",
                    conversation_id,
                    1,
                )
                .with_payload("conversation.member.v1", payload.as_str()),
            )
            .expect("typed inbox member projection should succeed");
    }

    let user_auth = app_context("100001", "1018", "user", Some("s_typed_inbox_user"), None);
    let agent_auth = app_context(
        "100001",
        "1018",
        "agent",
        Some("s_typed_inbox_agent"),
        None,
    );

    let user_inbox = service.inbox_from_auth_context(&user_auth);
    assert_eq!(user_inbox.len(), 1);
    assert_eq!(user_inbox[0].conversation_id, "c_typed_inbox_user");
    assert_eq!(user_inbox[0].member_id, "cm_typed_inbox_user");

    let agent_inbox = service.inbox_from_auth_context(&agent_auth);
    assert_eq!(agent_inbox.len(), 1);
    assert_eq!(agent_inbox[0].conversation_id, "c_typed_inbox_agent");
    assert_eq!(agent_inbox[0].member_id, "cm_typed_inbox_agent");
}

#[test]
fn test_client_route_sync_feed_projects_registered_client_routes_for_message_and_read_cursor_events()
 {
    let service = TimelineProjectionService::default();

    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_sync",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_sync",
            "memberId":"cm_demo",
            "principalId":"1",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-05T10:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let message_posted = im_domain_events::CommitEnvelope::minimal(
        "evt_message",
        "100001",
        "message.posted",
        "conversation",
        "c_sync",
        2,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_sync",
            "messageId":"msg_c_sync_1",
            "messageSeq":1,
            "sender":{"id":"1","kind":"user","memberId":"cm_demo","deviceId":"d_phone","sessionId":"s_demo","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_sync_1",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"sync hello","parts":[{"kind":"text","text":"sync hello"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:02Z",
            "committedAt":"2026-04-05T10:00:02Z"
        }"#,
    );
    let cursor_updated = im_domain_events::CommitEnvelope::minimal(
        "evt_cursor",
        "100001",
        "conversation.read_cursor_updated",
        "conversation",
        "c_sync",
        1,
    )
    .with_payload(
        "conversation.read_cursor.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_sync",
            "memberId":"cm_demo",
            "principalId":"1",
            "principalKind":"user",
            "readSeq":1,
            "lastReadMessageId":"msg_c_sync_1",
            "updatedAt":"2026-04-05T10:00:10Z"
        }"#,
    );

    service
        .apply(&member_joined)
        .expect("member projection should succeed");
    service.register_client_route("100001", "default", "1", "d_phone");
    service.register_client_route("100001", "default", "1", "d_pad");
    service
        .apply(&message_posted)
        .expect("message projection should succeed");
    service
        .apply(&cursor_updated)
        .expect("cursor projection should succeed");

    let feed = service
        .client_route_sync_feed_window_for_principal_kind(
            "100001", "default", "1",
            "user",
            "d_pad",
            Some(0),
            100,
        )
        .items;
    assert_eq!(feed.len(), 2);
    assert_eq!(feed[0].sync_seq, 1);
    assert_eq!(feed[0].origin_event_type, "message.posted");
    assert_eq!(feed[0].message_id.as_deref(), Some("msg_c_sync_1"));
    assert_eq!(feed[0].actor_device_id.as_deref(), Some("d_phone"));
    assert_eq!(feed[1].sync_seq, 2);
    assert_eq!(
        feed[1].origin_event_type,
        "conversation.read_cursor_updated"
    );
    assert_eq!(feed[1].read_seq, Some(1));
    assert_eq!(
        feed[1].last_read_message_id.as_deref(),
        Some("msg_c_sync_1")
    );
}

#[test]
fn test_rtc_signal_message_client_route_sync_feed_preserves_message_payload_for_state_backfill() {
    let service = TimelineProjectionService::default();

    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_rtc_sync_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_rtc_sync",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_rtc_sync",
            "memberId":"cm_rtc_sync_alice",
            "principalId":"1016",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-05T10:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let rtc_signal_message = im_domain_events::CommitEnvelope::minimal(
        "evt_rtc_signal_message",
        "100001",
        "message.posted",
        "conversation",
        "c_rtc_sync",
        2,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_rtc_sync",
            "messageId":"msg_rtc_signal_1",
            "messageSeq":1,
            "sender":{"id":"1016","kind":"user","memberId":"cm_rtc_sync_alice","deviceId":"d_phone","sessionId":"s_demo","metadata":{}},
            "messageType":"signal",
            "deliveryMode":"discrete",
            "clientMsgId":null,
            "streamSessionId":null,
            "rtcSessionId":"rtc_sync_1",
            "body":{
                "summary":"rtc.accept",
                "parts":[{
                    "kind":"signal",
                    "signalType":"rtc.accept",
                    "schemaRef":"rtc.signal.v1",
                    "payload":"{\"rtcSessionId\":\"rtc_sync_1\",\"conversationId\":\"c_rtc_sync\",\"rtcMode\":\"video\",\"state\":\"accepted\"}"
                }],
                "renderHints":{"channel":"rtc"}
            },
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:02Z",
            "committedAt":"2026-04-05T10:00:02Z"
        }"#,
    );

    service
        .apply(&member_joined)
        .expect("member projection should succeed");
    service.register_client_route("100001", "default", "1016", "d_pad");
    service
        .apply(&rtc_signal_message)
        .expect("rtc signal message projection should succeed");

    let feed = service
        .client_route_sync_feed_window_for_principal_kind(
            "100001", "default", "1016",
            "user",
            "d_pad",
            Some(0),
            100,
        )
        .items;
    assert_eq!(feed.len(), 1);
    assert_eq!(feed[0].origin_event_type, "message.posted");
    assert_eq!(feed[0].message_id.as_deref(), Some("msg_rtc_signal_1"));
    assert_eq!(feed[0].payload_schema.as_deref(), Some("message.posted.v1"));
    let payload: serde_json::Value = serde_json::from_str(
        feed[0]
            .payload
            .as_deref()
            .expect("rtc signal message payload should be present in client route sync feed"),
    )
    .expect("rtc signal message client route sync payload should be valid json");
    assert_eq!(payload["rtcSessionId"], "rtc_sync_1");
    assert_eq!(payload["body"]["parts"][0]["kind"], "signal");
    assert_eq!(payload["body"]["parts"][0]["signalType"], "rtc.accept");
    let signal_payload: serde_json::Value =
        serde_json::from_str(payload["body"]["parts"][0]["payload"].as_str().unwrap())
            .expect("rtc signal part payload should be valid json");
    assert_eq!(signal_payload["rtcSessionId"], "rtc_sync_1");
    assert_eq!(signal_payload["rtcMode"], "video");
}

#[test]
fn test_media_message_client_route_sync_feed_preserves_message_payload_for_state_backfill() {
    let service = TimelineProjectionService::default();

    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_media_sync_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_media_sync",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_media_sync",
            "memberId":"cm_media_sync_alice",
            "principalId":"1016",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-05T10:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let media_message = im_domain_events::CommitEnvelope::minimal(
        "evt_media_message",
        "100001",
        "message.posted",
        "conversation",
        "c_media_sync",
        2,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_media_sync",
            "messageId":"msg_media_1",
            "messageSeq":1,
            "sender":{"id":"1016","kind":"user","memberId":"cm_media_sync_alice","deviceId":"d_phone","sessionId":"s_demo","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_media_1",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{
                "summary":"Image",
                "parts":[{
                    "kind":"media",
                    "resource":{
                        "id":"asset_image_1",
                        "kind":"image",
                        "source":"external_url",
                        "url":null,
                        "publicUrl":"https://cdn.example.test/offline-image.png",
                        "uri":null,
                        "objectBlobId":null,
                        "fileName":"offline-image.png",
                        "mimeType":"image/png",
                        "sizeBytes":"4096",
                        "checksum":null,
                        "width":640,
                        "height":480,
                        "durationSeconds":null,
                        "altText":null,
                        "title":null,
                        "poster":null,
                        "thumbnails":null,
                        "variants":null,
                        "access":null,
                        "ai":null,
                        "metadata":null
                    },
                    "drive":{
                        "driveUri":"drive://spaces/im-demo/nodes/node-image-1",
                        "spaceId":"im-demo",
                        "nodeId":"node-image-1",
                        "nodeVersion":"v1"
                    },
                    "mediaRole":"attachment"
                }],
                "renderHints":{"sdkworkChatPcType":"image"},
                "replyTo":{
                    "messageId":"msg_root",
                    "senderDisplayName":"Alice",
                    "contentPreview":"root message"
                }
            },
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:02Z",
            "committedAt":"2026-04-05T10:00:02Z"
        }"#,
    );

    service
        .apply(&member_joined)
        .expect("member projection should succeed");
    service.register_client_route("100001", "default", "1016", "d_pad");
    service
        .apply(&media_message)
        .expect("media message projection should succeed");

    let feed = service
        .client_route_sync_feed_window_for_principal_kind(
            "100001", "default", "1016",
            "user",
            "d_pad",
            Some(0),
            100,
        )
        .items;
    assert_eq!(feed.len(), 1);
    assert_eq!(feed[0].origin_event_type, "message.posted");
    assert_eq!(feed[0].message_id.as_deref(), Some("msg_media_1"));
    assert_eq!(feed[0].payload_schema.as_deref(), Some("message.posted.v1"));
    let payload: serde_json::Value = serde_json::from_str(
        feed[0]
            .payload
            .as_deref()
            .expect("media message payload should be present in client route sync feed"),
    )
    .expect("media message client route sync payload should be valid json");
    assert_eq!(payload["body"]["parts"][0]["kind"], "media");
    assert_eq!(payload["body"]["parts"][0]["resource"]["kind"], "image");
    assert_eq!(
        payload["body"]["parts"][0]["resource"]["publicUrl"],
        "https://cdn.example.test/offline-image.png"
    );
    assert_eq!(
        payload["body"]["parts"][0]["resource"]["fileName"],
        "offline-image.png"
    );
    assert_eq!(payload["body"]["parts"][0]["resource"]["sizeBytes"], "4096");
    assert_eq!(payload["body"]["renderHints"]["sdkworkChatPcType"], "image");
    assert_eq!(payload["body"]["replyTo"]["messageId"], "msg_root");
}

#[test]
fn test_read_cursor_client_route_sync_fanout_uses_cursor_principal_kind() {
    let service = TimelineProjectionService::default();

    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_cursor_actor_kind_fanout_member",
                "100001",
                "conversation.member_joined",
                "conversation",
                "c_cursor_actor_kind_fanout",
                1,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"100001",
                    "conversationId":"c_cursor_actor_kind_fanout",
                    "memberId":"cm_c_cursor_actor_kind_fanout_agent_bot",
                    "principalId":"bot",
                    "principalKind":"agent",
                    "role":"owner",
                    "state":"joined",
                    "invitedBy":null,
                    "joinedAt":"2026-04-10T12:00:00Z",
                    "removedAt":null,
                    "attributes":{}
                }"#,
            ),
        )
        .expect("agent member should project");
    service.register_client_route_for_principal_kind("100001", "default", "bot", "agent", "d_agent");
    service.register_client_route_for_principal_kind("100001", "default", "bot", "system", "d_system");

    let cursor_updated = im_domain_events::CommitEnvelope::minimal(
        "evt_cursor_actor_kind_fanout_update",
        "100001",
        "conversation.read_cursor_updated",
        "conversation",
        "c_cursor_actor_kind_fanout",
        2,
    )
    .with_payload(
        "conversation.read_cursor.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_cursor_actor_kind_fanout",
            "memberId":"cm_c_cursor_actor_kind_fanout_agent_bot",
            "principalId":"bot",
            "principalKind":"agent",
            "readSeq":7,
            "lastReadMessageId":"msg_c_cursor_actor_kind_fanout_7",
            "updatedAt":"2026-04-10T12:00:10Z"
        }"#,
    );
    service
        .apply(&cursor_updated)
        .expect("agent cursor should project");

    let agent_feed = service
        .client_route_sync_feed_window_for_principal_kind(
            "100001", "default", "bot",
            "agent",
            "d_agent",
            Some(0),
            100,
        )
        .items;
    assert_eq!(agent_feed.len(), 1);
    assert_eq!(agent_feed[0].actor_id.as_deref(), Some("bot"));
    assert_eq!(agent_feed[0].actor_kind.as_deref(), Some("agent"));
    assert_eq!(agent_feed[0].read_seq, Some(7));

    let system_feed = service
        .client_route_sync_feed_window_for_principal_kind(
            "100001", "default", "bot",
            "system",
            "d_system",
            Some(0),
            100,
        )
        .items;
    assert!(
        system_feed.is_empty(),
        "read cursor fanout must not route payload agent cursors to same-id system devices"
    );
}

#[test]
fn test_client_route_sync_feed_window_is_bounded_and_reports_trimmed_boundary() {
    let service = TimelineProjectionService::default();
    let mut member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_bounded_sync_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_bounded_sync",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_bounded_sync",
            "memberId":"cm_bounded_sync_owner",
            "principalId":"1",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-16T10:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    member_joined.actor.actor_id = "1".into();
    member_joined.actor.actor_kind = "user".into();
    service
        .apply(&member_joined)
        .expect("member projection should succeed");
    service.register_client_route_for_principal_kind("100001", "default", "1", "user", "d_owner");
    service.register_client_route_for_principal_kind("100001", "default", "1", "user", "d_pad");

    let auth = app_context(
        "100001",
        "1",
        "user",
        Some("s_bounded_sync"),
        Some("d_pad"),
    );

    for message_seq in 1..=1002 {
        service
            .apply(
                &im_domain_events::CommitEnvelope::minimal(
                    &format!("evt_bounded_sync_message_{message_seq}"),
                    "100001",
                    "message.posted",
                    "conversation",
                    "c_bounded_sync",
                    message_seq + 1,
                )
                .with_payload(
                    "message.posted.v1",
                    &format!(
                        r#"{{
                            "tenantId":"100001",
                            "conversationId":"c_bounded_sync",
                            "messageId":"msg_bounded_sync_{message_seq}",
                            "messageSeq":{message_seq},
                            "sender":{{"id":"1","kind":"user","memberId":"cm_bounded_sync_owner","deviceId":"d_owner","sessionId":"s_bounded_sync","metadata":{{}}}},
                            "messageType":"standard",
                            "deliveryMode":"discrete",
                            "clientMsgId":"client_bounded_sync_{message_seq}",
                            "streamSessionId":null,
                            "rtcSessionId":null,
                            "body":{{"summary":"bounded {message_seq}","parts":[{{"kind":"text","text":"bounded {message_seq}"}}],"renderHints":{{}}}},
                            "attributes":{{}},
                            "metadata":{{}},
                            "occurredAt":"2026-04-16T10:00:00Z",
                            "committedAt":"2026-04-16T10:00:00Z"
                        }}"#
                    ),
                ),
            )
            .expect("message projection should succeed");
    }

    let latest_seq = service
        .latest_client_route_sync_seq_from_auth_context(&auth, "d_pad")
        .expect("latest sync seq should be accessible");
    assert_eq!(latest_seq, 1002);

    let window = service
        .client_route_sync_feed_window_from_auth_context(&auth, "d_pad", Some(0), Some(1000))
        .expect("bounded sync feed should be accessible");
    assert_eq!(window.items.len(), 1000);
    assert_eq!(window.trimmed_through_seq, 2);
    assert_eq!(window.items[0].sync_seq, 3);
    assert_eq!(window.next_after_seq, Some(1002));
    assert!(!window.has_more);
}

#[test]
fn test_member_governance_events_project_typed_sync_feed_deltas() {
    let service = TimelineProjectionService::default();

    let mut owner_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_owner_joined",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_member_sync",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_member_sync",
            "memberId":"cm_c_member_sync_user_u_1",
            "principalId":"1",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-06T12:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    owner_joined.actor.actor_id = "1".into();
    owner_joined.actor.actor_kind = "user".into();
    service
        .apply(&owner_joined)
        .expect("owner joined projection should succeed");

    service.register_client_route("100001", "default", "1", "d_owner");
    service.register_client_route("100001", "default", "1013", "d_other");
    service.register_client_route("100001", "default", "1019", "d_leave");

    let mut other_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_other_joined",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_member_sync",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_member_sync",
            "memberId":"cm_c_member_sync_user_u_other",
            "principalId":"1013",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"1",
            "joinedAt":"2026-04-06T12:01:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    other_joined.actor.actor_id = "1".into();
    other_joined.actor.actor_kind = "user".into();
    let mut other_role_changed = im_domain_events::CommitEnvelope::minimal(
        "evt_other_role_changed",
        "100001",
        "conversation.member_role_changed",
        "conversation",
        "c_member_sync",
        3,
    )
    .with_payload(
        "conversation.member_role_changed.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_member_sync",
            "previousMember":{
                "tenantId":"100001",
                "conversationId":"c_member_sync",
                "memberId":"cm_c_member_sync_user_u_other",
                "principalId":"1013",
                "principalKind":"user",
                "role":"member",
                "state":"joined",
                "invitedBy":"1",
                "joinedAt":"2026-04-06T12:01:00Z",
                "removedAt":null,
                "attributes":{}
            },
            "updatedMember":{
                "tenantId":"100001",
                "conversationId":"c_member_sync",
                "memberId":"cm_c_member_sync_user_u_other",
                "principalId":"1013",
                "principalKind":"user",
                "role":"admin",
                "state":"joined",
                "invitedBy":"1",
                "joinedAt":"2026-04-06T12:01:00Z",
                "removedAt":null,
                "attributes":{}
            },
            "changedAt":"2026-04-06T12:02:00Z"
        }"#,
    );
    other_role_changed.actor.actor_id = "1".into();
    other_role_changed.actor.actor_kind = "user".into();
    let mut other_removed = im_domain_events::CommitEnvelope::minimal(
        "evt_other_removed",
        "100001",
        "conversation.member_removed",
        "conversation",
        "c_member_sync",
        4,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_member_sync",
            "memberId":"cm_c_member_sync_user_u_other",
            "principalId":"1013",
            "principalKind":"user",
            "role":"admin",
            "state":"removed",
            "invitedBy":"1",
            "joinedAt":"2026-04-06T12:01:00Z",
            "removedAt":"2026-04-06T12:03:00Z",
            "attributes":{}
        }"#,
    );
    other_removed.actor.actor_id = "1".into();
    other_removed.actor.actor_kind = "user".into();
    let mut leave_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_leave_joined",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_member_sync",
        5,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_member_sync",
            "memberId":"cm_c_member_sync_user_u_leave",
            "principalId":"1019",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"1",
            "joinedAt":"2026-04-06T12:04:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    leave_joined.actor.actor_id = "1".into();
    leave_joined.actor.actor_kind = "user".into();
    let mut leave_left = im_domain_events::CommitEnvelope::minimal(
        "evt_leave_left",
        "100001",
        "conversation.member_left",
        "conversation",
        "c_member_sync",
        6,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_member_sync",
            "memberId":"cm_c_member_sync_user_u_leave",
            "principalId":"1019",
            "principalKind":"user",
            "role":"member",
            "state":"left",
            "invitedBy":"1",
            "joinedAt":"2026-04-06T12:04:00Z",
            "removedAt":"2026-04-06T12:05:00Z",
            "attributes":{}
        }"#,
    );
    leave_left.actor.actor_id = "1019".into();
    leave_left.actor.actor_kind = "user".into();

    for event in [
        other_joined,
        other_role_changed,
        other_removed,
        leave_joined,
        leave_left,
    ] {
        service
            .apply(&event)
            .expect("member governance projection should succeed");
    }

    let owner_feed = service
        .client_route_sync_feed_window_for_principal_kind(
            "100001", "default", "1",
            "user",
            "d_owner",
            Some(0),
            100,
        )
        .items;
    assert_eq!(owner_feed.len(), 5);
    assert_eq!(
        owner_feed[0].origin_event_type,
        "conversation.member_joined"
    );
    assert_eq!(
        owner_feed[1].origin_event_type,
        "conversation.member_role_changed"
    );
    assert_eq!(
        owner_feed[2].origin_event_type,
        "conversation.member_removed"
    );
    assert_eq!(
        owner_feed[3].origin_event_type,
        "conversation.member_joined"
    );
    assert_eq!(owner_feed[4].origin_event_type, "conversation.member_left");

    let joined_value =
        serde_json::to_value(&owner_feed[0]).expect("joined sync entry should serialize");
    assert_eq!(
        joined_value["payloadSchema"],
        serde_json::Value::String("conversation.member.v1".into())
    );
    let joined_payload: serde_json::Value = serde_json::from_str(
        joined_value["payload"]
            .as_str()
            .expect("joined sync payload should be present"),
    )
    .expect("joined sync payload should be valid json");
    assert_eq!(joined_payload["principalId"], "1013");
    assert_eq!(joined_payload["state"], "joined");
    assert_eq!(joined_value["actorId"], "1");
    assert_eq!(joined_value["actorKind"], "user");

    let role_changed_value =
        serde_json::to_value(&owner_feed[1]).expect("role change sync entry should serialize");
    assert_eq!(
        role_changed_value["payloadSchema"],
        serde_json::Value::String("conversation.member_role_changed.v1".into())
    );
    let role_changed_payload: serde_json::Value = serde_json::from_str(
        role_changed_value["payload"]
            .as_str()
            .expect("role change payload should be present"),
    )
    .expect("role change payload should be valid json");
    assert_eq!(role_changed_payload["previousMember"]["role"], "member");
    assert_eq!(role_changed_payload["updatedMember"]["role"], "admin");
    assert_eq!(role_changed_value["actorId"], "1");
    assert_eq!(role_changed_value["actorKind"], "user");

    let removed_value =
        serde_json::to_value(&owner_feed[2]).expect("removed sync entry should serialize");
    assert_eq!(
        removed_value["payloadSchema"],
        serde_json::Value::String("conversation.member.v1".into())
    );
    let removed_payload: serde_json::Value = serde_json::from_str(
        removed_value["payload"]
            .as_str()
            .expect("removed payload should be present"),
    )
    .expect("removed payload should be valid json");
    assert_eq!(removed_payload["principalId"], "1013");
    assert_eq!(removed_payload["state"], "removed");
    assert_eq!(removed_value["actorId"], "1");
    assert_eq!(removed_value["actorKind"], "user");

    let removed_principal_feed = service
        .client_route_sync_feed_window_for_principal_kind(
            "100001", "default", "1013",
            "user",
            "d_other",
            Some(0),
            100,
        )
        .items;
    assert_eq!(removed_principal_feed.len(), 3);
    assert_eq!(
        removed_principal_feed[2].origin_event_type,
        "conversation.member_removed"
    );
    let removed_principal_value = serde_json::to_value(&removed_principal_feed[2])
        .expect("removed principal sync entry should serialize");
    let removed_principal_payload: serde_json::Value = serde_json::from_str(
        removed_principal_value["payload"]
            .as_str()
            .expect("removed principal payload should be present"),
    )
    .expect("removed principal payload should be valid json");
    assert_eq!(removed_principal_payload["principalId"], "1013");
    assert_eq!(removed_principal_payload["state"], "removed");

    let leave_feed = service
        .client_route_sync_feed_window_for_principal_kind(
            "100001", "default", "1019",
            "user",
            "d_leave",
            Some(0),
            100,
        )
        .items;
    assert_eq!(leave_feed.len(), 2);
    assert_eq!(leave_feed[1].origin_event_type, "conversation.member_left");
    let leave_value =
        serde_json::to_value(&leave_feed[1]).expect("leave sync entry should serialize");
    assert_eq!(
        leave_value["payloadSchema"],
        serde_json::Value::String("conversation.member.v1".into())
    );
    let leave_payload: serde_json::Value = serde_json::from_str(
        leave_value["payload"]
            .as_str()
            .expect("leave payload should be present"),
    )
    .expect("leave payload should be valid json");
    assert_eq!(leave_payload["principalId"], "1019");
    assert_eq!(leave_payload["state"], "left");
    assert_eq!(leave_value["actorId"], "1019");
    assert_eq!(leave_value["actorKind"], "user");
}

#[test]
fn test_registered_client_routes_and_latest_sync_seq_are_queryable() {
    let service = TimelineProjectionService::default();

    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_resume",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_resume",
            "memberId":"cm_demo",
            "principalId":"1",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-05T10:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let message_posted = im_domain_events::CommitEnvelope::minimal(
        "evt_message",
        "100001",
        "message.posted",
        "conversation",
        "c_resume",
        2,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_resume",
            "messageId":"msg_c_resume_1",
            "messageSeq":1,
            "sender":{"id":"1","kind":"user","memberId":"cm_demo","deviceId":"d_phone","sessionId":"s_demo","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_resume_1",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"resume hello","parts":[{"kind":"text","text":"resume hello"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:02Z",
            "committedAt":"2026-04-05T10:00:02Z"
        }"#,
    );

    service
        .apply(&member_joined)
        .expect("member projection should succeed");
    service.register_client_route("100001", "default", "1", "d_phone");
    service.register_client_route("100001", "default", "1", "d_pad");
    service
        .apply(&message_posted)
        .expect("message projection should succeed");

    let devices = service.registered_client_routes("100001", "default", "1");
    assert_eq!(devices.len(), 2);
    assert!(devices.iter().any(|item| item.device_id == "d_phone"));
    assert!(devices.iter().any(|item| item.device_id == "d_pad"));

    assert_eq!(
        service.latest_client_route_sync_seq("100001", "default", "1", "d_phone"),
        1
    );
    assert_eq!(
        service.latest_client_route_sync_seq("100001", "default", "1", "d_pad"),
        1
    );
    assert_eq!(
        service.latest_client_route_sync_seq("100001", "default", "1", "d_missing"),
        0
    );
}

#[test]
fn test_friendship_events_project_to_contact_client_route_sync_feeds_for_both_users() {
    let service = TimelineProjectionService::default();

    service.register_client_route("100001", "default", "1016", "d_alice");
    service.register_client_route("100001", "default", "1020", "d_bob");

    let friendship_activated = im_domain_events::CommitEnvelope::minimal(
        "evt_friendship_contact_sync_activated",
        "100001",
        "friendship.activated",
        "friendship",
        "fs_contact_sync",
        1,
    )
    .with_payload(
        "social.friendship.activated.v1",
        r#"{
            "friendshipId":"fs_contact_sync",
            "userLowId":"1016",
            "userHighId":"1020",
            "initiatorUserId":"1016",
            "directChatId":"dc_contact_sync",
            "establishedAt":"2026-04-05T10:00:00Z"
        }"#,
    );
    let friendship_removed = im_domain_events::CommitEnvelope::minimal(
        "evt_friendship_contact_sync_removed",
        "100001",
        "friendship.removed",
        "friendship",
        "fs_contact_sync",
        2,
    )
    .with_payload(
        "social.friendship.removed.v1",
        r#"{
            "friendshipId":"fs_contact_sync",
            "userLowId":"1016",
            "userHighId":"1020",
            "removedByUserId":"1020",
            "removedAt":"2026-04-05T10:00:02Z"
        }"#,
    );

    service
        .apply(&friendship_activated)
        .expect("friendship activation projection should succeed");
    service
        .apply(&friendship_removed)
        .expect("friendship removal projection should succeed");

    for (user_id, device_id, expected_peer_id) in [
        ("1016", "d_alice", "1020"),
        ("1020", "d_bob", "1016"),
    ] {
        let feed = service
            .client_route_sync_feed_window_for_principal_kind(
                "100001",
                "default",
                user_id,
                "user",
                device_id,
                Some(0),
                100,
            )
            .items;
        assert_eq!(
            feed.len(),
            2,
            "friendship sync feed for {user_id}/{device_id} must include activation and removal"
        );
        assert_eq!(feed[0].origin_event_type, "friendship.activated");
        assert_eq!(
            feed[0].payload_schema.as_deref(),
            Some("social.friendship.activated.v1")
        );
        assert_eq!(feed[0].actor_id.as_deref(), Some("1016"));
        assert_eq!(feed[0].summary.as_deref(), Some(expected_peer_id));
        let activated_payload: serde_json::Value = serde_json::from_str(
            feed[0]
                .payload
                .as_deref()
                .expect("friendship activation payload should be present"),
        )
        .expect("friendship activation payload should be valid json");
        assert_eq!(activated_payload["friendshipId"], "fs_contact_sync");
        assert_eq!(activated_payload["userLowId"], "1016");
        assert_eq!(activated_payload["userHighId"], "1020");

        assert_eq!(feed[1].origin_event_type, "friendship.removed");
        assert_eq!(
            feed[1].payload_schema.as_deref(),
            Some("social.friendship.removed.v1")
        );
        assert_eq!(feed[1].actor_id.as_deref(), Some("1020"));
        assert_eq!(feed[1].summary.as_deref(), Some(expected_peer_id));
        let removed_payload: serde_json::Value = serde_json::from_str(
            feed[1]
                .payload
                .as_deref()
                .expect("friendship removal payload should be present"),
        )
        .expect("friendship removal payload should be valid json");
        assert_eq!(removed_payload["friendshipId"], "fs_contact_sync");
        assert_eq!(removed_payload["removedByUserId"], "1020");
    }
}

#[test]
fn test_realtime_fanout_targets_for_recipients_return_registered_principal_device_pairs() {
    let service = TimelineProjectionService::default();

    service.register_client_route("100001", "default", "1021", "d_phone");
    service.register_client_route("100001", "default", "1022", "d_watch");
    service.register_client_route("100001", "default", "1022", "d_pad");

    let auth = app_context("100001", "1022", "user", Some("s_a"), Some("d_pad"));

    let targets = service.realtime_fanout_targets_for_recipients_from_auth_context(
        &auth,
        vec![
            NotificationRecipientView {
                principal_id: "1021".into(),
                principal_kind: "user".into(),
            },
            NotificationRecipientView {
                principal_id: "1023".into(),
                principal_kind: "user".into(),
            },
            NotificationRecipientView {
                principal_id: "1022".into(),
                principal_kind: "user".into(),
            },
        ],
    );

    assert_eq!(
        targets,
        vec![
            RealtimeFanoutTarget {
                principal_id: "1022".into(),
                principal_kind: "user".into(),
                device_id: "d_pad".into(),
            },
            RealtimeFanoutTarget {
                principal_id: "1022".into(),
                principal_kind: "user".into(),
                device_id: "d_watch".into(),
            },
            RealtimeFanoutTarget {
                principal_id: "1021".into(),
                principal_kind: "user".into(),
                device_id: "d_phone".into(),
            },
        ]
    );
}

#[test]
fn test_client_route_sync_fanout_targets_for_conversation_include_active_members_and_fallback_devices()
 {
    let service = TimelineProjectionService::default();

    let owner_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_sync_targets_owner",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_sync_targets",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_sync_targets",
            "memberId":"cm_sync_targets_owner",
            "principalId":"1",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-07T09:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_sync_targets_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_sync_targets",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_sync_targets",
            "memberId":"cm_sync_targets_member",
            "principalId":"1014",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"1",
            "joinedAt":"2026-04-07T09:01:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );

    service
        .apply(&owner_joined)
        .expect("owner projection should succeed");
    service
        .apply(&member_joined)
        .expect("member projection should succeed");

    service.register_client_route("100001", "default", "1", "d_phone");
    service.register_client_route("100001", "default", "1", "d_pad");
    service.register_client_route("100001", "default", "1014", "d_watch");
    service.register_client_route("100001", "default", "1024", "d_removed");

    let targets = service.client_route_sync_fanout_targets_for_conversation(
        "100001",
        "default",
        "c_sync_targets",
        vec![NotificationRecipientView {
            principal_id: "1024".into(),
            principal_kind: "user".into(),
        }],
    );

    assert_eq!(
        targets,
        vec![
            RealtimeFanoutTarget {
                principal_id: "1014".into(),
                principal_kind: "user".into(),
                device_id: "d_watch".into(),
            },
            RealtimeFanoutTarget {
                principal_id: "1".into(),
                principal_kind: "user".into(),
                device_id: "d_pad".into(),
            },
            RealtimeFanoutTarget {
                principal_id: "1".into(),
                principal_kind: "user".into(),
                device_id: "d_phone".into(),
            },
            RealtimeFanoutTarget {
                principal_id: "1024".into(),
                principal_kind: "user".into(),
                device_id: "d_removed".into(),
            },
        ]
    );
}

#[test]
fn test_active_conversation_principal_recipients_from_auth_context_returns_current_active_members()
{
    let service = TimelineProjectionService::default();

    let owner_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_active_principals_owner",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_active_principals",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_active_principals",
            "memberId":"cm_active_principals_owner",
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
    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_active_principals_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_active_principals",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_active_principals",
            "memberId":"cm_active_principals_member",
            "principalId":"1014",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"1",
            "joinedAt":"2026-04-07T10:01:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let member_removed = im_domain_events::CommitEnvelope::minimal(
        "evt_active_principals_removed",
        "100001",
        "conversation.member_removed",
        "conversation",
        "c_active_principals",
        3,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_active_principals",
            "memberId":"cm_active_principals_member",
            "principalId":"1014",
            "principalKind":"user",
            "role":"member",
            "state":"removed",
            "invitedBy":"1",
            "joinedAt":"2026-04-07T10:01:00Z",
            "removedAt":"2026-04-07T10:02:00Z",
            "attributes":{}
        }"#,
    );

    for event in [owner_joined, member_joined, member_removed] {
        service
            .apply(&event)
            .expect("member projection should succeed");
    }

    let auth = app_context(
        "100001",
        "1",
        "user",
        Some("s_owner"),
        Some("d_owner"),
    );

    assert_eq!(
        service
            .active_conversation_principal_recipients_from_auth_context(
                &auth,
                "c_active_principals"
            )
            .expect("active member should read active principal recipients"),
        vec![NotificationRecipientView {
            principal_id: "1".into(),
            principal_kind: "user".into(),
        }]
    );
}

#[test]
fn test_message_posted_notification_recipients_from_auth_context_include_shared_linked_members() {
    let service = TimelineProjectionService::default();

    let owner_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_notification_targets_owner",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_notification_targets",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_notification_targets",
            "memberId":"cm_notification_targets_owner",
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
    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_notification_targets_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_notification_targets",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_notification_targets",
            "memberId":"cm_notification_targets_member",
            "principalId":"1014",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"1",
            "joinedAt":"2026-04-07T10:01:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let shared_linked = im_domain_events::CommitEnvelope::minimal(
        "evt_notification_targets_shared_linked",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_notification_targets",
        3,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_notification_targets",
            "memberId":"cm_notification_targets_shared",
            "principalId":"1025",
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
        service
            .apply(&event)
            .expect("member projection should accept notification target events");
    }

    let policy_applied = im_domain_events::CommitEnvelope::minimal(
        "evt_notification_targets_policy",
        "100001",
        "conversation.policy_applied",
        "conversation",
        "c_notification_targets",
        4,
    )
    .with_payload(
        "conversation.policy_applied.v1",
        r#"{
            "conversationId":"c_notification_targets",
            "policyVersion":"shared.policy.v1",
            "historyVisibility":"shared",
            "retentionPolicyRef":"tenant.standard"
        }"#,
    );
    service
        .apply(&policy_applied)
        .expect("shared history policy should project");

    let auth = app_context(
        "100001",
        "1",
        "user",
        Some("s_owner"),
        Some("d_owner"),
    );

    assert_eq!(
        service
            .active_conversation_principal_recipients_from_auth_context(
                &auth,
                "c_notification_targets"
            )
            .expect("active member should still resolve active principal recipients"),
        vec![
            NotificationRecipientView {
                principal_id: "1014".into(),
                principal_kind: "user".into(),
            },
            NotificationRecipientView {
                principal_id: "1".into(),
                principal_kind: "user".into(),
            }
        ]
    );
    assert_eq!(
        service
            .message_posted_notification_recipients_from_auth_context(
                &auth,
                "c_notification_targets"
            )
            .expect("active member should resolve shared notification principal ids"),
        vec![
            projection_service::NotificationRecipientView {
                principal_id: "1014".into(),
                principal_kind: "user".into(),
            },
            projection_service::NotificationRecipientView {
                principal_id: "1".into(),
                principal_kind: "user".into(),
            },
            projection_service::NotificationRecipientView {
                principal_id: "1025".into(),
                principal_kind: "external_user".into(),
            }
        ]
    );
}

#[test]
fn test_invited_history_visibility_allows_invited_member_timeline_reads_in_projection() {
    let service = TimelineProjectionService::default();

    let owner_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_invited_history_owner",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_invited_history",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_invited_history",
            "memberId":"cm_invited_history_owner",
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
    let invited_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_invited_history_invited",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_invited_history",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_invited_history",
            "memberId":"cm_invited_history_guest",
            "principalId":"1026",
            "principalKind":"user",
            "role":"guest",
            "state":"invited",
            "invitedBy":"1",
            "joinedAt":"2026-04-07T10:01:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let policy_applied = im_domain_events::CommitEnvelope::minimal(
        "evt_invited_history_policy",
        "100001",
        "conversation.policy_applied",
        "conversation",
        "c_invited_history",
        3,
    )
    .with_payload(
        "conversation.policy_applied.v1",
        r#"{
            "conversationId":"c_invited_history",
            "policyVersion":"invited.policy.v1",
            "historyVisibility":"invited",
            "retentionPolicyRef":"tenant.standard"
        }"#,
    );

    for event in [owner_joined, invited_joined, policy_applied] {
        service
            .apply(&event)
            .expect("invited history projection setup should succeed");
    }

    let invited_auth = app_context(
        "100001",
        "1026",
        "user",
        Some("s_invited"),
        Some("d_invited"),
    );

    service
        .ensure_history_reader_from_auth_context(&invited_auth, "c_invited_history")
        .expect("invited member should read history before join when policy is invited");

    let outsider_auth = app_context(
        "100001",
        "1027",
        "user",
        Some("s_outsider"),
        Some("d_outsider"),
    );
    assert_eq!(
        service
            .ensure_history_reader_from_auth_context(&outsider_auth, "c_invited_history")
            .expect_err("outsider should not read invited history")
            .code(),
        "conversation_permission_denied"
    );
}

#[test]
fn test_timeline_window_filters_expired_retention_entries() {
    let service = TimelineProjectionService::default();
    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_retention_filter_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_retention_filter",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_retention_filter",
            "memberId":"cm_retention_filter",
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
    let expired_message = im_domain_events::CommitEnvelope::minimal(
        "evt_retention_filter_expired",
        "100001",
        "message.posted",
        "conversation",
        "c_retention_filter",
        2,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_retention_filter",
            "messageId":"m_expired",
            "messageSeq":1,
            "sender":{"id":"1","kind":"user","memberId":"cm_retention_filter","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "body":{"summary":"expired","parts":[],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2020-01-01T00:00:00.000Z"
        }"#,
    );
    let fresh_message = im_domain_events::CommitEnvelope::minimal(
        "evt_retention_filter_fresh",
        "100001",
        "message.posted",
        "conversation",
        "c_retention_filter",
        3,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_retention_filter",
            "messageId":"m_fresh",
            "messageSeq":2,
            "sender":{"id":"1","kind":"user","memberId":"cm_retention_filter","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "body":{"summary":"fresh","parts":[],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-06-01T00:00:00.000Z"
        }"#,
    );

    for event in [member_joined, expired_message, fresh_message] {
        service
            .apply(&event)
            .expect("retention filter projection setup should succeed");
    }

    let window = service.timeline_window("100001", "default", "c_retention_filter", None, 10);
    assert_eq!(window.items.len(), 1);
    assert_eq!(window.items[0].message_id, "m_fresh");
}

#[test]
fn test_legal_hold_policy_clears_timeline_retention_until() {
    let service = TimelineProjectionService::default();
    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_legal_hold_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_legal_hold",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_legal_hold",
            "memberId":"cm_legal_hold",
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
    let mut message = im_domain_events::CommitEnvelope::minimal(
        "evt_legal_hold_message",
        "100001",
        "message.posted",
        "conversation",
        "c_legal_hold",
        2,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_legal_hold",
            "messageId":"m_hold",
            "messageSeq":1,
            "sender":{"id":"1","kind":"user","memberId":"cm_legal_hold","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "body":{"summary":"hold me","parts":[],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2020-01-01T00:00:00.000Z"
        }"#,
    );
    message.retention_class = "standard".into();
    let policy_applied = im_domain_events::CommitEnvelope::minimal(
        "evt_legal_hold_policy",
        "100001",
        "conversation.policy_applied",
        "conversation",
        "c_legal_hold",
        3,
    )
    .with_payload(
        "conversation.policy_applied.v1",
        r#"{
            "conversationId":"c_legal_hold",
            "policyVersion":"legal.policy.v1",
            "historyVisibility":"joined",
            "retentionPolicyRef":"tenant.legal_hold"
        }"#,
    );

    for event in [member_joined, message, policy_applied] {
        service
            .apply(&event)
            .expect("legal hold projection setup should succeed");
    }

    let window = service.timeline_window("100001", "default", "c_legal_hold", None, 10);
    assert_eq!(window.items.len(), 1);
    assert_eq!(window.items[0].message_id, "m_hold");
    assert!(window.items[0].retention_until.is_none());
}

#[test]
fn test_member_directory_and_notification_recipients_preserve_same_actor_id_across_principal_kinds()
{
    let service = TimelineProjectionService::default();

    let owner_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_typed_member_directory_owner",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_typed_member_directory",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_typed_member_directory",
            "memberId":"cm_typed_member_directory_owner",
            "principalId":"1018",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-07T11:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let agent_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_typed_member_directory_agent",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_typed_member_directory",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_typed_member_directory",
            "memberId":"cm_typed_member_directory_agent",
            "principalId":"1018",
            "principalKind":"agent",
            "role":"member",
            "state":"joined",
            "invitedBy":"1018",
            "joinedAt":"2026-04-07T11:01:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );

    for event in [owner_joined, agent_joined] {
        service
            .apply(&event)
            .expect("typed member projection should succeed");
    }

    let auth = app_context(
        "100001",
        "1018",
        "user",
        Some("s_dual_user"),
        Some("d_dual_user"),
    );

    let directory = service
        .member_directory_from_auth_context(&auth, "c_typed_member_directory")
        .expect("typed user member should still access directory");
    assert_eq!(directory.len(), 2);
    assert!(directory.iter().any(|member| {
        member.principal_id == "1018"
            && member.principal_kind == "user"
            && member.role == MembershipRole::Owner
    }));
    assert!(directory.iter().any(|member| {
        member.principal_id == "1018"
            && member.principal_kind == "agent"
            && member.role == MembershipRole::Member
    }));

    assert_eq!(
        service
            .message_posted_notification_recipients_from_auth_context(
                &auth,
                "c_typed_member_directory",
            )
            .expect("typed user member should still resolve typed recipients"),
        vec![
            NotificationRecipientView {
                principal_id: "1018".into(),
                principal_kind: "agent".into(),
            },
            NotificationRecipientView {
                principal_id: "1018".into(),
                principal_kind: "user".into(),
            }
        ]
    );
}

#[test]
fn test_typed_realtime_recipients_exclude_non_member_devices_sharing_same_actor_id() {
    let service = TimelineProjectionService::default();

    let owner_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_typed_realtime_targets_owner",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_typed_realtime_targets",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_typed_realtime_targets",
            "memberId":"cm_typed_realtime_targets_owner",
            "principalId":"1",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-07T12:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_typed_realtime_targets_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_typed_realtime_targets",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_typed_realtime_targets",
            "memberId":"cm_typed_realtime_targets_member",
            "principalId":"1018",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"1",
            "joinedAt":"2026-04-07T12:01:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );

    for event in [owner_joined, member_joined] {
        service
            .apply(&event)
            .expect("typed realtime target projection should succeed");
    }

    service.register_client_route_for_principal_kind("100001", "default", "1", "user", "d_owner");
    service.register_client_route_for_principal_kind("100001", "default", "1018", "user", "d_dual_user");
    service.register_client_route_for_principal_kind("100001", "default", "1018", "agent", "d_dual_agent");

    let auth = app_context(
        "100001",
        "1",
        "user",
        Some("s_owner"),
        Some("d_owner"),
    );

    let recipients = service
        .active_conversation_principal_recipients_from_auth_context(
            &auth,
            "c_typed_realtime_targets",
        )
        .expect("owner should resolve typed realtime recipients");
    assert_eq!(
        recipients,
        vec![
            NotificationRecipientView {
                principal_id: "1018".into(),
                principal_kind: "user".into(),
            },
            NotificationRecipientView {
                principal_id: "1".into(),
                principal_kind: "user".into(),
            }
        ]
    );

    assert_eq!(
        service.realtime_fanout_targets_for_recipients_from_auth_context(&auth, recipients),
        vec![
            RealtimeFanoutTarget {
                principal_id: "1018".into(),
                principal_kind: "user".into(),
                device_id: "d_dual_user".into(),
            },
            RealtimeFanoutTarget {
                principal_id: "1".into(),
                principal_kind: "user".into(),
                device_id: "d_owner".into(),
            }
        ]
    );
}

#[test]
fn test_client_route_sync_state_isolated_for_same_actor_and_device_across_principal_kinds() {
    let service = TimelineProjectionService::default();

    let owner_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_typed_device_scope_owner",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_typed_device_scope",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_typed_device_scope",
            "memberId":"cm_typed_device_scope_owner",
            "principalId":"1",
            "principalKind":"user",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-07T13:00:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );
    let user_member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_typed_device_scope_user_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_typed_device_scope",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_typed_device_scope",
            "memberId":"cm_typed_device_scope_user_member",
            "principalId":"1018",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"1",
            "joinedAt":"2026-04-07T13:01:00Z",
            "removedAt":null,
            "attributes":{}
        }"#,
    );

    for event in [owner_joined, user_member_joined] {
        service
            .apply(&event)
            .expect("typed device scope projection should succeed");
    }

    service.register_client_route_for_principal_kind("100001", "default", "1", "user", "d_owner");
    service.register_client_route_for_principal_kind("100001", "default", "1018", "user", "d_shared");
    service.register_client_route_for_principal_kind("100001", "default", "1018", "agent", "d_shared");

    let user_auth = app_context(
        "100001",
        "1018",
        "user",
        Some("s_typed_device_scope_user"),
        Some("d_shared"),
    );
    let agent_auth = app_context(
        "100001",
        "1018",
        "agent",
        Some("s_typed_device_scope_agent"),
        Some("d_shared"),
    );

    let user_client_routes = service.registered_client_routes_from_auth_context(&user_auth);
    assert_eq!(user_client_routes.len(), 1);
    assert_eq!(user_client_routes[0].device_id, "d_shared");
    assert_eq!(user_client_routes[0].principal_kind, "user");

    let agent_client_routes = service.registered_client_routes_from_auth_context(&agent_auth);
    assert_eq!(agent_client_routes.len(), 1);
    assert_eq!(agent_client_routes[0].device_id, "d_shared");
    assert_eq!(agent_client_routes[0].principal_kind, "agent");

    assert_eq!(
        service.client_route_sync_fanout_targets_for_conversation(
            "100001",
            "default",
            "c_typed_device_scope",
            vec![],
        ),
        vec![
            RealtimeFanoutTarget {
                principal_id: "1018".into(),
                principal_kind: "user".into(),
                device_id: "d_shared".into(),
            },
            RealtimeFanoutTarget {
                principal_id: "1".into(),
                principal_kind: "user".into(),
                device_id: "d_owner".into(),
            },
        ]
    );

    let message_posted = im_domain_events::CommitEnvelope::minimal(
        "evt_typed_device_scope_message",
        "100001",
        "message.posted",
        "conversation",
        "c_typed_device_scope",
        3,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_typed_device_scope",
            "messageId":"msg_typed_device_scope_1",
            "messageSeq":1,
            "sender":{"id":"1","kind":"user","memberId":"cm_typed_device_scope_owner","deviceId":"d_owner","sessionId":"s_typed_device_scope_owner","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_typed_device_scope_1",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"typed-device-scope","parts":[{"kind":"text","text":"typed-device-scope"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-07T13:02:00Z",
            "committedAt":"2026-04-07T13:02:00Z"
        }"#,
    );

    service
        .apply(&message_posted)
        .expect("typed device scope message projection should succeed");

    let user_feed = service
        .client_route_sync_feed_window_from_auth_context(&user_auth, "d_shared", Some(0), Some(100))
        .expect("user feed should remain accessible")
        .items;
    assert_eq!(user_feed.len(), 1);
    assert_eq!(
        user_feed[0].message_id.as_deref(),
        Some("msg_typed_device_scope_1")
    );
    assert_eq!(
        service
            .latest_client_route_sync_seq_from_auth_context(&user_auth, "d_shared")
            .expect("user seq should remain accessible"),
        1
    );

    let agent_feed = service
        .client_route_sync_feed_window_from_auth_context(
            &agent_auth,
            "d_shared",
            Some(0),
            Some(100),
        )
        .expect("agent feed should remain accessible")
        .items;
    assert!(agent_feed.is_empty());
    assert_eq!(
        service
            .latest_client_route_sync_seq_from_auth_context(&agent_auth, "d_shared")
            .expect("agent seq should remain accessible"),
        0
    );
}

#[test]
fn test_default_client_route_registration_does_not_leak_across_principal_kinds() {
    let service = TimelineProjectionService::default();

    let default_client_route = service.register_client_route("100001", "default", "1018", "d_legacy");
    assert_eq!(default_client_route.principal_kind, "user");

    let user_client_routes =
        service.registered_client_routes_for_principal_kind("100001", "default", "1018", "user");
    assert_eq!(user_client_routes.len(), 1);
    assert_eq!(user_client_routes[0].device_id, "d_legacy");
    assert_eq!(user_client_routes[0].principal_kind, "user");

    let agent_client_routes =
        service.registered_client_routes_for_principal_kind("100001", "default", "1018", "agent");
    assert!(
        agent_client_routes.is_empty(),
        "user default client route registration must not be visible to same-id agent principals"
    );
}

#[test]
fn test_default_client_route_queries_default_to_user_principal_kind() {
    let service = TimelineProjectionService::default();

    service.register_client_route_for_principal_kind("100001", "default", "1018", "user", "d_user");
    service.register_client_route_for_principal_kind("100001", "default", "1018", "agent", "d_agent");

    let default_client_routes = service.registered_client_routes("100001", "default", "1018");
    assert_eq!(default_client_routes.len(), 1);
    assert_eq!(default_client_routes[0].device_id, "d_user");
    assert_eq!(default_client_routes[0].principal_kind, "user");
}

#[test]
fn test_registered_client_route_timestamps_advance_between_distinct_registrations() {
    let service = TimelineProjectionService::default();

    let first = service.register_client_route("100001", "default", "1", "d_phone");
    sleep(Duration::from_millis(20));
    let second = service.register_client_route("100001", "default", "1", "d_pad");

    assert!(first.registered_at < second.registered_at);
}

#[test]
fn test_message_edit_and_recall_events_update_timeline_and_summary() {
    let service = TimelineProjectionService::default();

    let posted = im_domain_events::CommitEnvelope::minimal(
        "evt_message_posted",
        "100001",
        "message.posted",
        "conversation",
        "c_mutation",
        1,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_mutation",
            "messageId":"msg_c_mutation_1",
            "messageSeq":1,
            "sender":{"id":"1","kind":"user","memberId":"cm_demo","deviceId":"d_demo","sessionId":"s_demo","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_mutation_1",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"hello","parts":[{"kind":"text","text":"hello"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-05T10:00:01Z",
            "committedAt":"2026-04-05T10:00:01Z"
        }"#,
    );
    let edited = im_domain_events::CommitEnvelope::minimal(
        "evt_message_edited",
        "100001",
        "message.edited",
        "conversation",
        "c_mutation",
        1,
    )
    .with_payload(
        "message.edited.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_mutation",
            "messageId":"msg_c_mutation_1",
            "messageSeq":1,
            "body":{"summary":"edited","parts":[{"kind":"text","text":"edited"}],"renderHints":{}},
            "editor":{"id":"1","kind":"user","memberId":"cm_demo","deviceId":"d_demo","sessionId":"s_demo","metadata":{}},
            "editedAt":"2026-04-05T10:00:30Z"
        }"#,
    );
    let recalled = im_domain_events::CommitEnvelope::minimal(
        "evt_message_recalled",
        "100001",
        "message.recalled",
        "conversation",
        "c_mutation",
        1,
    )
    .with_payload(
        "message.recalled.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_mutation",
            "messageId":"msg_c_mutation_1",
            "messageSeq":1,
            "recalledBy":{"id":"1","kind":"user","memberId":"cm_demo","deviceId":"d_demo","sessionId":"s_demo","metadata":{}},
            "recalledAt":"2026-04-05T10:00:40Z"
        }"#,
    );

    service
        .apply(&posted)
        .expect("post projection should succeed");
    service
        .apply(&edited)
        .expect("edit projection should succeed");
    service
        .apply(&recalled)
        .expect("recall projection should succeed");

    let timeline = service.timeline("100001", "default", "c_mutation");
    assert_eq!(timeline.len(), 1);
    assert_eq!(timeline[0].message_id, "msg_c_mutation_1");
    assert_eq!(timeline[0].summary.as_deref(), Some("[recalled]"));

    let summary = service
        .conversation_summary("100001", "default", "c_mutation")
        .expect("summary should exist");
    assert_eq!(summary.last_message_id.as_deref(), Some("msg_c_mutation_1"));
    assert_eq!(summary.last_summary.as_deref(), Some("[recalled]"));
}

#[test]
fn test_message_mutation_client_route_sync_fanout_uses_payload_actor_kind() {
    let service = TimelineProjectionService::default();

    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_mutation_fanout_agent_member",
                "100001",
                "conversation.member_joined",
                "conversation",
                "c_mutation_actor_kind_fanout",
                1,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"100001",
                    "conversationId":"c_mutation_actor_kind_fanout",
                    "memberId":"cm_c_mutation_actor_kind_fanout_agent_bot",
                    "principalId":"bot",
                    "principalKind":"agent",
                    "role":"owner",
                    "state":"joined",
                    "invitedBy":null,
                    "joinedAt":"2026-04-10T12:00:00Z",
                    "removedAt":null,
                    "attributes":{}
                }"#,
            ),
        )
        .expect("agent member should project");
    service.register_client_route_for_principal_kind("100001", "default", "bot", "agent", "d_agent");
    service.register_client_route_for_principal_kind("100001", "default", "bot", "system", "d_system");

    let edited = im_domain_events::CommitEnvelope::minimal(
        "evt_mutation_fanout_agent_edit",
        "100001",
        "message.edited",
        "conversation",
        "c_mutation_actor_kind_fanout",
        2,
    )
    .with_payload(
        "message.edited.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_mutation_actor_kind_fanout",
            "messageId":"msg_c_mutation_actor_kind_fanout_1",
            "messageSeq":1,
            "body":{"summary":"edited by agent","parts":[{"kind":"text","text":"edited by agent"}],"renderHints":{}},
            "editor":{"id":"bot","kind":"agent","memberId":"cm_c_mutation_actor_kind_fanout_agent_bot","deviceId":"d_agent","sessionId":"s_agent","metadata":{}},
            "editedAt":"2026-04-10T12:00:10Z"
        }"#,
    );
    service.apply(&edited).expect("agent edit should project");

    let agent_feed = service
        .client_route_sync_feed_window_for_principal_kind(
            "100001", "default", "bot",
            "agent",
            "d_agent",
            Some(0),
            100,
        )
        .items;
    assert_eq!(agent_feed.len(), 1);
    assert_eq!(agent_feed[0].actor_id.as_deref(), Some("bot"));
    assert_eq!(agent_feed[0].actor_kind.as_deref(), Some("agent"));

    let system_feed = service
        .client_route_sync_feed_window_for_principal_kind(
            "100001", "default", "bot",
            "system",
            "d_system",
            Some(0),
            100,
        )
        .items;
    assert!(
        system_feed.is_empty(),
        "message mutation fanout must not route payload agent events to same-id system devices"
    );
}

#[test]
fn test_reaction_and_pin_events_project_into_interaction_summary_views() {
    let service = TimelineProjectionService::default();

    let posted = im_domain_events::CommitEnvelope::minimal(
        "evt_interaction_posted",
        "100001",
        "message.posted",
        "conversation",
        "c_interaction",
        1,
    )
    .with_payload(
        "message.posted.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_interaction",
            "messageId":"msg_c_interaction_1",
            "messageSeq":1,
            "sender":{"id":"1","kind":"user","memberId":"cm_owner","deviceId":"d_owner","sessionId":"s_owner","metadata":{}},
            "messageType":"standard",
            "deliveryMode":"discrete",
            "clientMsgId":"client_interaction_1",
            "streamSessionId":null,
            "rtcSessionId":null,
            "body":{"summary":"interaction target","parts":[{"kind":"text","text":"interaction target"}],"renderHints":{}},
            "attributes":{},
            "metadata":{},
            "occurredAt":"2026-04-10T12:00:00Z",
            "committedAt":"2026-04-10T12:00:00Z"
        }"#,
    );
    let reaction_added_owner = im_domain_events::CommitEnvelope::minimal(
        "evt_interaction_reaction_owner",
        "100001",
        "message.reaction_added",
        "conversation",
        "c_interaction",
        2,
    )
    .with_payload(
        "message.reaction_added.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_interaction",
            "messageId":"msg_c_interaction_1",
            "messageSeq":1,
            "reactionKey":"thumbs_up",
            "reactedBy":{"id":"1","kind":"user","memberId":"cm_owner","deviceId":"d_owner","sessionId":"s_owner","metadata":{}},
            "reactedAt":"2026-04-10T12:00:10Z"
        }"#,
    );
    let reaction_added_member = im_domain_events::CommitEnvelope::minimal(
        "evt_interaction_reaction_member",
        "100001",
        "message.reaction_added",
        "conversation",
        "c_interaction",
        3,
    )
    .with_payload(
        "message.reaction_added.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_interaction",
            "messageId":"msg_c_interaction_1",
            "messageSeq":1,
            "reactionKey":"thumbs_up",
            "reactedBy":{"id":"1014","kind":"user","memberId":"cm_member","deviceId":"d_member","sessionId":"s_member","metadata":{}},
            "reactedAt":"2026-04-10T12:00:11Z"
        }"#,
    );
    let pinned = im_domain_events::CommitEnvelope::minimal(
        "evt_interaction_pin",
        "100001",
        "message.pin_added",
        "conversation",
        "c_interaction",
        4,
    )
    .with_payload(
        "message.pin_added.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_interaction",
            "messageId":"msg_c_interaction_1",
            "messageSeq":1,
            "pinnedBy":{"id":"1","kind":"user","memberId":"cm_owner","deviceId":"d_owner","sessionId":"s_owner","metadata":{}},
            "pinnedAt":"2026-04-10T12:00:20Z"
        }"#,
    );
    let reaction_removed_owner = im_domain_events::CommitEnvelope::minimal(
        "evt_interaction_reaction_removed_owner",
        "100001",
        "message.reaction_removed",
        "conversation",
        "c_interaction",
        5,
    )
    .with_payload(
        "message.reaction_removed.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_interaction",
            "messageId":"msg_c_interaction_1",
            "messageSeq":1,
            "reactionKey":"thumbs_up",
            "removedBy":{"id":"1","kind":"user","memberId":"cm_owner","deviceId":"d_owner","sessionId":"s_owner","metadata":{}},
            "removedAt":"2026-04-10T12:00:30Z"
        }"#,
    );
    let unpinned = im_domain_events::CommitEnvelope::minimal(
        "evt_interaction_unpin",
        "100001",
        "message.pin_removed",
        "conversation",
        "c_interaction",
        6,
    )
    .with_payload(
        "message.pin_removed.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_interaction",
            "messageId":"msg_c_interaction_1",
            "messageSeq":1,
            "unpinnedBy":{"id":"1","kind":"user","memberId":"cm_owner","deviceId":"d_owner","sessionId":"s_owner","metadata":{}},
            "unpinnedAt":"2026-04-10T12:00:40Z"
        }"#,
    );

    for event in [
        posted,
        reaction_added_owner.clone(),
        reaction_added_owner,
        reaction_added_member,
        pinned.clone(),
        pinned,
        reaction_removed_owner.clone(),
        reaction_removed_owner,
        unpinned.clone(),
        unpinned,
    ] {
        service
            .apply(&event)
            .expect("interaction projection should succeed");
    }

    let summary = service
        .message_interaction_summary("100001", "default", "c_interaction", "msg_c_interaction_1")
        .expect("interaction summary should exist");
    assert_eq!(summary.message_seq, 1);
    assert_eq!(summary.total_reaction_count, 1);
    assert_eq!(
        summary.reaction_counts,
        vec![MessageReactionCountView {
            reaction_key: "thumbs_up".into(),
            count: 1,
        }]
    );
    assert_eq!(summary.pin, None);

    assert!(
        service
            .pinned_messages("100001", "default", "c_interaction")
            .is_empty(),
        "unpinned message should not stay in pinned-message summary view"
    );
}

#[test]
fn test_message_interaction_reactions_are_isolated_by_actor_kind() {
    let service = TimelineProjectionService::default();

    let user_reaction_added = im_domain_events::CommitEnvelope::minimal(
        "evt_interaction_reaction_user_shared",
        "100001",
        "message.reaction_added",
        "conversation",
        "c_interaction_typed_actor",
        1,
    )
    .with_payload(
        "message.reaction_added.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_interaction_typed_actor",
            "messageId":"msg_c_interaction_typed_actor_1",
            "messageSeq":1,
            "reactionKey":"thumbs_up",
            "reactedBy":{"id":"shared_actor","kind":"user","memberId":"cm_c_interaction_typed_actor_user_shared_actor","deviceId":"d_user","sessionId":"s_user","metadata":{}},
            "reactedAt":"2026-04-10T12:00:10Z"
        }"#,
    );
    let agent_reaction_added = im_domain_events::CommitEnvelope::minimal(
        "evt_interaction_reaction_agent_shared",
        "100001",
        "message.reaction_added",
        "conversation",
        "c_interaction_typed_actor",
        2,
    )
    .with_payload(
        "message.reaction_added.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_interaction_typed_actor",
            "messageId":"msg_c_interaction_typed_actor_1",
            "messageSeq":1,
            "reactionKey":"thumbs_up",
            "reactedBy":{"id":"shared_actor","kind":"agent","memberId":"cm_c_interaction_typed_actor_agent_shared_actor","deviceId":"d_agent","sessionId":"s_agent","metadata":{}},
            "reactedAt":"2026-04-10T12:00:11Z"
        }"#,
    );
    let user_reaction_removed = im_domain_events::CommitEnvelope::minimal(
        "evt_interaction_reaction_user_shared_removed",
        "100001",
        "message.reaction_removed",
        "conversation",
        "c_interaction_typed_actor",
        3,
    )
    .with_payload(
        "message.reaction_removed.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_interaction_typed_actor",
            "messageId":"msg_c_interaction_typed_actor_1",
            "messageSeq":1,
            "reactionKey":"thumbs_up",
            "removedBy":{"id":"shared_actor","kind":"user","memberId":"cm_c_interaction_typed_actor_user_shared_actor","deviceId":"d_user","sessionId":"s_user","metadata":{}},
            "removedAt":"2026-04-10T12:00:12Z"
        }"#,
    );

    service
        .apply(&user_reaction_added)
        .expect("user reaction should project");
    service
        .apply(&agent_reaction_added)
        .expect("agent reaction should project");

    let summary = service
        .message_interaction_summary("100001", "default", "c_interaction_typed_actor",
            "msg_c_interaction_typed_actor_1",
        )
        .expect("interaction summary should exist");
    assert_eq!(summary.total_reaction_count, 2);
    assert_eq!(
        summary.reaction_counts,
        vec![MessageReactionCountView {
            reaction_key: "thumbs_up".into(),
            count: 2,
        }]
    );

    service
        .apply(&user_reaction_removed)
        .expect("user reaction removal should project");

    let summary = service
        .message_interaction_summary("100001", "default", "c_interaction_typed_actor",
            "msg_c_interaction_typed_actor_1",
        )
        .expect("agent reaction should remain after removing same-id user reaction");
    assert_eq!(summary.total_reaction_count, 1);
    assert_eq!(
        summary.reaction_counts,
        vec![MessageReactionCountView {
            reaction_key: "thumbs_up".into(),
            count: 1,
        }]
    );
}

#[test]
fn test_message_interaction_client_route_sync_fanout_uses_payload_actor_kind() {
    let service = TimelineProjectionService::default();

    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_interaction_fanout_owner",
                "100001",
                "conversation.member_joined",
                "conversation",
                "c_interaction_actor_kind_fanout",
                1,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"100001",
                    "conversationId":"c_interaction_actor_kind_fanout",
                    "memberId":"cm_c_interaction_actor_kind_fanout_agent_bot",
                    "principalId":"bot",
                    "principalKind":"agent",
                    "role":"owner",
                    "state":"joined",
                    "invitedBy":null,
                    "joinedAt":"2026-04-10T12:00:00Z",
                    "removedAt":null,
                    "attributes":{}
                }"#,
            ),
        )
        .expect("agent member should project");
    service.register_client_route_for_principal_kind("100001", "default", "bot", "agent", "d_agent");
    service.register_client_route_for_principal_kind("100001", "default", "bot", "system", "d_system");

    let reaction_added = im_domain_events::CommitEnvelope::minimal(
        "evt_interaction_fanout_agent_reaction",
        "100001",
        "message.reaction_added",
        "conversation",
        "c_interaction_actor_kind_fanout",
        2,
    )
    .with_payload(
        "message.reaction_added.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_interaction_actor_kind_fanout",
            "messageId":"msg_c_interaction_actor_kind_fanout_1",
            "messageSeq":1,
            "reactionKey":"thumbs_up",
            "reactedBy":{"id":"bot","kind":"agent","memberId":"cm_c_interaction_actor_kind_fanout_agent_bot","deviceId":"d_agent","sessionId":"s_agent","metadata":{}},
            "reactedAt":"2026-04-10T12:00:10Z"
        }"#,
    );
    service
        .apply(&reaction_added)
        .expect("agent reaction should project");

    let agent_feed = service
        .client_route_sync_feed_window_for_principal_kind(
            "100001", "default", "bot",
            "agent",
            "d_agent",
            Some(0),
            100,
        )
        .items;
    assert_eq!(agent_feed.len(), 1);
    assert_eq!(agent_feed[0].actor_id.as_deref(), Some("bot"));
    assert_eq!(agent_feed[0].actor_kind.as_deref(), Some("agent"));

    let system_feed = service
        .client_route_sync_feed_window_for_principal_kind(
            "100001", "default", "bot",
            "system",
            "d_system",
            Some(0),
            100,
        )
        .items;
    assert!(
        system_feed.is_empty(),
        "message interaction fanout must not route payload agent events to same-id system devices"
    );
}

#[test]
fn test_agent_handoff_lifecycle_projects_into_summary_and_inbox_views() {
    let service = TimelineProjectionService::default();

    let conversation_created = im_domain_events::CommitEnvelope::minimal(
        "evt_handoff_created",
        "100001",
        "conversation.created",
        "conversation",
        "c_handoff_projection",
        0,
    )
    .with_payload(
        "conversation.created.v1",
        r#"{
            "conversationId":"c_handoff_projection",
            "conversationType":"agent_handoff",
            "source":{"id":"ag_source","kind":"agent"},
            "target":{"id":"1014","kind":"user"},
            "handoff":{"sessionId":"hs_projection","reason":"manual_escalation","status":"open"}
        }"#,
    );
    let source_member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_handoff_source_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_handoff_projection",
        1,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_handoff_projection",
            "memberId":"cm_handoff_source",
            "principalId":"ag_source",
            "principalKind":"agent",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-06T10:00:00Z",
            "removedAt":null,
            "attributes":{"handoffRole":"source"}
        }"#,
    );
    let target_member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_handoff_target_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_handoff_projection",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_handoff_projection",
            "memberId":"cm_handoff_target",
            "principalId":"1014",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"ag_source",
            "joinedAt":"2026-04-06T10:00:00Z",
            "removedAt":null,
            "attributes":{"handoffRole":"target"}
        }"#,
    );
    let handoff_accepted = im_domain_events::CommitEnvelope::minimal(
        "evt_handoff_accepted",
        "100001",
        "conversation.agent_handoff_status_changed",
        "conversation",
        "c_handoff_projection",
        3,
    )
    .with_payload(
        "conversation.agent_handoff_status_changed.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_handoff_projection",
            "previousStatus":"open",
            "currentStatus":"accepted",
            "changedBy":{"id":"1014","kind":"user"},
            "changedAt":"2026-04-06T10:01:00Z",
            "state":{
                "tenantId":"100001",
                "conversationId":"c_handoff_projection",
                "status":"accepted",
                "source":{"id":"ag_source","kind":"agent"},
                "target":{"id":"1014","kind":"user"},
                "handoffSessionId":"hs_projection",
                "handoffReason":"manual_escalation",
                "acceptedAt":"2026-04-06T10:01:00Z",
                "acceptedBy":{"id":"1014","kind":"user"},
                "resolvedAt":null,
                "resolvedBy":null,
                "closedAt":null,
                "closedBy":null
            }
        }"#,
    );

    service
        .apply(&conversation_created)
        .expect("conversation created projection should succeed");
    service
        .apply(&source_member_joined)
        .expect("source member projection should succeed");
    service
        .apply(&target_member_joined)
        .expect("target member projection should succeed");

    let initial_summary = service
        .conversation_summary("100001", "default", "c_handoff_projection")
        .expect("handoff summary should exist immediately after create");
    let initial_handoff = initial_summary
        .agent_handoff
        .as_ref()
        .expect("handoff summary should expose handoff state");
    assert_eq!(initial_summary.message_count, 0);
    assert_eq!(initial_summary.last_message_id, None);
    assert_eq!(initial_handoff.status, "open");
    assert_eq!(initial_handoff.source.id, "ag_source");
    assert_eq!(initial_handoff.target.id, "1014");

    let initial_inbox = service.inbox_for_principal_kind("100001", "1014", "user");
    assert_eq!(initial_inbox.len(), 1);
    let initial_inbox_handoff = initial_inbox[0]
        .agent_handoff
        .as_ref()
        .expect("inbox should expose handoff state");
    assert_eq!(initial_inbox_handoff.status, "open");
    assert_eq!(initial_inbox[0].message_count, 0);
    assert_eq!(initial_inbox[0].unread_count, 0);

    service
        .apply(&handoff_accepted)
        .expect("handoff accepted projection should succeed");

    let accepted_summary = service
        .conversation_summary("100001", "default", "c_handoff_projection")
        .expect("handoff summary should still exist after accept");
    let accepted_handoff = accepted_summary
        .agent_handoff
        .as_ref()
        .expect("accepted summary should expose handoff state");
    assert_eq!(accepted_handoff.status, "accepted");
    assert_eq!(
        accepted_handoff
            .accepted_by
            .as_ref()
            .map(|actor| actor.id.as_str()),
        Some("1014")
    );

    let accepted_inbox = service.inbox_for_principal_kind("100001", "1014", "user");
    let accepted_inbox_handoff = accepted_inbox[0]
        .agent_handoff
        .as_ref()
        .expect("accepted inbox should expose handoff state");
    assert_eq!(accepted_inbox_handoff.status, "accepted");
}

#[test]
fn test_agent_handoff_status_change_projects_client_route_sync_entries_for_active_members() {
    let service = TimelineProjectionService::default();

    let conversation_created = im_domain_events::CommitEnvelope::minimal(
        "evt_handoff_sync_created",
        "100001",
        "conversation.created",
        "conversation",
        "c_handoff_sync",
        1,
    )
    .with_payload(
        "conversation.created.v1",
        r#"{
            "conversationId":"c_handoff_sync",
            "conversationType":"agent_handoff",
            "source":{"id":"ag_source","kind":"agent"},
            "target":{"id":"1014","kind":"user"},
            "handoff":{"sessionId":"hs_sync","reason":"manual_escalation","status":"open"}
        }"#,
    );
    let source_member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_handoff_sync_source_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_handoff_sync",
        2,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_handoff_sync",
            "memberId":"cm_handoff_sync_source",
            "principalId":"ag_source",
            "principalKind":"agent",
            "role":"owner",
            "state":"joined",
            "invitedBy":null,
            "joinedAt":"2026-04-06T11:00:00Z",
            "removedAt":null,
            "attributes":{"handoffRole":"source"}
        }"#,
    );
    let target_member_joined = im_domain_events::CommitEnvelope::minimal(
        "evt_handoff_sync_target_member",
        "100001",
        "conversation.member_joined",
        "conversation",
        "c_handoff_sync",
        3,
    )
    .with_payload(
        "conversation.member.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_handoff_sync",
            "memberId":"cm_handoff_sync_target",
            "principalId":"1014",
            "principalKind":"user",
            "role":"member",
            "state":"joined",
            "invitedBy":"ag_source",
            "joinedAt":"2026-04-06T11:00:00Z",
            "removedAt":null,
            "attributes":{"handoffRole":"target"}
        }"#,
    );
    let handoff_accepted = im_domain_events::CommitEnvelope::minimal(
        "evt_handoff_sync_accepted",
        "100001",
        "conversation.agent_handoff_status_changed",
        "conversation",
        "c_handoff_sync",
        4,
    )
    .with_payload(
        "conversation.agent_handoff_status_changed.v1",
        r#"{
            "tenantId":"100001",
            "conversationId":"c_handoff_sync",
            "previousStatus":"open",
            "currentStatus":"accepted",
            "changedBy":{"id":"1014","kind":"user"},
            "changedAt":"2026-04-06T11:01:00Z",
            "state":{
                "tenantId":"100001",
                "conversationId":"c_handoff_sync",
                "status":"accepted",
                "source":{"id":"ag_source","kind":"agent"},
                "target":{"id":"1014","kind":"user"},
                "handoffSessionId":"hs_sync",
                "handoffReason":"manual_escalation",
                "acceptedAt":"2026-04-06T11:01:00Z",
                "acceptedBy":{"id":"1014","kind":"user"},
                "resolvedAt":null,
                "resolvedBy":null,
                "closedAt":null,
                "closedBy":null
            }
        }"#,
    );

    service
        .apply(&conversation_created)
        .expect("conversation created projection should succeed");
    service
        .apply(&source_member_joined)
        .expect("source member projection should succeed");
    service
        .apply(&target_member_joined)
        .expect("target member projection should succeed");
    service.register_client_route("100001", "default", "1014", "d_pad");
    service.register_client_route_for_principal_kind("100001", "default", "ag_source", "agent", "d_agent");
    service
        .apply(&handoff_accepted)
        .expect("handoff accepted projection should succeed");

    let target_feed = service
        .client_route_sync_feed_window_for_principal_kind(
            "100001", "default", "1014",
            "user",
            "d_pad",
            Some(0),
            100,
        )
        .items;
    assert_eq!(target_feed.len(), 1);
    assert_eq!(
        target_feed[0].origin_event_type,
        "conversation.agent_handoff_status_changed"
    );
    assert_eq!(
        target_feed[0].conversation_id.as_deref(),
        Some("c_handoff_sync")
    );
    assert_eq!(target_feed[0].actor_id.as_deref(), Some("1014"));
    assert_eq!(target_feed[0].summary.as_deref(), Some("accepted"));
    assert_eq!(
        target_feed[0].payload_schema.as_deref(),
        Some("conversation.agent_handoff_status_changed.v1")
    );
    let target_payload: serde_json::Value = serde_json::from_str(
        target_feed[0]
            .payload
            .as_deref()
            .expect("target payload should be present"),
    )
    .expect("target payload should be valid json");
    assert_eq!(target_payload["conversationId"], "c_handoff_sync");
    assert_eq!(target_payload["currentStatus"], "accepted");
    assert_eq!(target_payload["changedBy"]["id"], "1014");
    assert_eq!(target_payload["state"]["status"], "accepted");
    assert_eq!(target_feed[0].message_id, None);
    assert_eq!(target_feed[0].read_seq, None);
    assert_eq!(target_feed[0].occurred_at, "2026-04-06T11:01:00Z");

    let source_feed = service
        .client_route_sync_feed_window_for_principal_kind(
            "100001", "default", "ag_source",
            "agent",
            "d_agent",
            Some(0),
            100,
        )
        .items;
    assert_eq!(source_feed.len(), 1);
    assert_eq!(source_feed[0].actor_id.as_deref(), Some("1014"));
    assert_eq!(source_feed[0].summary.as_deref(), Some("accepted"));
    assert_eq!(
        source_feed[0].payload_schema.as_deref(),
        Some("conversation.agent_handoff_status_changed.v1")
    );
    let source_payload: serde_json::Value = serde_json::from_str(
        source_feed[0]
            .payload
            .as_deref()
            .expect("source payload should be present"),
    )
    .expect("source payload should be valid json");
    assert_eq!(source_payload["conversationId"], "c_handoff_sync");
    assert_eq!(source_payload["currentStatus"], "accepted");
    assert_eq!(source_payload["changedBy"]["id"], "1014");
}
