use axum::Router;

pub fn build_api_router() -> Router {
    automation_service::build_domain_api_router(automation_service::default_app_state())
}
