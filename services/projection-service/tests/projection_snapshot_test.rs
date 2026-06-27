use im_adapters_local_memory::{MemoryMetadataStore, MemoryTimelineProjectionStore};
use im_app_context::AppContext;
use im_domain_core::conversation::{
    MembershipRole, build_conversation_member, build_default_read_cursor,
};
use projection_service::TimelineProjectionService;

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

fn typed_member_id(conversation_id: &str, principal_kind: &str, principal_id: &str) -> String {
    format!("cm_{conversation_id}_{principal_kind}_{principal_id}")
}

fn message_posted_event(
    tenant_id: &str,
    conversation_id: &str,
    message_id: &str,
    message_seq: u64,
    sender_id: &str,
    summary: &str,
) -> im_domain_events::CommitEnvelope {
    message_posted_event_at(
        tenant_id,
        conversation_id,
        message_id,
        message_seq,
        sender_id,
        summary,
        &format!("2026-04-08T10:00:0{message_seq}Z"),
        &format!("2026-04-08T10:00:0{message_seq}Z"),
    )
}

#[allow(clippy::too_many_arguments)]
fn message_posted_event_at(
    tenant_id: &str,
    conversation_id: &str,
    message_id: &str,
    message_seq: u64,
    sender_id: &str,
    summary: &str,
    occurred_at: &str,
    committed_at: &str,
) -> im_domain_events::CommitEnvelope {
    let sender_member_id = typed_member_id(conversation_id, "user", sender_id);
    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{conversation_id}_{message_seq}"),
        tenant_id,
        "message.posted",
        "conversation",
        conversation_id,
        message_seq,
    )
    .with_payload(
        "message.posted.v1",
        &format!(
            r#"{{
                "tenantId":"{tenant_id}",
                "conversationId":"{conversation_id}",
                "messageId":"{message_id}",
                "messageSeq":{message_seq},
                "sender":{{"id":"{sender_id}","kind":"user","memberId":"{sender_member_id}","deviceId":"d_{sender_id}","sessionId":"s_{sender_id}","metadata":{{}}}},
                "messageType":"standard",
                "deliveryMode":"discrete",
                "clientMsgId":"client_{message_id}",
                "streamSessionId":null,
                "rtcSessionId":null,
                "body":{{"summary":"{summary}","parts":[{{"kind":"text","text":"{summary}"}}],"renderHints":{{}}}},
                "attributes":{{}},
                "metadata":{{}},
                "occurredAt":"{occurred_at}",
                "committedAt":"{committed_at}"
            }}"#
        ),
    )
}

fn conversation_created_event(
    tenant_id: &str,
    conversation_id: &str,
    conversation_type: &str,
) -> im_domain_events::CommitEnvelope {
    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{conversation_id}_created"),
        tenant_id,
        "conversation.created",
        "conversation",
        conversation_id,
        0,
    )
    .with_payload(
        "conversation.created.v1",
        &serde_json::json!({
            "conversationType": conversation_type,
        })
        .to_string(),
    )
}

fn member_joined_event(
    tenant_id: &str,
    conversation_id: &str,
    principal_id: &str,
    role: MembershipRole,
) -> im_domain_events::CommitEnvelope {
    let member = build_conversation_member(
        tenant_id,
        conversation_id,
        typed_member_id(conversation_id, "user", principal_id),
        principal_id,
        "user",
        role,
        Some("1".into()),
        "2026-04-08T10:00:00Z".into(),
    );

    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{conversation_id}_{principal_id}_joined"),
        tenant_id,
        "conversation.member_joined",
        "conversation",
        conversation_id,
        1,
    )
    .with_payload(
        "conversation.member_joined.v1",
        &serde_json::to_string(&member).expect("member should serialize"),
    )
}

fn read_cursor_updated_event(
    tenant_id: &str,
    conversation_id: &str,
    principal_id: &str,
    read_seq: u64,
    last_read_message_id: Option<&str>,
) -> im_domain_events::CommitEnvelope {
    let member = build_conversation_member(
        tenant_id,
        conversation_id,
        typed_member_id(conversation_id, "user", principal_id),
        principal_id,
        "user",
        MembershipRole::Member,
        Some("1".into()),
        "2026-04-08T10:00:00Z".into(),
    );
    let mut cursor = build_default_read_cursor(&member);
    cursor.read_seq = read_seq;
    cursor.last_read_message_id = last_read_message_id.map(str::to_owned);
    cursor.updated_at = "2026-04-08T10:00:09Z".into();

    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{conversation_id}_{principal_id}_cursor"),
        tenant_id,
        "conversation.read_cursor_updated",
        "conversation",
        conversation_id,
        read_seq,
    )
    .with_payload(
        "conversation.read_cursor_updated.v1",
        &serde_json::to_string(&cursor).expect("cursor should serialize"),
    )
}

