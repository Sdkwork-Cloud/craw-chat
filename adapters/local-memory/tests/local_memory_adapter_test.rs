use std::collections::BTreeMap;

use im_adapters_local_memory::{
    MemoryAutomationExecutionStore, MemoryCommitJournal, MemoryMetadataStore,
    MemoryNotificationTaskStore, MemoryPresenceStateStore, MemoryRealtimeCheckpointStore,
    MemoryRealtimeDisconnectFenceStore, MemoryRealtimeSubscriptionStore,
    MemoryStorageDomainSnapshotStore, MemoryStreamStateStore, MemoryTimelineProjectionStore,
};
use im_domain_core::{
    automation::{AutomationExecution, AutomationExecutionState},
    message::Sender,
    notification::{NotificationStatus, NotificationTask},
    presence::{PresenceClientView, PresenceStatus},
    realtime::RealtimeSubscription,
    stream::{StreamDurabilityClass, StreamFrame, StreamSession, StreamSessionState},
};
use im_platform_contracts::{
    AutomationExecutionRecord, AutomationExecutionStore, CommitJournal, MetadataStore,
    NotificationTaskRecord, NotificationTaskStore, PresenceStateRecord, PresenceStateStore,
    RealtimeCheckpointRecord, RealtimeCheckpointStore, RealtimeDisconnectFenceRecord,
    RealtimeDisconnectFenceStore, RealtimeSubscriptionRecord, RealtimeSubscriptionStore,
    StreamStateRecord, StreamStateStore, TimelineProjectionStore,
};
use im_storage_contracts::{
    StorageBindingRecord, StorageCatalog, StorageConfigRecord, StorageCredentialMode,
    StorageDomainSnapshot, StorageDomainSnapshotStore, StorageSecretRecord,
};

fn realtime_disconnect_fence_record(
    principal_id: &str,
    session_id: &str,
    owner_node_id: &str,
    disconnected_at: &str,
) -> RealtimeDisconnectFenceRecord {
    RealtimeDisconnectFenceRecord {
        tenant_id: "100001".into(),
            organization_id: "0".into(),
        principal_kind: "user".into(),
        principal_id: principal_id.into(),
        device_id: "d_pad".into(),
        session_id: Some(session_id.into()),
        owner_node_id: owner_node_id.into(),
        disconnected_at: disconnected_at.into(),
        fence_token: format!(
            "fence:100001:{principal_id}:d_pad:{session_id}:{owner_node_id}:{disconnected_at}"
        ),
    }
}

fn stream_state_record(
    state: StreamSessionState,
    last_frame_seq: u64,
    last_checkpoint_seq: Option<u64>,
    complete_frame_seq: Option<u64>,
    frame_seqs: Vec<u64>,
    updated_at: &str,
) -> StreamStateRecord {
    StreamStateRecord {
        tenant_id: "100001".into(),
        stream_id: "st_demo".into(),
        session: StreamSession {
            tenant_id: "100001".into(),
            stream_id: "st_demo".into(),
            owner_principal_id: "1".into(),
            owner_principal_kind: "user".into(),
            stream_type: "custom.delta.text".into(),
            scope_kind: "request".into(),
            scope_id: "req_demo".into(),
            durability_class: StreamDurabilityClass::DurableSession,
            ordering_scope: "stream".into(),
            schema_ref: Some("custom.delta.text.v1".into()),
            state,
            last_frame_seq,
            last_checkpoint_seq,
            result_message_id: complete_frame_seq.map(|_| "msg_done".into()),
            complete_frame_seq,
            abort_frame_seq: None,
            abort_reason: None,
            opened_at: "2026-05-06T00:00:00.000Z".into(),
            closed_at: complete_frame_seq.map(|_| "2026-05-06T00:00:03.000Z".into()),
            expires_at: None,
        },
        frames: frame_seqs.into_iter().map(stream_frame).collect(),
        updated_at: updated_at.into(),
    }
}

fn stream_frame(frame_seq: u64) -> StreamFrame {
    StreamFrame {
        tenant_id: "100001".into(),
        stream_id: "st_demo".into(),
        stream_type: "custom.delta.text".into(),
        scope_kind: "request".into(),
        scope_id: "req_demo".into(),
        frame_seq,
        frame_type: "delta".into(),
        schema_ref: Some("custom.delta.text.v1".into()),
        encoding: "json".into(),
        payload: format!("{{\"seq\":{frame_seq}}}"),
        sender: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_demo".into()),
            session_id: Some("s_demo".into()),
            metadata: BTreeMap::new(),
        },
        attributes: BTreeMap::new(),
        occurred_at: format!("2026-05-06T00:00:0{frame_seq}.000Z"),
    }
}

