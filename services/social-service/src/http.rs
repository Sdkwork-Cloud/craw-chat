//! Social Service HTTP helpers retained for in-process infra probes only.

use std::sync::Arc;

use axum::Router;
use axum::response::IntoResponse;
use axum::routing::get;
use sdkwork_im_web_bootstrap::{
    im_service_http_metrics, im_service_router_config, mount_im_infra_routes,
};
use sdkwork_web_core::HttpMetricsRegistry;

use crate::render_shared_channel_sync_prometheus_from_env;
use crate::runtime::SocialRuntime;

pub fn build_app(_social_runtime: Arc<SocialRuntime>) -> Router {
    mount_social_infra_routes(Router::new())
}

/// Mount IM infra routes with a custom `/metrics` handler that also renders
/// shared-channel sync metrics (`im_shared_channel_sync_*`, `im_health_status`).
fn mount_social_infra_routes(router: Router) -> Router {
    let config = im_service_router_config().skip_metrics();
    let http_metrics = config.metrics().unwrap_or_else(im_service_http_metrics);
    mount_im_infra_routes(router, config).route(
        "/metrics",
        get(move || {
            let metrics = http_metrics.clone();
            async move { social_metrics_handler(metrics).await }
        }),
    )
}

async fn social_metrics_handler(http_metrics: Arc<HttpMetricsRegistry>) -> impl IntoResponse {
    let mut output = http_metrics.render_prometheus();
    output.push('\n');
    output.push_str(&render_shared_channel_sync_prometheus_from_env());
    output.push('\n');
    output.push_str(&crate::render_social_write_prometheus());
    (
        axum::http::StatusCode::OK,
        [(
            axum::http::header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8",
        )],
        output,
    )
}
