use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_app_context::DualTokenRequestBuilderExt;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};
use tower::ServiceExt;

static UNIQUE_CATALOG_COUNTER: AtomicU64 = AtomicU64::new(0);

fn ensure_http_smoke_test_environment() {
    static TEST_ENVIRONMENT: OnceLock<()> = OnceLock::new();
    TEST_ENVIRONMENT.get_or_init(|| {
        // These handler tests use the explicit local AppState fixture. Server
        // environment bootstrap is covered separately with PostgreSQL.
        unsafe {
            std::env::set_var("SDKWORK_IM_ENVIRONMENT", "test");
        }
    });
}

fn build_public_test_app() -> axum::Router {
    ensure_http_smoke_test_environment();
    sdkwork_routes_im_chat_open_api::build_public_app()
}

fn build_service_public_test_app() -> axum::Router {
    ensure_http_smoke_test_environment();
    conversation_runtime::build_public_app()
}

fn build_default_test_app() -> axum::Router {
    ensure_http_smoke_test_environment();
    sdkwork_routes_im_chat_open_api::build_public_app()
}

fn build_default_test_app_with_principal_directory(
    principal_directory: Arc<dyn conversation_runtime::PrincipalDirectory>,
) -> axum::Router {
    ensure_http_smoke_test_environment();
    sdkwork_routes_im_chat_open_api::build_public_app_with_principal_directory(principal_directory)
}

fn response_item(value: &serde_json::Value) -> &serde_json::Value {
    value
        .get("data")
        .and_then(|data| data.get("item"))
        .expect("response should use standard data.item envelope")
}

fn assert_applied_create_conversation_response_shape(
    response: &serde_json::Value,
    expects_knowledgebase_initialization: bool,
) {
    let item = response_item(response);
    let mut field_names: Vec<String> = item
        .as_object()
        .expect("create response item should be an object")
        .keys()
        .cloned()
        .collect();
    field_names.sort_unstable();
    let expected_field_names = if expects_knowledgebase_initialization {
        vec![
            "conversationId",
            "deliveryStatus",
            "eventId",
            "knowledgebaseInitialization",
            "proofVersion",
            "requestKey",
        ]
    } else {
        vec![
            "conversationId",
            "deliveryStatus",
            "eventId",
            "proofVersion",
            "requestKey",
        ]
    };
    assert_eq!(
        field_names,
        expected_field_names
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>(),
        "create response item must match the published SDK contract"
    );
    assert!(item["conversationId"].is_string());
    assert!(item["eventId"].is_string());
    assert!(item["requestKey"].is_string());
    assert_eq!(item["deliveryStatus"], "applied");
    assert!(item["proofVersion"].is_string());
    if expects_knowledgebase_initialization {
        assert!(item["knowledgebaseInitialization"].is_string());
    } else {
        assert!(item.get("knowledgebaseInitialization").is_none());
    }
}

async fn create_test_group_conversation(
    app: axum::Router,
    tenant_id: &str,
    user_id: &str,
    actor_kind: &str,
    client_request_key: &str,
) -> String {
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant(tenant_id)
                .with_dual_token_user(user_id)
                .with_dual_token_actor_kind(actor_kind)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{"conversationType":"group","groupName":"test group","clientRequestKey":"{}"}}"#,
                    client_request_key.replace('"', "\\\"")
                )))
                .unwrap(),
        )
        .await
        .expect("create group conversation request should succeed");
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("create response should be valid json");
    value["data"]["item"]["conversationId"]
        .as_str()
        .expect("create response should include canonical conversation id")
        .to_string()
}

async fn post_test_text_message(
    app: axum::Router,
    conversation_id: &str,
    client_msg_id: &str,
    summary: &str,
) -> serde_json::Value {
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "clientMsgId": client_msg_id,
                        "summary": summary,
                        "text": summary,
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("post test message request should complete");
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("post test message body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("post test message body should be valid json");
    response_item(&value).clone()
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
    let app = build_service_public_test_app();

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
    let app = build_service_public_test_app();

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
async fn test_public_app_rejects_missing_credentials_over_http() {
    let app = build_public_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
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

    assert_eq!(value["code"], 40101);
}

#[tokio::test]
async fn test_create_conversation_and_post_message_over_http() {
    let app = build_default_test_app();

    let conversation_id =
        create_test_group_conversation(app.clone(), "100001", "1", "user", "c_http").await;

    let post_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{}/messages",
                    conversation_id
                ))
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

    assert_eq!(post_response.status(), StatusCode::CREATED);
    let body = post_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    let item = response_item(&value);

    assert_eq!(item["messageSeq"], 1);
    assert_eq!(item["messageId"], format!("msg_{}_1", conversation_id));
}

#[tokio::test]
async fn test_current_conversation_member_returns_authoritative_actor_role_over_http() {
    let app = build_default_test_app();
    let conversation_id =
        create_test_group_conversation(app.clone(), "100001", "1", "user", "c_http_current_member")
            .await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members/current"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("current member request should succeed");
    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("current member body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("current member response should be valid json");
    let item = response_item(&value);
    assert_eq!(item["principalId"], "1");
    assert_eq!(item["principalKind"], "user");
    assert_eq!(item["role"], "owner");
    assert_eq!(item["state"], "joined");

    let forbidden = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members/current"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("not-a-member")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("non-member current lookup should return a response");
    assert_eq!(forbidden.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_post_media_message_rejects_missing_drive_reference_over_http() {
    let app = build_default_test_app();

    let conversation_id = create_test_group_conversation(
        app.clone(),
        "100001",
        "1",
        "user",
        "c_media_missing_drive_http",
    )
    .await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{}/messages",
                    conversation_id
                ))
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
    assert_eq!(value["code"], 40001);
    assert!(
        value["detail"]
            .as_str()
            .expect("message should be present")
            .contains("drive")
    );
}

#[tokio::test]
async fn test_post_media_message_rejects_noncanonical_drive_reference_over_http() {
    let app = build_default_test_app();

    let conversation_id = create_test_group_conversation(
        app.clone(),
        "100001",
        "1",
        "user",
        "c_media_bad_drive_http",
    )
    .await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{}/messages",
                    conversation_id
                ))
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
    assert_eq!(value["code"], 40001);
    assert!(
        value["detail"]
            .as_str()
            .expect("message should be present")
            .contains("drive.driveUri")
    );
}

#[tokio::test]
async fn test_post_media_message_rejects_external_url_source_with_drive_reference_over_http() {
    let app = build_default_test_app();

    let conversation_id = create_test_group_conversation(
        app.clone(),
        "100001",
        "1",
        "user",
        "c_media_external_url_source_http",
    )
    .await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{}/messages",
                    conversation_id
                ))
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
    assert_eq!(value["code"], 40001);
    assert!(
        value["detail"]
            .as_str()
            .expect("message should be present")
            .contains("resource.source")
    );
}