fn notification_task_record(
    notification_id: &str,
    recipient_kind: &str,
    recipient_id: &str,
    status: NotificationStatus,
    dispatched_at: Option<&str>,
    failure_reason: Option<&str>,
    updated_at: &str,
) -> NotificationTaskRecord {
    NotificationTaskRecord {
        tenant_id: "100001".into(),
        notification_id: notification_id.into(),
        task: NotificationTask {
            tenant_id: "100001".into(),
            notification_id: notification_id.into(),
            source_event_id: format!("evt_{notification_id}"),
            source_event_type: "message.posted".into(),
            category: "message.new".into(),
            channel: "inapp".into(),
            recipient_id: recipient_id.into(),
            recipient_kind: recipient_kind.to_owned(),
            status,
            title: Some("hello".into()),
            body: Some("world".into()),
            payload: Some("{\"conversationId\":\"c_demo\"}".into()),
            requested_at: "2026-05-06T00:00:00.000Z".into(),
            dispatched_at: dispatched_at.map(str::to_owned),
            failure_reason: failure_reason.map(str::to_owned),
        },
        updated_at: updated_at.into(),
    }
}

fn automation_execution_record(
    state: AutomationExecutionState,
    retry_count: u32,
    output_payload: Option<&str>,
    completed_at: Option<&str>,
    failure_reason: Option<&str>,
    updated_at: &str,
) -> AutomationExecutionRecord {
    AutomationExecutionRecord {
        tenant_id: "100001".into(),
        principal_id: "1".into(),
        execution_id: "ae_demo".into(),
        execution: AutomationExecution {
            tenant_id: "100001".into(),
            principal_id: "1".into(),
            principal_kind: "user".into(),
            execution_id: "ae_demo".into(),
            trigger_type: "webhook.manual".into(),
            target_kind: "workflow".into(),
            target_ref: "wf_demo".into(),
            input_payload: Some("{\"conversationId\":\"c_demo\"}".into()),
            output_payload: output_payload.map(str::to_owned),
            state,
            retry_count,
            requested_at: "2026-05-06T00:00:00.000Z".into(),
            completed_at: completed_at.map(str::to_owned),
            failure_reason: failure_reason.map(str::to_owned),
        },
        updated_at: updated_at.into(),
    }
}

#[test]
fn test_memory_presence_state_store_uses_principal_index_for_listing() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");

    assert!(
        source.contains("presence_by_principal: HashMap<String, BTreeSet<String>>"),
        "local memory presence store should maintain a tenant/principal -> device-key index"
    );
    assert!(
        source.contains("by_device: HashMap<String, PresenceStateRecord>"),
        "local memory presence store should keep device records in the same indexed state object"
    );
    assert!(
        source.contains("online_by_seen_at: BTreeSet<PresenceOnlineSeenAtKey>"),
        "local memory presence store should maintain an online last-seen index for expiration jobs"
    );
    assert!(
        !source.contains(".values()\n            .filter(|record| record.tenant_id == tenant_id && record.principal_id == principal_id)"),
        "local memory presence store must not full-scan all records for list_states_for_principal"
    );
}

#[test]
fn test_memory_notification_task_store_uses_recipient_kind_index_for_listing() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");

    assert!(
        source.contains("tasks_by_recipient: HashMap<String, BTreeSet<String>>"),
        "local memory notification task store should maintain a tenant/recipient-kind/recipient-id -> notification-key index"
    );
    assert!(
        source.contains("notification_recipient_scope_key("),
        "notification task index must include recipient_kind in its scope key"
    );
    assert!(
        !source.contains(".values()\n            .filter(|record| {\n                record.tenant_id == tenant_id && record.task.recipient_id == recipient_id\n            })"),
        "local memory notification task listing must not full-scan all tasks"
    );
}

#[test]
fn test_memory_notification_task_store_lists_only_matching_recipient_kind() {
    let store = MemoryNotificationTaskStore::default();
    store
        .save_task(notification_task_record(
            "ntf_user",
            "user",
            "shared_id",
            NotificationStatus::Dispatched,
            Some("2026-05-06T00:00:02.000Z"),
            None,
            "2026-05-06T00:00:02.000Z",
        ))
        .expect("user notification save should succeed");
    store
        .save_task(notification_task_record(
            "ntf_system",
            "system",
            "shared_id",
            NotificationStatus::Dispatched,
            Some("2026-05-06T00:00:03.000Z"),
            None,
            "2026-05-06T00:00:03.000Z",
        ))
        .expect("system notification save should succeed");

    let listed = store
        .list_tasks_for_recipient("100001", "user", "shared_id")
        .expect("recipient listing should succeed");

    assert_eq!(
        listed
            .iter()
            .map(|record| record.notification_id.as_str())
            .collect::<Vec<_>>(),
        vec!["ntf_user"]
    );
}

