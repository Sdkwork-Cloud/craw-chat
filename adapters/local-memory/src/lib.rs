use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::{Arc, Mutex, MutexGuard};

use im_domain_core::stream::{StreamFrame, StreamSessionState};
use im_domain_events::CommitEnvelope;
use im_platform_contracts::{
    AutomationExecutionRecord, AutomationExecutionStore, CommitJournal,
    CommitJournalAggregateScope, CommitJournalReplayCursor, CommitJournalReplayPage,
    CommitPosition, ContractError, ExpireOnlinePresenceStateCommand, MetadataSnapshotRecord,
    MetadataStore, NotificationTaskListCursor, NotificationTaskRecord, NotificationTaskStore,
    PresenceStateRecord, PresenceStateStore, RealtimeCheckpointRecord, RealtimeCheckpointStore,
    RealtimeDisconnectFenceRecord, RealtimeDisconnectFenceStore,
    RealtimeEventWindowDiagnosticsSnapshot, RealtimeEventWindowRecord, RealtimeEventWindowStore,
    RealtimeMatchingSubscriptionQuery, RealtimeSubscriptionRecord, RealtimeSubscriptionStore,
    StreamAppendOutcome, StreamCreateOutcome, StreamScope, StreamSessionRecord, StreamStateStore,
    StreamTransitionOutcome, TimelineProjectionBatch, TimelineProjectionRecord,
    TimelineProjectionScope, TimelineProjectionStore, TimelineProjectionWindow,
};
use im_storage_contracts::{StorageDomainSnapshot, StorageDomainSnapshotStore};
use im_time::rfc3339_le;

fn lock_memory_mutex<'a, T>(mutex: &'a Mutex<T>, lock_name: &'static str) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("warn: recovered poisoned local-memory mutex lock={lock_name}");
            poisoned.into_inner()
        }
    }
}

#[derive(Clone)]
pub struct MemoryCommitJournal {
    partition: Arc<String>,
    events: Arc<Mutex<Vec<CommitEnvelope>>>,
}

impl Default for MemoryCommitJournal {
    fn default() -> Self {
        Self::with_partition("local-memory")
    }
}

impl MemoryCommitJournal {
    pub fn with_partition(partition: impl Into<String>) -> Self {
        Self {
            partition: Arc::new(partition.into()),
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn recorded(&self) -> Vec<CommitEnvelope> {
        lock_memory_mutex(&self.events, "journal").clone()
    }
}

impl CommitJournal for MemoryCommitJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let mut events = lock_memory_mutex(&self.events, "journal");
        events.push(envelope);
        Ok(CommitPosition::new(
            self.partition.as_str(),
            events.len() as u64,
        ))
    }

    fn append_batch(
        &self,
        envelopes: Vec<CommitEnvelope>,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        let mut events = lock_memory_mutex(&self.events, "journal");
        let start_offset = events.len() as u64 + 1;
        let batch_len = envelopes.len() as u64;
        events.extend(envelopes);
        Ok((0..batch_len)
            .map(|index| CommitPosition::new(self.partition.as_str(), start_offset + index))
            .collect())
    }

    fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        Ok(MemoryCommitJournal::recorded(self))
    }

    fn recorded_page(
        &self,
        cursor: Option<&CommitJournalReplayCursor>,
        limit: usize,
    ) -> Result<CommitJournalReplayPage, ContractError> {
        let limit = limit.max(1);
        let start_offset = cursor
            .map(|cursor| usize::try_from(cursor.commit_offset).unwrap_or(usize::MAX))
            .unwrap_or(0);
        let events = lock_memory_mutex(&self.events, "journal");
        if start_offset >= events.len() {
            return Ok(CommitJournalReplayPage::default());
        }

        let end_offset = start_offset.saturating_add(limit).min(events.len());
        let items = events[start_offset..end_offset].to_vec();
        let next_cursor = (end_offset < events.len()).then(|| CommitJournalReplayCursor {
            partition_key: self.partition.as_str().to_owned(),
            commit_offset: end_offset as u64,
        });

        Ok(CommitJournalReplayPage { items, next_cursor })
    }

    fn recorded_page_for_aggregate(
        &self,
        scope: &CommitJournalAggregateScope,
        cursor: Option<&CommitJournalReplayCursor>,
        limit: usize,
    ) -> Result<CommitJournalReplayPage, ContractError> {
        let limit = limit.max(1);
        let mut scan_offset = cursor
            .map(|cursor| usize::try_from(cursor.commit_offset).unwrap_or(usize::MAX))
            .unwrap_or(0);
        let events = lock_memory_mutex(&self.events, "journal");
        let mut items = Vec::with_capacity(limit);

        while scan_offset < events.len() && items.len() < limit {
            let event = &events[scan_offset];
            scan_offset += 1;
            if event.tenant_id == scope.tenant_id
                && (event.aggregate_id == scope.aggregate_id
                    || event.scope_id == scope.aggregate_id)
            {
                items.push(event.clone());
            }
        }

        let next_cursor = (scan_offset < events.len()).then(|| CommitJournalReplayCursor {
            partition_key: self.partition.as_str().to_owned(),
            commit_offset: scan_offset as u64,
        });

        Ok(CommitJournalReplayPage { items, next_cursor })
    }
}

