use std::sync::OnceLock;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use im_app_context::DualTokenRequestBuilderExt;
use tower::ServiceExt;

fn ensure_route_merge_test_environment() {
    static TEST_ENVIRONMENT: OnceLock<()> = OnceLock::new();
    TEST_ENVIRONMENT.get_or_init(|| unsafe {
        std::env::set_var("SDKWORK_IM_ENVIRONMENT", "test");
    });
}

async fn chat_gateway_mount_for_test() -> axum::Router {
    let state = conversation_runtime::http::default_app_state();
    sdkwork_routes_im_chat_open_api::gateway_mount_with_state(state)
        .await
        .expect("chat gateway mount should complete with explicit test state")
}

#[tokio::test]
async fn chat_router_mounts_conversation_queries_without_duplicate_routes() {
    ensure_route_merge_test_environment();

    let _router = axum::Router::<()>::new().merge(chat_gateway_mount_for_test().await);
}

#[tokio::test]
async fn gateway_domain_routers_merge_without_duplicate_routes() {
    ensure_route_merge_test_environment();

    let _router = axum::Router::<()>::new()
        .merge(sdkwork_routes_im_audit_backend_api::gateway_mount())
        .merge(sdkwork_routes_im_automation_app_api::gateway_mount())
        .merge(sdkwork_routes_im_calls_open_api::gateway_mount())
        .merge(chat_gateway_mount_for_test().await)
        .merge(sdkwork_routes_im_governance_backend_api::gateway_mount())
        .merge(sdkwork_routes_im_media_app_api::gateway_mount())
        .merge(sdkwork_routes_im_notification_app_api::gateway_mount())
        .merge(sdkwork_routes_im_ops_backend_api::gateway_mount())
        .merge(sdkwork_routes_im_portal_app_api::gateway_mount());
}

#[tokio::test]
async fn automation_app_mount_does_not_export_backend_governance() {
    ensure_route_merge_test_environment();

    let response = sdkwork_routes_im_automation_app_api::gateway_mount()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/automation/governance")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("admin")
                .with_dual_token_permission_scope("automation.read")
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("app mount should return a response");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn governance_backend_mount_exports_automation_governance() {
    ensure_route_merge_test_environment();

    let response = sdkwork_routes_im_governance_backend_api::gateway_mount()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/automation/governance")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("admin")
                .with_dual_token_permission_scope("automation.read")
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("backend mount should return a response");

    assert_eq!(response.status(), StatusCode::OK);
}