#[tokio::test]
async fn test_duplicate_create_conversation_request_is_idempotent_and_conflicting_retry_is_rejected_over_http()
 {
    let app = build_default_test_app();

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
                    r#"{"conversationType":"group","groupName":"test group","clientRequestKey":"c_create_retry_http"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first create should return response");
    assert_eq!(first_create.status(), StatusCode::CREATED);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value =
        serde_json::from_slice(&first_create_body).expect("first create should be valid json");
    let first_create_item = response_item(&first_create_json);
    assert_eq!(first_create_item["deliveryStatus"], "applied");
    assert_eq!(
        first_create_item["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert!(first_create_item["requestKey"].is_string());
    let conversation_id = first_create_item["conversationId"]
        .as_str()
        .expect("first create should return canonical conversation id")
        .to_string();

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
                    r#"{"conversationType":"group","groupName":"test group","clientRequestKey":"c_create_retry_http"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::CREATED);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate create should be valid json");
    let duplicate_create_item = response_item(&duplicate_create_json);
    assert_eq!(duplicate_create_item["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_item["requestKey"],
        first_create_item["requestKey"]
    );
    assert_eq!(
        duplicate_create_item["eventId"],
        first_create_item["eventId"]
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
                .body(Body::from(format!(
                    r#"{{"conversationId":"{}","conversationType":"direct"}}"#,
                    conversation_id
                )))
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
    assert_eq!(conflicting_retry_json["code"], 40901);
}

#[tokio::test]
async fn test_group_creation_keeps_knowledgebase_lazy_unless_explicitly_requested() {
    let app = build_default_test_app();

    let lazy_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("0")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"conversationType":"group","groupName":"lazy group","clientRequestKey":"c_lazy_group_no_knowledgebase"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("lazy group creation should return a response");
    assert_eq!(lazy_create.status(), StatusCode::CREATED);
    let lazy_body = lazy_create
        .into_body()
        .collect()
        .await
        .expect("lazy group response body should collect")
        .to_bytes();
    let lazy_json: serde_json::Value =
        serde_json::from_slice(&lazy_body).expect("lazy group response should be valid json");
    assert_applied_create_conversation_response_shape(&lazy_json, false);

    let explicit_lazy_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("0")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"conversationType":"group","groupName":"explicit lazy group","clientRequestKey":"c_lazy_group_explicit_false","initializeKnowledgebase":false}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("explicit lazy group creation should return a response");
    assert_eq!(explicit_lazy_create.status(), StatusCode::CREATED);
    let explicit_lazy_body = explicit_lazy_create
        .into_body()
        .collect()
        .await
        .expect("explicit lazy group response body should collect")
        .to_bytes();
    let explicit_lazy_json: serde_json::Value = serde_json::from_slice(&explicit_lazy_body)
        .expect("explicit lazy group response should be valid json");
    assert_applied_create_conversation_response_shape(&explicit_lazy_json, false);

    let tenant_wide_initialization = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("0")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"conversationType":"group","groupName":"tenant group","clientRequestKey":"c_tenant_group_knowledgebase","initializeKnowledgebase":true}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("explicit tenant-wide initialization should return a response");
    assert_eq!(tenant_wide_initialization.status(), StatusCode::FORBIDDEN);

    let non_group_initialization = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("200001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"conversationId":"c_direct_knowledgebase_rejected","conversationType":"direct","initializeKnowledgebase":true}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("non-group initialization should return a response");
    assert_eq!(non_group_initialization.status(), StatusCode::BAD_REQUEST);

    let explicit_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("200001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"conversationType":"group","groupName":"explicit group","clientRequestKey":"c_explicit_group_knowledgebase","initializeKnowledgebase":true}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("explicit group initialization should return a response");
    assert_eq!(explicit_create.status(), StatusCode::CREATED);
    let explicit_body = explicit_create
        .into_body()
        .collect()
        .await
        .expect("explicit group response body should collect")
        .to_bytes();
    let explicit_json: serde_json::Value = serde_json::from_slice(&explicit_body)
        .expect("explicit group response should be valid json");
    assert_applied_create_conversation_response_shape(&explicit_json, true);

    let first_idempotent_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("200001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"conversationType":"group","groupName":"idempotent group","clientRequestKey":"c_group_knowledgebase_intent"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first idempotent group create should return a response");
    assert_eq!(first_idempotent_create.status(), StatusCode::CREATED);

    let conflicting_intent_retry = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("200001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"conversationType":"group","groupName":"idempotent group","clientRequestKey":"c_group_knowledgebase_intent","initializeKnowledgebase":true}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting initialization intent should return a response");
    assert_eq!(conflicting_intent_retry.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_group_agent_assignments_are_atomic_and_generation_checked_over_http() {
    let app = build_default_test_app();
    let create_body = serde_json::json!({
        "conversationType": "group",
        "groupName": "agent team",
        "clientRequestKey": "c_group_agents_atomic_http",
        "memberUserIds": ["3", "2", "2", "1"],
        "agentAssignments": [
            {"agentId": "agent.im.reviewer", "revisionId": "revision.reviewer.1"},
            {"agentId": "agent.im.writer"}
        ]
    });
    let create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(create_body.to_string()))
                .unwrap(),
        )
        .await
        .expect("group create with agents should return response");
    assert_eq!(create.status(), StatusCode::CREATED);
    let create_json: serde_json::Value = serde_json::from_slice(
        &create
            .into_body()
            .collect()
            .await
            .expect("create body should collect")
            .to_bytes(),
    )
    .expect("create body should be json");
    let conversation_id = response_item(&create_json)["conversationId"]
        .as_str()
        .expect("create should return conversation id")
        .to_owned();

    let get = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/agents"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("agent assignment read should return response");
    assert_eq!(get.status(), StatusCode::OK);
    let get_json: serde_json::Value = serde_json::from_slice(
        &get.into_body()
            .collect()
            .await
            .expect("get body should collect")
            .to_bytes(),
    )
    .expect("get body should be json");
    let assignments = response_item(&get_json);
    assert_eq!(assignments["generation"], 1);
    assert_eq!(assignments["source"], "conversation_override");
    assert_eq!(assignments["agents"].as_array().map(Vec::len), Some(2));

    let members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("initial member read should return response");
    assert_eq!(members.status(), StatusCode::OK);
    let members_json: serde_json::Value = serde_json::from_slice(
        &members
            .into_body()
            .collect()
            .await
            .expect("member body should collect")
            .to_bytes(),
    )
    .expect("member body should be json");
    let mut member_ids = members_json["data"]["items"]
        .as_array()
        .expect("member response should contain items")
        .iter()
        .filter_map(|member| member["principalId"].as_str())
        .collect::<Vec<_>>();
    member_ids.sort_unstable();
    assert_eq!(member_ids, vec!["1", "2", "3"]);

    let replay = app
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
                    serde_json::json!({
                        "conversationType": "group",
                        "groupName": "agent team",
                        "clientRequestKey": "c_group_agents_atomic_http",
                        "memberUserIds": ["2", "1", "3", "2"],
                        "agentAssignments": [
                            {"agentId": "agent.im.reviewer", "revisionId": "revision.reviewer.1"},
                            {"agentId": "agent.im.writer"}
                        ]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("normalized group create retry should return response");
    assert_eq!(replay.status(), StatusCode::CREATED);
    let replay_json: serde_json::Value = serde_json::from_slice(
        &replay
            .into_body()
            .collect()
            .await
            .expect("replay body should collect")
            .to_bytes(),
    )
    .expect("replay body should be json");
    assert_eq!(response_item(&replay_json)["deliveryStatus"], "replayed");

    let conflicting_members = app
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
                    serde_json::json!({
                        "conversationType": "group",
                        "groupName": "agent team",
                        "clientRequestKey": "c_group_agents_atomic_http",
                        "memberUserIds": ["2", "4"],
                        "agentAssignments": [
                            {"agentId": "agent.im.reviewer", "revisionId": "revision.reviewer.1"},
                            {"agentId": "agent.im.writer"}
                        ]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting member create retry should return response");
    assert_eq!(conflicting_members.status(), StatusCode::CONFLICT);

    let update = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/agents"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "expectedGeneration": 1,
                        "agentAssignments": [{"agentId": "agent.im.reviewer"}]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("agent assignment update should return response");
    assert_eq!(update.status(), StatusCode::OK);
    let update_json: serde_json::Value = serde_json::from_slice(
        &update
            .into_body()
            .collect()
            .await
            .expect("update body should collect")
            .to_bytes(),
    )
    .expect("update body should be json");
    assert_eq!(response_item(&update_json)["generation"], 2);

    let stale = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/agents"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "expectedGeneration": 1,
                        "agentAssignments": [{"agentId": "agent.im.writer"}]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("stale update should return response");
    assert_eq!(stale.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_post_message_rejects_invalid_agent_mention_display_text_over_http() {
    let app = build_default_test_app();
    let conversation_id = create_test_group_conversation(
        app.clone(),
        "100001",
        "1",
        "user",
        "c_http_agent_mention_display_text",
    )
    .await;

    for (client_msg_id, display_text) in [
        ("client_agent_mention_blank_label", "   ".to_owned()),
        ("client_agent_mention_oversized_label", "x".repeat(513)),
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!(
                        "/im/v3/api/chat/conversations/{conversation_id}/messages"
                    ))
                    .with_dual_token_tenant("100001")
                    .with_dual_token_user("1")
                    .with_dual_token_actor_kind("user")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "clientMsgId": client_msg_id,
                            "parts": [{
                                "kind": "mention",
                                "targetKind": "agent",
                                "targetId": "agent.im.default",
                                "displayText": display_text,
                                "assignmentGeneration": 1
                            }]
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .expect("invalid agent mention should return a response");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = response
            .into_body()
            .collect()
            .await
            .expect("invalid mention response body should collect")
            .to_bytes();
        let value: serde_json::Value = serde_json::from_slice(&body)
            .expect("invalid mention response should be valid problem json");
        assert_eq!(value["code"], 40001);
        assert!(
            value["detail"]
                .as_str()
                .is_some_and(|detail| detail.contains("displayText"))
        );
    }
}

#[tokio::test]
async fn test_duplicate_post_message_request_is_idempotent_and_conflicting_retry_is_rejected_over_http()
 {
    let app = build_default_test_app();

    let conversation_id =
        create_test_group_conversation(app.clone(), "100001", "1", "user", "c_http_post_retry")
            .await;

    let first_post = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{}/messages",
                    conversation_id
                ))
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
    assert_eq!(first_post.status(), StatusCode::CREATED);
    let first_post_body = first_post
        .into_body()
        .collect()
        .await
        .expect("first post body should collect")
        .to_bytes();
    let first_post_json: serde_json::Value =
        serde_json::from_slice(&first_post_body).expect("first post should be valid json");
    let first_post_item = response_item(&first_post_json);
    assert_eq!(first_post_item["deliveryStatus"], "applied");
    assert_eq!(
        first_post_item["proofVersion"],
        "conversation.message.delivery-proof.v1"
    );

    let duplicate_post = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{}/messages",
                    conversation_id
                ))
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
    assert_eq!(duplicate_post.status(), StatusCode::CREATED);
    let duplicate_post_body = duplicate_post
        .into_body()
        .collect()
        .await
        .expect("duplicate post body should collect")
        .to_bytes();
    let duplicate_post_json: serde_json::Value =
        serde_json::from_slice(&duplicate_post_body).expect("duplicate post should be valid json");
    let duplicate_post_item = response_item(&duplicate_post_json);
    assert_eq!(duplicate_post_item["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_post_item["requestKey"],
        first_post_item["requestKey"]
    );
    assert_eq!(
        duplicate_post_item["messageId"],
        first_post_item["messageId"]
    );
    assert_eq!(
        duplicate_post_item["messageSeq"],
        first_post_item["messageSeq"]
    );
    assert_eq!(duplicate_post_item["eventId"], first_post_item["eventId"]);

    let history = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{}/messages",
                    conversation_id
                ))
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
    assert_eq!(history_json["data"]["highWatermark"], 1);
    assert_eq!(history_json["data"]["items"].as_array().unwrap().len(), 1);

    let conflicting_retry = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{}/messages",
                    conversation_id
                ))
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
    assert_eq!(conflicting_retry_json["code"], 40901);
}

#[tokio::test]
async fn test_post_message_http_and_message_history_get_are_served() {
    let app = build_default_test_app();

    let conversation_id =
        create_test_group_conversation(app.clone(), "100001", "1", "user", "c_history_page_http")
            .await;

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{}/messages",
                    conversation_id
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_history_page_1",
                        "summary":"message 1",
                        "text":"message 1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_response.status(), StatusCode::CREATED);
    let post_body = post_response
        .into_body()
        .collect()
        .await
        .expect("post body should collect")
        .to_bytes();
    let post_json: serde_json::Value =
        serde_json::from_slice(&post_body).expect("post body should be valid json");
    let post_item = response_item(&post_json);

    let message_history_response = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{}/messages?page_size=1",
                    conversation_id
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("message history read request should complete");
    assert_eq!(message_history_response.status(), StatusCode::OK);
    let message_history_body = message_history_response
        .into_body()
        .collect()
        .await
        .expect("message history body should collect")
        .to_bytes();
    let message_history_json: serde_json::Value = serde_json::from_slice(&message_history_body)
        .expect("message history body should be valid json");
    assert_eq!(message_history_json["data"]["pageInfo"]["mode"], "cursor");
    assert_eq!(
        message_history_json["data"]["items"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let message_history_item = &message_history_json["data"]["items"][0];
    assert_eq!(message_history_item["conversationId"], conversation_id);
    assert_eq!(message_history_item["messageId"], post_item["messageId"]);
    assert_eq!(message_history_item["messageSeq"], post_item["messageSeq"]);
    assert_eq!(message_history_item["sender"]["id"], "1");
    assert_eq!(message_history_item["body"]["summary"], "message 1");
    assert_eq!(message_history_item["summary"], "message 1");
    assert_eq!(message_history_item["messageType"], "standard");
    assert_eq!(message_history_item["deliveryMode"], "discrete");
    assert!(message_history_item.get("message").is_none());
}

#[tokio::test]
async fn test_message_history_pages_backward_with_opaque_cursor_under_new_inserts() {
    let app = build_default_test_app();
    let conversation_id = create_test_group_conversation(
        app.clone(),
        "100001",
        "1",
        "user",
        "c_history_backward_http",
    )
    .await;

    for message_seq in 1..=4 {
        post_test_text_message(
            app.clone(),
            conversation_id.as_str(),
            format!("client_history_backward_{message_seq}").as_str(),
            format!("message {message_seq}").as_str(),
        )
        .await;
    }

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages?page_size=2"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("first history page should complete");
    assert_eq!(first_response.status(), StatusCode::OK);
    let first_json: serde_json::Value = serde_json::from_slice(
        &first_response
            .into_body()
            .collect()
            .await
            .expect("first history body should collect")
            .to_bytes(),
    )
    .expect("first history body should be valid json");
    let first_sequences = first_json["data"]["items"]
        .as_array()
        .expect("first history items should be an array")
        .iter()
        .map(|item| {
            item["messageSeq"]
                .as_u64()
                .expect("messageSeq should be u64")
        })
        .collect::<Vec<_>>();
    assert_eq!(first_sequences, [3, 4]);
    assert_eq!(first_json["data"]["pageInfo"]["mode"], "cursor");
    assert_eq!(first_json["data"]["pageInfo"]["hasMore"], true);
    let cursor = first_json["data"]["pageInfo"]["nextCursor"]
        .as_str()
        .expect("first history page should return nextCursor")
        .to_owned();
    assert!(cursor.parse::<u64>().is_err(), "cursor must not be numeric");

    post_test_text_message(
        app.clone(),
        conversation_id.as_str(),
        "client_history_backward_5",
        "message 5",
    )
    .await;

    let second_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages?page_size=2&cursor={cursor}"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second history page should complete");
    assert_eq!(second_response.status(), StatusCode::OK);
    let second_json: serde_json::Value = serde_json::from_slice(
        &second_response
            .into_body()
            .collect()
            .await
            .expect("second history body should collect")
            .to_bytes(),
    )
    .expect("second history body should be valid json");
    let second_sequences = second_json["data"]["items"]
        .as_array()
        .expect("second history items should be an array")
        .iter()
        .map(|item| {
            item["messageSeq"]
                .as_u64()
                .expect("messageSeq should be u64")
        })
        .collect::<Vec<_>>();
    assert_eq!(second_sequences, [1, 2]);
    assert_eq!(second_json["data"]["pageInfo"]["hasMore"], false);
    assert!(second_json["data"]["pageInfo"]["nextCursor"].is_null());
}

#[tokio::test]
async fn test_message_history_rejects_legacy_aliases_numeric_cursor_and_oversized_pages() {
    let app = build_default_test_app();
    let conversation_id = create_test_group_conversation(
        app.clone(),
        "100001",
        "1",
        "user",
        "c_history_invalid_paging_http",
    )
    .await;

    for query in [
        "afterSeq=0&page_size=20",
        "after_seq=0&page_size=20",
        "pageSize=20",
        "limit=20",
        "cursor=8&page_size=20",
        "page_size=201",
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!(
                        "/im/v3/api/chat/conversations/{conversation_id}/messages?{query}"
                    ))
                    .with_dual_token_tenant("100001")
                    .with_dual_token_user("1")
                    .with_dual_token_actor_kind("user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("invalid message history pagination should return a response");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST, "query: {query}");
        let body = response
            .into_body()
            .collect()
            .await
            .expect("invalid pagination body should collect")
            .to_bytes();
        let value: serde_json::Value = serde_json::from_slice(&body)
            .expect("invalid pagination response should be valid json");
        assert_eq!(value["code"], 40003, "query: {query}");
    }
}

#[tokio::test]
async fn test_create_conversation_rejects_unknown_type_over_http() {
    let app = build_default_test_app();

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
    assert_eq!(value["code"], 40001);
}

#[tokio::test]
async fn test_create_conversation_rejects_unknown_user_creator_over_http() {
    let app = build_default_test_app_with_principal_directory(Arc::new(
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
                        "groupName":"test group",
                        "clientRequestKey":"c_unknown_creator_http",
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
    assert_eq!(value["code"], 40001);
}

#[tokio::test]
async fn test_group_create_rejects_unknown_initial_member_without_partial_commit_over_http() {
    let app = build_default_test_app_with_principal_directory(Arc::new(
        StrictKnownPrincipalDirectory::new(&["1", "2"]),
    ));
    let request = || {
        Request::builder()
            .method("POST")
            .uri("/im/v3/api/chat/conversations")
            .with_dual_token_tenant("100001")
            .with_dual_token_user("1")
            .with_dual_token_actor_kind("user")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "conversationType": "group",
                    "groupName": "strict initial members",
                    "clientRequestKey": "c_strict_initial_members",
                    "memberUserIds": ["2", "missing-user"]
                })
                .to_string(),
            ))
            .unwrap()
    };

    let rejected = app.clone().oneshot(request()).await.unwrap();
    assert_eq!(rejected.status(), StatusCode::BAD_REQUEST);
    let rejected_json: serde_json::Value =
        serde_json::from_slice(&rejected.into_body().collect().await.unwrap().to_bytes()).unwrap();
    assert_eq!(rejected_json["code"], 40001);

    let retry = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "conversationType": "group",
                        "groupName": "strict initial members",
                        "clientRequestKey": "c_strict_initial_members",
                        "memberUserIds": ["2"]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(retry.status(), StatusCode::CREATED);
    let retry_json: serde_json::Value =
        serde_json::from_slice(&retry.into_body().collect().await.unwrap().to_bytes()).unwrap();
    assert_eq!(response_item(&retry_json)["deliveryStatus"], "applied");
}

#[tokio::test]
async fn test_create_conversation_rejects_oversized_conversation_id_over_http() {
    let app = build_default_test_app();
    let request_body = serde_json::json!({
        "conversationId": "c".repeat(2048),
        "conversationType": "direct",
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
    assert_eq!(value["code"], 41301);
    assert!(
        value["detail"]
            .as_str()
            .expect("message should be present")
            .contains("conversationId")
    );
}

#[tokio::test]
async fn test_generic_create_rejects_reserved_special_types_over_http() {
    let app = build_default_test_app();

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
        assert_eq!(value["code"], 40001);
    }
}

#[tokio::test]
async fn test_group_create_preserves_actor_kind_over_http() {
    let app = build_default_test_app();

    let conversation_id = create_test_group_conversation(
        app.clone(),
        "100001",
        "svc_ops",
        "system",
        "c_group_actor_http",
    )
    .await;

    let list_members = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members"
                ))
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
    assert_eq!(value["data"]["items"][0]["principalId"], "svc_ops");
    assert_eq!(value["data"]["items"][0]["principalKind"], "system");
}

