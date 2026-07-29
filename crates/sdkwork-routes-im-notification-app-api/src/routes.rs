use axum::Router;
use std::sync::Arc;

pub fn build_api_router_with_runtime(
    runtime: Arc<notification_service::NotificationRuntime>,
) -> Router {
    notification_service::build_domain_api_router(notification_service::AppState::new(runtime))
}
