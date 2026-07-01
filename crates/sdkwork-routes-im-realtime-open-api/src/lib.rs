mod manifest;
mod paths;
mod routes;
mod web_bootstrap;

pub use manifest::{route_manifest, API_SURFACE};
pub use paths::PREFIX;

use axum::Router;
use session_gateway::RealtimePlaneAssembly;
use session_gateway::RealtimePlaneBootstrap;

fn compose_public_app(websocket_router: Router, http_router: Router) -> Router {
    websocket_router.merge(web_bootstrap::wrap_http_router(http_router))
}

async fn compose_public_app_from_env(websocket_router: Router, http_router: Router) -> Router {
    websocket_router.merge(web_bootstrap::wrap_http_router_from_env(http_router).await)
}

fn public_http_router_from_bootstrap(bootstrap: &RealtimePlaneBootstrap) -> Router {
    routes::build_api_router_from_bootstrap(bootstrap)
}

fn public_websocket_router_from_bootstrap(bootstrap: &RealtimePlaneBootstrap) -> Router {
    session_gateway::build_realtime_websocket_router(
        session_gateway::AppState::from_realtime_bootstrap(bootstrap),
    )
}

pub fn build_public_app() -> Router {
    let state = session_gateway::default_app_state();
    compose_public_app(
        session_gateway::build_realtime_websocket_router(state.clone()),
        routes::build_api_router(),
    )
}

pub fn build_public_app_with_realtime_plane(
    assembly: RealtimePlaneAssembly,
    node_id: &str,
) -> Router {
    build_public_app_with_realtime_bootstrap(&RealtimePlaneBootstrap {
        assembly,
        node_id: node_id.to_owned(),
        cluster_bus: None,
        iam_auth_pool: None,
    })
}

pub fn build_public_app_with_realtime_bootstrap(
    bootstrap: &RealtimePlaneBootstrap,
) -> Router {
    compose_public_app(
        public_websocket_router_from_bootstrap(bootstrap),
        public_http_router_from_bootstrap(bootstrap),
    )
}

pub async fn build_public_app_with_realtime_bootstrap_from_env(
    bootstrap: &RealtimePlaneBootstrap,
) -> Router {
    compose_public_app_from_env(
        public_websocket_router_from_bootstrap(bootstrap),
        public_http_router_from_bootstrap(bootstrap),
    )
    .await
}

pub fn gateway_route_manifest() -> sdkwork_web_core::HttpRouteManifest {
    route_manifest()
}

pub fn gateway_mount() -> axum::Router {
    build_public_app()
}