#[tokio::test]
async fn test_create_agent_dialog_rejects_non_standard_agent_id_over_http() {
    let app = build_default_test_app();

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
    assert_eq!(value["code"], 40001);
}

#[tokio::test]
async fn test_create_agent_dialog_over_http() {
    let app = build_default_test_app();

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
                        "agentId":"agent.demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create agent dialog request should succeed");
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("create body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(&create_body).expect("create response should be valid json");
    let create_item = response_item(&create_json);
    let conversation_id = create_item["conversationId"]
        .as_str()
        .expect("create response should include canonical conversation id");
    assert!(conversation_id.starts_with("a_"));

    let list_members = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members"
                ))
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
    assert_eq!(value["data"]["items"].as_array().unwrap().len(), 2);
    assert!(
        value["data"]["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["principalId"] == "1" && item["principalKind"] == "user")
    );
    assert!(
        value["data"]["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["principalId"] == "agent.demo" && item["principalKind"] == "agent")
    );
}

#[tokio::test]
async fn test_duplicate_create_agent_dialog_request_is_idempotent_and_conflicting_retry_is_rejected_over_http()
 {
    let app = build_default_test_app();

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
                        "agentId":"agent.demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first agent dialog create should return response");
    assert_eq!(first_create.status(), StatusCode::CREATED);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first agent dialog create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value = serde_json::from_slice(&first_create_body)
        .expect("first agent dialog create should be valid json");
    let first_create_item = response_item(&first_create_json);
    let conversation_id = first_create_item["conversationId"]
        .as_str()
        .expect("first create should return canonical conversation id");
    assert_eq!(first_create_item["deliveryStatus"], "applied");
    assert_eq!(
        first_create_item["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert!(first_create_item["requestKey"].is_string());

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
                        "agentId":"agent.demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate agent dialog create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::CREATED);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate agent dialog create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate agent dialog create should be valid json");
    let duplicate_create_item = response_item(&duplicate_create_json);
    assert_eq!(duplicate_create_item["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_item["requestKey"],
        first_create_item["requestKey"]
    );
    assert_eq!(
        duplicate_create_item["eventId"],
        first_create_item["eventId"]
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
                .body(Body::from(format!(
                    r#"{{
                        "conversationId":"{conversation_id}",
                        "agentId":"agent.other"
                    }}"#
                )))
                .unwrap(),
        )
        .await
        .expect("conflicting agent dialog create should return response");
    assert_eq!(conflicting_retry.status(), StatusCode::BAD_REQUEST);
    let conflicting_retry_body = conflicting_retry
        .into_body()
        .collect()
        .await
        .expect("conflicting agent dialog create body should collect")
        .to_bytes();
    let conflicting_retry_json: serde_json::Value = serde_json::from_slice(&conflicting_retry_body)
        .expect("conflicting agent dialog create should be valid json");
    assert_eq!(conflicting_retry_json["code"], 40001);
}

