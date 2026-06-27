use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use im_adapters_local_disk::{
    FileCommitJournal, FileMetadataStore, FileRealtimeCheckpointStore,
    FileStorageDomainSnapshotStore, FileTimelineProjectionStore, read_commit_journal_file,
    validate_metadata_store_file, validate_storage_domain_snapshot_store_file,
    validate_timeline_projection_store_file,
};
use im_platform_contracts::{
    CommitEnvelope, CommitJournal, ContractError, MetadataSnapshotRecord, MetadataStore,
    RealtimeCheckpointRecord, RealtimeCheckpointStore, TimelineProjectionBatch,
    TimelineProjectionRecord, TimelineProjectionStore,
};
use im_storage_contracts::{
    StorageBindingRecord, StorageCatalog, StorageConfigRecord, StorageCredentialMode,
    StorageDomainSnapshot, StorageDomainSnapshotStore, StorageSecretRecord,
};

fn unique_store_file(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("sdkwork_im_{prefix}_{unique}.json"))
}

fn commit_envelope(thread_id: usize, seq: usize) -> CommitEnvelope {
    CommitEnvelope::minimal(
        &format!("evt_thread_{thread_id}_{seq}"),
        "100001",
        "test.appended",
        "test",
        &format!("agg_{thread_id}"),
        seq as u64,
    )
}

fn object_storage_snapshot(provider_plugin_id: &str) -> StorageDomainSnapshot {
    StorageDomainSnapshot::new(StorageCatalog::object_storage())
        .with_binding(StorageBindingRecord::new_global(provider_plugin_id))
        .with_config(StorageConfigRecord::new_global(provider_plugin_id))
}

