use std::sync::Arc;

use axum::Router;

pub fn build_api_router_with_runtime(
    runtime: Arc<automation_service::AutomationRuntime>,
) -> Router {
    automation_service::build_app_api_router(automation_service::AppState::new(runtime))
}
