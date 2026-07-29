use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_app_context::DualTokenRequestBuilderExt;
use std::sync::Once;
use tower::ServiceExt;

static INIT_PORTAL_HTTP_TEST_ENV: Once = Once::new();

fn init_portal_http_test_env() {
    INIT_PORTAL_HTTP_TEST_ENV.call_once(|| unsafe {
        std::env::set_var("SDKWORK_IM_ENVIRONMENT", "dev");
    });
}

fn portal_http_test_app() -> axum::Router {
    init_portal_http_test_env();
    portal_service::build_public_app()
}

fn portal_route_http_test_app() -> axum::Router {
    init_portal_http_test_env();
    sdkwork_routes_im_portal_app_api::build_public_app()
}

#[tokio::test]
async fn test_route_composition_exports_required_infrastructure_endpoints() {
    let app = portal_route_http_test_app();

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
async fn test_public_app_exports_live_openapi_json() {
    let app = portal_http_test_app();

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
    assert_eq!(value["info"]["title"], "Sdkwork IM Portal Service API");
    assert!(value["paths"]["/app/v3/api/portal/workspace"].is_object());
    for path in [
        "/app/v3/api/portal/access",
        "/app/v3/api/portal/automation",
        "/app/v3/api/portal/conversations",
        "/app/v3/api/portal/dashboard",
        "/app/v3/api/portal/governance",
        "/app/v3/api/portal/home",
        "/app/v3/api/portal/media",
        "/app/v3/api/portal/realtime",
    ] {
        assert!(
            value["paths"][path].is_object(),
            "missing live path: {path}"
        );
    }
}

#[tokio::test]
async fn test_portal_workspace_returns_sdkwork_envelope() {
    let app = portal_route_http_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/portal/workspace")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("workspace request should succeed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");

    assert_eq!(json["code"], 0);
    assert!(json["data"]["item"]["slug"].as_str().is_some());
    assert!(
        json["traceId"]
            .as_str()
            .is_some_and(|value| !value.is_empty())
    );
}

#[tokio::test]
async fn test_portal_dashboard_requires_authenticated_session() {
    let app = portal_route_http_test_app();

    let unauthorized = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/portal/dashboard")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("dashboard request should complete");

    assert_eq!(unauthorized.status(), StatusCode::UNAUTHORIZED);

    let authorized = app
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/portal/dashboard")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("authorized dashboard request should succeed");

    assert_eq!(authorized.status(), StatusCode::OK);

    let body = authorized
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");

    assert_eq!(json["code"], 0);
    assert_eq!(json["data"]["item"]["meta"]["section"], "dashboard");
    assert_eq!(json["data"]["item"]["availability"]["state"], "unavailable");
    assert!(json["data"]["item"].get("metrics").is_none());
    assert!(
        json["traceId"]
            .as_str()
            .is_some_and(|value| !value.is_empty())
    );
}

#[tokio::test]
async fn test_portal_governance_fail_closed_without_audit_records() {
    let app = portal_route_http_test_app();

    let forbidden = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/portal/governance")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("governance request without audit.read should complete");

    assert_eq!(forbidden.status(), StatusCode::FORBIDDEN);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/portal/governance")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("audit.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("governance request with audit.read should succeed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");

    assert_eq!(json["code"], 0);
    assert_eq!(json["data"]["item"]["meta"]["section"], "governance");
    assert_eq!(json["data"]["item"]["availability"]["state"], "available");
    assert_eq!(json["data"]["item"]["sampledEventCount"], "0");
}

#[tokio::test]
async fn test_portal_access_requires_audit_read_permission() {
    let app = portal_route_http_test_app();

    let forbidden = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/portal/access")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("access request without audit.read should complete");

    assert_eq!(forbidden.status(), StatusCode::FORBIDDEN);

    let authorized = app
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/portal/access")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("audit.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("access request with audit.read should succeed");

    assert_eq!(authorized.status(), StatusCode::OK);
}
