use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_app_context::{build_dual_token_headers_for_context, local_service_app_context};
use sdkwork_im_web_bootstrap::wrap_im_service_router;
use social_service::friendship::AppState;
use social_service::{SocialRuntime, build_control_domain_api_router, build_open_api_router};
use std::sync::Arc;
use tower::ServiceExt;

fn auth_headers() -> axum::http::HeaderMap {
    auth_headers_for_user("30")
}

fn auth_headers_for_user(user_id: &str) -> axum::http::HeaderMap {
    let mut context =
        local_service_app_context("100001", user_id, "user", Some("device_test"), ["*"]);
    context.organization_id = "0".into();
    build_dual_token_headers_for_context(&context, context.permission_scope.iter())
}

fn backend_control_auth_headers() -> axum::http::HeaderMap {
    let mut context = local_service_app_context("100001", "30", "user", Some("device_test"), ["*"]);
    context.organization_id = "org_30".into();
    build_dual_token_headers_for_context(&context, context.permission_scope.iter())
}

fn wrapped_open_api_app(state: AppState) -> axum::Router {
    wrap_im_service_router(build_open_api_router(state))
}

fn wrapped_control_api_app(state: AppState) -> axum::Router {
    wrap_im_service_router(build_control_domain_api_router(state))
}

#[tokio::test]
async fn open_api_friend_requests_list_returns_sdkwork_envelope() {
    let app = wrapped_open_api_app(AppState {
        social_runtime: Arc::new(SocialRuntime::for_test()),
    });

    let mut request = Request::builder()
        .method("GET")
        .uri("/im/v3/api/social/friend_requests?direction=incoming&status=pending&page_size=100")
        .body(Body::empty())
        .expect("request builder should succeed");
    *request.headers_mut() = auth_headers();

    let response = app
        .oneshot(request)
        .await
        .expect("friend request list should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let content_type = response
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    assert!(
        content_type.starts_with("application/json"),
        "expected JSON response, got content-type {content_type}"
    );

    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(body.as_ref()).expect("response body should be JSON");
    assert_eq!(json.get("code").and_then(|value| value.as_i64()), Some(0));
    let data = json.get("data").expect("SdkWorkApiResponse data");
    assert!(
        data.get("items")
            .and_then(|value| value.as_array())
            .is_some()
    );
    assert!(data.get("pageInfo").is_some());
}

#[tokio::test]
async fn backend_control_friend_requests_list_uses_page_size_query() {
    let app = wrapped_control_api_app(AppState {
        social_runtime: Arc::new(SocialRuntime::for_test()),
    });

    let mut request = Request::builder()
        .method("GET")
        .uri("/backend/v3/api/control/social/friend_requests?userId=30&direction=incoming&status=pending&page_size=7")
        .body(Body::empty())
        .expect("request builder should succeed");
    *request.headers_mut() = backend_control_auth_headers();

    let response = app
        .oneshot(request)
        .await
        .expect("friend request list should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(body.as_ref()).expect("response body should be JSON");
    let page_info = json
        .pointer("/data/pageInfo")
        .expect("list response should include pageInfo");
    assert_eq!(
        page_info.get("pageSize").and_then(|value| value.as_i64()),
        Some(7)
    );
}

#[tokio::test]
async fn backend_control_friend_requests_list_rejects_limit_alias() {
    let app = wrapped_control_api_app(AppState {
        social_runtime: Arc::new(SocialRuntime::for_test()),
    });

    let mut request = Request::builder()
        .method("GET")
        .uri("/backend/v3/api/control/social/friend_requests?userId=30&direction=incoming&status=pending&limit=7")
        .body(Body::empty())
        .expect("request builder should succeed");
    *request.headers_mut() = backend_control_auth_headers();

    let response = app
        .oneshot(request)
        .await
        .expect("friend request list should return problem response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn backend_control_friend_request_create_returns_created_resource_item() {
    let app = wrapped_control_api_app(AppState {
        social_runtime: Arc::new(SocialRuntime::for_test()),
    });

    let body = serde_json::json!({
        "eventId": "evt_control_friend_request_create",
        "requesterUserId": "30",
        "targetUserId": "31",
        "requestMessage": "hello from control",
        "requestedAt": "2026-07-07T00:00:00.000Z"
    });
    let mut request = Request::builder()
        .method("POST")
        .uri("/backend/v3/api/control/social/friend_requests")
        .header(axum::http::header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .expect("request builder should succeed");
    *request.headers_mut() = backend_control_auth_headers();
    request.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("application/json"),
    );

    let response = app
        .oneshot(request)
        .await
        .expect("control friend request create should succeed");

    let status = response.status();
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    assert_eq!(
        status,
        StatusCode::CREATED,
        "unexpected response body: {}",
        String::from_utf8_lossy(body.as_ref())
    );
    let json: serde_json::Value =
        serde_json::from_slice(body.as_ref()).expect("response body should be JSON");
    assert_eq!(json.get("code").and_then(|value| value.as_i64()), Some(0));
    assert!(
        json.get("data")
            .and_then(|value| value.get("item"))
            .and_then(|value| value.get("friendRequest"))
            .is_some(),
        "create response must use data.item"
    );
}

#[tokio::test]
async fn open_api_contact_tag_create_returns_created_and_delete_returns_no_content() {
    let app = wrapped_open_api_app(AppState {
        social_runtime: Arc::new(SocialRuntime::for_test()),
    });

    let body = serde_json::json!({
        "name": "project",
        "color": "#2563eb"
    });
    let mut create_request = Request::builder()
        .method("POST")
        .uri("/im/v3/api/social/contacts/tags")
        .header(axum::http::header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .expect("request builder should succeed");
    *create_request.headers_mut() = auth_headers();
    create_request.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("application/json"),
    );

    let create_response = app
        .clone()
        .oneshot(create_request)
        .await
        .expect("contact tag create should succeed");
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("create response body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(create_body.as_ref()).expect("create response body should be JSON");
    let tag_id = create_json
        .get("data")
        .and_then(|value| value.get("item"))
        .and_then(|value| value.get("tagId"))
        .and_then(|value| value.as_str())
        .expect("created tag id")
        .to_owned();

    let mut delete_request = Request::builder()
        .method("DELETE")
        .uri(format!("/im/v3/api/social/contacts/tags/{tag_id}"))
        .body(Body::empty())
        .expect("request builder should succeed");
    *delete_request.headers_mut() = auth_headers();

    let delete_response = app
        .oneshot(delete_request)
        .await
        .expect("contact tag delete should succeed");
    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);
    let delete_body = delete_response
        .into_body()
        .collect()
        .await
        .expect("delete response body should collect")
        .to_bytes();
    assert!(
        delete_body.is_empty(),
        "204 delete response must not include a JSON body"
    );
}

#[tokio::test]
async fn open_api_friend_request_create_uses_friend_request_id_wire_field() {
    let app = wrapped_open_api_app(AppState {
        social_runtime: Arc::new(SocialRuntime::for_test()),
    });

    let body = serde_json::json!({
        "targetUserId": "31",
        "requestMessage": "hello"
    });
    let mut request = Request::builder()
        .method("POST")
        .uri("/im/v3/api/social/friend_requests")
        .header(axum::http::header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .expect("request builder should succeed");
    *request.headers_mut() = auth_headers();
    request.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("application/json"),
    );

    let response = app
        .oneshot(request)
        .await
        .expect("friend request create should succeed");

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(body.as_ref()).expect("response body should be JSON");
    assert_eq!(json.get("code").and_then(|value| value.as_i64()), Some(0));
    let friend_request = json
        .get("data")
        .and_then(|value| value.get("item"))
        .and_then(|value| value.get("friendRequest"))
        .expect("friendRequest response item");
    assert!(
        friend_request.get("friendRequestId").is_some(),
        "friendRequestId must be the HTTP wire resource id"
    );
    assert!(
        friend_request.get("requestId").is_none(),
        "HTTP responses must not expose forbidden requestId fields"
    );
}

#[tokio::test]
async fn open_api_friend_request_accept_keeps_its_direct_conversation_wire_shape() {
    let app = wrapped_open_api_app(AppState {
        social_runtime: Arc::new(SocialRuntime::for_test()),
    });

    let mut create_request = Request::builder()
        .method("POST")
        .uri("/im/v3/api/social/friend_requests")
        .header(axum::http::header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            serde_json::json!({
                "targetUserId": "31",
                "requestMessage": "please add me"
            })
            .to_string(),
        ))
        .expect("friend request creation should build");
    *create_request.headers_mut() = auth_headers();
    create_request.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("application/json"),
    );

    let create_response = app
        .clone()
        .oneshot(create_request)
        .await
        .expect("friend request creation should respond");
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("friend request creation body should collect")
        .to_bytes();
    let create_json: serde_json::Value = serde_json::from_slice(create_body.as_ref())
        .expect("friend request creation should return JSON");
    let friend_request_id = create_json
        .pointer("/data/item/friendRequest/friendRequestId")
        .and_then(serde_json::Value::as_str)
        .expect("friend request creation should return friendRequestId");

    let mut accept_request = Request::builder()
        .method("POST")
        .uri(format!(
            "/im/v3/api/social/friend_requests/{friend_request_id}/accept"
        ))
        .body(Body::empty())
        .expect("friend request acceptance should build");
    *accept_request.headers_mut() = auth_headers_for_user("31");

    let accept_response = app
        .oneshot(accept_request)
        .await
        .expect("friend request acceptance should respond");
    assert_eq!(accept_response.status(), StatusCode::OK);
    let accept_body = accept_response
        .into_body()
        .collect()
        .await
        .expect("friend request acceptance body should collect")
        .to_bytes();
    let accept_json: serde_json::Value = serde_json::from_slice(accept_body.as_ref())
        .expect("friend request acceptance should return JSON");
    let conversation = accept_json
        .pointer("/data/item/conversation")
        .expect("friend request acceptance should return a direct conversation");
    let mut field_names: Vec<String> = conversation
        .as_object()
        .expect("accepted conversation should be an object")
        .keys()
        .cloned()
        .collect();
    field_names.sort_unstable();
    assert_eq!(
        field_names,
        ["conversationId", "createdAt", "kind", "tenantId"].map(str::to_owned)
    );
    assert_eq!(conversation["tenantId"], "100001");
    assert_eq!(conversation["kind"], "direct");
    assert!(conversation["conversationId"].is_string());
    assert!(conversation["createdAt"].is_string());
}

#[tokio::test]
async fn open_api_friend_requests_list_accepts_all_direction() {
    let app = wrapped_open_api_app(AppState {
        social_runtime: Arc::new(SocialRuntime::for_test()),
    });

    let mut request = Request::builder()
        .method("GET")
        .uri("/im/v3/api/social/friend_requests?direction=all&status=pending&page_size=100")
        .body(Body::empty())
        .expect("request builder should succeed");
    *request.headers_mut() = auth_headers();

    let response = app
        .oneshot(request)
        .await
        .expect("friend request list with direction=all should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(body.as_ref()).expect("response body should be JSON");
    assert_eq!(json.get("code").and_then(|value| value.as_i64()), Some(0));
    let data = json.get("data").expect("SdkWorkApiResponse data");
    assert!(
        data.get("items")
            .and_then(|value| value.as_array())
            .is_some()
    );
    assert!(data.get("pageInfo").is_some());
}

#[tokio::test]
async fn open_api_contact_tags_list_returns_sdkwork_envelope() {
    let app = wrapped_open_api_app(AppState {
        social_runtime: Arc::new(SocialRuntime::for_test()),
    });

    let mut request = Request::builder()
        .method("GET")
        .uri("/im/v3/api/social/contacts/tags?page_size=100")
        .body(Body::empty())
        .expect("request builder should succeed");
    *request.headers_mut() = auth_headers();

    let response = app
        .oneshot(request)
        .await
        .expect("contact tags list should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(body.as_ref()).expect("response body should be JSON");
    assert_eq!(json.get("code").and_then(|value| value.as_i64()), Some(0));
    let data = json.get("data").expect("SdkWorkApiResponse data");
    assert!(
        data.get("items")
            .and_then(|value| value.as_array())
            .is_some()
    );
    assert_eq!(
        data.get("pageInfo")
            .and_then(|value| value.get("hasMore"))
            .and_then(|value| value.as_bool()),
        Some(false)
    );
}
