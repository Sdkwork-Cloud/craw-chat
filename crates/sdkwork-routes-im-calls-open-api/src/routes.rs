use axum::Router;

pub fn build_api_router() -> Router {
    calls_service::build_domain_api_router(calls_service::default_app_state())
}
