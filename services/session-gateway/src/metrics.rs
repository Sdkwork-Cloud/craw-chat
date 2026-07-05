//! Prometheus metrics endpoint for the session-gateway.
//!
//! Exposes the shared SDKWork HTTP metrics registry together with
//! session-gateway-specific runtime indicators:
//! - `im_session_gateway_websocket_connections_active` — current WebSocket connections
//! - `im_session_gateway_websocket_connections_capacity` — configured max connections
//! - `im_session_gateway_realtime_subscriptions` — total realtime subscriptions
//! - `im_session_gateway_presence_entries` — total presence entries
//! - `im_session_gateway_cluster_nodes` — total realtime cluster nodes observed
//!
//! The endpoint is mounted at `/metrics` and bypasses the in-flight gate
//! (see `http_guardrails::enforce_in_flight_gate`).

use std::sync::Arc;

use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use sdkwork_im_web_bootstrap::im_service_http_metrics;
use sdkwork_web_core::HttpMetricsRegistry;

use crate::AppState;

/// Render the Prometheus exposition for the session-gateway.
///
/// Returns a `text/plain; version=0.0.4; charset=utf-8` body that concatenates
/// the shared SDKWork HTTP metrics with session-gateway runtime gauges.
pub async fn session_gateway_metrics_handler(State(state): State<AppState>) -> impl IntoResponse {
    let http_metrics = im_service_http_metrics();
    let mut output = http_metrics.render_prometheus();
    output.push('\n');
    append_session_gateway_runtime_metrics(&mut output, &http_metrics, &state);

    (
        StatusCode::OK,
        [(
            header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8",
        )],
        output,
    )
}

fn append_session_gateway_runtime_metrics(
    output: &mut String,
    http_metrics: &Arc<HttpMetricsRegistry>,
    state: &AppState,
) {
    let dimensions = http_metrics.dimensions();
    let labels = format!(
        "service=\"{}\",environment=\"{}\",deployment_profile=\"{}\",runtime_target=\"{}\"",
        dimensions.service,
        dimensions.environment,
        dimensions.deployment_profile,
        dimensions.runtime_target,
    );

    let active_connections = state.websocket_active_connections();
    let capacity = state.websocket_connection_capacity();

    output.push_str("# HELP im_session_gateway_websocket_connections_active Currently active WebSocket connections on this node.\n");
    output.push_str("# TYPE im_session_gateway_websocket_connections_active gauge\n");
    output.push_str(&format!(
        "im_session_gateway_websocket_connections_active{{{labels}}} {active_connections}\n"
    ));

    output.push_str("# HELP im_session_gateway_websocket_connections_capacity Configured WebSocket connection capacity for this node.\n");
    output.push_str("# TYPE im_session_gateway_websocket_connections_capacity gauge\n");
    output.push_str(&format!(
        "im_session_gateway_websocket_connections_capacity{{{labels}}} {capacity}\n"
    ));

    let realtime_subscription_count = state.realtime_subscription_count();
    output.push_str("# HELP im_session_gateway_realtime_subscriptions Total realtime subscriptions on this node.\n");
    output.push_str("# TYPE im_session_gateway_realtime_subscriptions gauge\n");
    output.push_str(&format!(
        "im_session_gateway_realtime_subscriptions{{{labels}}} {realtime_subscription_count}\n"
    ));

    let presence_entries = state.presence_entry_count();
    output.push_str(
        "# HELP im_session_gateway_presence_entries Total presence entries tracked on this node.\n",
    );
    output.push_str("# TYPE im_session_gateway_presence_entries gauge\n");
    output.push_str(&format!(
        "im_session_gateway_presence_entries{{{labels}}} {presence_entries}\n"
    ));

    let cluster_node_count = state.cluster_node_count();
    output.push_str("# HELP im_session_gateway_cluster_nodes Total realtime cluster nodes observed by this node.\n");
    output.push_str("# TYPE im_session_gateway_cluster_nodes gauge\n");
    output.push_str(&format!(
        "im_session_gateway_cluster_nodes{{{labels}}} {cluster_node_count}\n"
    ));
}