#[test]
fn test_memory_notification_task_store_rejects_stale_status_regression_writes() {
    let store = MemoryNotificationTaskStore::default();
    store
        .save_task(notification_task_record(
            "ntf_demo",
            "user",
            "1",
            NotificationStatus::Dispatched,
            Some("2026-05-06T00:00:02.000Z"),
            None,
            "2026-05-06T00:00:02.000Z",
        ))
        .expect("current notification save should succeed");
    store
        .save_task(notification_task_record(
            "ntf_demo",
            "user",
            "1",
            NotificationStatus::Requested,
            None,
            None,
            "2026-05-06T00:00:01.000Z",
        ))
        .expect("stale notification save should not fail the caller");

    let restored = store
        .load_task("100001", "ntf_demo")
        .expect("notification load should succeed")
        .expect("notification should be present");
    assert_eq!(restored.task.status, NotificationStatus::Dispatched);
    assert_eq!(
        restored.task.dispatched_at.as_deref(),
        Some("2026-05-06T00:00:02.000Z")
    );
    assert_eq!(restored.updated_at, "2026-05-06T00:00:02.000Z");
}

#[test]
fn test_memory_presence_state_store_lists_stale_online_devices_by_seen_at() {
    let store = MemoryPresenceStateStore::default();
    for (device_id, status, last_seen_at) in [
        ("d_new", PresenceStatus::Online, "2026-05-06T00:00:03.000Z"),
        (
            "d_old_2",
            PresenceStatus::Online,
            "2026-05-06T00:00:02.000Z",
        ),
        (
            "d_offline",
            PresenceStatus::Offline,
            "2026-05-06T00:00:01.000Z",
        ),
        (
            "d_old_1",
            PresenceStatus::Online,
            "2026-05-06T00:00:01.000Z",
        ),
    ] {
        store
            .save_state(PresenceStateRecord {
                tenant_id: "100001".into(),
            organization_id: "0".into(),
                principal_kind: "user".into(),
                principal_id: "1".into(),
                device_id: device_id.into(),
                presence: PresenceClientView {
                    tenant_id: "100001".into(),
                    principal_id: "1".into(),
                    device_id: device_id.into(),
                    platform: None,
                    session_id: Some(format!("s_{device_id}")),
                    status,
                    last_sync_seq: 0,
                    last_resume_at: Some(last_seen_at.into()),
                    last_seen_at: Some(last_seen_at.into()),
                },
                resume_required: false,
                updated_at: last_seen_at.into(),
            })
            .expect("presence state save should succeed");
    }

    let stale = store
        .list_online_states_seen_at_or_before("2026-05-06T00:00:02.000Z", 10)
        .expect("stale online list should succeed");

    assert_eq!(
        stale
            .iter()
            .map(|record| record.device_id.as_str())
            .collect::<Vec<_>>(),
        vec!["d_old_1", "d_old_2"]
    );

    let limited = store
        .list_online_states_seen_at_or_before("2026-05-06T00:00:02.000Z", 1)
        .expect("limited stale online list should succeed");
    assert_eq!(limited[0].device_id, "d_old_1");
}

#[test]
fn test_memory_presence_state_store_seen_at_cutoff_compares_rfc3339_by_instant() {
    let store = MemoryPresenceStateStore::default();
    for (device_id, last_seen_at) in [
        ("d_later_fraction", "2026-05-06T00:00:00.100Z"),
        ("d_whole_second", "2026-05-06T00:00:00Z"),
    ] {
        store
            .save_state(PresenceStateRecord {
                tenant_id: "100001".into(),
            organization_id: "0".into(),
                principal_kind: "user".into(),
                principal_id: "1".into(),
                device_id: device_id.into(),
                presence: PresenceClientView {
                    tenant_id: "100001".into(),
                    principal_id: "1".into(),
                    device_id: device_id.into(),
                    platform: None,
                    session_id: Some(format!("s_{device_id}")),
                    status: PresenceStatus::Online,
                    last_sync_seq: 0,
                    last_resume_at: Some(last_seen_at.into()),
                    last_seen_at: Some(last_seen_at.into()),
                },
                resume_required: false,
                updated_at: last_seen_at.into(),
            })
            .expect("presence state save should succeed");
    }

    let stale = store
        .list_online_states_seen_at_or_before("2026-05-06T00:00:00Z", 10)
        .expect("stale online list should succeed");

    assert_eq!(
        stale
            .iter()
            .map(|record| record.device_id.as_str())
            .collect::<Vec<_>>(),
        vec!["d_whole_second"]
    );
}

