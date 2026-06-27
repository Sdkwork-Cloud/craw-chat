use axum::Router;

pub fn build_api_router() -> Router {
    streaming_service::build_domain_api_router(streaming_service::default_app_state())
}