fn member_role_changed_event(
    tenant_id: &str,
    conversation_id: &str,
    principal_id: &str,
    previous_role: MembershipRole,
    updated_role: MembershipRole,
) -> im_domain_events::CommitEnvelope {
    let previous_member = build_conversation_member(
        tenant_id,
        conversation_id,
        typed_member_id(conversation_id, "user", principal_id),
        principal_id,
        "user",
        previous_role,
        Some("1".into()),
        "2026-04-08T10:00:00Z".into(),
    );
    let updated_member = build_conversation_member(
        tenant_id,
        conversation_id,
        typed_member_id(conversation_id, "user", principal_id),
        principal_id,
        "user",
        updated_role,
        Some("1".into()),
        "2026-04-08T10:00:00Z".into(),
    );

    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{conversation_id}_{principal_id}_role_changed"),
        tenant_id,
        "conversation.member_role_changed",
        "conversation",
        conversation_id,
        3,
    )
    .with_payload(
        "conversation.member_role_changed.v1",
        &serde_json::json!({
            "tenantId": tenant_id,
            "conversationId": conversation_id,
            "previousMember": previous_member,
            "updatedMember": updated_member,
            "changedAt": "2026-04-08T10:02:00Z"
        })
        .to_string(),
    )
}

fn friendship_activated_event(
    tenant_id: &str,
    friendship_id: &str,
    user_low_id: &str,
    user_high_id: &str,
    direct_chat_id: Option<&str>,
    established_at: &str,
) -> im_domain_events::CommitEnvelope {
    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{friendship_id}_friendship"),
        tenant_id,
        "friendship.activated",
        "friendship",
        friendship_id,
        1,
    )
    .with_payload(
        "social.friendship.activated.v1",
        &serde_json::json!({
            "friendshipId": friendship_id,
            "userLowId": user_low_id,
            "userHighId": user_high_id,
            "initiatorUserId": user_low_id,
            "directChatId": direct_chat_id,
            "establishedAt": established_at,
        })
        .to_string(),
    )
}

fn direct_chat_bound_event(
    tenant_id: &str,
    direct_chat_id: &str,
    conversation_id: &str,
    bound_at: &str,
) -> im_domain_events::CommitEnvelope {
    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{direct_chat_id}_bound"),
        tenant_id,
        "direct_chat.bound",
        "direct_chat",
        direct_chat_id,
        1,
    )
    .with_payload(
        "social.direct_chat.bound.v1",
        &serde_json::json!({
            "directChatId": direct_chat_id,
            "conversationId": conversation_id,
            "leftActorId": "actor_alice",
            "rightActorId": "actor_bob",
            "pairHash": "actor_alice:actor_bob",
            "boundAt": bound_at,
        })
        .to_string(),
    )
}

fn message_reaction_added_event(
    tenant_id: &str,
    conversation_id: &str,
    message_id: &str,
    message_seq: u64,
    reaction_key: &str,
    actor_id: &str,
    reacted_at: &str,
) -> im_domain_events::CommitEnvelope {
    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{message_id}_{reaction_key}_{actor_id}_reaction_added"),
        tenant_id,
        "message.reaction_added",
        "conversation",
        conversation_id,
        message_seq + 10,
    )
    .with_payload(
        "message.reaction_added.v1",
        &serde_json::json!({
            "tenantId": tenant_id,
            "conversationId": conversation_id,
            "messageId": message_id,
            "messageSeq": message_seq,
            "reactionKey": reaction_key,
            "reactedBy": {
                "id": actor_id,
                "kind": "user",
                "memberId": typed_member_id(conversation_id, "user", actor_id),
                "deviceId": format!("d_{actor_id}"),
                "sessionId": format!("s_{actor_id}"),
                "metadata": {}
            },
            "reactedAt": reacted_at
        })
        .to_string(),
    )
}

