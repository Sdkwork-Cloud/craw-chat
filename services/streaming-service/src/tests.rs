use std::collections::BTreeMap;

use im_app_context::AppContext;
use im_domain_core::message::Sender;
use im_domain_core::stream::{
    StreamDurabilityClass, StreamFrame, StreamSession, StreamSessionState,
};
use sdkwork_im_contract_stream::{StreamStateRecord, StreamStateStore};

use crate::helpers::{stream_append_request_key, stream_open_request_key};
use crate::state::{RuntimeMemoryStreamStateStore, StreamingRuntime};

#[test]
fn test_ensure_stream_state_recovers_from_poisoned_sessions_lock() {
    let runtime = StreamingRuntime::default();
    let _ = std::panic::catch_unwind(|| {
        let _guard = runtime.sessions.lock().expect("stream runtime should lock");
        panic!("poison stream runtime sessions lock");
    });

    runtime
        .ensure_stream_state("100001", "st_poison")
        .expect("poisoned sessions lock should be recovered");
}

#[test]
fn test_runtime_memory_state_store_load_recovers_from_poisoned_lock() {
    let store = RuntimeMemoryStreamStateStore::default();
    let _ = std::panic::catch_unwind(|| {
        let _guard = store.states.lock().expect("stream state store should lock");
        panic!("poison stream state store lock");
    });

    let restored = store
        .load_state("100001", "st_poison")
        .expect("poisoned state store lock should be recovered");
    assert!(restored.is_none());
}

#[test]
fn test_runtime_memory_state_store_rejects_stale_cursor_and_frame_regression() {
    let store = RuntimeMemoryStreamStateStore::default();
    store
        .save_state(test_stream_state_record(
            StreamSessionState::Completed,
            3,
            Some(2),
            Some(3),
            vec![1, 2, 3],
            "2026-05-06T00:00:03.000Z",
        ))
        .expect("current stream state save should succeed");
    store
        .save_state(test_stream_state_record(
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
}

#[test]
fn test_stream_state_store_scope_key_is_segment_safe() {
    let store = RuntimeMemoryStreamStateStore::default();
    store
        .save_state(test_stream_state_record_with_identity(
            "tenant:a",
            "b",
            "2026-05-06T00:00:01.000Z",
        ))
        .expect("first stream state should save");
    store
        .save_state(test_stream_state_record_with_identity(
            "tenant",
            "a:b",
            "2026-05-06T00:00:02.000Z",
        ))
        .expect("second stream state should save");

    assert_eq!(
        store
            .load_state("tenant:a", "b")
            .expect("first stream load should succeed")
            .expect("first stream should not be overwritten")
            .stream_id,
        "b"
    );
    assert_eq!(
        store
            .load_state("tenant", "a:b")
            .expect("second stream load should succeed")
            .expect("second stream should be retrievable")
            .stream_id,
        "a:b"
    );
}

#[test]
fn test_stream_request_keys_are_segment_safe() {
    let first = AppContext {
        tenant_id: "tenant:a".into(),
        organization_id: "0".to_owned(),
        user_id: "b".into(),
        session_id: None,
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        permission_scope: Default::default(),
        actor_id: "b".into(),
        actor_kind: "user".into(),
        device_id: None,
    };
    let second = AppContext {
        tenant_id: "tenant".into(),
        organization_id: "0".to_owned(),
        user_id: "b".into(),
        session_id: None,
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        permission_scope: Default::default(),
        actor_id: "b".into(),
        actor_kind: "a:user".into(),
        device_id: None,
    };

    assert_ne!(
        stream_open_request_key(&first, "stream"),
        stream_open_request_key(&second, "stream")
    );
    assert_ne!(
        stream_append_request_key(&first, "stream", 7),
        stream_append_request_key(&second, "stream", 7)
    );
}

fn test_stream_state_record(
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
        frames: frame_seqs.into_iter().map(test_stream_frame).collect(),
        updated_at: updated_at.into(),
    }
}

fn test_stream_state_record_with_identity(
    tenant_id: &str,
    stream_id: &str,
    updated_at: &str,
) -> StreamStateRecord {
    let mut record = test_stream_state_record(
        StreamSessionState::Active,
        1,
        None,
        None,
        vec![1],
        updated_at,
    );
    record.tenant_id = tenant_id.into();
    record.stream_id = stream_id.into();
    record.session.tenant_id = tenant_id.into();
    record.session.stream_id = stream_id.into();
    for frame in &mut record.frames {
        frame.tenant_id = tenant_id.into();
        frame.stream_id = stream_id.into();
    }
    record
}

fn test_stream_frame(frame_seq: u64) -> StreamFrame {
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