#[tokio::test]
async fn test_create_agent_dialog_rejects_non_user_actor_over_http() {
    let app = build_default_test_app();

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
    assert_eq!(value["code"], 40301);
}

#[tokio::test]
async fn test_create_agent_dialog_rejects_unknown_user_requester_over_http() {
    let app = build_default_test_app_with_principal_directory(Arc::new(
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
    assert_eq!(value["code"], 40001);
}

#[tokio::test]
async fn test_create_agent_handoff_over_http() {
    let app = build_default_test_app();

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
    assert_eq!(create_response.status(), StatusCode::CREATED);

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
    assert_eq!(value["data"]["items"].as_array().unwrap().len(), 2);
    assert!(
        value["data"]["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["principalId"] == "ag_source" && item["principalKind"] == "agent")
    );
    assert!(
        value["data"]["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["principalId"] == "1" && item["principalKind"] == "user")
    );
}

#[tokio::test]
async fn test_create_agent_handoff_rejects_non_agent_actor_over_http() {
    let app = build_default_test_app();

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
    assert_eq!(value["code"], 40301);
}

#[tokio::test]
async fn test_create_agent_handoff_rejects_unknown_user_target_over_http() {
    let app = build_default_test_app_with_principal_directory(Arc::new(
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
    assert_eq!(value["code"], 40001);
}

#[tokio::test]
async fn test_duplicate_create_agent_handoff_request_is_idempotent_and_conflicting_retry_is_rejected_over_http()
 {
    let app = build_default_test_app();

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
    assert_eq!(first_create.status(), StatusCode::CREATED);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first agent handoff create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value = serde_json::from_slice(&first_create_body)
        .expect("first agent handoff create should be valid json");
    let first_create_item = response_item(&first_create_json);
    assert_eq!(first_create_item["deliveryStatus"], "applied");
    assert_eq!(
        first_create_item["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_create_item["requestKey"],
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
    assert_eq!(duplicate_create.status(), StatusCode::CREATED);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate agent handoff create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate agent handoff create should be valid json");
    let duplicate_create_item = response_item(&duplicate_create_json);
    assert_eq!(duplicate_create_item["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_item["requestKey"],
        first_create_item["requestKey"]
    );
    assert_eq!(
        duplicate_create_item["eventId"],
        first_create_item["eventId"]
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
    assert_eq!(conflicting_retry_json["code"], 40901);
}

#[tokio::test]
async fn test_agent_handoff_target_can_post_over_http() {
    let app = build_default_test_app();

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
    assert_eq!(create_response.status(), StatusCode::CREATED);

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

    assert_eq!(post_response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_agent_handoff_accept_resolve_close_over_http() {
    let app = build_default_test_app();

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
    assert_eq!(create_response.status(), StatusCode::CREATED);

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
    assert_eq!(get_open_json["data"]["status"], "open");

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
    assert_eq!(accept_json["data"]["status"], "accepted");
    assert_eq!(accept_json["data"]["acceptedBy"]["id"], "1");

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
    assert_eq!(resolve_json["data"]["status"], "resolved");

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
    assert_eq!(close_json["data"]["status"], "closed");

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
    assert_eq!(post_after_close_json["code"], 40901);
}

#[tokio::test]
async fn test_agent_handoff_accept_rejects_non_target_actor_over_http() {
    let app = build_default_test_app();

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
    assert_eq!(create_response.status(), StatusCode::CREATED);

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
    let app = build_default_test_app();

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
    assert_eq!(create_response.status(), StatusCode::CREATED);

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
    assert_eq!(value["data"]["items"].as_array().unwrap().len(), 2);
    assert!(
        value["data"]["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["principalId"] == "svc_ops" && item["principalKind"] == "system")
    );
    assert!(
        value["data"]["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["principalId"] == "1" && item["principalKind"] == "user")
    );
}

#[tokio::test]
async fn test_chat_room_create_enter_leave_over_http() {
    let app = build_default_test_app();

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
                        "roomId":"room_chat_http",
                        "roomKind":"chat"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create chat room request should succeed");
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let created: serde_json::Value =
        serde_json::from_slice(&create_body).expect("room create response should be json");
    let conversation_id = response_item(&created)["conversationId"]
        .as_str()
        .expect("room create should return a conversation id")
        .to_owned();
    assert!(conversation_id.starts_with("r_"));

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
    let view_item = response_item(&view);
    assert_eq!(view_item["roomKind"], "chat");
    assert_eq!(view_item["conversationId"], conversation_id);
    assert_eq!(view_item["activeMemberCount"], 1);

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
    let app = build_default_test_app();

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
    assert_eq!(value["code"], 40301);
}

#[tokio::test]
async fn test_create_system_channel_rejects_unknown_user_subscriber_over_http() {
    let app = build_default_test_app_with_principal_directory(Arc::new(
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
    assert_eq!(value["code"], 40001);
}

#[tokio::test]
async fn test_duplicate_create_system_channel_request_is_idempotent_and_conflicting_retry_is_rejected_over_http()
 {
    let app = build_default_test_app();

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
    assert_eq!(first_create.status(), StatusCode::CREATED);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first system channel create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value = serde_json::from_slice(&first_create_body)
        .expect("first system channel create should be valid json");
    let first_create_item = response_item(&first_create_json);
    assert_eq!(first_create_item["deliveryStatus"], "applied");
    assert_eq!(
        first_create_item["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_create_item["requestKey"],
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
    assert_eq!(duplicate_create.status(), StatusCode::CREATED);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate system channel create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate system channel create should be valid json");
    let duplicate_create_item = response_item(&duplicate_create_json);
    assert_eq!(duplicate_create_item["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_item["requestKey"],
        first_create_item["requestKey"]
    );
    assert_eq!(
        duplicate_create_item["eventId"],
        first_create_item["eventId"]
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
    assert_eq!(conflicting_retry_json["code"], 40901);
}

#[tokio::test]
async fn test_system_channel_subscriber_cannot_post_over_http() {
    let app = build_default_test_app();

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
    assert_eq!(create_response.status(), StatusCode::CREATED);

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
    assert_eq!(value["code"], 40301);
}

#[tokio::test]
async fn test_system_channel_publisher_must_use_dedicated_publish_route_over_http() {
    let app = build_default_test_app();

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
    assert_eq!(create_response.status(), StatusCode::CREATED);

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
    assert_eq!(value["code"], 40301);
}

#[tokio::test]
async fn test_system_channel_dedicated_publish_over_http() {
    let app = build_default_test_app();

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
    assert_eq!(create_response.status(), StatusCode::CREATED);

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
    let item = response_item(&value);
    assert_eq!(item["messageSeq"], 1);

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
    assert_eq!(subscriber_value["code"], 40301);
}

#[tokio::test]
async fn test_post_message_accepts_structured_parts_over_http() {
    let app = build_default_test_app();

    let conversation_id =
        create_test_group_conversation(app.clone(), "100001", "1", "user", "c_media_http").await;

    let post_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/im/v3/api/chat/conversations/{conversation_id}/messages"))
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

    assert_eq!(post_response.status(), StatusCode::CREATED);
    let body = post_response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    let item = response_item(&value);

    assert_eq!(item["messageSeq"], 1);
    assert_eq!(item["messageId"], format!("msg_{conversation_id}_1"));
}

#[tokio::test]
async fn test_post_message_rejects_oversized_text_over_http() {
    let app = build_default_test_app();

    let conversation_id =
        create_test_group_conversation(app.clone(), "100001", "1", "user", "c_http_oversized_text")
            .await;

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
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
    assert_eq!(value["code"], 41301);
    assert!(
        value["detail"]
            .as_str()
            .expect("message should be present")
            .contains("messageBody")
    );
}

#[tokio::test]
async fn test_post_message_rejects_oversized_sender_session_id_over_http() {
    let app = build_default_test_app();

    let conversation_id = create_test_group_conversation(
        app.clone(),
        "100001",
        "1",
        "user",
        "c_http_oversized_sender_session",
    )
    .await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
    assert_eq!(value["code"], 41301);
    assert!(
        value["detail"]
            .as_str()
            .expect("message should be present")
            .contains("senderSessionId")
    );
}

#[tokio::test]
async fn test_add_member_rejects_oversized_attributes_over_http() {
    let app = build_default_test_app();

    let conversation_id = create_test_group_conversation(
        app.clone(),
        "100001",
        "1",
        "user",
        "c_member_attributes_http",
    )
    .await;

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
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members/add"
                ))
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
    assert_eq!(value["code"], 41301);
    assert!(
        value["detail"]
            .as_str()
            .expect("message should be present")
            .contains("memberAttributes")
    );
}

#[tokio::test]
async fn test_add_member_rejects_unknown_user_principal_over_http() {
    let app = build_default_test_app_with_principal_directory(Arc::new(
        StrictKnownPrincipalDirectory::new(&["1"]),
    ));

    let conversation_id = create_test_group_conversation(
        app.clone(),
        "100001",
        "1",
        "user",
        "c_members_unknown_principal_http",
    )
    .await;

    let add_member_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members/add"
                ))
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
    assert_eq!(value["code"], 40001);
}

#[tokio::test]
async fn test_conversation_member_endpoints_manage_roster_over_http() {
    let app = build_default_test_app();

    let conversation_id =
        create_test_group_conversation(app.clone(), "100001", "1", "user", "c_members_http").await;
    let added_member_id = format!("cm_{conversation_id}_user_1043");

    let list_initial_members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members"
                ))
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
    assert_eq!(initial_json["data"]["items"][0]["principalId"], "1");
    assert_eq!(initial_json["data"]["items"][0]["role"], "owner");

    let add_member_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members/add"
                ))
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
    assert_eq!(add_member_json["data"]["memberId"], added_member_id);
    assert_eq!(add_member_json["data"]["state"], "joined");

    let list_after_add = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members"
                ))
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
    assert_eq!(
        list_after_add_json["data"]["items"]
            .as_array()
            .unwrap()
            .len(),
        2
    );

    let remove_member_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members/remove"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(format!(r#"{{"memberId":"{added_member_id}"}}"#)))
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
    assert_eq!(remove_member_json["data"]["state"], "removed");

    let list_after_remove = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members"
                ))
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
    assert_eq!(
        list_after_remove_json["data"]["items"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        list_after_remove_json["data"]["items"][0]["principalId"],
        "1"
    );
}

#[tokio::test]
async fn test_group_member_governance_over_http_rejects_actor_kind_mismatch() {
    let app = build_default_test_app();

    let conversation_id = create_test_group_conversation(
        app.clone(),
        "100001",
        "1",
        "user",
        "c_members_actor_kind_http",
    )
    .await;

    let add_member_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members/add"
                ))
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
    assert_eq!(add_member_json["code"], 40301);
}

#[tokio::test]
async fn test_group_member_can_leave_roster_over_http() {
    let app = build_default_test_app();

    let conversation_id =
        create_test_group_conversation(app.clone(), "100001", "1", "user", "c_members_leave_http")
            .await;

    let add_member_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members/add"
                ))
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
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members/leave"
                ))
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
    assert_eq!(leave_json["data"]["state"], "left");

    let list_after_leave = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members"
                ))
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
    assert_eq!(
        list_after_leave_json["data"]["items"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        list_after_leave_json["data"]["items"][0]["principalId"],
        "1"
    );
}

#[tokio::test]
async fn test_group_owner_transfer_over_http() {
    let app = build_default_test_app();

    let conversation_id = create_test_group_conversation(
        app.clone(),
        "100001",
        "1",
        "user",
        "c_members_transfer_http",
    )
    .await;
    let added_member_id = format!("cm_{conversation_id}_user_1043");

    let add_member_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members/add"
                ))
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
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members/transfer_owner"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(format!(r#"{{"memberId":"{added_member_id}"}}"#)))
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
    assert_eq!(transfer_json["data"]["previousOwner"]["role"], "admin");
    assert_eq!(transfer_json["data"]["newOwner"]["role"], "owner");

    let leave_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members/leave"
                ))
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
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members"
                ))
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
    assert_eq!(list_json["data"]["items"].as_array().unwrap().len(), 1);
    assert_eq!(list_json["data"]["items"][0]["principalId"], "1043");
    assert_eq!(list_json["data"]["items"][0]["role"], "owner");
}

