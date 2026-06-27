use axum::body::Body;
use axum::http::{Request, StatusCode, header::CONTENT_TYPE};
use http_body_util::BodyExt;
use im_app_context::DualTokenRequestBuilderExt;
use tower::ServiceExt;

fn timeline_message_posted_event(
    tenant_id: &str,
    conversation_id: &str,
    message_id: &str,
    message_seq: u64,
    sender_id: &str,
    member_id: &str,
    summary: &str,
) -> im_domain_events::CommitEnvelope {
    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{message_id}"),
        tenant_id,
        "message.posted",
        "conversation",
        conversation_id,
        message_seq,
    )
    .with_payload(
        "message.posted.v1",
        &serde_json::json!({
            "tenantId": tenant_id,
            "conversationId": conversation_id,
            "messageId": message_id,
            "messageSeq": message_seq,
            "sender": {
                "id": sender_id,
                "kind": "user",
                "memberId": member_id,
                "deviceId": "d_demo",
                "sessionId": "s_demo",
                "metadata": {}
            },
            "messageType": "standard",
            "deliveryMode": "discrete",
            "clientMsgId": format!("client_{message_id}"),
            "streamSessionId": null,
            "rtcSessionId": null,
            "body": {
                "summary": summary,
                "parts": [{"kind": "text", "text": summary}],
                "renderHints": {}
            },
            "attributes": {},
            "metadata": {},
            "occurredAt": format!("2026-04-05T10:00:0{message_seq}Z"),
            "committedAt": format!("2026-04-05T10:00:0{message_seq}Z")
        })
        .to_string(),
    )
}

#[tokio::test]
async fn test_public_app_exports_live_openapi_json() {
    let app = projection_service::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/openapi.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("body should be valid json");

    assert_eq!(value["openapi"], "3.1.0");
    assert_eq!(value["info"]["title"], "Sdkwork IM Projection Service API");
    assert!(value["paths"]["/im/v3/api/chat/inbox"].is_object());
}

#[tokio::test]
async fn test_public_app_serves_docs_page_for_live_openapi() {
    let app = projection_service::build_public_app();

    let response = app
        .oneshot(Request::builder().uri("/docs").body(Body::empty()).unwrap())
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let html = String::from_utf8(body.to_vec()).expect("docs should be valid utf-8");

    assert!(html.contains("OpenAPI 3.1"));
    assert!(html.contains("Sdkwork IM Projection Service API"));
    assert!(html.contains("/openapi.json"));
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
        message_seq + 1,
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
                "memberId": format!("cm_{actor_id}"),
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
        message_seq + 2,
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
                "memberId": format!("cm_{actor_id}"),
                "deviceId": format!("d_{actor_id}"),
                "sessionId": format!("s_{actor_id}"),
                "metadata": {}
            },
            "pinnedAt": pinned_at
        })
        .to_string(),
    )
}

