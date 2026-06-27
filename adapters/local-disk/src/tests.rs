use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use super::*;
use im_domain_core::automation::{AutomationExecution, AutomationExecutionState};
use im_domain_core::notification::{NotificationStatus, NotificationTask};
use im_domain_core::presence::{PresenceClientView, PresenceStatus};
use im_domain_core::realtime::{RealtimeEvent, RealtimeSubscription};
use im_domain_core::stream::{
    StreamDurabilityClass, StreamFrame, StreamSession, StreamSessionState,
};
use im_platform_contracts::{
    AutomationExecutionRecord, AutomationExecutionStore, CommitEnvelope, CommitJournal,
    ContractError, NotificationTaskRecord, NotificationTaskStore, PresenceStateRecord,
    PresenceStateStore, RealtimeCheckpointRecord, RealtimeCheckpointStore,
    RealtimeDisconnectFenceRecord, RealtimeDisconnectFenceStore, RealtimeEventWindowRecord,
    RealtimeEventWindowStore, RealtimeSubscriptionRecord, RealtimeSubscriptionStore,
    StreamStateRecord, StreamStateStore,
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
        sender: im_domain_core::message::Sender {
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

fn unique_store_file() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("sdkwork_im_disconnect_fence_store_{unique}.json"))
}

fn unique_checkpoint_store_file() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "sdkwork_im_realtime_checkpoint_store_{unique}.json"
    ))
}

fn unique_subscription_store_file() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "sdkwork_im_realtime_subscription_store_{unique}.json"
    ))
}

fn unique_event_window_store_file() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "sdkwork_im_realtime_event_window_store_{unique}.json"
    ))
}

fn unique_commit_journal_file() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("sdkwork_im_commit_journal_{unique}.json"))
}

fn unique_stream_state_store_file() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("sdkwork_im_stream_state_store_{unique}.json"))
}

fn unique_notification_task_store_file() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("sdkwork_im_notification_task_store_{unique}.json"))
}

fn unique_automation_execution_store_file() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "sdkwork_im_automation_execution_store_{unique}.json"
    ))
}

fn unique_presence_state_store_file() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("sdkwork_im_presence_state_store_{unique}.json"))
}

fn pending_temp_file(file_path: &Path) -> PathBuf {
    file_path.with_extension("json.tmp")
}

fn commit_journal_json_lines(events: &[CommitEnvelope]) -> String {
    let mut lines = String::new();
    for event in events {
        lines.push_str(
            serde_json::to_string(event)
                .expect("commit journal event should serialize")
                .as_str(),
        );
        lines.push('\n');
    }
    lines
}

