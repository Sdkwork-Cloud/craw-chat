use tokio::task::JoinHandle;

use crate::{
    RealtimeAuthContextResolver, RealtimePlaneBootstrap, bootstrap_realtime_plane_from_env,
    spawn_cluster_route_event_subscriber, spawn_link_transport_listeners,
};

/// Runtime handles produced when a gateway process embeds the session-gateway realtime plane.
pub struct GatewayEmbeddedRealtimePlane {
    pub bootstrap: RealtimePlaneBootstrap,
    pub link_transport_handles: Vec<JoinHandle<()>>,
    pub cluster_subscriber: Option<std::thread::JoinHandle<()>>,
    pub maintenance_handle: Option<JoinHandle<()>>,
}

impl GatewayEmbeddedRealtimePlane {
    pub fn node_id(&self) -> &str {
        self.bootstrap.node_id.as_str()
    }
}

/// Bootstraps the embedded realtime plane (stores, cluster bus, link listeners) from env.
pub async fn bootstrap_gateway_embedded_realtime_plane()
-> Result<GatewayEmbeddedRealtimePlane, String> {
    let bootstrap = bootstrap_realtime_plane_from_env().await?;
    let node_id = bootstrap.node_id.clone();
    let cluster_subscriber = spawn_cluster_route_event_subscriber(&bootstrap);
    let link_transport_handles = spawn_link_transport_listeners(
        bootstrap.assembly.clone(),
        node_id.as_str(),
        RealtimeAuthContextResolver::new(bootstrap.iam_auth_pool.clone()),
    );
    let maintenance_handle =
        crate::maintenance::spawn_realtime_maintenance_jobs(bootstrap.assembly.clone());
    Ok(GatewayEmbeddedRealtimePlane {
        bootstrap,
        link_transport_handles,
        cluster_subscriber,
        maintenance_handle,
    })
}
