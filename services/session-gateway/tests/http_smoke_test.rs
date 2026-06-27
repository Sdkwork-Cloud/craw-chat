use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_app_context::DualTokenRequestBuilderExt;
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn test_healthz_returns_ok_and_service_metadata() {
    let app = session_gateway::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
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

    assert_eq!(value["status"], "ok");
    assert_eq!(value["service"], "session-gateway");
}

#[tokio::test]
async fn test_public_app_exports_live_openapi_json() {
    let app = session_gateway::build_public_app();

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
    assert_eq!(value["info"]["title"], "Sdkwork IM Realtime Gateway API");
    assert_eq!(
        value["paths"]["/im/v3/api/realtime/ws"]["get"]["summary"],
        "Open realtime websocket client route"
    );
}

#[tokio::test]
async fn test_public_app_serves_docs_page_for_live_openapi() {
    let app = session_gateway::build_public_app();

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
    assert!(html.contains("Sdkwork IM Realtime Gateway API"));
    assert!(html.contains("/openapi.json"));
}

#[tokio::test]
async fn test_public_app_rejects_missing_access_token_header_over_http() {
    let app = session_gateway::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("authorization", "Bearer auth_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"deviceId":"d_demo","lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("request should return response");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("body should be valid json");

    assert_eq!(value["code"], "access_token_missing");
}

#[tokio::test]
async fn test_presence_heartbeat_returns_presence_snapshot_for_current_route() {
    let app = session_gateway::build_app();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_demo")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "deviceId":"d_demo",
                        "lastSeenSyncSeq":0
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("presence heartbeat should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("body should be valid json");

    assert_eq!(value["currentDeviceId"], "d_demo");
    assert_eq!(value["devices"][0]["deviceId"], "d_demo");
    assert_eq!(value["devices"][0]["status"], "online");

    let snapshot = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/presence/me")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_demo")
                .with_dual_token_device("d_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("presence request should succeed");

    assert_eq!(snapshot.status(), StatusCode::OK);
    let snapshot_body = snapshot
        .into_body()
        .collect()
        .await
        .expect("snapshot body should collect")
        .to_bytes();
    let snapshot_value: serde_json::Value =
        serde_json::from_slice(&snapshot_body).expect("snapshot should be valid json");

    assert_eq!(snapshot_value["currentDeviceId"], "d_demo");
    assert_eq!(snapshot_value["devices"][0]["status"], "online");
}

#[tokio::test]
async fn test_presence_heartbeat_rejects_mismatched_client_route_id() {
    let app = session_gateway::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_demo")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "deviceId":"d_other",
                        "lastSeenSyncSeq":0
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("resume request should return response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("body should be valid json");

    assert_eq!(value["code"], "device_id_mismatch");
}

#[tokio::test]
async fn test_presence_snapshot_isolated_by_actor_kind_over_http() {
    let app = session_gateway::build_app();

    let user_resume = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1139")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_user")
                .with_dual_token_device("d_user")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("user resume should succeed");
    assert_eq!(user_resume.status(), StatusCode::OK);

    let agent_resume = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1139")
                .with_dual_token_actor_kind("agent")
                .with_dual_token_session("s_agent")
                .with_dual_token_device("d_agent")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("agent resume should succeed");
    assert_eq!(agent_resume.status(), StatusCode::OK);

    let user_presence = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/presence/me")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1139")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_user")
                .with_dual_token_device("d_user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("user presence request should succeed");
    assert_eq!(user_presence.status(), StatusCode::OK);
    let user_presence_body = user_presence
        .into_body()
        .collect()
        .await
        .expect("user presence body should collect")
        .to_bytes();
    let user_presence_json: serde_json::Value =
        serde_json::from_slice(&user_presence_body).expect("user presence should be valid json");
    let user_devices = user_presence_json["devices"]
        .as_array()
        .expect("user devices should be an array");
    assert_eq!(
        user_devices.len(),
        1,
        "user presence should not include devices from another actor kind sharing the same actor id"
    );
    assert_eq!(user_devices[0]["deviceId"], "d_user");

    let agent_presence = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/presence/me")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1139")
                .with_dual_token_actor_kind("agent")
                .with_dual_token_session("s_agent")
                .with_dual_token_device("d_agent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("agent presence request should succeed");
    assert_eq!(agent_presence.status(), StatusCode::OK);
    let agent_presence_body = agent_presence
        .into_body()
        .collect()
        .await
        .expect("agent presence body should collect")
        .to_bytes();
    let agent_presence_json: serde_json::Value =
        serde_json::from_slice(&agent_presence_body).expect("agent presence should be valid json");
    let agent_devices = agent_presence_json["devices"]
        .as_array()
        .expect("agent devices should be an array");
    assert_eq!(
        agent_devices.len(),
        1,
        "agent presence should not include devices from another actor kind sharing the same actor id"
    );
    assert_eq!(agent_devices[0]["deviceId"], "d_agent");
}

#[tokio::test]
async fn test_presence_heartbeat_rejects_same_route_id_with_different_actor_kind_over_http() {
    let app = session_gateway::build_app();

    let user_resume = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1139")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_user")
                .with_dual_token_device("d_shared")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("user resume should succeed");
    assert_eq!(user_resume.status(), StatusCode::OK);

    let agent_resume = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1139")
                .with_dual_token_actor_kind("agent")
                .with_dual_token_session("s_agent")
                .with_dual_token_device("d_shared")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("agent resume should return response");

    assert_eq!(agent_resume.status(), StatusCode::CONFLICT);
    let agent_resume_body = agent_resume
        .into_body()
        .collect()
        .await
        .expect("agent resume body should collect")
        .to_bytes();
    let agent_resume_json: serde_json::Value =
        serde_json::from_slice(&agent_resume_body).expect("agent resume should be valid json");
    assert_eq!(agent_resume_json["code"], "client_route_scope_conflict");
}

#[tokio::test]
async fn test_presence_heartbeat_rejects_same_route_id_with_different_principal_over_http() {
    let app = session_gateway::build_app();

    let first_resume = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1001")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_owner_a")
                .with_dual_token_device("d_shared_owner")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("first owner resume should succeed");
    assert_eq!(first_resume.status(), StatusCode::OK);

    let conflicting_resume = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1002")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_owner_b")
                .with_dual_token_device("d_shared_owner")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("conflicting owner resume should return response");

    assert_eq!(conflicting_resume.status(), StatusCode::CONFLICT);
    let body = conflicting_resume
        .into_body()
        .collect()
        .await
        .expect("conflicting owner resume body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("conflicting owner resume should be valid json");
    assert_eq!(json["code"], "client_route_scope_conflict");
}

