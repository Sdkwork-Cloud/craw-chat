use axum::Router;
use sdkwork_im_web_bootstrap::wrap_im_service_router;

use crate::manifest::route_manifest;

pub fn wrap_router(router: Router) -> Router {
    wrap_im_service_router(router)
}

pub fn route_manifest_for_wrap() -> sdkwork_web_core::HttpRouteManifest {
    route_manifest()
}
