use axum::Router;
use sdkwork_im_cloud_gateway_config::{should_embed_session_gateway, WebGatewayConfig};
use tokio::task::JoinHandle;
use tracing::warn;

pub struct EmbeddedSessionGatewayRuntime {
    pub session_router: Option<Router>,
    pub embedded_realtime_app_state: Option<session_gateway::AppState>,
    pub link_transport_handles: Vec<JoinHandle<()>>,
    pub cluster_subscriber: Option<std::thread::JoinHandle<()>>,
}

impl EmbeddedSessionGatewayRuntime {
    pub fn empty() -> Self {
        Self {
            session_router: None,
            embedded_realtime_app_state: None,
            link_transport_handles: Vec::new(),
            cluster_subscriber: None,
        }
    }

    pub async fn shutdown(mut self) {
        for handle in self.link_transport_handles {
            handle.abort();
        }
        if let Some(handle) = self.cluster_subscriber {
            let _ = handle.join();
        }
        if let Some(router) = self.session_router.take() {
            if let Err(error) = tokio::task::spawn_blocking(move || drop(router)).await {
                warn!(
                    target: "sdkwork.im",
                    event = "im.gateway.embedded_router_drop_failed",
                    error = %error,
                    "failed to drop embedded session router off async runtime"
                );
            }
        }
    }
}

/// Builds an embedded session-gateway router when unified-process layout or explicit embed env is active.
pub async fn bootstrap_embedded_session_gateway_runtime(
    config: &WebGatewayConfig,
) -> Result<EmbeddedSessionGatewayRuntime, String> {
    if !should_embed_session_gateway(config) {
        return Ok(EmbeddedSessionGatewayRuntime::empty());
    }

    let embedded = session_gateway::bootstrap_gateway_embedded_realtime_plane().await?;
    let node_id = embedded.node_id().to_owned();
    let cluster_bus_configured = embedded.bootstrap.cluster_bus.is_some();
    let _ = sdkwork_im_web_bootstrap::shared_iam_web_request_context_resolver_from_env().await;
    let embedded_realtime_app_state =
        session_gateway::AppState::from_realtime_bootstrap(&embedded.bootstrap);
    let session_router =
        sdkwork_routes_im_realtime_open_api::build_public_app_with_realtime_bootstrap_from_env(
            &embedded.bootstrap,
        )
        .await;
    tracing::info!(
        target: "sdkwork.im",
        event = "im.gateway.embed_session_gateway",
        node_id = %node_id,
        cluster_bus = cluster_bus_configured,
        runtime_mode = ?config.runtime_mode,
        "embedded session-gateway realtime plane in gateway process"
    );

    Ok(EmbeddedSessionGatewayRuntime {
        session_router: Some(session_router),
        embedded_realtime_app_state: Some(embedded_realtime_app_state),
        link_transport_handles: embedded.link_transport_handles,
        cluster_subscriber: embedded.cluster_subscriber,
    })
}
