//! White-box unit tests for projection snapshots.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "snapshot_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use std::collections::{BTreeMap, HashMap};
use std::panic::{self, AssertUnwindSafe};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use im_adapters_local_memory::{MemoryMetadataStore, MemoryTimelineProjectionStore};
use im_domain_core::conversation::{MembershipRole, build_conversation_member};
use im_platform_contracts::{
    ContractError, MetadataSnapshotRecord, MetadataStore, TimelineProjectionBatch,
    TimelineProjectionRecord, TimelineProjectionStore,
};

use super::*;

#[derive(Clone, Default)]
struct CountingMetadataStore {
    snapshots: Arc<Mutex<HashMap<(String, String), String>>>,
    single_put_calls: Arc<AtomicUsize>,
    batch_put_calls: Arc<AtomicUsize>,
}

impl CountingMetadataStore {
    fn single_put_calls(&self) -> usize {
        self.single_put_calls.load(Ordering::Relaxed)
    }

    fn batch_put_calls(&self) -> usize {
        self.batch_put_calls.load(Ordering::Relaxed)
    }

    fn insert_snapshots(&self, snapshots: &[MetadataSnapshotRecord]) {
        let mut stored = self.snapshots.lock().expect("metadata store should lock");
        for snapshot in snapshots {
            stored.insert(
                (snapshot.scope.clone(), snapshot.key.clone()),
                snapshot.value.clone(),
            );
        }
    }
}

impl MetadataStore for CountingMetadataStore {
    fn put_snapshot(&self, scope: &str, key: &str, value: &str) -> Result<(), ContractError> {
        self.single_put_calls.fetch_add(1, Ordering::Relaxed);
        self.snapshots
            .lock()
            .expect("metadata store should lock")
            .insert((scope.to_owned(), key.to_owned()), value.to_owned());
        Ok(())
    }

    fn load_snapshot(&self, scope: &str, key: &str) -> Result<Option<String>, ContractError> {
        Ok(self
            .snapshots
            .lock()
            .expect("metadata store should lock")
            .get(&(scope.to_owned(), key.to_owned()))
            .cloned())
    }

    fn put_snapshots(&self, snapshots: &[MetadataSnapshotRecord]) -> Result<(), ContractError> {
        self.batch_put_calls.fetch_add(1, Ordering::Relaxed);
        self.insert_snapshots(snapshots);
        Ok(())
    }
}

type TimelineProjectionTestEntries = Arc<Mutex<HashMap<(String, String), BTreeMap<u64, String>>>>;

#[derive(Clone, Default)]
struct CountingTimelineProjectionStore {
    entries: TimelineProjectionTestEntries,
    single_upsert_calls: Arc<AtomicUsize>,
    batch_upsert_calls: Arc<AtomicUsize>,
}

impl CountingTimelineProjectionStore {
    fn single_upsert_calls(&self) -> usize {
        self.single_upsert_calls.load(Ordering::Relaxed)
    }

    fn batch_upsert_calls(&self) -> usize {
        self.batch_upsert_calls.load(Ordering::Relaxed)
    }

    fn upsert_records(
        &self,
        tenant_id: &str,
        timeline_scope: &str,
        records: &[TimelineProjectionRecord],
    ) {
        let mut stored = self.entries.lock().expect("timeline store should lock");
        let scope_entries = stored
            .entry((tenant_id.to_owned(), timeline_scope.to_owned()))
            .or_default();
        for record in records {
            scope_entries.insert(record.message_seq, record.payload.clone());
        }
    }
}

impl TimelineProjectionStore for CountingTimelineProjectionStore {
    fn upsert_timeline_entry(
        &self,
        tenant_id: &str,
        timeline_scope: &str,
        message_seq: u64,
        payload: &str,
    ) -> Result<(), ContractError> {
        self.single_upsert_calls.fetch_add(1, Ordering::Relaxed);
        self.entries
            .lock()
            .expect("timeline store should lock")
            .entry((tenant_id.to_owned(), timeline_scope.to_owned()))
            .or_default()
            .insert(message_seq, payload.to_owned());
        Ok(())
    }

