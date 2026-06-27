use sdkwork_im_contract_control::{
    normalize_realtime_organization_id, PresenceStateRecord, PresenceStateStore,
};
use sdkwork_im_contract_core::ContractError;
use sdkwork_im_runtime_link::decide_resume;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::{Arc, Mutex, MutexGuard};

use im_app_context::AppContext;
use im_domain_core::presence::{
    PresenceClientView, PresenceResumeView, PresenceSnapshotView, PresenceStatus,
};
use im_time::{rfc3339_cmp, rfc3339_gt, rfc3339_le, utc_now_rfc3339_millis};

use crate::principal_scope::{typed_client_route_scope_key, typed_principal_scope_key};

const PRESENCE_EXPIRATION_BATCH_LIMIT: usize = 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
struct PresenceRuntimeEntry {
    view: PresenceClientView,
    resume_required: bool,
}

#[derive(Clone)]
pub struct PresenceRuntime {
    entries: Arc<Mutex<HashMap<String, HashMap<String, PresenceRuntimeEntry>>>>,
    restored_principals: Arc<Mutex<HashSet<String>>>,
    state_store: Arc<dyn PresenceStateStore>,
}

#[derive(Clone, Default)]
struct RuntimeMemoryPresenceStateStore {
    state: Arc<Mutex<RuntimeMemoryPresenceState>>,
}

