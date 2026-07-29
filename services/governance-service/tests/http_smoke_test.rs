use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use std::sync::Once;
use tower::ServiceExt;

static INIT_GOVERNANCE_HTTP_TEST_ENV: Once = Once::new();

fn init_governance_http_test_env() {
    INIT_GOVERNANCE_HTTP_TEST_ENV.call_once(|| unsafe {
        std::env::set_var("SDKWORK_IM_ENVIRONMENT", "dev");
    });
}

#[tokio::test]
async fn test_route_composition_exports_required_infrastructure_endpoints() {
    init_governance_http_test_env();
    let app = sdkwork_routes_im_governance_backend_api::build_public_app();

    for path in ["/healthz", "/metrics", "/openapi.json", "/docs"] {
        let response = app
            .clone()
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .expect("infrastructure request should succeed");
        assert_eq!(response.status(), StatusCode::OK, "endpoint {path}");
    }

    let readiness = app
        .oneshot(
            Request::builder()
                .uri("/readyz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("readiness request should succeed");
    assert!(
        matches!(
            readiness.status(),
            StatusCode::OK | StatusCode::SERVICE_UNAVAILABLE
        ),
        "readiness endpoint must report actual dependency state"
    );
}

#[tokio::test]
async fn test_healthz_returns_ok_and_service_metadata() {
    let app = governance_service::build_app();

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
}

#[tokio::test]
async fn test_public_app_exports_live_openapi_json() {
    let app = governance_service::build_public_app();

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

    assert_eq!(value["openapi"], "3.1.2");
    assert_eq!(value["info"]["title"], "Control Plane API");
    assert!(value["paths"]["/backend/v3/api/control/protocol_registry"].is_object());
}

#[tokio::test]
async fn test_public_app_serves_docs_page_for_live_openapi() {
    let app = governance_service::build_public_app();

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
    assert!(html.contains("Control Plane API"));
    assert!(html.contains("/openapi.json"));
}
