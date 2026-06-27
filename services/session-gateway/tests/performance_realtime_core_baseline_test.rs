use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use im_adapters_local_memory::{
    MemoryRealtimeCheckpointStore, MemoryRealtimeEventWindowStore, MemoryRealtimeSubscriptionStore,
};
use im_platform_contracts::{ContractError, RealtimeCheckpointRecord, RealtimeCheckpointStore};
use serde::Deserialize;
use serde_json::{Value, json};
use session_gateway::{
    RealtimeClusterBridge, RealtimeDeliveryRuntime, RealtimeRuntimeError,
    RealtimeSubscriptionItemInput,
};

const TENANT_ID: &str = "t_step11_realtime";
const PRINCIPAL_ID: &str = "1130";
const PRINCIPAL_KIND: &str = "user";
const CONVERSATION_ID: &str = "c_step11_realtime";
const EVENT_TYPE: &str = "message.posted";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Step11ImRealtimeCoreBaseline {
    profile: String,
    tier: String,
    realtime_core: RealtimeCoreBaseline,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RealtimeCoreBaseline {
    message_count: usize,
    subscribed_device_count: usize,
    expected_fanout_per_message: usize,
    expected_capacity_trimmed_event_count: u64,
    ack_checkpoint_count: usize,
    min_fanout_success_permille: u64,
    max_publish_p95_ms: f64,
    max_publish_p99_ms: f64,
    max_ack_p95_ms: f64,
    max_restore_duration_ms: f64,
    max_compensation_rollback_ms: f64,
    max_cluster_handoff_ms: f64,
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("service dir should have parent")
        .parent()
        .expect("workspace root should exist")
        .to_path_buf()
}

fn realtime_core_baseline_path() -> PathBuf {
    workspace_root()
        .join("tools")
        .join("perf")
        .join("step-11-cp11-4-im-realtime-core-baseline.json")
}

fn step11_catalog_path() -> PathBuf {
    workspace_root()
        .join("tools")
        .join("perf")
        .join("step-11-scenario-catalog.json")
}

fn operator_doc_path() -> PathBuf {
    workspace_root()
        .join("docs")
        .join("部署")
        .join("性能与灾备演练场景.md")
}

fn read_operator_doc() -> String {
    let doc_path = operator_doc_path();
    let doc_bytes = fs::read(&doc_path).unwrap_or_else(|err| {
        panic!(
            "missing Step 11 operator doc: {} ({err})",
            doc_path.display()
        );
    });
    String::from_utf8_lossy(&doc_bytes).into_owned()
}

fn load_realtime_core_baseline() -> Step11ImRealtimeCoreBaseline {
    let path = realtime_core_baseline_path();
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("missing IM realtime core baseline: {}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|_| panic!("invalid IM realtime core baseline: {}", path.display()))
}

fn expect_ok<T>(result: Result<T, RealtimeRuntimeError>) -> T {
    result.expect("realtime core performance gate operation should succeed")
}

fn round3(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

fn percentile_ms(samples: &[f64], percentile: f64) -> f64 {
    assert!(!samples.is_empty(), "percentile samples must not be empty");
    let mut ordered = samples.to_vec();
    ordered.sort_by(|left, right| {
        left.partial_cmp(right)
            .expect("latency samples should be comparable")
    });
    let rank = ((ordered.len() as f64) * percentile).ceil() as usize;
    ordered[rank.saturating_sub(1).min(ordered.len() - 1)]
}

fn print_metric(metric: Value) {
    println!("STEP11_REALTIME_CORE {}", metric);
}

fn subscription(scope_id: &str) -> Vec<RealtimeSubscriptionItemInput> {
    vec![RealtimeSubscriptionItemInput {
        scope_type: "conversation".into(),
        scope_id: scope_id.into(),
        event_types: vec![EVENT_TYPE.into()],
    }]
}

fn message_payload(index: usize) -> String {
    format!(r#"{{"messageId":"msg_step11_realtime_{index:04}","index":{index}}}"#)
}

fn publish_message(
    runtime: &RealtimeDeliveryRuntime,
    index: usize,
    candidate_device_ids: Vec<String>,
) -> usize {
    expect_ok(runtime.publish_scope_event_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "conversation",
        CONVERSATION_ID,
        EVENT_TYPE,
        message_payload(index),
        candidate_device_ids,
    ))
}

#[test]
fn test_step11_im_realtime_core_baseline_config_is_frozen() {
    let baseline = load_realtime_core_baseline();
    assert_eq!(baseline.profile, "session-gateway");
    assert_eq!(baseline.tier, "CI Smoke Tier");
    assert!(baseline.realtime_core.message_count > 1_000);
    assert_eq!(baseline.realtime_core.subscribed_device_count, 3);
    assert_eq!(baseline.realtime_core.expected_fanout_per_message, 3);
    assert_eq!(baseline.realtime_core.ack_checkpoint_count, 3);
    assert!(
        baseline.realtime_core.expected_capacity_trimmed_event_count > 0,
        "baseline must force bounded-window capacity trimming"
    );
    assert_eq!(baseline.realtime_core.min_fanout_success_permille, 1_000);
    assert!(baseline.realtime_core.max_publish_p95_ms > 0.0);
    assert!(baseline.realtime_core.max_publish_p99_ms > 0.0);
    assert!(baseline.realtime_core.max_ack_p95_ms > 0.0);
    assert!(baseline.realtime_core.max_restore_duration_ms > 0.0);
    assert!(baseline.realtime_core.max_compensation_rollback_ms > 0.0);
    assert!(baseline.realtime_core.max_cluster_handoff_ms > 0.0);

    let catalog_path = step11_catalog_path();
    let catalog = fs::read_to_string(&catalog_path)
        .unwrap_or_else(|_| panic!("missing Step 11 catalog: {}", catalog_path.display()));
    for required_text in [
        "\"family\": \"im-realtime-core\"",
        "services/session-gateway/tests/performance_realtime_core_baseline_test.rs",
        "services/session-gateway/tests/commercial_realtime_acceptance_test.rs",
        "tools/perf/step-11-cp11-4-im-realtime-core-baseline.json",
    ] {
        assert!(
            catalog.contains(required_text),
            "Step 11 catalog must reference IM realtime core asset {required_text}"
        );
    }

    let doc = read_operator_doc();
    for required_text in [
        "`im-realtime-core`",
        "STEP11_REALTIME_CORE",
        "step-11-cp11-4-im-realtime-core-baseline.json",
        "performance_realtime_core_baseline_test.rs",
    ] {
        assert!(
            doc.contains(required_text),
            "Step 11 operator doc must reference IM realtime core marker {required_text}"
        );
    }
}

#[test]
fn test_step11_im_realtime_core_quant_gate_emits_thresholded_metrics() {
    let baseline = load_realtime_core_baseline();
    let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    let subscription_store = Arc::new(MemoryRealtimeSubscriptionStore::default());
    let event_window_store = Arc::new(MemoryRealtimeEventWindowStore::default());
    let runtime = RealtimeDeliveryRuntime::with_durable_stores_for_standalone_gateway(
        checkpoint_store.clone(),
        subscription_store.clone(),
        event_window_store.clone(),
    );

    for device_id in ["d_primary", "d_mobile", "d_tablet"] {
        expect_ok(runtime.sync_subscriptions_for_principal_kind(
            TENANT_ID,
        "default",
        PRINCIPAL_ID,
            PRINCIPAL_KIND,
            device_id,
            subscription(CONVERSATION_ID),
        ));
    }
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_other_conversation",
        subscription("c_other"),
    ));

    let candidate_devices = vec![
        "d_primary".to_owned(),
        "d_mobile".to_owned(),
        "d_tablet".to_owned(),
        "d_other_conversation".to_owned(),
        "d_missing".to_owned(),
    ];
    let mut publish_latencies_ms = Vec::with_capacity(baseline.realtime_core.message_count);
    let mut delivered_event_count = 0usize;
    let publish_total_started = Instant::now();
    for index in 1..=baseline.realtime_core.message_count {
        let started = Instant::now();
        let delivered = publish_message(&runtime, index, candidate_devices.clone());
        publish_latencies_ms.push(started.elapsed().as_secs_f64() * 1000.0);
        assert_eq!(
            delivered,
            baseline.realtime_core.expected_fanout_per_message
        );
        delivered_event_count += delivered;
    }
    let publish_total_duration_ms = publish_total_started.elapsed().as_secs_f64() * 1000.0;

    let expected_delivered_event_count =
        baseline.realtime_core.message_count * baseline.realtime_core.expected_fanout_per_message;
    let fanout_success_permille =
        ((delivered_event_count * 1_000) / expected_delivered_event_count) as u64;
    assert_eq!(delivered_event_count, expected_delivered_event_count);
    assert!(fanout_success_permille >= baseline.realtime_core.min_fanout_success_permille);

    let capacity_trimmed_event_count = ["d_primary", "d_mobile", "d_tablet"]
        .into_iter()
        .map(|device_id| {
            checkpoint_store
                .checkpoint(TENANT_ID, "default", PRINCIPAL_KIND, PRINCIPAL_ID, device_id)
                .expect("trimmed checkpoint should persist")
                .capacity_trimmed_event_count
        })
        .sum::<u64>();
    assert_eq!(
        capacity_trimmed_event_count,
        baseline.realtime_core.expected_capacity_trimmed_event_count
    );

    for device_id in ["d_primary", "d_mobile", "d_tablet"] {
        let window = expect_ok(runtime.list_events_for_principal_kind(
            TENANT_ID,
        "default",
        PRINCIPAL_ID,
            PRINCIPAL_KIND,
            device_id,
            0,
            1_000,
        ));
        assert_eq!(window.items.len(), 1_000);
        assert_eq!(window.items[0].realtime_seq, 51);
        assert_eq!(
            window.items[999].realtime_seq,
            baseline.realtime_core.message_count as u64
        );
    }

    let mut ack_latencies_ms = Vec::with_capacity(baseline.realtime_core.ack_checkpoint_count);
    for (device_id, ack_seq) in [
        ("d_primary", 800_u64),
        ("d_mobile", 700_u64),
        ("d_tablet", 600_u64),
    ] {
        let started = Instant::now();
        let ack = expect_ok(runtime.ack_events_for_principal_kind(
            TENANT_ID,
        "default",
        PRINCIPAL_ID,
            PRINCIPAL_KIND,
            device_id,
            ack_seq,
        ));
        ack_latencies_ms.push(started.elapsed().as_secs_f64() * 1000.0);
        assert_eq!(ack.acked_through_seq, ack_seq);
        assert_eq!(ack.trimmed_through_seq, ack_seq);
    }
    assert_eq!(
        ack_latencies_ms.len(),
        baseline.realtime_core.ack_checkpoint_count
    );

    let restore_started = Instant::now();
    let rebuilt_runtime = RealtimeDeliveryRuntime::with_durable_stores_for_standalone_gateway(
        checkpoint_store.clone(),
        subscription_store.clone(),
        event_window_store.clone(),
    );
    let restored_primary = expect_ok(rebuilt_runtime.list_events_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_primary",
        0,
        1_000,
    ));
    let restore_duration_ms = restore_started.elapsed().as_secs_f64() * 1000.0;
    assert_eq!(restored_primary.acked_through_seq, 800);
    assert_eq!(restored_primary.items[0].realtime_seq, 801);

    let failing_checkpoint_store = Arc::new(ToggleRealtimeCheckpointStore::with_seed(
        checkpoint_store.clone(),
    ));
    let failure_runtime = RealtimeDeliveryRuntime::with_durable_stores_and_scope_access_policy(
        failing_checkpoint_store.clone(),
        subscription_store,
        event_window_store,
        Arc::new(session_gateway::StandaloneRealtimeScopeAccessPolicy),
    );
    failing_checkpoint_store.fail_saves();
    let compensation_started = Instant::now();
    let error = failure_runtime
        .ack_events_for_principal_kind(TENANT_ID, "default", PRINCIPAL_ID, PRINCIPAL_KIND, "d_mobile", 900)
        .expect_err("failed checkpoint persistence should reject ack");
    let compensation_rollback_ms = compensation_started.elapsed().as_secs_f64() * 1000.0;
    assert_eq!(error.code, "checkpoint_store_unavailable");
    let mobile_after_failed_ack = expect_ok(failure_runtime.list_events_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_mobile",
        0,
        1_000,
    ));
    assert_eq!(mobile_after_failed_ack.acked_through_seq, 700);
    assert_eq!(mobile_after_failed_ack.trimmed_through_seq, 700);
    assert_eq!(mobile_after_failed_ack.items[0].realtime_seq, 701);

    let handoff_duration_ms = run_cluster_handoff_drill_ms();
    let publish_p50_ms = percentile_ms(&publish_latencies_ms, 0.50);
    let publish_p95_ms = percentile_ms(&publish_latencies_ms, 0.95);
    let publish_p99_ms = percentile_ms(&publish_latencies_ms, 0.99);
    let ack_p95_ms = percentile_ms(&ack_latencies_ms, 0.95);
    let publish_tps = baseline.realtime_core.message_count as f64
        / (publish_total_duration_ms / 1000.0).max(f64::EPSILON);

    assert!(
        publish_p95_ms <= baseline.realtime_core.max_publish_p95_ms,
        "publish p95 {}ms exceeded baseline {}ms",
        round3(publish_p95_ms),
        baseline.realtime_core.max_publish_p95_ms
    );
    assert!(
        publish_p99_ms <= baseline.realtime_core.max_publish_p99_ms,
        "publish p99 {}ms exceeded baseline {}ms",
        round3(publish_p99_ms),
        baseline.realtime_core.max_publish_p99_ms
    );
    assert!(
        ack_p95_ms <= baseline.realtime_core.max_ack_p95_ms,
        "ack p95 {}ms exceeded baseline {}ms",
        round3(ack_p95_ms),
        baseline.realtime_core.max_ack_p95_ms
    );
    assert!(
        restore_duration_ms <= baseline.realtime_core.max_restore_duration_ms,
        "restore duration {}ms exceeded baseline {}ms",
        round3(restore_duration_ms),
        baseline.realtime_core.max_restore_duration_ms
    );
    assert!(
        compensation_rollback_ms <= baseline.realtime_core.max_compensation_rollback_ms,
        "compensation rollback duration {}ms exceeded baseline {}ms",
        round3(compensation_rollback_ms),
        baseline.realtime_core.max_compensation_rollback_ms
    );
    assert!(
        handoff_duration_ms <= baseline.realtime_core.max_cluster_handoff_ms,
        "cluster handoff duration {}ms exceeded baseline {}ms",
        round3(handoff_duration_ms),
        baseline.realtime_core.max_cluster_handoff_ms
    );

    print_metric(json!({
        "scenario": "im-realtime-core",
        "profile": baseline.profile,
        "tier": baseline.tier,
        "messageCount": baseline.realtime_core.message_count,
        "subscribedDeviceCount": baseline.realtime_core.subscribed_device_count,
        "expectedDeliveredEventCount": expected_delivered_event_count,
        "deliveredEventCount": delivered_event_count,
        "fanoutSuccessPermille": fanout_success_permille,
        "capacityTrimmedEventCount": capacity_trimmed_event_count,
        "publishTotalDurationMs": round3(publish_total_duration_ms),
        "publishP50Ms": round3(publish_p50_ms),
        "publishP95Ms": round3(publish_p95_ms),
        "publishP99Ms": round3(publish_p99_ms),
        "publishTps": round3(publish_tps),
        "ackP95Ms": round3(ack_p95_ms),
        "restoreDurationMs": round3(restore_duration_ms),
        "compensationRollbackMs": round3(compensation_rollback_ms),
        "clusterHandoffMs": round3(handoff_duration_ms),
        "thresholds": {
            "maxPublishP95Ms": baseline.realtime_core.max_publish_p95_ms,
            "maxPublishP99Ms": baseline.realtime_core.max_publish_p99_ms,
            "maxAckP95Ms": baseline.realtime_core.max_ack_p95_ms,
            "maxRestoreDurationMs": baseline.realtime_core.max_restore_duration_ms,
            "maxCompensationRollbackMs": baseline.realtime_core.max_compensation_rollback_ms,
            "maxClusterHandoffMs": baseline.realtime_core.max_cluster_handoff_ms
        }
    }));
}

