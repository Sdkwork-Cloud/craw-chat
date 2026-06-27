use std::sync::Arc;

use axum::Router;
use projection_service::{default_projection_service, TimelineProjectionService};

pub fn build_api_router() -> Router {
    build_api_router_with_service(default_projection_service())
}

pub fn build_api_router_with_service(service: Arc<TimelineProjectionService>) -> Router {
    projection_service::http::build_domain_api_router(service)
}

pub fn build_supplemental_api_router() -> Router {
    build_supplemental_api_router_with_service(default_projection_service())
}

pub fn build_supplemental_api_router_with_service(
    service: Arc<TimelineProjectionService>,
) -> Router {
    projection_service::http::build_supplemental_domain_api_router(service)
}