#[test]
fn test_memory_presence_state_store_conditionally_expires_only_stale_online_state() {
    let store = MemoryPresenceStateStore::default();
    store
        .save_state(PresenceStateRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            presence: PresenceClientView {
                tenant_id: "100001".into(),
                principal_id: "1".into(),
                device_id: "d_pad".into(),
                platform: None,
                session_id: Some("s_old".into()),
                status: PresenceStatus::Online,
                last_sync_seq: 7,
                last_resume_at: Some("2026-05-06T00:00:00.000Z".into()),
                last_seen_at: Some("2026-05-06T00:00:00.000Z".into()),
            },
            resume_required: false,
            updated_at: "2026-05-06T00:00:00.000Z".into(),
        })
        .expect("presence state save should succeed");

    let expired = store
        .expire_online_state_if_seen_at_or_before(
            "100001",
            "default",
            "user",
            "1",
            "d_pad",
            "2026-05-06T00:00:01.000Z",
            "2026-05-06T00:00:02.000Z",
        )
        .expect("conditional expire should succeed")
        .expect("stale online device should expire");
    assert_eq!(expired.presence.status.as_str(), "offline");
    assert!(expired.presence.session_id.is_none());
    assert!(expired.resume_required);

    let replay = store
        .expire_online_state_if_seen_at_or_before(
            "100001",
            "default",
            "user",
            "1",
            "d_pad",
            "2026-05-06T00:00:03.000Z",
            "2026-05-06T00:00:04.000Z",
        )
        .expect("replayed conditional expire should succeed");
    assert!(replay.is_none());
}

#[test]
fn test_memory_commit_journal_appends_in_order() {
    let journal = MemoryCommitJournal::with_partition("dev-memory-journal");

    let first = journal
        .append(im_domain_events::CommitEnvelope::minimal(
            "evt_1",
            "100001",
            "message.posted",
            "conversation",
            "c_demo",
            1,
        ))
        .expect("first append should succeed");
    let second = journal
        .append(im_domain_events::CommitEnvelope::minimal(
            "evt_2",
            "100001",
            "message.posted",
            "conversation",
            "c_demo",
            2,
        ))
        .expect("second append should succeed");

    assert_eq!(first.partition, "dev-memory-journal");
    assert_eq!(first.offset, 1);
    assert_eq!(second.partition, "dev-memory-journal");
    assert_eq!(second.offset, 2);
    assert_eq!(journal.recorded().len(), 2);
}

#[test]
fn test_memory_metadata_store_overwrites_snapshot_for_same_scope_and_key() {
    let metadata = MemoryMetadataStore::default();

    metadata
        .put_snapshot(
            "tenant:100001",
            "conversation:c_demo",
            "{\"state\":\"draft\"}",
        )
        .expect("first snapshot should succeed");
    metadata
        .put_snapshot(
            "tenant:100001",
            "conversation:c_demo",
            "{\"state\":\"ready\"}",
        )
        .expect("second snapshot should succeed");

    assert_eq!(
        metadata
            .snapshot("tenant:100001", "conversation:c_demo")
            .as_deref(),
        Some("{\"state\":\"ready\"}")
    );
}

#[test]
fn test_memory_metadata_store_does_not_collapse_delimiter_shaped_scope_and_key() {
    let metadata = MemoryMetadataStore::default();

    metadata
        .put_snapshot(
            "tenant:100001",
            "conversation:c_demo",
            "{\"state\":\"one\"}",
        )
        .expect("first metadata snapshot should succeed");
    metadata
        .put_snapshot(
            "tenant:100001:conversation",
            "c_demo",
            "{\"state\":\"two\"}",
        )
        .expect("second metadata snapshot should succeed");

    assert_eq!(
        metadata
            .snapshot("tenant:100001", "conversation:c_demo")
            .as_deref(),
        Some("{\"state\":\"one\"}")
    );
    assert_eq!(
        metadata
            .snapshot("tenant:100001:conversation", "c_demo")
            .as_deref(),
        Some("{\"state\":\"two\"}")
    );
}

#[test]
fn test_memory_timeline_projection_store_upserts_by_sequence() {
    let projection = MemoryTimelineProjectionStore::default();

    projection
        .upsert_timeline_entry("100001", "c_demo", 1, "{\"summary\":\"first\"}")
        .expect("first upsert should succeed");
    projection
        .upsert_timeline_entry("100001", "c_demo", 2, "{\"summary\":\"second\"}")
        .expect("second upsert should succeed");
    projection
        .upsert_timeline_entry("100001", "c_demo", 2, "{\"summary\":\"second-v2\"}")
        .expect("idempotent upsert should succeed");

    assert_eq!(
        projection.entries("100001", "c_demo"),
        vec![
            (1, "{\"summary\":\"first\"}".to_string()),
            (2, "{\"summary\":\"second-v2\"}".to_string()),
        ]
    );
}

#[test]
fn test_memory_timeline_projection_store_isolates_same_scope_across_tenants() {
    let projection = MemoryTimelineProjectionStore::default();

    projection
        .upsert_timeline_entry("t_alpha", "c_shared", 1, "{\"summary\":\"alpha\"}")
        .expect("alpha tenant timeline upsert should succeed");
    projection
        .upsert_timeline_entry("t_beta", "c_shared", 1, "{\"summary\":\"beta\"}")
        .expect("beta tenant timeline upsert should succeed");

    assert_eq!(
        projection.entries("t_alpha", "c_shared"),
        vec![(1, "{\"summary\":\"alpha\"}".to_string())]
    );
    assert_eq!(
        projection.entries("t_beta", "c_shared"),
        vec![(1, "{\"summary\":\"beta\"}".to_string())]
    );
}

