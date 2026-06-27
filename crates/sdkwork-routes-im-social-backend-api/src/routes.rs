use std::sync::Arc;

use axum::Router;
use social_service::friendship::AppState;
use social_service::{build_control_domain_api_router, SocialRuntime};

pub fn build_control_app(state: AppState) -> Router {
    build_control_domain_api_router(state)
}

pub fn build_control_public_router(social_runtime: Arc<SocialRuntime>) -> Router {
    social_service::build_control_public_router(social_runtime)
}
