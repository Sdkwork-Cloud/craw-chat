use im_time::rfc3339_le;
use sdkwork_im_contract_control::{RealtimeDisconnectFenceRecord, RealtimeDisconnectFenceStore};
use sdkwork_im_contract_core::ContractError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::{
    RealtimeClusterBridge, RealtimeClusterError, client_route_scope_key, cluster_timestamp,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct RealtimeDisconnectFence {
    tenant_id: String,
    organization_id: String,
    principal_id: String,
    principal_kind: String,
    device_id: String,
    session_id: Option<String>,
    pub(super) owner_node_id: String,
    disconnected_at: String,
    fence_token: String,
}

#[derive(Clone, Default)]
pub(super) struct ClusterMemoryDisconnectFenceStore {
    fences: Arc<Mutex<HashMap<String, RealtimeDisconnectFenceRecord>>>,
}

impl RealtimeDisconnectFenceStore for ClusterMemoryDisconnectFenceStore {
    fn load_fence(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeDisconnectFenceRecord>, ContractError> {
        Ok(self
            .fences
            .lock_cluster_disconnect_fences()
            .get(
                client_route_scope_key(
                    tenant_id,
                    organization_id,
                    principal_id,
                    principal_kind,
                    device_id,
                )
                .as_str(),
            )
            .cloned())
    }

    fn save_fence(&self, record: RealtimeDisconnectFenceRecord) -> Result<(), ContractError> {
        let key = client_route_scope_key(
            record.tenant_id.as_str(),
            record.organization_id.as_str(),
            record.principal_id.as_str(),
            record.principal_kind.as_str(),
            record.device_id.as_str(),
        );
        let mut fences = self.fences.lock_cluster_disconnect_fences();
        let next = fences
            .remove(key.as_str())
            .map(|previous| previous.merge_latest(record.clone()))
            .unwrap_or(record);
        fences.insert(key, next);
        Ok(())
    }

    fn clear_fence(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        Ok(self
            .fences
            .lock_cluster_disconnect_fences()
            .remove(
                client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id).as_str(),
            )
            .is_some())
    }

    fn clear_fence_disconnected_at_or_before(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        cutoff_disconnected_at: &str,
    ) -> Result<bool, ContractError> {
        let key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        let mut fences = self.fences.lock_cluster_disconnect_fences();
        let should_clear = fences
            .get(key.as_str())
            .map(|record| rfc3339_le(record.disconnected_at.as_str(), cutoff_disconnected_at))
            .unwrap_or(false);
        if !should_clear {
            return Ok(false);
        }
        Ok(fences.remove(key.as_str()).is_some())
    }

    fn clear_fence_if_matches(
        &self,
        expected: &RealtimeDisconnectFenceRecord,
    ) -> Result<bool, ContractError> {
        let key = client_route_scope_key(
            expected.tenant_id.as_str(),
            expected.organization_id.as_str(),
            expected.principal_id.as_str(),
            expected.principal_kind.as_str(),
            expected.device_id.as_str(),
        );
        let mut fences = self.fences.lock_cluster_disconnect_fences();
        let should_clear = fences
            .get(key.as_str())
            .map(|record| record == expected)
            .unwrap_or(false);
        if !should_clear {
            return Ok(false);
        }
        Ok(fences.remove(key.as_str()).is_some())
    }
}

impl ClusterMemoryDisconnectFenceStore {
    /// P1-7 fix: Expire disconnect fences older than the cutoff timestamp.
    ///
    /// This prevents storage膨胀 from long-term offline devices that accumulate
    /// stale fence records. The cleanup job should be run periodically (e.g., daily)
    /// to remove fences for devices that have been offline for more than N days.
    ///
    /// # Arguments
    ///
    /// * `cutoff_timestamp` - ISO 8601 timestamp; fences with `disconnected_at`
    ///   older than this will be removed
    ///
    /// # Returns
    ///
    /// Number of expired fences removed
    #[allow(dead_code)]
    pub fn expire_fences_older_than(&self, cutoff_timestamp: &str) -> Result<usize, ContractError> {
        let mut fences = self.fences.lock_cluster_disconnect_fences();
        let mut expired_keys = Vec::new();
        
        for (key, record) in fences.iter() {
            if rfc3339_le(record.disconnected_at.as_str(), cutoff_timestamp) {
                expired_keys.push(key.clone());
            }
        }
        
        let removed_count = expired_keys.len();
        for key in expired_keys {
            fences.remove(key.as_str());
        }
        
        Ok(removed_count)
    }
    
    /// Get the count of stored disconnect fences.
    #[allow(dead_code)]
    pub fn fence_count(&self) -> usize {
        self.fences.lock_cluster_disconnect_fences().len()
    }
}

impl RealtimeClusterBridge {
    pub fn mark_client_route_disconnected_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        session_id: Option<&str>,
        owner_node_id: &str,
    ) -> Result<(), RealtimeClusterError> {
        self.mark_client_route_disconnected_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            session_id,
            owner_node_id,
        )
    }

    fn mark_client_route_disconnected_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        session_id: Option<&str>,
        owner_node_id: &str,
    ) -> Result<(), RealtimeClusterError> {
        let scope_key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        let disconnected_at = cluster_timestamp();
        let fence = RealtimeDisconnectFence {
            tenant_id: tenant_id.into(),
            organization_id: organization_id.into(),
            principal_id: principal_id.into(),
            principal_kind: principal_kind.into(),
            device_id: device_id.into(),
            session_id: session_id.map(str::to_owned),
            owner_node_id: owner_node_id.into(),
            fence_token: disconnect_fence_token(
                tenant_id,
                principal_id,
                principal_kind,
                device_id,
                session_id,
                owner_node_id,
                disconnected_at.as_str(),
            ),
            disconnected_at,
        };
        self.disconnect_fence_store
            .save_fence(fence.to_record())
            .map_err(|error| {
                self.disconnect_fence_store_error("persist disconnect fence", owner_node_id, error)
            })?;
        self.disconnect_fences
            .lock_cluster_disconnect_fence_cache()
            .insert(scope_key, fence);
        Ok(())
    }

    pub fn clear_client_route_disconnect_fence_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<bool, RealtimeClusterError> {
        self.clear_client_route_disconnect_fence_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )
    }

    pub fn clear_client_route_disconnect_fence_for_current_session(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        current_session_id: Option<&str>,
    ) -> Result<bool, RealtimeClusterError> {
        self.clear_client_route_disconnect_fence_for_current_session_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            current_session_id,
        )
    }

    fn clear_client_route_disconnect_fence_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<bool, RealtimeClusterError> {
        let removed_fence = self
            .disconnect_fences
            .lock_cluster_disconnect_fence_cache()
            .remove(
                client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id).as_str(),
            )
            .map(|fence| fence.to_record());
        let persisted_removed = if let Some(expected) = removed_fence.as_ref() {
            self.disconnect_fence_store
                .clear_fence_if_matches(expected)
                .map_err(|error| {
                    self.disconnect_fence_store_error("clear disconnect fence", "storage", error)
                })?
        } else {
            self.disconnect_fence_store
                .clear_fence(tenant_id, organization_id, principal_kind, principal_id, device_id)
                .map_err(|error| {
                    self.disconnect_fence_store_error("clear disconnect fence", "storage", error)
                })?
        };
        Ok(removed_fence.is_some() || persisted_removed)
    }

    fn clear_client_route_disconnect_fence_for_current_session_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        current_session_id: Option<&str>,
    ) -> Result<bool, RealtimeClusterError> {
        let Some(fence) =
            self.load_disconnect_fence(
                tenant_id,
                organization_id,
                principal_id,
                principal_kind,
                device_id,
            )?
        else {
            return Ok(false);
        };
        if fence.session_id.as_deref() == current_session_id {
            return Ok(false);
        }

        let scope_key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        let expected = fence.to_record();
        let persisted_removed = self
            .disconnect_fence_store
            .clear_fence_if_matches(&expected)
            .map_err(|error| {
                self.disconnect_fence_store_error("clear disconnect fence", "storage", error)
            })?;
        let cache_removed = self
            .disconnect_fences
            .lock_cluster_disconnect_fence_cache()
            .get(scope_key.as_str())
            .map(|cached| cached.to_record() == expected)
            .unwrap_or(false);
        if cache_removed {
            self.disconnect_fences
                .lock_cluster_disconnect_fence_cache()
                .remove(scope_key.as_str());
        }
        Ok(persisted_removed || cache_removed)
    }

    pub fn ensure_client_route_resume_not_required_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<(), RealtimeClusterError> {
        self.ensure_client_route_resume_not_required_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )
    }

    fn ensure_client_route_resume_not_required_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<(), RealtimeClusterError> {
        let Some(fence) =
            self.load_disconnect_fence(
                tenant_id,
                organization_id,
                principal_id,
                principal_kind,
                device_id,
            )?
        else {
            return Ok(());
        };
        Err(self.node_error(
            "reconnect_required",
            fence.owner_node_id.as_str(),
            format!(
                "device must resume a fresh session before continuing after disconnect on node {}",
                fence.owner_node_id
            ),
        ))
    }

    pub fn disconnect_fence_matches_client_route_session_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        session_id: Option<&str>,
    ) -> Result<bool, RealtimeClusterError> {
        self.disconnect_fence_matches_client_route_session_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            session_id,
        )
    }

    fn disconnect_fence_matches_client_route_session_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        session_id: Option<&str>,
    ) -> Result<bool, RealtimeClusterError> {
        Ok(self
            .load_disconnect_fence(
                tenant_id,
                organization_id,
                principal_id,
                principal_kind,
                device_id,
            )?
            .as_ref()
            .map(|fence| fence.session_id.as_deref() == session_id)
            .unwrap_or(false))
    }

    fn load_disconnect_fence(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeDisconnectFence>, RealtimeClusterError> {
        let scope_key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        if let Some(fence) = self
            .disconnect_fences
            .lock_cluster_disconnect_fence_cache()
            .get(scope_key.as_str())
            .cloned()
        {
            return Ok(Some(fence));
        }

        let restored = self
            .disconnect_fence_store
            .load_fence(tenant_id, organization_id, principal_kind, principal_id, device_id)
            .map_err(|error| {
                self.disconnect_fence_store_error("load disconnect fence", "storage", error)
            })?
            .map(RealtimeDisconnectFence::from_record);
        if let Some(fence) = restored.as_ref() {
            self.disconnect_fences
                .lock_cluster_disconnect_fence_cache()
                .insert(scope_key, fence.clone());
        }
        Ok(restored)
    }

    fn disconnect_fence_store_error(
        &self,
        action: &str,
        node_id: &str,
        error: ContractError,
    ) -> RealtimeClusterError {
        self.node_error(
            "disconnect_fence_store_unavailable",
            node_id,
            format!("{action} failed: {error:?}"),
        )
    }
}

