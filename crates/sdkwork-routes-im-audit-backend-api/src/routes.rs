use axum::Router;

pub fn build_api_router() -> Router {
    audit_service::build_domain_api_router(audit_service::default_app_state())
}