fn message_pinned_event(
    tenant_id: &str,
    conversation_id: &str,
    message_id: &str,
    message_seq: u64,
    actor_id: &str,
    pinned_at: &str,
) -> im_domain_events::CommitEnvelope {
    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{message_id}_{actor_id}_pin_added"),
        tenant_id,
        "message.pin_added",
        "conversation",
        conversation_id,
        message_seq + 11,
    )
    .with_payload(
        "message.pin_added.v1",
        &serde_json::json!({
            "tenantId": tenant_id,
            "conversationId": conversation_id,
            "messageId": message_id,
            "messageSeq": message_seq,
            "pinnedBy": {
                "id": actor_id,
                "kind": "user",
                "memberId": typed_member_id(conversation_id, "user", actor_id),
                "deviceId": format!("d_{actor_id}"),
                "sessionId": format!("s_{actor_id}"),
                "metadata": {}
            },
            "pinnedAt": pinned_at
        })
        .to_string(),
    )
}

#[test]
fn test_projection_service_restores_tenant_scoped_conversation_snapshots_from_shared_stores() {
    let metadata_store = MemoryMetadataStore::default();
    let timeline_store = MemoryTimelineProjectionStore::default();
    let service = TimelineProjectionService::default();

    let alpha_event = message_posted_event(
        "t_alpha",
        "c_shared",
        "msg_alpha_1",
        1,
        "1010",
        "alpha summary",
    );
    let beta_event = message_posted_event(
        "t_beta",
        "c_shared",
        "msg_beta_1",
        1,
        "1028",
        "beta summary",
    );

    service
        .apply(&alpha_event)
        .expect("alpha projection should succeed");
    service
        .apply(&beta_event)
        .expect("beta projection should succeed");

    service
        .persist_conversation_snapshot("t_alpha", "default", "c_shared", &metadata_store, &timeline_store)
        .expect("alpha snapshot should persist");
    service
        .persist_conversation_snapshot("t_beta", "default", "c_shared", &metadata_store, &timeline_store)
        .expect("beta snapshot should persist");

    let restored = TimelineProjectionService::default();
    assert!(
        restored
            .restore_conversation_snapshot("t_alpha", "default", "c_shared", &metadata_store, &timeline_store,)
            .expect("alpha snapshot should restore"),
        "alpha snapshot should exist"
    );
    assert!(
        restored
            .restore_conversation_snapshot("t_beta", "default", "c_shared", &metadata_store, &timeline_store)
            .expect("beta snapshot should restore"),
        "beta snapshot should exist"
    );

    let alpha_timeline = restored.timeline("t_alpha", "default", "c_shared");
    assert_eq!(alpha_timeline.len(), 1);
    assert_eq!(alpha_timeline[0].message_id, "msg_alpha_1");
    assert_eq!(alpha_timeline[0].summary.as_deref(), Some("alpha summary"));

    let beta_timeline = restored.timeline("t_beta", "default", "c_shared");
    assert_eq!(beta_timeline.len(), 1);
    assert_eq!(beta_timeline[0].message_id, "msg_beta_1");
    assert_eq!(beta_timeline[0].summary.as_deref(), Some("beta summary"));

    let alpha_summary = restored
        .conversation_summary("t_alpha", "default", "c_shared")
        .expect("alpha summary should restore");
    assert_eq!(
        alpha_summary.last_message_id.as_deref(),
        Some("msg_alpha_1")
    );
    assert_eq!(alpha_summary.last_summary.as_deref(), Some("alpha summary"));

    let beta_summary = restored
        .conversation_summary("t_beta", "default", "c_shared")
        .expect("beta summary should restore");
    assert_eq!(beta_summary.last_message_id.as_deref(), Some("msg_beta_1"));
    assert_eq!(beta_summary.last_summary.as_deref(), Some("beta summary"));
}

