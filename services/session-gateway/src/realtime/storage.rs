use im_platform_contracts::{
    RealtimeEventWindowDiagnosticsSnapshot, RealtimeEventWindowRecord, RealtimeEventWindowStore,
};
use im_time::rfc3339_le;
use sdkwork_im_contract_control::{
    RealtimeCheckpointRecord, RealtimeCheckpointStore, RealtimeMatchingSubscriptionQuery,
    RealtimeSubscriptionRecord, RealtimeSubscriptionStore,
};
use sdkwork_im_contract_core::ContractError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::{
    RealtimeDeliveryRuntime, RealtimeRuntimeError, client_route_scope_key, lock_realtime_mutex,
    normalize_checkpoint_fields, realtime_checkpoint_timestamp, subscription_record_from_items,
};

#[derive(Clone, Default)]
pub(super) struct RuntimeMemoryCheckpointStore {
    checkpoints: Arc<Mutex<HashMap<String, RealtimeCheckpointRecord>>>,
}

impl RealtimeCheckpointStore for RuntimeMemoryCheckpointStore {
    fn load_checkpoint(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        Ok(
            lock_realtime_mutex(&self.checkpoints, "runtime checkpoint store")
                .get(
                    client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id)
                        .as_str(),
                )
                .cloned(),
        )
    }

    fn save_checkpoints(
        &self,
        records: Vec<RealtimeCheckpointRecord>,
    ) -> Result<(), ContractError> {
        let mut checkpoints = lock_realtime_mutex(&self.checkpoints, "runtime checkpoint store");
        for record in records {
            let key = client_route_scope_key(
                record.tenant_id.as_str(),
                record.organization_id.as_str(),
                record.principal_id.as_str(),
                record.principal_kind.as_str(),
                record.device_id.as_str(),
            );
            let next = checkpoints
                .remove(key.as_str())
                .map(|previous| previous.merge_monotonic(record.clone()))
                .unwrap_or_else(|| record.normalized());
            checkpoints.insert(key, next);
        }
        Ok(())
    }
}

#[derive(Clone, Default)]
pub(super) struct RuntimeMemorySubscriptionStore {
    subscriptions: Arc<Mutex<HashMap<String, RealtimeSubscriptionRecord>>>,
}

#[derive(Clone, Default)]
pub(super) struct RuntimeMemoryEventWindowStore {
    windows: Arc<Mutex<HashMap<String, RealtimeEventWindowRecord>>>,
}

impl RealtimeSubscriptionStore for RuntimeMemorySubscriptionStore {
    fn load_subscriptions(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeSubscriptionRecord>, ContractError> {
        Ok(
            lock_realtime_mutex(&self.subscriptions, "runtime subscription store")
                .get(
                    client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id)
                        .as_str(),
                )
                .cloned(),
        )
    }

    fn load_matching_subscriptions(
        &self,
        query: RealtimeMatchingSubscriptionQuery<'_>,
    ) -> Result<Vec<RealtimeSubscriptionRecord>, ContractError> {
        let subscriptions = lock_realtime_mutex(&self.subscriptions, "runtime subscription store");
        Ok(query
            .candidate_device_ids
            .iter()
            .filter_map(|device_id| {
                subscriptions
                    .get(
                        client_route_scope_key(
                            query.tenant_id,
                            query.organization_id,
                            query.principal_id,
                            query.principal_kind,
                            device_id,
                        )
                        .as_str(),
                    )
                    .filter(|record| {
                        record.matches_scope_event(
                            query.scope_type,
                            query.scope_id,
                            query.event_type,
                        )
                    })
                    .cloned()
            })
            .collect())
    }

    fn save_subscriptions(&self, record: RealtimeSubscriptionRecord) -> Result<(), ContractError> {
        lock_realtime_mutex(&self.subscriptions, "runtime subscription store").insert(
            client_route_scope_key(
                record.tenant_id.as_str(),
                record.organization_id.as_str(),
                record.principal_id.as_str(),
                record.principal_kind.as_str(),
                record.device_id.as_str(),
            ),
            record,
        );
        Ok(())
    }

