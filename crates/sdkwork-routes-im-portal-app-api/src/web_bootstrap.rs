use axum::Router;

use crate::manifest::route_manifest;

pub fn wrap_router(router: Router) -> Router {
    sdkwork_im_web_bootstrap::wrap_im_service_router_with_manifest(router, route_manifest())
}