#[derive(Default)]
struct RuntimeMemoryPresenceState {
    by_device: HashMap<String, PresenceStateRecord>,
    by_principal: HashMap<String, BTreeSet<String>>,
    online_by_seen_at: BTreeSet<PresenceOnlineSeenAtKey>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PresenceOnlineSeenAtKey {
    last_seen_at: String,
    device_key: String,
}

impl PresenceStateStore for RuntimeMemoryPresenceStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError> {
        Ok(lock_presence_mutex(&self.state, "presence state store")
            .by_device
            .get(
                client_route_scope_key(
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
        let device_key = client_route_scope_key(
            record.tenant_id.as_str(),
            record.organization_id.as_str(),
            record.principal_kind.as_str(),
            record.principal_id.as_str(),
            record.device_id.as_str(),
        );
        let principal_key = principal_scope_key(
            record.tenant_id.as_str(),
            record.organization_id.as_str(),
            record.principal_kind.as_str(),
            record.principal_id.as_str(),
        );
        let mut state = lock_presence_mutex(&self.state, "presence state store");
        if let Some(previous) = state.by_device.get(device_key.as_str()).cloned() {
            remove_presence_online_seen_at_index(&mut state.online_by_seen_at, &previous);
        }
        insert_presence_online_seen_at_index(
            &mut state.online_by_seen_at,
            device_key.as_str(),
            &record,
        );
        state.by_device.insert(device_key.clone(), record);
        state
            .by_principal
            .entry(principal_key)
            .or_default()
            .insert(device_key);
        Ok(())
    }

    fn list_states_for_principal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Result<Vec<PresenceStateRecord>, ContractError> {
        let state = lock_presence_mutex(&self.state, "presence state store");
        let device_keys = state
            .by_principal
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
            .filter_map(|device_key| state.by_device.get(device_key.as_str()).cloned())
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
        let state = lock_presence_mutex(&self.state, "presence state store");
        let mut stale_keys = state
            .online_by_seen_at
            .iter()
            .filter(|key| rfc3339_le(key.last_seen_at.as_str(), cutoff_seen_at))
            .collect::<Vec<_>>();
        stale_keys.sort_by(|left, right| {
            rfc3339_cmp(left.last_seen_at.as_str(), right.last_seen_at.as_str())
                .then_with(|| left.device_key.cmp(&right.device_key))
        });
        Ok(stale_keys
            .into_iter()
            .take(limit)
            .filter_map(|key| state.by_device.get(key.device_key.as_str()).cloned())
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
        let device_key = client_route_scope_key(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        );
        let mut state = lock_presence_mutex(&self.state, "presence state store");
        let Some(current) = state.by_device.get(device_key.as_str()).cloned() else {
            return Ok(None);
        };
        if !current.is_online_seen_at_or_before(cutoff_seen_at) {
            return Ok(None);
        }
        remove_presence_online_seen_at_index(&mut state.online_by_seen_at, &current);
        let expired = current.into_expired_offline(expired_at);
        insert_presence_online_seen_at_index(
            &mut state.online_by_seen_at,
            device_key.as_str(),
            &expired,
        );
        state.by_device.insert(device_key, expired.clone());
        Ok(Some(expired))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresenceRuntimeError {
    code: &'static str,
    message: String,
}

impl PresenceRuntimeError {
    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    fn reconnect_required(device_id: &str) -> Self {
        Self {
            code: "reconnect_required",
            message: format!("device must resume a fresh session before continuing: {device_id}"),
        }
    }

    fn presence_store(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                code: "presence_store_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                code: "presence_store_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                code: "presence_store_unsupported",
                message,
            },
        }
    }
}

impl PresenceRuntime {
    pub fn with_store<S>(state_store: Arc<S>) -> Self
    where
        S: PresenceStateStore + 'static,
    {
        Self {
            entries: Arc::new(Mutex::new(HashMap::new())),
            restored_principals: Arc::new(Mutex::new(HashSet::new())),
            state_store,
        }
    }

    pub fn register_client_route(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Result<(), PresenceRuntimeError> {
        self.ensure_client_route_entry(auth, device_id).map(|_| ())
    }

    pub fn ensure_client_route_resume_not_required(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Result<(), PresenceRuntimeError> {
        let entry = self.ensure_client_route_entry(auth, device_id)?;
        if entry.resume_required {
            return Err(PresenceRuntimeError::reconnect_required(device_id));
        }
        Ok(())
    }

    pub fn resume(
        &self,
        auth: &AppContext,
        device_id: String,
        last_seen_sync_seq: u64,
        latest_sync_seq: u64,
        registered_client_routes: Vec<String>,
    ) -> Result<PresenceResumeView, PresenceRuntimeError> {
        self.ensure_client_route_entry(auth, device_id.as_str())?;
        let resumed_at = session_timestamp();
        let updated_entry = {
            let scope = typed_principal_scope_key(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
            );
            let mut entries = lock_presence_mutex(&self.entries, "presence store");
            let scope_entries = entries.entry(scope).or_default();
            let entry =
                scope_entries
                    .entry(device_id.clone())
                    .or_insert_with(|| PresenceRuntimeEntry {
                        view: empty_presence_view(auth, device_id.as_str()),
                        resume_required: false,
                    });
            entry.view.session_id = auth.session_id.clone();
            entry.view.status = PresenceStatus::Online;
            entry.view.last_sync_seq = latest_sync_seq;
            entry.view.last_resume_at = Some(resumed_at.clone());
            entry.view.last_seen_at = Some(resumed_at.clone());
            entry.resume_required = false;
            entry.clone()
        };
        self.persist_entry(
            updated_entry,
            resumed_at.clone(),
            auth.organization_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
        )?;

        let presence =
            self.presence_snapshot(auth, Some(device_id.clone()), registered_client_routes)?;
        let resume = decide_resume(last_seen_sync_seq, latest_sync_seq);

        Ok(PresenceResumeView {
            tenant_id: auth.tenant_id.clone(),
            actor_id: auth.actor_id.clone(),
            actor_kind: auth.actor_kind.clone(),
            session_id: auth.session_id.clone(),
            device_id,
            resume_required: resume.resume_required,
            resume_from_sync_seq: resume.resume_from_sync_seq,
            latest_sync_seq: resume.latest_sync_seq,
            resumed_at,
            presence,
        })
    }

    pub fn presence_snapshot(
        &self,
        auth: &AppContext,
        current_device_id: Option<String>,
        registered_client_routes: Vec<String>,
    ) -> Result<PresenceSnapshotView, PresenceRuntimeError> {
        self.ensure_principal_state(auth)?;
        let scope = typed_principal_scope_key(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        );
        let stored_devices = lock_presence_mutex(&self.entries, "presence store")
            .get(scope.as_str())
            .cloned()
            .unwrap_or_default();

        let mut device_ids = BTreeSet::new();
        for device_id in registered_client_routes {
            device_ids.insert(device_id);
        }
        if let Some(device_id) = current_device_id.clone() {
            device_ids.insert(device_id);
        }
        for device_id in stored_devices.keys() {
            device_ids.insert(device_id.clone());
        }

        let mut devices = device_ids
            .into_iter()
            .map(|device_id| {
                stored_devices
                    .get(device_id.as_str())
                    .map(|entry| entry.view.clone())
                    .unwrap_or_else(|| {
                        empty_presence_view_for_scope(
                            auth.tenant_id.as_str(),
                            auth.actor_id.as_str(),
                            &device_id,
                        )
                    })
            })
            .collect::<Vec<_>>();

        devices.sort_by(|left, right| {
            let left_current = current_device_id
                .as_deref()
                .map(|value| value == left.device_id.as_str())
                .unwrap_or(false);
            let right_current = current_device_id
                .as_deref()
                .map(|value| value == right.device_id.as_str())
                .unwrap_or(false);
            right_current
                .cmp(&left_current)
                .then_with(|| left.device_id.cmp(&right.device_id))
        });

        Ok(PresenceSnapshotView {
            tenant_id: auth.tenant_id.clone(),
            principal_id: auth.actor_id.clone(),
            current_device_id,
            devices,
        })
    }

    pub fn heartbeat(
        &self,
        auth: &AppContext,
        device_id: String,
        latest_sync_seq: u64,
        registered_client_routes: Vec<String>,
    ) -> Result<PresenceSnapshotView, PresenceRuntimeError> {
        self.ensure_client_route_resume_not_required(auth, device_id.as_str())?;
        let observed_at = session_timestamp().to_owned();
        self.update_presence_entry(
            auth,
            device_id.clone(),
            latest_sync_seq,
            Some(auth.session_id.clone()),
            PresenceStatus::Online,
            observed_at,
            false,
            false,
        )?;
        self.presence_snapshot(auth, Some(device_id), registered_client_routes)
    }

    pub fn disconnect(
        &self,
        auth: &AppContext,
        device_id: String,
        registered_client_routes: Vec<String>,
    ) -> Result<PresenceSnapshotView, PresenceRuntimeError> {
        self.ensure_client_route_entry(auth, device_id.as_str())?;
        let latest_sync_seq = lock_presence_mutex(&self.entries, "presence store")
            .get(
                typed_principal_scope_key(
                    auth.tenant_id.as_str(),
                    auth.organization_id.as_str(),
                    auth.actor_id.as_str(),
                    auth.actor_kind.as_str(),
                )
                .as_str(),
            )
            .and_then(|scope_entries| scope_entries.get(device_id.as_str()))
            .map(|entry| entry.view.last_sync_seq)
            .unwrap_or_default();
        self.update_presence_entry(
            auth,
            device_id.clone(),
            latest_sync_seq,
            Some(None),
            PresenceStatus::Offline,
            session_timestamp(),
            false,
            true,
        )?;
        self.presence_snapshot(auth, Some(device_id), registered_client_routes)
    }

    pub fn expire_stale_online_devices(
        &self,
        cutoff_seen_at: &str,
        expired_at: &str,
    ) -> Result<usize, PresenceRuntimeError> {
        let stale_records = self
            .state_store
            .list_online_states_seen_at_or_before(cutoff_seen_at, PRESENCE_EXPIRATION_BATCH_LIMIT)
            .map_err(PresenceRuntimeError::presence_store)?;
        let mut expired_count = 0usize;

        for record in stale_records {
            if !matches!(record.presence.status, PresenceStatus::Online) {
                continue;
            }
            let Some(last_seen_at) = record.presence.last_seen_at.as_deref() else {
                continue;
            };
            if rfc3339_gt(last_seen_at, cutoff_seen_at) {
                continue;
            }

            let expired_record = self
                .state_store
                .expire_online_state_if_seen_at_or_before(
                    record.tenant_id.as_str(),
                    record.organization_id.as_str(),
                    record.principal_kind.as_str(),
                    record.principal_id.as_str(),
                    record.device_id.as_str(),
                    cutoff_seen_at,
                    expired_at,
                )
                .map_err(PresenceRuntimeError::presence_store)?;
            let Some(expired_record) = expired_record else {
                continue;
            };
            self.apply_expired_entry_to_runtime_cache(
                expired_record.tenant_id.as_str(),
                expired_record.organization_id.as_str(),
                expired_record.principal_kind.as_str(),
                expired_record.principal_id.as_str(),
                expired_record.device_id.as_str(),
                PresenceRuntimeEntry {
                    view: expired_record.presence.clone(),
                    resume_required: expired_record.resume_required,
                },
            );
            expired_count += 1;
        }

        Ok(expired_count)
    }

    fn ensure_principal_state(&self, auth: &AppContext) -> Result<(), PresenceRuntimeError> {
        let scope_key = typed_principal_scope_key(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        );
        if lock_presence_mutex(&self.restored_principals, "presence runtime")
            .contains(scope_key.as_str())
        {
            return Ok(());
        }

        let restored = self
            .state_store
            .list_states_for_principal(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                auth.actor_kind.as_str(),
                auth.actor_id.as_str(),
            )
            .map_err(PresenceRuntimeError::presence_store)?;
        let mut normalized_records = Vec::new();
        let mut runtime_entries = Vec::new();
        for record in restored {
            let (entry, normalized_record) = normalize_presence_record(record);
            if let Some(normalized_record) = normalized_record {
                normalized_records.push(normalized_record);
            }
            runtime_entries.push((entry.view.device_id.clone(), entry));
        }

        for normalized_record in normalized_records {
            self.state_store
                .save_state(normalized_record)
                .map_err(PresenceRuntimeError::presence_store)?;
        }

        let mut entries = lock_presence_mutex(&self.entries, "presence runtime");
        let scope_entries = entries.entry(scope_key.clone()).or_default();
        for (device_id, entry) in runtime_entries {
            scope_entries.entry(device_id).or_insert(entry);
        }
        drop(entries);
        lock_presence_mutex(&self.restored_principals, "presence runtime").insert(scope_key);

        Ok(())
    }

    fn ensure_client_route_entry(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Result<PresenceRuntimeEntry, PresenceRuntimeError> {
        self.ensure_principal_state(auth)?;

        if let Some(entry) = lock_presence_mutex(&self.entries, "presence store")
            .get(
                typed_principal_scope_key(
                    auth.tenant_id.as_str(),
                    auth.organization_id.as_str(),
                    auth.actor_id.as_str(),
                    auth.actor_kind.as_str(),
                )
                .as_str(),
            )
            .and_then(|scope_entries| scope_entries.get(device_id))
            .cloned()
        {
            return Ok(entry);
        }

        let entry = PresenceRuntimeEntry {
            view: empty_presence_view(auth, device_id),
            resume_required: false,
        };
        let scope = typed_principal_scope_key(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        );
        let mut entries = lock_presence_mutex(&self.entries, "presence store");
        entries
            .entry(scope)
            .or_default()
            .insert(device_id.to_owned(), entry.clone());
        drop(entries);
        self.persist_entry(
            entry.clone(),
            session_timestamp(),
            auth.organization_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
        )?;

        Ok(entry)
    }

    // Presence updates mirror the stored client route session tuple and explicit flags
    // so reconnect and resume decisions stay readable at each call site.
    #[allow(clippy::too_many_arguments)]
    fn update_presence_entry(
        &self,
        auth: &AppContext,
        device_id: String,
        latest_sync_seq: u64,
        session_id: Option<Option<String>>,
        status: PresenceStatus,
        observed_at: String,
        refresh_resume_at: bool,
        resume_required: bool,
    ) -> Result<(), PresenceRuntimeError> {
        self.ensure_client_route_entry(auth, device_id.as_str())?;
        let scope = typed_principal_scope_key(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        );
        let mut entries = lock_presence_mutex(&self.entries, "presence store");
        let scope_entries = entries.entry(scope).or_default();
        let entry =
            scope_entries
                .entry(device_id.clone())
                .or_insert_with(|| PresenceRuntimeEntry {
                    view: empty_presence_view(auth, device_id.as_str()),
                    resume_required: false,
                });
        if let Some(session_id) = session_id {
            entry.view.session_id = session_id;
        }
        entry.view.status = status;
        entry.view.last_sync_seq = latest_sync_seq;
        if refresh_resume_at || entry.view.last_resume_at.is_none() {
            entry.view.last_resume_at = Some(observed_at.clone());
        }
        entry.view.last_seen_at = Some(observed_at.clone());
        entry.resume_required = resume_required;
        let updated = entry.clone();
        drop(entries);
        self.persist_entry(
            updated,
            observed_at,
            auth.organization_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
        )
    }

    fn persist_entry(
        &self,
        entry: PresenceRuntimeEntry,
        updated_at: String,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Result<(), PresenceRuntimeError> {
        self.state_store
            .save_state(PresenceStateRecord {
                tenant_id: entry.view.tenant_id.clone(),
                organization_id: normalize_realtime_organization_id(organization_id),
                principal_kind: principal_kind.into(),
                principal_id: principal_id.into(),
                device_id: entry.view.device_id.clone(),
                presence: entry.view,
                resume_required: entry.resume_required,
                updated_at,
            })
            .map_err(PresenceRuntimeError::presence_store)
    }

    fn apply_expired_entry_to_runtime_cache(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        expired_entry: PresenceRuntimeEntry,
    ) {
        let scope_key = principal_scope_key(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
        );
        let restored = lock_presence_mutex(&self.restored_principals, "presence runtime")
            .contains(scope_key.as_str());
        if !restored {
            return;
        }

        let mut entries = lock_presence_mutex(&self.entries, "presence runtime");
        entries
            .entry(scope_key)
            .or_default()
            .insert(device_id.to_owned(), expired_entry);
    }
}

impl Default for PresenceRuntime {
    fn default() -> Self {
        Self::with_store(Arc::new(RuntimeMemoryPresenceStateStore::default()))
    }
}

pub(crate) fn client_route_scope_key(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
    device_id: &str,
) -> String {
    typed_client_route_scope_key(
        tenant_id,
        organization_id,
        principal_id,
        principal_kind,
        device_id,
    )
}

fn principal_scope_key(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
) -> String {
    typed_principal_scope_key(tenant_id, organization_id, principal_id, principal_kind)
}

fn presence_online_seen_at_key(
    device_key: &str,
    record: &PresenceStateRecord,
) -> Option<PresenceOnlineSeenAtKey> {
    Some(PresenceOnlineSeenAtKey {
        last_seen_at: record.online_seen_at()?.to_owned(),
        device_key: device_key.to_owned(),
    })
}

fn insert_presence_online_seen_at_index(
    index: &mut BTreeSet<PresenceOnlineSeenAtKey>,
    device_key: &str,
    record: &PresenceStateRecord,
) {
    if let Some(key) = presence_online_seen_at_key(device_key, record) {
        index.insert(key);
    }
}

fn remove_presence_online_seen_at_index(
    index: &mut BTreeSet<PresenceOnlineSeenAtKey>,
    record: &PresenceStateRecord,
) {
    let device_key = client_route_scope_key(
        record.tenant_id.as_str(),
        record.organization_id.as_str(),
        record.principal_kind.as_str(),
        record.principal_id.as_str(),
        record.device_id.as_str(),
    );
    if let Some(key) = presence_online_seen_at_key(device_key.as_str(), record) {
        index.remove(&key);
    }
}

fn empty_presence_view(auth: &AppContext, device_id: &str) -> PresenceClientView {
    empty_presence_view_for_scope(auth.tenant_id.as_str(), auth.actor_id.as_str(), device_id)
}

fn empty_presence_view_for_scope(
    tenant_id: &str,
    principal_id: &str,
    device_id: &str,
) -> PresenceClientView {
    PresenceClientView {
        tenant_id: tenant_id.into(),
        principal_id: principal_id.into(),
        device_id: device_id.into(),
        platform: None,
        session_id: None,
        status: PresenceStatus::Offline,
        last_sync_seq: 0,
        last_resume_at: None,
        last_seen_at: None,
    }
}

fn normalize_presence_record(
    record: PresenceStateRecord,
) -> (PresenceRuntimeEntry, Option<PresenceStateRecord>) {
    let mut presence = record.presence.clone();
    let mut resume_required = record.resume_required;
    let mut normalized = false;

    if matches!(presence.status, PresenceStatus::Online) {
        presence.status = PresenceStatus::Offline;
        presence.session_id = None;
        resume_required = true;
        normalized = true;
    } else if presence.session_id.is_some() {
        presence.session_id = None;
        normalized = true;
    }

    let entry = PresenceRuntimeEntry {
        view: presence.clone(),
        resume_required,
    };
    let normalized_record = if normalized || resume_required != record.resume_required {
        Some(PresenceStateRecord {
            tenant_id: record.tenant_id,
            organization_id: record.organization_id,
            principal_kind: record.principal_kind,
            principal_id: record.principal_id,
            device_id: record.device_id,
            presence,
            resume_required,
            updated_at: session_timestamp(),
        })
    } else {
        None
    };

    (entry, normalized_record)
}

fn session_timestamp() -> String {
    utc_now_rfc3339_millis()
}

fn lock_presence_mutex<'a, T>(mutex: &'a Mutex<T>, lock_name: &'static str) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovered poisoned presence mutex lock={lock_name}");
            poisoned.into_inner()
        }
    }
}

#[cfg(test)]
#[path = "presence_tests.rs"]
mod tests;
