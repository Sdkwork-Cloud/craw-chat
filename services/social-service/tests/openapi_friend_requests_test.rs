use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_app_context::{build_dual_token_headers_for_context, local_service_app_context};
use social_service::friendship::AppState;
use social_service::{build_open_api_router, SocialRuntime};
use std::sync::Arc;
use tower::ServiceExt;

fn auth_headers() -> axum::http::HeaderMap {
    let mut context = local_service_app_context(
        "100001",
        "30",
        "user",
        Some("device_test"),
        ["*"],
    );
    context.organization_id = "0".into();
    build_dual_token_headers_for_context(&context, context.permission_scope.iter())
}

#[tokio::test]
async fn open_api_friend_requests_list_returns_json_not_html() {
    let app = build_open_api_router(AppState {
        social_runtime: Arc::new(SocialRuntime::default()),
    });

    let mut request = Request::builder()
        .method("GET")
        .uri("/im/v3/api/social/friend_requests?direction=incoming&status=pending&limit=100")
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
    assert!(json.get("items").and_then(|value| value.as_array()).is_some());
}

#[tokio::test]
async fn open_api_contact_tags_list_returns_json() {
    let app = build_open_api_router(AppState {
        social_runtime: Arc::new(SocialRuntime::default()),
    });

    let mut request = Request::builder()
        .method("GET")
        .uri("/im/v3/api/social/contacts/tags?limit=100")
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
    assert!(json.get("items").and_then(|value| value.as_array()).is_some());
    assert_eq!(json.get("hasMore").and_then(|value| value.as_bool()), Some(false));
}