    fn load_timeline(
        &self,
        tenant_id: &str,
        timeline_scope: &str,
    ) -> Result<Vec<(u64, String)>, ContractError> {
        Ok(self
            .entries
            .lock()
            .expect("timeline store should lock")
            .get(&(tenant_id.to_owned(), timeline_scope.to_owned()))
            .map(|items| {
                items
                    .iter()
                    .map(|(message_seq, payload)| (*message_seq, payload.clone()))
                    .collect()
            })
            .unwrap_or_default())
    }

    fn upsert_timeline_batches(
        &self,
        batches: &[TimelineProjectionBatch],
    ) -> Result<(), ContractError> {
        self.batch_upsert_calls.fetch_add(1, Ordering::Relaxed);
        for batch in batches {
            self.upsert_records(
                batch.tenant_id.as_str(),
                batch.timeline_scope.as_str(),
                &batch.records,
            );
        }
        Ok(())
    }
}

fn conversation_created_event(
    tenant_id: &str,
    conversation_id: &str,
    conversation_type: &str,
) -> im_domain_events::CommitEnvelope {
    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{conversation_id}_created"),
        tenant_id,
        "conversation.created",
        "conversation",
        conversation_id,
        0,
    )
    .with_payload(
        "conversation.created.v1",
        &serde_json::json!({
            "conversationType": conversation_type,
        })
        .to_string(),
    )
}

fn member_joined_event(
    tenant_id: &str,
    conversation_id: &str,
    principal_id: &str,
    role: MembershipRole,
) -> im_domain_events::CommitEnvelope {
    let member = build_conversation_member(
        tenant_id,
        conversation_id,
        format!("cm_{conversation_id}_{principal_id}"),
        principal_id,
        "user",
        role,
        Some("1".into()),
        "2026-04-08T10:00:00Z".into(),
    );

    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{conversation_id}_{principal_id}_joined"),
        tenant_id,
        "conversation.member_joined",
        "conversation",
        conversation_id,
        1,
    )
    .with_payload(
        "conversation.member_joined.v1",
        &serde_json::to_string(&member).expect("member should serialize"),
    )
}

fn message_posted_event(
    tenant_id: &str,
    conversation_id: &str,
    message_id: &str,
    message_seq: u64,
    sender_id: &str,
    summary: &str,
) -> im_domain_events::CommitEnvelope {
    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{conversation_id}_{message_seq}"),
        tenant_id,
        "message.posted",
        "conversation",
        conversation_id,
        message_seq,
    )
    .with_payload(
        "message.posted.v1",
        &serde_json::json!({
            "tenantId": tenant_id,
            "conversationId": conversation_id,
            "messageId": message_id,
            "messageSeq": message_seq,
            "sender": {
                "id": sender_id,
                "kind": "user",
                "memberId": format!("cm_{sender_id}"),
                "deviceId": format!("d_{sender_id}"),
                "sessionId": format!("s_{sender_id}"),
                "metadata": {}
            },
            "messageType": "standard",
            "deliveryMode": "discrete",
            "clientMsgId": format!("client_{message_id}"),
            "streamSessionId": null,
            "rtcSessionId": null,
            "body": {
                "summary": summary,
                "parts": [{
                    "kind": "text",
                    "text": summary
                }],
                "renderHints": {}
            },
            "attributes": {},
            "metadata": {},
            "occurredAt": "2026-04-08T10:00:01Z",
            "committedAt": "2026-04-08T10:00:01Z"
        })
        .to_string(),
    )
}

fn poison_mutex<T>(mutex: &Mutex<T>) {
    let _ = panic::catch_unwind(AssertUnwindSafe(|| {
        let _guard = mutex.lock().expect("test poison lock should succeed");
        panic!("intentional poison for regression coverage");
    }));
}

fn demo_summary(tenant_id: &str, conversation_id: &str) -> ConversationSummaryView {
    ConversationSummaryView {
        tenant_id: tenant_id.into(),
        conversation_id: conversation_id.into(),
        message_count: 0,
        last_message_id: None,
        last_message_seq: 0,
        last_sender_id: None,
        last_sender_kind: None,
        last_sender: None,
        last_summary: None,
        last_message_at: None,
        agent_handoff: None,
    }
}

