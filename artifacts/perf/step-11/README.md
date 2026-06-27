# Step 11 tier artifact roots

- scope: `Pre-Release Tier` and `Capacity Tier`
- current state:
  - `Pre-Release Tier = evidence_collected_gate_blocked`
  - `Capacity Tier = template_only_pending_execution`
- machine-readable schema: `artifacts/perf/step-11/schemas/step-11-tier-evidence-index.schema.json`
- frozen roots:
  - `artifacts/perf/step-11/pre-release`
  - `artifacts/perf/step-11/capacity`
- co-located evidence indexes:
  - `artifacts/perf/step-11/pre-release/pre-release-tier-evidence-index.json`
  - `artifacts/perf/step-11/capacity/capacity-tier-evidence-index.json`
- source templates:
  - `tools/perf/step-11-pre-release-tier-gate.json`
  - `tools/perf/step-11-capacity-tier-gate.json`
- boundary: `Pre-Release Tier` now materializes collected `connection/metrics.json`, `message/metrics.json`, `stream/metrics.json`, `drain-rebalance/drill.json`, `failover/drill.json`, `restore-recovery/drill.json`, and `upgrade-rollback/drill.json` artifacts from published CP11-2 and CP11-3 local evidence. `Pre-Release Tier` remains `evidence_collected_gate_blocked`, not full gate sign-off, and all `Capacity Tier` slots are still pending.
- CP11-6 supplemental boundary: `artifacts/perf/step-11/pre-release/im-websocket-e2e/metrics.json` standardizes the WebSocket E2E supplemental pre-release artifact path from CP11-5 `STEP11_WEBSOCKET_E2E`; it is not full Pre-Release Tier sign-off and remains `supplement_collected_gate_blocked_pending_real_pre_release_run` until a real pre-release topology run is collected.
