use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_app_context::DualTokenRequestBuilderExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_public_app_exports_live_openapi_json() {
    let app = audit_service::build_public_app();

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
    assert_eq!(value["info"]["title"], "Sdkwork IM Audit Service API");
    assert!(value["paths"]["/backend/v3/api/audit/records"].is_object());
}

#[tokio::test]
async fn test_public_app_serves_docs_page_for_live_openapi() {
    let app = audit_service::build_public_app();

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
    assert!(html.contains("Sdkwork IM Audit Service API"));
    assert!(html.contains("/openapi.json"));
}

#[tokio::test]
async fn test_record_list_and_export_audit_over_http() {
    let app = sdkwork_routes_im_audit_backend_api::build_public_app();

    let record_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/audit/records")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("audit.write,audit.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "recordId":"audit_http_demo",
                        "aggregateType":"notification",
                        "aggregateId":"ntf_demo",
                        "action":"notification.requested",
                        "payload":"{\"recipientId\":\"1105\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("record audit should succeed");
    assert_eq!(record_response.status(), StatusCode::OK);

    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/audit/records")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("audit.write,audit.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list records should succeed");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body = list_response
        .into_body()
        .collect()
        .await
        .expect("list body should collect")
        .to_bytes();
    let list_json: serde_json::Value =
        serde_json::from_slice(&list_body).expect("list body should be valid json");
    assert_eq!(list_json["code"], 0);
    assert_eq!(list_json["data"]["items"][0]["recordId"], "audit_http_demo");
    assert_eq!(list_json["data"]["items"][0]["auditSeq"], 1);
    assert_eq!(list_json["data"]["nextAfterAuditSeq"], 1);
    assert_eq!(list_json["data"]["hasMore"], false);

    let export_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/audit/export")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("audit.write,audit.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("export should succeed");
    assert_eq!(export_response.status(), StatusCode::OK);
    let export_body = export_response
        .into_body()
        .collect()
        .await
        .expect("export body should collect")
        .to_bytes();
    let export_json: serde_json::Value =
        serde_json::from_slice(&export_body).expect("export body should be valid json");
    assert_eq!(export_json["code"], 0);
    assert_eq!(export_json["data"]["total"], 1);
    assert_eq!(
        export_json["data"]["items"][0]["action"],
        "notification.requested"
    );

    let verify_response = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/audit/verify")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("audit.write,audit.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("verify should succeed");
    assert_eq!(verify_response.status(), StatusCode::OK);
    let verify_body = verify_response
        .into_body()
        .collect()
        .await
        .expect("verify body should collect")
        .to_bytes();
    let verify_json: serde_json::Value =
        serde_json::from_slice(&verify_body).expect("verify body should be valid json");
    assert_eq!(verify_json["code"], 0);
    assert_eq!(verify_json["data"]["tenantId"], "100001");
    assert_eq!(verify_json["data"]["total"], 1);
    assert_eq!(verify_json["data"]["chainValid"], true);
    assert!(
        verify_json["data"]["chainHeadHash"]
            .as_str()
            .is_some(),
        "verify response should include chain head hash when records exist"
    );
}