#[tokio::test]
async fn test_session_gateway_rejects_sessionless_device_rebind_after_session_resume() {
    let app = session_gateway::build_app();

    let resume = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_demo")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("resume request should succeed");
    assert_eq!(resume.status(), StatusCode::OK);

    let heartbeat = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header(
                    "authorization",
                    format!(
                        "Bearer {}",
                        serde_json::json!({
                            "tenant_id": "100001",
                            "login_scope": "TENANT",
                            "user_id": "1",
                            "app_id": "sdkwork-im",
                            "auth_level": "password",
                            "subject_type": "user"
                        })
                    ),
                )
                .header(
                    "Access-Token",
                    serde_json::json!({
                        "tenant_id": "100001",
                        "login_scope": "TENANT",
                        "user_id": "1",
                        "app_id": "sdkwork-im",
                        "environment": "dev",
                        "deployment_mode": "saas",
                        "auth_level": "password",
                        "actor_id": "1",
                        "actor_kind": "user",
                        "device_id": "d_demo",
                        "data_scope": ["tenant"],
                        "permission_scope": ["*"],
                        "subject_type": "user"
                    })
                    .to_string(),
                )
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("sessionless heartbeat should return response");
    assert_eq!(heartbeat.status(), StatusCode::CONFLICT);
    let heartbeat_body = heartbeat
        .into_body()
        .collect()
        .await
        .expect("heartbeat body should collect")
        .to_bytes();
    let heartbeat_json: serde_json::Value =
        serde_json::from_slice(&heartbeat_body).expect("heartbeat should be valid json");
    assert_eq!(heartbeat_json["code"], "session_id_required");
}

