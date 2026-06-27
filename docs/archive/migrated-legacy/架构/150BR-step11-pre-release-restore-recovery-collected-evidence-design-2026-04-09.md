# Step 11 Pre-Release Restore-Recovery Collected Evidence Design

## Decision

- Promote one already published CP11-3 local `restore-recovery` result into the `Pre-Release Tier` artifact root.
- Do not fake any missing `connection`, `message`, `stream`, `drain-rebalance`, `failover`, or `upgrade-rollback` artifacts.

## State Model

- previous `Pre-Release Tier` state: `evidence_partially_collected`
- new `Pre-Release Tier` state: `evidence_partially_collected`
- newly collected slot: `restore_recovery_drill`
- total collected slots: `2`
- `Capacity Tier` state: `template_only_pending_execution`

## Artifact Contract

- artifact path: `artifacts/perf/step-11/pre-release/restore-recovery/drill.json`
- collected metric snapshot: `restoreDurationMs = 17.983`
- required fields preserved:
  - `runId`
  - `restoreSuccessRate`
  - `restoreRtoSeconds`
  - `previewDiffAccuracy`
- supporting fields:
  - `expectedRestoredFileCount`
  - `restoredFileCount`
  - `restoreStatus`
  - `sourceBaselinePath`
  - `sourceTestPath`
  - `sourceReviewId`

## Boundary

- `restoreRtoSeconds = 0.017983` is derived from the published `restoreDurationMs` evidence.
- This artifact is a truthful partial collection record, not full `Pre-Release Tier` sign-off.
- It does not change `Capacity Tier`.
