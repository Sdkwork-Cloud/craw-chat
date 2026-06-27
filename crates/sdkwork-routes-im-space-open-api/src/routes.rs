use axum::Router;
use space_service::http::AppState;

pub fn build_api_router(state: AppState) -> Router {
    space_service::http::build_embedded_app(state)
}