#[test]
fn test_persist_conversation_snapshot_batches_projection_store_flushes() {
    let projection = TimelineProjectionService::default();
    let metadata_store = CountingMetadataStore::default();
    let timeline_store = CountingTimelineProjectionStore::default();

    projection
        .apply(&conversation_created_event("100001", "c_batch", "group"))
        .expect("conversation projection should succeed");
    projection
        .apply(&member_joined_event(
            "100001",
            "c_batch",
            "1014",
            MembershipRole::Owner,
        ))
        .expect("member projection should succeed");
    projection.register_client_route("100001", "default", "1014", "d_phone");
    projection
        .apply(&message_posted_event(
            "100001",
            "c_batch",
            "msg_batch_1",
            1,
            "1014",
            "batched snapshot",
        ))
        .expect("message projection should succeed");

    assert!(
        projection
            .persist_conversation_snapshot("100001", "default", "c_batch", &metadata_store, &timeline_store,)
            .expect("snapshot should persist"),
        "snapshot should exist"
    );

    assert_eq!(
        metadata_store.single_put_calls(),
        0,
        "conversation snapshot persistence should avoid per-record metadata flushes"
    );
    assert_eq!(
        metadata_store.batch_put_calls(),
        1,
        "conversation snapshot persistence should flush metadata once per envelope"
    );
    assert_eq!(
        timeline_store.single_upsert_calls(),
        0,
        "conversation snapshot persistence should avoid per-entry timeline flushes"
    );
    assert_eq!(
        timeline_store.batch_upsert_calls(),
        1,
        "conversation snapshot persistence should flush timeline projections once per envelope"
    );
    assert_eq!(
        timeline_store
            .load_timeline("100001", "c_batch")
            .expect("conversation timeline should load")
            .len(),
        1,
        "conversation timeline should still persist its message entries"
    );
    assert_eq!(
        timeline_store
            .load_timeline(
                "100001",
                client_route_sync_snapshot_scope(&client_route_feed_scope_key(
                    "100001", "default", "user", "1014", "d_phone"
                ))
                .as_str(),
            )
            .expect("client route sync timeline should load")
            .len(),
        1,
        "client route sync timeline should still persist its feed entries"
    );
}

#[test]
fn test_persist_contact_snapshot_recovers_from_poisoned_contact_store_lock() {
    let projection = TimelineProjectionService::default();
    let metadata_store = MemoryMetadataStore::default();
    poison_mutex(&projection.contacts);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        projection.persist_contact_snapshot(&metadata_store)
    }));
    assert!(
        result.is_ok(),
        "persist_contact_snapshot should not panic when contact store lock is poisoned"
    );
    let persist_result = result.expect("panic status should be captured");
    assert!(
        persist_result.is_ok(),
        "persist_contact_snapshot should recover from poisoned lock"
    );
}

#[test]
fn test_restore_conversation_snapshot_recovers_from_poisoned_summary_store_lock() {
    let projection = TimelineProjectionService::default();
    let metadata_store = MemoryMetadataStore::default();
    let timeline_store = MemoryTimelineProjectionStore::default();
    let snapshot_scope = conversation_snapshot_scope("100001", "c_demo");
    persist_metadata_snapshot(
        &metadata_store,
        snapshot_scope.as_str(),
        CONVERSATION_SUMMARY_KEY,
        &demo_summary("100001", "c_demo"),
    )
    .expect("summary snapshot should persist");

    poison_mutex(&projection.summaries);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        projection.restore_conversation_snapshot("100001", "default", "c_demo", &metadata_store,
            &timeline_store,
        )
    }));
    assert!(
        result.is_ok(),
        "restore_conversation_snapshot should not panic when summary store lock is poisoned"
    );
    let restore_result = result.expect("panic status should be captured");
    assert!(
        restore_result.is_ok(),
        "restore_conversation_snapshot should recover from poisoned lock"
    );
}

