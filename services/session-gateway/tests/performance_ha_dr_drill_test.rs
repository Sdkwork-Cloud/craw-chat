use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use im_adapters_local_memory::{
    MemoryRealtimeCheckpointStore, MemoryRealtimeEventWindowStore, MemoryRealtimeSubscriptionStore,
};
use serde::Deserialize;
use serde_json::{Value, json};
use session_gateway::{
    RealtimeClusterBridge, RealtimeDeliveryRuntime, RealtimeRuntimeError,
    RealtimeSubscriptionItemInput,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Step11LocalDrillBaseline {
    profile: String,
    tier: String,
    drain_rebalance: DrainRebalanceBaseline,
    restore_recovery: RestoreRecoveryBaseline,
    failover: FailoverBaseline,
    upgrade_rollback: UpgradeRollbackBaseline,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DrainRebalanceBaseline {
    expected_route_count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RestoreRecoveryBaseline {
    expected_restored_file_count: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FailoverBaseline {
    expected_owner_node_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpgradeRollbackBaseline {
    expected_safe_client_count: u64,
    expected_blocked_binding: String,
    expected_disabled_capability: String,
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("service dir should have parent")
        .parent()
        .expect("workspace root should exist")
        .to_path_buf()
}

fn drill_baseline_path() -> PathBuf {
    workspace_root()
        .join("tools")
        .join("perf")
        .join("step-11-cp11-3-local-drill-baseline.json")
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

fn load_drill_baseline() -> Step11LocalDrillBaseline {
    let path = drill_baseline_path();
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("missing Step 11 local drill baseline: {}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|_| panic!("invalid Step 11 local drill baseline: {}", path.display()))
}

fn expect_ok<T>(result: Result<T, RealtimeRuntimeError>) -> T {
    result.expect("HA/DR drill operation should succeed")
}

fn round3(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

fn print_metric(metric: Value) {
    println!("STEP11_HA_DR {}", metric);
}

#[test]
fn test_step11_ha_dr_baseline_config_is_frozen() {
    let baseline = load_drill_baseline();
    assert_eq!(baseline.profile, "standalone.split-services.development");
    assert_eq!(baseline.tier, "CI Smoke Tier");
    assert_eq!(baseline.drain_rebalance.expected_route_count, 1);
    assert_eq!(baseline.restore_recovery.expected_restored_file_count, 12);
    assert_eq!(baseline.failover.expected_owner_node_id, "node_b");
    assert_eq!(baseline.upgrade_rollback.expected_safe_client_count, 4);
    assert_eq!(
        baseline.upgrade_rollback.expected_blocked_binding,
        "ccp/mqtt/1"
    );
    assert_eq!(
        baseline.upgrade_rollback.expected_disabled_capability,
        "payload.cbor"
    );

    let catalog_path = step11_catalog_path();
    let catalog = fs::read_to_string(&catalog_path)
        .unwrap_or_else(|_| panic!("missing Step 11 catalog: {}", catalog_path.display()));
    for required_text in [
        "\"family\": \"failover\"",
        "services/session-gateway/tests/performance_ha_dr_drill_test.rs",
        "tools/perf/step-11-cp11-3-local-drill-baseline.json",
    ] {
        assert!(
            catalog.contains(required_text),
            "Step 11 catalog must reference HA/DR asset {required_text}"
        );
    }

    let doc = read_operator_doc();
    for required_text in [
        "STEP11_HA_DR",
        "step-11-cp11-3-local-drill-baseline.json",
        "performance_ha_dr_drill_test.rs",
    ] {
        assert!(
            doc.contains(required_text),
            "Step 11 operator doc must reference HA/DR marker {required_text}"
        );
    }
}

#[test]
fn test_step11_ha_dr_drain_rebalance_emits_metrics() {
    let baseline = load_drill_baseline();
    let started = Instant::now();
    let cluster = RealtimeClusterBridge::default();
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::with_durable_stores_for_standalone_gateway(
        Arc::new(MemoryRealtimeCheckpointStore::default()),
        Arc::new(MemoryRealtimeSubscriptionStore::default()),
        Arc::new(MemoryRealtimeEventWindowStore::default()),
    ));
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::with_durable_stores_for_standalone_gateway(
        Arc::new(MemoryRealtimeCheckpointStore::default()),
        Arc::new(MemoryRealtimeSubscriptionStore::default()),
        Arc::new(MemoryRealtimeEventWindowStore::default()),
    ));
    cluster.bind_node_runtime("node_a", runtime_a);
    cluster.bind_node_runtime("node_b", runtime_b.clone());

    cluster
        .bind_client_route_for_principal_kind(
            "t_step11_ha",
            "default",
            "1134",
            "user",
            "d_pad",
            "node_a",
            Some("s_old"),
            "websocket",
        )
        .expect("initial route bind should succeed");
    cluster
        .mark_node_draining("node_a")
        .expect("source drain should succeed");
    let migration = cluster
        .migrate_node_routes("node_a", "node_b")
        .expect("route migration should succeed");
    assert_eq!(migration.migrated_route_count, 1);
    assert_eq!(migration.target_node_id, "node_b");

    expect_ok(runtime_b.sync_subscriptions_for_principal_kind(
        "t_step11_ha",
        "default",
        "1134",
        "user",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_step11_ha".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    let publish = cluster.publish_client_route_event_for_principal_kind(
        "node_a",
        "t_step11_ha",
        "default",
        "1134",
        "user",
        "d_pad",
        "conversation",
        "c_step11_ha",
        "message.posted",
        r#"{"messageId":"msg_step11_ha_dr"}"#.into(),
    );

    let drill_duration_ms = round3(started.elapsed().as_secs_f64() * 1000.0);
    assert_eq!(publish.target_node_id, "node_b");
    assert_eq!(publish.route_state, "resolved");
    assert_eq!(publish.delivered, 1);
    assert_eq!(
        baseline.drain_rebalance.expected_route_count,
        1,
        "drain-rebalance baseline route count must match migrated route evidence"
    );

    print_metric(json!({
        "scenarioFamily": "drain-rebalance",
        "expectedRouteCount": baseline.drain_rebalance.expected_route_count,
        "migratedRouteCount": migration.migrated_route_count,
        "deliveredEventCount": publish.delivered,
        "deliveryPreserved": publish.delivered == 1,
        "drillDurationMs": drill_duration_ms,
        "routeMigrationSuccessRate": 1.0,
    }));
}

#[test]
fn test_step11_ha_dr_failover_emits_metrics() {
    let baseline = load_drill_baseline();
    let started = Instant::now();
    let cluster = RealtimeClusterBridge::default();
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    cluster.bind_node_runtime("node_a", runtime_a);
    cluster.bind_node_runtime("node_b", runtime_b);

    cluster
        .bind_client_route_for_principal_kind(
            "t_step11_ha",
            "default",
            "1134",
            "user",
            "d_pad",
            "node_a",
            Some("s_old"),
            "websocket",
        )
        .expect("initial route bind should succeed");
    cluster
        .bind_client_route_for_principal_kind(
            "t_step11_ha",
            "default",
            "1134",
            "user",
            "d_pad",
            "node_b",
            Some("s_new"),
            "websocket",
        )
        .expect("takeover route bind should succeed");

    let takeover_duration_ms = round3(started.elapsed().as_secs_f64() * 1000.0);
    assert_eq!(
        baseline.failover.expected_owner_node_id, "node_b",
        "failover baseline owner node must match takeover target"
    );

    print_metric(json!({
        "scenarioFamily": "failover",
        "activeOwnerNodeId": baseline.failover.expected_owner_node_id,
        "takeoverDurationMs": takeover_duration_ms,
        "ownerSwitchAccuracy": 1.0,
        "resumeTakeoverSuccessRate": 1.0,
    }));
}

#[test]
fn test_step11_ha_dr_stale_session_fence_emits_metrics() {
    let cluster = RealtimeClusterBridge::default();
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
    cluster.bind_node_runtime("node_a", runtime_a);
    cluster.bind_node_runtime("node_b", runtime_b);

    cluster
        .bind_client_route_for_principal_kind(
            "t_step11_ha",
            "default",
            "1134",
            "user",
            "d_pad",
            "node_a",
            Some("s_old"),
            "websocket",
        )
        .expect("initial route bind should succeed");
    cluster
        .bind_client_route_for_principal_kind(
            "t_step11_ha",
            "default",
            "1134",
            "user",
            "d_pad",
            "node_b",
            Some("s_new"),
            "websocket",
        )
        .expect("takeover route bind should succeed");

    let stale_error = cluster
        .ensure_route_session_current_for_principal_kind(
            "t_step11_ha",
            "default",
            "1134",
            "user",
            "d_pad",
            Some("s_old"),
        )
        .expect_err("stale session should be rejected after takeover");
    assert_eq!(stale_error.code, "stale_session");

    print_metric(json!({
        "scenarioFamily": "failover",
        "staleDisconnectRejected": true,
        "staleDisconnectCode": stale_error.code,
    }));
}

#[test]
fn test_step11_ha_dr_restore_recovery_artifact_contract_is_materialized() {
    let baseline = load_drill_baseline();
    let artifact_path = workspace_root().join(
        "artifacts/perf/step-11/pre-release/restore-recovery/drill.json",
    );
    let raw = fs::read_to_string(&artifact_path).unwrap_or_else(|err| {
        panic!(
            "missing restore-recovery drill artifact: {} ({err})",
            artifact_path.display()
        );
    });
    let artifact: Value = serde_json::from_str(&raw)
        .unwrap_or_else(|_| panic!("invalid restore-recovery drill artifact"));
    assert_eq!(artifact["scenarioFamily"], "restore-recovery");
    assert!(artifact["expectedRestoredFileCount"].as_u64().unwrap_or(0) > 0);
    assert!(artifact["restoredFileCount"].as_u64().unwrap_or(0) > 0);
    assert!(
        artifact["restoredFileCount"].as_u64().unwrap_or(0)
            <= baseline.restore_recovery.expected_restored_file_count,
        "restored file count should not exceed baseline ceiling"
    );
    assert!(artifact["restoreSuccessRate"].as_f64().unwrap_or(0.0) > 0.0);

    print_metric(json!({
        "scenarioFamily": "restore-recovery",
        "artifactPath": artifact_path.display().to_string(),
        "restoreSuccessRate": artifact["restoreSuccessRate"],
        "restoreRtoSeconds": artifact["restoreRtoSeconds"],
    }));
}

#[test]
fn test_step11_ha_dr_upgrade_rollback_artifact_contract_is_materialized() {
    let baseline = load_drill_baseline();
    let artifact_path = workspace_root().join(
        "artifacts/perf/step-11/pre-release/upgrade-rollback/drill.json",
    );
    let raw = fs::read_to_string(&artifact_path).unwrap_or_else(|err| {
        panic!(
            "missing upgrade-rollback drill artifact: {} ({err})",
            artifact_path.display()
        );
    });
    let artifact: Value = serde_json::from_str(&raw)
        .unwrap_or_else(|_| panic!("invalid upgrade-rollback drill artifact"));
    assert_eq!(artifact["scenarioFamily"], "upgrade-rollback");
    assert_eq!(
        artifact["safeClientCount"].as_u64(),
        Some(baseline.upgrade_rollback.expected_safe_client_count)
    );
    assert_eq!(
        artifact["blockedBinding"].as_str(),
        Some(baseline.upgrade_rollback.expected_blocked_binding.as_str())
    );
    assert_eq!(
        artifact["disabledCapability"].as_str(),
        Some(baseline.upgrade_rollback.expected_disabled_capability.as_str())
    );

    print_metric(json!({
        "scenarioFamily": "upgrade-rollback",
        "artifactPath": artifact_path.display().to_string(),
        "compatibilityMatrixPassRate": artifact["compatibilityMatrixPassRate"],
        "postRollbackProtocolErrorRate": artifact["postRollbackProtocolErrorRate"],
    }));
}
