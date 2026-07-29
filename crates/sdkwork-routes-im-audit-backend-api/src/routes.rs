use std::sync::Arc;

use axum::Router;

pub fn build_api_router_with_runtime(runtime: Arc<audit_service::AuditRuntime>) -> Router {
    audit_service::build_domain_api_router(audit_service::AppState::new(runtime))
}
