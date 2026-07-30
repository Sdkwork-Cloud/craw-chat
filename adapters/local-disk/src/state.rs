use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use im_domain_core::stream::{StreamFrame, StreamSessionState};
use im_platform_contracts::{
    ContractError, ExpireOnlinePresenceStateCommand, PresenceStateRecord, PresenceStateStore,
    StreamAppendOutcome, StreamCreateOutcome, StreamScope, StreamSessionRecord, StreamStateStore,
    StreamTransitionOutcome,
};
use im_time::rfc3339_le;

use crate::shared::{
    principal_scope_key, read_json_records_or_default, scope_key, update_json_records,
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

    fn read_records(&self) -> Result<PersistedStreamStateRecords, ContractError> {
        read_json_records_or_default(self.file_path.as_path(), "stream state store")
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
struct PersistedStreamStateRecords {
    sessions: BTreeMap<String, StreamSessionRecord>,
    frames: BTreeMap<String, BTreeMap<u64, StreamFrame>>,
}

impl StreamStateStore for FileStreamStateStore {
    fn check_ready(&self) -> Result<(), ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("stream state file store lock should lock");
        self.read_records().map(|_| ())
    }

    fn load_session(
        &self,
        scope: &StreamScope,
    ) -> Result<Option<StreamSessionRecord>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("stream state file store lock should lock");
        Ok(self
            .read_records()?
            .sessions
            .get(stream_record_key(scope).as_str())
            .cloned())
    }

    fn create_session(
        &self,
        record: StreamSessionRecord,
        max_active_streams: u64,
    ) -> Result<StreamCreateOutcome, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("stream state file store lock should lock");
        let mut outcome = None;
        update_json_records(
            self.file_path.as_path(),
            "stream state store",
            |records: &mut PersistedStreamStateRecords| {
                let key = stream_record_key(&record.scope);
                if let Some(existing) = records.sessions.get(key.as_str()) {
                    outcome = Some(StreamCreateOutcome::Existing(existing.clone()));
                    return;
                }
                let active = records
                    .sessions
                    .values()
                    .filter(|candidate| {
                        candidate.scope.tenant_id == record.scope.tenant_id
                            && candidate.scope.organization_id == record.scope.organization_id
                            && !matches!(
                                candidate.session.state,
                                StreamSessionState::Completed
                                    | StreamSessionState::Aborted
                                    | StreamSessionState::Expired
                            )
                    })
                    .count() as u64;
                if active >= max_active_streams {
                    outcome = Some(StreamCreateOutcome::CapacityExceeded);
                    return;
                }
                records.sessions.insert(key, record.clone());
                outcome = Some(StreamCreateOutcome::Applied(record.clone()));
            },
        )?;
        outcome.ok_or_else(|| ContractError::Unavailable("stream create outcome missing".into()))
    }

    fn append_frame(
        &self,
        expected_version: u64,
        next_session: StreamSessionRecord,
        frame: StreamFrame,
    ) -> Result<StreamAppendOutcome, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("stream state file store lock should lock");
        let mut outcome = None;
        update_json_records(
            self.file_path.as_path(),
            "stream state store",
            |records: &mut PersistedStreamStateRecords| {
                let key = stream_record_key(&next_session.scope);
                let Some(current) = records.sessions.get(key.as_str()).cloned() else {
                    return;
                };
                if let Some(existing) = records
                    .frames
                    .get(key.as_str())
                    .and_then(|frames| frames.get(&frame.frame_seq))
                    .cloned()
                {
                    outcome = Some(StreamAppendOutcome::Existing {
                        session: current,
                        frame: existing,
                    });
                    return;
                }
                if current.version != expected_version {
                    outcome = Some(StreamAppendOutcome::VersionConflict);
                    return;
                }
                records
                    .frames
                    .entry(key.clone())
                    .or_default()
                    .insert(frame.frame_seq, frame.clone());
                records.sessions.insert(key, next_session.clone());
                outcome = Some(StreamAppendOutcome::Applied {
                    session: next_session.clone(),
                    frame: frame.clone(),
                });
            },
        )?;
        outcome.ok_or_else(|| ContractError::Invalid("stream session does not exist".into()))
    }

    fn transition_session(
        &self,
        expected_version: u64,
        next_session: StreamSessionRecord,
    ) -> Result<StreamTransitionOutcome, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("stream state file store lock should lock");
        let mut outcome = None;
        update_json_records(
            self.file_path.as_path(),
            "stream state store",
            |records: &mut PersistedStreamStateRecords| {
                let key = stream_record_key(&next_session.scope);
                if records
                    .sessions
                    .get(key.as_str())
                    .map(|record| record.version)
                    != Some(expected_version)
                {
                    outcome = Some(StreamTransitionOutcome::VersionConflict);
                    return;
                }
                records.sessions.insert(key, next_session.clone());
                outcome = Some(StreamTransitionOutcome::Applied(next_session.clone()));
            },
        )?;
        outcome
            .ok_or_else(|| ContractError::Unavailable("stream transition outcome missing".into()))
    }

    fn list_frames_after(
        &self,
        scope: &StreamScope,
        after_frame_seq: u64,
        page_size: usize,
    ) -> Result<Vec<StreamFrame>, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("stream state file store lock should lock");
        let records = self.read_records()?;
        Ok(records
            .frames
            .get(stream_record_key(scope).as_str())
            .into_iter()
            .flat_map(|frames| {
                frames.range((
                    std::ops::Bound::Excluded(after_frame_seq),
                    std::ops::Bound::Unbounded,
                ))
            })
            .take(page_size)
            .map(|(_, frame)| frame.clone())
            .collect())
    }

    fn clear_stream(&self, scope: &StreamScope) -> Result<bool, ContractError> {
        let _guard = self
            .io_lock
            .lock()
            .expect("stream state file store lock should lock");
        update_json_records(
            self.file_path.as_path(),
            "stream state store",
            |records: &mut PersistedStreamStateRecords| {
                let key = stream_record_key(scope);
                records.frames.remove(key.as_str());
                records.sessions.remove(key.as_str()).is_some()
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
                principal_scope_key(tenant_id, organization_id, principal_kind, principal_id)
                    .as_str(),
            )
            .cloned()
            .unwrap_or_default();
        Ok(device_keys
            .into_iter()
            .filter_map(|device_key| records.by_device.get(device_key.as_str()).cloned())
            .collect())
    }

    fn discover_stale_online_states(
        &self,
        request: im_platform_contracts::StalePresenceScopeDiscoveryRequest<'_>,
    ) -> Result<Vec<PresenceStateRecord>, ContractError> {
        let cutoff_seen_at = request.cutoff_seen_at();
        let limit = request.limit();
        if limit == 0 {
            return Ok(Vec::new());
        }
        let _guard = self
            .io_lock
            .lock()
            .expect("presence state file store lock should lock");
        let records = self.read_records()?;
        Ok(records
            .online_by_seen_at
            .iter()
            .filter(|(last_seen_at, _)| rfc3339_le(last_seen_at.as_str(), cutoff_seen_at))
            .flat_map(|(_, device_keys)| device_keys.iter())
            .take(limit)
            .filter_map(|device_key| records.by_device.get(device_key.as_str()).cloned())
            .collect())
    }

    fn expire_online_state_if_seen_at_or_before(
        &self,
        command: ExpireOnlinePresenceStateCommand<'_>,
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
                    command.tenant_id,
                    command.organization_id,
                    command.principal_kind,
                    command.principal_id,
                    command.device_id,
                );
                let Some(current) = records.by_device.get(key.as_str()).cloned() else {
                    return;
                };
                if !current.is_online_seen_at_or_before(command.cutoff_seen_at) {
                    return;
                }
                remove_presence_indexes(records, key.as_str(), &current);
                let next = current.into_expired_offline(command.expired_at);
                insert_presence_indexes(records, key.as_str(), &next);
                records.by_device.insert(key, next.clone());
                expired = Some(next);
            },
        )?;
        Ok(expired)
    }
}

pub fn validate_stream_state_store_file(file_path: impl AsRef<Path>) -> Result<(), ContractError> {
    let _: PersistedStreamStateRecords =
        read_json_records_or_default(file_path.as_ref(), "stream state store")?;
    Ok(())
}

fn stream_record_key(scope: &StreamScope) -> String {
    crate::shared::scope_key_parts(&[
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        scope.stream_id.as_str(),
    ])
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
