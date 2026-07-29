use axum::Router;
use std::sync::Arc;

pub fn build_api_router_with_runtime(runtime: Arc<ops_service::OpsRuntime>) -> Router {
    ops_service::build_domain_api_router(ops_service::AppState::new(runtime))
}