#[test]
fn test_memory_realtime_checkpoint_store_overwrites_same_device_checkpoint() {
    let store = MemoryRealtimeCheckpointStore::default();

    store
        .save_checkpoint(RealtimeCheckpointRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            latest_realtime_seq: 3,
            acked_through_seq: 2,
            trimmed_through_seq: 2,
            capacity_trimmed_event_count: 0,
            capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            updated_at: "2026-04-05T12:00:00Z".into(),
        })
        .expect("first checkpoint save should succeed");
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
            capacity_trimmed_event_count: 0,
            capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            updated_at: "2026-04-05T12:01:00Z".into(),
        })
        .expect("second checkpoint save should succeed");

    let checkpoint = store
        .load_checkpoint("100001", "default", "user", "1", "d_pad")
        .expect("checkpoint load should succeed")
        .expect("checkpoint should exist");
    assert_eq!(checkpoint.latest_realtime_seq, 5);
    assert_eq!(checkpoint.acked_through_seq, 4);
    assert_eq!(checkpoint.trimmed_through_seq, 4);
}

#[test]
fn test_memory_realtime_checkpoint_store_does_not_collapse_delimiter_shaped_device_scope() {
    let store = MemoryRealtimeCheckpointStore::default();

    store
        .save_checkpoint(RealtimeCheckpointRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "u:demo".into(),
            device_id: "d_pad".into(),
            latest_realtime_seq: 3,
            acked_through_seq: 2,
            trimmed_through_seq: 2,
            capacity_trimmed_event_count: 0,
            capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            updated_at: "2026-05-06T00:00:01.000Z".into(),
        })
        .expect("first checkpoint save should succeed");
    store
        .save_checkpoint(RealtimeCheckpointRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "u".into(),
            device_id: "demo:d_pad".into(),
            latest_realtime_seq: 9,
            acked_through_seq: 8,
            trimmed_through_seq: 8,
            capacity_trimmed_event_count: 0,
            capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            updated_at: "2026-05-06T00:00:02.000Z".into(),
        })
        .expect("second checkpoint save should succeed");

    assert_eq!(
        store
            .load_checkpoint("100001", "default", "user", "u:demo", "d_pad")
            .expect("first checkpoint load should succeed")
            .expect("first checkpoint should exist")
            .latest_realtime_seq,
        3
    );
    assert_eq!(
        store
            .load_checkpoint("100001", "default", "user", "u", "demo:d_pad")
            .expect("second checkpoint load should succeed")
            .expect("second checkpoint should exist")
            .latest_realtime_seq,
        9
    );
}

