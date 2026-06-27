use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_app_context::DualTokenRequestBuilderExt;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tower::ServiceExt;

static UNIQUE_CATALOG_COUNTER: AtomicU64 = AtomicU64::new(0);

struct ScopedEnvVar {
    name: &'static str,
    previous: Option<String>,
}

impl ScopedEnvVar {
    fn set(name: &'static str, value: &str) -> Self {
        let previous = std::env::var(name).ok();
        unsafe {
            std::env::set_var(name, value);
        }
        Self { name, previous }
    }
}

impl Drop for ScopedEnvVar {
    fn drop(&mut self) {
        if let Some(previous) = &self.previous {
            unsafe {
                std::env::set_var(self.name, previous);
            }
            return;
        }
        unsafe {
            std::env::remove_var(self.name);
        }
    }
}

fn unique_principal_catalog_path() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    let counter = UNIQUE_CATALOG_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "conversation_runtime_principal_catalog_{unique}_{counter}.json"
    ))
}

#[derive(Clone)]
struct StrictKnownPrincipalDirectory {
    known_user_ids: Arc<Vec<&'static str>>,
}

impl StrictKnownPrincipalDirectory {
    fn new(known_user_ids: &[&'static str]) -> Self {
        Self {
            known_user_ids: Arc::new(known_user_ids.to_vec()),
        }
    }
}

impl conversation_runtime::PrincipalDirectory for StrictKnownPrincipalDirectory {
    fn ensure_active_principal(
        &self,
        _tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Result<(), conversation_runtime::PrincipalDirectoryError> {
        if principal_kind != "user" {
            return Ok(());
        }
        if self.known_user_ids.contains(&principal_id) {
            return Ok(());
        }

        Err(
            conversation_runtime::PrincipalDirectoryError::PrincipalNotFound {
                tenant_id: "100001".into(),
                principal_id: principal_id.into(),
                principal_kind: principal_kind.into(),
            },
        )
    }
}

#[tokio::test]
async fn test_public_app_exports_live_openapi_json() {
    let app = conversation_runtime::build_public_app();

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
        "Sdkwork IM Conversation Runtime API"
    );
    assert!(value["paths"]["/im/v3/api/chat/conversations/{conversation_id}/messages"].is_object());
}

#[tokio::test]
async fn test_public_app_serves_docs_page_for_live_openapi() {
    let app = conversation_runtime::build_public_app();

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
    assert!(html.contains("Sdkwork IM Conversation Runtime API"));
    assert!(html.contains("/openapi.json"));
}

#[tokio::test]
async fn test_public_app_rejects_missing_access_token_header_over_http() {
    let app = conversation_runtime::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("authorization", "Bearer auth_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_missing_access_token",
                        "conversationType":"group"
                    }"#,
                ))
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
async fn test_create_conversation_and_post_message_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");

    assert_eq!(create_response.status(), StatusCode::OK);

    let post_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_http",
                        "summary":"hello",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message request should succeed");

    assert_eq!(post_response.status(), StatusCode::OK);
    let body = post_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");

    assert_eq!(value["messageSeq"], 1);
    assert_eq!(value["messageId"], "msg_c_http_1");
}

#[tokio::test]
async fn test_post_media_message_rejects_missing_drive_reference_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_media_missing_drive_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_media_missing_drive_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_media_missing_drive_http",
                        "summary":"missing drive",
                        "parts":[
                            {
                                "kind":"media",
                                "resource":{
                                    "id":"node_missing_drive",
                                    "kind":"image",
                                    "source":"drive",
                                    "uri":"drive://spaces/space_app_upload_demo/nodes/node_missing_drive"
                                }
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post missing Drive media request should return response");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "invalid_json");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("drive")
    );
}

#[tokio::test]
async fn test_post_media_message_rejects_noncanonical_drive_reference_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_media_bad_drive_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_media_bad_drive_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_media_bad_drive_http",
                        "summary":"bad drive",
                        "parts":[
                            {
                                "kind":"media",
                                "drive":{
                                    "driveUri":"drive://spaces/space_app_upload_demo/nodes/node_other",
                                    "spaceId":"space_app_upload_demo",
                                    "nodeId":"node_bad_drive"
                                },
                                "resource":{
                                    "id":"node_bad_drive",
                                    "kind":"image",
                                    "source":"drive",
                                    "uri":"drive://spaces/space_app_upload_demo/nodes/node_bad_drive"
                                }
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post noncanonical Drive media request should return response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_request_invalid");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("drive.driveUri")
    );
}

#[tokio::test]
async fn test_post_media_message_rejects_external_url_source_with_drive_reference_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_media_external_url_source_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_media_external_url_source_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_media_external_url_source_http",
                        "summary":"external url source",
                        "parts":[
                            {
                                "kind":"media",
                                "drive":{
                                    "driveUri":"drive://spaces/space_app_upload_demo/nodes/node_external_url_source",
                                    "spaceId":"space_app_upload_demo",
                                    "nodeId":"node_external_url_source"
                                },
                                "resource":{
                                    "id":"node_external_url_source",
                                    "kind":"image",
                                    "source":"external_url",
                                    "uri":"drive://spaces/space_app_upload_demo/nodes/node_external_url_source",
                                    "url":"https://example.com/not-drive-owned.png"
                                }
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post external URL media request should return response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_request_invalid");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("resource.source")
    );
}

