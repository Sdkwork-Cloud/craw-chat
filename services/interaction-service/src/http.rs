//! Health-only router for the deprecated interaction-service workspace member.
//! Canonical client paths live under `/im/v3/api/chat/` in `sdkwork-im-im.openapi.yaml`.

use axum::Router;
use sdkwork_im_web_bootstrap::{im_service_router_config, mount_im_infra_routes};

pub fn build_app() -> Router {
    mount_im_infra_routes(Router::new(), im_service_router_config())
}

pub fn build_public_app() -> Router {
    build_app()
}