#[test]
fn test_projection_service_restores_member_cursor_and_inbox_views_from_snapshot_metadata() {
    let metadata_store = MemoryMetadataStore::default();
    let timeline_store = MemoryTimelineProjectionStore::default();
    let service = TimelineProjectionService::default();

    service
        .apply(&conversation_created_event("t_alpha", "c_restore", "group"))
        .expect("conversation projection should succeed");
    service
        .apply(&member_joined_event(
            "t_alpha",
            "c_restore",
            "1014",
            MembershipRole::Member,
        ))
        .expect("member join projection should succeed");
    service
        .apply(&member_joined_event(
            "t_alpha",
            "c_restore",
            "1029",
            MembershipRole::Member,
        ))
        .expect("peer join projection should succeed");
    service
        .apply(&message_posted_event(
            "t_alpha",
            "c_restore",
            "msg_restore_1",
            1,
            "1014",
            "restored summary",
        ))
        .expect("message projection should succeed");
    service
        .apply(&message_posted_event(
            "t_alpha",
            "c_restore",
            "msg_restore_2",
            2,
            "1029",
            "restored peer reply",
        ))
        .expect("peer message projection should succeed");
    service
        .apply(&read_cursor_updated_event(
            "t_alpha",
            "c_restore",
            "1014",
            1,
            Some("msg_restore_1"),
        ))
        .expect("cursor projection should succeed");

    assert!(
        service
            .persist_conversation_snapshot("t_alpha", "default", "c_restore", &metadata_store, &timeline_store)
            .expect("snapshot should persist"),
        "snapshot should exist"
    );

    let restored = TimelineProjectionService::default();
    assert!(
        restored
            .restore_conversation_snapshot("t_alpha", "default", "c_restore", &metadata_store, &timeline_store)
            .expect("snapshot should restore"),
        "snapshot should restore"
    );

    let member = restored
        .member_snapshot_for_principal_kind("t_alpha", "default", "c_restore", "1014", "user")
        .expect("member should restore");
    assert_eq!(member.member_id, "cm_c_restore_user_u_member");
    assert_eq!(member.principal_id, "1014");

    let read_cursor = restored
        .read_cursor_for_principal_kind("t_alpha", "default", "c_restore", "1014", "user")
        .expect("read cursor should restore");
    assert_eq!(read_cursor.read_seq, 1);
    assert_eq!(
        read_cursor.last_read_message_id.as_deref(),
        Some("msg_restore_1")
    );
    assert_eq!(read_cursor.unread_count, 1);

    let inbox = restored.inbox_for_principal_kind("t_alpha", "1014", "user");
    assert_eq!(inbox.len(), 1);
    assert_eq!(inbox[0].conversation_id, "c_restore");
    assert_eq!(inbox[0].conversation_type, "group");
    assert_eq!(
        inbox[0].last_summary.as_deref(),
        Some("restored peer reply")
    );
    assert_eq!(inbox[0].unread_count, 1);
}

#[test]
fn test_projection_service_restores_member_directory_view_from_snapshot_metadata() {
    let metadata_store = MemoryMetadataStore::default();
    let timeline_store = MemoryTimelineProjectionStore::default();
    let service = TimelineProjectionService::default();

    service
        .apply(&conversation_created_event(
            "t_alpha",
            "c_directory_restore",
            "group",
        ))
        .expect("conversation projection should succeed");
    service
        .apply(&member_joined_event(
            "t_alpha",
            "c_directory_restore",
            "1",
            MembershipRole::Owner,
        ))
        .expect("owner join projection should succeed");
    service
        .apply(&member_joined_event(
            "t_alpha",
            "c_directory_restore",
            "1014",
            MembershipRole::Member,
        ))
        .expect("member join projection should succeed");
    service
        .apply(&member_role_changed_event(
            "t_alpha",
            "c_directory_restore",
            "1014",
            MembershipRole::Member,
            MembershipRole::Admin,
        ))
        .expect("role change projection should succeed");

    assert!(
        service
            .persist_conversation_snapshot("t_alpha", "default", "c_directory_restore", &metadata_store,
                &timeline_store,
            )
            .expect("snapshot should persist"),
        "snapshot should exist"
    );

    let restored = TimelineProjectionService::default();
    assert!(
        restored
            .restore_conversation_snapshot("t_alpha", "default", "c_directory_restore", &metadata_store,
                &timeline_store,
            )
            .expect("snapshot should restore"),
        "snapshot should restore"
    );

    let directory = restored.member_directory("t_alpha", "default", "c_directory_restore");
    assert_eq!(directory.len(), 2);
    assert_eq!(directory[0].principal_id, "1");
    assert_eq!(directory[0].role, MembershipRole::Owner);
    assert_eq!(directory[1].principal_id, "1014");
    assert_eq!(directory[1].role, MembershipRole::Admin);
}

