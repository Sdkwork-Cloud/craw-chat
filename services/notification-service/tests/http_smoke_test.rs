use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_app_context::DualTokenRequestBuilderExt;
use tokio::time::{Duration, sleep};
use tower::ServiceExt;

#[tokio::test]
async fn test_public_app_exports_live_openapi_json() {
    let app = notification_service::build_public_app();

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
    assert_eq!(
        value["info"]["title"],
        "Sdkwork IM Notification Service API"
    );
    assert!(value["paths"]["/app/v3/api/notifications"].is_object());
}

#[tokio::test]
async fn test_public_app_serves_docs_page_for_live_openapi() {
    let app = notification_service::build_public_app();

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
    assert!(html.contains("Sdkwork IM Notification Service API"));
    assert!(html.contains("/openapi.json"));
}

#[tokio::test]
async fn test_request_and_query_notifications_over_http() {
    let app = notification_service::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/notifications/requests")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("notification.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_http_demo",
                        "sourceEventId":"evt_http_demo",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"1105",
        "recipientKind":"user",
                        "title":"New message",
                        "body":"hello",
                        "payload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("request notification should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("create body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(&create_body).expect("create body should be valid json");
    assert_eq!(create_json["notificationId"], "ntf_http_demo");
    assert_eq!(create_json["status"], "dispatched");

    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/notifications")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1105")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list notifications should succeed");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body = list_response
        .into_body()
        .collect()
        .await
        .expect("list body should collect")
        .to_bytes();
    let list_json: serde_json::Value =
        serde_json::from_slice(&list_body).expect("list body should be valid json");
    assert_eq!(list_json["items"][0]["notificationId"], "ntf_http_demo");

    let get_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/notifications/ntf_http_demo")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1105")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get notification should succeed");
    assert_eq!(get_response.status(), StatusCode::OK);
    let get_body = get_response
        .into_body()
        .collect()
        .await
        .expect("get body should collect")
        .to_bytes();
    let get_json: serde_json::Value =
        serde_json::from_slice(&get_body).expect("get body should be valid json");
    assert_eq!(get_json["sourceEventType"], "message.posted");
    assert_eq!(get_json["recipientId"], "1105");

    let non_recipient_list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/notifications")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("non-recipient list notifications should succeed");
    assert_eq!(non_recipient_list_response.status(), StatusCode::OK);
    let non_recipient_list_body = non_recipient_list_response
        .into_body()
        .collect()
        .await
        .expect("non-recipient list body should collect")
        .to_bytes();
    let non_recipient_list_json: serde_json::Value =
        serde_json::from_slice(&non_recipient_list_body)
            .expect("non-recipient list body should be valid json");
    assert_eq!(
        non_recipient_list_json["items"]
            .as_array()
            .expect("items should be an array")
            .len(),
        0
    );

    let non_recipient_get_response = app
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/notifications/ntf_http_demo")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("non-recipient get notification should succeed");
    assert_eq!(non_recipient_get_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_notification_queries_reject_same_actor_id_with_different_actor_kind_over_http() {
    let app = notification_service::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/notifications/requests")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1106")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("notification.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_http_actor_kind_isolation",
                        "sourceEventId":"evt_http_actor_kind_isolation",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"1",
        "recipientKind":"user",
                        "title":"New message",
                        "body":"hello",
                        "payload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("request notification should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let recipient_list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/notifications")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("recipient list notifications should succeed");
    assert_eq!(recipient_list_response.status(), StatusCode::OK);
    let recipient_list_body = recipient_list_response
        .into_body()
        .collect()
        .await
        .expect("recipient list body should collect")
        .to_bytes();
    let recipient_list_json: serde_json::Value =
        serde_json::from_slice(&recipient_list_body).expect("recipient list should be valid json");
    assert_eq!(
        recipient_list_json["items"][0]["notificationId"],
        "ntf_http_actor_kind_isolation"
    );

    let cross_kind_list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/notifications")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("system")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("cross-kind list notifications should succeed");
    assert_eq!(cross_kind_list_response.status(), StatusCode::OK);
    let cross_kind_list_body = cross_kind_list_response
        .into_body()
        .collect()
        .await
        .expect("cross-kind list body should collect")
        .to_bytes();
    let cross_kind_list_json: serde_json::Value = serde_json::from_slice(&cross_kind_list_body)
        .expect("cross-kind list should be valid json");
    assert_eq!(
        cross_kind_list_json["items"]
            .as_array()
            .expect("items should be an array")
            .len(),
        0,
        "a different actor_kind with the same actor_id must not share the inbox"
    );

    let cross_kind_get_response = app
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/notifications/ntf_http_actor_kind_isolation")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("system")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("cross-kind get notification should succeed");
    assert_eq!(cross_kind_get_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_duplicate_notification_id_is_idempotent_and_conflicting_retry_is_rejected_over_http()
{
    let app = notification_service::build_default_app();

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/notifications/requests")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("notification.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_http_idempotent",
                        "sourceEventId":"evt_http_idempotent",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"1",
        "recipientKind":"user",
                        "title":"New message",
                        "body":"hello",
                        "payload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first notification request should return response");
    assert_eq!(first_response.status(), StatusCode::OK);
    let first_body = first_response
        .into_body()
        .collect()
        .await
        .expect("first body should collect")
        .to_bytes();
    let first_json: serde_json::Value =
        serde_json::from_slice(&first_body).expect("first body should be valid json");
    assert_eq!(first_json["deliveryStatus"], "applied");
    assert_eq!(
        first_json["proofVersion"],
        "notification.request.delivery-proof.v1"
    );
    assert!(
        !first_json["requestKey"]
            .as_str()
            .expect("requestKey should be string")
            .is_empty()
    );

    let idempotent_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/notifications/requests")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("notification.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_http_idempotent",
                        "sourceEventId":"evt_http_idempotent",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"1",
        "recipientKind":"user",
                        "title":"New message",
                        "body":"hello",
                        "payload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("idempotent notification request should return response");
    assert_eq!(idempotent_response.status(), StatusCode::OK);
    let idempotent_body = idempotent_response
        .into_body()
        .collect()
        .await
        .expect("idempotent body should collect")
        .to_bytes();
    let idempotent_json: serde_json::Value =
        serde_json::from_slice(&idempotent_body).expect("idempotent body should be valid json");
    assert_eq!(idempotent_json["notificationId"], "ntf_http_idempotent");
    assert_eq!(idempotent_json["status"], "dispatched");
    assert_eq!(idempotent_json["deliveryStatus"], "replayed");
    assert_eq!(idempotent_json["requestKey"], first_json["requestKey"]);
    assert_eq!(idempotent_json["proofVersion"], first_json["proofVersion"]);

    let conflicting_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/notifications/requests")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("notification.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_http_idempotent",
                        "sourceEventId":"evt_http_conflict",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"1110",
        "recipientKind":"user",
                        "title":"Changed message",
                        "body":"different",
                        "payload":"{\"conversationId\":\"c_other\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting notification request should return response");
    assert_eq!(conflicting_response.status(), StatusCode::CONFLICT);
    let conflicting_body = conflicting_response
        .into_body()
        .collect()
        .await
        .expect("conflicting body should collect")
        .to_bytes();
    let conflicting_json: serde_json::Value =
        serde_json::from_slice(&conflicting_body).expect("conflicting body should be valid json");
    assert_eq!(conflicting_json["code"], "notification_conflict");
}

#[tokio::test]
async fn test_duplicate_notification_request_from_different_principal_keeps_stable_request_key_over_http()
 {
    let app = notification_service::build_default_app();

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/notifications/requests")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1109")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("notification.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_http_stable_request_key",
                        "sourceEventId":"evt_http_stable_request_key",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"1105",
        "recipientKind":"user",
                        "title":"New message",
                        "body":"hello",
                        "payload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first request should return response");
    assert_eq!(first_response.status(), StatusCode::OK);
    let first_body = first_response
        .into_body()
        .collect()
        .await
        .expect("first body should collect")
        .to_bytes();
    let first_json: serde_json::Value =
        serde_json::from_slice(&first_body).expect("first body should be valid json");
    assert_eq!(first_json["deliveryStatus"], "applied");

    let replayed_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/notifications/requests")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1107")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("notification.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_http_stable_request_key",
                        "sourceEventId":"evt_http_stable_request_key",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"1105",
        "recipientKind":"user",
                        "title":"New message",
                        "body":"hello",
                        "payload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("replayed request should return response");
    assert_eq!(replayed_response.status(), StatusCode::OK);
    let replayed_body = replayed_response
        .into_body()
        .collect()
        .await
        .expect("replayed body should collect")
        .to_bytes();
    let replayed_json: serde_json::Value =
        serde_json::from_slice(&replayed_body).expect("replayed body should be valid json");
    assert_eq!(replayed_json["deliveryStatus"], "replayed");
    assert_eq!(replayed_json["requestKey"], first_json["requestKey"]);
}

