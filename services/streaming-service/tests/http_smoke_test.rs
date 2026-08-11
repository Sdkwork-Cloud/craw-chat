use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_app_context::DualTokenRequestBuilderExt;
use std::sync::Once;
use tower::ServiceExt;

static INIT_STREAMING_HTTP_TEST_ENV: Once = Once::new();

fn init_streaming_http_test_env() {
    INIT_STREAMING_HTTP_TEST_ENV.call_once(|| unsafe {
        std::env::set_var("SDKWORK_IM_ENVIRONMENT", "dev");
    });
}

fn streaming_http_test_app() -> axum::Router {
    init_streaming_http_test_env();
    streaming_service::build_public_app()
}

fn streaming_route_http_test_app() -> axum::Router {
    init_streaming_http_test_env();
    sdkwork_routes_im_stream_app_api::build_public_app()
}

#[tokio::test]
async fn test_service_readiness_probes_store_and_metrics_include_stream_counters() {
    init_streaming_http_test_env();
    let runtime = std::sync::Arc::new(streaming_service::StreamingRuntime::default());
    let app = streaming_service::build_app(runtime.clone());

    let readiness = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/readyz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("readiness request should succeed");
    // Readiness must be mounted and report actual dependency state: the
    // in-memory test runtime is ready, but the shared env dependency probe is
    // fail-closed when no PostgreSQL URL is configured.
    assert!(
        matches!(
            readiness.status(),
            StatusCode::OK | StatusCode::SERVICE_UNAVAILABLE
        ),
        "readiness endpoint must be mounted and report actual dependency state"
    );

    let metrics = app
        .oneshot(
            Request::builder()
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("metrics request should succeed");
    assert_eq!(metrics.status(), StatusCode::OK);
    assert_eq!(
        metrics.headers().get("content-type").unwrap(),
        "text/plain; version=0.0.4; charset=utf-8"
    );
    let body = metrics
        .into_body()
        .collect()
        .await
        .expect("metrics body should collect")
        .to_bytes();
    let body = String::from_utf8(body.to_vec()).expect("metrics should be utf-8");
    assert!(body.contains("sdkwork_http_requests_total"));
    assert!(body.contains("im_stream_append_requests_total"));
    assert!(body.contains("im_stream_store_errors_total"));
    assert!(body.contains("im_stream_frame_page_items_total"));
}

#[tokio::test]
async fn test_public_app_exports_live_openapi_json() {
    let app = streaming_http_test_app();

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
    assert_eq!(value["info"]["title"], "Sdkwork IM Streaming Service API");
    assert_eq!(
        value["paths"]
            .as_object()
            .map(|paths| paths.len())
            .unwrap_or(0),
        0,
        "standalone streaming-service live OpenAPI export is metadata-only until route extraction covers nested mounts"
    );
}

#[tokio::test]
async fn test_public_app_serves_docs_page_for_live_openapi() {
    let app = streaming_http_test_app();

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
    assert!(html.contains("Sdkwork IM Streaming Service API"));
    assert!(html.contains("/openapi.json"));
}

#[tokio::test]
async fn test_open_stream_over_http() {
    let app = streaming_route_http_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_demo",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");

    assert_eq!(value["data"]["streamId"], "st_demo");
    assert_eq!(value["data"]["state"], "opened");
}

#[tokio::test]
async fn test_standalone_streaming_service_rejects_conversation_scope_over_http() {
    let app = streaming_route_http_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_conversation_scope_rejected",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should return response");

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");

    assert_eq!(value["code"].as_i64(), Some(40301));
}

#[tokio::test]
async fn test_open_stream_rejects_oversized_stream_id_over_http() {
    let app = streaming_route_http_test_app();
    let oversized_stream_id = "s".repeat(257);
    let request_body = serde_json::json!({
        "streamId": oversized_stream_id,
        "streamType":"custom.delta.text",
        "scopeKind":"request",
        "scopeId":"req_demo",
        "durabilityClass":"durableSession",
        "schemaRef":"custom.delta.text.v1"
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized stream id open request should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_open_stream_rejects_oversized_durability_class_over_http() {
    let app = streaming_route_http_test_app();
    let oversized_durability_class = "d".repeat(65);
    let request_body = serde_json::json!({
        "streamId":"st_oversized_durability_class",
        "streamType":"custom.delta.text",
        "scopeKind":"request",
        "scopeId":"req_demo",
        "durabilityClass": oversized_durability_class,
        "schemaRef":"custom.delta.text.v1"
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized durability class open request should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("rejection body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"].as_i64(), Some(41301));
    assert!(
        value["detail"]
            .as_str()
            .expect("rejection detail should be a string")
            .contains("durabilityClass"),
        "error should point to durabilityClass guard, got: {value:?}"
    );
}
