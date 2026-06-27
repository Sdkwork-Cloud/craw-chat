use std::sync::Arc;

use im_app_context::AppContext;
use tokio::sync::watch;

use super::ApiError;
use super::client_route_state::ClientRouteState;
use super::cluster::RealtimeClusterBridge;
use super::presence::PresenceRuntime;
use super::realtime::RealtimeDeliveryRuntime;

#[derive(Clone)]
pub(crate) struct ClientRouteRegistration {
    node_id: String,
    realtime_cluster: Arc<RealtimeClusterBridge>,
    presence_runtime: Arc<PresenceRuntime>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    client_route_state: ClientRouteState,
}

impl ClientRouteRegistration {
    pub(crate) fn new(
        node_id: String,
        realtime_cluster: Arc<RealtimeClusterBridge>,
        presence_runtime: Arc<PresenceRuntime>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
        client_route_state: ClientRouteState,
    ) -> Self {
        Self {
            node_id,
            realtime_cluster,
            presence_runtime,
            realtime_runtime,
            client_route_state,
        }
    }

    #[rustfmt::skip]
    pub(crate) fn register_client_route(
        &self,
        auth: &AppContext,
        device_id: &str,
        connection_kind: &str,
        allow_session_takeover: bool,
    ) -> Result<(), ApiError> {
        self.client_route_state
            .ensure_route_key_available(auth, device_id)?;
        let tenant_id = auth.tenant_id.as_str();
        let organization_id = auth.organization_id.as_str();
        let principal_id = auth.actor_id.as_str();
        let principal_kind = auth.actor_kind.as_str();
        let session_id = auth.session_id.as_deref();
        if !allow_session_takeover {
            self.realtime_cluster
                .ensure_client_route_resume_not_required_for_principal_kind(
                    tenant_id,
                    organization_id,
                    principal_id,
                    principal_kind,
                    device_id,
                )?;
            self.presence_runtime
                .ensure_client_route_resume_not_required(auth, device_id)?;
        }
        self.presence_runtime
            .register_client_route(auth, device_id)?;
        self.realtime_runtime
            .ensure_client_route_state_for_principal_kind(
                tenant_id,
                organization_id,
                principal_id,
                principal_kind,
                device_id,
            )?;
        self.client_route_state.register_route_key(auth, device_id);
        self.realtime_cluster.bind_client_route_for_principal_kind(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            self.node_id.as_str(),
            session_id,
            connection_kind,
        )?;
        if allow_session_takeover {
            self.realtime_cluster
                .clear_client_route_disconnect_fence_for_current_session(
                    tenant_id,
                    organization_id,
                    principal_id,
                    principal_kind,
                    device_id,
                    session_id,
                )?;
        }
        Ok(())
    }

    pub(crate) fn prepare_active_client_route(
        &self,
        auth: &AppContext,
        device_id: &str,
        connection_kind: &str,
        allow_session_takeover: bool,
    ) -> Result<(), ApiError> {
        self.client_route_state
            .ensure_route_key_available(auth, device_id)?;
        self.ensure_route_session_current(auth, device_id, auth.session_id.as_deref())?;
        self.register_client_route(auth, device_id, connection_kind, allow_session_takeover)?;
        Ok(())
    }

    pub(crate) fn current_active_client_route(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Option<super::RealtimeClientRoute> {
        self.realtime_cluster
            .resolve_client_route_for_principal_kind(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                device_id,
            )
    }

    pub(crate) fn restore_active_client_route_if_current(
        &self,
        expected_current: &super::RealtimeClientRoute,
        restore_to: super::RealtimeClientRoute,
    ) -> Option<super::RealtimeClientRoute> {
        self.realtime_cluster
            .restore_client_route_if_current(expected_current, restore_to)
    }

    pub(crate) fn release_active_client_route_if_current_session(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) {
        if self
            .ensure_route_session_current(auth, device_id, auth.session_id.as_deref())
            .is_err()
        {
            return;
        }

        let _ = self
            .realtime_cluster
            .release_client_route_for_principal_kind(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                device_id,
                self.node_id.as_str(),
            );
    }

    pub(crate) fn ensure_active_client_route_current_session(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Result<(), ApiError> {
        self.client_route_state
            .ensure_route_key_available(auth, device_id)?;
        self.ensure_route_session_current(auth, device_id, auth.session_id.as_deref())
    }

    pub(crate) fn subscribe_active_client_route_epoch(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Result<watch::Receiver<u64>, ApiError> {
        self.client_route_state
            .ensure_route_key_available(auth, device_id)?;
        Ok(self
            .realtime_cluster
            .subscribe_client_route_epoch_for_principal_kind(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                device_id,
            ))
    }

    fn ensure_route_session_current(
        &self,
        auth: &AppContext,
        device_id: &str,
        session_id: Option<&str>,
    ) -> Result<(), ApiError> {
        let tenant_id = auth.tenant_id.as_str();
        let organization_id = auth.organization_id.as_str();
        let principal_id = auth.actor_id.as_str();
        self.realtime_cluster
            .ensure_route_session_current_for_principal_kind(
                tenant_id,
                organization_id,
                principal_id,
                auth.actor_kind.as_str(),
                device_id,
                session_id,
            )?;
        Ok(())
    }
}
