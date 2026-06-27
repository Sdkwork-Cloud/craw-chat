# Step 11 Pre-Release Failover Collected Evidence Design

## Decision

- Promote one already published CP11-3 local failover result into the `Pre-Release Tier` artifact root.
- Do not fake any missing `connection`, `message`, `stream`, `restore-recovery`, `drain-rebalance`, or `upgrade-rollback` artifacts.

## State Model

- previous state: `template_only_pending_execution`
- new `Pre-Release Tier` state: `evidence_partially_collected`
- collected slot: `failover_drill`
- remaining slots: `pending_collection`
- `Capacity Tier` state: `template_only_pending_execution`

## Artifact Contract

- artifact path: `artifacts/perf/step-11/pre-release/failover/drill.json`
- collected metric snapshot: `takeoverDurationMs = 0.553`
- required fields preserved:
  - `runId`
  - `takeoverDurationMs`
  - `ownerSwitchAccuracy`
  - `resumeTakeoverSuccessRate`
- supporting fields:
  - `activeOwnerNodeId`
  - `staleDisconnectCode`
  - `sourceBaselinePath`
  - `sourceTestPath`
  - `sourceReviewId`

## Boundary

- This artifact is a truthful partial collection record derived from published local evidence.
- It does not upgrade the whole `Pre-Release Tier` to gate-ready.
- It does not change `Capacity Tier`.