#[tokio::test]
async fn test_change_member_role_over_http() {
    let app = build_default_test_app();

    let conversation_id =
        create_test_group_conversation(app.clone(), "100001", "1", "user", "c_members_role_http")
            .await;
    let added_member_id = format!("cm_{conversation_id}_user_1043");

    let add_member_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members/add"
                ))
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
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members/change_role"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{"memberId":"{added_member_id}","role":"admin"}}"#
                )))
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
    assert_eq!(change_role_json["data"]["previousMember"]["role"], "member");
    assert_eq!(change_role_json["data"]["updatedMember"]["role"], "admin");

    let list_response = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members"
                ))
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
    let member = list_json["data"]["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["principalId"] == "1043")
        .expect("member should exist");
    assert_eq!(member["role"], "admin");
}

#[tokio::test]
async fn test_list_members_returns_bounded_cursor_window_over_http() {
    let app = build_default_test_app();

    let conversation_id =
        create_test_group_conversation(app.clone(), "100001", "1", "user", "c_members_window_http")
            .await;

    for principal_id in ["1045", "1046", "1047"] {
        let add_member_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!(
                        "/im/v3/api/chat/conversations/{conversation_id}/members/add"
                    ))
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
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members?page_size=2"
                ))
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
    assert_eq!(
        first_page_json["data"]["items"].as_array().unwrap().len(),
        2
    );
    assert_eq!(first_page_json["data"]["pageInfo"]["mode"], "cursor");
    assert_eq!(first_page_json["data"]["pageInfo"]["hasMore"], true);
    let next_cursor = first_page_json["data"]["pageInfo"]["nextCursor"]
        .as_str()
        .expect("first member page should include nextCursor")
        .to_owned();
    assert_ne!(
        next_cursor, "2",
        "member cursors must be opaque and must not expose the in-process offset"
    );
    assert!(
        next_cursor.parse::<usize>().is_err(),
        "member cursors must not be numeric offset aliases"
    );

    let second_page_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members?page_size=2&cursor={next_cursor}"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second member page should return response");
    let second_page_status = second_page_response.status();
    let second_page_body = second_page_response
        .into_body()
        .collect()
        .await
        .expect("second page body should collect")
        .to_bytes();
    let second_page_json: serde_json::Value =
        serde_json::from_slice(&second_page_body).expect("second page should be valid json");
    assert_eq!(
        second_page_status,
        StatusCode::OK,
        "unexpected second member page response: {second_page_json}"
    );
    assert_eq!(
        second_page_json["data"]["items"].as_array().unwrap().len(),
        2
    );
    assert_eq!(second_page_json["data"]["pageInfo"]["mode"], "cursor");
    assert_eq!(second_page_json["data"]["pageInfo"]["hasMore"], false);
    assert!(second_page_json["data"]["pageInfo"]["nextCursor"].is_null());

    let mut principal_ids = first_page_json["data"]["items"]
        .as_array()
        .unwrap()
        .iter()
        .chain(second_page_json["data"]["items"].as_array().unwrap().iter())
        .map(|item| item["principalId"].as_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    principal_ids.sort();
    assert_eq!(principal_ids, ["1", "1045", "1046", "1047"]);

    let invalid_limit_response = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members?page_size=0"
                ))
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
    assert_eq!(invalid_limit_json["code"], 40001);
}

