//! Gateway shared state assembled into the axum [`Router`] state slot.

use std::sync::Arc;

use crate::anomaly_detector::AnomalyDetector;
use crate::gateway_protection::CircuitBreakerRegistry;
use crate::openapi::OpenApiAggregateCache;
use axum::Router;
use reqwest::Client;
use sdkwork_im_api_registry::RouteRegistry;
use sdkwork_im_cloud_gateway_config::WebGatewayConfig;
use session_gateway::RealtimeAuthContextResolver;
use tokio::sync::Semaphore;

/// Shared state cloned into every gateway handler.
///
/// Constructed by the `build_app*` family in [`crate::app`] and threaded through
/// the axum router as `State<GatewayState>`.
#[derive(Clone)]
pub struct GatewayState {
    pub(crate) client: Client,
    pub(crate) config: WebGatewayConfig,
    pub(crate) registry: RouteRegistry,
    pub(crate) product_runtime_router: Option<Router>,
    pub(crate) embedded_session_gateway: Option<Router>,
    pub(crate) realtime_auth: RealtimeAuthContextResolver,
    pub(crate) circuit_breakers: CircuitBreakerRegistry,
    pub(crate) anomaly_detector: Arc<AnomalyDetector>,
    pub(crate) openapi_aggregate_cache: OpenApiAggregateCache,
    pub(crate) websocket_connection_semaphore: Arc<Semaphore>,
}