#[test]
fn test_projection_service_restores_client_route_sync_state_from_projection_snapshot() {
    let metadata_store = MemoryMetadataStore::default();
    let timeline_store = MemoryTimelineProjectionStore::default();
    let service = TimelineProjectionService::default();

    service
        .apply(&conversation_created_event(
            "t_alpha",
            "c_client_route_sync_restore",
            "group",
        ))
        .expect("conversation projection should succeed");
    service
        .apply(&member_joined_event(
            "t_alpha",
            "c_client_route_sync_restore",
            "1014",
            MembershipRole::Member,
        ))
        .expect("member join projection should succeed");
    service.register_client_route("t_alpha", "default", "1014", "d_phone");
    service.register_client_route("t_alpha", "default", "1014", "d_pad");
    service
        .apply(&message_posted_event(
            "t_alpha",
            "c_client_route_sync_restore",
            "msg_client_route_sync_restore_1",
            1,
            "1014",
            "device snapshot summary",
        ))
        .expect("message projection should succeed");
    service
        .apply(&read_cursor_updated_event(
            "t_alpha",
            "c_client_route_sync_restore",
            "1014",
            1,
            Some("msg_client_route_sync_restore_1"),
        ))
        .expect("read cursor projection should succeed");

    assert!(
        service
            .persist_conversation_snapshot("t_alpha", "default", "c_client_route_sync_restore", &metadata_store,
                &timeline_store,
            )
            .expect("snapshot should persist"),
        "snapshot should exist"
    );

    let restored = TimelineProjectionService::default();
    assert!(
        restored
            .restore_conversation_snapshot("t_alpha", "default", "c_client_route_sync_restore", &metadata_store,
                &timeline_store,
            )
            .expect("snapshot should restore"),
        "snapshot should exist"
    );

    let devices = restored.registered_client_routes("t_alpha", "default", "1014");
    assert_eq!(devices.len(), 2);
    assert_eq!(devices[0].device_id, "d_pad");
    assert_eq!(devices[1].device_id, "d_phone");

    let phone_feed = restored
        .client_route_sync_feed_window_for_principal_kind(
            "t_alpha", "default", "1014",
            "user",
            "d_phone",
            Some(0),
            100,
        )
        .items;
    assert_eq!(phone_feed.len(), 2);
    assert_eq!(phone_feed[0].origin_event_type, "message.posted");
    assert_eq!(
        phone_feed[0].message_id.as_deref(),
        Some("msg_client_route_sync_restore_1")
    );
    assert_eq!(
        phone_feed[1].origin_event_type,
        "conversation.read_cursor_updated"
    );
    assert_eq!(phone_feed[1].read_seq, Some(1));

    let pad_feed = restored
        .client_route_sync_feed_window_for_principal_kind(
            "t_alpha", "default", "1014",
            "user",
            "d_pad",
            Some(0),
            100,
        )
        .items;
    assert_eq!(pad_feed.len(), 2);
    assert_eq!(pad_feed[0].origin_event_type, "message.posted");
    assert_eq!(
        pad_feed[1].origin_event_type,
        "conversation.read_cursor_updated"
    );

    assert_eq!(
        restored.latest_client_route_sync_seq("t_alpha", "default", "1014", "d_phone"),
        2
    );
    assert_eq!(
        restored.latest_client_route_sync_seq("t_alpha", "default", "1014", "d_pad"),
        2
    );
}

