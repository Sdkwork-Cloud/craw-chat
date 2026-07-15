use axum::Router;
use sdkwork_im_web_bootstrap::wrap_im_service_router_with_manifest;

use crate::manifest::route_manifest;

pub fn wrap_router(router: Router) -> Router {
    wrap_im_service_router_with_manifest(router, route_manifest())
}