#[tokio::test]
async fn test_duplicate_create_conversation_request_is_idempotent_and_conflicting_retry_is_rejected_over_http()
 {
    let app = conversation_runtime::build_default_app();

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_create_retry_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first create should return response");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value =
        serde_json::from_slice(&first_create_body).expect("first create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert_eq!(
        first_create_json["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_create_json["requestKey"],
        "6#1000014#user1#1#19#create-conversation19#c_create_retry_http"
    );

    let duplicate_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_create_retry_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::OK);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate create should be valid json");
    assert_eq!(duplicate_create_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        duplicate_create_json["eventId"],
        first_create_json["eventId"]
    );

    let conflicting_retry = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_create_retry_http",
                        "conversationType":"direct"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting create should return response");
    assert_eq!(conflicting_retry.status(), StatusCode::CONFLICT);
    let conflicting_retry_body = conflicting_retry
        .into_body()
        .collect()
        .await
        .expect("conflicting create body should collect")
        .to_bytes();
    let conflicting_retry_json: serde_json::Value = serde_json::from_slice(&conflicting_retry_body)
        .expect("conflicting create should be valid json");
    assert_eq!(conflicting_retry_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_duplicate_post_message_request_is_idempotent_and_conflicting_retry_is_rejected_over_http()
 {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_http_post_retry",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let first_post = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_http_post_retry/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_http_post_retry",
                        "summary":"hello",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first post should succeed");
    assert_eq!(first_post.status(), StatusCode::OK);
    let first_post_body = first_post
        .into_body()
        .collect()
        .await
        .expect("first post body should collect")
        .to_bytes();
    let first_post_json: serde_json::Value =
        serde_json::from_slice(&first_post_body).expect("first post should be valid json");
    assert_eq!(first_post_json["deliveryStatus"], "applied");
    assert_eq!(
        first_post_json["proofVersion"],
        "conversation.message.delivery-proof.v1"
    );

    let duplicate_post = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_http_post_retry/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_http_post_retry",
                        "summary":"hello",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate post should return response");
    assert_eq!(duplicate_post.status(), StatusCode::OK);
    let duplicate_post_body = duplicate_post
        .into_body()
        .collect()
        .await
        .expect("duplicate post body should collect")
        .to_bytes();
    let duplicate_post_json: serde_json::Value =
        serde_json::from_slice(&duplicate_post_body).expect("duplicate post should be valid json");
    assert_eq!(duplicate_post_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_post_json["requestKey"],
        first_post_json["requestKey"]
    );
    assert_eq!(
        duplicate_post_json["messageId"],
        first_post_json["messageId"]
    );
    assert_eq!(
        duplicate_post_json["messageSeq"],
        first_post_json["messageSeq"]
    );
    assert_eq!(duplicate_post_json["eventId"], first_post_json["eventId"]);

    let history = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_http_post_retry/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("history request should succeed");
    assert_eq!(history.status(), StatusCode::OK);
    let history_body = history
        .into_body()
        .collect()
        .await
        .expect("history body should collect")
        .to_bytes();
    let history_json: serde_json::Value =
        serde_json::from_slice(&history_body).expect("history should be valid json");
    assert_eq!(history_json["highWatermark"], 1);
    assert_eq!(history_json["items"].as_array().unwrap().len(), 1);

    let conflicting_retry = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_http_post_retry/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_http_post_retry",
                        "summary":"hello conflict",
                        "text":"hello conflict"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting retry should return response");
    assert_eq!(conflicting_retry.status(), StatusCode::CONFLICT);
    let conflicting_retry_body = conflicting_retry
        .into_body()
        .collect()
        .await
        .expect("conflicting retry body should collect")
        .to_bytes();
    let conflicting_retry_json: serde_json::Value = serde_json::from_slice(&conflicting_retry_body)
        .expect("conflicting retry should be valid json");
    assert_eq!(conflicting_retry_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_list_messages_http_returns_bounded_cursor_window() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_history_page_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    for seq in 1..=2 {
        let post_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/chat/conversations/c_history_page_http/messages")
                    .with_dual_token_tenant("100001")
                    .with_dual_token_user("1")
                    .with_dual_token_actor_kind("user")
                    .header("content-type", "application/json")
                    .body(Body::from(format!(
                        r#"{{
                            "clientMsgId":"client_history_page_{seq}",
                            "summary":"message {seq}",
                            "text":"message {seq}"
                        }}"#
                    )))
                    .unwrap(),
            )
            .await
            .expect("post message should succeed");
        assert_eq!(post_response.status(), StatusCode::OK);
    }

    let first_page = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(
                    "/im/v3/api/chat/conversations/c_history_page_http/messages?afterSeq=0&limit=1",
                )
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("first page request should complete");
    assert_eq!(first_page.status(), StatusCode::OK);
    let first_body = first_page
        .into_body()
        .collect()
        .await
        .expect("first page body should collect")
        .to_bytes();
    let first_json: serde_json::Value =
        serde_json::from_slice(&first_body).expect("first page should be valid json");
    assert_eq!(first_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(first_json["items"][0]["message"]["messageSeq"], 1);
    assert_eq!(first_json["highWatermark"], 2);
    assert_eq!(first_json["nextAfterSeq"], 1);
    assert_eq!(first_json["hasMore"], true);

    let second_page = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(
                    "/im/v3/api/chat/conversations/c_history_page_http/messages?afterSeq=1&limit=1",
                )
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second page request should complete");
    assert_eq!(second_page.status(), StatusCode::OK);
    let second_body = second_page
        .into_body()
        .collect()
        .await
        .expect("second page body should collect")
        .to_bytes();
    let second_json: serde_json::Value =
        serde_json::from_slice(&second_body).expect("second page should be valid json");
    assert_eq!(second_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(second_json["items"][0]["message"]["messageSeq"], 2);
    assert_eq!(second_json["highWatermark"], 2);
    assert_eq!(second_json["nextAfterSeq"], 2);
    assert_eq!(second_json["hasMore"], false);

    let invalid_limit = app
        .oneshot(
            Request::builder()
                .uri(
                    "/im/v3/api/chat/conversations/c_history_page_http/messages?afterSeq=0&limit=0",
                )
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("invalid limit request should complete");
    assert_eq!(invalid_limit.status(), StatusCode::BAD_REQUEST);
    let invalid_body = invalid_limit
        .into_body()
        .collect()
        .await
        .expect("invalid limit body should collect")
        .to_bytes();
    let invalid_json: serde_json::Value =
        serde_json::from_slice(&invalid_body).expect("invalid limit should be valid json");
    assert_eq!(invalid_json["code"], "limit_invalid");
}

#[tokio::test]
async fn test_create_conversation_rejects_unknown_type_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_invalid_type_http",
                        "conversationType":"workspace"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create invalid conversation should return response");

    assert_eq!(create_response.status(), StatusCode::BAD_REQUEST);
    let body = create_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_type_invalid");
}

#[tokio::test]
async fn test_create_conversation_rejects_unknown_user_creator_over_http() {
    let app = conversation_runtime::build_default_app_with_principal_directory(Arc::new(
        StrictKnownPrincipalDirectory::new(&["actor_a"]),
    ));

    let create_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("actor_missing")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_unknown_creator_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation with unknown creator should return response");

    assert_eq!(create_response.status(), StatusCode::BAD_REQUEST);
    let body = create_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_principal_not_found");
}

#[tokio::test]
async fn test_create_conversation_rejects_oversized_conversation_id_over_http() {
    let app = conversation_runtime::build_default_app();
    let request_body = serde_json::json!({
        "conversationId": "c".repeat(2048),
        "conversationType": "group"
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized create conversation should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
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
async fn test_generic_create_rejects_reserved_special_types_over_http() {
    let app = conversation_runtime::build_default_app();

    for (conversation_id, conversation_type) in [
        ("c_agent_dialog_http", "agent_dialog"),
        ("c_agent_handoff_http", "agent_handoff"),
        ("c_system_channel_http", "system_channel"),
    ] {
        let create_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/chat/conversations")
                    .with_dual_token_tenant("100001")
                    .with_dual_token_user("svc_ops")
                    .with_dual_token_actor_kind("system")
                    .header("content-type", "application/json")
                    .body(Body::from(format!(
                        r#"{{
                            "conversationId":"{conversation_id}",
                            "conversationType":"{conversation_type}"
                        }}"#
                    )))
                    .unwrap(),
            )
            .await
            .expect("reserved special create should return response");

        assert_eq!(
            create_response.status(),
            StatusCode::BAD_REQUEST,
            "reserved type should be rejected: {conversation_type}"
        );
        let body = create_response
            .into_body()
            .collect()
            .await
            .expect("body should collect")
            .to_bytes();
        let value: serde_json::Value =
            serde_json::from_slice(&body).expect("response should be valid json");
        assert_eq!(value["code"], "conversation_type_invalid");
    }
}

#[tokio::test]
async fn test_group_create_preserves_actor_kind_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_ops")
                .with_dual_token_actor_kind("system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_group_actor_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create group request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let list_members = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_group_actor_http/members")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_ops")
                .with_dual_token_actor_kind("system")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list members request should succeed");
    assert_eq!(list_members.status(), StatusCode::OK);
    let body = list_members
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["items"][0]["principalId"], "svc_ops");
    assert_eq!(value["items"][0]["principalKind"], "system");
}

#[tokio::test]
async fn test_create_agent_dialog_rejects_non_standard_agent_id_over_http() {
    let app = conversation_runtime::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_dialogs")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_dialog_invalid_agent_id_http",
                        "agentId":"ag_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("invalid agent dialog request should return response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("invalid agent dialog body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "agent_id_invalid");
}

#[tokio::test]
async fn test_create_agent_dialog_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_dialogs")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_dialog_http",
                        "agentId":"agent.demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create agent dialog request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let list_members = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_agent_dialog_http/members")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list members request should succeed");
    assert_eq!(list_members.status(), StatusCode::OK);
    let body = list_members
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["items"].as_array().unwrap().len(), 2);
    assert!(
        value["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["principalId"] == "1" && item["principalKind"] == "user")
    );
    assert!(
        value["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["principalId"] == "agent.demo" && item["principalKind"] == "agent")
    );
}

#[tokio::test]
async fn test_duplicate_create_agent_dialog_request_is_idempotent_and_conflicting_retry_is_rejected_over_http()
 {
    let app = conversation_runtime::build_default_app();

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_dialogs")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_dialog_retry_http",
                        "agentId":"agent.demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first agent dialog create should return response");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first agent dialog create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value = serde_json::from_slice(&first_create_body)
        .expect("first agent dialog create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert_eq!(
        first_create_json["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_create_json["requestKey"],
        "6#1000014#user1#1#19#create-agent-dialog25#c_agent_dialog_retry_http"
    );

    let duplicate_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_dialogs")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_dialog_retry_http",
                        "agentId":"agent.demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate agent dialog create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::OK);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate agent dialog create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate agent dialog create should be valid json");
    assert_eq!(duplicate_create_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        duplicate_create_json["eventId"],
        first_create_json["eventId"]
    );

    let conflicting_retry = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_dialogs")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_dialog_retry_http",
                        "agentId":"agent.other"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting agent dialog create should return response");
    assert_eq!(conflicting_retry.status(), StatusCode::CONFLICT);
    let conflicting_retry_body = conflicting_retry
        .into_body()
        .collect()
        .await
        .expect("conflicting agent dialog create body should collect")
        .to_bytes();
    let conflicting_retry_json: serde_json::Value = serde_json::from_slice(&conflicting_retry_body)
        .expect("conflicting agent dialog create should be valid json");
    assert_eq!(conflicting_retry_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_create_agent_dialog_rejects_non_user_actor_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_dialogs")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_ops")
                .with_dual_token_actor_kind("system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_dialog_system_http",
                        "agentId":"agent.demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create invalid agent dialog should return response");

    assert_eq!(create_response.status(), StatusCode::FORBIDDEN);
    let body = create_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_create_agent_dialog_rejects_unknown_user_requester_over_http() {
    let app = conversation_runtime::build_default_app_with_principal_directory(Arc::new(
        StrictKnownPrincipalDirectory::new(&["actor_a"]),
    ));

    let create_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_dialogs")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("actor_missing")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_dialog_unknown_requester_http",
                        "agentId":"agent.demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create agent dialog with unknown requester should return response");

    assert_eq!(create_response.status(), StatusCode::BAD_REQUEST);
    let body = create_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_principal_not_found");
}