#[test]
fn test_memory_realtime_checkpoint_store_rejects_stale_regression_writes() {
    let store = MemoryRealtimeCheckpointStore::default();
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
        .checkpoint("100001", "default", "user", "1", "d_pad")
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
fn test_memory_realtime_checkpoint_store_saves_checkpoint_batch_under_one_store_operation() {
    let store = MemoryRealtimeCheckpointStore::default();

    store
        .save_checkpoints(vec![
            RealtimeCheckpointRecord {
                tenant_id: "100001".into(),
            organization_id: "0".into(),
                principal_kind: "user".into(),
                principal_id: "1".into(),
                device_id: "d_pad".into(),
                latest_realtime_seq: 3,
                acked_through_seq: 2,
                trimmed_through_seq: 2,
                capacity_trimmed_event_count: 0,
                capacity_trimmed_through_seq: 0,
                last_capacity_trimmed_at: None,
                updated_at: "2026-05-06T00:00:01.000Z".into(),
            },
            RealtimeCheckpointRecord {
                tenant_id: "100001".into(),
            organization_id: "0".into(),
                principal_kind: "user".into(),
                principal_id: "1".into(),
                device_id: "d_phone".into(),
                latest_realtime_seq: 4,
                acked_through_seq: 1,
                trimmed_through_seq: 1,
                capacity_trimmed_event_count: 0,
                capacity_trimmed_through_seq: 0,
                last_capacity_trimmed_at: None,
                updated_at: "2026-05-06T00:00:01.000Z".into(),
            },
        ])
        .expect("checkpoint batch save should succeed");

    assert_eq!(
        store
            .checkpoint("100001", "default", "user", "1", "d_pad")
            .expect("pad checkpoint should exist")
            .latest_realtime_seq,
        3
    );
    assert_eq!(
        store
            .checkpoint("100001", "default", "user", "1", "d_phone")
            .expect("phone checkpoint should exist")
            .latest_realtime_seq,
        4
    );
}

#[test]
fn test_memory_realtime_disconnect_fence_store_overwrites_and_clears_same_device_fence() {
    let store = MemoryRealtimeDisconnectFenceStore::default();

    store
        .save_fence(realtime_disconnect_fence_record(
            "1",
            "s_old",
            "node_a",
            "2026-04-06T12:00:00Z",
        ))
        .expect("first fence save should succeed");
    store
        .save_fence(realtime_disconnect_fence_record(
            "1",
            "s_new",
            "node_b",
            "2026-04-06T12:01:00Z",
        ))
        .expect("second fence save should succeed");

    let fence = store
        .load_fence("100001", "default", "user", "1", "d_pad")
        .expect("fence load should succeed")
        .expect("fence should exist");
    assert_eq!(fence.session_id.as_deref(), Some("s_new"));
    assert_eq!(fence.owner_node_id, "node_b");

    assert!(
        store
            .clear_fence("100001", "default", "user", "1", "d_pad")
            .expect("fence clear should succeed")
    );
    assert!(
        store
            .load_fence("100001", "default", "user", "1", "d_pad")
            .expect("fence load should succeed")
            .is_none()
    );
}

#[test]
fn test_memory_realtime_disconnect_fence_store_rejects_stale_regression_writes() {
    let store = MemoryRealtimeDisconnectFenceStore::default();
    store
        .save_fence(realtime_disconnect_fence_record(
            "1",
            "s_new",
            "node_b",
            "2026-05-06T00:00:02.000Z",
        ))
        .expect("new fence save should succeed");
    store
        .save_fence(realtime_disconnect_fence_record(
            "1",
            "s_old",
            "node_a",
            "2026-05-06T00:00:01.000Z",
        ))
        .expect("stale fence save should not fail the caller");

    let fence = store
        .fence("100001", "default", "user", "1", "d_pad")
        .expect("disconnect fence should be present");
    assert_eq!(fence.session_id.as_deref(), Some("s_new"));
    assert_eq!(fence.owner_node_id, "node_b");
    assert_eq!(fence.disconnected_at, "2026-05-06T00:00:02.000Z");
}

#[test]
fn test_memory_realtime_disconnect_fence_store_conditionally_clears_only_old_fence() {
    let store = MemoryRealtimeDisconnectFenceStore::default();
    store
        .save_fence(realtime_disconnect_fence_record(
            "1",
            "s_new",
            "node_b",
            "2026-05-06T00:00:02.000Z",
        ))
        .expect("new fence save should succeed");

    let cleared = store
        .clear_fence_disconnected_at_or_before(
            "100001",
            "default",
            "user",
            "1",
            "d_pad",
            "2026-05-06T00:00:01.000Z",
        )
        .expect("conditional fence clear should succeed");

    assert!(!cleared);
    assert!(
        store.fence("100001", "default", "user", "1", "d_pad").is_some(),
        "newer disconnect fence must not be deleted by an older resume cleanup"
    );
}

#[test]
fn test_memory_realtime_disconnect_fence_store_compares_cutoff_by_rfc3339_instant() {
    let store = MemoryRealtimeDisconnectFenceStore::default();
    store
        .save_fence(realtime_disconnect_fence_record(
            "1",
            "s_new",
            "node_b",
            "2026-05-06T00:00:00.100Z",
        ))
        .expect("fence save should succeed");

    let cleared = store
        .clear_fence_disconnected_at_or_before(
            "100001",
            "default",
            "user",
            "1",
            "d_pad",
            "2026-05-06T00:00:00Z",
        )
        .expect("conditional fence clear should succeed");

    assert!(!cleared);
    assert!(
        store.fence("100001", "default", "user", "1", "d_pad").is_some(),
        "fractional-second later disconnect fence must not be deleted by whole-second cutoff"
    );
}

#[test]
fn test_memory_realtime_disconnect_fence_store_clears_only_exact_matching_fence() {
    let store = MemoryRealtimeDisconnectFenceStore::default();
    let stale =
        realtime_disconnect_fence_record("1", "s_old", "node_a", "2026-05-06T00:00:02.000Z");
    let current =
        realtime_disconnect_fence_record("1", "s_new", "node_b", "2026-05-06T00:00:02.000Z");
    store
        .save_fence(current.clone())
        .expect("current fence save should succeed");

    let cleared = store
        .clear_fence_if_matches(&stale)
        .expect("exact fence clear should succeed");

    assert!(!cleared);
    assert_eq!(
        store
            .fence("100001", "default", "user", "1", "d_pad")
            .expect("disconnect fence should still exist"),
        current
    );
}

#[test]
fn test_memory_realtime_subscription_store_does_not_clear_newer_subscription() {
    let store = MemoryRealtimeSubscriptionStore::default();
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
            .subscriptions("100001", "default", "user", "1", "d_pad")
            .is_some(),
        "newer subscription must not be deleted by an older disconnect cleanup"
    );
}

#[test]
fn test_memory_realtime_subscription_store_compares_cutoff_by_rfc3339_instant() {
    let store = MemoryRealtimeSubscriptionStore::default();
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

    assert!(!cleared);
    assert!(
        store
            .subscriptions("100001", "default", "user", "1", "d_pad")
            .is_some(),
        "fractional-second later subscription must not be deleted by whole-second cutoff"
    );
}