#[test]
fn test_file_metadata_store_persists_latest_snapshot_across_reopen() {
    let file_path = unique_store_file("metadata_store");
    let store = FileMetadataStore::new(&file_path);

    store
        .put_snapshot(
            "tenant:100001",
            "conversation:c_demo",
            "{\"state\":\"draft\"}",
        )
        .expect("first metadata snapshot should succeed");
    store
        .put_snapshot(
            "tenant:100001",
            "conversation:c_demo",
            "{\"state\":\"ready\"}",
        )
        .expect("second metadata snapshot should succeed");

    let reopened = FileMetadataStore::new(&file_path);
    assert_eq!(
        reopened
            .snapshot("tenant:100001", "conversation:c_demo")
            .as_deref(),
        Some("{\"state\":\"ready\"}")
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_metadata_store_does_not_collapse_delimiter_shaped_scope_and_key() {
    let file_path = unique_store_file("metadata_store_delimiter_scope");
    let store = FileMetadataStore::new(&file_path);

    store
        .put_snapshot(
            "tenant:100001",
            "conversation:c_demo",
            "{\"state\":\"one\"}",
        )
        .expect("first metadata snapshot should succeed");
    store
        .put_snapshot(
            "tenant:100001:conversation",
            "c_demo",
            "{\"state\":\"two\"}",
        )
        .expect("second metadata snapshot should succeed");

    let reopened = FileMetadataStore::new(&file_path);
    assert_eq!(
        reopened
            .snapshot("tenant:100001", "conversation:c_demo")
            .as_deref(),
        Some("{\"state\":\"one\"}")
    );
    assert_eq!(
        reopened
            .snapshot("tenant:100001:conversation", "c_demo")
            .as_deref(),
        Some("{\"state\":\"two\"}")
    );

    let mut scopes = reopened.scopes_for_key("c_demo");
    scopes.sort();
    assert_eq!(scopes, vec!["tenant:100001:conversation".to_string()]);

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_metadata_store_scopes_for_key_handles_encoded_separator_characters() {
    let file_path = unique_store_file("metadata_store_encoded_separator");
    let store = FileMetadataStore::new(&file_path);

    store
        .put_snapshot(
            "tenant|100001",
            "conversation|c_demo",
            "{\"state\":\"one\"}",
        )
        .expect("metadata snapshot should succeed");

    let reopened = FileMetadataStore::new(&file_path);
    assert_eq!(
        reopened.scopes_for_key("conversation|c_demo"),
        vec!["tenant|100001".to_string()]
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_realtime_checkpoint_store_does_not_collapse_delimiter_shaped_device_scope() {
    let file_path = unique_store_file("realtime_checkpoint_store_delimiter_scope");
    let store = FileRealtimeCheckpointStore::new(&file_path);

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

    let reopened = FileRealtimeCheckpointStore::new(&file_path);
    assert_eq!(
        reopened
            .load_checkpoint("100001", "default", "user", "u:demo", "d_pad")
            .expect("first checkpoint load should succeed")
            .expect("first checkpoint should exist")
            .latest_realtime_seq,
        3
    );
    assert_eq!(
        reopened
            .load_checkpoint("100001", "default", "user", "u", "demo:d_pad")
            .expect("second checkpoint load should succeed")
            .expect("second checkpoint should exist")
            .latest_realtime_seq,
        9
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_commit_journal_preserves_cross_instance_appends() {
    let file_path = unique_store_file("commit_journal_concurrent");
    let thread_count = 4;
    let appends_per_thread = 64;
    let barrier = Arc::new(Barrier::new(thread_count));
    let mut handles = Vec::new();

    for thread_id in 0..thread_count {
        let barrier = barrier.clone();
        let file_path = file_path.clone();
        handles.push(thread::spawn(move || {
            let journal = FileCommitJournal::new("local-disk-test", &file_path);
            for seq in 0..appends_per_thread {
                barrier.wait();
                journal
                    .append(commit_envelope(thread_id, seq))
                    .expect("cross-instance append should succeed");
            }
        }));
    }

    for handle in handles {
        handle.join().expect("writer thread should join");
    }

    let events = read_commit_journal_file(&file_path)
        .expect("commit journal should remain readable after concurrent appends");
    assert_eq!(
        events.len(),
        thread_count * appends_per_thread,
        "concurrent appends from distinct journal instances should not lose events"
    );

    let _ = fs::remove_file(&file_path);
    let _ = fs::remove_file(file_path.with_extension("json.lock"));
}

#[test]
fn test_file_metadata_store_preserves_cross_instance_snapshot_updates() {
    let file_path = unique_store_file("metadata_store_concurrent");
    let thread_count = 4;
    let writes_per_thread = 32;
    let barrier = Arc::new(Barrier::new(thread_count));
    let mut handles = Vec::new();

    for thread_id in 0..thread_count {
        let barrier = barrier.clone();
        let file_path = file_path.clone();
        handles.push(thread::spawn(move || {
            let store = FileMetadataStore::new(&file_path);
            for seq in 0..writes_per_thread {
                let key = format!("conversation:c_{thread_id}_{seq}");
                let value = format!("{{\"thread\":{thread_id},\"seq\":{seq}}}");
                barrier.wait();
                store
                    .put_snapshot("tenant:100001", key.as_str(), value.as_str())
                    .expect("cross-instance metadata snapshot should succeed");
            }
        }));
    }

    for handle in handles {
        handle.join().expect("metadata writer thread should join");
    }

    let reopened = FileMetadataStore::new(&file_path);
    for thread_id in 0..thread_count {
        for seq in 0..writes_per_thread {
            let key = format!("conversation:c_{thread_id}_{seq}");
            let expected = format!("{{\"thread\":{thread_id},\"seq\":{seq}}}");
            assert_eq!(
                reopened.snapshot("tenant:100001", key.as_str()).as_deref(),
                Some(expected.as_str()),
                "cross-instance metadata updates should retain every unique key"
            );
        }
    }

    let _ = fs::remove_file(&file_path);
    let _ = fs::remove_file(file_path.with_extension("json.lock"));
}

#[test]
fn test_file_timeline_projection_store_preserves_cross_instance_entries() {
    let file_path = unique_store_file("timeline_projection_store_concurrent");
    let tenant_id = "100001";
    let timeline_scope = "c_concurrent";
    let thread_count = 4;
    let writes_per_thread = 32;
    let barrier = Arc::new(Barrier::new(thread_count));
    let mut handles = Vec::new();

    for thread_id in 0..thread_count {
        let barrier = barrier.clone();
        let file_path = file_path.clone();
        handles.push(thread::spawn(move || {
            let store = FileTimelineProjectionStore::new(&file_path);
            for seq in 0..writes_per_thread {
                let message_seq = (thread_id * writes_per_thread + seq + 1) as u64;
                let payload = format!("{{\"thread\":{thread_id},\"seq\":{seq}}}");
                barrier.wait();
                store
                    .upsert_timeline_entry(tenant_id, timeline_scope, message_seq, payload.as_str())
                    .expect("cross-instance timeline upsert should succeed");
            }
        }));
    }

    for handle in handles {
        handle.join().expect("timeline writer thread should join");
    }

    let reopened = FileTimelineProjectionStore::new(&file_path);
    let entries = reopened.entries(tenant_id, timeline_scope);
    assert_eq!(
        entries.len(),
        thread_count * writes_per_thread,
        "cross-instance timeline upserts should retain every unique message sequence"
    );
    for thread_id in 0..thread_count {
        for seq in 0..writes_per_thread {
            let message_seq = (thread_id * writes_per_thread + seq + 1) as u64;
            let payload = format!("{{\"thread\":{thread_id},\"seq\":{seq}}}");
            assert!(
                entries.iter().any(|(stored_seq, stored_payload)| {
                    *stored_seq == message_seq && stored_payload == &payload
                }),
                "missing timeline projection entry for message_seq={message_seq}"
            );
        }
    }

    let _ = fs::remove_file(&file_path);
    let _ = fs::remove_file(file_path.with_extension("json.lock"));
}

#[test]
fn test_validate_metadata_store_file_rejects_array_shape() {
    let file_path = unique_store_file("metadata_store_invalid");
    fs::write(&file_path, b"[]").expect("metadata store file should be written");

    let error = validate_metadata_store_file(&file_path)
        .expect_err("array-shaped metadata store should be rejected");
    assert!(matches!(error, ContractError::Unavailable(_)));
    let message = match error {
        ContractError::Unavailable(message) => message,
        other => panic!("unexpected error variant: {other:?}"),
    };
    assert!(message.contains("failed to parse metadata store"));

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_timeline_projection_store_upserts_by_sequence_across_reopen() {
    let file_path = unique_store_file("timeline_projection_store");
    let store = FileTimelineProjectionStore::new(&file_path);

    store
        .upsert_timeline_entry("100001", "c_demo", 1, "{\"summary\":\"first\"}")
        .expect("first timeline upsert should succeed");
    store
        .upsert_timeline_entry("100001", "c_demo", 2, "{\"summary\":\"second\"}")
        .expect("second timeline upsert should succeed");
    store
        .upsert_timeline_entry("100001", "c_demo", 2, "{\"summary\":\"second-v2\"}")
        .expect("idempotent timeline upsert should succeed");

    let reopened = FileTimelineProjectionStore::new(&file_path);
    assert_eq!(
        reopened.entries("100001", "c_demo"),
        vec![
            (1, "{\"summary\":\"first\"}".to_string()),
            (2, "{\"summary\":\"second-v2\"}".to_string()),
        ]
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_timeline_projection_store_isolates_same_scope_across_tenants() {
    let file_path = unique_store_file("timeline_projection_store_tenant_scope");
    let store = FileTimelineProjectionStore::new(&file_path);

    store
        .upsert_timeline_entry("t_alpha", "c_shared", 1, "{\"summary\":\"alpha\"}")
        .expect("alpha tenant timeline upsert should succeed");
    store
        .upsert_timeline_entry("t_beta", "c_shared", 1, "{\"summary\":\"beta\"}")
        .expect("beta tenant timeline upsert should succeed");

    let reopened = FileTimelineProjectionStore::new(&file_path);
    assert_eq!(
        reopened.entries("t_alpha", "c_shared"),
        vec![(1, "{\"summary\":\"alpha\"}".to_string())]
    );
    assert_eq!(
        reopened.entries("t_beta", "c_shared"),
        vec![(1, "{\"summary\":\"beta\"}".to_string())]
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_metadata_store_batches_snapshot_updates_across_reopen() {
    let file_path = unique_store_file("metadata_store_batch");
    let store = FileMetadataStore::new(&file_path);

    store
        .put_snapshots(&[
            MetadataSnapshotRecord {
                scope: "tenant:100001".into(),
                key: "conversation:c_demo".into(),
                value: "{\"state\":\"draft\"}".into(),
            },
            MetadataSnapshotRecord {
                scope: "tenant:100001".into(),
                key: "profile:1".into(),
                value: "{\"name\":\"demo\"}".into(),
            },
            MetadataSnapshotRecord {
                scope: "tenant:100001".into(),
                key: "conversation:c_demo".into(),
                value: "{\"state\":\"ready\"}".into(),
            },
        ])
        .expect("batched metadata snapshots should succeed");

    let reopened = FileMetadataStore::new(&file_path);
    assert_eq!(
        reopened
            .snapshot("tenant:100001", "conversation:c_demo")
            .as_deref(),
        Some("{\"state\":\"ready\"}")
    );
    assert_eq!(
        reopened
            .snapshot("tenant:100001", "profile:1")
            .as_deref(),
        Some("{\"name\":\"demo\"}")
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_timeline_projection_store_batches_multiple_scopes_across_reopen() {
    let file_path = unique_store_file("timeline_projection_store_batch");
    let store = FileTimelineProjectionStore::new(&file_path);

    store
        .upsert_timeline_batches(&[
            TimelineProjectionBatch {
                tenant_id: "100001".into(),
                timeline_scope: "c_demo".into(),
                records: vec![
                    TimelineProjectionRecord {
                        message_seq: 1,
                        payload: "{\"summary\":\"first\"}".into(),
                    },
                    TimelineProjectionRecord {
                        message_seq: 2,
                        payload: "{\"summary\":\"second\"}".into(),
                    },
                ],
            },
            TimelineProjectionBatch {
                tenant_id: "100001".into(),
                timeline_scope: "client-route-sync:1:d_demo".into(),
                records: vec![TimelineProjectionRecord {
                    message_seq: 9,
                    payload: "{\"syncSeq\":9}".into(),
                }],
            },
            TimelineProjectionBatch {
                tenant_id: "100001".into(),
                timeline_scope: "c_demo".into(),
                records: vec![TimelineProjectionRecord {
                    message_seq: 2,
                    payload: "{\"summary\":\"second-v2\"}".into(),
                }],
            },
        ])
        .expect("batched timeline upserts should succeed");

    let reopened = FileTimelineProjectionStore::new(&file_path);
    assert_eq!(
        reopened.entries("100001", "c_demo"),
        vec![
            (1, "{\"summary\":\"first\"}".to_string()),
            (2, "{\"summary\":\"second-v2\"}".to_string()),
        ]
    );
    assert_eq!(
        reopened.entries("100001", "client-route-sync:1:d_demo"),
        vec![(9, "{\"syncSeq\":9}".to_string())]
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_validate_timeline_projection_store_file_rejects_array_shape() {
    let file_path = unique_store_file("timeline_projection_store_invalid");
    fs::write(&file_path, b"[]").expect("timeline projection store file should be written");

    let error = validate_timeline_projection_store_file(&file_path)
        .expect_err("array-shaped timeline projection store should be rejected");
    assert!(matches!(error, ContractError::Unavailable(_)));
    let message = match error {
        ContractError::Unavailable(message) => message,
        other => panic!("unexpected error variant: {other:?}"),
    };
    assert!(message.contains("failed to parse timeline projection store"));

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_storage_domain_snapshot_store_persists_latest_snapshot_across_reopen() {
    let file_path = unique_store_file("storage_snapshot_store");
    let store = FileStorageDomainSnapshotStore::new(&file_path);

    store
        .save_snapshot(object_storage_snapshot("object-storage-aws"))
        .expect("first storage snapshot should succeed");
    store
        .save_snapshot(
            object_storage_snapshot("object-storage-google").with_secret(
                StorageSecretRecord::new_global(
                    "object-storage-google",
                    StorageCredentialMode::ServiceAccountJson,
                    "{\"serviceAccountJson\":{\"client_email\":\"storage@sdkwork.local\"}}",
                )
                .with_secret_fingerprint("fp-object-storage-google"),
            ),
        )
        .expect("second storage snapshot should succeed");

    let reopened = FileStorageDomainSnapshotStore::new(&file_path);
    let snapshot = reopened
        .load_snapshot("object-storage")
        .expect("storage snapshot load should succeed")
        .expect("storage snapshot should exist");

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

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_storage_domain_snapshot_store_isolates_domains_across_reopen() {
    let file_path = unique_store_file("storage_snapshot_store_domain_isolation");
    let store = FileStorageDomainSnapshotStore::new(&file_path);

    store
        .save_snapshot(object_storage_snapshot("object-storage-aws"))
        .expect("object storage snapshot should succeed");
    store
        .save_snapshot(
            StorageDomainSnapshot::new(StorageCatalog {
                domain: "chat-archive".into(),
                provider_schemas: Vec::new(),
            })
            .with_binding(StorageBindingRecord::new_global("archive-provider"))
            .with_config(StorageConfigRecord::new_global("archive-provider")),
        )
        .expect("archive snapshot should succeed");

    let reopened = FileStorageDomainSnapshotStore::new(&file_path);
    let object_storage = reopened
        .load_snapshot("object-storage")
        .expect("object storage snapshot load should succeed")
        .expect("object storage snapshot should exist");
    let chat_archive = reopened
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

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_validate_storage_domain_snapshot_store_file_rejects_array_shape() {
    let file_path = unique_store_file("storage_snapshot_store_invalid");
    fs::write(&file_path, b"[]").expect("storage snapshot store file should be written");

    let error = validate_storage_domain_snapshot_store_file(&file_path)
        .expect_err("array-shaped storage snapshot store should be rejected");
    assert!(matches!(error, ContractError::Unavailable(_)));
    let message = match error {
        ContractError::Unavailable(message) => message,
        other => panic!("unexpected error variant: {other:?}"),
    };
    assert!(message.contains("failed to parse storage domain snapshot store"));

    let _ = fs::remove_file(file_path);
}
