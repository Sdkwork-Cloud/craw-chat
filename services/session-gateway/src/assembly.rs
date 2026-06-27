use std::sync::Arc;

use im_platform_contracts::RealtimeEventWindowStore;
use sdkwork_im_contract_control::{
    PresenceStateStore, RealtimeCheckpointStore, RealtimeDisconnectFenceStore,
    RealtimeSubscriptionStore,
};

use crate::{PresenceRuntime, RealtimeClusterBridge, RealtimeDeliveryRuntime};

#[derive(Clone)]
pub struct RealtimePlaneAssembly {
    realtime_cluster: Arc<RealtimeClusterBridge>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    presence_runtime: Arc<PresenceRuntime>,
}

impl Default for RealtimePlaneAssembly {
    fn default() -> Self {
        Self::new(
            Arc::new(RealtimeClusterBridge::default()),
            Arc::new(RealtimeDeliveryRuntime::standalone_gateway()),
            Arc::new(PresenceRuntime::default()),
        )
    }
}

impl RealtimePlaneAssembly {
    pub fn new(
        realtime_cluster: Arc<RealtimeClusterBridge>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
        presence_runtime: Arc<PresenceRuntime>,
    ) -> Self {
        Self {
            realtime_cluster,
            realtime_runtime,
            presence_runtime,
        }
    }

    pub fn with_cluster(realtime_cluster: Arc<RealtimeClusterBridge>) -> Self {
        Self::new(
            realtime_cluster,
            Arc::new(RealtimeDeliveryRuntime::standalone_gateway()),
            Arc::new(PresenceRuntime::default()),
        )
    }

    pub fn with_cluster_and_runtime(
        realtime_cluster: Arc<RealtimeClusterBridge>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    ) -> Self {
        Self::new(
            realtime_cluster,
            realtime_runtime,
            Arc::new(PresenceRuntime::default()),
        )
    }

    pub fn with_stores<D, C, S, E, P>(
        disconnect_fence_store: Arc<D>,
        checkpoint_store: Arc<C>,
        subscription_store: Arc<S>,
        event_window_store: Arc<E>,
        presence_state_store: Arc<P>,
    ) -> Self
    where
        D: RealtimeDisconnectFenceStore + 'static,
        C: RealtimeCheckpointStore + 'static,
        S: RealtimeSubscriptionStore + 'static,
        E: RealtimeEventWindowStore + 'static,
        P: PresenceStateStore + 'static,
    {
        Self::new(
            Arc::new(RealtimeClusterBridge::with_disconnect_fence_store(
                disconnect_fence_store,
            )),
            Arc::new(
                RealtimeDeliveryRuntime::with_durable_stores_for_standalone_gateway(
                    checkpoint_store,
                    subscription_store,
                    event_window_store,
                ),
            ),
            Arc::new(PresenceRuntime::with_store(presence_state_store)),
        )
    }

    pub fn bind_node_runtime(&self, node_id: &str) {
        self.realtime_cluster
            .bind_node_runtime(node_id, self.realtime_runtime.clone());
    }

    pub fn realtime_cluster(&self) -> Arc<RealtimeClusterBridge> {
        self.realtime_cluster.clone()
    }

    pub fn realtime_runtime(&self) -> Arc<RealtimeDeliveryRuntime> {
        self.realtime_runtime.clone()
    }

    pub fn presence_runtime(&self) -> Arc<PresenceRuntime> {
        self.presence_runtime.clone()
    }
}