#[tokio::test]
async fn test_timeline_query_returns_projected_messages() {
    let service = projection_service::TimelineProjectionService::default();
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_member",
                "100001",
                "conversation.member_joined",
                "conversation",
                "c_demo",
                0,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"100001",
                    "conversationId":"c_demo",
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
            ),
        )
        .expect("member projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
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
            ),
        )
        .expect("projection should succeed");

    let app = projection_service::build_app(std::sync::Arc::new(service));

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_demo/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");

    assert_eq!(value["items"][0]["messageId"], "m_demo");
    assert_eq!(value["items"][0]["summary"], "hello");

    let summary_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_demo")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("summary request should succeed");

    assert_eq!(summary_response.status(), StatusCode::OK);
    let summary_body = summary_response
        .into_body()
        .collect()
        .await
        .expect("summary body should collect")
        .to_bytes();
    let summary_value: serde_json::Value =
        serde_json::from_slice(&summary_body).expect("summary should be valid json");

    assert_eq!(summary_value["messageCount"], 1);
    assert_eq!(summary_value["lastMessageId"], "m_demo");
    assert_eq!(summary_value["lastSender"]["id"], "1");

    let forbidden_timeline_response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_demo/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1030")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("forbidden timeline request should succeed");
    assert_eq!(forbidden_timeline_response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_timeline_http_returns_bounded_cursor_window() {
    let service = projection_service::TimelineProjectionService::default();

    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_member",
                "100001",
                "conversation.member_joined",
                "conversation",
                "c_timeline_page",
                1,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"100001",
                    "conversationId":"c_timeline_page",
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
            ),
        )
        .expect("member projection should succeed");
    for seq in 1..=3 {
        service
            .apply(&timeline_message_posted_event(
                "100001",
                "c_timeline_page",
                &format!("m_page_{seq}"),
                seq,
                "1",
                "cm_demo",
                &format!("message {seq}"),
            ))
            .expect("message projection should succeed");
    }

    let app = projection_service::build_app(std::sync::Arc::new(service));

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_timeline_page/messages?afterSeq=0&limit=2")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("first timeline request should succeed");
    assert_eq!(first_response.status(), StatusCode::OK);
    let first_body = first_response
        .into_body()
        .collect()
        .await
        .expect("first body should collect")
        .to_bytes();
    let first_value: serde_json::Value =
        serde_json::from_slice(&first_body).expect("first page should be valid json");
    assert_eq!(first_value["items"].as_array().unwrap().len(), 2);
    assert_eq!(first_value["items"][0]["messageSeq"], 1);
    assert_eq!(first_value["items"][1]["messageSeq"], 2);
    assert_eq!(first_value["nextAfterSeq"], 2);
    assert_eq!(first_value["hasMore"], true);

    let second_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_timeline_page/messages?afterSeq=2&limit=2")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second timeline request should succeed");
    assert_eq!(second_response.status(), StatusCode::OK);
    let second_body = second_response
        .into_body()
        .collect()
        .await
        .expect("second body should collect")
        .to_bytes();
    let second_value: serde_json::Value =
        serde_json::from_slice(&second_body).expect("second page should be valid json");
    assert_eq!(second_value["items"].as_array().unwrap().len(), 1);
    assert_eq!(second_value["items"][0]["messageSeq"], 3);
    assert_eq!(second_value["nextAfterSeq"], 3);
    assert_eq!(second_value["hasMore"], false);

    let invalid_limit_response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_timeline_page/messages?afterSeq=0&limit=0")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("invalid limit request should complete");
    assert_eq!(invalid_limit_response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        invalid_limit_response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("application/problem+json; charset=utf-8")
    );
    let invalid_body = invalid_limit_response
        .into_body()
        .collect()
        .await
        .expect("invalid body should collect")
        .to_bytes();
    let invalid_value: serde_json::Value =
        serde_json::from_slice(&invalid_body).expect("invalid response should be valid json");
    assert_eq!(invalid_value["type"], "about:blank");
    assert_eq!(invalid_value["title"], "Bad Request");
    assert_eq!(invalid_value["status"], 400);
    assert!(
        invalid_value["detail"]
            .as_str()
            .expect("detail should be present")
            .contains("limit")
    );
    assert_eq!(invalid_value["code"], "limit_invalid");
}

