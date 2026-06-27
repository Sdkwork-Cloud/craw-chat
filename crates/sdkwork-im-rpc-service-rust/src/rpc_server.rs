use std::sync::Arc;

use sdkwork_rpc_discovery::DiscoveryInstanceHandle;
use tonic::transport::server::Router;

use crate::ImRpcServerConfig;

pub async fn serve_im_rpc_with_discovery<F>(
    router: Router,
    config: &ImRpcServerConfig,
    discovery: Option<Arc<DiscoveryInstanceHandle>>,
    shutdown: F,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    if let Some(discovery_handle) = discovery {
        sdkwork_rpc_server::serve_with_discovery_lifecycle(
            router,
            &config.bind_addr,
            discovery_handle,
            shutdown,
            None,
        )
        .await
        .map_err(Into::into)
    } else {
        sdkwork_rpc_server::serve_with_graceful_shutdown(router, &config.bind_addr, shutdown)
            .await
            .map_err(Into::into)
    }
}
