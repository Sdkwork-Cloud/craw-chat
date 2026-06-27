use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use im_platform_contracts::{
    ContractError, RealtimeCheckpointRecord, RealtimeCheckpointStore,
    RealtimeDisconnectFenceRecord, RealtimeDisconnectFenceStore,
    RealtimeEventWindowDiagnosticsSnapshot, RealtimeEventWindowRecord, RealtimeEventWindowStore,
    RealtimeMatchingSubscriptionQuery, RealtimeSubscriptionRecord, RealtimeSubscriptionStore,
};
use im_time::rfc3339_le;

use crate::shared::{read_json_records_or_default, scope_key, update_json_records};

#[derive(Clone, Debug)]
pub struct FileRealtimeCheckpointStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileRealtimeCheckpointStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, RealtimeCheckpointRecord>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "realtime checkpoint store")
    }
}

impl RealtimeCheckpointStore for FileRealtimeCheckpointStore {
    fn load_checkpoint(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("checkpoint file store lock should lock");
        Ok(self
            .read_records()?
            .remove(scope_key(tenant_id, organization_id, principal_kind, principal_id, device_id).as_str()))
    }

    fn save_checkpoints(
        &self,
        records: Vec<RealtimeCheckpointRecord>,
    ) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("checkpoint file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "realtime checkpoint store",
            |stored_records: &mut BTreeMap<String, RealtimeCheckpointRecord>| {
                for record in records {
                    let key = scope_key(
                        record.tenant_id.as_str(),
                        record.organization_id.as_str(),
                        record.principal_kind.as_str(),
                        record.principal_id.as_str(),
                        record.device_id.as_str(),
                    );
                    let next = stored_records
                        .remove(key.as_str())
                        .map(|previous| previous.merge_monotonic(record.clone()))
                        .unwrap_or_else(|| record.normalized());
                    stored_records.insert(key, next);
                }
            },
        )
    }
}

#[derive(Clone, Debug)]
pub struct FileRealtimeEventWindowStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileRealtimeEventWindowStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, RealtimeEventWindowRecord>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "realtime event window store")
    }
}

impl RealtimeEventWindowStore for FileRealtimeEventWindowStore {
    fn load_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeEventWindowRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("realtime event window file store lock should lock");
        Ok(self
            .read_records()?
            .remove(scope_key(tenant_id, organization_id, principal_kind, principal_id, device_id).as_str())
            .map(RealtimeEventWindowRecord::normalized))
    }

    fn save_windows(&self, records: Vec<RealtimeEventWindowRecord>) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("realtime event window file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "realtime event window store",
            |stored_records: &mut BTreeMap<String, RealtimeEventWindowRecord>| {
                for record in records {
                    stored_records.insert(
                        scope_key(
                            record.tenant_id.as_str(),
                            record.organization_id.as_str(),
                            record.principal_kind.as_str(),
                            record.principal_id.as_str(),
                            record.device_id.as_str(),
                        ),
                        record.normalized(),
                    );
                }
            },
        )
    }

    fn clear_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("realtime event window file store lock should lock");
        let mut removed = false;
        update_json_records(
            self.file_path.as_path(),
            "realtime event window store",
            |records: &mut BTreeMap<String, RealtimeEventWindowRecord>| {
                removed = records
                    .remove(scope_key(tenant_id, organization_id, principal_kind, principal_id, device_id).as_str())
                    .is_some();
            },
        )?;
        Ok(removed)
    }

    fn diagnostics_snapshot(
        &self,
    ) -> Result<RealtimeEventWindowDiagnosticsSnapshot, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("realtime event window file store lock should lock");
        Ok(RealtimeEventWindowDiagnosticsSnapshot::from_records(
            self.read_records()?.into_values(),
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
        let _guard = self
            .io_lock
            .lock()
            .expect("realtime event window file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "realtime event window store",
            |records: &mut BTreeMap<String, RealtimeEventWindowRecord>| {
                let key = scope_key(tenant_id, organization_id, principal_kind, principal_id, device_id);
                if let Some(record) = records.get_mut(key.as_str()) {
                    record.trimmed_through_seq = record.trimmed_through_seq.max(acked_through_seq);
                    record
                        .events
                        .retain(|event| event.realtime_seq > record.trimmed_through_seq);
                }
            },
        )
    }
}

#[derive(Clone, Debug)]
pub struct FileRealtimeDisconnectFenceStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileRealtimeDisconnectFenceStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(
        &self,
    ) -> Result<BTreeMap<String, RealtimeDisconnectFenceRecord>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "disconnect fence store")
    }
}

impl RealtimeDisconnectFenceStore for FileRealtimeDisconnectFenceStore {
    fn load_fence(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeDisconnectFenceRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("disconnect fence file store lock should lock");
        Ok(self
            .read_records()?
            .remove(scope_key(tenant_id, organization_id, principal_kind, principal_id, device_id).as_str()))
    }

    fn save_fence(&self, record: RealtimeDisconnectFenceRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("disconnect fence file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "disconnect fence store",
            |records: &mut BTreeMap<String, RealtimeDisconnectFenceRecord>| {
                let key = scope_key(
                    record.tenant_id.as_str(),
                    record.organization_id.as_str(),
                    record.principal_kind.as_str(),
                    record.principal_id.as_str(),
                    record.device_id.as_str(),
                );
                let next = records
                    .remove(key.as_str())
                    .map(|previous| previous.merge_latest(record.clone()))
                    .unwrap_or(record);
                records.insert(key, next);
            },
        )
    }