#[tokio::test]
async fn test_realtime_subscription_sync_and_empty_event_window_over_http() {
    let app = session_gateway::build_app();

    let sync_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_pad")
                .with_dual_token_device("d_pad")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_demo",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync request should succeed");
    assert_eq!(sync_response.status(), StatusCode::OK);
    let sync_body = sync_response
        .into_body()
        .collect()
        .await
        .expect("subscription sync body should collect")
        .to_bytes();
    let sync_json: serde_json::Value =
        serde_json::from_slice(&sync_body).expect("subscription sync should be valid json");
    assert_eq!(sync_json["deviceId"], "d_pad");
    assert_eq!(sync_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(sync_json["items"][0]["scopeType"], "conversation");
    assert_eq!(sync_json["items"][0]["scopeId"], "c_demo");

    let event_window = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_pad")
                .with_dual_token_device("d_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime event window request should succeed");
    let event_window_status = event_window.status();
    let event_window_body = event_window
        .into_body()
        .collect()
        .await
        .expect("realtime event window body should collect")
        .to_bytes();
    assert_eq!(
        event_window_status,
        StatusCode::OK,
        "unexpected event window response: {}",
        String::from_utf8_lossy(&event_window_body)
    );
    let event_window_json: serde_json::Value =
        serde_json::from_slice(&event_window_body).expect("event window should be valid json");
    assert_eq!(event_window_json["deviceId"], "d_pad");
    assert_eq!(event_window_json["items"].as_array().unwrap().len(), 0);
    assert_eq!(event_window_json["ackedThroughSeq"], 0);
    assert_eq!(event_window_json["trimmedThroughSeq"], 0);
    assert_eq!(event_window_json["hasMore"], false);
}

#[tokio::test]
async fn test_realtime_subscription_sync_returns_403_when_scope_policy_denies_over_http() {
    let cluster = Arc::new(session_gateway::RealtimeClusterBridge::default());
    let app = session_gateway::build_app_with_cluster_and_runtime(
        cluster.clone(),
        Arc::new(session_gateway::RealtimeDeliveryRuntime::default()),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_demo")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_denied",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should return response");

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "realtime_scope_access_denied");
    assert!(
        cluster
            .resolve_client_route_for_principal_kind("100001", "default", "1", "user", "d_demo")
            .is_none(),
        "denied realtime subscription sync must not bind a client route"
    );
}

#[tokio::test]
async fn test_realtime_ack_endpoint_accepts_empty_window_over_http() {
    let app = session_gateway::build_app();

    let sync_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_pad")
                .with_dual_token_device("d_pad")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_demo",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync request should succeed");
    assert_eq!(sync_response.status(), StatusCode::OK);

    let ack_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/events/ack")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_pad")
                .with_dual_token_device("d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"ackedSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("ack request should succeed");
    assert_eq!(ack_response.status(), StatusCode::OK);
    let ack_body = ack_response
        .into_body()
        .collect()
        .await
        .expect("ack body should collect")
        .to_bytes();
    let ack_json: serde_json::Value =
        serde_json::from_slice(&ack_body).expect("ack response should be valid json");
    assert_eq!(ack_json["deviceId"], "d_pad");
    assert_eq!(ack_json["ackedThroughSeq"], 0);
    assert_eq!(ack_json["trimmedThroughSeq"], 0);
    assert_eq!(ack_json["retainedEventCount"], 0);
}

