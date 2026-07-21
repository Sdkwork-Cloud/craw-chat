use im_time::{rfc3339_add_secs, rfc3339_gt, rfc3339_le, utc_now_rfc3339_millis};
use sdkwork_im_contract_control::{RealtimeDisconnectFenceRecord, RealtimeDisconnectFenceStore};
use sdkwork_im_contract_core::ContractError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::session_fence::{SessionFenceDecision, decide_session_fence};

use super::{
    RealtimeClusterBridge, RealtimeClusterError, client_route_scope_key, cluster_timestamp,
};

/// Disconnect fences older than this TTL (seconds) are treated as stale and
/// ignored by `ensure_client_route_resume_not_required`, and periodically
/// purged by the maintenance job. Keeps storage bounded for long-offline
/// devices without requiring an explicit clear.
const DISCONNECT_FENCE_TTL_SECS: i64 = 86_400; // 24h

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

#[derive(Clone, Copy, Debug)]
pub struct ClientRouteDisconnectCommand<'a> {
    pub tenant_id: &'a str,
    pub organization_id: &'a str,
    pub principal_id: &'a str,
    pub principal_kind: &'a str,
    pub device_id: &'a str,
    pub session_id: Option<&'a str>,
    pub owner_node_id: &'a str,
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
                client_route_scope_key(
                    tenant_id,
                    organization_id,
                    principal_id,
                    principal_kind,
                    device_id,
                )
                .as_str(),
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
        let key = client_route_scope_key(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        );
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
    pub fn fence_and_release_node_routes_batch(
        &self,
        node_id: &str,
        batch_size: usize,
    ) -> Result<usize, RealtimeClusterError> {
        let routes = self.routes_for_node_page(node_id, None, batch_size).items;
        let mut released = 0usize;
        for route in routes {
            self.mark_client_route_disconnected_for_principal_kind(ClientRouteDisconnectCommand {
                tenant_id: route.tenant_id.as_str(),
                organization_id: route.organization_id.as_str(),
                principal_id: route.principal_id.as_str(),
                principal_kind: route.principal_kind.as_str(),
                device_id: route.device_id.as_str(),
                session_id: route.session_id.as_deref(),
                owner_node_id: node_id,
            })?;
            if self
                .release_client_route_for_principal_kind(
                    route.tenant_id.as_str(),
                    route.organization_id.as_str(),
                    route.principal_id.as_str(),
                    route.principal_kind.as_str(),
                    route.device_id.as_str(),
                    node_id,
                )
                .is_some()
            {
                released = released.saturating_add(1);
            }
        }
        Ok(released)
    }

    pub fn fence_and_release_node_routes(
        &self,
        node_id: &str,
    ) -> Result<usize, RealtimeClusterError> {
        let mut released = 0usize;
        while self.has_routes_for_node(node_id) {
            let batch_released = self.fence_and_release_node_routes_batch(
                node_id,
                sdkwork_im_runtime_route::ROUTE_BINDING_PAGE_MAX_SIZE,
            )?;
            if batch_released == 0 {
                break;
            }
            released = released.saturating_add(batch_released);
        }
        Ok(released)
    }

    pub fn mark_client_route_disconnected_for_principal_kind(
        &self,
        command: ClientRouteDisconnectCommand<'_>,
    ) -> Result<(), RealtimeClusterError> {
        self.mark_client_route_disconnected_internal(command)
    }

    fn mark_client_route_disconnected_internal(
        &self,
        command: ClientRouteDisconnectCommand<'_>,
    ) -> Result<(), RealtimeClusterError> {
        let scope_key = client_route_scope_key(
            command.tenant_id,
            command.organization_id,
            command.principal_id,
            command.principal_kind,
            command.device_id,
        );
        let disconnected_at = cluster_timestamp();
        let fence = RealtimeDisconnectFence {
            tenant_id: command.tenant_id.into(),
            organization_id: command.organization_id.into(),
            principal_id: command.principal_id.into(),
            principal_kind: command.principal_kind.into(),
            device_id: command.device_id.into(),
            session_id: command.session_id.map(str::to_owned),
            owner_node_id: command.owner_node_id.into(),
            fence_token: disconnect_fence_token(
                command.tenant_id,
                command.principal_id,
                command.principal_kind,
                command.device_id,
                command.session_id,
                command.owner_node_id,
                disconnected_at.as_str(),
            ),
            disconnected_at,
        };
        self.disconnect_fence_store
            .save_fence(fence.to_record())
            .map_err(|error| {
                self.disconnect_fence_store_error(
                    "persist disconnect fence",
                    command.owner_node_id,
                    error,
                )
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
                client_route_scope_key(
                    tenant_id,
                    organization_id,
                    principal_id,
                    principal_kind,
                    device_id,
                )
                .as_str(),
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
                .clear_fence(
                    tenant_id,
                    organization_id,
                    principal_kind,
                    principal_id,
                    device_id,
                )
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
        let Some(fence) = self.load_disconnect_fence(
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

        let scope_key = client_route_scope_key(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        );
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
            None,
        )
    }

    /// Same as `ensure_client_route_resume_not_required_for_principal_kind`
    /// but carries the current request session id. A different authenticated
    /// session clears the stale fence; the fenced session must explicitly resume.
    pub fn ensure_client_route_resume_not_required_with_session_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        current_session_id: Option<&str>,
    ) -> Result<(), RealtimeClusterError> {
        self.ensure_client_route_resume_not_required_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            current_session_id,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn ensure_client_route_resume_not_required_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        current_session_id: Option<&str>,
    ) -> Result<(), RealtimeClusterError> {
        let fence = self.load_disconnect_fence(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )?;
        match decide_session_fence(
            fence.is_some(),
            fence.as_ref().and_then(|item| item.session_id.as_deref()),
            current_session_id,
        ) {
            SessionFenceDecision::Allow => Ok(()),
            SessionFenceDecision::ClearAndAllow => {
                self.clear_client_route_disconnect_fence_for_current_session_internal(
                    tenant_id,
                    organization_id,
                    principal_id,
                    principal_kind,
                    device_id,
                    current_session_id,
                )?;
                Ok(())
            }
            SessionFenceDecision::RequireReconnect => {
                let owner_node_id = fence
                    .as_ref()
                    .map(|item| item.owner_node_id.as_str())
                    .unwrap_or("unknown");
                Err(self.node_error(
                    "reconnect_required",
                    owner_node_id,
                    format!("device must resume a fresh session before continuing: {device_id}"),
                ))
            }
        }
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
        let scope_key = client_route_scope_key(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        );
        if let Some(fence) = self
            .disconnect_fences
            .lock_cluster_disconnect_fence_cache()
            .get(scope_key.as_str())
            .cloned()
        {
            if is_disconnect_fence_expired(&fence) {
                self.disconnect_fences
                    .lock_cluster_disconnect_fence_cache()
                    .remove(scope_key.as_str());
                return Ok(None);
            }
            return Ok(Some(fence));
        }

        let restored = self
            .disconnect_fence_store
            .load_fence(
                tenant_id,
                organization_id,
                principal_kind,
                principal_id,
                device_id,
            )
            .map_err(|error| {
                self.disconnect_fence_store_error("load disconnect fence", "storage", error)
            })?
            .map(RealtimeDisconnectFence::from_record);
        if let Some(fence) = restored.as_ref() {
            if is_disconnect_fence_expired(fence) {
                // Stale fence: best-effort clear from the backing store so it
                // does not accumulate, then return None.
                let _ = self.clear_client_route_disconnect_fence_internal(
                    tenant_id,
                    organization_id,
                    principal_id,
                    principal_kind,
                    device_id,
                );
                return Ok(None);
            }
            self.disconnect_fences
                .lock_cluster_disconnect_fence_cache()
                .insert(scope_key, fence.clone());
        }
        Ok(restored.filter(|fence| !is_disconnect_fence_expired(fence)))
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

    /// Periodic maintenance: purge expired disconnect fences from the backing
    /// store and the in-memory cache. Called by the maintenance job so storage
    /// stays bounded for long-offline devices.
    pub fn purge_expired_disconnect_fences(&self) {
        let cutoff = match rfc3339_add_secs(&utc_now_rfc3339_millis(), -DISCONNECT_FENCE_TTL_SECS) {
            Some(cutoff) => cutoff,
            None => return,
        };

        // Purge from the backing store via the time-bounded clear API.
        let mut fences = self.disconnect_fences.lock_cluster_disconnect_fence_cache();
        let expired_keys: Vec<String> = fences
            .iter()
            .filter(|(_, fence)| rfc3339_le(fence.disconnected_at.as_str(), cutoff.as_str()))
            .map(|(key, _)| key.clone())
            .collect();
        for key in &expired_keys {
            fences.remove(key.as_str());
        }
        drop(fences);

        // Best-effort store-level purge per known scope key. The store's own
        // `clear_fence_disconnected_at_or_before` is the canonical purge path
        // and is invoked from the maintenance job directly when a store handle
        // is available; this in-memory sweep covers the cache.
        for key in expired_keys {
            let parts: Vec<&str> = key.split(':').collect();
            if parts.len() != 5 {
                continue;
            }
            let _ = self
                .disconnect_fence_store
                .clear_fence_disconnected_at_or_before(
                    parts[0],
                    parts[1],
                    parts[2],
                    parts[3],
                    parts[4],
                    cutoff.as_str(),
                );
        }
    }
}

/// Returns true when the fence has exceeded the TTL and should be ignored /
/// purged.
fn is_disconnect_fence_expired(fence: &RealtimeDisconnectFence) -> bool {
    let Some(cutoff) = rfc3339_add_secs(&utc_now_rfc3339_millis(), -DISCONNECT_FENCE_TTL_SECS)
    else {
        return false;
    };
    rfc3339_gt(&cutoff, fence.disconnected_at.as_str())
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
                ClientRouteDisconnectCommand {
                    tenant_id: "100001",
                    organization_id: "default",
                    principal_id: "1",
                    principal_kind: "user",
                    device_id: "d_demo",
                    session_id: Some("s_demo"),
                    owner_node_id: "node_a",
                },
            )
        }));
        assert!(
            result.is_ok(),
            "mark_client_route_disconnected should not panic when disconnect cache lock is poisoned"
        );
        let mark_result = result.expect("panic status should be captured");
        assert!(mark_result.is_ok());
    }

    #[test]
    fn expired_persisted_disconnect_fence_does_not_block_reconnect() {
        let cluster = RealtimeClusterBridge::default();
        cluster
            .disconnect_fence_store
            .save_fence(RealtimeDisconnectFenceRecord {
                tenant_id: "100001".into(),
                organization_id: "default".into(),
                principal_kind: "user".into(),
                principal_id: "1".into(),
                device_id: "d_pad".into(),
                session_id: Some("s_old".into()),
                owner_node_id: "node_a".into(),
                disconnected_at: "2000-01-01T00:00:00.000Z".into(),
                fence_token: "expired-fence".into(),
            })
            .expect("expired fence fixture should persist");

        for _ in 0..2 {
            cluster
                .ensure_client_route_resume_not_required_with_session_for_principal_kind(
                    "100001",
                    "default",
                    "1",
                    "user",
                    "d_pad",
                    Some("s_old"),
                )
                .expect("expired fence must not block reconnect");
        }
    }
}
