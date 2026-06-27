use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use im_platform_contracts::{
    ContractError, PresenceStateRecord, PresenceStateStore, StreamStateRecord, StreamStateStore,
};
use im_time::{rfc3339_cmp, rfc3339_le};

use crate::shared::{
    principal_scope_key, read_json_records_or_default, scope_key, stream_scope_key,
    update_json_records,
};

#[derive(Clone, Debug)]
pub struct FileStreamStateStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

impl FileStreamStateStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<BTreeMap<String, StreamStateRecord>, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "stream state store")
    }
}

impl StreamStateStore for FileStreamStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        stream_id: &str,
    ) -> Result<Option<StreamStateRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("stream state file store lock should lock");
        Ok(self
            .read_records()?
            .remove(stream_scope_key(tenant_id, stream_id).as_str()))
    }

    fn save_state(&self, record: StreamStateRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("stream state file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "stream state store",
            |records: &mut BTreeMap<String, StreamStateRecord>| {
                let key = stream_scope_key(record.tenant_id.as_str(), record.stream_id.as_str());
                let next = records
                    .remove(key.as_str())
                    .map(|previous| previous.merge_monotonic(record.clone()))
                    .unwrap_or(record);
                records.insert(key, next);
            },
        )
    }

    fn clear_state(&self, tenant_id: &str, stream_id: &str) -> Result<bool, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("stream state file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "stream state store",
            |records: &mut BTreeMap<String, StreamStateRecord>| {
                records
                    .remove(stream_scope_key(tenant_id, stream_id).as_str())
                    .is_some()
            },
        )
    }
}

#[derive(Clone, Debug)]
pub struct FilePresenceStateStore {
    file_path: Arc<PathBuf>,
    io_lock: Arc<Mutex<()>>,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
struct PersistedPresenceStateRecords {
    by_device: BTreeMap<String, PresenceStateRecord>,
    presence_by_principal: BTreeMap<String, BTreeSet<String>>,
    online_by_seen_at: BTreeMap<String, BTreeSet<String>>,
}

impl FilePresenceStateStore {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    fn read_records(&self) -> Result<PersistedPresenceStateRecords, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "presence state store")
    }
}

