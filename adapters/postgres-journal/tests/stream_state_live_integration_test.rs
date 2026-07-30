//! Live PostgreSQL coverage for organization isolation, optimistic concurrency,
//! and bounded keyset frame reads.

use std::collections::BTreeMap;
use std::sync::{Arc, Barrier};
use std::time::{SystemTime, UNIX_EPOCH};

use im_adapters_postgres_journal::{PostgresJournalConfig, PostgresStreamStateStore};
use im_domain_core::message::Sender;
use im_domain_core::stream::{
    StreamDurabilityClass, StreamFrame, StreamSession, StreamSessionState,
};
use im_platform_contracts::{
    StreamAppendOutcome, StreamCreateOutcome, StreamScope, StreamSessionRecord, StreamStateStore,
};

fn suffix() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock must be after epoch")
        .as_nanos()
        .to_string()
}

fn session_record(scope: StreamScope) -> StreamSessionRecord {
    let now = chrono::Utc::now().to_rfc3339();
    StreamSessionRecord {
        session: StreamSession {
            tenant_id: scope.tenant_id.clone(),
            stream_id: scope.stream_id.clone(),
            owner_principal_id: "1".into(),
            owner_principal_kind: "user".into(),
            stream_type: "custom.delta.text".into(),
            scope_kind: "request".into(),
            scope_id: "request-live".into(),
            durability_class: StreamDurabilityClass::DurableSession,
            ordering_scope: "stream".into(),
            schema_ref: Some("custom.delta.text.v1".into()),
            state: StreamSessionState::Opened,
            last_frame_seq: 0,
            last_checkpoint_seq: None,
            result_message_id: None,
            complete_frame_seq: None,
            abort_frame_seq: None,
            abort_reason: None,
            opened_at: now.clone(),
            closed_at: None,
            expires_at: None,
        },
        scope,
        version: 1,
        updated_at: now,
    }
}

fn frame(scope: &StreamScope, sequence: u64, payload: &str) -> StreamFrame {
    StreamFrame {
        tenant_id: scope.tenant_id.clone(),
        stream_id: scope.stream_id.clone(),
        stream_type: "custom.delta.text".into(),
        scope_kind: "request".into(),
        scope_id: "request-live".into(),
        frame_seq: sequence,
        frame_type: "delta".into(),
        schema_ref: Some("custom.delta.text.v1".into()),
        encoding: "json".into(),
        payload: payload.into(),
        sender: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("device-live".into()),
            session_id: Some("session-live".into()),
            metadata: BTreeMap::new(),
        },
        attributes: BTreeMap::new(),
        occurred_at: chrono::Utc::now().to_rfc3339(),
    }
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live PostgreSQL via SDKWORK_DATABASE_URL"]
async fn stream_store_is_scoped_atomic_and_bounded() {
    let database_url =
        std::env::var("SDKWORK_DATABASE_URL").expect("SDKWORK_DATABASE_URL must be set");
    sdkwork_im_database_pool::bootstrap_im_process_database_pools_from_env()
        .await
        .expect("shared IM database pools should bootstrap");
    let pool = PostgresJournalConfig::new(database_url)
        .connect_pool()
        .expect("PostgreSQL pool should connect");
    let store = Arc::new(PostgresStreamStateStore::from_pool(pool));
    let suffix = suffix();
    let tenant_id = format!("9{}", &suffix[suffix.len().saturating_sub(15)..]);
    let scope_a = StreamScope::new(&tenant_id, "100001", format!("stream-{suffix}"));
    let scope_b = StreamScope::new(&tenant_id, "100002", format!("stream-{suffix}"));

    for scope in [&scope_a, &scope_b] {
        assert!(matches!(
            store
                .create_session(session_record(scope.clone()), 10)
                .unwrap(),
            StreamCreateOutcome::Applied(_)
        ));
    }
    let mut next = store.load_session(&scope_a).unwrap().unwrap();
    let expected = next.version;
    next.version += 1;
    next.session.last_frame_seq = 1;
    next.session.state = StreamSessionState::Active;
    assert!(matches!(
        store
            .append_frame(expected, next, frame(&scope_a, 1, "first"))
            .unwrap(),
        StreamAppendOutcome::Applied { .. }
    ));
    assert!(store.list_frames_after(&scope_b, 0, 2).unwrap().is_empty());

    let current = store.load_session(&scope_a).unwrap().unwrap();
    let barrier = Arc::new(Barrier::new(3));
    let handles = ["left", "right"].map(|payload| {
        let store = store.clone();
        let barrier = barrier.clone();
        let scope = scope_a.clone();
        let mut next = current.clone();
        next.version += 1;
        next.session.last_frame_seq = 2;
        std::thread::spawn(move || {
            barrier.wait();
            store.append_frame(current.version, next, frame(&scope, 2, payload))
        })
    });
    barrier.wait();
    let outcomes = handles.map(|handle| handle.join().unwrap().unwrap());
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| matches!(outcome, StreamAppendOutcome::Applied { .. }))
            .count(),
        1
    );
    assert_eq!(store.list_frames_after(&scope_a, 0, 1).unwrap().len(), 1);
    assert_eq!(store.list_frames_after(&scope_a, 1, 2).unwrap().len(), 1);

    store.clear_stream(&scope_a).unwrap();
    store.clear_stream(&scope_b).unwrap();
}