    fn clear_subscriptions(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        Ok(
            lock_realtime_mutex(&self.subscriptions, "runtime subscription store")
                .remove(
                    client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id)
                        .as_str(),
                )
                .is_some(),
        )
    }

    fn clear_subscriptions_synced_at_or_before(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        cutoff_synced_at: &str,
    ) -> Result<bool, ContractError> {
        let key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        let mut subscriptions =
            lock_realtime_mutex(&self.subscriptions, "runtime subscription store");
        let should_clear = subscriptions
            .get(key.as_str())
            .map(|record| rfc3339_le(record.synced_at.as_str(), cutoff_synced_at))
            .unwrap_or(false);
        if !should_clear {
            return Ok(false);
        }
        Ok(subscriptions.remove(key.as_str()).is_some())
    }
}

impl RealtimeEventWindowStore for RuntimeMemoryEventWindowStore {
    fn load_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeEventWindowRecord>, ContractError> {
        Ok(
            lock_realtime_mutex(&self.windows, "runtime event window store")
                .get(
                    client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id)
                        .as_str(),
                )
                .cloned()
                .map(RealtimeEventWindowRecord::normalized),
        )
    }

    fn save_windows(&self, records: Vec<RealtimeEventWindowRecord>) -> Result<(), ContractError> {
        let mut windows = lock_realtime_mutex(&self.windows, "runtime event window store");
        for record in records {
            windows.insert(
                client_route_scope_key(
                    record.tenant_id.as_str(),
                    record.organization_id.as_str(),
                    record.principal_id.as_str(),
                    record.principal_kind.as_str(),
                    record.device_id.as_str(),
                ),
                record.normalized(),
            );
        }
        Ok(())
    }

    fn clear_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        Ok(
            lock_realtime_mutex(&self.windows, "runtime event window store")
                .remove(
                    client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id)
                        .as_str(),
                )
                .is_some(),
        )
    }

    fn diagnostics_snapshot(
        &self,
    ) -> Result<RealtimeEventWindowDiagnosticsSnapshot, ContractError> {
        let windows = lock_realtime_mutex(&self.windows, "runtime event window store");
        Ok(RealtimeEventWindowDiagnosticsSnapshot::from_records(
            windows.values().cloned(),
        ))
    }

    fn trim_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        acked_through_seq: u64,
    ) -> Result<(), ContractError> {
        let key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        if let Some(record) =
            lock_realtime_mutex(&self.windows, "runtime event window store").get_mut(key.as_str())
        {
            record.trimmed_through_seq = record.trimmed_through_seq.max(acked_through_seq);
            record
                .events
                .retain(|event| event.realtime_seq > record.trimmed_through_seq);
            record.updated_at = realtime_checkpoint_timestamp();
        }
        Ok(())
    }
}