#[test]
fn test_persist_client_route_sync_snapshot_recovers_from_poisoned_registered_client_route_store_lock()
 {
    let projection = TimelineProjectionService::default();
    let metadata_store = MemoryMetadataStore::default();
    let timeline_store = MemoryTimelineProjectionStore::default();
    poison_mutex(&projection.registered_client_routes);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        projection.persist_client_route_sync_snapshot(&metadata_store, &timeline_store)
    }));
    assert!(
        result.is_ok(),
        "persist_client_route_sync_snapshot should not panic when registered-client-route lock is poisoned"
    );
    let persist_result = result.expect("panic status should be captured");
    assert!(
        persist_result.is_ok(),
        "persist_client_route_sync_snapshot should recover from poisoned lock"
    );
}

#[test]
fn test_restore_contact_snapshot_recovers_from_poisoned_binding_store_lock() {
    let projection = TimelineProjectionService::default();
    let metadata_store = MemoryMetadataStore::default();
    persist_metadata_snapshot(
        &metadata_store,
        CONTACT_CATALOG_SCOPE,
        CONTACT_OWNERS_KEY,
        &Vec::<ContactOwnerCatalogEntry>::new(),
    )
    .expect("contact owner catalog should persist");
    persist_metadata_snapshot(
        &metadata_store,
        CONTACT_CATALOG_SCOPE,
        CONTACT_DIRECT_CHAT_BINDINGS_KEY,
        &vec![crate::model::ContactDirectChatBindingView {
            tenant_id: Some("100001".into()),
            direct_chat_id: "dc_demo".into(),
            conversation_id: "c_demo".into(),
            bound_at: "2026-04-01T00:00:00.000Z".into(),
            status: im_domain_core::social::DirectChatStatus::Active,
            updated_at: Some("2026-04-01T00:00:00.000Z".into()),
        }],
    )
    .expect("contact direct-chat binding snapshot should persist");
    poison_mutex(&projection.direct_chat_bindings);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        projection.restore_contact_snapshot(&metadata_store)
    }));
    assert!(
        result.is_ok(),
        "restore_contact_snapshot should not panic when direct-chat-binding lock is poisoned"
    );
    let restore_result = result.expect("panic status should be captured");
    assert!(
        restore_result.is_ok(),
        "restore_contact_snapshot should recover from poisoned lock"
    );
}

#[test]
fn test_restore_client_route_sync_snapshot_recovers_from_poisoned_sequence_store_lock() {
    let projection = TimelineProjectionService::default();
    let metadata_store = MemoryMetadataStore::default();
    let timeline_store = MemoryTimelineProjectionStore::default();
    let device_scope = ClientRouteSyncScopeCatalogEntry {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_id: "1".into(),
        principal_kind: "user".into(),
        device_id: "d_demo".into(),
    };
    persist_metadata_snapshot(
        &metadata_store,
        CLIENT_ROUTE_SYNC_CATALOG_SCOPE,
        REGISTERED_CLIENT_ROUTE_PRINCIPALS_KEY,
        &Vec::<PrincipalSnapshotCatalogEntry>::new(),
    )
    .expect("client route sync principal catalog should persist");
    persist_metadata_snapshot(
        &metadata_store,
        CLIENT_ROUTE_SYNC_CATALOG_SCOPE,
        CLIENT_ROUTE_SYNC_SCOPE_CATALOG_KEY,
        &vec![device_scope.clone()],
    )
    .expect("client route sync scope catalog should persist");
    persist_metadata_snapshot(
        &metadata_store,
        client_route_sync_snapshot_scope(&client_route_feed_scope_key(
            device_scope.tenant_id.as_str(),
            device_scope.organization_id.as_str(),
            device_scope.principal_kind.as_str(),
            device_scope.principal_id.as_str(),
            device_scope.device_id.as_str(),
        ))
        .as_str(),
        CLIENT_ROUTE_SYNC_SEQUENCE_KEY,
        &42_u64,
    )
    .expect("client route sync sequence snapshot should persist");
    poison_mutex(&projection.client_route_sync_sequences);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        projection.restore_client_route_sync_snapshot(&metadata_store, &timeline_store)
    }));
    assert!(
        result.is_ok(),
        "restore_client_route_sync_snapshot should not panic when sequence lock is poisoned"
    );
    let restore_result = result.expect("panic status should be captured");
    assert!(
        restore_result.is_ok(),
        "restore_client_route_sync_snapshot should recover from poisoned lock"
    );
}