#[test]
fn test_projection_service_restores_typed_client_route_sync_state_for_same_actor_and_device() {
    let metadata_store = MemoryMetadataStore::default();
    let timeline_store = MemoryTimelineProjectionStore::default();
    let service = TimelineProjectionService::default();

    service
        .apply(&conversation_created_event(
            "t_alpha",
            "c_typed_client_route_sync_restore",
            "group",
        ))
        .expect("conversation projection should succeed");
    service
        .apply(&member_joined_event(
            "t_alpha",
            "c_typed_client_route_sync_restore",
            "1",
            MembershipRole::Owner,
        ))
        .expect("owner join projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_t_alpha_c_typed_client_route_sync_restore_u_dual_joined",
                "t_alpha",
                "conversation.member_joined",
                "conversation",
                "c_typed_client_route_sync_restore",
                2,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"t_alpha",
                    "conversationId":"c_typed_client_route_sync_restore",
                    "memberId":"cm_c_typed_client_route_sync_restore_u_dual_user",
                    "principalId":"1018",
                    "principalKind":"user",
                    "role":"member",
                    "state":"joined",
                    "invitedBy":"1",
                    "joinedAt":"2026-04-08T10:00:00Z",
                    "removedAt":null,
                    "attributes":{}
                }"#,
            ),
        )
        .expect("typed user join projection should succeed");

    service.register_client_route_for_principal_kind("t_alpha", "default", "1", "user", "d_owner");
    service.register_client_route_for_principal_kind("t_alpha", "default", "1018", "user", "d_shared");
    service.register_client_route_for_principal_kind("t_alpha", "default", "1018", "agent", "d_shared");
    service
        .apply(&message_posted_event(
            "t_alpha",
            "c_typed_client_route_sync_restore",
            "msg_typed_client_route_sync_restore_1",
            1,
            "1",
            "typed snapshot summary",
        ))
        .expect("message projection should succeed");

    assert!(
        service
            .persist_conversation_snapshot("t_alpha", "default", "c_typed_client_route_sync_restore", &metadata_store,
                &timeline_store,
            )
            .expect("snapshot should persist"),
        "snapshot should exist"
    );

    let restored = TimelineProjectionService::default();
    assert!(
        restored
            .restore_conversation_snapshot("t_alpha", "default", "c_typed_client_route_sync_restore", &metadata_store,
                &timeline_store,
            )
            .expect("snapshot should restore"),
        "snapshot should exist"
    );

    let user_auth = app_context(
        "t_alpha",
        "1018",
        "user",
        Some("s_typed_restore_user"),
        Some("d_shared"),
    );
    let agent_auth = app_context(
        "t_alpha",
        "1018",
        "agent",
        Some("s_typed_restore_agent"),
        Some("d_shared"),
    );

    let user_client_routes = restored.registered_client_routes_from_auth_context(&user_auth);
    assert_eq!(user_client_routes.len(), 1);
    assert_eq!(user_client_routes[0].device_id, "d_shared");
    assert_eq!(user_client_routes[0].principal_kind, "user");

    let agent_client_routes = restored.registered_client_routes_from_auth_context(&agent_auth);
    assert_eq!(agent_client_routes.len(), 1);
    assert_eq!(agent_client_routes[0].device_id, "d_shared");
    assert_eq!(agent_client_routes[0].principal_kind, "agent");

    let user_feed = restored
        .client_route_sync_feed_window_from_auth_context(&user_auth, "d_shared", Some(0), Some(100))
        .expect("restored user feed should remain accessible")
        .items;
    assert_eq!(user_feed.len(), 1);
    assert_eq!(
        user_feed[0].message_id.as_deref(),
        Some("msg_typed_client_route_sync_restore_1")
    );
    assert_eq!(
        restored
            .latest_client_route_sync_seq_from_auth_context(&user_auth, "d_shared")
            .expect("restored user seq should remain accessible"),
        1
    );

    let agent_feed = restored
        .client_route_sync_feed_window_from_auth_context(
            &agent_auth,
            "d_shared",
            Some(0),
            Some(100),
        )
        .expect("restored agent feed should remain accessible")
        .items;
    assert!(agent_feed.is_empty());
    assert_eq!(
        restored
            .latest_client_route_sync_seq_from_auth_context(&agent_auth, "d_shared")
            .expect("restored agent seq should remain accessible"),
        0
    );
}

#[test]
fn test_projection_service_restores_contacts_view_from_snapshot_metadata() {
    let metadata_store = MemoryMetadataStore::default();
    let timeline_store = MemoryTimelineProjectionStore::default();
    let service = TimelineProjectionService::default();

    service
        .apply(&conversation_created_event(
            "t_alpha",
            "c_contacts_seed",
            "direct",
        ))
        .expect("conversation projection should succeed");
    service
        .apply(&friendship_activated_event(
            "t_alpha",
            "fs_001",
            "1016",
            "1020",
            Some("dc_001"),
            "2026-04-10T12:00:00Z",
        ))
        .expect("friendship projection should succeed");
    service
        .apply(&direct_chat_bound_event(
            "t_alpha",
            "dc_001",
            "c_direct_001",
            "2026-04-10T12:05:00Z",
        ))
        .expect("direct chat enrich should succeed");

    assert!(
        service
            .persist_conversation_snapshot("t_alpha", "default", "c_contacts_seed", &metadata_store,
                &timeline_store,
            )
            .expect("snapshot should persist"),
        "snapshot should exist"
    );

    let restored = TimelineProjectionService::default();
    assert!(
        restored
            .restore_conversation_snapshot("t_alpha", "default", "c_contacts_seed", &metadata_store,
                &timeline_store,
            )
            .expect("snapshot should restore"),
        "snapshot should restore"
    );

    let contacts = restored.contacts("t_alpha", "default", "1016");
    assert_eq!(contacts.len(), 1);
    assert_eq!(contacts[0].target_user_id, "1020");
    assert_eq!(contacts[0].conversation_id.as_deref(), Some("c_direct_001"));
    assert_eq!(contacts[0].last_interaction_at, "2026-04-10T12:05:00Z");
    assert_eq!(
        restored.direct_chat_id_for_conversation("t_alpha", "default", "c_direct_001"),
        Some("dc_001".into())
    );
}

