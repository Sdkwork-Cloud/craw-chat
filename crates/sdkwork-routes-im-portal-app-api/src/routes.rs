use std::sync::Arc;

use axum::Router;

pub fn build_api_router_with_runtime(runtime: Arc<portal_service::PortalRuntime>) -> Router {
    portal_service::build_domain_api_router(portal_service::AppState::new(runtime))
}