#[derive(Clone, Default)]
pub struct MemoryMetadataStore {
    snapshots: Arc<Mutex<HashMap<String, String>>>,
}

impl MemoryMetadataStore {
    pub fn snapshot(&self, scope: &str, key: &str) -> Option<String> {
        lock_memory_mutex(&self.snapshots, "metadata store")
            .get(snapshot_key(scope, key).as_str())
            .cloned()
    }
}

impl MetadataStore for MemoryMetadataStore {
    fn put_snapshot(&self, scope: &str, key: &str, value: &str) -> Result<(), ContractError> {
        lock_memory_mutex(&self.snapshots, "metadata store")
            .insert(snapshot_key(scope, key), value.to_string());
        Ok(())
    }

    fn load_snapshot(&self, scope: &str, key: &str) -> Result<Option<String>, ContractError> {
        Ok(self.snapshot(scope, key))
    }

    fn put_snapshots(&self, snapshots: &[MetadataSnapshotRecord]) -> Result<(), ContractError> {
        let mut stored = lock_memory_mutex(&self.snapshots, "metadata store");
        for snapshot in snapshots {
            stored.insert(
                snapshot_key(snapshot.scope.as_str(), snapshot.key.as_str()),
                snapshot.value.clone(),
            );
        }
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct MemoryStorageDomainSnapshotStore {
    snapshots: Arc<Mutex<HashMap<String, StorageDomainSnapshot>>>,
}

impl MemoryStorageDomainSnapshotStore {
    pub fn snapshot(&self, domain: &str) -> Option<StorageDomainSnapshot> {
        lock_memory_mutex(&self.snapshots, "storage snapshot store")
            .get(domain)
            .cloned()
    }
}

impl StorageDomainSnapshotStore for MemoryStorageDomainSnapshotStore {
    fn load_snapshot(&self, domain: &str) -> Result<Option<StorageDomainSnapshot>, ContractError> {
        Ok(self.snapshot(domain))
    }

    fn save_snapshot(&self, snapshot: StorageDomainSnapshot) -> Result<(), ContractError> {
        lock_memory_mutex(&self.snapshots, "storage snapshot store")
            .insert(snapshot.catalog.domain.clone(), snapshot);
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct MemoryRealtimeCheckpointStore {
    checkpoints: Arc<Mutex<HashMap<String, RealtimeCheckpointRecord>>>,
}

impl MemoryRealtimeCheckpointStore {
    pub fn checkpoint(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Option<RealtimeCheckpointRecord> {
        lock_memory_mutex(&self.checkpoints, "realtime checkpoint store")
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
            .cloned()
    }
}

impl RealtimeCheckpointStore for MemoryRealtimeCheckpointStore {
    fn load_checkpoint(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        Ok(self.checkpoint(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        ))
    }

    fn save_checkpoints(
        &self,
        records: Vec<RealtimeCheckpointRecord>,
    ) -> Result<(), ContractError> {
        let mut checkpoints = lock_memory_mutex(&self.checkpoints, "realtime checkpoint store");
        for record in records {
            let key = client_route_scope_key(
                record.tenant_id.as_str(),
                record.organization_id.as_str(),
                record.principal_kind.as_str(),
                record.principal_id.as_str(),
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
pub struct MemoryRealtimeEventWindowStore {
    windows: Arc<Mutex<HashMap<String, RealtimeEventWindowRecord>>>,
}

impl MemoryRealtimeEventWindowStore {
    pub fn window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Option<RealtimeEventWindowRecord> {
        lock_memory_mutex(&self.windows, "realtime event window store")
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
            .cloned()
    }
}

impl RealtimeEventWindowStore for MemoryRealtimeEventWindowStore {
    fn load_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeEventWindowRecord>, ContractError> {
        Ok(self.window(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        ))
    }

    fn save_windows(&self, records: Vec<RealtimeEventWindowRecord>) -> Result<(), ContractError> {
        let mut windows = lock_memory_mutex(&self.windows, "realtime event window store");
        for record in records {
            windows.insert(
                client_route_scope_key(
                    record.tenant_id.as_str(),
                    record.organization_id.as_str(),
                    record.principal_kind.as_str(),
                    record.principal_id.as_str(),
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
            lock_memory_mutex(&self.windows, "realtime event window store")
                .remove(
                    client_route_scope_key(
                        tenant_id,
                        organization_id,
                        principal_kind,
                        principal_id,
                        device_id,
                    )
                    .as_str(),
                )
                .is_some(),
        )
    }

    fn diagnostics_snapshot(
        &self,
    ) -> Result<RealtimeEventWindowDiagnosticsSnapshot, ContractError> {
        let windows = lock_memory_mutex(&self.windows, "realtime event window store");
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
        let key = client_route_scope_key(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        );
        if let Some(record) =
            lock_memory_mutex(&self.windows, "realtime event window store").get_mut(key.as_str())
        {
            record.trimmed_through_seq = record.trimmed_through_seq.max(acked_through_seq);
            record
                .events
                .retain(|event| event.realtime_seq > record.trimmed_through_seq);
        }
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct MemoryRealtimeDisconnectFenceStore {
    fences: Arc<Mutex<HashMap<String, RealtimeDisconnectFenceRecord>>>,
}

impl MemoryRealtimeDisconnectFenceStore {
    pub fn fence(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Option<RealtimeDisconnectFenceRecord> {
        lock_memory_mutex(&self.fences, "realtime disconnect fence store")
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
            .cloned()
    }
}

impl RealtimeDisconnectFenceStore for MemoryRealtimeDisconnectFenceStore {
    fn load_fence(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeDisconnectFenceRecord>, ContractError> {
        Ok(self.fence(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        ))
    }

    fn save_fence(&self, record: RealtimeDisconnectFenceRecord) -> Result<(), ContractError> {
        let key = client_route_scope_key(
            record.tenant_id.as_str(),
            record.organization_id.as_str(),
            record.principal_kind.as_str(),
            record.principal_id.as_str(),
            record.device_id.as_str(),
        );
        let mut fences = lock_memory_mutex(&self.fences, "realtime disconnect fence store");
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
        Ok(
            lock_memory_mutex(&self.fences, "realtime disconnect fence store")
                .remove(
                    client_route_scope_key(
                        tenant_id,
                        organization_id,
                        principal_kind,
                        principal_id,
                        device_id,
                    )
                    .as_str(),
                )
                .is_some(),
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
        let key = client_route_scope_key(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        );
        let mut fences = lock_memory_mutex(&self.fences, "realtime disconnect fence store");
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
            expected.principal_kind.as_str(),
            expected.principal_id.as_str(),
            expected.device_id.as_str(),
        );
        let mut fences = lock_memory_mutex(&self.fences, "realtime disconnect fence store");
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

#[derive(Clone, Default)]
pub struct MemoryRealtimeSubscriptionStore {
    subscriptions: Arc<Mutex<HashMap<String, RealtimeSubscriptionRecord>>>,
}

impl MemoryRealtimeSubscriptionStore {
    pub fn subscriptions(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Option<RealtimeSubscriptionRecord> {
        lock_memory_mutex(&self.subscriptions, "realtime subscription store")
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
            .cloned()
    }
}

impl RealtimeSubscriptionStore for MemoryRealtimeSubscriptionStore {
    fn load_subscriptions(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeSubscriptionRecord>, ContractError> {
        Ok(self.subscriptions(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        ))
    }

    fn load_matching_subscriptions(
        &self,
        query: RealtimeMatchingSubscriptionQuery<'_>,
    ) -> Result<Vec<RealtimeSubscriptionRecord>, ContractError> {
        let subscriptions = lock_memory_mutex(&self.subscriptions, "realtime subscription store");
        Ok(query
            .candidate_device_ids
            .iter()
            .filter_map(|device_id| {
                subscriptions
                    .get(
                        client_route_scope_key(
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
        lock_memory_mutex(&self.subscriptions, "realtime subscription store").insert(
            client_route_scope_key(
                record.tenant_id.as_str(),
                record.organization_id.as_str(),
                record.principal_kind.as_str(),
                record.principal_id.as_str(),
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
            lock_memory_mutex(&self.subscriptions, "realtime subscription store")
                .remove(
                    client_route_scope_key(
                        tenant_id,
                        organization_id,
                        principal_kind,
                        principal_id,
                        device_id,
                    )
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
        let key = client_route_scope_key(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        );
        let mut subscriptions =
            lock_memory_mutex(&self.subscriptions, "realtime subscription store");
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

#[derive(Clone, Default)]
pub struct MemoryStreamStateStore {
    state: Arc<Mutex<MemoryStreamState>>,
}

#[derive(Default)]
struct MemoryStreamState {
    sessions: HashMap<String, StreamSessionRecord>,
    frames: HashMap<String, BTreeMap<u64, StreamFrame>>,
}

impl StreamStateStore for MemoryStreamStateStore {
    fn check_ready(&self) -> Result<(), ContractError> {
        drop(lock_memory_mutex(
            &self.state,
            "stream state store readiness",
        ));
        Ok(())
    }

    fn load_session(
        &self,
        scope: &StreamScope,
    ) -> Result<Option<StreamSessionRecord>, ContractError> {
        Ok(lock_memory_mutex(&self.state, "stream state store")
            .sessions
            .get(stream_scope_key(scope).as_str())
            .cloned())
    }

    fn create_session(
        &self,
        record: StreamSessionRecord,
        max_active_streams: u64,
    ) -> Result<StreamCreateOutcome, ContractError> {
        let mut state = lock_memory_mutex(&self.state, "stream state store");
        let key = stream_scope_key(&record.scope);
        if let Some(existing) = state.sessions.get(key.as_str()) {
            return Ok(StreamCreateOutcome::Existing(existing.clone()));
        }
        let active = state
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
            return Ok(StreamCreateOutcome::CapacityExceeded);
        }
        state.sessions.insert(key, record.clone());
        Ok(StreamCreateOutcome::Applied(record))
    }

    fn append_frame(
        &self,
        expected_version: u64,
        next_session: StreamSessionRecord,
        frame: StreamFrame,
    ) -> Result<StreamAppendOutcome, ContractError> {
        let mut state = lock_memory_mutex(&self.state, "stream state store");
        let key = stream_scope_key(&next_session.scope);
        let current = state
            .sessions
            .get(key.as_str())
            .cloned()
            .ok_or_else(|| ContractError::Invalid("stream session does not exist".into()))?;
        if let Some(existing) = state
            .frames
            .get(key.as_str())
            .and_then(|frames| frames.get(&frame.frame_seq))
            .cloned()
        {
            return Ok(StreamAppendOutcome::Existing {
                session: current,
                frame: existing,
            });
        }
        if current.version != expected_version {
            return Ok(StreamAppendOutcome::VersionConflict);
        }
        state
            .frames
            .entry(key.clone())
            .or_default()
            .insert(frame.frame_seq, frame.clone());
        state.sessions.insert(key, next_session.clone());
        Ok(StreamAppendOutcome::Applied {
            session: next_session,
            frame,
        })
    }

    fn transition_session(
        &self,
        expected_version: u64,
        next_session: StreamSessionRecord,
    ) -> Result<StreamTransitionOutcome, ContractError> {
        let mut state = lock_memory_mutex(&self.state, "stream state store");
        let key = stream_scope_key(&next_session.scope);
        if state
            .sessions
            .get(key.as_str())
            .map(|record| record.version)
            != Some(expected_version)
        {
            return Ok(StreamTransitionOutcome::VersionConflict);
        }
        state.sessions.insert(key, next_session.clone());
        Ok(StreamTransitionOutcome::Applied(next_session))
    }

    fn list_frames_after(
        &self,
        scope: &StreamScope,
        after_frame_seq: u64,
        page_size: usize,
    ) -> Result<Vec<StreamFrame>, ContractError> {
        let state = lock_memory_mutex(&self.state, "stream state store");
        Ok(state
            .frames
            .get(stream_scope_key(scope).as_str())
            .into_iter()
            .flat_map(|frames| frames.range((Excluded(after_frame_seq), Unbounded)))
            .take(page_size)
            .map(|(_, frame)| frame.clone())
            .collect())
    }

    fn clear_stream(&self, scope: &StreamScope) -> Result<bool, ContractError> {
        let mut state = lock_memory_mutex(&self.state, "stream state store");
        let key = stream_scope_key(scope);
        state.frames.remove(key.as_str());
        Ok(state.sessions.remove(key.as_str()).is_some())
    }
}

#[derive(Clone, Default)]
pub struct MemoryNotificationTaskStore {
    state: Arc<Mutex<MemoryNotificationTaskState>>,
}

#[derive(Default)]
struct MemoryNotificationTaskState {
    tasks: HashMap<String, NotificationTaskRecord>,
    tasks_by_recipient: HashMap<String, BTreeMap<NotificationRecipientSortKey, String>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct NotificationRecipientSortKey(std::cmp::Reverse<(String, String)>);

impl MemoryNotificationTaskStore {
    pub fn task(
        &self,
        tenant_id: &str,
        organization_id: &str,
        notification_id: &str,
    ) -> Option<NotificationTaskRecord> {
        lock_memory_mutex(&self.state, "notification task store")
            .tasks
            .get(notification_scope_key(tenant_id, organization_id, notification_id).as_str())
            .cloned()
    }
}

impl NotificationTaskStore for MemoryNotificationTaskStore {
    fn load_task(
        &self,
        tenant_id: &str,
        organization_id: &str,
        notification_id: &str,
    ) -> Result<Option<NotificationTaskRecord>, ContractError> {
        Ok(self.task(tenant_id, organization_id, notification_id))
    }

    fn save_task(&self, record: NotificationTaskRecord) -> Result<(), ContractError> {
        let notification_key = notification_scope_key(
            record.tenant_id.as_str(),
            record.organization_id.as_str(),
            record.notification_id.as_str(),
        );
        let mut state = lock_memory_mutex(&self.state, "notification task store");
        if let Some(previous) = state.tasks.get(notification_key.as_str()).cloned() {
            remove_notification_recipient_index(
                &mut state.tasks_by_recipient,
                notification_key.as_str(),
                &previous,
            );
            let merged = previous.merge_monotonic(record);
            insert_notification_recipient_index(
                &mut state.tasks_by_recipient,
                notification_key.as_str(),
                &merged,
            );
            state.tasks.insert(notification_key, merged);
            return Ok(());
        }
        insert_notification_recipient_index(
            &mut state.tasks_by_recipient,
            notification_key.as_str(),
            &record,
        );
        state.tasks.insert(notification_key, record);
        Ok(())
    }

    fn list_tasks_for_recipient_page(
        &self,
        tenant_id: &str,
        organization_id: &str,
        recipient_kind: &str,
        recipient_id: &str,
        cursor: Option<&NotificationTaskListCursor>,
        page_size: usize,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError> {
        let state = lock_memory_mutex(&self.state, "notification task store");
        let recipient_key = notification_recipient_scope_key(
            tenant_id,
            organization_id,
            recipient_kind,
            recipient_id,
        );
        let Some(index) = state.tasks_by_recipient.get(recipient_key.as_str()) else {
            return Ok(Vec::new());
        };
        let cursor_key = cursor.map(|value| {
            NotificationRecipientSortKey(std::cmp::Reverse((
                value.updated_at.clone(),
                value.notification_id.clone(),
            )))
        });
        let values: Box<dyn Iterator<Item = &String> + '_> = match cursor_key.as_ref() {
            Some(key) => Box::new(
                index
                    .range((Excluded(key), Unbounded))
                    .map(|(_, value)| value),
            ),
            None => Box::new(index.values()),
        };
        Ok(values
            .take(page_size.saturating_add(1))
            .filter_map(|task_key| state.tasks.get(task_key.as_str()).cloned())
            .collect())
    }
}

#[derive(Clone, Default)]
pub struct MemoryAutomationExecutionStore {
    executions: Arc<Mutex<HashMap<String, AutomationExecutionRecord>>>,
}

impl MemoryAutomationExecutionStore {
    pub fn execution(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Option<AutomationExecutionRecord> {
        lock_memory_mutex(&self.executions, "automation execution store")
            .get(
                execution_scope_key(
                    tenant_id,
                    organization_id,
                    principal_kind,
                    principal_id,
                    execution_id,
                )
                .as_str(),
            )
            .cloned()
    }
}

impl AutomationExecutionStore for MemoryAutomationExecutionStore {
    fn load_execution(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Result<Option<AutomationExecutionRecord>, ContractError> {
        Ok(self.execution(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            execution_id,
        ))
    }

    fn save_execution(&self, record: AutomationExecutionRecord) -> Result<(), ContractError> {
        let key = execution_scope_key(
            record.tenant_id.as_str(),
            record.organization_id.as_str(),
            record.execution.principal_kind.as_str(),
            record.principal_id.as_str(),
            record.execution_id.as_str(),
        );
        let mut executions = lock_memory_mutex(&self.executions, "automation execution store");
        let next = executions
            .remove(key.as_str())
            .map(|previous| previous.merge_monotonic(record.clone()))
            .unwrap_or(record);
        executions.insert(key, next);
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct MemoryPresenceStateStore {
    state: Arc<Mutex<MemoryPresenceState>>,
}

#[derive(Default)]
struct MemoryPresenceState {
    by_device: HashMap<String, PresenceStateRecord>,
    presence_by_principal: HashMap<String, BTreeSet<String>>,
    online_by_seen_at: BTreeSet<PresenceOnlineSeenAtKey>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PresenceOnlineSeenAtKey {
    last_seen_at: String,
    device_key: String,
}

impl MemoryPresenceStateStore {
    pub fn state(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Option<PresenceStateRecord> {
        lock_memory_mutex(&self.state, "presence state store")
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
            .cloned()
    }
}

impl PresenceStateStore for MemoryPresenceStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError> {
        Ok(self.state(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        ))
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
        let mut state = lock_memory_mutex(&self.state, "presence state store");
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
            .presence_by_principal
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
        let state = lock_memory_mutex(&self.state, "presence state store");
        let device_keys = state
            .presence_by_principal
            .get(
                principal_scope_key(tenant_id, organization_id, principal_kind, principal_id)
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
        let state = lock_memory_mutex(&self.state, "presence state store");
        Ok(state
            .online_by_seen_at
            .iter()
            .filter(|key| rfc3339_le(key.last_seen_at.as_str(), cutoff_seen_at))
            .take(limit)
            .filter_map(|key| state.by_device.get(key.device_key.as_str()).cloned())
            .collect())
    }

    fn expire_online_state_if_seen_at_or_before(
        &self,
        command: ExpireOnlinePresenceStateCommand<'_>,
    ) -> Result<Option<PresenceStateRecord>, ContractError> {
        let device_key = client_route_scope_key(
            command.tenant_id,
            command.organization_id,
            command.principal_kind,
            command.principal_id,
            command.device_id,
        );
        let mut state = lock_memory_mutex(&self.state, "presence state store");
        let Some(current) = state.by_device.get(device_key.as_str()).cloned() else {
            return Ok(None);
        };
        if !current.is_online_seen_at_or_before(command.cutoff_seen_at) {
            return Ok(None);
        }
        remove_presence_online_seen_at_index(&mut state.online_by_seen_at, &current);
        let expired = current.into_expired_offline(command.expired_at);
        insert_presence_online_seen_at_index(
            &mut state.online_by_seen_at,
            device_key.as_str(),
            &expired,
        );
        state.by_device.insert(device_key, expired.clone());
        Ok(Some(expired))
    }
}

#[derive(Clone, Default)]
pub struct MemoryTimelineProjectionStore {
    entries: Arc<Mutex<HashMap<String, BTreeMap<u64, String>>>>,
}

impl MemoryTimelineProjectionStore {
    pub fn entries(&self, scope: &TimelineProjectionScope) -> Vec<(u64, String)> {
        lock_memory_mutex(&self.entries, "timeline projection store")
            .get(timeline_projection_scope_key(scope).as_str())
            .map(|items| {
                items
                    .iter()
                    .map(|(message_seq, payload)| (*message_seq, payload.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl TimelineProjectionStore for MemoryTimelineProjectionStore {
    fn upsert_timeline_entry(
        &self,
        scope: &TimelineProjectionScope,
        message_seq: u64,
        payload: &str,
    ) -> Result<(), ContractError> {
        lock_memory_mutex(&self.entries, "timeline projection store")
            .entry(timeline_projection_scope_key(scope))
            .or_default()
            .insert(message_seq, payload.to_string());
        Ok(())
    }

    fn load_timeline(
        &self,
        scope: &TimelineProjectionScope,
    ) -> Result<Vec<(u64, String)>, ContractError> {
        Ok(self.entries(scope))
    }

    fn load_timeline_window(
        &self,
        scope: &TimelineProjectionScope,
        after_seq: u64,
        limit: usize,
    ) -> Result<TimelineProjectionWindow, ContractError> {
        let entries = lock_memory_mutex(&self.entries, "timeline projection store");
        let mut items = entries
            .get(timeline_projection_scope_key(scope).as_str())
            .map(|scope_entries| {
                scope_entries
                    .range((Excluded(after_seq), Unbounded))
                    .take(limit.saturating_add(1))
                    .map(|(message_seq, payload)| (*message_seq, payload.clone()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let has_more = items.len() > limit;
        items.truncate(limit);
        Ok(TimelineProjectionWindow { items, has_more })
    }

    fn upsert_timeline_entries(
        &self,
        scope: &TimelineProjectionScope,
        records: &[TimelineProjectionRecord],
    ) -> Result<(), ContractError> {
        let mut entries = lock_memory_mutex(&self.entries, "timeline projection store");
        let scope_entries = entries
            .entry(timeline_projection_scope_key(scope))
            .or_default();
        for record in records {
            scope_entries.insert(record.message_seq, record.payload.clone());
        }
        Ok(())
    }

    fn upsert_timeline_batches(
        &self,
        batches: &[TimelineProjectionBatch],
    ) -> Result<(), ContractError> {
        let mut entries = lock_memory_mutex(&self.entries, "timeline projection store");
        for batch in batches {
            let scope_entries = entries
                .entry(timeline_projection_scope_key(&batch.scope))
                .or_default();
            for record in &batch.records {
                scope_entries.insert(record.message_seq, record.payload.clone());
            }
        }
        Ok(())
    }
}

fn scope_key_parts(parts: &[&str]) -> String {
    parts
        .iter()
        .map(|part| format!("{}:{part}", part.len()))
        .collect::<Vec<_>>()
        .join("|")
}

fn snapshot_key(scope: &str, key: &str) -> String {
    scope_key_parts(&[scope, key])
}

fn client_route_scope_key(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
    device_id: &str,
) -> String {
    im_platform_contracts::realtime_client_route_scope_key(
        tenant_id,
        organization_id,
        principal_kind,
        principal_id,
        device_id,
    )
}

fn principal_scope_key(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
) -> String {
    im_platform_contracts::realtime_principal_scope_key(
        tenant_id,
        organization_id,
        principal_kind,
        principal_id,
    )
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

fn stream_scope_key(scope: &StreamScope) -> String {
    scope_key_parts(&[
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        scope.stream_id.as_str(),
    ])
}

fn notification_scope_key(tenant_id: &str, organization_id: &str, notification_id: &str) -> String {
    scope_key_parts(&[tenant_id, organization_id, notification_id])
}

fn notification_recipient_scope_key(
    tenant_id: &str,
    organization_id: &str,
    recipient_kind: &str,
    recipient_id: &str,
) -> String {
    scope_key_parts(&[tenant_id, organization_id, recipient_kind, recipient_id])
}

fn timeline_projection_scope_key(scope: &TimelineProjectionScope) -> String {
    scope_key_parts(&[
        scope.tenant_id(),
        scope.organization_id(),
        scope.timeline_scope(),
    ])
}

fn record_notification_recipient_scope_key(record: &NotificationTaskRecord) -> String {
    notification_recipient_scope_key(
        record.tenant_id.as_str(),
        record.organization_id.as_str(),
        record.task.recipient_kind.as_str(),
        record.task.recipient_id.as_str(),
    )
}

fn notification_recipient_sort_key(
    record: &NotificationTaskRecord,
) -> NotificationRecipientSortKey {
    NotificationRecipientSortKey(std::cmp::Reverse((
        record.updated_at.clone(),
        record.notification_id.clone(),
    )))
}

fn insert_notification_recipient_index(
    index: &mut HashMap<String, BTreeMap<NotificationRecipientSortKey, String>>,
    notification_key: &str,
    record: &NotificationTaskRecord,
) {
    let sort_key = notification_recipient_sort_key(record);
    index
        .entry(record_notification_recipient_scope_key(record))
        .or_default()
        .insert(sort_key, notification_key.to_owned());
}

fn remove_notification_recipient_index(
    index: &mut HashMap<String, BTreeMap<NotificationRecipientSortKey, String>>,
    notification_key: &str,
    record: &NotificationTaskRecord,
) {
    let recipient_key = record_notification_recipient_scope_key(record);
    let Some(task_keys) = index.get_mut(recipient_key.as_str()) else {
        return;
    };
    task_keys.retain(|_, key| key != notification_key);
    if task_keys.is_empty() {
        index.remove(recipient_key.as_str());
    }
}

fn execution_scope_key(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
    execution_id: &str,
) -> String {
    scope_key_parts(&[
        tenant_id,
        organization_id,
        principal_kind,
        principal_id,
        execution_id,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn poison_mutex<T>(mutex: Arc<Mutex<T>>) {
        let _ = std::panic::catch_unwind(move || {
            let _guard = mutex.lock().expect("test mutex should lock before poison");
            panic!("poison local-memory mutex");
        });
    }

    #[test]
    fn test_commit_journal_append_recovers_from_poisoned_lock() {
        let journal = MemoryCommitJournal::default();
        poison_mutex(journal.events.clone());

        let position = journal
            .append(CommitEnvelope::minimal(
                "evt_poison",
                "100001",
                "message.posted",
                "conversation",
                "c_demo",
                1,
            ))
            .expect("poisoned journal lock should be recovered");

        assert_eq!(position.offset, 1);
    }

    #[test]
    fn test_commit_journal_recorded_page_uses_explicit_memory_window() {
        let journal = MemoryCommitJournal::default();
        for seq in 0..3 {
            let event_id = format!("evt_page_{seq}");
            journal
                .append(CommitEnvelope::minimal(
                    event_id.as_str(),
                    "100001",
                    "message.posted",
                    "conversation",
                    "c_demo",
                    seq,
                ))
                .expect("append should succeed");
        }

        let first_page = journal
            .recorded_page(None, 2)
            .expect("first page should load");
        assert_eq!(
            first_page
                .items
                .iter()
                .map(|event| event.event_id.as_str())
                .collect::<Vec<_>>(),
            vec!["evt_page_0", "evt_page_1"]
        );
        assert_eq!(
            first_page
                .next_cursor
                .as_ref()
                .map(|cursor| cursor.commit_offset),
            Some(2)
        );

        let second_page = journal
            .recorded_page(first_page.next_cursor.as_ref(), 2)
            .expect("second page should load");
        assert_eq!(second_page.items.len(), 1);
        assert_eq!(second_page.items[0].event_id, "evt_page_2");
        assert!(second_page.next_cursor.is_none());
    }

    #[test]
    fn test_disconnect_fence_store_load_recovers_from_poisoned_lock() {
        let store = MemoryRealtimeDisconnectFenceStore::default();
        poison_mutex(store.fences.clone());

        let restored = store
            .load_fence("100001", "default", "user", "1", "d_pad")
            .expect("poisoned disconnect fence lock should be recovered");

        assert!(restored.is_none());
    }

    #[test]
    fn test_presence_state_store_load_recovers_from_poisoned_lock() {
        let store = MemoryPresenceStateStore::default();
        poison_mutex(store.state.clone());

        let restored = store
            .load_state("100001", "default", "user", "1", "d_pad")
            .expect("poisoned presence state lock should be recovered");

        assert!(restored.is_none());
    }
}
