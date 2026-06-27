use axum::Router;

pub fn build_api_router() -> Router {
    ops_service::build_domain_api_router(ops_service::default_app_state())
}
