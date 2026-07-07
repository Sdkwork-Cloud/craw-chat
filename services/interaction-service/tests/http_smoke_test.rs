//! Interaction Service smoke tests.
//!
//! `interaction-service` is a deprecated public HTTP surface. Reactions,
//! pins, threads, and conversation settings now live under the `chat`
//! OpenAPI tag at `/im/v3/api/chat/*`. This service exposes only infra
//! routes (health/metrics). These tests verify:
//!
//! 1. `build_app` and `build_public_app` produce working routers.
//! 2. Infra routes (healthz, readyz, metrics) respond.
//! 3. Deprecated `/im/v3/api/interactions/*` routes are NOT mounted.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

/// `build_app` must produce a router that responds to `/healthz`.
#[tokio::test]
async fn build_app_exposes_healthz() {
    let app = interaction_service::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .expect("healthz request builder should succeed"),
        )
        .await
        .expect("healthz request should complete");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("healthz body should collect")
        .to_bytes();
    let text = String::from_utf8(body.to_vec()).expect("healthz body should be utf-8");
    assert!(
        text.contains("ok") || text.contains("healthy") || text.contains("ready"),
        "healthz should report readiness, got: {text}"
    );
}

/// `build_public_app` must be equivalent to `build_app` for infra routes.
#[tokio::test]
async fn build_public_app_exposes_healthz() {
    let app = interaction_service::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .expect("healthz request builder should succeed"),
        )
        .await
        .expect("healthz request should complete");

    assert_eq!(response.status(), StatusCode::OK);
}

/// The deprecated `/im/v3/api/interactions/*` surface must NOT be mounted.
/// Canonical paths live under `/im/v3/api/chat/*`. This guards against
/// accidental route revival.
#[tokio::test]
async fn deprecated_interactions_api_routes_are_not_mounted() {
    let app = interaction_service::build_app();

    let deprecated_paths = [
        "/im/v3/api/interactions/reactions",
        "/im/v3/api/interactions/pins",
        "/im/v3/api/interactions/threads",
        "/im/v3/api/interactions/conversation_settings",
    ];

    for path in deprecated_paths {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(path)
                    .body(Body::empty())
                    .expect("request builder should succeed"),
            )
            .await
            .expect("deprecated interactions request should complete");

        assert!(
            response.status() == StatusCode::NOT_FOUND
                || response.status() == StatusCode::METHOD_NOT_ALLOWED,
            "deprecated {path} should not be mounted, got {}",
            response.status()
        );
    }
}

/// `/readyz` should respond (either OK if deps are available, or 503 if
/// deps are missing — both prove the route is wired correctly).
#[tokio::test]
async fn readyz_route_is_wired() {
    let app = interaction_service::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/readyz")
                .body(Body::empty())
                .expect("readyz request builder should succeed"),
        )
        .await
        .expect("readyz request should complete");

    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::SERVICE_UNAVAILABLE,
        "readyz should respond with 200 or 503, got {}",
        response.status()
    );
}
