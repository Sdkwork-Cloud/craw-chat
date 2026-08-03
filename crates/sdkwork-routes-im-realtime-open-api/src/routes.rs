use axum::Router;
use session_gateway::RealtimePlaneBootstrap;

/// Domain-only realtime HTTP surface for embedded assembly composition.
///
/// `build_public_http_app` also mounts process infrastructure routes
/// (`/openapi.json`, `/docs`, `/healthz`, `/readyz`); the composed gateway
/// assembly mounts those once itself, so embedded routers must stay
/// infrastructure-free to avoid overlapping route panics.
pub fn build_api_router() -> Router {
    session_gateway::build_domain_http_api_router(session_gateway::default_app_state())
}

pub fn build_api_router_from_bootstrap(bootstrap: &RealtimePlaneBootstrap) -> Router {
    session_gateway::build_domain_http_api_router(
        session_gateway::AppState::from_realtime_bootstrap(bootstrap),
    )
}
