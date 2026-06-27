use axum::Router;

pub fn build_api_router() -> Router {
    notification_service::build_domain_api_router(notification_service::default_app_state())
}