#[tokio::test]
async fn test_read_cursor_endpoints_expose_unread_progress_over_http() {
    let app = build_default_test_app();

    let conversation_id =
        create_test_group_conversation(app.clone(), "100001", "1", "user", "c_cursor_http").await;

    let add_member_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members/add"
                ))
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
                    .uri(format!(
                        "/im/v3/api/chat/conversations/{conversation_id}/messages"
                    ))
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
        assert_eq!(post_response.status(), StatusCode::CREATED);
    }

    let initial_cursor_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/read_cursor"
                ))
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
    let initial_cursor_item = response_item(&initial_cursor_json);
    assert_eq!(initial_cursor_item["readSeq"], 0);
    assert_eq!(initial_cursor_item["unreadCount"], 2);

    let update_cursor_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/read_cursor"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "readSeq": 1,
                        "lastReadMessageId":"msg_{conversation_id}_1"
                    }}"#,
                )))
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
    let update_cursor_item = response_item(&update_cursor_json);
    assert_eq!(update_cursor_item["readSeq"], 1);
    assert_eq!(update_cursor_item["unreadCount"], 1);
}

#[tokio::test]
async fn test_read_cursor_over_http_rejects_actor_kind_mismatch() {
    let app = build_default_test_app();

    let conversation_id = create_test_group_conversation(
        app.clone(),
        "100001",
        "1",
        "user",
        "c_cursor_actor_kind_http",
    )
    .await;

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
    assert_eq!(post_response.status(), StatusCode::CREATED);

    let update_cursor_response = app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/read_cursor"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("agent")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "readSeq": 1,
                        "lastReadMessageId":"msg_{conversation_id}_1"
                    }}"#,
                )))
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
    assert_eq!(update_cursor_json["code"], 40301);
}

#[tokio::test]
async fn test_edit_and_recall_message_over_http() {
    let app = build_default_test_app();

    let conversation_id =
        create_test_group_conversation(app.clone(), "100001", "1", "user", "c_edit_http").await;
    let message_id = format!("msg_{conversation_id}_1");

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
    assert_eq!(post_response.status(), StatusCode::CREATED);

    let edit_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/im/v3/api/chat/messages/{message_id}/edit"))
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
    let edit_item = response_item(&edit_json);
    assert_eq!(edit_item["messageId"], message_id);
    assert_eq!(edit_item["messageSeq"], 1);

    let recall_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/im/v3/api/chat/messages/{message_id}/recall"))
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
    let recall_item = response_item(&recall_json);
    assert_eq!(recall_item["messageId"], message_id);
    assert_eq!(recall_item["messageSeq"], 1);
}