#[test]
fn test_projection_service_restores_interaction_summary_view_from_snapshot_metadata() {
    let metadata_store = MemoryMetadataStore::default();
    let timeline_store = MemoryTimelineProjectionStore::default();
    let service = TimelineProjectionService::default();

    service
        .apply(&conversation_created_event(
            "t_alpha",
            "c_interaction_restore",
            "group",
        ))
        .expect("conversation projection should succeed");
    service
        .apply(&member_joined_event(
            "t_alpha",
            "c_interaction_restore",
            "1",
            MembershipRole::Owner,
        ))
        .expect("owner join projection should succeed");
    service
        .apply(&message_posted_event(
            "t_alpha",
            "c_interaction_restore",
            "msg_c_interaction_restore_1",
            1,
            "1",
            "restore interaction summary",
        ))
        .expect("message projection should succeed");
    service
        .apply(&message_reaction_added_event(
            "t_alpha",
            "c_interaction_restore",
            "msg_c_interaction_restore_1",
            1,
            "thumbs_up",
            "1",
            "2026-04-10T12:00:10Z",
        ))
        .expect("reaction projection should succeed");
    service
        .apply(&message_pinned_event(
            "t_alpha",
            "c_interaction_restore",
            "msg_c_interaction_restore_1",
            1,
            "1",
            "2026-04-10T12:00:20Z",
        ))
        .expect("pin projection should succeed");

    assert!(
        service
            .persist_conversation_snapshot("t_alpha", "default", "c_interaction_restore", &metadata_store,
                &timeline_store,
            )
            .expect("snapshot should persist"),
        "snapshot should exist"
    );

    let restored = TimelineProjectionService::default();
    assert!(
        restored
            .restore_conversation_snapshot("t_alpha", "default", "c_interaction_restore", &metadata_store,
                &timeline_store,
            )
            .expect("snapshot should restore"),
        "snapshot should restore"
    );

    let summary = restored
        .message_interaction_summary("t_alpha", "default", "c_interaction_restore",
            "msg_c_interaction_restore_1",
        )
        .expect("interaction summary should restore");
    assert_eq!(summary.total_reaction_count, 1);
    assert_eq!(summary.reaction_counts[0].reaction_key, "thumbs_up");
    assert_eq!(summary.reaction_counts[0].count, 1);
    assert_eq!(
        summary.pin.as_ref().map(|pin| pin.pinned_by.id.as_str()),
        Some("1")
    );

    let pins = restored.pinned_messages("t_alpha", "default", "c_interaction_restore");
    assert_eq!(pins.len(), 1);
    assert_eq!(pins[0].message_id, "msg_c_interaction_restore_1");
}

#[test]
fn test_projection_service_records_snapshot_observability_metrics_traces_and_logs() {
    let metadata_store = MemoryMetadataStore::default();
    let timeline_store = MemoryTimelineProjectionStore::default();
    let service = TimelineProjectionService::default();

    service
        .apply(&conversation_created_event("t_alpha", "c_obs", "group"))
        .expect("conversation projection should succeed");
    service
        .apply(&member_joined_event(
            "t_alpha",
            "c_obs",
            "1014",
            MembershipRole::Member,
        ))
        .expect("member join projection should succeed");
    service.register_client_route("t_alpha", "default", "1014", "d_pad");
    service
        .apply(&message_posted_event(
            "t_alpha",
            "c_obs",
            "msg_obs_1",
            1,
            "1014",
            "projection observability",
        ))
        .expect("message projection should succeed");

    assert!(
        service
            .persist_conversation_snapshot("t_alpha", "default", "c_obs", &metadata_store, &timeline_store)
            .expect("snapshot should persist"),
        "snapshot should exist"
    );

    let restored = TimelineProjectionService::default();
    assert!(
        restored
            .restore_conversation_snapshot("t_alpha", "default", "c_obs", &metadata_store, &timeline_store)
            .expect("snapshot should restore"),
        "snapshot should restore"
    );

    let snapshot = restored.projection_plane_observability();
    assert_eq!(snapshot.status, "ok");
    assert_eq!(
        snapshot.metrics.conversation_snapshot_restore.success_count,
        1
    );
    assert_eq!(
        snapshot
            .metrics
            .client_route_sync_snapshot_restore
            .success_count,
        1
    );
    assert!(
        snapshot
            .traces
            .iter()
            .any(|item| item.operation == "conversation_snapshot.restore"
                && item.scope_id == "7#t_alpha7#default5#c_obs"
                && item.outcome == "success"),
        "restore trace should be recorded"
    );
    assert!(
        snapshot
            .logs
            .iter()
            .any(|item| item.code == "projection_snapshot_restore_succeeded"
                && item.operation == "conversation_snapshot.restore"),
        "restore structured log should be recorded"
    );

    let live_snapshot = service.projection_plane_observability();
    assert_eq!(live_snapshot.status, "ok");
    assert_eq!(
        live_snapshot
            .metrics
            .conversation_snapshot_persist
            .success_count,
        1
    );
    assert_eq!(
        live_snapshot
            .metrics
            .client_route_sync_snapshot_persist
            .success_count,
        1
    );
    assert!(
        live_snapshot.traces.iter().any(|item| item.operation
            == "client_route_sync_snapshot.persist"
            && item.outcome == "success"),
        "client route sync persist trace should be recorded"
    );
    assert!(
        live_snapshot
            .logs
            .iter()
            .any(|item| item.code == "projection_snapshot_persist_succeeded"
                && item.operation == "conversation_snapshot.persist"),
        "persist structured log should be recorded"
    );
}