fn run_cluster_handoff_drill_ms() -> f64 {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    cluster.bind_node_runtime("node_a", runtime_a.clone());
    cluster.bind_node_runtime("node_b", runtime_b.clone());

    expect_ok(runtime_a.sync_subscriptions_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_handoff",
        subscription(CONVERSATION_ID),
    ));
    cluster
        .bind_client_route_for_principal_kind(
            TENANT_ID,
            "default",
            PRINCIPAL_ID,
            PRINCIPAL_KIND,
            "d_handoff",
            "node_a",
            Some("s_handoff"),
            "websocket",
        )
        .expect("initial route should bind");

    for index in 1..=24 {
        assert_eq!(
            expect_ok(runtime_a.publish_scope_event_for_principal_kind(
                TENANT_ID,
        "default",
        PRINCIPAL_ID,
                PRINCIPAL_KIND,
                "conversation",
                CONVERSATION_ID,
                EVENT_TYPE,
                message_payload(index),
                vec!["d_handoff".into()],
            )),
            1
        );
    }
    expect_ok(runtime_a.ack_events_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_handoff",
        12,
    ));

    let handoff_started = Instant::now();
    cluster
        .mark_node_draining("node_a")
        .expect("source node should drain");
    cluster
        .migrate_node_routes("node_a", "node_b")
        .expect("draining node should migrate route");
    let routed = cluster.publish_client_route_event_for_principal_kind(
        "node_a",
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_handoff",
        "conversation",
        CONVERSATION_ID,
        EVENT_TYPE,
        message_payload(25),
    );
    let duration_ms = handoff_started.elapsed().as_secs_f64() * 1000.0;
    assert_eq!(routed.route_state, "resolved");
    assert_eq!(routed.target_node_id, "node_b");
    assert_eq!(routed.delivered, 1);

    let target_window = expect_ok(runtime_b.list_events_for_principal_kind(
        TENANT_ID,
        "default",
        PRINCIPAL_ID,
        PRINCIPAL_KIND,
        "d_handoff",
        12,
        100,
    ));
    assert_eq!(target_window.items.len(), 13);
    assert_eq!(target_window.items[12].realtime_seq, 25);
    duration_ms
}

#[derive(Clone)]
struct ToggleRealtimeCheckpointStore {
    seed: Arc<MemoryRealtimeCheckpointStore>,
    fail_saves: Arc<AtomicBool>,
}

impl ToggleRealtimeCheckpointStore {
    fn with_seed(seed: Arc<MemoryRealtimeCheckpointStore>) -> Self {
        Self {
            seed,
            fail_saves: Arc::new(AtomicBool::new(false)),
        }
    }

    fn fail_saves(&self) {
        self.fail_saves.store(true, Ordering::SeqCst);
    }
}

impl RealtimeCheckpointStore for ToggleRealtimeCheckpointStore {
    fn load_checkpoint(
        &self,
        tenant_id: &str,
        _organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        self.seed
            .load_checkpoint(tenant_id, "default", principal_kind, principal_id, device_id)
    }

    fn save_checkpoints(
        &self,
        records: Vec<RealtimeCheckpointRecord>,
    ) -> Result<(), ContractError> {
        if self.fail_saves.load(Ordering::SeqCst) {
            return Err(ContractError::Unavailable(
                "synthetic realtime core checkpoint save failure".into(),
            ));
        }
        self.seed.save_checkpoints(records)
    }
}