#[tokio::test]
async fn test_reaction_and_pin_message_over_http() {
    let app = build_default_test_app();

    let conversation_id =
        create_test_group_conversation(app.clone(), "100001", "1", "user", "c_reaction_pin_http")
            .await;
    let message_id = format!("msg_{conversation_id}_1");

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
    assert_eq!(post_response.status(), StatusCode::CREATED);

    let reaction_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/im/v3/api/chat/messages/{message_id}/reactions"))
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
    assert_eq!(reaction_response.status(), StatusCode::CREATED);
    let reaction_body = reaction_response
        .into_body()
        .collect()
        .await
        .expect("reaction body should collect")
        .to_bytes();
    let reaction_json: serde_json::Value =
        serde_json::from_slice(&reaction_body).expect("reaction response should be valid json");
    let reaction_item = response_item(&reaction_json);
    assert_eq!(reaction_item["messageId"], message_id);
    assert_eq!(reaction_item["messageSeq"], 1);
    assert_eq!(reaction_item["reactionKey"], "thumbs_up");
    assert_eq!(reaction_item["changed"], true);

    let pin_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/im/v3/api/chat/messages/{message_id}/pin"))
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
    let pin_item = response_item(&pin_json);
    assert_eq!(pin_item["messageId"], message_id);
    assert_eq!(pin_item["messageSeq"], 1);
    assert_eq!(pin_item["changed"], true);

    let unpin_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/im/v3/api/chat/messages/{message_id}/unpin"))
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
                .uri(format!(
                    "/im/v3/api/chat/messages/{message_id}/reactions/remove"
                ))
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
    let app = build_default_test_app();

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
                        "groupName":"test group",
                        "clientRequestKey":"c_policy_http",
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
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("create body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(&create_body).expect("create response should be valid json");
    let conversation_id = response_item(&create_json)["conversationId"]
        .as_str()
        .expect("create response should include canonical conversation id")
        .to_string();
    let message_id = format!("msg_{conversation_id}_1");

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
    assert_eq!(post_response.status(), StatusCode::CREATED);

    let reaction_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/im/v3/api/chat/messages/{message_id}/reactions"))
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
    assert_eq!(reaction_response.status(), StatusCode::CREATED);

    let pin_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/im/v3/api/chat/messages/{message_id}/pin"))
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
    assert_eq!(pin_json["code"], 40301);
}

#[tokio::test]
async fn test_joined_history_visibility_blocks_non_member_history_reads_over_http() {
    let app = build_default_test_app();

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
                        "groupName":"test group",
                        "clientRequestKey":"c_history_joined_http",
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
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("create body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(&create_body).expect("create response should be valid json");
    let conversation_id = response_item(&create_json)["conversationId"]
        .as_str()
        .expect("create response should include canonical conversation id")
        .to_string();

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
    assert_eq!(post_response.status(), StatusCode::CREATED);

    let history_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
    assert_eq!(history_json["code"], 40301);
}

#[tokio::test]
async fn test_world_readable_history_visibility_allows_non_member_history_reads_over_http() {
    let app = build_default_test_app();

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
                        "groupName":"test group",
                        "clientRequestKey":"c_history_world_http",
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
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("create body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(&create_body).expect("create response should be valid json");
    let conversation_id = response_item(&create_json)["conversationId"]
        .as_str()
        .expect("create response should include canonical conversation id")
        .to_string();

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
    assert_eq!(post_response.status(), StatusCode::CREATED);

    let history_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
        history_json["data"]["items"][0]["messageId"],
        format!("msg_{conversation_id}_1")
    );
    assert_eq!(
        history_json["data"]["items"][0]["body"]["summary"],
        "hello world"
    );
}

#[tokio::test]
async fn test_bind_direct_chat_conversation_over_http_and_query_binding() {
    let app = build_default_test_app();

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
    assert_eq!(bind_response.status(), StatusCode::CREATED);
    let bind_body = bind_response
        .into_body()
        .collect()
        .await
        .expect("bind body should collect")
        .to_bytes();
    let bind_json: serde_json::Value =
        serde_json::from_slice(&bind_body).expect("bind response should be valid json");
    let bind_item = response_item(&bind_json);
    let conversation_id = bind_item["conversationId"]
        .as_str()
        .expect("direct chat response should include canonical conversationId")
        .to_owned();

    let binding_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/binding"
                ))
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
    assert_eq!(binding_json["data"]["conversationId"], conversation_id);
    assert_eq!(binding_json["data"]["businessType"], "direct_chat");
    assert!(
        binding_json["data"]["businessId"].is_string(),
        "binding response should include canonical businessId"
    );

    let members_response = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("actor_a")
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
    assert_eq!(members_json["data"]["items"].as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn test_user_participant_can_bind_direct_chat_conversation_over_http() {
    let app = build_default_test_app();

    let bind_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/direct_chats/bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("actor_a")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "leftActorId":"actor_a",
                        "leftActorKind":"user",
                        "rightActorId":"actor_b",
                        "rightActorKind":"user"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("participant direct chat binding request should return response");
    assert_eq!(bind_response.status(), StatusCode::CREATED);
    let bind_body = bind_response
        .into_body()
        .collect()
        .await
        .expect("participant bind body should collect")
        .to_bytes();
    let bind_json: serde_json::Value =
        serde_json::from_slice(&bind_body).expect("participant bind should be valid json");
    let bind_item = response_item(&bind_json);
    let conversation_id = bind_item["conversationId"]
        .as_str()
        .expect("participant bind response should include canonical conversationId")
        .to_owned();

    let members_response = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("actor_a")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("participant list members request should return response");
    assert_eq!(members_response.status(), StatusCode::OK);
    let members_body = members_response
        .into_body()
        .collect()
        .await
        .expect("participant members body should collect")
        .to_bytes();
    let members_json: serde_json::Value = serde_json::from_slice(&members_body)
        .expect("participant members response should be valid json");
    assert_eq!(members_json["data"]["items"].as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn test_bind_direct_chat_conversation_rejects_unknown_user_participant_over_http() {
    let app = build_default_test_app_with_principal_directory(Arc::new(
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
    assert_eq!(bind_json["code"], 40001);
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
    let app = build_default_test_app_with_principal_directory(Arc::new(principal_directory));

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
    assert_eq!(bind_json["code"], 40001);

    let _ = fs::remove_file(catalog_path);
}

#[tokio::test]
async fn test_duplicate_bind_direct_chat_conversation_request_is_idempotent_and_conflicting_retry_is_rejected_over_http()
 {
    let app = build_default_test_app();

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
    assert_eq!(first_bind.status(), StatusCode::CREATED);
    let first_bind_body = first_bind
        .into_body()
        .collect()
        .await
        .expect("first bind body should collect")
        .to_bytes();
    let first_bind_json: serde_json::Value =
        serde_json::from_slice(&first_bind_body).expect("first bind should be valid json");
    let first_bind_item = response_item(&first_bind_json);
    assert_eq!(first_bind_item["deliveryStatus"], "applied");
    assert_eq!(
        first_bind_item["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert!(first_bind_item["requestKey"].is_string());

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
    assert_eq!(duplicate_bind.status(), StatusCode::CREATED);
    let duplicate_bind_body = duplicate_bind
        .into_body()
        .collect()
        .await
        .expect("duplicate bind body should collect")
        .to_bytes();
    let duplicate_bind_json: serde_json::Value =
        serde_json::from_slice(&duplicate_bind_body).expect("duplicate bind should be valid json");
    let duplicate_bind_item = response_item(&duplicate_bind_json);
    assert_eq!(duplicate_bind_item["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_bind_item["requestKey"],
        first_bind_item["requestKey"]
    );
    assert_eq!(duplicate_bind_item["eventId"], first_bind_item["eventId"]);

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
    assert_eq!(conflicting_bind.status(), StatusCode::BAD_REQUEST);
    let conflicting_bind_body = conflicting_bind
        .into_body()
        .collect()
        .await
        .expect("conflicting bind body should collect")
        .to_bytes();
    let conflicting_bind_json: serde_json::Value = serde_json::from_slice(&conflicting_bind_body)
        .expect("conflicting bind should be valid json");
    assert_eq!(conflicting_bind_json["code"], 40001);
}

#[tokio::test]
async fn test_create_thread_conversation_over_http_and_query_binding() {
    let app = build_default_test_app();

    let parent_conversation_id =
        create_test_group_conversation(app.clone(), "100001", "1", "user", "c_parent_thread_http")
            .await;

    let post_root_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{parent_conversation_id}/messages"
                ))
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
    assert_eq!(post_root_response.status(), StatusCode::CREATED);
    let post_root_body = post_root_response
        .into_body()
        .collect()
        .await
        .expect("post root body should collect")
        .to_bytes();
    let post_root_json: serde_json::Value =
        serde_json::from_slice(&post_root_body).expect("post root response should be valid json");
    let post_root_item = response_item(&post_root_json);

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
                        "parentConversationId":"{parent_conversation_id}",
                        "rootMessageId":"{}"
                    }}"#,
                    post_root_item["messageId"].as_str().unwrap()
                )))
                .unwrap(),
        )
        .await
        .expect("create thread request should return response");
    assert_eq!(create_thread_response.status(), StatusCode::CREATED);
    let create_thread_body = create_thread_response
        .into_body()
        .collect()
        .await
        .expect("create thread body should collect")
        .to_bytes();
    let create_thread_json: serde_json::Value = serde_json::from_slice(&create_thread_body)
        .expect("create thread response should be valid json");
    let create_thread_item = response_item(&create_thread_json);
    assert_eq!(create_thread_item["conversationId"], "c_thread_http");

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
    assert_eq!(binding_json["data"]["conversationId"], "c_thread_http");
    assert_eq!(binding_json["data"]["businessType"], "thread");
    assert_eq!(
        binding_json["data"]["businessId"],
        post_root_item["messageId"]
    );

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
        members_json["data"]["items"][0]["attributes"]["parentConversationId"],
        parent_conversation_id
    );
    assert_eq!(
        members_json["data"]["items"][0]["attributes"]["rootMessageId"],
        post_root_item["messageId"]
    );
    assert_eq!(
        members_json["data"]["items"][0]["attributes"]["threadRole"],
        "owner"
    );
}

#[tokio::test]
async fn test_duplicate_create_thread_conversation_request_is_idempotent_and_conflicting_retry_is_rejected_over_http()
 {
    let app = build_default_test_app();

    let parent_conversation_id = create_test_group_conversation(
        app.clone(),
        "100001",
        "1",
        "user",
        "c_parent_thread_retry_http",
    )
    .await;

    let first_root_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{parent_conversation_id}/messages"
                ))
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
    assert_eq!(first_root_response.status(), StatusCode::CREATED);
    let first_root_body = first_root_response
        .into_body()
        .collect()
        .await
        .expect("first root body should collect")
        .to_bytes();
    let first_root_json: serde_json::Value =
        serde_json::from_slice(&first_root_body).expect("first root response should be valid json");
    let first_root_item = response_item(&first_root_json);

    let second_root_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{parent_conversation_id}/messages"
                ))
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
    assert_eq!(second_root_response.status(), StatusCode::CREATED);
    let second_root_body = second_root_response
        .into_body()
        .collect()
        .await
        .expect("second root body should collect")
        .to_bytes();
    let second_root_json: serde_json::Value = serde_json::from_slice(&second_root_body)
        .expect("second root response should be valid json");
    let second_root_item = response_item(&second_root_json);

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
                        "parentConversationId":"{parent_conversation_id}",
                        "rootMessageId":"{}"
                    }}"#,
                    first_root_item["messageId"].as_str().unwrap()
                )))
                .unwrap(),
        )
        .await
        .expect("first thread create should return response");
    assert_eq!(first_create.status(), StatusCode::CREATED);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first thread create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value = serde_json::from_slice(&first_create_body)
        .expect("first thread create should be valid json");
    let first_create_item = response_item(&first_create_json);
    assert_eq!(first_create_item["deliveryStatus"], "applied");
    assert_eq!(
        first_create_item["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_create_item["requestKey"],
        "6#1000014#user1#113#create-thread19#c_thread_retry_http"
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
                        "parentConversationId":"{parent_conversation_id}",
                        "rootMessageId":"{}"
                    }}"#,
                    first_root_item["messageId"].as_str().unwrap()
                )))
                .unwrap(),
        )
        .await
        .expect("duplicate thread create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::CREATED);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate thread create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate thread create should be valid json");
    let duplicate_create_item = response_item(&duplicate_create_json);
    assert_eq!(duplicate_create_item["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_item["requestKey"],
        first_create_item["requestKey"]
    );
    assert_eq!(
        duplicate_create_item["eventId"],
        first_create_item["eventId"]
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
                        "parentConversationId":"{parent_conversation_id}",
                        "rootMessageId":"{}"
                    }}"#,
                    second_root_item["messageId"].as_str().unwrap()
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
    assert_eq!(conflicting_retry_json["code"], 40901);
}

