use axum::Router;
use session_gateway::RealtimePlaneBootstrap;

pub fn build_api_router() -> Router {
    session_gateway::build_domain_api_router(session_gateway::default_app_state())
}

pub fn build_api_router_from_bootstrap(bootstrap: &RealtimePlaneBootstrap) -> Router {
    session_gateway::build_public_http_app(session_gateway::AppState::from_realtime_bootstrap(
        bootstrap,
    ))
}
