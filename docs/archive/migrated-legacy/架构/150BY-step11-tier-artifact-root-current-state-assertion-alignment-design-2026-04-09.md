# Step 11 Tier Artifact Root Current-State Assertion Alignment Design

## Decision

- Treat the shared `artifacts/perf/step-11/pre-release/README.md` as a current-state contract, not a historical snapshot.
- Keep historical backwrite docs free to describe their own phase-specific state.

## State Model

- previous shared assertion state: `evidence_partially_collected`
- current shared truth: `evidence_collected_gate_blocked`
- collected slots: `7`
- pending slots: `0`
- `Capacity Tier` state: `template_only_pending_execution`

## Contract

- shared README must contain:
  - `evidence_collected_gate_blocked`
  - `connection_metrics`
  - `message_metrics`
  - `stream_metrics`
  - `connection/metrics.json`
  - `message/metrics.json`
  - `stream/metrics.json`
  - `drain-rebalance/drill.json`
  - `failover/drill.json`
  - `restore-recovery/drill.json`
  - `upgrade-rollback/drill.json`
  - `all seven truthful local artifacts`
- shared README must not contain `pending_collection`.

## Boundary

- This is a regression-contract correction only.
- It does not reclassify historical docs that intentionally recorded earlier partial collection states.