#[tokio::test]
async fn test_record_list_returns_bounded_audit_seq_cursor_window_over_http() {
    let app = sdkwork_routes_im_audit_backend_api::build_public_app();

    for (record_id, action) in [
        ("audit_http_window_first", "notification.requested"),
        ("audit_http_window_second", "notification.dispatched"),
        ("audit_http_window_third", "notification.delivered"),
    ] {
        let record_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/backend/v3/api/audit/records")
                    .with_dual_token_tenant("100001")
                    .with_dual_token_organization("100001")
                    .with_dual_token_user("1")
                    .with_dual_token_actor_kind("user")
                    .with_dual_token_permission_scope("audit.write,audit.read")
                    .header("content-type", "application/json")
                    .body(Body::from(format!(
                        r#"{{
                            "recordId":"{record_id}",
                            "aggregateType":"notification",
                            "aggregateId":"ntf_window",
                            "action":"{action}",
                            "payload":"{{\"step\":\"{action}\"}}"
                        }}"#
                    )))
                    .unwrap(),
            )
            .await
            .expect("record audit should succeed");
        assert_eq!(record_response.status(), StatusCode::OK);
    }

    let first_window_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/audit/records?afterAuditSeq=0&limit=2")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("audit.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("first audit window should succeed");
    assert_eq!(first_window_response.status(), StatusCode::OK);
    let first_window_body = first_window_response
        .into_body()
        .collect()
        .await
        .expect("first audit window body should collect")
        .to_bytes();
    let first_window_json: serde_json::Value =
        serde_json::from_slice(&first_window_body).expect("first audit window should be json");
    assert_eq!(
        first_window_json["data"]["items"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(first_window_json["data"]["items"][0]["auditSeq"], 1);
    assert_eq!(first_window_json["data"]["items"][1]["auditSeq"], 2);
    assert_eq!(first_window_json["data"]["nextAfterAuditSeq"], 2);
    assert_eq!(first_window_json["data"]["hasMore"], true);

    let second_window_response = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/audit/records?afterAuditSeq=2&limit=2")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("audit.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second audit window should succeed");
    assert_eq!(second_window_response.status(), StatusCode::OK);
    let second_window_body = second_window_response
        .into_body()
        .collect()
        .await
        .expect("second audit window body should collect")
        .to_bytes();
    let second_window_json: serde_json::Value =
        serde_json::from_slice(&second_window_body).expect("second audit window should be json");
    assert_eq!(
        second_window_json["data"]["items"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(second_window_json["data"]["items"][0]["auditSeq"], 3);
    assert_eq!(
        second_window_json["data"]["items"][0]["action"],
        "notification.delivered"
    );
    assert_eq!(second_window_json["data"]["nextAfterAuditSeq"], 3);
    assert_eq!(second_window_json["data"]["hasMore"], false);
}

#[tokio::test]
async fn test_duplicate_record_anchor_request_is_idempotent_and_conflicting_retry_is_rejected() {
    let app = sdkwork_routes_im_audit_backend_api::build_public_app();

    let first_record = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/audit/records")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("audit.write,audit.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "recordId":"audit_http_idempotent",
                        "aggregateType":"notification",
                        "aggregateId":"ntf_http_idempotent",
                        "action":"notification.requested",
                        "payload":"{\"recipientId\":\"1105\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first audit record should succeed");
    assert_eq!(first_record.status(), StatusCode::OK);
    let first_record_body = first_record
        .into_body()
        .collect()
        .await
        .expect("first record body should collect")
        .to_bytes();
    let first_record_json: serde_json::Value =
        serde_json::from_slice(&first_record_body).expect("first record should be valid json");
    assert_eq!(first_record_json["data"]["deliveryStatus"], "applied");
    assert_eq!(
        first_record_json["data"]["proofVersion"],
        "audit.record.delivery-proof.v1"
    );

    let duplicate_record = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/audit/records")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("audit.write,audit.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "recordId":"audit_http_idempotent",
                        "aggregateType":"notification",
                        "aggregateId":"ntf_http_idempotent",
                        "action":"notification.requested",
                        "payload":"{\"recipientId\":\"1105\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate audit record should return response");
    assert_eq!(duplicate_record.status(), StatusCode::OK);
    let duplicate_record_body = duplicate_record
        .into_body()
        .collect()
        .await
        .expect("duplicate record body should collect")
        .to_bytes();
    let duplicate_record_json: serde_json::Value = serde_json::from_slice(&duplicate_record_body)
        .expect("duplicate record should be valid json");
    assert_eq!(duplicate_record_json["data"]["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_record_json["data"]["requestKey"],
        first_record_json["data"]["requestKey"]
    );

    let list_records = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/audit/records")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("audit.write,audit.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list records should succeed");
    assert_eq!(list_records.status(), StatusCode::OK);
    let list_records_body = list_records
        .into_body()
        .collect()
        .await
        .expect("list records body should collect")
        .to_bytes();
    let list_records_json: serde_json::Value =
        serde_json::from_slice(&list_records_body).expect("list records should be valid json");
    assert_eq!(
        list_records_json["data"]["items"]
            .as_array()
            .unwrap()
            .len(),
        1
    );

    let conflicting_record = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/audit/records")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("audit.write,audit.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "recordId":"audit_http_idempotent",
                        "aggregateType":"notification",
                        "aggregateId":"ntf_http_idempotent",
                        "action":"notification.delivered",
                        "payload":"{\"recipientId\":\"1105\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting audit record should return response");
    assert_eq!(conflicting_record.status(), StatusCode::CONFLICT);
    let conflicting_record_body = conflicting_record
        .into_body()
        .collect()
        .await
        .expect("conflicting record body should collect")
        .to_bytes();
    let conflicting_record_json: serde_json::Value =
        serde_json::from_slice(&conflicting_record_body)
            .expect("conflicting record should be valid json");
    assert_eq!(conflicting_record_json["code"].as_i64(), Some(40901));
}

#[tokio::test]
async fn test_duplicate_record_anchor_request_replays_after_session_rotation() {
    let app = sdkwork_routes_im_audit_backend_api::build_public_app();

    let first_record = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/audit/records")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_before")
                .with_dual_token_permission_scope("audit.write,audit.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "recordId":"audit_http_session_rotation",
                        "aggregateType":"notification",
                        "aggregateId":"ntf_http_session_rotation",
                        "action":"notification.requested",
                        "payload":"{\"recipientId\":\"1105\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first audit record should succeed");
    assert_eq!(first_record.status(), StatusCode::OK);
    let first_record_body = first_record
        .into_body()
        .collect()
        .await
        .expect("first record body should collect")
        .to_bytes();
    let first_record_json: serde_json::Value =
        serde_json::from_slice(&first_record_body).expect("first record should be valid json");
    assert_eq!(first_record_json["data"]["deliveryStatus"], "applied");

    let duplicate_record = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/audit/records")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_after")
                .with_dual_token_permission_scope("audit.write,audit.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "recordId":"audit_http_session_rotation",
                        "aggregateType":"notification",
                        "aggregateId":"ntf_http_session_rotation",
                        "action":"notification.requested",
                        "payload":"{\"recipientId\":\"1105\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate audit record should return response after session rotation");
    assert_eq!(duplicate_record.status(), StatusCode::OK);
    let duplicate_record_body = duplicate_record
        .into_body()
        .collect()
        .await
        .expect("duplicate record body should collect")
        .to_bytes();
    let duplicate_record_json: serde_json::Value = serde_json::from_slice(&duplicate_record_body)
        .expect("duplicate record should be valid json");
    assert_eq!(duplicate_record_json["data"]["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_record_json["data"]["requestKey"],
        first_record_json["data"]["requestKey"]
    );

    let list_records = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/audit/records")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("audit.write,audit.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list records should succeed");
    assert_eq!(list_records.status(), StatusCode::OK);
    let list_records_body = list_records
        .into_body()
        .collect()
        .await
        .expect("list records body should collect")
        .to_bytes();
    let list_records_json: serde_json::Value =
        serde_json::from_slice(&list_records_body).expect("list records should be valid json");
    assert_eq!(
        list_records_json["data"]["items"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
}

#[tokio::test]
async fn test_record_audit_rejects_oversized_payload_over_http() {
    let app = sdkwork_routes_im_audit_backend_api::build_public_app();
    let request_body = serde_json::json!({
        "recordId": "audit_http_oversized_payload",
        "aggregateType": "notification",
        "aggregateId": "ntf_http_oversized_payload",
        "action": "notification.requested",
        "payload": "x".repeat(200_000)
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/audit/records")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("audit.write,audit.read")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized audit payload should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"].as_i64(), Some(41301));
    assert!(
        value["detail"]
            .as_str()
            .expect("detail should be present")
            .contains("payload")
    );
}