#[tokio::test]
async fn test_timeline_query_rejects_same_actor_id_with_different_actor_kind_over_http() {
    let service = projection_service::TimelineProjectionService::default();
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_actor_kind_member",
                "100001",
                "conversation.member_joined",
                "conversation",
                "c_actor_kind_guard",
                0,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"100001",
                    "conversationId":"c_actor_kind_guard",
                    "memberId":"cm_actor_kind_guard_demo",
                    "principalId":"1",
                    "principalKind":"user",
                    "role":"owner",
                    "state":"joined",
                    "invitedBy":null,
                    "joinedAt":"2026-04-13T10:00:00Z",
                    "removedAt":null,
                    "attributes":{}
                }"#,
            ),
        )
        .expect("member projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_actor_kind_message",
                "100001",
                "message.posted",
                "conversation",
                "c_actor_kind_guard",
                1,
            )
            .with_payload(
                "message.posted.v1",
                r#"{
                    "tenantId":"100001",
                    "conversationId":"c_actor_kind_guard",
                    "messageId":"msg_actor_kind_guard_1",
                    "messageSeq":1,
                    "sender":{"id":"1","kind":"user","memberId":"cm_actor_kind_guard_demo","deviceId":"d_demo","sessionId":"s_demo","metadata":{}},
                    "messageType":"standard",
                    "deliveryMode":"discrete",
                    "clientMsgId":"client_actor_kind_guard_1",
                    "streamSessionId":null,
                    "rtcSessionId":null,
                    "body":{"summary":"guarded","parts":[{"kind":"text","text":"guarded"}],"renderHints":{}},
                    "attributes":{},
                    "metadata":{},
                    "occurredAt":"2026-04-13T10:00:01Z",
                    "committedAt":"2026-04-13T10:00:01Z"
                }"#,
            ),
        )
        .expect("message projection should succeed");

    let app = projection_service::build_app(std::sync::Arc::new(service));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_actor_kind_guard/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("system")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("actor-kind mismatch timeline request should return response");

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    assert_eq!(
        response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|header| header.to_str().ok()),
        Some("application/problem+json; charset=utf-8")
    );
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["type"], "about:blank");
    assert_eq!(value["title"], "Forbidden");
    assert_eq!(value["status"], 403);
    assert!(
        !value["detail"]
            .as_str()
            .expect("detail should be present")
            .is_empty()
    );
    assert_eq!(value["detail"], value["message"]);
    assert_eq!(value["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_read_cursor_query_returns_projected_cursor_view() {
    let service = projection_service::TimelineProjectionService::default();
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_member",
                "100001",
                "conversation.member_joined",
                "conversation",
                "c_cursor",
                0,
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
            ),
        )
        .expect("member projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_peer_member",
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
                    "memberId":"cm_peer",
                    "principalId":"1029",
                    "principalKind":"user",
                    "role":"member",
                    "state":"joined",
                    "invitedBy":"1",
                    "joinedAt":"2026-04-05T10:00:01Z",
                    "removedAt":null,
                    "attributes":{}
                }"#,
            ),
        )
        .expect("peer member projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
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
                    "messageId":"m_demo_2",
                    "messageSeq":2,
                    "sender":{"id":"1029","kind":"user","memberId":"cm_peer","deviceId":null,"sessionId":"s_peer","metadata":{}},
                    "messageType":"standard",
                    "deliveryMode":"discrete",
                    "clientMsgId":"client_demo_2",
                    "streamSessionId":null,
                    "rtcSessionId":null,
                    "body":{"summary":"hello","parts":[{"kind":"text","text":"hello"}],"renderHints":{}},
                    "attributes":{},
                    "metadata":{},
                    "occurredAt":"2026-04-05T10:00:02Z",
                    "committedAt":"2026-04-05T10:00:02Z"
                }"#,
            ),
        )
        .expect("message projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
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
                    "lastReadMessageId":"m_demo_1",
                    "updatedAt":"2026-04-05T10:00:10Z"
                }"#,
            ),
        )
        .expect("read cursor projection should succeed");

    let app = projection_service::build_app(std::sync::Arc::new(service));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_cursor/read_cursor")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("read cursor request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");

    assert_eq!(value["readSeq"], 1);
    assert_eq!(value["unreadCount"], 1);
    assert_eq!(value["memberId"], "cm_demo");
}

#[tokio::test]
async fn test_inbox_query_returns_projected_entries() {
    let service = projection_service::TimelineProjectionService::default();
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
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
            ),
        )
        .expect("conversation projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
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
            ),
        )
        .expect("member projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
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
                    "messageId":"m_demo_2",
                    "messageSeq":2,
                    "sender":{"id":"1013","kind":"user","memberId":"cm_other","deviceId":null,"sessionId":"s_other","metadata":{}},
                    "messageType":"standard",
                    "deliveryMode":"discrete",
                    "clientMsgId":"client_demo_2",
                    "streamSessionId":null,
                    "rtcSessionId":null,
                    "body":{"summary":"hello","parts":[{"kind":"text","text":"hello"}],"renderHints":{}},
                    "attributes":{},
                    "metadata":{},
                    "occurredAt":"2026-04-05T10:00:02Z",
                    "committedAt":"2026-04-05T10:00:02Z"
                }"#,
            ),
        )
        .expect("message projection should succeed");

    let app = projection_service::build_app(std::sync::Arc::new(service));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/inbox")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("inbox request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");

    assert_eq!(value["items"][0]["conversationId"], "c_inbox");
    assert_eq!(value["items"][0]["conversationType"], "group");
    assert_eq!(value["items"][0]["messageCount"], 2);
}

