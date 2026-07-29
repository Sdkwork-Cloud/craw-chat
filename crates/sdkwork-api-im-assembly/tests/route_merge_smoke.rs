use std::sync::OnceLock;

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