#[tokio::test]
async fn test_request_notification_rejects_oversized_payload_over_http() {
    let app = notification_service::build_default_app();

    let oversized_payload = "x".repeat(262145);
    let request_body = serde_json::json!({
        "notificationId":"ntf_http_oversized_payload",
        "sourceEventId":"evt_http_oversized_payload",
        "sourceEventType":"message.posted",
        "category":"message.new",
        "channel":"inapp",
        "recipientId":"1",
        "recipientKind":"user",
        "title":"New message",
        "body":"hello",
        "payload": oversized_payload
    })
    .to_string();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/notifications/requests")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized notification request should return response");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_request_notification_rejects_oversized_notification_id_over_http() {
    let app = notification_service::build_default_app();

    let oversized_notification_id = "n".repeat(513);
    let request_body = serde_json::json!({
        "notificationId": oversized_notification_id,
        "sourceEventId":"evt_http_oversized_id",
        "sourceEventType":"message.posted",
        "category":"message.new",
        "channel":"inapp",
        "recipientId":"1",
        "recipientKind":"user",
        "title":"New message",
        "body":"hello",
        "payload":"{\"conversationId\":\"c_demo\"}"
    })
    .to_string();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/notifications/requests")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized notification id request should return response");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_list_notifications_returns_newest_first_with_distinct_timestamps() {
    let app = notification_service::build_default_app();

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/notifications/requests")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_order_first",
                        "sourceEventId":"evt_order_first",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"1",
        "recipientKind":"user",
                        "title":"First message",
                        "body":"first",
                        "payload":"{\"conversationId\":\"c_first\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first notification request should succeed");
    assert_eq!(first_response.status(), StatusCode::OK);
    let first_body = first_response
        .into_body()
        .collect()
        .await
        .expect("first body should collect")
        .to_bytes();
    let first_json: serde_json::Value =
        serde_json::from_slice(&first_body).expect("first body should be valid json");

    sleep(Duration::from_millis(5)).await;

    let second_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/notifications/requests")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_order_second",
                        "sourceEventId":"evt_order_second",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"1",
        "recipientKind":"user",
                        "title":"Second message",
                        "body":"second",
                        "payload":"{\"conversationId\":\"c_second\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second notification request should succeed");
    assert_eq!(second_response.status(), StatusCode::OK);
    let second_body = second_response
        .into_body()
        .collect()
        .await
        .expect("second body should collect")
        .to_bytes();
    let second_json: serde_json::Value =
        serde_json::from_slice(&second_body).expect("second body should be valid json");

    assert_ne!(
        first_json["requestedAt"], second_json["requestedAt"],
        "separate notification requests must not reuse a fixed requestedAt timestamp"
    );
    assert_ne!(
        first_json["dispatchedAt"], second_json["dispatchedAt"],
        "separate notification requests must not reuse a fixed dispatchedAt timestamp"
    );

    let list_response = app
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/notifications")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list notifications should succeed");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body = list_response
        .into_body()
        .collect()
        .await
        .expect("list body should collect")
        .to_bytes();
    let list_json: serde_json::Value =
        serde_json::from_slice(&list_body).expect("list body should be valid json");
    let items = list_json["items"]
        .as_array()
        .expect("items should be an array");
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["notificationId"], "ntf_order_second");
    assert_eq!(items[1]["notificationId"], "ntf_order_first");
}
