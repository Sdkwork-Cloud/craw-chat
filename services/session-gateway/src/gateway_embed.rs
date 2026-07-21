use std::time::{Duration, Instant};

use tokio::task::JoinHandle;

use crate::{
    RealtimeAuthContextResolver, RealtimePlaneBootstrap, bootstrap_realtime_plane_from_env,
    spawn_cluster_route_event_subscriber, spawn_link_transport_listeners,
};

const ROUTE_DRAIN_BATCH_SIZE: usize = 256;

/// Runtime handles produced when a gateway process embeds the session-gateway realtime plane.
pub struct GatewayEmbeddedRealtimePlane {
    pub bootstrap: RealtimePlaneBootstrap,
    pub link_transport_handles: Vec<JoinHandle<()>>,
    pub cluster_subscriber: Option<crate::runtime_bootstrap::ClusterRouteEventSubscriber>,
    pub maintenance_handle: Option<JoinHandle<()>>,
}

impl GatewayEmbeddedRealtimePlane {
    pub fn node_id(&self) -> &str {
        self.bootstrap.node_id.as_str()
    }

    pub async fn shutdown(self, timeout: Duration) -> Result<(), String> {
        let started_at = Instant::now();
        let deadline = started_at + timeout;
        let node_id = self.bootstrap.node_id.clone();
        let readiness = self.bootstrap.assembly.readiness();
        let cluster = self.bootstrap.assembly.realtime_cluster();
        let routes_before = cluster.route_count_for_node(node_id.as_str());

        readiness.mark_draining();
        let lifecycle_error = cluster
            .mark_node_draining(node_id.as_str())
            .err()
            .map(|error| error.message);
        tracing::info!(
            target: "sdkwork.im",
            event = "im.session_gateway.embedded_drain_started",
            node_id = %node_id,
            routes_before,
            timeout_secs = timeout.as_secs(),
            "embedded session-gateway drain started"
        );

        abort_tasks(self.link_transport_handles).await;
        if let Some(handle) = self.maintenance_handle {
            abort_tasks(vec![handle]).await;
        }

        let mut route_drain_error = None;
        loop {
            if !cluster.has_routes_for_node(node_id.as_str()) {
                break;
            }
            match cluster
                .fence_and_release_node_routes_batch(node_id.as_str(), ROUTE_DRAIN_BATCH_SIZE)
            {
                Ok(_) => route_drain_error = None,
                Err(error) => route_drain_error = Some(error.message),
            }
            if !cluster.has_routes_for_node(node_id.as_str()) {
                break;
            }
            let remaining = remaining_duration(deadline);
            if remaining.is_zero() {
                break;
            }
            tokio::time::sleep(remaining.min(Duration::from_millis(100))).await;
        }

        if let Some(subscriber) = self.cluster_subscriber {
            subscriber.shutdown(remaining_duration(deadline)).await;
        }

        let remaining_routes = cluster.route_count_for_node(node_id.as_str());
        cluster.unbind_node_runtime(node_id.as_str());
        tracing::info!(
            target: "sdkwork.im",
            event = "im.session_gateway.embedded_drain_completed",
            node_id = %node_id,
            remaining_routes,
            elapsed_ms = started_at.elapsed().as_millis() as u64,
            "embedded session-gateway drain completed"
        );

        if let Some(error) = lifecycle_error {
            return Err(format!(
                "mark embedded session-gateway node draining failed: {error}"
            ));
        }
        if remaining_routes > 0 {
            return Err(format!(
                "embedded session-gateway drain deadline reached with {remaining_routes} route(s) remaining{}",
                route_drain_error
                    .as_deref()
                    .map(|error| format!(": {error}"))
                    .unwrap_or_default()
            ));
        }
        Ok(())
    }
}

async fn abort_tasks(handles: Vec<JoinHandle<()>>) {
    for handle in &handles {
        handle.abort();
    }
    for handle in handles {
        let _ = handle.await;
    }
}

fn remaining_duration(deadline: Instant) -> Duration {
    deadline.saturating_duration_since(Instant::now())
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{RealtimePlaneAssembly, RealtimePlaneBootstrap};

    use super::GatewayEmbeddedRealtimePlane;

    #[tokio::test]
    async fn embedded_plane_shutdown_fences_routes_and_fails_readiness_closed() {
        let assembly = RealtimePlaneAssembly::default();
        let cluster = assembly.realtime_cluster();
        let readiness = assembly.readiness();
        let node_id = "node_embedded";
        assembly.bind_node_runtime(node_id);
        cluster
            .bind_client_route_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                node_id,
                Some("s_live"),
                "websocket",
            )
            .expect("embedded route should bind before shutdown");
        let plane = GatewayEmbeddedRealtimePlane {
            bootstrap: RealtimePlaneBootstrap {
                assembly,
                node_id: node_id.to_owned(),
                cluster_bus: None,
                iam_auth_pool: None,
            },
            link_transport_handles: Vec::new(),
            cluster_subscriber: None,
            maintenance_handle: None,
        };

        tokio::time::timeout(
            Duration::from_millis(50),
            plane.shutdown(Duration::from_secs(1)),
        )
        .await
        .expect("a successful route drain must not wait for the retry interval")
        .expect("embedded plane should drain within its deadline");

        assert!(readiness.is_draining());
        assert!(cluster.routes_for_node(node_id).is_empty());
        assert!(
            cluster
                .disconnect_fence_matches_client_route_session_for_principal_kind(
                    "100001",
                    "default",
                    "1",
                    "user",
                    "d_pad",
                    Some("s_live"),
                )
                .expect("drain fence should remain readable")
        );
    }
}
