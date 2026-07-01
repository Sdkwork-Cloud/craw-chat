use axum::Router;
use sdkwork_im_web_bootstrap::{
    wrap_im_open_api_service_router_from_env, wrap_im_service_router_with_manifest,
};

use crate::manifest::route_manifest;

/// Wrap the IM chat open-api router with the canonical SDKWork interceptor
/// pipeline (`WebFrameworkLayer`) and the actual [`HttpRouteManifest`]
/// declared by `manifest::route_manifest()`.
///
/// Passing the real route table (instead of an empty manifest) enables
/// `IamAuthorizationPolicy` enforcement, route-level HTTP metrics
/// dimensions, and OpenAPI metadata consistency per `API_SPEC.md` §4.5,
/// §14, and §15. Uses the cached IAM resolver; for split-deploy processes
/// that need IAM database lookup, use [`wrap_router_from_env`].
pub fn wrap_router(router: Router) -> Router {
    wrap_im_service_router_with_manifest(router, route_manifest())
}

/// Bootstrap from environment (split-deploy service processes with IAM
/// database lookup). Passes the real route manifest so
/// `IamAuthorizationPolicy` and route-level metrics dimensions are enforced.
pub async fn wrap_router_from_env(router: Router) -> Router {
    wrap_im_open_api_service_router_from_env(route_manifest(), router).await
}
