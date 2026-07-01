use axum::Router;
use sdkwork_im_web_bootstrap::wrap_im_service_router_with_manifest;

use crate::manifest::open_route_manifest;

/// Wrap the IM social open-api router with the canonical SDKWork interceptor
/// pipeline (`WebFrameworkLayer`) and the actual [`HttpRouteManifest`]
/// declared by `manifest::open_route_manifest()`.
///
/// Passing the real route table (instead of an empty manifest) enables
/// `IamAuthorizationPolicy` enforcement, route-level HTTP metrics
/// dimensions, and OpenAPI metadata consistency per `API_SPEC.md` §4.5,
/// §14, and §15.
pub fn wrap_router(router: Router) -> Router {
    wrap_im_service_router_with_manifest(router, open_route_manifest())
}