#[test]
fn test_file_commit_journal_persists_across_reopen() {
    let file_path = unique_commit_journal_file();
    let journal = FileCommitJournal::new("dev-file-journal", &file_path);
    journal
        .append(CommitEnvelope::minimal(
            "evt_demo_1",
            "100001",
            "conversation.created",
            "conversation",
            "c_demo",
            0,
        ))
        .expect("append should succeed");
    journal
        .append(CommitEnvelope::minimal(
            "evt_demo_2",
            "100001",
            "message.posted",
            "conversation",
            "c_demo",
            1,
        ))
        .expect("append should succeed");

    let reopened = FileCommitJournal::new("dev-file-journal", &file_path);
    let recorded = reopened.recorded().expect("recorded should succeed");
    assert_eq!(recorded.len(), 2);
    assert_eq!(recorded[0].event_id, "evt_demo_1");
    assert_eq!(recorded[1].event_id, "evt_demo_2");

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_commit_journal_writes_append_only_json_lines() {
    let file_path = unique_commit_journal_file();
    let journal = FileCommitJournal::new("dev-file-journal", &file_path);
    journal
        .append(CommitEnvelope::minimal(
            "evt_demo_1",
            "100001",
            "conversation.created",
            "conversation",
            "c_demo",
            0,
        ))
        .expect("first append should succeed");
    journal
        .append(CommitEnvelope::minimal(
            "evt_demo_2",
            "100001",
            "message.posted",
            "conversation",
            "c_demo",
            1,
        ))
        .expect("second append should succeed");

    let content = fs::read_to_string(&file_path).expect("journal file should be readable");
    assert!(
        !content.trim_start().starts_with('['),
        "commit journal must be append-only JSON Lines, not a rewritten JSON array"
    );

    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines.len(), 2);
    let first: CommitEnvelope =
        serde_json::from_str(lines[0]).expect("first JSONL record should parse");
    let second: CommitEnvelope =
        serde_json::from_str(lines[1]).expect("second JSONL record should parse");
    assert_eq!(first.event_id, "evt_demo_1");
    assert_eq!(second.event_id, "evt_demo_2");

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_commit_journal_append_path_does_not_read_full_journal() {
    let source = include_str!("journal.rs");
    let append_impl = source
        .split("fn append(&self, envelope: CommitEnvelope)")
        .nth(1)
        .and_then(|tail| tail.split("fn append_batch(").next())
        .expect("append impl should be discoverable");

    assert!(
        !append_impl.contains("read_events_unlocked"),
        "append must not read the full journal before writing a single event"
    );
    assert!(
        !append_impl.contains("write_events_unlocked"),
        "append must not rewrite the full journal file"
    );
}

#[test]
fn test_read_commit_journal_file_restores_minimal_events() {
    let file_path = unique_commit_journal_file();
    fs::write(
        &file_path,
        commit_journal_json_lines(&[CommitEnvelope::minimal(
            "evt_demo_1",
            "100001",
            "conversation.created",
            "conversation",
            "c_demo",
            0,
        )]),
    )
    .expect("journal file should be written");

    let restored = read_commit_journal_file(&file_path).expect("journal should parse");
    assert_eq!(restored.len(), 1);
    assert_eq!(restored[0].event_id, "evt_demo_1");

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_commit_journal_recovers_pending_tmp_file_on_reopen() {
    let file_path = unique_commit_journal_file();
    let temp_path = pending_temp_file(&file_path);
    fs::write(
        &temp_path,
        commit_journal_json_lines(&[CommitEnvelope::minimal(
            "evt_demo_tmp",
            "100001",
            "conversation.created",
            "conversation",
            "c_demo",
            0,
        )]),
    )
    .expect("pending temp journal file should be written");

    let reopened = FileCommitJournal::new("dev-file-journal", &file_path);
    let recorded = reopened.recorded().expect("recorded should succeed");
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].event_id, "evt_demo_tmp");
    assert!(
        !temp_path.exists(),
        "pending temp journal file should be promoted into the live file"
    );
    assert!(file_path.exists(), "live journal file should be restored");

    let _ = fs::remove_file(file_path);
    let _ = fs::remove_file(temp_path);
}

#[test]
fn test_file_commit_journal_prefers_live_file_over_stale_tmp_file() {
    let file_path = unique_commit_journal_file();
    let temp_path = pending_temp_file(&file_path);
    fs::write(
        &file_path,
        commit_journal_json_lines(&[CommitEnvelope::minimal(
            "evt_demo_live",
            "100001",
            "conversation.created",
            "conversation",
            "c_demo",
            0,
        )]),
    )
    .expect("live journal file should be written");
    fs::write(
        &temp_path,
        commit_journal_json_lines(&[CommitEnvelope::minimal(
            "evt_demo_tmp",
            "100001",
            "message.posted",
            "conversation",
            "c_demo",
            1,
        )]),
    )
    .expect("stale temp journal file should be written");

    let reopened = FileCommitJournal::new("dev-file-journal", &file_path);
    let recorded = reopened.recorded().expect("recorded should succeed");
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].event_id, "evt_demo_live");
    assert!(
        !temp_path.exists(),
        "stale temp journal file should be removed once the live file wins"
    );

    let _ = fs::remove_file(file_path);
    let _ = fs::remove_file(temp_path);
}

