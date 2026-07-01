use axum::Router;
use sdkwork_im_web_bootstrap::{
    wrap_im_open_api_service_router_from_env, wrap_im_service_router_with_manifest,
};

use crate::manifest::route_manifest;

/// Wrap realtime HTTP routes with the SDKWork interceptor pipeline and the
/// actual [`HttpRouteManifest`] declared by `manifest::route_manifest()`.
///
/// Passing the real route table (instead of an empty manifest) enables
/// `IamAuthorizationPolicy` enforcement, route-level HTTP metrics
/// dimensions, and OpenAPI metadata consistency per `API_SPEC.md` §4.5,
/// §14, and §15.
///
/// Realtime websocket upgrade (`/im/v3/api/realtime/ws`) must stay outside
/// this wrapper so Axum can preserve websocket upgrade state for browser
/// clients.
pub fn wrap_http_router(router: Router) -> Router {
    wrap_im_service_router_with_manifest(router, route_manifest())
}

/// Bootstrap HTTP routes with IAM resolver and route manifest from environment.
pub async fn wrap_http_router_from_env(router: Router) -> Router {
    wrap_im_open_api_service_router_from_env(route_manifest(), router).await
}

#[cfg(test)]
mod tests {
    use super::route_manifest;
    use sdkwork_im_realtime_api_paths::REALTIME_WS;
    use sdkwork_web_contract::RouteAuth;

    #[test]
    fn route_manifest_marks_realtime_websocket_as_public() {
        let manifest = route_manifest();
        let route = manifest
            .routes()
            .iter()
            .find(|route| route.path == REALTIME_WS)
            .expect("realtime websocket route must exist in manifest");
        assert_eq!(route.auth, RouteAuth::Public);
    }
}