impl RealtimeDisconnectFence {
    fn to_record(&self) -> RealtimeDisconnectFenceRecord {
        RealtimeDisconnectFenceRecord {
            tenant_id: self.tenant_id.clone(),
            organization_id: self.organization_id.clone(),
            principal_kind: self.principal_kind.clone(),
            principal_id: self.principal_id.clone(),
            device_id: self.device_id.clone(),
            session_id: self.session_id.clone(),
            owner_node_id: self.owner_node_id.clone(),
            disconnected_at: self.disconnected_at.clone(),
            fence_token: self.fence_token.clone(),
        }
    }

    fn from_record(record: RealtimeDisconnectFenceRecord) -> Self {
        Self {
            tenant_id: record.tenant_id,
            organization_id: record.organization_id,
            principal_kind: record.principal_kind,
            principal_id: record.principal_id,
            device_id: record.device_id,
            session_id: record.session_id,
            owner_node_id: record.owner_node_id,
            disconnected_at: record.disconnected_at,
            fence_token: record.fence_token,
        }
    }
}

fn disconnect_fence_token(
    tenant_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
    session_id: Option<&str>,
    owner_node_id: &str,
    disconnected_at: &str,
) -> String {
    let (session_route_state, session_value) = match session_id {
        Some(session_id) => ("some-session", session_id),
        None => ("no-session", ""),
    };
    encode_disconnect_fence_token_segments([
        "fence",
        tenant_id,
        principal_kind,
        principal_id,
        device_id,
        session_route_state,
        session_value,
        owner_node_id,
        disconnected_at,
    ])
}