#[test]
fn test_projection_service_records_projection_replay_metrics() {
    let service = TimelineProjectionService::default();

    let idle = service.projection_plane_observability();
    assert_eq!(idle.replay.backlog_size, 0);
    assert_eq!(idle.replay.replayed_event_count, 0);
    assert_eq!(idle.replay.duration_ms, 0);

    service.record_projection_replay_metrics(3, 2, 17);

    let replay = service.projection_plane_observability();
    assert_eq!(replay.replay.backlog_size, 3);
    assert_eq!(replay.replay.replayed_event_count, 2);
    assert_eq!(replay.replay.duration_ms, 17);
}

#[test]
fn test_projection_service_records_projection_rebuild_duration() {
    let service = TimelineProjectionService::default();

    let idle = service.projection_plane_observability();
    assert_eq!(idle.rebuild_duration_ms, 0);

    service.record_projection_rebuild_duration(23);

    let snapshot = service.projection_plane_observability();
    assert_eq!(snapshot.rebuild_duration_ms, 23);
}

#[test]
fn test_projection_service_tracks_live_projection_lag_per_scope() {
    let service = TimelineProjectionService::default();
    service
        .apply(&conversation_created_event(
            "100001",
            "c_projection_live_lag",
            "group",
        ))
        .expect("conversation projection should succeed");
    service
        .apply(&message_posted_event(
            "100001",
            "c_projection_live_lag",
            "msg_projection_live_lag_1",
            1,
            "1",
            "live lag summary",
        ))
        .expect("message projection should succeed");

    let lag_json = serde_json::to_value(service.projection_live_lag_items())
        .expect("lag items should serialize");
    assert!(
        lag_json.as_array().unwrap().iter().any(|item| {
            item["component"] == "projection_live"
                && item["scopeId"] == "6#1000017#default21#c_projection_live_lag"
                && item["currentOffset"] == 1
                && item["committedOffset"] == 1
                && item["lag"] == 0
        }),
        "projection-service should expose zero live lag after the real apply path catches up"
    );
}

#[test]
fn test_projection_service_records_projection_update_delay_metrics() {
    let service = TimelineProjectionService::default();
    service
        .apply(&conversation_created_event(
            "100001",
            "c_projection_delay",
            "group",
        ))
        .expect("conversation projection should succeed");
    service
        .apply(&member_joined_event(
            "100001",
            "c_projection_delay",
            "1",
            MembershipRole::Member,
        ))
        .expect("member join projection should succeed");
    service
        .apply(&message_posted_event_at(
            "100001",
            "c_projection_delay",
            "msg_projection_delay_1",
            1,
            "1",
            "projection delay summary",
            "1970-01-01T00:00:00.000Z",
            "1970-01-01T00:00:00.000Z",
        ))
        .expect("message projection should succeed");

    let snapshot_json = serde_json::to_value(service.projection_plane_observability())
        .expect("view should serialize");
    assert!(
        snapshot_json["updateDelay"]["timelineMs"].as_u64().unwrap() >= 1,
        "projection observability should expose positive timeline update delay"
    );
    assert!(
        snapshot_json["updateDelay"]["inboxMs"].as_u64().unwrap() >= 1,
        "projection observability should expose positive inbox update delay"
    );
    assert_eq!(
        snapshot_json["updateDelay"]["sourceEventType"],
        "message.posted"
    );
    assert_eq!(
        snapshot_json["updateDelay"]["scopeId"],
        "6#1000017#default18#c_projection_delay"
    );
}
