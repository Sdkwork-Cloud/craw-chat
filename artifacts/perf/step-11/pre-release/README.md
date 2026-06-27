# Step 11 Pre-Release Tier artifact root

- artifactRoot: `artifacts/perf/step-11/pre-release`
- gateTemplate: `tools/perf/step-11-pre-release-tier-gate.json`
- evidenceIndex: `pre-release-tier-evidence-index.json`
- schema: `../schemas/step-11-tier-evidence-index.schema.json`
- checksumManifestPath: `checksum-manifest.txt`
- artifactFileListPath: `artifact-file-list.txt`
- current gate state: `evidence_collected_gate_blocked`
- supplemental CP11-6 state: `supplement_collected_gate_blocked_pending_real_pre_release_run`
- collected slots: `connection_metrics`, `message_metrics`, `stream_metrics`, `drain_rebalance_drill`, `failover_drill`, `restore_recovery_drill`, `upgrade_rollback_drill`
- current evidence slot states: `collected`
- default naming rule: `artifactPath = artifactRoot + "/" + suggestedRelativePath`
- collected artifacts: `connection/metrics.json`, `message/metrics.json`, `stream/metrics.json`, `drain-rebalance/drill.json`, `failover/drill.json`, `restore-recovery/drill.json`, `upgrade-rollback/drill.json`
- CP11-6 supplemental artifact: `im-websocket-e2e/metrics.json`
- collected runIds: `step11-cp11-2-local-connection-2026-04-08-doc-capture`, `step11-cp11-2-local-message-2026-04-08-doc-capture`, `step11-cp11-2-local-stream-2026-04-08-doc-capture`, `step11-cp11-3-local-drain-rebalance-2026-04-08-doc-capture`, `step11-cp11-3-local-failover-2026-04-08-doc-capture`, `step11-cp11-3-local-restore-recovery-2026-04-08-doc-capture`, `step11-cp11-3-local-upgrade-rollback-2026-04-08-doc-capture`
- boundary: this root now carries all seven truthful local artifacts. `Pre-Release Tier` is `evidence_collected_gate_blocked`, not full gate sign-off, because the artifacts are backfilled from published CI Smoke Tier / self-hosted.split-services.development evidence while `Capacity Tier` is now `evidence_collected_gate_passed` from the same local doc-capture backfill.
- CP11-6 boundary: `im-websocket-e2e/metrics.json` standardizes the WebSocket E2E supplemental pre-release evidence path from CP11-5 `STEP11_WEBSOCKET_E2E`. It covers real TCP WebSocket long connections, subscription fanout, disconnect recovery, message window trimming, cross-device checkpoint, consistency compensation, and cluster route handoff, but it is not full Pre-Release Tier sign-off until a real pre-release topology run replaces the CI Smoke source.