#[tokio::test]
async fn test_bind_direct_chat_conversation_rejects_non_participant_user_over_http() {
    let app = build_default_test_app();

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
    assert_eq!(bind_json["code"], 40301);
}

#[tokio::test]
async fn test_invited_history_visibility_allows_invited_member_history_reads_before_join_over_http()
{
    let app = build_default_test_app();

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
                        "groupName":"test group",
                        "clientRequestKey":"c_history_invited_http",
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
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("create body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(&create_body).expect("create response should be valid json");
    let conversation_id = response_item(&create_json)["conversationId"]
        .as_str()
        .expect("create response should include canonical conversation id")
        .to_string();

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
    assert_eq!(post_response.status(), StatusCode::CREATED);

    let add_member_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members/add"
                ))
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
    assert_eq!(add_member_json["data"]["state"], "invited");

    let invited_history_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
        invited_history_json["data"]["items"][0]["messageId"],
        format!("msg_{conversation_id}_1")
    );
    assert_eq!(
        invited_history_json["data"]["items"][0]["body"]["summary"],
        "hello invited"
    );

    let outsider_history_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
    assert_eq!(outsider_history_json["code"], 40301);
}

#[tokio::test]
async fn test_shared_history_visibility_allows_external_linked_history_reads_but_not_writes_over_http()
 {
    let app = build_default_test_app();

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
                        "groupName":"test group",
                        "clientRequestKey":"c_history_shared_http",
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
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("create body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(&create_body).expect("create response should be valid json");
    let conversation_id = response_item(&create_json)["conversationId"]
        .as_str()
        .expect("create response should include canonical conversation id")
        .to_string();

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
    assert_eq!(post_response.status(), StatusCode::CREATED);

    let add_member_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members/add"
                ))
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
    assert_eq!(add_member_json["data"]["state"], "linked");
    assert_eq!(
        add_member_json["data"]["attributes"]["sharedChannelPolicyId"],
        "scp_001"
    );

    let linked_history_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
        linked_history_json["data"]["items"][0]["messageId"],
        format!("msg_{conversation_id}_1")
    );
    assert_eq!(
        linked_history_json["data"]["items"][0]["body"]["summary"],
        "hello shared"
    );

    let linked_post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
    assert_eq!(linked_post_json["code"], 40301);

    let outsider_history_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
    assert_eq!(outsider_history_json["code"], 40301);
}

#[tokio::test]
async fn test_sync_shared_channel_linked_member_over_http_materializes_linked_history_reader() {
    let app = build_default_test_app();

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
                        "groupName":"test group",
                        "clientRequestKey":"c_history_shared_sync_http",
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
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("create body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(&create_body).expect("create response should be valid json");
    let conversation_id = response_item(&create_json)["conversationId"]
        .as_str()
        .expect("create response should include canonical conversation id")
        .to_string();

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
    assert_eq!(post_response.status(), StatusCode::CREATED);

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
                .body(Body::from(format!(
                    r#"{{
                        "conversationId":"{conversation_id}",
                        "sharedChannelPolicyId":"scp_sync_http",
                        "externalConnectionId":"ec_sync_http",
                        "localActorId":"1007",
                        "localActorKind":"user",
                        "externalMemberId":"partner::sync-user"
                    }}"#
                )))
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
    assert_eq!(
        sync_json["data"]["proofVersion"],
        "shared_channel_sync_ack.v1"
    );
    assert_eq!(sync_json["data"]["status"], "applied");
    assert_eq!(
        sync_json["data"]["requestKey"],
        format!("100001|{conversation_id}|scp_sync_http|ec_sync_http|1007|user|partner::sync-user")
    );
    assert_eq!(sync_json["data"]["principalId"], "1007");
    assert_eq!(sync_json["data"]["principalKind"], "user");
    assert_eq!(sync_json["data"]["role"], "guest");
    assert_eq!(sync_json["data"]["state"], "linked");
    assert_eq!(
        sync_json["data"]["attributes"]["sharedChannelPolicyId"],
        "scp_sync_http"
    );
    assert_eq!(
        sync_json["data"]["attributes"]["externalConnectionId"],
        "ec_sync_http"
    );
    assert_eq!(
        sync_json["data"]["attributes"]["externalMemberId"],
        "partner::sync-user"
    );

    let linked_history_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
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
        linked_history_json["data"]["items"][0]["messageId"],
        format!("msg_{conversation_id}_1")
    );
    assert_eq!(
        linked_history_json["data"]["items"][0]["body"]["summary"],
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
                .body(Body::from(format!(
                    r#"{{
                        "conversationId":"{conversation_id}",
                        "sharedChannelPolicyId":"scp_sync_http",
                        "externalConnectionId":"ec_sync_http",
                        "localActorId":"1007",
                        "localActorKind":"user",
                        "externalMemberId":"partner::sync-user",
                        "requestKey":"100001|{conversation_id}|scp_sync_http|ec_sync_http|1007|user|partner::sync-user"
                    }}"#
                )))
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
    assert_eq!(
        resync_json["data"]["proofVersion"],
        "shared_channel_sync_ack.v1"
    );
    assert_eq!(resync_json["data"]["status"], "replayed");
    assert_eq!(
        resync_json["data"]["requestKey"],
        format!("100001|{conversation_id}|scp_sync_http|ec_sync_http|1007|user|partner::sync-user")
    );
    assert_eq!(
        resync_json["data"]["attributes"]["sharedChannelSyncRequestKey"],
        format!("100001|{conversation_id}|scp_sync_http|ec_sync_http|1007|user|partner::sync-user")
    );
}

#[tokio::test]
async fn test_sync_shared_channel_linked_member_rejects_unknown_user_local_actor_over_http() {
    let app = build_default_test_app_with_principal_directory(Arc::new(
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
                        "groupName":"test group",
                        "clientRequestKey":"c_history_shared_sync_unknown_http",
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
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("create body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(&create_body).expect("create response should be valid json");
    let conversation_id = response_item(&create_json)["conversationId"]
        .as_str()
        .expect("create response should include canonical conversation id")
        .to_string();

    let sync_request_body = serde_json::json!({
        "conversationId": conversation_id,
        "sharedChannelPolicyId":"scp_sync_unknown_http",
        "externalConnectionId":"ec_sync_unknown_http",
        "localActorId":"1044",
        "localActorKind":"user",
        "externalMemberId":"partner::unknown-user"
    })
    .to_string();

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
                .body(Body::from(sync_request_body))
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
    assert_eq!(value["code"], 40001);
}

#[tokio::test]
async fn test_shared_history_sync_rejects_oversized_local_actor_kind_over_http() {
    let app = build_default_test_app();

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
                        "groupName":"test group",
                        "clientRequestKey":"c_history_shared_sync_oversized_kind",
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
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("create body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(&create_body).expect("create response should be valid json");
    let conversation_id = response_item(&create_json)["conversationId"]
        .as_str()
        .expect("create response should include canonical conversation id")
        .to_string();

    let request_body = serde_json::json!({
        "conversationId": conversation_id,
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
    assert_eq!(value["code"], 41301);
    assert!(
        value["detail"]
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

#[tokio::test]
async fn test_ensure_welcome_message_is_idempotent_over_http() {
    let app = build_default_test_app();

    // 首次调用：新用户 → 发送系统消息（系统智能体↔用户 direct chat）。
    let first = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/me/welcome/ensure")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("u_welcome_alice")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("welcome ensure request should succeed");
    assert_eq!(first.status(), StatusCode::OK);
    let body = first
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    let item = response_item(&value);
    assert_eq!(item["status"], "sent");
    let conversation_id = item["conversationId"]
        .as_str()
        .expect("sent welcome should carry conversation id")
        .to_owned();
    let message_id = item["messageId"]
        .as_str()
        .expect("sent welcome should carry message id")
        .to_owned();
    assert!(item["messageSeq"].as_u64().unwrap_or(0) >= 1);

    // 欢迎消息以 messageType=system 且发送者为系统智能体投递。
    let messages = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("u_welcome_alice")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list messages should succeed");
    let messages_body = messages
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let messages_value: serde_json::Value =
        serde_json::from_slice(&messages_body).expect("response should be valid json");
    let items = messages_value
        .pointer("/data/items")
        .and_then(|items| items.as_array())
        .expect("message list should carry items");
    let welcome_message = items
        .iter()
        .find(|entry| entry["messageId"] == serde_json::Value::String(message_id.clone()))
        .expect("welcome message should be listed");
    assert_eq!(welcome_message["messageType"], "system");
    assert_eq!(welcome_message["sender"]["kind"], "system");
    assert_eq!(welcome_message["sender"]["id"], "system");

    // 再次调用：不再重复发送。
    let second = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/me/welcome/ensure")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("u_welcome_alice")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second welcome ensure should succeed");
    let second_body = second
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let second_value: serde_json::Value =
        serde_json::from_slice(&second_body).expect("response should be valid json");
    assert_eq!(response_item(&second_value)["status"], "already_sent");
}