#[tokio::test]
async fn test_inbox_query_returns_bounded_cursor_window() {
    let service = projection_service::TimelineProjectionService::default();

    for seq in 1..=3 {
        let conversation_id = format!("c_inbox_page_{seq}");
        service
            .apply(
                &im_domain_events::CommitEnvelope::minimal(
                    &format!("evt_inbox_page_conversation_{seq}"),
                    "100001",
                    "conversation.created",
                    "conversation",
                    conversation_id.as_str(),
                    seq,
                )
                .with_payload(
                    "conversation.created.v1",
                    &serde_json::json!({
                        "conversationId": conversation_id,
                        "conversationType": "group"
                    })
                    .to_string(),
                ),
            )
            .expect("conversation projection should succeed");
        service
            .apply(
                &im_domain_events::CommitEnvelope::minimal(
                    &format!("evt_inbox_page_member_{seq}"),
                    "100001",
                    "conversation.member_joined",
                    "conversation",
                    conversation_id.as_str(),
                    seq,
                )
                .with_payload(
                    "conversation.member.v1",
                    &serde_json::json!({
                        "tenantId":"100001",
                        "conversationId": conversation_id,
                        "memberId": format!("cm_inbox_page_{seq}"),
                        "principalId":"1",
                        "principalKind":"user",
                        "role":"owner",
                        "state":"joined",
                        "invitedBy":null,
                        "joinedAt":"2026-04-05T10:00:00Z",
                        "removedAt":null,
                        "attributes":{}
                    })
                    .to_string(),
                ),
            )
            .expect("member projection should succeed");
        service
            .apply(&timeline_message_posted_event(
                "100001",
                conversation_id.as_str(),
                &format!("m_inbox_page_{seq}"),
                seq,
                "1",
                &format!("cm_inbox_page_{seq}"),
                &format!("page {seq}"),
            ))
            .expect("message projection should succeed");
    }

    let app = projection_service::build_app(std::sync::Arc::new(service));
    let first = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/inbox?limit=2")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("first inbox page should return response");
    assert_eq!(first.status(), StatusCode::OK);
    let first_body = first
        .into_body()
        .collect()
        .await
        .expect("first inbox page body should collect")
        .to_bytes();
    let first_json: serde_json::Value =
        serde_json::from_slice(&first_body).expect("first inbox page should be json");
    assert_eq!(first_json["items"].as_array().unwrap().len(), 2);
    assert_eq!(first_json["items"][0]["conversationId"], "c_inbox_page_3");
    assert_eq!(first_json["items"][1]["conversationId"], "c_inbox_page_2");
    assert_eq!(first_json["hasMore"], true);
    let next_cursor = first_json["nextCursor"]
        .as_str()
        .expect("first inbox page should include nextCursor");

    let second = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/inbox?limit=2&cursor={next_cursor}"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second inbox page should return response");
    assert_eq!(second.status(), StatusCode::OK);
    let second_body = second
        .into_body()
        .collect()
        .await
        .expect("second inbox page body should collect")
        .to_bytes();
    let second_json: serde_json::Value =
        serde_json::from_slice(&second_body).expect("second inbox page should be json");
    assert_eq!(second_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(second_json["items"][0]["conversationId"], "c_inbox_page_1");
    assert_eq!(second_json["hasMore"], false);
    assert_eq!(second_json["nextCursor"], serde_json::Value::Null);

    let invalid = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/inbox?limit=0")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("invalid inbox limit should return response");
    assert_eq!(invalid.status(), StatusCode::BAD_REQUEST);
    let invalid_body = invalid
        .into_body()
        .collect()
        .await
        .expect("invalid inbox body should collect")
        .to_bytes();
    let invalid_json: serde_json::Value =
        serde_json::from_slice(&invalid_body).expect("invalid inbox body should be json");
    assert_eq!(invalid_json["code"], "limit_invalid");
}

#[tokio::test]
async fn test_timeline_query_rejects_oversized_conversation_id_over_http() {
    let app = projection_service::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{}/messages",
                    "c".repeat(2048)
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("oversized timeline query should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("conversationId")
    );
}

