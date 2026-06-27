use axum::Router;
use conversation_runtime::http::AppState;

pub fn build_api_router(state: AppState) -> Router {
    conversation_runtime::http::build_domain_api_router(state)
}