impl RealtimeDeliveryRuntime {
    #[cfg(test)]
    #[allow(dead_code)]
    pub(super) fn persist_checkpoint_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        self.persist_checkpoint_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )
    }

    #[cfg(test)]
    pub(super) fn persist_checkpoint_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        self.checkpoint_store
            .save_checkpoint(self.checkpoint_record(
                tenant_id,
                organization_id,
                principal_id,
                principal_kind,
                device_id,
            ))
            .map_err(RealtimeRuntimeError::checkpoint_store)
    }

    pub(super) fn persist_checkpoint_records(
        &self,
        records: Vec<RealtimeCheckpointRecord>,
    ) -> Result<(), RealtimeRuntimeError> {
        self.checkpoint_store
            .save_checkpoints(records)
            .map_err(RealtimeRuntimeError::checkpoint_store)
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(super) fn persist_subscriptions_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        self.persist_subscriptions_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )
    }

    pub(super) fn persist_subscriptions_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        match self.subscription_record(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        ) {
            Some(record) => self
                .subscription_store
                .save_subscriptions(record)
                .map_err(RealtimeRuntimeError::subscription_store),
            None => self
                .subscription_store
                .clear_subscriptions(
                    tenant_id,
                    organization_id,
                    principal_kind,
                    principal_id,
                    device_id,
                )
                .map(|_| ())
                .map_err(RealtimeRuntimeError::subscription_store),
        }
    }

    pub(super) fn clear_persisted_subscriptions_synced_at_or_before(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        cutoff_synced_at: &str,
    ) -> Result<bool, RealtimeRuntimeError> {
        self.subscription_store
            .clear_subscriptions_synced_at_or_before(
                tenant_id,
                organization_id,
                principal_kind,
                principal_id,
                device_id,
                cutoff_synced_at,
            )
            .map_err(RealtimeRuntimeError::subscription_store)
    }

    #[cfg(test)]
    fn checkpoint_record(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> RealtimeCheckpointRecord {
        let scope_key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        let latest_realtime_seq =
            lock_realtime_mutex(&self.latest_sequences, "realtime sequence store")
                .get(scope_key.as_str())
                .copied()
                .unwrap_or(0);
        let acked_through_seq = lock_realtime_mutex(&self.acked_sequences, "realtime ack store")
            .get(scope_key.as_str())
            .copied()
            .unwrap_or(0);
        let trimmed_through_seq =
            lock_realtime_mutex(&self.trimmed_sequences, "realtime trim store")
                .get(scope_key.as_str())
                .copied()
                .unwrap_or(0);
        let capacity_trimmed_event_count = lock_realtime_mutex(
            &self.capacity_trimmed_event_counts,
            "realtime capacity trim count store",
        )
        .get(scope_key.as_str())
        .copied()
        .unwrap_or(0);
        let capacity_trimmed_through_seq = lock_realtime_mutex(
            &self.capacity_trimmed_sequences,
            "realtime capacity trim sequence store",
        )
        .get(scope_key.as_str())
        .copied()
        .unwrap_or(0);
        let last_capacity_trimmed_at = lock_realtime_mutex(
            &self.last_capacity_trimmed_at,
            "realtime capacity trim timestamp store",
        )
        .get(scope_key.as_str())
        .cloned();
        checkpoint_record_from_sequences(RealtimeCheckpointRecordParts {
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            latest_realtime_seq,
            acked_through_seq,
            trimmed_through_seq,
            capacity_trimmed_event_count,
            capacity_trimmed_through_seq,
            last_capacity_trimmed_at,
        })
    }

    fn subscription_record(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Option<RealtimeSubscriptionRecord> {
        let scope_key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        let items = lock_realtime_mutex(&self.subscriptions, "realtime subscription store")
            .get(scope_key.as_str())
            .cloned()
            .map(|subscriptions| subscriptions.ordered_items())
            .unwrap_or_default();
        if items.is_empty() {
            return None;
        }

        subscription_record_from_items(tenant_id, organization_id, principal_id, principal_kind, device_id, items)
    }
}

pub(super) struct RealtimeCheckpointRecordParts<'a> {
    pub tenant_id: &'a str,
    pub organization_id: &'a str,
    pub principal_id: &'a str,
    pub principal_kind: &'a str,
    pub device_id: &'a str,
    pub latest_realtime_seq: u64,
    pub acked_through_seq: u64,
    pub trimmed_through_seq: u64,
    pub capacity_trimmed_event_count: u64,
    pub capacity_trimmed_through_seq: u64,
    pub last_capacity_trimmed_at: Option<String>,
}