#[tokio::test]
async fn test_interaction_summary_rejects_oversized_message_id_over_http() {
    let service = projection_service::TimelineProjectionService::default();
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_member_interaction_limit",
                "100001",
                "conversation.member_joined",
                "conversation",
                "c_limit_interaction",
                1,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"100001",
                    "conversationId":"c_limit_interaction",
                    "memberId":"cm_demo",
                    "principalId":"1",
                    "principalKind":"user",
                    "role":"owner",
                    "state":"joined",
                    "invitedBy":null,
                    "joinedAt":"2026-04-12T10:00:00Z",
                    "removedAt":null,
                    "attributes":{}
                }"#,
            ),
        )
        .expect("member projection should succeed");

    let app = projection_service::build_app(std::sync::Arc::new(service));
    let response = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/c_limit_interaction/messages/{}/interaction_summary",
                    "m".repeat(2048)
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("oversized interaction summary query should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("messageId")
    );
}

#[tokio::test]
async fn test_member_directory_query_returns_projected_members() {
    let service = projection_service::TimelineProjectionService::default();
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_directory_owner",
                "100001",
                "conversation.member_joined",
                "conversation",
                "c_directory",
                1,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"100001",
                    "conversationId":"c_directory",
                    "memberId":"cm_directory_owner",
                    "principalId":"1",
                    "principalKind":"user",
                    "role":"owner",
                    "state":"joined",
                    "invitedBy":null,
                    "joinedAt":"2026-04-05T10:00:00Z",
                    "removedAt":null,
                    "attributes":{"displayName":"Owner"}
                }"#,
            ),
        )
        .expect("owner projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_directory_member",
                "100001",
                "conversation.member_joined",
                "conversation",
                "c_directory",
                2,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"100001",
                    "conversationId":"c_directory",
                    "memberId":"cm_directory_member",
                    "principalId":"1014",
                    "principalKind":"user",
                    "role":"member",
                    "state":"joined",
                    "invitedBy":"1",
                    "joinedAt":"2026-04-05T10:01:00Z",
                    "removedAt":null,
                    "attributes":{"displayName":"Member"}
                }"#,
            ),
        )
        .expect("member projection should succeed");

    let app = projection_service::build_app(std::sync::Arc::new(service));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_directory/member_directory")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("member directory request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");

    assert_eq!(value["items"].as_array().unwrap().len(), 2);
    assert_eq!(value["items"][0]["principalId"], "1");
    assert_eq!(value["items"][0]["role"], "owner");
    assert_eq!(value["items"][1]["principalId"], "1014");
    assert_eq!(value["items"][1]["attributes"]["displayName"], "Member");
}

#[tokio::test]
async fn test_contacts_query_returns_friendship_projection_with_direct_chat_enrich() {
    let service = projection_service::TimelineProjectionService::default();
    service
        .apply(&friendship_activated_event(
            "100001",
            "fs_001",
            "1016",
            "1020",
            Some("dc_001"),
            "2026-04-10T12:00:00Z",
        ))
        .expect("friendship projection should succeed");
    service
        .apply(&friendship_activated_event(
            "100001",
            "fs_002",
            "1016",
            "1031",
            None,
            "2026-04-10T11:00:00Z",
        ))
        .expect("second friendship projection should succeed");
    service
        .apply(&direct_chat_bound_event(
            "100001",
            "dc_001",
            "c_direct_001",
            "2026-04-10T12:05:00Z",
        ))
        .expect("direct chat enrich should succeed");

    let app = projection_service::build_app(std::sync::Arc::new(service));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/contacts")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1016")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("contacts request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("contacts body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("contacts body should be valid json");

    let items = value["items"]
        .as_array()
        .expect("contacts items should be array");
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["ownerUserId"], "1016");
    assert_eq!(items[0]["targetUserId"], "1020");
    assert_eq!(items[0]["contactType"], "friendship");
    assert_eq!(items[0]["relationshipState"], "active");
    assert_eq!(items[0]["friendshipId"], "fs_001");
    assert_eq!(items[0]["directChatId"], "dc_001");
    assert_eq!(items[0]["conversationId"], "c_direct_001");
    assert_eq!(items[0]["lastInteractionAt"], "2026-04-10T12:05:00Z");
    assert_eq!(items[1]["targetUserId"], "1031");
    assert_eq!(items[1]["conversationId"], serde_json::Value::Null);
}