#[tokio::test]
async fn test_create_agent_handoff_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("ag_source")
                .with_dual_token_actor_kind("agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_http",
                        "targetId":"1",
                        "targetKind":"user",
                        "handoffSessionId":"hs_http",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create agent handoff request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let list_members = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_http/members")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("ag_source")
                .with_dual_token_actor_kind("agent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list members request should succeed");
    assert_eq!(list_members.status(), StatusCode::OK);
    let body = list_members
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["items"].as_array().unwrap().len(), 2);
    assert!(
        value["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["principalId"] == "ag_source" && item["principalKind"] == "agent")
    );
    assert!(
        value["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["principalId"] == "1" && item["principalKind"] == "user")
    );
}

#[tokio::test]
async fn test_create_agent_handoff_rejects_non_agent_actor_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_invalid_http",
                        "targetId":"1",
                        "targetKind":"user",
                        "handoffSessionId":"hs_invalid_http",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create invalid agent handoff should return response");

    assert_eq!(create_response.status(), StatusCode::FORBIDDEN);
    let body = create_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_create_agent_handoff_rejects_unknown_user_target_over_http() {
    let app = conversation_runtime::build_default_app_with_principal_directory(Arc::new(
        StrictKnownPrincipalDirectory::new(&["actor_a"]),
    ));

    let create_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("ag_source")
                .with_dual_token_actor_kind("agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_unknown_target_http",
                        "targetId":"actor_missing",
                        "targetKind":"user",
                        "handoffSessionId":"hs_unknown_target_http",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create agent handoff with unknown target should return response");

    assert_eq!(create_response.status(), StatusCode::BAD_REQUEST);
    let body = create_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_principal_not_found");
}

#[tokio::test]
async fn test_duplicate_create_agent_handoff_request_is_idempotent_and_conflicting_retry_is_rejected_over_http()
 {
    let app = conversation_runtime::build_default_app();

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("ag_source")
                .with_dual_token_actor_kind("agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_retry_http",
                        "targetId":"1",
                        "targetKind":"user",
                        "handoffSessionId":"hs_retry_http",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first agent handoff create should return response");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first agent handoff create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value = serde_json::from_slice(&first_create_body)
        .expect("first agent handoff create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert_eq!(
        first_create_json["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_create_json["requestKey"],
        "6#1000015#agent9#ag_source20#create-agent_handoff26#c_agent_handoff_retry_http"
    );

    let duplicate_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("ag_source")
                .with_dual_token_actor_kind("agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_retry_http",
                        "targetId":"1",
                        "targetKind":"user",
                        "handoffSessionId":"hs_retry_http",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate agent handoff create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::OK);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate agent handoff create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate agent handoff create should be valid json");
    assert_eq!(duplicate_create_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        duplicate_create_json["eventId"],
        first_create_json["eventId"]
    );

    let conflicting_retry = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("ag_source")
                .with_dual_token_actor_kind("agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_retry_http",
                        "targetId":"1041",
                        "targetKind":"user",
                        "handoffSessionId":"hs_retry_http",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting agent handoff create should return response");
    assert_eq!(conflicting_retry.status(), StatusCode::CONFLICT);
    let conflicting_retry_body = conflicting_retry
        .into_body()
        .collect()
        .await
        .expect("conflicting agent handoff create body should collect")
        .to_bytes();
    let conflicting_retry_json: serde_json::Value = serde_json::from_slice(&conflicting_retry_body)
        .expect("conflicting agent handoff create should be valid json");
    assert_eq!(conflicting_retry_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_agent_handoff_target_can_post_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("ag_source")
                .with_dual_token_actor_kind("agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_post_http",
                        "targetId":"1",
                        "targetKind":"user",
                        "handoffSessionId":"hs_post_http",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create agent handoff request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let post_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_post_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_handoff_target_post",
                        "text":"accepted"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("target post request should return response");

    assert_eq!(post_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_agent_handoff_accept_resolve_close_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("ag_source")
                .with_dual_token_actor_kind("agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_lifecycle_http",
                        "targetId":"1",
                        "targetKind":"user",
                        "handoffSessionId":"hs_http",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create agent handoff request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let get_open = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_lifecycle_http/agent_handoff")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("ag_source")
                .with_dual_token_actor_kind("agent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get handoff state request should succeed");
    assert_eq!(get_open.status(), StatusCode::OK);
    let get_open_body = get_open
        .into_body()
        .collect()
        .await
        .expect("open state body should collect")
        .to_bytes();
    let get_open_json: serde_json::Value =
        serde_json::from_slice(&get_open_body).expect("open state should be valid json");
    assert_eq!(get_open_json["status"], "open");

    let accept_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_lifecycle_http/agent_handoff/accept")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("accept request should succeed");
    assert_eq!(accept_response.status(), StatusCode::OK);
    let accept_body = accept_response
        .into_body()
        .collect()
        .await
        .expect("accept body should collect")
        .to_bytes();
    let accept_json: serde_json::Value =
        serde_json::from_slice(&accept_body).expect("accept response should be valid json");
    assert_eq!(accept_json["status"], "accepted");
    assert_eq!(accept_json["acceptedBy"]["id"], "1");

    let resolve_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_lifecycle_http/agent_handoff/resolve")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("resolve request should succeed");
    assert_eq!(resolve_response.status(), StatusCode::OK);
    let resolve_body = resolve_response
        .into_body()
        .collect()
        .await
        .expect("resolve body should collect")
        .to_bytes();
    let resolve_json: serde_json::Value =
        serde_json::from_slice(&resolve_body).expect("resolve response should be valid json");
    assert_eq!(resolve_json["status"], "resolved");

    let close_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_lifecycle_http/agent_handoff/close")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("ag_source")
                .with_dual_token_actor_kind("agent")
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("close request should succeed");
    assert_eq!(close_response.status(), StatusCode::OK);
    let close_body = close_response
        .into_body()
        .collect()
        .await
        .expect("close body should collect")
        .to_bytes();
    let close_json: serde_json::Value =
        serde_json::from_slice(&close_body).expect("close response should be valid json");
    assert_eq!(close_json["status"], "closed");

    let post_after_close = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_lifecycle_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_handoff_closed_http",
                        "summary":"should fail",
                        "text":"should fail"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("closed post request should return response");
    assert_eq!(post_after_close.status(), StatusCode::CONFLICT);
    let post_after_close_body = post_after_close
        .into_body()
        .collect()
        .await
        .expect("closed post body should collect")
        .to_bytes();
    let post_after_close_json: serde_json::Value = serde_json::from_slice(&post_after_close_body)
        .expect("closed post response should be valid json");
    assert_eq!(post_after_close_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_agent_handoff_accept_rejects_non_target_actor_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("ag_source")
                .with_dual_token_actor_kind("agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_accept_invalid_http",
                        "targetId":"1",
                        "targetKind":"user",
                        "handoffSessionId":"hs_invalid_http",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create agent handoff request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let accept_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_accept_invalid_http/agent_handoff/accept")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("ag_source")
                .with_dual_token_actor_kind("agent")
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("invalid accept request should return response");
    assert_eq!(accept_response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_create_system_channel_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_ops")
                .with_dual_token_actor_kind("system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_http",
                        "subscriberId":"1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create system channel request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let list_members = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_system_channel_http/members")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_ops")
                .with_dual_token_actor_kind("system")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list members request should succeed");
    assert_eq!(list_members.status(), StatusCode::OK);
    let body = list_members
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["items"].as_array().unwrap().len(), 2);
    assert!(
        value["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["principalId"] == "svc_ops" && item["principalKind"] == "system")
    );
    assert!(
        value["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["principalId"] == "1" && item["principalKind"] == "user")
    );
}

#[tokio::test]
async fn test_chat_room_create_enter_leave_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/rooms")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_chat_room_http",
                        "roomId":"room_chat_http",
                        "roomKind":"chat"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create chat room request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let view_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/rooms/room_chat_http")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get room request should succeed");
    assert_eq!(view_response.status(), StatusCode::OK);
    let view_body = view_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let view: serde_json::Value =
        serde_json::from_slice(&view_body).expect("room view should be json");
    assert_eq!(view["roomKind"], "chat");
    assert_eq!(view["conversationId"], "c_chat_room_http");
    assert_eq!(view["activeMemberCount"], 1);

    let enter_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/rooms/room_chat_http/enter")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("2")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("enter room request should succeed");
    assert_eq!(enter_response.status(), StatusCode::OK);

    let leave_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/rooms/room_chat_http/leave")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("2")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("leave room request should succeed");
    assert_eq!(leave_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_system_channel_rejects_non_system_actor_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_invalid_http",
                        "subscriberId":"1042"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create invalid system channel should return response");

    assert_eq!(create_response.status(), StatusCode::FORBIDDEN);
    let body = create_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_create_system_channel_rejects_unknown_user_subscriber_over_http() {
    let app = conversation_runtime::build_default_app_with_principal_directory(Arc::new(
        StrictKnownPrincipalDirectory::new(&["actor_a"]),
    ));

    let create_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_ops")
                .with_dual_token_actor_kind("system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_unknown_subscriber_http",
                        "subscriberId":"actor_missing"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create system channel with unknown subscriber should return response");

    assert_eq!(create_response.status(), StatusCode::BAD_REQUEST);
    let body = create_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_principal_not_found");
}

#[tokio::test]
async fn test_duplicate_create_system_channel_request_is_idempotent_and_conflicting_retry_is_rejected_over_http()
 {
    let app = conversation_runtime::build_default_app();

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_ops")
                .with_dual_token_actor_kind("system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_retry_http",
                        "subscriberId":"1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first system channel create should return response");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first system channel create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value = serde_json::from_slice(&first_create_body)
        .expect("first system channel create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert_eq!(
        first_create_json["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_create_json["requestKey"],
        "6#1000016#system7#svc_ops21#create-system_channel27#c_system_channel_retry_http"
    );

    let duplicate_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_ops")
                .with_dual_token_actor_kind("system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_retry_http",
                        "subscriberId":"1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate system channel create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::OK);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate system channel create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate system channel create should be valid json");
    assert_eq!(duplicate_create_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        duplicate_create_json["eventId"],
        first_create_json["eventId"]
    );

    let conflicting_retry = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_ops")
                .with_dual_token_actor_kind("system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_retry_http",
                        "subscriberId":"1041"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting system channel create should return response");
    assert_eq!(conflicting_retry.status(), StatusCode::CONFLICT);
    let conflicting_retry_body = conflicting_retry
        .into_body()
        .collect()
        .await
        .expect("conflicting system channel create body should collect")
        .to_bytes();
    let conflicting_retry_json: serde_json::Value = serde_json::from_slice(&conflicting_retry_body)
        .expect("conflicting system channel create should be valid json");
    assert_eq!(conflicting_retry_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_system_channel_subscriber_cannot_post_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_ops")
                .with_dual_token_actor_kind("system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_post_http",
                        "subscriberId":"1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create system channel request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let post_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_system_channel_post_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_subscriber_post",
                        "text":"should fail"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscriber post request should return response");

    assert_eq!(post_response.status(), StatusCode::FORBIDDEN);
    let body = post_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_system_channel_publisher_must_use_dedicated_publish_route_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_ops")
                .with_dual_token_actor_kind("system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_publish_http",
                        "subscriberId":"1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create system channel request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let post_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_system_channel_publish_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_ops")
                .with_dual_token_actor_kind("system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_system_channel_generic_post",
                        "text":"must use dedicated route"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("generic publish request should return response");

    assert_eq!(post_response.status(), StatusCode::FORBIDDEN);
    let body = post_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_system_channel_dedicated_publish_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_ops")
                .with_dual_token_actor_kind("system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_publish_http_dedicated",
                        "subscriberId":"1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create system channel request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let publish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_system_channel_publish_http_dedicated/system_channel/publish")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_ops")
                .with_dual_token_actor_kind("system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_system_channel_dedicated_publish",
                        "text":"system notice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("dedicated publish request should return response");

    assert_eq!(publish_response.status(), StatusCode::OK);
    let body = publish_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["messageSeq"], 1);

    let subscriber_publish = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_system_channel_publish_http_dedicated/system_channel/publish")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_system_channel_subscriber_publish",
                        "text":"should fail"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscriber dedicated publish request should return response");

    assert_eq!(subscriber_publish.status(), StatusCode::FORBIDDEN);
    let subscriber_body = subscriber_publish
        .into_body()
        .collect()
        .await
        .expect("subscriber body should collect")
        .to_bytes();
    let subscriber_value: serde_json::Value =
        serde_json::from_slice(&subscriber_body).expect("response should be valid json");
    assert_eq!(subscriber_value["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_post_message_accepts_structured_parts_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_media_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let post_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_media_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_media_http",
                        "summary":"media message",
                        "parts":[
                            {
                                "kind":"text",
                                "text":"caption"
                            },
                            {
                                "kind":"media",
                                "drive":{
                                    "driveUri":"drive://spaces/space_app_upload_demo/nodes/node_ma_demo",
                                    "spaceId":"space_app_upload_demo",
                                    "nodeId":"node_ma_demo"
                                },
                                "mediaRole":"attachment",
                                "resource":{
                                    "id":"node_ma_demo",
                                    "kind":"image",
                                    "source":"provider_asset",
                                    "uri":"drive://spaces/space_app_upload_demo/nodes/node_ma_demo",
                                    "mimeType":"image/png",
                                    "sizeBytes":"42",
                                    "fileName":"demo.png"
                                }
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post media message request should succeed");

    assert_eq!(post_response.status(), StatusCode::OK);
    let body = post_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");

    assert_eq!(value["messageSeq"], 1);
    assert_eq!(value["messageId"], "msg_c_media_http_1");
}

#[tokio::test]
async fn test_post_message_rejects_oversized_text_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_http_oversized_text",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let request_body = serde_json::json!({
        "clientMsgId": "client_http_oversized_text",
        "summary": "oversized text payload",
        "text": "x".repeat(600_000)
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_http_oversized_text/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized post message should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("messageBody")
    );
}

#[tokio::test]
async fn test_post_message_rejects_oversized_sender_session_id_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_http_oversized_sender_session",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_http_oversized_sender_session/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s".repeat(257))
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_http_oversized_sender_session",
                        "summary":"oversized sender session",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("oversized sender session post should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("senderSessionId")
    );
}

#[tokio::test]
async fn test_add_member_rejects_oversized_attributes_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_member_attributes_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let oversized_request = serde_json::json!({
        "principalId": "1043",
        "principalKind": "user",
        "role": "member",
        "attributes": {
            "profile": "x".repeat(70 * 1024)
        }
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_member_attributes_http/members/add")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(oversized_request))
                .unwrap(),
        )
        .await
        .expect("oversized add member request should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("memberAttributes")
    );
}

#[tokio::test]
async fn test_add_member_rejects_unknown_user_principal_over_http() {
    let app = conversation_runtime::build_default_app_with_principal_directory(Arc::new(
        StrictKnownPrincipalDirectory::new(&["1"]),
    ));

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_members_unknown_principal_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let add_member_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_members_unknown_principal_http/members/add")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"1044",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add unknown member request should return response");

    assert_eq!(add_member_response.status(), StatusCode::BAD_REQUEST);
    let body = add_member_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_principal_not_found");
}

#[tokio::test]
async fn test_conversation_member_endpoints_manage_roster_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_members_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let list_initial_members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_members_http/members")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list members request should succeed");
    assert_eq!(list_initial_members.status(), StatusCode::OK);
    let initial_body = list_initial_members
        .into_body()
        .collect()
        .await
        .expect("initial body should collect")
        .to_bytes();
    let initial_json: serde_json::Value =
        serde_json::from_slice(&initial_body).expect("initial members should be valid json");
    assert_eq!(initial_json["items"][0]["principalId"], "1");
    assert_eq!(initial_json["items"][0]["role"], "owner");

    let add_member_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_members_http/members/add")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"1043",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member request should succeed");
    assert_eq!(add_member_response.status(), StatusCode::OK);
    let add_member_body = add_member_response
        .into_body()
        .collect()
        .await
        .expect("add member body should collect")
        .to_bytes();
    let add_member_json: serde_json::Value =
        serde_json::from_slice(&add_member_body).expect("add member response should be valid json");
    assert_eq!(
        add_member_json["memberId"],
        "cm_c_members_http_user_1043"
    );
    assert_eq!(add_member_json["state"], "joined");

    let list_after_add = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_members_http/members")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list members after add should succeed");
    assert_eq!(list_after_add.status(), StatusCode::OK);
    let list_after_add_body = list_after_add
        .into_body()
        .collect()
        .await
        .expect("list after add body should collect")
        .to_bytes();
    let list_after_add_json: serde_json::Value = serde_json::from_slice(&list_after_add_body)
        .expect("list after add response should be valid json");
    assert_eq!(list_after_add_json["items"].as_array().unwrap().len(), 2);

    let remove_member_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_members_http/members/remove")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_members_http_user_1043"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("remove member request should succeed");
    assert_eq!(remove_member_response.status(), StatusCode::OK);
    let remove_member_body = remove_member_response
        .into_body()
        .collect()
        .await
        .expect("remove member body should collect")
        .to_bytes();
    let remove_member_json: serde_json::Value = serde_json::from_slice(&remove_member_body)
        .expect("remove member response should be valid json");
    assert_eq!(remove_member_json["state"], "removed");

    let list_after_remove = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_members_http/members")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list members after remove should succeed");
    assert_eq!(list_after_remove.status(), StatusCode::OK);
    let list_after_remove_body = list_after_remove
        .into_body()
        .collect()
        .await
        .expect("list after remove body should collect")
        .to_bytes();
    let list_after_remove_json: serde_json::Value = serde_json::from_slice(&list_after_remove_body)
        .expect("list after remove should be valid json");
    assert_eq!(list_after_remove_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(list_after_remove_json["items"][0]["principalId"], "1");
}

#[tokio::test]
async fn test_group_member_governance_over_http_rejects_actor_kind_mismatch() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_members_actor_kind_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let add_member_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_members_actor_kind_http/members/add")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"1043",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member request should return response");
    assert_eq!(add_member_response.status(), StatusCode::FORBIDDEN);
    let add_member_body = add_member_response
        .into_body()
        .collect()
        .await
        .expect("add member body should collect")
        .to_bytes();
    let add_member_json: serde_json::Value =
        serde_json::from_slice(&add_member_body).expect("add member response should be valid json");
    assert_eq!(add_member_json["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_group_member_can_leave_roster_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_members_leave_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let add_member_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_members_leave_http/members/add")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"1043",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member request should succeed");
    assert_eq!(add_member_response.status(), StatusCode::OK);

    let leave_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_members_leave_http/members/leave")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1043")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("leave request should return response");
    assert_eq!(leave_response.status(), StatusCode::OK);
    let leave_body = leave_response
        .into_body()
        .collect()
        .await
        .expect("leave body should collect")
        .to_bytes();
    let leave_json: serde_json::Value =
        serde_json::from_slice(&leave_body).expect("leave response should be valid json");
    assert_eq!(leave_json["state"], "left");

    let list_after_leave = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_members_leave_http/members")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list after leave request should succeed");
    assert_eq!(list_after_leave.status(), StatusCode::OK);
    let list_after_leave_body = list_after_leave
        .into_body()
        .collect()
        .await
        .expect("list after leave body should collect")
        .to_bytes();
    let list_after_leave_json: serde_json::Value = serde_json::from_slice(&list_after_leave_body)
        .expect("list after leave should be valid json");
    assert_eq!(list_after_leave_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(list_after_leave_json["items"][0]["principalId"], "1");
}

#[tokio::test]
async fn test_group_owner_transfer_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_members_transfer_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let add_member_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_members_transfer_http/members/add")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"1043",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member request should succeed");
    assert_eq!(add_member_response.status(), StatusCode::OK);

    let transfer_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_members_transfer_http/members/transfer_owner")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_members_transfer_http_user_1043"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("transfer request should return response");
    assert_eq!(transfer_response.status(), StatusCode::OK);
    let transfer_body = transfer_response
        .into_body()
        .collect()
        .await
        .expect("transfer body should collect")
        .to_bytes();
    let transfer_json: serde_json::Value =
        serde_json::from_slice(&transfer_body).expect("transfer response should be valid json");
    assert_eq!(transfer_json["previousOwner"]["role"], "admin");
    assert_eq!(transfer_json["newOwner"]["role"], "owner");

    let leave_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_members_transfer_http/members/leave")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("leave after transfer should return response");
    assert_eq!(leave_response.status(), StatusCode::OK);

    let list_response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_members_transfer_http/members")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1043")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("new owner list members should succeed");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body = list_response
        .into_body()
        .collect()
        .await
        .expect("list body should collect")
        .to_bytes();
    let list_json: serde_json::Value =
        serde_json::from_slice(&list_body).expect("list response should be valid json");
    assert_eq!(list_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(list_json["items"][0]["principalId"], "1043");
    assert_eq!(list_json["items"][0]["role"], "owner");
}

#[tokio::test]
async fn test_change_member_role_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_members_role_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let add_member_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_members_role_http/members/add")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"1043",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member request should succeed");
    assert_eq!(add_member_response.status(), StatusCode::OK);

    let change_role_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_members_role_http/members/change_role")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_members_role_http_user_1043",
                        "role":"admin"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("change role request should return response");
    assert_eq!(change_role_response.status(), StatusCode::OK);
    let change_role_body = change_role_response
        .into_body()
        .collect()
        .await
        .expect("change role body should collect")
        .to_bytes();
    let change_role_json: serde_json::Value = serde_json::from_slice(&change_role_body)
        .expect("change role response should be valid json");
    assert_eq!(change_role_json["previousMember"]["role"], "member");
    assert_eq!(change_role_json["updatedMember"]["role"], "admin");

    let list_response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_members_role_http/members")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list members request should succeed");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body = list_response
        .into_body()
        .collect()
        .await
        .expect("list body should collect")
        .to_bytes();
    let list_json: serde_json::Value =
        serde_json::from_slice(&list_body).expect("list response should be valid json");
    let member = list_json["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["principalId"] == "1043")
        .expect("member should exist");
    assert_eq!(member["role"], "admin");
}

#[tokio::test]
async fn test_list_members_returns_bounded_cursor_window_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_members_window_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    for principal_id in ["1045", "1046", "1047"] {
        let add_member_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/chat/conversations/c_members_window_http/members/add")
                    .with_dual_token_tenant("100001")
                    .with_dual_token_user("1")
                    .with_dual_token_actor_kind("user")
                    .header("content-type", "application/json")
                    .body(Body::from(format!(
                        r#"{{
                            "principalId":"{principal_id}",
                            "principalKind":"user",
                            "role":"member"
                        }}"#,
                    )))
                    .unwrap(),
            )
            .await
            .expect("add member request should succeed");
        assert_eq!(add_member_response.status(), StatusCode::OK);
    }

    let first_page_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_members_window_http/members?limit=2")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("first member page should return response");
    assert_eq!(first_page_response.status(), StatusCode::OK);
    let first_page_body = first_page_response
        .into_body()
        .collect()
        .await
        .expect("first page body should collect")
        .to_bytes();
    let first_page_json: serde_json::Value =
        serde_json::from_slice(&first_page_body).expect("first page should be valid json");
    assert_eq!(first_page_json["items"].as_array().unwrap().len(), 2);
    assert_eq!(first_page_json["hasMore"], true);
    let next_cursor = first_page_json["nextCursor"]
        .as_str()
        .expect("first member page should include nextCursor")
        .to_owned();

    let second_page_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/c_members_window_http/members?limit=2&cursor={next_cursor}"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second member page should return response");
    assert_eq!(second_page_response.status(), StatusCode::OK);
    let second_page_body = second_page_response
        .into_body()
        .collect()
        .await
        .expect("second page body should collect")
        .to_bytes();
    let second_page_json: serde_json::Value =
        serde_json::from_slice(&second_page_body).expect("second page should be valid json");
    assert_eq!(second_page_json["items"].as_array().unwrap().len(), 2);
    assert_eq!(second_page_json["hasMore"], false);
    assert!(second_page_json["nextCursor"].is_null());

    let mut principal_ids = first_page_json["items"]
        .as_array()
        .unwrap()
        .iter()
        .chain(second_page_json["items"].as_array().unwrap().iter())
        .map(|item| item["principalId"].as_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    principal_ids.sort();
    assert_eq!(principal_ids, ["1045", "1046", "1047", "1"]);

    let invalid_limit_response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_members_window_http/members?limit=0")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("invalid member limit should return response");
    assert_eq!(invalid_limit_response.status(), StatusCode::BAD_REQUEST);
    let invalid_limit_body = invalid_limit_response
        .into_body()
        .collect()
        .await
        .expect("invalid limit body should collect")
        .to_bytes();
    let invalid_limit_json: serde_json::Value = serde_json::from_slice(&invalid_limit_body)
        .expect("invalid limit body should be valid json");
    assert_eq!(invalid_limit_json["code"], "limit_invalid");
}

#[tokio::test]
async fn test_read_cursor_endpoints_expose_unread_progress_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_cursor_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let add_member_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_cursor_http/members/add")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"1043",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member request should succeed");
    assert_eq!(add_member_response.status(), StatusCode::OK);

    for (client_msg_id, summary) in [("client_1", "one"), ("client_2", "two")] {
        let post_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/chat/conversations/c_cursor_http/messages")
                    .with_dual_token_tenant("100001")
                    .with_dual_token_user("1043")
                    .with_dual_token_actor_kind("user")
                    .header("content-type", "application/json")
                    .body(Body::from(format!(
                        r#"{{
                            "clientMsgId":"{client_msg_id}",
                            "summary":"{summary}",
                            "text":"{summary}"
                        }}"#,
                    )))
                    .unwrap(),
            )
            .await
            .expect("post message request should succeed");
        assert_eq!(post_response.status(), StatusCode::OK);
    }

    let initial_cursor_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_cursor_http/read_cursor")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get read cursor request should succeed");
    assert_eq!(initial_cursor_response.status(), StatusCode::OK);
    let initial_cursor_body = initial_cursor_response
        .into_body()
        .collect()
        .await
        .expect("initial cursor body should collect")
        .to_bytes();
    let initial_cursor_json: serde_json::Value =
        serde_json::from_slice(&initial_cursor_body).expect("initial cursor should be valid json");
    assert_eq!(initial_cursor_json["readSeq"], 0);
    assert_eq!(initial_cursor_json["unreadCount"], 2);

    let update_cursor_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_cursor_http/read_cursor")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "readSeq": 1,
                        "lastReadMessageId":"msg_c_cursor_http_1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("update read cursor request should succeed");
    assert_eq!(update_cursor_response.status(), StatusCode::OK);
    let update_cursor_body = update_cursor_response
        .into_body()
        .collect()
        .await
        .expect("update cursor body should collect")
        .to_bytes();
    let update_cursor_json: serde_json::Value =
        serde_json::from_slice(&update_cursor_body).expect("updated cursor should be valid json");
    assert_eq!(update_cursor_json["readSeq"], 1);
    assert_eq!(update_cursor_json["unreadCount"], 1);
}

