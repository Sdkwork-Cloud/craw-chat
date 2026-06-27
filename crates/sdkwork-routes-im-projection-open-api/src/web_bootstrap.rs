use axum::Router;
use sdkwork_im_web_bootstrap::{
    wrap_im_open_api_service_router, wrap_im_open_api_service_router_from_env,
};

use crate::manifest::route_manifest;

pub fn wrap_router(router: Router) -> Router {
    wrap_im_open_api_service_router(router)
}

pub async fn wrap_router_from_env(router: Router) -> Router {
    wrap_im_open_api_service_router_from_env(route_manifest(), router).await
}

pub fn route_manifest_for_wrap() -> sdkwork_web_core::HttpRouteManifest {
    route_manifest()
}