#[tokio::test]
async fn test_contacts_query_returns_bounded_cursor_window() {
    let service = projection_service::TimelineProjectionService::default();
    for seq in 1..=3 {
        service
            .apply(&friendship_activated_event(
                "100001",
                &format!("fs_contact_page_{seq}"),
                "1016",
                &format!("{}", 1034 + seq),
                None,
                &format!("2026-04-10T12:0{seq}:00Z"),
            ))
            .expect("friendship projection should succeed");
    }

    let app = projection_service::build_app(std::sync::Arc::new(service));
    let first = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/contacts?limit=2")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1016")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("first contacts page should return response");
    assert_eq!(first.status(), StatusCode::OK);
    let first_body = first
        .into_body()
        .collect()
        .await
        .expect("first contacts page body should collect")
        .to_bytes();
    let first_json: serde_json::Value =
        serde_json::from_slice(&first_body).expect("first contacts page should be json");
    assert_eq!(first_json["items"].as_array().unwrap().len(), 2);
    assert_eq!(first_json["items"][0]["targetUserId"], "1037");
    assert_eq!(first_json["items"][1]["targetUserId"], "1036");
    assert_eq!(first_json["hasMore"], true);
    let next_cursor = first_json["nextCursor"]
        .as_str()
        .expect("first contacts page should include nextCursor");

    let second = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/contacts?limit=2&cursor={next_cursor}"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1016")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second contacts page should return response");
    assert_eq!(second.status(), StatusCode::OK);
    let second_body = second
        .into_body()
        .collect()
        .await
        .expect("second contacts page body should collect")
        .to_bytes();
    let second_json: serde_json::Value =
        serde_json::from_slice(&second_body).expect("second contacts page should be json");
    assert_eq!(second_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(second_json["items"][0]["targetUserId"], "1035");
    assert_eq!(second_json["hasMore"], false);
    assert_eq!(second_json["nextCursor"], serde_json::Value::Null);

    let invalid = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/contacts?limit=0")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1016")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("invalid contacts limit should return response");
    assert_eq!(invalid.status(), StatusCode::BAD_REQUEST);
    let invalid_body = invalid
        .into_body()
        .collect()
        .await
        .expect("invalid contacts body should collect")
        .to_bytes();
    let invalid_json: serde_json::Value =
        serde_json::from_slice(&invalid_body).expect("invalid contacts body should be json");
    assert_eq!(invalid_json["code"], "limit_invalid");
}

#[tokio::test]
async fn test_contacts_query_rejects_same_actor_id_with_different_actor_kind_over_http() {
    let service = projection_service::TimelineProjectionService::default();
    service
        .apply(&friendship_activated_event(
            "100001",
            "fs_actor_kind_contacts",
            "1016",
            "1020",
            Some("dc_actor_kind_contacts"),
            "2026-04-13T12:00:00Z",
        ))
        .expect("friendship projection should succeed");
    service
        .apply(&direct_chat_bound_event(
            "100001",
            "dc_actor_kind_contacts",
            "c_actor_kind_contacts",
            "2026-04-13T12:05:00Z",
        ))
        .expect("direct chat enrich should succeed");

    let app = projection_service::build_app(std::sync::Arc::new(service));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/contacts")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1016")
                .with_dual_token_actor_kind("system")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("actor-kind mismatch contacts request should return response");

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "contact_scope_forbidden");
}