pub(super) fn checkpoint_record_from_sequences(
    parts: RealtimeCheckpointRecordParts<'_>,
) -> RealtimeCheckpointRecord {
    let (latest_realtime_seq, acked_through_seq, trimmed_through_seq) = normalize_checkpoint_fields(
        parts.latest_realtime_seq,
        parts.acked_through_seq,
        parts.trimmed_through_seq,
    );
    RealtimeCheckpointRecord {
        tenant_id: parts.tenant_id.into(),
        organization_id: parts.organization_id.into(),
        principal_kind: parts.principal_kind.into(),
        principal_id: parts.principal_id.into(),
        device_id: parts.device_id.into(),
        latest_realtime_seq,
        acked_through_seq,
        trimmed_through_seq,
        capacity_trimmed_event_count: parts.capacity_trimmed_event_count,
        capacity_trimmed_through_seq: parts.capacity_trimmed_through_seq,
        last_capacity_trimmed_at: parts.last_capacity_trimmed_at,
        updated_at: realtime_checkpoint_timestamp(),
    }
    .normalized()
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_domain_core::realtime::RealtimeSubscription;

    #[test]
    fn test_runtime_checkpoint_store_load_recovers_from_poisoned_lock() {
        let store = RuntimeMemoryCheckpointStore::default();
        let _ = std::panic::catch_unwind({
            let checkpoints = store.checkpoints.clone();
            move || {
                let _guard = checkpoints
                    .lock()
                    .expect("runtime checkpoint store should lock");
                panic!("poison runtime checkpoint store lock");
            }
        });

        let checkpoint = store
            .load_checkpoint("100001", "default", "user", "1", "d_poison")
            .expect("poisoned lock should be recovered");
        assert!(checkpoint.is_none());
    }

    #[test]
    fn test_runtime_checkpoint_store_rejects_stale_regression_writes() {
        let store = RuntimeMemoryCheckpointStore::default();
        store
            .save_checkpoint(RealtimeCheckpointRecord {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                principal_kind: "user".into(),
                principal_id: "1".into(),
                device_id: "d_pad".into(),
                latest_realtime_seq: 9,
                acked_through_seq: 7,
                trimmed_through_seq: 6,
                capacity_trimmed_event_count: 3,
                capacity_trimmed_through_seq: 6,
                last_capacity_trimmed_at: Some("2026-05-06T00:00:02.000Z".into()),
                updated_at: "2026-05-06T00:00:02.000Z".into(),
            })
            .expect("new checkpoint save should succeed");
        store
            .save_checkpoint(RealtimeCheckpointRecord {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                principal_kind: "user".into(),
                principal_id: "1".into(),
                device_id: "d_pad".into(),
                latest_realtime_seq: 5,
                acked_through_seq: 4,
                trimmed_through_seq: 4,
                capacity_trimmed_event_count: 2,
                capacity_trimmed_through_seq: 4,
                last_capacity_trimmed_at: Some("2026-05-06T00:00:01.000Z".into()),
                updated_at: "2026-05-06T00:00:01.000Z".into(),
            })
            .expect("stale checkpoint save should not fail the caller");

        let checkpoint = store
            .load_checkpoint("100001", "default", "user", "1", "d_pad")
            .expect("checkpoint load should succeed")
            .expect("checkpoint should be present");
        assert_eq!(checkpoint.latest_realtime_seq, 9);
        assert_eq!(checkpoint.acked_through_seq, 7);
        assert_eq!(checkpoint.trimmed_through_seq, 6);
        assert_eq!(checkpoint.capacity_trimmed_event_count, 3);
        assert_eq!(checkpoint.capacity_trimmed_through_seq, 6);
        assert_eq!(
            checkpoint.last_capacity_trimmed_at.as_deref(),
            Some("2026-05-06T00:00:02.000Z")
        );
        assert_eq!(checkpoint.updated_at, "2026-05-06T00:00:02.000Z");
    }

    #[test]
    fn test_runtime_subscription_store_does_not_clear_newer_subscription() {
        let store = RuntimeMemorySubscriptionStore::default();
        store
            .save_subscriptions(RealtimeSubscriptionRecord {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                principal_kind: "user".into(),
                principal_id: "1".into(),
                device_id: "d_pad".into(),
                items: vec![RealtimeSubscription {
                    scope_type: "conversation".into(),
                    scope_id: "c_demo".into(),
                    event_types: Vec::new(),
                    subscribed_at: "2026-05-06T00:00:02.000Z".into(),
                }],
                synced_at: "2026-05-06T00:00:02.000Z".into(),
            })
            .expect("subscription save should succeed");

        let cleared = store
            .clear_subscriptions_synced_at_or_before(
                "100001",
                "default",
                "user",
                "1",
                "d_pad",
                "2026-05-06T00:00:01.000Z",
            )
            .expect("conditional clear should succeed");

        assert!(!cleared);
        assert!(
            store
                .load_subscriptions("100001", "default", "user", "1", "d_pad")
                .expect("subscription load should succeed")
                .is_some(),
            "newer subscription must not be deleted by an older disconnect cleanup"
        );
    }

    #[test]
    fn test_runtime_subscription_store_compares_synced_at_by_rfc3339_instant() {
        let store = RuntimeMemorySubscriptionStore::default();
        store
            .save_subscriptions(RealtimeSubscriptionRecord {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                principal_kind: "user".into(),
                principal_id: "1".into(),
                device_id: "d_pad".into(),
                items: vec![RealtimeSubscription {
                    scope_type: "conversation".into(),
                    scope_id: "c_demo".into(),
                    event_types: Vec::new(),
                    subscribed_at: "2026-05-06T00:00:00.100Z".into(),
                }],
                synced_at: "2026-05-06T00:00:00.100Z".into(),
            })
            .expect("subscription save should succeed");

        let cleared = store
            .clear_subscriptions_synced_at_or_before(
                "100001",
                "default",
                "user",
                "1",
                "d_pad",
                "2026-05-06T00:00:00Z",
            )
            .expect("conditional clear should succeed");

        assert!(
            !cleared,
            "a later fractional timestamp must not be cleared by an earlier whole-second cutoff"
        );
        assert!(
            store
                .load_subscriptions("100001", "default", "user", "1", "d_pad")
                .expect("subscription load should succeed")
                .is_some(),
            "subscription must remain after an earlier cutoff"
        );
    }
}