    fn clear_fence(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("disconnect fence file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "disconnect fence store",
            |records: &mut BTreeMap<String, RealtimeDisconnectFenceRecord>| {
                records
                    .remove(scope_key(tenant_id, organization_id, principal_kind, principal_id, device_id).as_str())
                    .is_some()
            },
        )
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
        let _guard = self
            .io_lock
            .lock()
            .expect("disconnect fence file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "disconnect fence store",
            |records: &mut BTreeMap<String, RealtimeDisconnectFenceRecord>| {
                let key = scope_key(tenant_id, organization_id, principal_kind, principal_id, device_id);
                let should_clear = records
                    .get(key.as_str())
                    .map(|record| {
                        rfc3339_le(record.disconnected_at.as_str(), cutoff_disconnected_at)
                    })
                    .unwrap_or(false);
                if !should_clear {
                    return false;
                }
                records.remove(key.as_str()).is_some()
            },
        )
    }

    fn clear_fence_if_matches(
        &self,
        expected: &RealtimeDisconnectFenceRecord,
    ) -> Result<bool, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("disconnect fence file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "disconnect fence store",
            |records: &mut BTreeMap<String, RealtimeDisconnectFenceRecord>| {
                let key = scope_key(
            expected.tenant_id.as_str(),
            expected.organization_id.as_str(),
            expected.principal_kind.as_str(),
                    expected.principal_id.as_str(),
                    expected.device_id.as_str(),
                );
                let should_clear = records
                    .get(key.as_str())
                    .map(|record| record == expected)
                    .unwrap_or(false);
                if !should_clear {
                    return false;
                }
                records.remove(key.as_str()).is_some()
            },
        )
    }
}

#[derive(Clone, Debug)]
pub struct FileRealtimeSubscriptionStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileRealtimeSubscriptionStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, RealtimeSubscriptionRecord>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "realtime subscription store")
    }
}

impl RealtimeSubscriptionStore for FileRealtimeSubscriptionStore {
    fn load_subscriptions(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeSubscriptionRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("subscription file store lock should lock");
        Ok(self
            .read_records()?
            .remove(scope_key(tenant_id, organization_id, principal_kind, principal_id, device_id).as_str()))
    }

    fn load_matching_subscriptions(
        &self,
        query: RealtimeMatchingSubscriptionQuery<'_>,
    ) -> Result<Vec<RealtimeSubscriptionRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("subscription file store lock should lock");
        let records = self.read_records()?;
        Ok(query
            .candidate_device_ids
            .iter()
            .filter_map(|device_id| {
                records
                    .get(
                        scope_key(
                            query.tenant_id,
                            query.organization_id,
                            query.principal_kind,
                            query.principal_id,
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
        let _guard = self
            .io_lock
            .lock()
            .expect("subscription file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "realtime subscription store",
            |records: &mut BTreeMap<String, RealtimeSubscriptionRecord>| {
                records.insert(
                    scope_key(
                        record.tenant_id.as_str(),
                        record.organization_id.as_str(),
                        record.principal_kind.as_str(),
                        record.principal_id.as_str(),
                        record.device_id.as_str(),
                    ),
                    record,
                );
            },
        )
    }

    fn clear_subscriptions(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("subscription file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "realtime subscription store",
            |records: &mut BTreeMap<String, RealtimeSubscriptionRecord>| {
                records
                    .remove(scope_key(tenant_id, organization_id, principal_kind, principal_id, device_id).as_str())
                    .is_some()
            },
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
        let _guard = self
            .io_lock
            .lock()
            .expect("subscription file store lock should lock");
        let mut cleared = false;
        update_json_records(
            self.file_path.as_path(),
            "realtime subscription store",
            |records: &mut BTreeMap<String, RealtimeSubscriptionRecord>| {
                let key = scope_key(tenant_id, organization_id, principal_kind, principal_id, device_id);
                let should_clear = records
                    .get(key.as_str())
                    .map(|record| rfc3339_le(record.synced_at.as_str(), cutoff_synced_at))
                    .unwrap_or(false);
                if should_clear {
                    cleared = records.remove(key.as_str()).is_some();
                }
            },
        )?;
        Ok(cleared)
    }
}

pub fn validate_realtime_checkpoint_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, RealtimeCheckpointRecord> =
        read_json_records_or_default(file_path.as_ref(), "realtime checkpoint store")?;
    Ok(())
}

pub fn validate_realtime_event_window_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, RealtimeEventWindowRecord> =
        read_json_records_or_default(file_path.as_ref(), "realtime event window store")?;
    Ok(())
}

pub fn validate_realtime_disconnect_fence_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, RealtimeDisconnectFenceRecord> =
        read_json_records_or_default(file_path.as_ref(), "disconnect fence store")?;
    Ok(())
}

pub fn validate_realtime_subscription_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: BTreeMap<String, RealtimeSubscriptionRecord> =
        read_json_records_or_default(file_path.as_ref(), "realtime subscription store")?;
    Ok(())
}