#[tokio::test]
async fn test_interaction_summary_and_pins_query_return_projected_reaction_and_pin_views() {
    let service = projection_service::TimelineProjectionService::default();
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_interaction_owner_joined",
                "100001",
                "conversation.member_joined",
                "conversation",
                "c_interaction_http",
                1,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"100001",
                    "conversationId":"c_interaction_http",
                    "memberId":"cm_u_1",
                    "principalId":"1",
                    "principalKind":"user",
                    "role":"owner",
                    "state":"joined",
                    "invitedBy":null,
                    "joinedAt":"2026-04-10T12:00:00Z",
                    "removedAt":null,
                    "attributes":{}
                }"#,
            ),
        )
        .expect("owner projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_interaction_member_joined",
                "100001",
                "conversation.member_joined",
                "conversation",
                "c_interaction_http",
                2,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"100001",
                    "conversationId":"c_interaction_http",
                    "memberId":"cm_u_member",
                    "principalId":"1014",
                    "principalKind":"user",
                    "role":"member",
                    "state":"joined",
                    "invitedBy":"1",
                    "joinedAt":"2026-04-10T12:00:01Z",
                    "removedAt":null,
                    "attributes":{}
                }"#,
            ),
        )
        .expect("member projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_interaction_posted",
                "100001",
                "message.posted",
                "conversation",
                "c_interaction_http",
                3,
            )
            .with_payload(
                "message.posted.v1",
                r#"{
                    "tenantId":"100001",
                    "conversationId":"c_interaction_http",
                    "messageId":"msg_c_interaction_http_1",
                    "messageSeq":1,
                    "sender":{"id":"1","kind":"user","memberId":"cm_u_1","deviceId":"d_owner","sessionId":"s_owner","metadata":{}},
                    "messageType":"standard",
                    "deliveryMode":"discrete",
                    "clientMsgId":"client_interaction_http_1",
                    "streamSessionId":null,
                    "rtcSessionId":null,
                    "body":{"summary":"interaction http","parts":[{"kind":"text","text":"interaction http"}],"renderHints":{}},
                    "attributes":{},
                    "metadata":{},
                    "occurredAt":"2026-04-10T12:00:02Z",
                    "committedAt":"2026-04-10T12:00:02Z"
                }"#,
            ),
        )
        .expect("message projection should succeed");
    service
        .apply(&message_reaction_added_event(
            "100001",
            "c_interaction_http",
            "msg_c_interaction_http_1",
            1,
            "thumbs_up",
            "1",
            "2026-04-10T12:00:10Z",
        ))
        .expect("reaction projection should succeed");
    service
        .apply(&message_reaction_added_event(
            "100001",
            "c_interaction_http",
            "msg_c_interaction_http_1",
            1,
            "thumbs_up",
            "1014",
            "2026-04-10T12:00:11Z",
        ))
        .expect("second reaction projection should succeed");
    service
        .apply(&message_pinned_event(
            "100001",
            "c_interaction_http",
            "msg_c_interaction_http_1",
            1,
            "1",
            "2026-04-10T12:00:20Z",
        ))
        .expect("pin projection should succeed");

    let app = projection_service::build_app(std::sync::Arc::new(service));
    let summary_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_interaction_http/messages/msg_c_interaction_http_1/interaction_summary")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                    .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("interaction summary request should succeed");

    assert_eq!(summary_response.status(), StatusCode::OK);
    let summary_body = summary_response
        .into_body()
        .collect()
        .await
        .expect("interaction summary body should collect")
        .to_bytes();
    let summary_value: serde_json::Value = serde_json::from_slice(&summary_body)
        .expect("interaction summary body should be valid json");

    assert_eq!(summary_value["messageId"], "msg_c_interaction_http_1");
    assert_eq!(summary_value["messageSeq"], 1);
    assert_eq!(summary_value["totalReactionCount"], 2);
    assert_eq!(
        summary_value["reactionCounts"][0]["reactionKey"],
        "thumbs_up"
    );
    assert_eq!(summary_value["reactionCounts"][0]["count"], 2);
    assert_eq!(summary_value["pin"]["pinnedBy"]["id"], "1");
    assert_eq!(summary_value["pin"]["pinnedAt"], "2026-04-10T12:00:20Z");

    let pins_response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_interaction_http/pins")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1014")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("pins request should succeed");

    assert_eq!(pins_response.status(), StatusCode::OK);
    let pins_body = pins_response
        .into_body()
        .collect()
        .await
        .expect("pins body should collect")
        .to_bytes();
    let pins_value: serde_json::Value =
        serde_json::from_slice(&pins_body).expect("pins response should be valid json");

    let items = pins_value["items"]
        .as_array()
        .expect("pins items should be array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["messageId"], "msg_c_interaction_http_1");
    assert_eq!(items[0]["pin"]["pinnedBy"]["id"], "1");
}
