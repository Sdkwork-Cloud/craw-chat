//! Social Service HTTP helpers retained for in-process infra probes only.

use std::sync::Arc;

use axum::Router;
use sdkwork_im_web_bootstrap::{im_service_router_config, mount_im_infra_routes};

use crate::runtime::SocialRuntime;

pub fn build_app(_social_runtime: Arc<SocialRuntime>) -> Router {
    mount_im_infra_routes(Router::new(), im_service_router_config())
}