fn encode_disconnect_fence_token_segments<'a>(
    segments: impl IntoIterator<Item = &'a str>,
) -> String {
    let mut encoded = String::new();
    for segment in segments {
        encoded.push_str(segment.len().to_string().as_str());
        encoded.push('#');
        encoded.push_str(segment);
    }
    encoded
}

trait ClusterDisconnectMutexExt<T> {
    fn lock_cluster_disconnect_fences(&self) -> std::sync::MutexGuard<'_, T>;
}

impl<T> ClusterDisconnectMutexExt<T> for Mutex<T> {
    fn lock_cluster_disconnect_fences(&self) -> std::sync::MutexGuard<'_, T> {
        super::lock_cluster_mutex(self, "disconnect_fence_store")
    }
}

trait ClusterDisconnectCacheMutexExt<T> {
    fn lock_cluster_disconnect_fence_cache(&self) -> std::sync::MutexGuard<'_, T>;
}

impl<T> ClusterDisconnectCacheMutexExt<T> for Mutex<T> {
    fn lock_cluster_disconnect_fence_cache(&self) -> std::sync::MutexGuard<'_, T> {
        super::lock_cluster_mutex(self, "disconnect_fence_cache")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{self, AssertUnwindSafe};

    fn poison_mutex<T>(mutex: &Mutex<T>) {
        let _ = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = mutex.lock().expect("test poison lock should succeed");
            panic!("intentional poison for regression coverage");
        }));
    }

    #[test]
    fn test_disconnect_fence_store_load_recovers_from_poisoned_lock() {
        let store = ClusterMemoryDisconnectFenceStore::default();
        poison_mutex(&store.fences);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            store.load_fence("100001", "default", "user", "1", "d_demo")
        }));
        assert!(
            result.is_ok(),
            "disconnect fence store load should not panic when lock is poisoned"
        );
        let load_result = result.expect("panic status should be captured");
        assert!(load_result.is_ok());
    }

    #[test]
    fn test_mark_client_route_disconnected_recovers_from_poisoned_disconnect_cache_lock() {
        let cluster = RealtimeClusterBridge::default();
        cluster.bind_node_runtime(
            "node_a",
            std::sync::Arc::new(crate::RealtimeDeliveryRuntime::default()),
        );
        poison_mutex(&cluster.disconnect_fences);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            cluster.mark_client_route_disconnected_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_demo",
                Some("s_demo"),
                "node_a",
            )
        }));
        assert!(
            result.is_ok(),
            "mark_client_route_disconnected should not panic when disconnect cache lock is poisoned"
        );
        let mark_result = result.expect("panic status should be captured");
        assert!(mark_result.is_ok());
    }
}