#[test]
fn test_validate_commit_journal_file_rejects_json_array_shape() {
    let file_path = unique_commit_journal_file();
    fs::write(&file_path, b"[]").expect("commit journal file should be written");

    let error = validate_commit_journal_file(&file_path)
        .expect_err("array-shaped commit journal should be rejected");
    assert!(matches!(error, ContractError::Unavailable(_)));
    let message = match error {
        ContractError::Unavailable(message) => message,
        other => panic!("unexpected error variant: {other:?}"),
    };
    assert!(message.contains("failed to parse commit journal"));

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_validate_checkpoint_store_file_rejects_array_shape() {
    let file_path = unique_checkpoint_store_file();
    fs::write(&file_path, b"[]").expect("checkpoint file should be written");

    let error = validate_realtime_checkpoint_store_file(&file_path)
        .expect_err("array-shaped checkpoint store should be rejected");
    assert!(matches!(error, ContractError::Unavailable(_)));
    let message = match error {
        ContractError::Unavailable(message) => message,
        other => panic!("unexpected error variant: {other:?}"),
    };
    assert!(message.contains("failed to parse realtime checkpoint store"));

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_checkpoint_store_recovers_pending_tmp_file_on_reopen() {
    let file_path = unique_checkpoint_store_file();
    let temp_path = pending_temp_file(&file_path);
    let pending_payload = BTreeMap::from([(
        "6:100001|7:default|4:user|1:1|5:d_pad".to_string(),
        RealtimeCheckpointRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            latest_realtime_seq: 9,
            acked_through_seq: 7,
            trimmed_through_seq: 7,
            capacity_trimmed_event_count: 0,
            capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            updated_at: "2026-04-06T00:00:00.000Z".into(),
        },
    )]);
    fs::write(
        &temp_path,
        serde_json::to_vec_pretty(&pending_payload)
            .expect("pending temp checkpoint payload should serialize"),
    )
    .expect("pending temp checkpoint file should be written");

    let reopened = FileRealtimeCheckpointStore::new(&file_path);
    let restored = reopened
        .load_checkpoint("100001", "default", "user", "1", "d_pad")
        .expect("load should succeed")
        .expect("checkpoint should exist");
    assert_eq!(restored.latest_realtime_seq, 9);
    assert_eq!(restored.acked_through_seq, 7);
    assert!(
        !temp_path.exists(),
        "pending temp checkpoint file should be promoted into the live file"
    );
    assert!(
        file_path.exists(),
        "live checkpoint file should be restored"
    );

    let _ = fs::remove_file(file_path);
    let _ = fs::remove_file(temp_path);
}

#[test]
fn test_file_checkpoint_store_persists_across_reopen() {
    let file_path = unique_checkpoint_store_file();
    let store = FileRealtimeCheckpointStore::new(&file_path);
    store
        .save_checkpoint(RealtimeCheckpointRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            latest_realtime_seq: 7,
            acked_through_seq: 5,
            trimmed_through_seq: 5,
            capacity_trimmed_event_count: 0,
            capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            updated_at: "2026-04-06T00:00:00.000Z".into(),
        })
        .expect("save should succeed");

    let reopened = FileRealtimeCheckpointStore::new(&file_path);
    let restored = reopened
        .load_checkpoint("100001", "default", "user", "1", "d_pad")
        .expect("load should succeed")
        .expect("checkpoint should exist");
    assert_eq!(restored.latest_realtime_seq, 7);
    assert_eq!(restored.acked_through_seq, 5);
    assert_eq!(restored.trimmed_through_seq, 5);

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_realtime_event_window_store_persists_and_trims_across_reopen() {
    let file_path = unique_event_window_store_file();
    let store = FileRealtimeEventWindowStore::new(&file_path);
    store
        .save_window(RealtimeEventWindowRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            trimmed_through_seq: 0,
            capacity_trimmed_event_count: 0,
            capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            updated_at: "2026-04-06T00:00:00.000Z".into(),
            events: vec![RealtimeEvent {
                tenant_id: "100001".into(),
                principal_id: "1".into(),
                device_id: "d_pad".into(),
                realtime_seq: 1,
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_type: "message.posted".into(),
                delivery_class: "ephemeral".into(),
                payload: r#"{"messageId":"msg_demo"}"#.into(),
                occurred_at: "2026-04-06T00:00:00.000Z".into(),
            }],
        })
        .expect("event window save should succeed");

    let reopened = FileRealtimeEventWindowStore::new(&file_path);
    let restored = reopened
        .load_window("100001", "default", "user", "1", "d_pad")
        .expect("event window load should succeed")
        .expect("event window should exist");
    assert_eq!(restored.events.len(), 1);
    assert_eq!(restored.events[0].realtime_seq, 1);
    assert_eq!(restored.events[0].payload, r#"{"messageId":"msg_demo"}"#);

    reopened
        .trim_window("100001", "default", "user", "1", "d_pad", 1)
        .expect("event window trim should succeed");
    let trimmed = reopened
        .load_window("100001", "default", "user", "1", "d_pad")
        .expect("trimmed event window load should succeed")
        .expect("trimmed event window should exist");
    assert_eq!(trimmed.trimmed_through_seq, 1);
    assert!(trimmed.events.is_empty());

    assert!(
        reopened
            .clear_window("100001", "default", "user", "1", "d_pad")
            .expect("event window clear should succeed"),
        "existing event window should be cleared"
    );
    let reopened_after_clear = FileRealtimeEventWindowStore::new(&file_path);
    assert!(
        reopened_after_clear
            .load_window("100001", "default", "user", "1", "d_pad")
            .expect("cleared event window load should succeed")
            .is_none(),
        "cleared event window must not restore after reopen"
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_checkpoint_store_rejects_stale_regression_writes() {
    let file_path = unique_checkpoint_store_file();
    let store = FileRealtimeCheckpointStore::new(&file_path);
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

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_checkpoint_store_persists_checkpoint_batch_across_reopen() {
    let file_path = unique_checkpoint_store_file();
    let store = FileRealtimeCheckpointStore::new(&file_path);
    store
        .save_checkpoints(vec![
            RealtimeCheckpointRecord {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                principal_kind: "user".into(),
                principal_id: "1".into(),
                device_id: "d_pad".into(),
                latest_realtime_seq: 7,
                acked_through_seq: 5,
                trimmed_through_seq: 5,
                capacity_trimmed_event_count: 0,
                capacity_trimmed_through_seq: 0,
                last_capacity_trimmed_at: None,
                updated_at: "2026-04-06T00:00:00.000Z".into(),
            },
            RealtimeCheckpointRecord {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                principal_kind: "user".into(),
                principal_id: "1".into(),
                device_id: "d_phone".into(),
                latest_realtime_seq: 8,
                acked_through_seq: 2,
                trimmed_through_seq: 2,
                capacity_trimmed_event_count: 0,
                capacity_trimmed_through_seq: 0,
                last_capacity_trimmed_at: None,
                updated_at: "2026-04-06T00:00:00.000Z".into(),
            },
        ])
        .expect("checkpoint batch save should succeed");

    let reopened = FileRealtimeCheckpointStore::new(&file_path);
    assert_eq!(
        reopened
            .load_checkpoint("100001", "default", "user", "1", "d_pad")
            .expect("pad checkpoint load should succeed")
            .expect("pad checkpoint should exist")
            .latest_realtime_seq,
        7
    );
    assert_eq!(
        reopened
            .load_checkpoint("100001", "default", "user", "1", "d_phone")
            .expect("phone checkpoint load should succeed")
            .expect("phone checkpoint should exist")
            .latest_realtime_seq,
        8
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_disconnect_fence_store_persists_and_clears_across_reopen() {
    let file_path = unique_store_file();
    let store = FileRealtimeDisconnectFenceStore::new(&file_path);
    store
        .save_fence(realtime_disconnect_fence_record(
            "1",
            "s_old",
            "node_a",
            "2026-04-06T00:00:00.000Z",
        ))
        .expect("save should succeed");

    let reopened = FileRealtimeDisconnectFenceStore::new(&file_path);
    let restored = reopened
        .load_fence("100001", "default", "user", "1", "d_pad")
        .expect("load should succeed")
        .expect("fence should exist");
    assert_eq!(restored.session_id.as_deref(), Some("s_old"));
    assert_eq!(restored.owner_node_id, "node_a");

    assert!(
        reopened
            .clear_fence("100001", "default", "user", "1", "d_pad")
            .expect("clear should succeed")
    );

    let reopened_after_clear = FileRealtimeDisconnectFenceStore::new(&file_path);
    assert!(
        reopened_after_clear
            .load_fence("100001", "default", "user", "1", "d_pad")
            .expect("load after clear should succeed")
            .is_none()
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_disconnect_fence_store_rejects_stale_regression_writes() {
    let file_path = unique_store_file();
    let store = FileRealtimeDisconnectFenceStore::new(&file_path);
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
        .load_fence("100001", "default", "user", "1", "d_pad")
        .expect("disconnect fence load should succeed")
        .expect("disconnect fence should be present");
    assert_eq!(fence.session_id.as_deref(), Some("s_new"));
    assert_eq!(fence.owner_node_id, "node_b");
    assert_eq!(fence.disconnected_at, "2026-05-06T00:00:02.000Z");

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_disconnect_fence_store_conditionally_clears_only_old_fence() {
    let file_path = unique_store_file();
    let store = FileRealtimeDisconnectFenceStore::new(&file_path);
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
        store
            .load_fence("100001", "default", "user", "1", "d_pad")
            .expect("disconnect fence load should succeed")
            .is_some(),
        "newer disconnect fence must not be deleted by an older resume cleanup"
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_disconnect_fence_store_compares_cutoff_by_rfc3339_instant() {
    let file_path = unique_store_file();
    let store = FileRealtimeDisconnectFenceStore::new(&file_path);
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
        store
            .load_fence("100001", "default", "user", "1", "d_pad")
            .expect("disconnect fence load should succeed")
            .is_some(),
        "fractional-second later disconnect fence must not be deleted by whole-second cutoff"
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_disconnect_fence_store_clears_only_exact_matching_fence() {
    let file_path = unique_store_file();
    let store = FileRealtimeDisconnectFenceStore::new(&file_path);
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
            .load_fence("100001", "default", "user", "1", "d_pad")
            .expect("disconnect fence load should succeed")
            .expect("disconnect fence should still exist"),
        current
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_subscription_store_persists_across_reopen() {
    let file_path = unique_subscription_store_file();
    let store = FileRealtimeSubscriptionStore::new(&file_path);
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
                event_types: vec!["message.posted".into()],
                subscribed_at: "2026-04-06T00:00:00.000Z".into(),
            }],
            synced_at: "2026-04-06T00:00:00.000Z".into(),
        })
        .expect("save should succeed");

    let reopened = FileRealtimeSubscriptionStore::new(&file_path);
    let restored = reopened
        .load_subscriptions("100001", "default", "user", "1", "d_pad")
        .expect("load should succeed")
        .expect("subscriptions should exist");
    assert_eq!(restored.items.len(), 1);
    assert_eq!(restored.items[0].scope_id, "c_demo");
    assert_eq!(restored.items[0].event_types, vec!["message.posted"]);
    assert_eq!(restored.synced_at, "2026-04-06T00:00:00.000Z");

    assert!(
        reopened
            .clear_subscriptions("100001", "default", "user", "1", "d_pad")
            .expect("clear should succeed")
    );

    let reopened_after_clear = FileRealtimeSubscriptionStore::new(&file_path);
    assert!(
        reopened_after_clear
            .load_subscriptions("100001", "default", "user", "1", "d_pad")
            .expect("load after clear should succeed")
            .is_none()
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_subscription_store_loads_matching_scope_event_candidates_across_reopen() {
    let file_path = unique_subscription_store_file();
    let store = FileRealtimeSubscriptionStore::new(&file_path);
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
                    subscribed_at: "2026-04-06T00:00:00.000Z".into(),
                }],
                synced_at: "2026-04-06T00:00:00.000Z".into(),
            })
            .expect("save should succeed");
    }

    let reopened = FileRealtimeSubscriptionStore::new(&file_path);
    let matches = reopened
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

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_stream_state_store_persists_across_reopen() {
    let file_path = unique_stream_state_store_file();
    let store = FileStreamStateStore::new(&file_path);
    store
        .save_state(StreamStateRecord {
            tenant_id: "100001".into(),
            stream_id: "st_demo".into(),
            session: im_domain_core::stream::StreamSession {
                tenant_id: "100001".into(),
                stream_id: "st_demo".into(),
                owner_principal_id: "1".into(),
                owner_principal_kind: "user".into(),
                stream_type: "custom.delta.text".into(),
                scope_kind: "request".into(),
                scope_id: "req_demo".into(),
                durability_class: im_domain_core::stream::StreamDurabilityClass::DurableSession,
                ordering_scope: "stream".into(),
                schema_ref: Some("custom.delta.text.v1".into()),
                state: im_domain_core::stream::StreamSessionState::Active,
                last_frame_seq: 1,
                last_checkpoint_seq: Some(1),
                result_message_id: None,
                complete_frame_seq: None,
                abort_frame_seq: None,
                abort_reason: None,
                opened_at: "2026-04-06T00:00:00.000Z".into(),
                closed_at: None,
                expires_at: None,
            },
            frames: vec![im_domain_core::stream::StreamFrame {
                tenant_id: "100001".into(),
                stream_id: "st_demo".into(),
                stream_type: "custom.delta.text".into(),
                scope_kind: "request".into(),
                scope_id: "req_demo".into(),
                frame_seq: 1,
                frame_type: "delta".into(),
                schema_ref: Some("custom.delta.text.v1".into()),
                encoding: "json".into(),
                payload: "{\"delta\":\"hello\"}".into(),
                sender: im_domain_core::message::Sender {
                    id: "1".into(),
                    kind: "user".into(),
                    member_id: None,
                    device_id: Some("d_demo".into()),
                    session_id: Some("s_demo".into()),
                    metadata: BTreeMap::new(),
                },
                attributes: BTreeMap::new(),
                occurred_at: "2026-04-06T00:00:00.000Z".into(),
            }],
            updated_at: "2026-04-06T00:00:00.000Z".into(),
        })
        .expect("save should succeed");

    let reopened = FileStreamStateStore::new(&file_path);
    let restored = reopened
        .load_state("100001", "st_demo")
        .expect("load should succeed")
        .expect("stream state should exist");
    assert_eq!(restored.session.last_frame_seq, 1);
    assert_eq!(restored.session.owner_principal_id, "1");
    assert_eq!(restored.session.owner_principal_kind, "user");
    assert_eq!(restored.frames.len(), 1);
    assert_eq!(restored.frames[0].frame_seq, 1);

    assert!(
        reopened
            .clear_state("100001", "st_demo")
            .expect("clear should succeed")
    );

    let reopened_after_clear = FileStreamStateStore::new(&file_path);
    assert!(
        reopened_after_clear
            .load_state("100001", "st_demo")
            .expect("load after clear should succeed")
            .is_none()
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_subscription_store_does_not_clear_newer_subscription() {
    let file_path = unique_subscription_store_file();
    let store = FileRealtimeSubscriptionStore::new(&file_path);
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

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_subscription_store_compares_cutoff_by_rfc3339_instant() {
    let file_path = unique_subscription_store_file();
    let store = FileRealtimeSubscriptionStore::new(&file_path);
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
            .load_subscriptions("100001", "default", "user", "1", "d_pad")
            .expect("subscription load should succeed")
            .is_some(),
        "fractional-second later subscription must not be deleted by whole-second cutoff"
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_stream_state_store_rejects_stale_cursor_and_frame_regression() {
    let file_path = unique_stream_state_store_file();
    let store = FileStreamStateStore::new(&file_path);
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
        .load_state("100001", "st_demo")
        .expect("stream state load should succeed")
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

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_notification_task_store_persists_across_reopen() {
    let file_path = unique_notification_task_store_file();
    let store = FileNotificationTaskStore::new(&file_path);
    store
        .save_task(NotificationTaskRecord {
            tenant_id: "100001".into(),
            notification_id: "ntf_demo".into(),
            task: im_domain_core::notification::NotificationTask {
                tenant_id: "100001".into(),
                notification_id: "ntf_demo".into(),
                source_event_id: "evt_demo".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "1".into(),
                recipient_kind: "user".into(),
                status: im_domain_core::notification::NotificationStatus::Dispatched,
                title: Some("hello".into()),
                body: Some("world".into()),
                payload: Some("{\"conversationId\":\"c_demo\"}".into()),
                requested_at: "2026-04-06T00:00:00.000Z".into(),
                dispatched_at: Some("2026-04-06T00:00:01.000Z".into()),
                failure_reason: None,
            },
            updated_at: "2026-04-06T00:00:01.000Z".into(),
        })
        .expect("save should succeed");

    let reopened = FileNotificationTaskStore::new(&file_path);
    let restored = reopened
        .load_task("100001", "ntf_demo")
        .expect("load should succeed")
        .expect("notification task should exist");
    assert_eq!(restored.task.notification_id, "ntf_demo");
    assert_eq!(restored.task.recipient_id, "1");
    assert_eq!(restored.task.recipient_kind, "user");

    let listed = reopened
        .list_tasks_for_recipient("100001", "user", "1")
        .expect("list should succeed");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].notification_id, "ntf_demo");

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_notification_task_store_lists_only_matching_recipient_kind() {
    let file_path = unique_notification_task_store_file();
    let store = FileNotificationTaskStore::new(&file_path);
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

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_notification_task_store_rejects_stale_status_regression_writes() {
    let file_path = unique_notification_task_store_file();
    let store = FileNotificationTaskStore::new(&file_path);
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

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_automation_execution_store_persists_across_reopen() {
    let file_path = unique_automation_execution_store_file();
    let store = FileAutomationExecutionStore::new(&file_path);
    store
        .save_execution(AutomationExecutionRecord {
            tenant_id: "100001".into(),
            principal_id: "1".into(),
            execution_id: "ae_demo".into(),
            execution: im_domain_core::automation::AutomationExecution {
                tenant_id: "100001".into(),
                principal_id: "1".into(),
                principal_kind: "user".into(),
                execution_id: "ae_demo".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some("{\"conversationId\":\"c_demo\"}".into()),
                output_payload: Some("{\"accepted\":true}".into()),
                state: im_domain_core::automation::AutomationExecutionState::Succeeded,
                retry_count: 0,
                requested_at: "2026-04-06T00:00:00.000Z".into(),
                completed_at: Some("2026-04-06T00:00:01.000Z".into()),
                failure_reason: None,
            },
            updated_at: "2026-04-06T00:00:01.000Z".into(),
        })
        .expect("save should succeed");

    let reopened = FileAutomationExecutionStore::new(&file_path);
    let restored = reopened
        .load_execution("100001", "user", "1", "ae_demo")
        .expect("load should succeed")
        .expect("automation execution should exist");
    assert_eq!(restored.execution.execution_id, "ae_demo");
    assert_eq!(restored.execution.principal_id, "1");
    assert_eq!(
        restored.execution.state,
        im_domain_core::automation::AutomationExecutionState::Succeeded
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_automation_execution_store_isolates_same_actor_id_across_principal_kind() {
    let file_path = unique_automation_execution_store_file();
    let store = FileAutomationExecutionStore::new(&file_path);
    for principal_kind in ["user", "system"] {
        store
            .save_execution(AutomationExecutionRecord {
                tenant_id: "100001".into(),
                principal_id: "1".into(),
                execution_id: "ae_kind_isolation".into(),
                execution: im_domain_core::automation::AutomationExecution {
                    tenant_id: "100001".into(),
                    principal_id: "1".into(),
                    principal_kind: principal_kind.into(),
                    execution_id: "ae_kind_isolation".into(),
                    trigger_type: "webhook.manual".into(),
                    target_kind: "workflow".into(),
                    target_ref: "wf_demo".into(),
                    input_payload: Some("{\"conversationId\":\"c_demo\"}".into()),
                    output_payload: Some("{\"accepted\":true}".into()),
                    state: im_domain_core::automation::AutomationExecutionState::Succeeded,
                    retry_count: 0,
                    requested_at: "2026-04-06T00:00:00.000Z".into(),
                    completed_at: Some("2026-04-06T00:00:01.000Z".into()),
                    failure_reason: None,
                },
                updated_at: "2026-04-06T00:00:01.000Z".into(),
            })
            .expect("save should succeed");
    }

    let reopened = FileAutomationExecutionStore::new(&file_path);
    let user_execution = reopened
        .load_execution("100001", "user", "1", "ae_kind_isolation")
        .expect("user execution load should succeed")
        .expect("user execution should exist");
    let system_execution = reopened
        .load_execution("100001", "system", "1", "ae_kind_isolation")
        .expect("system execution load should succeed")
        .expect("system execution should exist");
    assert_eq!(user_execution.execution.principal_kind, "user");
    assert_eq!(system_execution.execution.principal_kind, "system");

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_automation_execution_store_ignores_legacy_key_without_principal_kind() {
    let file_path = unique_automation_execution_store_file();
    let legacy_payload = BTreeMap::from([(
        "100001:1:ae_legacy".to_string(),
        AutomationExecutionRecord {
            tenant_id: "100001".into(),
            principal_id: "1".into(),
            execution_id: "ae_legacy".into(),
            execution: im_domain_core::automation::AutomationExecution {
                tenant_id: "100001".into(),
                principal_id: "1".into(),
                principal_kind: "system".into(),
                execution_id: "ae_legacy".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some("{\"conversationId\":\"c_demo\"}".into()),
                output_payload: Some("{\"accepted\":true}".into()),
                state: im_domain_core::automation::AutomationExecutionState::Succeeded,
                retry_count: 0,
                requested_at: "2026-04-06T00:00:00.000Z".into(),
                completed_at: Some("2026-04-06T00:00:01.000Z".into()),
                failure_reason: None,
            },
            updated_at: "2026-04-06T00:00:01.000Z".into(),
        },
    )]);
    fs::write(
        &file_path,
        serde_json::to_vec_pretty(&legacy_payload)
            .expect("legacy automation payload should serialize"),
    )
    .expect("legacy automation execution file should be written");

    let reopened = FileAutomationExecutionStore::new(&file_path);
    assert!(
        reopened
            .load_execution("100001", "system", "1", "ae_legacy")
            .expect("legacy execution load should succeed")
            .is_none(),
        "local disk automation execution store must not read principal-kind-less legacy keys"
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_automation_execution_store_rejects_stale_status_regression_writes() {
    let file_path = unique_automation_execution_store_file();
    let store = FileAutomationExecutionStore::new(&file_path);
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

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_presence_state_store_persists_across_reopen() {
    let file_path = unique_presence_state_store_file();
    let store = FilePresenceStateStore::new(&file_path);
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
                session_id: None,
                status: PresenceStatus::Offline,
                last_sync_seq: 7,
                last_resume_at: Some("2026-04-06T00:00:00.000Z".into()),
                last_seen_at: Some("2026-04-06T00:00:01.000Z".into()),
            },
            resume_required: true,
            updated_at: "2026-04-06T00:00:01.000Z".into(),
        })
        .expect("save should succeed");
    store
        .save_state(PresenceStateRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_phone".into(),
            presence: PresenceClientView {
                tenant_id: "100001".into(),
                principal_id: "1".into(),
                device_id: "d_phone".into(),
                platform: None,
                session_id: None,
                status: PresenceStatus::Offline,
                last_sync_seq: 0,
                last_resume_at: None,
                last_seen_at: None,
            },
            resume_required: false,
            updated_at: "2026-04-06T00:00:02.000Z".into(),
        })
        .expect("save should succeed");

    let reopened = FilePresenceStateStore::new(&file_path);
    let restored = reopened
        .load_state("100001", "default", "user", "1", "d_pad")
        .expect("load should succeed")
        .expect("presence state should exist");
    assert_eq!(restored.device_id, "d_pad");
    assert!(restored.resume_required);
    assert_eq!(restored.presence.last_sync_seq, 7);

    let listed = reopened
        .list_states_for_principal("100001", "default", "user", "1")
        .expect("list should succeed");
    assert_eq!(listed.len(), 2);
    assert!(listed.iter().any(|record| record.device_id == "d_pad"));
    assert!(listed.iter().any(|record| record.device_id == "d_phone"));

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_presence_state_store_lists_stale_online_devices_by_seen_at() {
    let file_path = unique_presence_state_store_file();
    let store = FilePresenceStateStore::new(&file_path);
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

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_presence_state_store_seen_at_cutoff_compares_rfc3339_by_instant() {
    let file_path = unique_presence_state_store_file();
    let store = FilePresenceStateStore::new(&file_path);
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

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_presence_state_store_conditionally_expires_only_stale_online_state() {
    let file_path = unique_presence_state_store_file();
    let store = FilePresenceStateStore::new(&file_path);
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

    let _ = fs::remove_file(file_path);
}