#[test]
fn test_memory_realtime_subscription_store_loads_matching_scope_event_candidates() {
    let store = MemoryRealtimeSubscriptionStore::default();
    for (device_id, scope_id, event_types) in [
        ("d_match", "c_demo", vec!["message.posted"]),
        ("d_wildcard", "c_demo", Vec::new()),
        ("d_other_scope", "c_other", vec!["message.posted"]),
        ("d_other_event", "c_demo", vec!["message.read"]),
    ] {
        store
            .save_subscriptions(RealtimeSubscriptionRecord {
                tenant_id: "100001".into(),
            organization_id: "0".into(),
                principal_kind: "user".into(),
                principal_id: "1".into(),
                device_id: device_id.into(),
                items: vec![RealtimeSubscription {
                    scope_type: "conversation".into(),
                    scope_id: scope_id.into(),
                    event_types: event_types.into_iter().map(str::to_owned).collect(),
                    subscribed_at: "2026-05-06T00:00:02.000Z".into(),
                }],
                synced_at: "2026-05-06T00:00:02.000Z".into(),
            })
            .expect("subscription save should succeed");
    }

    let matches = store
        .load_matching_subscriptions(im_platform_contracts::RealtimeMatchingSubscriptionQuery {
            tenant_id: "100001",
            organization_id: "0",
            principal_kind: "user",
            principal_id: "1",
            scope_type: "conversation",
            scope_id: "c_demo",
            event_type: "message.posted",
            candidate_device_ids: &[
                "d_match".into(),
                "d_wildcard".into(),
                "d_other_scope".into(),
                "d_other_event".into(),
                "d_missing".into(),
            ],
        })
        .expect("matching subscription load should succeed");
    let device_ids = matches
        .into_iter()
        .map(|record| record.device_id)
        .collect::<Vec<_>>();

    assert_eq!(device_ids, vec!["d_match", "d_wildcard"]);
}

#[test]
fn test_memory_stream_state_store_rejects_stale_cursor_and_frame_regression() {
    let store = MemoryStreamStateStore::default();
    store
        .save_state(stream_state_record(
            StreamSessionState::Completed,
            3,
            Some(2),
            Some(3),
            vec![1, 2, 3],
            "2026-05-06T00:00:03.000Z",
        ))
        .expect("current stream state save should succeed");
    store
        .save_state(stream_state_record(
            StreamSessionState::Active,
            1,
            None,
            None,
            vec![1],
            "2026-05-06T00:00:01.000Z",
        ))
        .expect("stale stream state save should not fail the caller");

    let state = store
        .state("100001", "st_demo")
        .expect("stream state should be present");
    assert_eq!(state.session.state, StreamSessionState::Completed);
    assert_eq!(state.session.last_frame_seq, 3);
    assert_eq!(state.session.last_checkpoint_seq, Some(2));
    assert_eq!(state.session.complete_frame_seq, Some(3));
    assert_eq!(
        state
            .frames
            .iter()
            .map(|frame| frame.frame_seq)
            .collect::<Vec<_>>(),
        vec![1, 2, 3]
    );
    assert_eq!(state.updated_at, "2026-05-06T00:00:03.000Z");
}

#[test]
fn test_memory_automation_execution_store_isolates_same_actor_id_across_principal_kind() {
    let store = MemoryAutomationExecutionStore::default();

    for principal_kind in ["user", "system"] {
        store
            .save_execution(AutomationExecutionRecord {
                tenant_id: "100001".into(),
                principal_id: "1".into(),
                execution_id: "ae_kind_isolation".into(),
                execution: AutomationExecution {
                    tenant_id: "100001".into(),
                    principal_id: "1".into(),
                    principal_kind: principal_kind.into(),
                    execution_id: "ae_kind_isolation".into(),
                    trigger_type: "webhook.manual".into(),
                    target_kind: "workflow".into(),
                    target_ref: "wf_demo".into(),
                    input_payload: Some("{\"conversationId\":\"c_demo\"}".into()),
                    output_payload: Some("{\"accepted\":true}".into()),
                    state: AutomationExecutionState::Succeeded,
                    retry_count: 0,
                    requested_at: "2026-04-06T00:00:00.000Z".into(),
                    completed_at: Some("2026-04-06T00:00:01.000Z".into()),
                    failure_reason: None,
                },
                updated_at: "2026-04-06T00:00:01.000Z".into(),
            })
            .expect("save should succeed");
    }

    let user_execution = store
        .load_execution("100001", "user", "1", "ae_kind_isolation")
        .expect("user execution load should succeed")
        .expect("user execution should exist");
    let system_execution = store
        .load_execution("100001", "system", "1", "ae_kind_isolation")
        .expect("system execution load should succeed")
        .expect("system execution should exist");
    assert_eq!(user_execution.execution.principal_kind, "user");
    assert_eq!(system_execution.execution.principal_kind, "system");
}