#[tokio::test]
async fn test_presence_heartbeat_rejects_oversized_client_route_id_over_http() {
    let app = session_gateway::build_app();
    let oversized_device_id = "d".repeat(1024);
    let request_body = serde_json::json!({
        "deviceId": oversized_device_id,
        "lastSeenSyncSeq": 0
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_demo")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("resume request should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "payload_too_large");
    assert!(
        json["message"]
            .as_str()
            .expect("message should be present")
            .contains("deviceId")
    );
}

#[tokio::test]
async fn test_realtime_subscription_sync_rejects_oversized_scope_id_over_http() {
    let cluster = Arc::new(session_gateway::RealtimeClusterBridge::default());
    let app = session_gateway::build_app_with_cluster(cluster.clone());
    let oversized_scope_id = "c".repeat(2048);
    let request_body = serde_json::json!({
        "items": [
            {
                "scopeType": "conversation",
                "scopeId": oversized_scope_id,
                "eventTypes": ["message.posted"]
            }
        ]
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_demo")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("subscription sync should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "payload_too_large");
    assert!(
        json["message"]
            .as_str()
            .expect("message should be present")
            .contains("scopeId")
    );
    assert!(
        cluster
            .resolve_client_route_for_principal_kind("100001", "default", "1", "user", "d_demo")
            .is_none(),
        "invalid realtime subscription sync must not bind a client route"
    );
}

#[tokio::test]
async fn test_realtime_event_window_rejects_limit_above_guardrail_over_http() {
    let app = session_gateway::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=5000")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_demo")
                .with_dual_token_device("d_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("event window request should return response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "limit_invalid");
}

#[tokio::test]
async fn test_realtime_event_window_rejects_zero_limit_without_binding_route_over_http() {
    let cluster = Arc::new(session_gateway::RealtimeClusterBridge::default());
    let app = session_gateway::build_app_with_cluster(cluster.clone());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=0")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_demo")
                .with_dual_token_device("d_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("event window request should return response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "limit_invalid");
    assert!(
        cluster
            .resolve_client_route_for_principal_kind("100001", "default", "1", "user", "d_demo")
            .is_none(),
        "invalid realtime event window request must not bind a client route"
    );
}

#[tokio::test]
async fn test_realtime_subscription_sync_rejects_oversized_event_types_payload_over_http() {
    let app = session_gateway::build_app();
    let oversized_event_types = (0..300)
        .map(|index| format!("evt_{index:03}_{}", "x".repeat(64)))
        .collect::<Vec<_>>();
    let request_body = serde_json::json!({
        "items": [
            {
                "scopeType": "conversation",
                "scopeId": "c_demo",
                "eventTypes": oversized_event_types
            }
        ]
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_demo")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("subscription sync should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "payload_too_large");
    assert!(
        json["message"]
            .as_str()
            .expect("message should be present")
            .contains("eventTypes")
    );
}

#[tokio::test]
async fn test_realtime_subscription_sync_rejects_too_many_items_over_http() {
    let app = session_gateway::build_app();
    let oversized_items = (0..300)
        .map(|index| {
            serde_json::json!({
                "scopeType": "conversation",
                "scopeId": format!("c_{index:03}"),
                "eventTypes": []
            })
        })
        .collect::<Vec<_>>();
    let request_body = serde_json::json!({
        "items": oversized_items
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_demo")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("subscription sync should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "payload_too_large");
    assert!(
        json["message"]
            .as_str()
            .expect("message should be present")
            .contains("items")
    );
}

#[tokio::test]
async fn test_realtime_subscription_sync_rejects_oversized_total_payload_over_http() {
    let app = session_gateway::build_app();
    let oversized_items = (0..40)
        .map(|index| {
            serde_json::json!({
                "scopeType": "conversation",
                "scopeId": format!("c_{index:03}_{}", "x".repeat(480)),
                "eventTypes": (0..120)
                    .map(|event_index| format!("evt_{event_index:02}_{}", "y".repeat(120)))
                    .collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>();
    let request_body = serde_json::json!({
        "items": oversized_items
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_demo")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("subscription sync should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "payload_too_large");
    assert!(
        json["message"]
            .as_str()
            .expect("message should be present")
            .contains("items")
    );
}
