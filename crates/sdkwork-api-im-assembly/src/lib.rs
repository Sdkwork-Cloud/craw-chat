//! Generated API assembly for sdkwork-im.

mod generated;

pub struct ApiAssembly {
    pub router: axum::Router,
}

pub async fn assemble_api_router() -> ApiAssembly {
    let mut router = axum::Router::new();
    router = router.merge(sdkwork_routes_im_audit_backend_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_automation_app_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_calls_open_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_chat_open_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_governance_backend_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_knowledgebase_app_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_media_app_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_notification_app_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_ops_backend_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_portal_app_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_projection_open_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_realtime_open_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_social_backend_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_social_open_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_space_open_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_stream_app_api::gateway_mount());
    ApiAssembly { router }
}

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
