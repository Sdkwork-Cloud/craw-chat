use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, Mutex, MutexGuard};

use im_app_context::AppContext;

use super::ApiError;
use super::principal_scope::{
    tenant_client_route_scope_key, typed_client_route_scope_key, typed_principal_scope_key,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ClientRouteStateSnapshot {
    pub(crate) registered_route_keys: Vec<String>,
    pub(crate) latest_sync_seq: u64,
}

#[derive(Clone, Default)]
pub(crate) struct ClientRouteState {
    registered_route_keys: Arc<Mutex<HashMap<String, BTreeSet<String>>>>,
    latest_sync_sequences: Arc<Mutex<HashMap<String, u64>>>,
    route_owner_scopes: Arc<Mutex<HashMap<String, String>>>,
}

impl ClientRouteState {
    pub(crate) fn ensure_route_key_available(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Result<(), ApiError> {
        // The route key is principal-scoped for sync state, but remains tenant-global at
        // registration time so the same client connection key cannot be rebound to another owner.
        let scope_key = tenant_client_route_scope_key(auth.tenant_id.as_str(), device_id);
        let requested_owner_scope = typed_principal_scope_key(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        );
        if let Some(existing_owner_scope) =
            lock_client_route_mutex(&self.route_owner_scopes, "client route owner store")
                .get(scope_key.as_str())
                .cloned()
            && existing_owner_scope != requested_owner_scope
        {
            return Err(ApiError::conflict(
                "client_route_scope_conflict",
                format!("client route already bound to a different principal: {device_id}"),
            ));
        }

        Ok(())
    }

    pub(crate) fn register_route_key(&self, auth: &AppContext, device_id: &str) {
        lock_client_route_mutex(&self.registered_route_keys, "client route key store")
            .entry(typed_principal_scope_key(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
            ))
            .or_default()
            .insert(device_id.into());
        lock_client_route_mutex(&self.latest_sync_sequences, "latest sync sequence store")
            .entry(typed_client_route_scope_key(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                device_id,
            ))
            .or_insert(0);
        lock_client_route_mutex(&self.route_owner_scopes, "client route owner store")
            .entry(tenant_client_route_scope_key(
                auth.tenant_id.as_str(),
                device_id,
            ))
            .or_insert_with(|| {
                typed_principal_scope_key(
                    auth.tenant_id.as_str(),
                    auth.organization_id.as_str(),
                    auth.actor_id.as_str(),
                    auth.actor_kind.as_str(),
                )
            });
    }

    #[cfg(test)]
    pub(crate) fn has_registered_route_key(&self, auth: &AppContext, device_id: &str) -> bool {
        lock_client_route_mutex(&self.registered_route_keys, "client route key store")
            .get(
                typed_principal_scope_key(
                    auth.tenant_id.as_str(),
                    auth.organization_id.as_str(),
                    auth.actor_id.as_str(),
                    auth.actor_kind.as_str(),
                )
                .as_str(),
            )
            .is_some_and(|items| items.contains(device_id))
    }

    pub(crate) fn client_route_state_snapshot(
        &self,
        auth: &AppContext,
        requested_device_id: Option<&str>,
    ) -> Result<ClientRouteStateSnapshot, ApiError> {
        if let (Some(requested), Some(bound)) = (requested_device_id, auth.device_id.as_deref())
            && requested != bound
        {
            return Err(ApiError::bad_request(
                "device_id_mismatch",
                format!("device id does not match auth context: {requested}"),
            ));
        }

        let latest_sync_seq = requested_device_id
            .or(auth.device_id.as_deref())
            .map(|device_id| {
                self.latest_route_sync_seq(
                    auth.tenant_id.as_str(),
                    auth.organization_id.as_str(),
                    auth.actor_id.as_str(),
                    auth.actor_kind.as_str(),
                    device_id,
                )
            })
            .unwrap_or_default();

        Ok(ClientRouteStateSnapshot {
            registered_route_keys: self.registered_route_keys(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
            ),
            latest_sync_seq,
        })
    }

    #[rustfmt::skip]
    fn registered_route_keys(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Vec<String> {
        lock_client_route_mutex(&self.registered_route_keys, "client route key store")
            .get(
                typed_principal_scope_key(tenant_id, organization_id, principal_id, principal_kind)
                    .as_str(),
            )
            .map(|items| items.iter().cloned().collect())
            .unwrap_or_default()
    }

    fn latest_route_sync_seq(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> u64 {
        lock_client_route_mutex(&self.latest_sync_sequences, "latest sync sequence store")
            .get(
                typed_client_route_scope_key(
                    tenant_id,
                    organization_id,
                    principal_id,
                    principal_kind,
                    device_id,
                )
                .as_str(),
            )
            .copied()
            .unwrap_or_default()
    }
}

fn lock_client_route_mutex<'a, T>(
    mutex: &'a Mutex<T>,
    lock_name: &'static str,
) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovered poisoned client-route mutex lock={lock_name}");
            poisoned.into_inner()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn auth_context(principal_id: &str, actor_kind: &str, device_id: &str) -> AppContext {
        AppContext {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            user_id: principal_id.into(),
            actor_id: principal_id.into(),
            actor_kind: actor_kind.into(),
            session_id: Some(format!("s_{actor_kind}")),
            app_id: None,
            environment: None,
            deployment_mode: None,
            auth_level: None,
            data_scope: Default::default(),
            permission_scope: Default::default(),
            device_id: Some(device_id.into()),
        }
    }

    #[test]
    fn test_register_route_key_recovers_from_poisoned_route_key_lock() {
        let state = ClientRouteState::default();
        let _ = std::panic::catch_unwind({
            let registered_route_keys = state.registered_route_keys.clone();
            move || {
                let _guard = registered_route_keys
                    .lock()
                    .expect("client route key store should lock");
                panic!("poison client route key store lock");
            }
        });

        let auth = auth_context("1", "user", "d_poison");
        state.register_route_key(&auth, "d_poison");
        assert!(state.has_registered_route_key(&auth, "d_poison"));
    }

    #[test]
    fn test_client_route_state_isolated_by_actor_kind_for_same_actor_id() {
        let state = ClientRouteState::default();
        let user_auth = auth_context("1", "user", "d_user");
        let agent_auth = auth_context("1", "agent", "d_agent");

        state.register_route_key(&user_auth, "d_user");
        state.register_route_key(&agent_auth, "d_agent");

        let user_state = state
            .client_route_state_snapshot(&user_auth, Some("d_user"))
            .expect("user sync state should resolve");
        let agent_state = state
            .client_route_state_snapshot(&agent_auth, Some("d_agent"))
            .expect("agent sync state should resolve");

        assert_eq!(user_state.registered_route_keys, vec!["d_user"]);
        assert_eq!(agent_state.registered_route_keys, vec!["d_agent"]);
    }

    #[test]
    fn test_route_key_conflict_rejected_for_same_actor_and_actor_kind_change() {
        let state = ClientRouteState::default();
        let user_auth = auth_context("1", "user", "d_shared");
        let agent_auth = auth_context("1", "agent", "d_shared");

        state.register_route_key(&user_auth, "d_shared");
        let error = state
            .ensure_route_key_available(&agent_auth, "d_shared")
            .expect_err("different actor kind should be rejected for same actor route key");
        assert_eq!(error.code, "client_route_scope_conflict");
    }

    #[test]
    fn test_route_key_owner_conflict_rejected_for_different_actor_same_route_key() {
        let state = ClientRouteState::default();
        let owner_a = auth_context("1001", "user", "d_shared");
        let owner_b = auth_context("1002", "user", "d_shared");

        state.register_route_key(&owner_a, "d_shared");
        let error = state
            .ensure_route_key_available(&owner_b, "d_shared")
            .expect_err("different owner should be rejected for same tenant route key");
        assert_eq!(error.code, "client_route_scope_conflict");
    }
}
