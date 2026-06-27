use axum::Router;
use sdkwork_im_web_bootstrap::{
    wrap_im_open_api_service_router, wrap_im_open_api_service_router_from_env,
};

use crate::manifest::route_manifest;

/// Wrap realtime HTTP routes with the SDKWork interceptor pipeline.
///
/// Realtime websocket upgrade (`/im/v3/api/realtime/ws`) must stay outside this wrapper so Axum can
/// preserve websocket upgrade state for browser clients.
pub fn wrap_http_router(router: Router) -> Router {
    wrap_im_open_api_service_router(router)
}

/// Bootstrap HTTP routes with IAM resolver and route manifest from environment.
pub async fn wrap_http_router_from_env(router: Router) -> Router {
    wrap_im_open_api_service_router_from_env(route_manifest(), router).await
}

/// Route manifest used when wrapping HTTP realtime routes with IAM interceptors.
#[allow(dead_code)]
pub fn route_manifest_for_wrap() -> sdkwork_web_core::HttpRouteManifest {
    route_manifest()
}

#[cfg(test)]
mod tests {
    use super::route_manifest_for_wrap;
    use sdkwork_im_realtime_api_paths::REALTIME_WS;
    use sdkwork_web_contract::RouteAuth;

    #[test]
    fn route_manifest_marks_realtime_websocket_as_public() {
        let manifest = route_manifest_for_wrap();
        let route = manifest
            .routes()
            .iter()
            .find(|route| route.path == REALTIME_WS)
            .expect("realtime websocket route must exist in manifest");
        assert_eq!(route.auth, RouteAuth::Public);
    }
}