#[tokio::test]
async fn test_read_cursor_over_http_rejects_actor_kind_mismatch() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_cursor_actor_kind_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_cursor_actor_kind_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_cursor_actor_kind_http",
                        "summary":"one",
                        "text":"one"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message request should succeed");
    assert_eq!(post_response.status(), StatusCode::OK);

    let update_cursor_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_cursor_actor_kind_http/read_cursor")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "readSeq": 1,
                        "lastReadMessageId":"msg_c_cursor_actor_kind_http_1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("update read cursor request should return response");
    assert_eq!(update_cursor_response.status(), StatusCode::FORBIDDEN);
    let update_cursor_body = update_cursor_response
        .into_body()
        .collect()
        .await
        .expect("update cursor body should collect")
        .to_bytes();
    let update_cursor_json: serde_json::Value =
        serde_json::from_slice(&update_cursor_body).expect("updated cursor should be valid json");
    assert_eq!(update_cursor_json["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_edit_and_recall_message_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_edit_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_edit_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_edit_http",
                        "summary":"hello",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message request should succeed");
    assert_eq!(post_response.status(), StatusCode::OK);

    let edit_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/messages/msg_c_edit_http_1/edit")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "summary":"edited",
                        "text":"edited"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("edit message request should succeed");
    assert_eq!(edit_response.status(), StatusCode::OK);
    let edit_body = edit_response
        .into_body()
        .collect()
        .await
        .expect("edit body should collect")
        .to_bytes();
    let edit_json: serde_json::Value =
        serde_json::from_slice(&edit_body).expect("edit response should be valid json");
    assert_eq!(edit_json["messageId"], "msg_c_edit_http_1");
    assert_eq!(edit_json["messageSeq"], 1);

    let recall_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/messages/msg_c_edit_http_1/recall")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("recall message request should succeed");
    assert_eq!(recall_response.status(), StatusCode::OK);
    let recall_body = recall_response
        .into_body()
        .collect()
        .await
        .expect("recall body should collect")
        .to_bytes();
    let recall_json: serde_json::Value =
        serde_json::from_slice(&recall_body).expect("recall response should be valid json");
    assert_eq!(recall_json["messageId"], "msg_c_edit_http_1");
    assert_eq!(recall_json["messageSeq"], 1);
}

#[tokio::test]
async fn test_reaction_and_pin_message_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_reaction_pin_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_reaction_pin_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_reaction_pin_http",
                        "summary":"hello",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message request should succeed");
    assert_eq!(post_response.status(), StatusCode::OK);

    let reaction_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/messages/msg_c_reaction_pin_http_1/reactions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "reactionKey":"thumbs_up"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add reaction request should succeed");
    assert_eq!(reaction_response.status(), StatusCode::OK);
    let reaction_body = reaction_response
        .into_body()
        .collect()
        .await
        .expect("reaction body should collect")
        .to_bytes();
    let reaction_json: serde_json::Value =
        serde_json::from_slice(&reaction_body).expect("reaction response should be valid json");
    assert_eq!(reaction_json["messageId"], "msg_c_reaction_pin_http_1");
    assert_eq!(reaction_json["messageSeq"], 1);
    assert_eq!(reaction_json["reactionKey"], "thumbs_up");
    assert_eq!(reaction_json["changed"], true);

    let pin_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/messages/msg_c_reaction_pin_http_1/pin")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("pin message request should succeed");
    assert_eq!(pin_response.status(), StatusCode::OK);
    let pin_body = pin_response
        .into_body()
        .collect()
        .await
        .expect("pin body should collect")
        .to_bytes();
    let pin_json: serde_json::Value =
        serde_json::from_slice(&pin_body).expect("pin response should be valid json");
    assert_eq!(pin_json["messageId"], "msg_c_reaction_pin_http_1");
    assert_eq!(pin_json["messageSeq"], 1);
    assert_eq!(pin_json["changed"], true);

    let unpin_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/messages/msg_c_reaction_pin_http_1/unpin")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("unpin message request should succeed");
    assert_eq!(unpin_response.status(), StatusCode::OK);

    let remove_reaction_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/messages/msg_c_reaction_pin_http_1/reactions/remove")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "reactionKey":"thumbs_up"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("remove reaction request should succeed");
    assert_eq!(remove_reaction_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_conversation_with_business_policy_disables_pin_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_policy_http",
                        "conversationType":"group",
                        "policyVersion":"group.policy.v1",
                        "capabilityFlags":["message.reaction"],
                        "historyVisibility":"joined",
                        "retentionPolicyRef":"tenant.standard"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_policy_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_policy_http",
                        "summary":"hello",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message request should succeed");
    assert_eq!(post_response.status(), StatusCode::OK);

    let reaction_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/messages/msg_c_policy_http_1/reactions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "reactionKey":"thumbs_up"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add reaction request should succeed");
    assert_eq!(reaction_response.status(), StatusCode::OK);

    let pin_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/messages/msg_c_policy_http_1/pin")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("pin message request should return response");
    assert_eq!(pin_response.status(), StatusCode::FORBIDDEN);
    let pin_body = pin_response
        .into_body()
        .collect()
        .await
        .expect("pin body should collect")
        .to_bytes();
    let pin_json: serde_json::Value =
        serde_json::from_slice(&pin_body).expect("pin response should be valid json");
    assert_eq!(pin_json["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_joined_history_visibility_blocks_non_member_history_reads_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_history_joined_http",
                        "conversationType":"group",
                        "policyVersion":"group.policy.v1",
                        "historyVisibility":"joined",
                        "retentionPolicyRef":"tenant.standard"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_history_joined_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_history_joined_http",
                        "summary":"hello",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message request should succeed");
    assert_eq!(post_response.status(), StatusCode::OK);

    let history_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/conversations/c_history_joined_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1048")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("history request should return response");
    assert_eq!(history_response.status(), StatusCode::FORBIDDEN);
    let history_body = history_response
        .into_body()
        .collect()
        .await
        .expect("history body should collect")
        .to_bytes();
    let history_json: serde_json::Value =
        serde_json::from_slice(&history_body).expect("history response should be valid json");
    assert_eq!(history_json["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_world_readable_history_visibility_allows_non_member_history_reads_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_history_world_http",
                        "conversationType":"group",
                        "policyVersion":"group.policy.v1",
                        "historyVisibility":"world_readable",
                        "retentionPolicyRef":"tenant.standard"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_history_world_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_history_world_http",
                        "summary":"hello world",
                        "text":"hello world"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message request should succeed");
    assert_eq!(post_response.status(), StatusCode::OK);

    let history_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/conversations/c_history_world_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1048")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("history request should return response");
    assert_eq!(history_response.status(), StatusCode::OK);
    let history_body = history_response
        .into_body()
        .collect()
        .await
        .expect("history body should collect")
        .to_bytes();
    let history_json: serde_json::Value =
        serde_json::from_slice(&history_body).expect("history response should be valid json");
    assert_eq!(
        history_json["items"][0]["message"]["messageId"],
        "msg_c_history_world_http_1"
    );
    assert_eq!(
        history_json["items"][0]["message"]["body"]["summary"],
        "hello world"
    );
}

#[tokio::test]
async fn test_bind_direct_chat_conversation_over_http_and_query_binding() {
    let app = conversation_runtime::build_default_app();

    let bind_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/direct_chats/bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_control")
                .with_dual_token_actor_kind("system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_direct_binding_http",
                        "directChatId":"dc_http",
                        "leftActorId":"actor_a",
                        "leftActorKind":"user",
                        "rightActorId":"actor_b"
                        ,"rightActorKind":"user"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat binding request should return response");
    assert_eq!(bind_response.status(), StatusCode::OK);
    let bind_body = bind_response
        .into_body()
        .collect()
        .await
        .expect("bind body should collect")
        .to_bytes();
    let bind_json: serde_json::Value =
        serde_json::from_slice(&bind_body).expect("bind response should be valid json");
    assert_eq!(bind_json["conversationId"], "c_direct_binding_http");

    let binding_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/conversations/c_direct_binding_http/binding")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_control")
                .with_dual_token_actor_kind("system")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("binding query should return response");
    assert_eq!(binding_response.status(), StatusCode::OK);
    let binding_body = binding_response
        .into_body()
        .collect()
        .await
        .expect("binding body should collect")
        .to_bytes();
    let binding_json: serde_json::Value =
        serde_json::from_slice(&binding_body).expect("binding response should be valid json");
    assert_eq!(binding_json["conversationId"], "c_direct_binding_http");
    assert_eq!(binding_json["businessType"], "direct_chat");
    assert_eq!(binding_json["businessId"], "dc_http");

    let members_response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_direct_binding_http/members")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("actor_a")
                .with_dual_token_actor_kind("user")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list members request should return response");
    assert_eq!(members_response.status(), StatusCode::OK);
    let members_body = members_response
        .into_body()
        .collect()
        .await
        .expect("members body should collect")
        .to_bytes();
    let members_json: serde_json::Value =
        serde_json::from_slice(&members_body).expect("members response should be valid json");
    assert_eq!(members_json["items"].as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn test_bind_direct_chat_conversation_rejects_unknown_user_participant_over_http() {
    let app = conversation_runtime::build_default_app_with_principal_directory(Arc::new(
        StrictKnownPrincipalDirectory::new(&["actor_a"]),
    ));

    let bind_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/direct_chats/bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_control")
                .with_dual_token_actor_kind("system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_direct_binding_http_unknown",
                        "directChatId":"dc_http_unknown",
                        "leftActorId":"actor_a",
                        "leftActorKind":"user",
                        "rightActorId":"actor_missing",
                        "rightActorKind":"user"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat binding request should return response");
    assert_eq!(bind_response.status(), StatusCode::BAD_REQUEST);
    let bind_body = bind_response
        .into_body()
        .collect()
        .await
        .expect("bind body should collect")
        .to_bytes();
    let bind_json: serde_json::Value =
        serde_json::from_slice(&bind_body).expect("bind response should be valid json");
    assert_eq!(bind_json["code"], "conversation_principal_not_found");
}

#[tokio::test]
async fn test_bind_direct_chat_conversation_rejects_unknown_user_participant_with_static_catalog_over_http()
 {
    let catalog_path = unique_principal_catalog_path();
    fs::write(
        &catalog_path,
        r#"{
            "principals":[
                {
                    "tenantId":"100001",
                    "principalId":"actor_a",
                    "principalKind":"user"
                }
            ]
        }"#,
    )
    .expect("principal catalog should be written");
    let principal_directory =
        conversation_runtime::StaticPrincipalDirectory::from_json_file(catalog_path.as_path())
            .expect("static principal directory should load catalog");
    let app = conversation_runtime::build_default_app_with_principal_directory(Arc::new(
        principal_directory,
    ));

    let bind_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/direct_chats/bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_control")
                .with_dual_token_actor_kind("system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_direct_binding_http_static_unknown",
                        "directChatId":"dc_http_static_unknown",
                        "leftActorId":"actor_a",
                        "leftActorKind":"user",
                        "rightActorId":"actor_missing",
                        "rightActorKind":"user"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat binding request should return response");
    assert_eq!(bind_response.status(), StatusCode::BAD_REQUEST);
    let bind_body = bind_response
        .into_body()
        .collect()
        .await
        .expect("bind body should collect")
        .to_bytes();
    let bind_json: serde_json::Value =
        serde_json::from_slice(&bind_body).expect("bind response should be valid json");
    assert_eq!(bind_json["code"], "conversation_principal_not_found");

    let _ = fs::remove_file(catalog_path);
}

#[tokio::test]
async fn test_duplicate_bind_direct_chat_conversation_request_is_idempotent_and_conflicting_retry_is_rejected_over_http()
 {
    let app = conversation_runtime::build_default_app();

    let first_bind = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/direct_chats/bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_control")
                .with_dual_token_actor_kind("system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_direct_retry_http",
                        "directChatId":"dc_retry_http",
                        "leftActorId":"actor_a",
                        "leftActorKind":"user",
                        "rightActorId":"actor_b",
                        "rightActorKind":"user"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first direct chat binding should return response");
    assert_eq!(first_bind.status(), StatusCode::OK);
    let first_bind_body = first_bind
        .into_body()
        .collect()
        .await
        .expect("first bind body should collect")
        .to_bytes();
    let first_bind_json: serde_json::Value =
        serde_json::from_slice(&first_bind_body).expect("first bind should be valid json");
    assert_eq!(first_bind_json["deliveryStatus"], "applied");
    assert_eq!(
        first_bind_json["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_bind_json["requestKey"],
        "6#1000016#system11#svc_control16#bind-direct-chat19#c_direct_retry_http"
    );

    let duplicate_bind = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/direct_chats/bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_control")
                .with_dual_token_actor_kind("system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_direct_retry_http",
                        "directChatId":"dc_retry_http",
                        "leftActorId":"actor_a",
                        "leftActorKind":"user",
                        "rightActorId":"actor_b",
                        "rightActorKind":"user"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate direct chat binding should return response");
    assert_eq!(duplicate_bind.status(), StatusCode::OK);
    let duplicate_bind_body = duplicate_bind
        .into_body()
        .collect()
        .await
        .expect("duplicate bind body should collect")
        .to_bytes();
    let duplicate_bind_json: serde_json::Value =
        serde_json::from_slice(&duplicate_bind_body).expect("duplicate bind should be valid json");
    assert_eq!(duplicate_bind_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_bind_json["requestKey"],
        first_bind_json["requestKey"]
    );
    assert_eq!(duplicate_bind_json["eventId"], first_bind_json["eventId"]);

    let conflicting_bind = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/direct_chats/bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("svc_control")
                .with_dual_token_actor_kind("system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_direct_retry_http",
                        "directChatId":"dc_other_http",
                        "leftActorId":"actor_a",
                        "leftActorKind":"user",
                        "rightActorId":"actor_b",
                        "rightActorKind":"user"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting direct chat binding should return response");
    assert_eq!(conflicting_bind.status(), StatusCode::CONFLICT);
    let conflicting_bind_body = conflicting_bind
        .into_body()
        .collect()
        .await
        .expect("conflicting bind body should collect")
        .to_bytes();
    let conflicting_bind_json: serde_json::Value = serde_json::from_slice(&conflicting_bind_body)
        .expect("conflicting bind should be valid json");
    assert_eq!(conflicting_bind_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_create_thread_conversation_over_http_and_query_binding() {
    let app = conversation_runtime::build_default_app();

    let create_parent_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_parent_thread_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create parent conversation request should return response");
    assert_eq!(create_parent_response.status(), StatusCode::OK);

    let post_root_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_parent_thread_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_thread_root_http",
                        "summary":"root",
                        "text":"root"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post root message request should return response");
    assert_eq!(post_root_response.status(), StatusCode::OK);
    let post_root_body = post_root_response
        .into_body()
        .collect()
        .await
        .expect("post root body should collect")
        .to_bytes();
    let post_root_json: serde_json::Value =
        serde_json::from_slice(&post_root_body).expect("post root response should be valid json");

    let create_thread_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/threads")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "conversationId":"c_thread_http",
                        "parentConversationId":"c_parent_thread_http",
                        "rootMessageId":"{}"
                    }}"#,
                    post_root_json["messageId"].as_str().unwrap()
                )))
                .unwrap(),
        )
        .await
        .expect("create thread request should return response");
    assert_eq!(create_thread_response.status(), StatusCode::OK);
    let create_thread_body = create_thread_response
        .into_body()
        .collect()
        .await
        .expect("create thread body should collect")
        .to_bytes();
    let create_thread_json: serde_json::Value = serde_json::from_slice(&create_thread_body)
        .expect("create thread response should be valid json");
    assert_eq!(create_thread_json["conversationId"], "c_thread_http");

    let binding_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/conversations/c_thread_http/binding")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("thread binding query should return response");
    assert_eq!(binding_response.status(), StatusCode::OK);
    let binding_body = binding_response
        .into_body()
        .collect()
        .await
        .expect("thread binding body should collect")
        .to_bytes();
    let binding_json: serde_json::Value =
        serde_json::from_slice(&binding_body).expect("binding response should be valid json");
    assert_eq!(binding_json["conversationId"], "c_thread_http");
    assert_eq!(binding_json["businessType"], "thread");
    assert_eq!(binding_json["businessId"], post_root_json["messageId"]);

    let members_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/conversations/c_thread_http/members")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("thread members query should return response");
    assert_eq!(members_response.status(), StatusCode::OK);
    let members_body = members_response
        .into_body()
        .collect()
        .await
        .expect("thread members body should collect")
        .to_bytes();
    let members_json: serde_json::Value = serde_json::from_slice(&members_body)
        .expect("thread members response should be valid json");
    assert_eq!(
        members_json["items"][0]["attributes"]["parentConversationId"],
        "c_parent_thread_http"
    );
    assert_eq!(
        members_json["items"][0]["attributes"]["rootMessageId"],
        post_root_json["messageId"]
    );
    assert_eq!(
        members_json["items"][0]["attributes"]["threadRole"],
        "owner"
    );
}

#[tokio::test]
async fn test_duplicate_create_thread_conversation_request_is_idempotent_and_conflicting_retry_is_rejected_over_http()
 {
    let app = conversation_runtime::build_default_app();

    let create_parent_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_parent_thread_retry_http",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create parent conversation request should return response");
    assert_eq!(create_parent_response.status(), StatusCode::OK);

    let first_root_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_parent_thread_retry_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_thread_retry_http_root_1",
                        "summary":"root-1",
                        "text":"root-1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first root message request should return response");
    assert_eq!(first_root_response.status(), StatusCode::OK);
    let first_root_body = first_root_response
        .into_body()
        .collect()
        .await
        .expect("first root body should collect")
        .to_bytes();
    let first_root_json: serde_json::Value =
        serde_json::from_slice(&first_root_body).expect("first root response should be valid json");

    let second_root_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_parent_thread_retry_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_thread_retry_http_root_2",
                        "summary":"root-2",
                        "text":"root-2"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second root message request should return response");
    assert_eq!(second_root_response.status(), StatusCode::OK);
    let second_root_body = second_root_response
        .into_body()
        .collect()
        .await
        .expect("second root body should collect")
        .to_bytes();
    let second_root_json: serde_json::Value = serde_json::from_slice(&second_root_body)
        .expect("second root response should be valid json");

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/threads")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "conversationId":"c_thread_retry_http",
                        "parentConversationId":"c_parent_thread_retry_http",
                        "rootMessageId":"{}"
                    }}"#,
                    first_root_json["messageId"].as_str().unwrap()
                )))
                .unwrap(),
        )
        .await
        .expect("first thread create should return response");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first thread create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value = serde_json::from_slice(&first_create_body)
        .expect("first thread create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert_eq!(
        first_create_json["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_create_json["requestKey"],
        "6#1000014#user1#1#13#create-thread19#c_thread_retry_http"
    );

    let duplicate_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/threads")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "conversationId":"c_thread_retry_http",
                        "parentConversationId":"c_parent_thread_retry_http",
                        "rootMessageId":"{}"
                    }}"#,
                    first_root_json["messageId"].as_str().unwrap()
                )))
                .unwrap(),
        )
        .await
        .expect("duplicate thread create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::OK);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate thread create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate thread create should be valid json");
    assert_eq!(duplicate_create_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        duplicate_create_json["eventId"],
        first_create_json["eventId"]
    );

    let conflicting_retry = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/threads")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "conversationId":"c_thread_retry_http",
                        "parentConversationId":"c_parent_thread_retry_http",
                        "rootMessageId":"{}"
                    }}"#,
                    second_root_json["messageId"].as_str().unwrap()
                )))
                .unwrap(),
        )
        .await
        .expect("conflicting thread create should return response");
    assert_eq!(conflicting_retry.status(), StatusCode::CONFLICT);
    let conflicting_retry_body = conflicting_retry
        .into_body()
        .collect()
        .await
        .expect("conflicting thread create body should collect")
        .to_bytes();
    let conflicting_retry_json: serde_json::Value = serde_json::from_slice(&conflicting_retry_body)
        .expect("conflicting thread create should be valid json");
    assert_eq!(conflicting_retry_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_bind_direct_chat_conversation_rejects_non_system_actor_over_http() {
    let app = conversation_runtime::build_default_app();

    let bind_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/direct_chats/bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_direct_binding_http_denied",
                        "directChatId":"dc_http_denied",
                        "leftActorId":"actor_a",
                        "leftActorKind":"user",
                        "rightActorId":"actor_b"
                        ,"rightActorKind":"user"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat binding request should return response");

    assert_eq!(bind_response.status(), StatusCode::FORBIDDEN);
    let bind_body = bind_response
        .into_body()
        .collect()
        .await
        .expect("bind body should collect")
        .to_bytes();
    let bind_json: serde_json::Value =
        serde_json::from_slice(&bind_body).expect("bind response should be valid json");
    assert_eq!(bind_json["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_invited_history_visibility_allows_invited_member_history_reads_before_join_over_http()
{
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_history_invited_http",
                        "conversationType":"group",
                        "policyVersion":"group.policy.v1",
                        "historyVisibility":"invited",
                        "retentionPolicyRef":"tenant.standard"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create invited-history conversation request should return response");
    assert_eq!(create_response.status(), StatusCode::OK);

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_history_invited_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_history_invited_http",
                        "summary":"hello invited",
                        "text":"hello invited"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post invited-history message request should succeed");
    assert_eq!(post_response.status(), StatusCode::OK);

    let add_member_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_history_invited_http/members/add")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"1049",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add invited member request should succeed");
    assert_eq!(add_member_response.status(), StatusCode::OK);
    let add_member_body = add_member_response
        .into_body()
        .collect()
        .await
        .expect("add invited member body should collect")
        .to_bytes();
    let add_member_json: serde_json::Value =
        serde_json::from_slice(&add_member_body).expect("add invited member should be valid json");
    assert_eq!(add_member_json["state"], "invited");

    let invited_history_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/conversations/c_history_invited_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1049")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("invited history request should return response");
    assert_eq!(invited_history_response.status(), StatusCode::OK);
    let invited_history_body = invited_history_response
        .into_body()
        .collect()
        .await
        .expect("invited history body should collect")
        .to_bytes();
    let invited_history_json: serde_json::Value = serde_json::from_slice(&invited_history_body)
        .expect("invited history response should be valid json");
    assert_eq!(
        invited_history_json["items"][0]["message"]["messageId"],
        "msg_c_history_invited_http_1"
    );
    assert_eq!(
        invited_history_json["items"][0]["message"]["body"]["summary"],
        "hello invited"
    );

    let outsider_history_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/conversations/c_history_invited_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1048")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("outsider history request should return response");
    assert_eq!(outsider_history_response.status(), StatusCode::FORBIDDEN);
    let outsider_history_body = outsider_history_response
        .into_body()
        .collect()
        .await
        .expect("outsider history body should collect")
        .to_bytes();
    let outsider_history_json: serde_json::Value = serde_json::from_slice(&outsider_history_body)
        .expect("outsider history should be valid json");
    assert_eq!(
        outsider_history_json["code"],
        "conversation_permission_denied"
    );
}

#[tokio::test]
async fn test_shared_history_visibility_allows_external_linked_history_reads_but_not_writes_over_http()
 {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_history_shared_http",
                        "conversationType":"group",
                        "policyVersion":"group.policy.v1",
                        "historyVisibility":"shared",
                        "retentionPolicyRef":"tenant.standard"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create shared-history conversation request should return response");
    assert_eq!(create_response.status(), StatusCode::OK);

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_history_shared_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_history_shared_http",
                        "summary":"hello shared",
                        "text":"hello shared"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post shared-history message request should succeed");
    assert_eq!(post_response.status(), StatusCode::OK);

    let add_member_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_history_shared_http/members/add")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"1050",
                        "principalKind":"user",
                        "role":"guest",
                        "attributes":{
                            "sharedChannelPolicyId":"scp_001",
                            "externalConnectionId":"ec_003",
                            "externalMemberId":"partner_user_42"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add shared-linked member request should succeed");
    assert_eq!(add_member_response.status(), StatusCode::OK);
    let add_member_body = add_member_response
        .into_body()
        .collect()
        .await
        .expect("add shared-linked member body should collect")
        .to_bytes();
    let add_member_json: serde_json::Value = serde_json::from_slice(&add_member_body)
        .expect("add shared-linked member body should be valid json");
    assert_eq!(add_member_json["state"], "linked");
    assert_eq!(
        add_member_json["attributes"]["sharedChannelPolicyId"],
        "scp_001"
    );

    let linked_history_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/conversations/c_history_shared_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1050")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("shared linked history request should return response");
    assert_eq!(linked_history_response.status(), StatusCode::OK);
    let linked_history_body = linked_history_response
        .into_body()
        .collect()
        .await
        .expect("shared linked history body should collect")
        .to_bytes();
    let linked_history_json: serde_json::Value = serde_json::from_slice(&linked_history_body)
        .expect("shared linked history should be valid json");
    assert_eq!(
        linked_history_json["items"][0]["message"]["messageId"],
        "msg_c_history_shared_http_1"
    );
    assert_eq!(
        linked_history_json["items"][0]["message"]["body"]["summary"],
        "hello shared"
    );

    let linked_post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_history_shared_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1050")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_history_shared_http_external",
                        "summary":"external write should fail",
                        "text":"external write should fail"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared linked write request should return response");
    assert_eq!(linked_post_response.status(), StatusCode::FORBIDDEN);
    let linked_post_body = linked_post_response
        .into_body()
        .collect()
        .await
        .expect("shared linked write body should collect")
        .to_bytes();
    let linked_post_json: serde_json::Value = serde_json::from_slice(&linked_post_body)
        .expect("shared linked write body should be valid json");
    assert_eq!(linked_post_json["code"], "conversation_permission_denied");

    let outsider_history_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/conversations/c_history_shared_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1048")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("shared outsider history request should return response");
    assert_eq!(outsider_history_response.status(), StatusCode::FORBIDDEN);
    let outsider_history_body = outsider_history_response
        .into_body()
        .collect()
        .await
        .expect("shared outsider history body should collect")
        .to_bytes();
    let outsider_history_json: serde_json::Value = serde_json::from_slice(&outsider_history_body)
        .expect("shared outsider history should be valid json");
    assert_eq!(
        outsider_history_json["code"],
        "conversation_permission_denied"
    );
}

#[tokio::test]
async fn test_sync_shared_channel_linked_member_over_http_materializes_linked_history_reader() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_history_shared_sync_http",
                        "conversationType":"group",
                        "policyVersion":"group.policy.v1",
                        "historyVisibility":"shared",
                        "retentionPolicyRef":"tenant.standard"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create shared-history conversation request should return response");
    assert_eq!(create_response.status(), StatusCode::OK);

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_history_shared_sync_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_history_shared_sync_http",
                        "summary":"hello sync",
                        "text":"hello sync"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post shared-history message request should succeed");
    assert_eq!(post_response.status(), StatusCode::OK);

    let sync_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/shared_channel_links/sync")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("control-plane-sync")
                .with_dual_token_actor_kind("system")
                .with_dual_token_permission_scope("conversation.shared_channel.sync")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_history_shared_sync_http",
                        "sharedChannelPolicyId":"scp_sync_http",
                        "externalConnectionId":"ec_sync_http",
                        "localActorId":"1007",
                        "localActorKind":"user",
                        "externalMemberId":"partner::sync-user"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel linked-member sync request should return response");
    assert_eq!(sync_response.status(), StatusCode::OK);
    let sync_body = sync_response
        .into_body()
        .collect()
        .await
        .expect("sync body should collect")
        .to_bytes();
    let sync_json: serde_json::Value =
        serde_json::from_slice(&sync_body).expect("sync body should be valid json");
    assert_eq!(sync_json["proofVersion"], "shared_channel_sync_ack.v1");
    assert_eq!(sync_json["status"], "applied");
    assert_eq!(
        sync_json["requestKey"],
        "100001|c_history_shared_sync_http|scp_sync_http|ec_sync_http|1007|user|partner::sync-user"
    );
    assert_eq!(sync_json["principalId"], "1007");
    assert_eq!(sync_json["principalKind"], "user");
    assert_eq!(sync_json["role"], "guest");
    assert_eq!(sync_json["state"], "linked");
    assert_eq!(
        sync_json["attributes"]["sharedChannelPolicyId"],
        "scp_sync_http"
    );
    assert_eq!(
        sync_json["attributes"]["externalConnectionId"],
        "ec_sync_http"
    );
    assert_eq!(
        sync_json["attributes"]["externalMemberId"],
        "partner::sync-user"
    );

    let linked_history_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/conversations/c_history_shared_sync_http/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1007")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("shared linked history request should return response");
    assert_eq!(linked_history_response.status(), StatusCode::OK);
    let linked_history_body = linked_history_response
        .into_body()
        .collect()
        .await
        .expect("shared linked history body should collect")
        .to_bytes();
    let linked_history_json: serde_json::Value = serde_json::from_slice(&linked_history_body)
        .expect("shared linked history should be valid json");
    assert_eq!(
        linked_history_json["items"][0]["message"]["messageId"],
        "msg_c_history_shared_sync_http_1"
    );
    assert_eq!(
        linked_history_json["items"][0]["message"]["body"]["summary"],
        "hello sync"
    );

    let resync_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/shared_channel_links/sync")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("control-plane-sync")
                .with_dual_token_actor_kind("system")
                .with_dual_token_permission_scope("conversation.shared_channel.sync")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_history_shared_sync_http",
                        "sharedChannelPolicyId":"scp_sync_http",
                        "externalConnectionId":"ec_sync_http",
                        "localActorId":"1007",
                        "localActorKind":"user",
                        "externalMemberId":"partner::sync-user",
                        "requestKey":"100001|c_history_shared_sync_http|scp_sync_http|ec_sync_http|1007|user|partner::sync-user"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel linked-member resync request should return response");
    assert_eq!(resync_response.status(), StatusCode::OK);
    let resync_body = resync_response
        .into_body()
        .collect()
        .await
        .expect("resync body should collect")
        .to_bytes();
    let resync_json: serde_json::Value =
        serde_json::from_slice(&resync_body).expect("resync body should be valid json");
    assert_eq!(resync_json["proofVersion"], "shared_channel_sync_ack.v1");
    assert_eq!(resync_json["status"], "replayed");
    assert_eq!(
        resync_json["requestKey"],
        "100001|c_history_shared_sync_http|scp_sync_http|ec_sync_http|1007|user|partner::sync-user"
    );
    assert_eq!(
        resync_json["attributes"]["sharedChannelSyncRequestKey"],
        "100001|c_history_shared_sync_http|scp_sync_http|ec_sync_http|1007|user|partner::sync-user"
    );
}

#[tokio::test]
async fn test_sync_shared_channel_linked_member_rejects_unknown_user_local_actor_over_http() {
    let app = conversation_runtime::build_default_app_with_principal_directory(Arc::new(
        StrictKnownPrincipalDirectory::new(&["1"]),
    ));

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_history_shared_sync_unknown_http",
                        "conversationType":"group",
                        "policyVersion":"group.policy.v1",
                        "historyVisibility":"shared",
                        "retentionPolicyRef":"tenant.standard"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create shared-history conversation request should return response");
    assert_eq!(create_response.status(), StatusCode::OK);

    let sync_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/shared_channel_links/sync")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("control-plane-sync")
                .with_dual_token_actor_kind("system")
                .with_dual_token_permission_scope("conversation.shared_channel.sync")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_history_shared_sync_unknown_http",
                        "sharedChannelPolicyId":"scp_sync_unknown_http",
                        "externalConnectionId":"ec_sync_unknown_http",
                        "localActorId":"1044",
                        "localActorKind":"user",
                        "externalMemberId":"partner::unknown-user"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel sync with unknown local actor should return response");

    assert_eq!(sync_response.status(), StatusCode::BAD_REQUEST);
    let body = sync_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_principal_not_found");
}

#[tokio::test]
async fn test_shared_history_sync_rejects_oversized_local_actor_kind_over_http() {
    let app = conversation_runtime::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_history_shared_sync_oversized_kind",
                        "conversationType":"group",
                        "policyVersion":"group.policy.v1",
                        "historyVisibility":"shared",
                        "retentionPolicyRef":"tenant.standard"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create shared-history conversation request should return response");
    assert_eq!(create_response.status(), StatusCode::OK);

    let request_body = serde_json::json!({
        "conversationId":"c_history_shared_sync_oversized_kind",
        "sharedChannelPolicyId":"scp_sync_http_oversized_kind",
        "externalConnectionId":"ec_sync_http_oversized_kind",
        "localActorId":"1007_oversized_kind",
        "localActorKind":"k".repeat(2048),
        "externalMemberId":"partner::sync-user-oversized-kind"
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/shared_channel_links/sync")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("control-plane-sync")
                .with_dual_token_actor_kind("system")
                .with_dual_token_permission_scope("conversation.shared_channel.sync")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized shared history sync should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("localActorKind")
    );
}

#[test]
fn test_static_principal_directory_rejects_missing_principal_kind() {
    let catalog_path = unique_principal_catalog_path();
    fs::write(
        &catalog_path,
        r#"{
            "principals":[
                {
                    "tenantId":"100001",
                    "principalId":"actor_without_kind"
                }
            ]
        }"#,
    )
    .expect("principal catalog should be written");

    let error =
        conversation_runtime::StaticPrincipalDirectory::from_json_file(catalog_path.as_path())
            .expect_err("principalKind must be explicit in static principal catalogs");

    assert!(
        error.contains("principalKind"),
        "error should point to the missing principalKind field, got: {error}"
    );
}
