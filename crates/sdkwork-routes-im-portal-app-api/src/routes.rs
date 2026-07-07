use axum::Router;

pub fn build_api_router() -> Router {
    portal_service::build_domain_api_router(portal_service::default_app_state())
}