impl PresenceStateStore for FilePresenceStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("presence state file store lock should lock");
        Ok(self
            .read_records()?
            .by_device
            .get(
                scope_key(
                    tenant_id,
                    organization_id,
                    principal_kind,
                    principal_id,
                    device_id,
                )
                .as_str(),
            )
            .cloned())
    }

    fn save_state(&self, record: PresenceStateRecord) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("presence state file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "presence state store",
            |records: &mut PersistedPresenceStateRecords| {
                let device_key = scope_key(
                    record.tenant_id.as_str(),
                    record.organization_id.as_str(),
                    record.principal_kind.as_str(),
                    record.principal_id.as_str(),
                    record.device_id.as_str(),
                );
                if let Some(previous) = records.by_device.get(device_key.as_str()).cloned() {
                    remove_presence_indexes(records, device_key.as_str(), &previous);
                }
                insert_presence_indexes(records, device_key.as_str(), &record);
                records.by_device.insert(device_key, record);
            },
        )
    }

    fn list_states_for_principal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Result<Vec<PresenceStateRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("presence state file store lock should lock");
        let records = self.read_records()?;
        let device_keys = records
            .presence_by_principal
            .get(
                principal_scope_key(
                    tenant_id,
                    organization_id,
                    principal_kind,
                    principal_id,
                )
                .as_str(),
            )
            .cloned()
            .unwrap_or_default();
        Ok(device_keys
            .into_iter()
            .filter_map(|device_key| records.by_device.get(device_key.as_str()).cloned())
            .collect())
    }

    fn list_online_states_seen_at_or_before(
        &self,
        cutoff_seen_at: &str,
        limit: usize,
    ) -> Result<Vec<PresenceStateRecord>, ContractError> {
        if limit == 0 {
            return Ok(Vec::new());
        }
        let _guard = self
            .io_lock
            .lock()
            .expect("presence state file store lock should lock");
        let records = self.read_records()?;
        let mut stale_entries = records
            .online_by_seen_at
            .iter()
            .filter(|(last_seen_at, _)| rfc3339_le(last_seen_at.as_str(), cutoff_seen_at))
            .collect::<Vec<_>>();
        stale_entries.sort_by(|left, right| rfc3339_cmp(left.0.as_str(), right.0.as_str()));
        Ok(stale_entries
            .into_iter()
            .flat_map(|(_, device_keys)| device_keys.iter())
            .take(limit)
            .filter_map(|device_key| records.by_device.get(device_key.as_str()).cloned())
            .collect())
    }

    fn expire_online_state_if_seen_at_or_before(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        cutoff_seen_at: &str,
        expired_at: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("presence state file store lock should lock");
        let mut expired = None;
        update_json_records(
            self.file_path.as_path(),
            "presence state store",
            |records: &mut PersistedPresenceStateRecords| {
                let key = scope_key(
                    tenant_id,
                    organization_id,
                    principal_kind,
                    principal_id,
                    device_id,
                );
                let Some(current) = records.by_device.get(key.as_str()).cloned() else {
                    return;
                };
                if !current.is_online_seen_at_or_before(cutoff_seen_at) {
                    return;
                }
                remove_presence_indexes(records, key.as_str(), &current);
                let next = current.into_expired_offline(expired_at);
                insert_presence_indexes(records, key.as_str(), &next);
                records.by_device.insert(key, next.clone());
                expired = Some(next);
            },
        )?;
        Ok(expired)
    }
}

pub fn validate_stream_state_store_file(file_path: impl AsRef<Path>) -> Result<(), ContractError> {
    let _: BTreeMap<String, StreamStateRecord> =
        read_json_records_or_default(file_path.as_ref(), "stream state store")?;
    Ok(())
}

pub fn validate_presence_state_store_file(
    file_path: impl AsRef<Path>,
) -> Result<(), ContractError> {
    let _: PersistedPresenceStateRecords =
        read_json_records_or_default(file_path.as_ref(), "presence state store")?;
    Ok(())
}

fn insert_presence_indexes(
    records: &mut PersistedPresenceStateRecords,
    device_key: &str,
    record: &PresenceStateRecord,
) {
    let principal_key = principal_scope_key(
        record.tenant_id.as_str(),
        record.organization_id.as_str(),
        record.principal_kind.as_str(),
        record.principal_id.as_str(),
    );
    records
        .presence_by_principal
        .entry(principal_key)
        .or_default()
        .insert(device_key.to_owned());
    if let Some(last_seen_at) = record.online_seen_at() {
        records
            .online_by_seen_at
            .entry(last_seen_at.to_owned())
            .or_default()
            .insert(device_key.to_owned());
    }
}

fn remove_presence_indexes(
    records: &mut PersistedPresenceStateRecords,
    device_key: &str,
    record: &PresenceStateRecord,
) {
    let principal_key = principal_scope_key(
        record.tenant_id.as_str(),
        record.organization_id.as_str(),
        record.principal_kind.as_str(),
        record.principal_id.as_str(),
    );
    if let Some(device_keys) = records
        .presence_by_principal
        .get_mut(principal_key.as_str())
    {
        device_keys.remove(device_key);
        if device_keys.is_empty() {
            records.presence_by_principal.remove(principal_key.as_str());
        }
    }
    let Some(last_seen_at) = record.online_seen_at() else {
        return;
    };
    if let Some(device_keys) = records.online_by_seen_at.get_mut(last_seen_at) {
        device_keys.remove(device_key);
        if device_keys.is_empty() {
            records.online_by_seen_at.remove(last_seen_at);
        }
    }
}
