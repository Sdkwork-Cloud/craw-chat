use std::sync::OnceLock;

fn ensure_route_merge_test_environment() {
    static TEST_ENVIRONMENT: OnceLock<()> = OnceLock::new();
    TEST_ENVIRONMENT.get_or_init(|| unsafe {
        std::env::set_var("SDKWORK_IM_ENVIRONMENT", "test");
        std::env::set_var("SDKWORK_IM_DATABASE_ENGINE", "sqlite");
        std::env::set_var("SDKWORK_IM_DATABASE_URL", "sqlite::memory:");
    });
}

#[tokio::test]
async fn chat_and_projection_supplemental_routers_merge_without_duplicate_routes() {
    ensure_route_merge_test_environment();

    let _router = axum::Router::<()>::new()
        .merge(sdkwork_routes_im_chat_open_api::gateway_mount().await)
        .merge(sdkwork_routes_im_projection_open_api::build_supplemental_public_app());
}

#[tokio::test]
async fn gateway_pre_projection_routers_merge_projection_without_duplicate_routes() {
    ensure_route_merge_test_environment();

    let _router = axum::Router::<()>::new()
        .merge(sdkwork_routes_im_audit_backend_api::gateway_mount())
        .merge(sdkwork_routes_im_automation_app_api::gateway_mount())
        .merge(sdkwork_routes_im_calls_open_api::gateway_mount())
        .merge(sdkwork_routes_im_chat_open_api::gateway_mount().await)
        .merge(sdkwork_routes_im_governance_backend_api::gateway_mount())
        .merge(sdkwork_routes_im_media_app_api::gateway_mount())
        .merge(sdkwork_routes_im_notification_app_api::gateway_mount())
        .merge(sdkwork_routes_im_ops_backend_api::gateway_mount())
        .merge(sdkwork_routes_im_portal_app_api::gateway_mount())
        .merge(sdkwork_routes_im_projection_open_api::build_supplemental_public_app());
}
