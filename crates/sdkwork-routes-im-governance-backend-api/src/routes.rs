use axum::Router;

pub fn build_api_router() -> Router {
    governance_service::build_domain_api_router(governance_service::default_control_state())
}
