use axum::Router;

pub fn build_api_router() -> Router {
    media_service::build_domain_api_router(media_service::default_app_state())
}