#[test]
fn test_memory_automation_execution_store_rejects_stale_status_regression_writes() {
    let store = MemoryAutomationExecutionStore::default();
    store
        .save_execution(automation_execution_record(
            AutomationExecutionState::Succeeded,
            2,
            Some("{\"accepted\":true}"),
            Some("2026-05-06T00:00:02.000Z"),
            None,
            "2026-05-06T00:00:02.000Z",
        ))
        .expect("current automation execution save should succeed");
    store
        .save_execution(automation_execution_record(
            AutomationExecutionState::Running,
            1,
            None,
            None,
            None,
            "2026-05-06T00:00:01.000Z",
        ))
        .expect("stale automation execution save should not fail the caller");

    let restored = store
        .load_execution("100001", "user", "1", "ae_demo")
        .expect("automation execution load should succeed")
        .expect("automation execution should be present");
    assert_eq!(
        restored.execution.state,
        AutomationExecutionState::Succeeded
    );
    assert_eq!(restored.execution.retry_count, 2);
    assert_eq!(
        restored.execution.output_payload.as_deref(),
        Some("{\"accepted\":true}")
    );
    assert_eq!(
        restored.execution.completed_at.as_deref(),
        Some("2026-05-06T00:00:02.000Z")
    );
    assert_eq!(restored.updated_at, "2026-05-06T00:00:02.000Z");
}

#[test]
fn test_memory_storage_domain_snapshot_store_returns_none_for_unknown_domain() {
    let store = MemoryStorageDomainSnapshotStore::default();

    let snapshot = store
        .load_snapshot("object-storage")
        .expect("snapshot load should succeed");

    assert!(snapshot.is_none());
}

#[test]
fn test_memory_storage_domain_snapshot_store_overwrites_same_domain_snapshot() {
    let store = MemoryStorageDomainSnapshotStore::default();

    store
        .save_snapshot(
            StorageDomainSnapshot::new(StorageCatalog::object_storage())
                .with_binding(StorageBindingRecord::new_global("object-storage-aws"))
                .with_config(StorageConfigRecord::new_global("object-storage-aws")),
        )
        .expect("first snapshot save should succeed");
    store
        .save_snapshot(
            StorageDomainSnapshot::new(StorageCatalog::object_storage())
                .with_binding(StorageBindingRecord::new_global("object-storage-google"))
                .with_config(StorageConfigRecord::new_global("object-storage-google"))
                .with_secret(
                    StorageSecretRecord::new_global(
                        "object-storage-google",
                        StorageCredentialMode::ServiceAccountJson,
                        "{\"serviceAccountJson\":{\"client_email\":\"storage@sdkwork.local\"}}",
                    )
                    .with_secret_fingerprint("fp-object-storage-google"),
                ),
        )
        .expect("second snapshot save should succeed");

    let snapshot = store
        .load_snapshot("object-storage")
        .expect("snapshot load should succeed")
        .expect("snapshot should exist");

    assert_eq!(snapshot.bindings.len(), 1);
    assert_eq!(
        snapshot.bindings[0].provider_plugin_id,
        "object-storage-google"
    );
    assert_eq!(snapshot.secrets.len(), 1);
    assert_eq!(
        snapshot.secrets[0].secret_fingerprint,
        "fp-object-storage-google"
    );
}

#[test]
fn test_memory_storage_domain_snapshot_store_isolates_domains() {
    let store = MemoryStorageDomainSnapshotStore::default();

    store
        .save_snapshot(
            StorageDomainSnapshot::new(StorageCatalog::object_storage())
                .with_binding(StorageBindingRecord::new_global("object-storage-aws"))
                .with_config(StorageConfigRecord::new_global("object-storage-aws")),
        )
        .expect("object storage snapshot save should succeed");
    store
        .save_snapshot(
            StorageDomainSnapshot::new(StorageCatalog {
                domain: "chat-archive".into(),
                provider_schemas: Vec::new(),
            })
            .with_binding(StorageBindingRecord::new_global("archive-provider"))
            .with_config(StorageConfigRecord::new_global("archive-provider")),
        )
        .expect("archive storage snapshot save should succeed");

    let object_storage = store
        .load_snapshot("object-storage")
        .expect("object storage snapshot load should succeed")
        .expect("object storage snapshot should exist");
    let chat_archive = store
        .load_snapshot("chat-archive")
        .expect("chat archive snapshot load should succeed")
        .expect("chat archive snapshot should exist");

    assert_eq!(object_storage.catalog.domain, "object-storage");
    assert_eq!(
        object_storage.bindings[0].provider_plugin_id,
        "object-storage-aws"
    );
    assert_eq!(chat_archive.catalog.domain, "chat-archive");
    assert_eq!(
        chat_archive.bindings[0].provider_plugin_id,
        "archive-provider"
    );
}
