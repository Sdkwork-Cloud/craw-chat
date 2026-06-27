> Migrated from `docs/架构/150BS-step11-pre-release-drain-rebalance-collected-evidence-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 11 Pre-Release Drain-Rebalance Collected Evidence Design

## Decision

- Promote one already published CP11-3 local `drain-rebalance` result into the `Pre-Release Tier` artifact root.
- Do not fake any missing `connection`, `message`, `stream`, `failover`, `restore-recovery`, or `upgrade-rollback` artifacts.

## State Model

- previous `Pre-Release Tier` state: `evidence_partially_collected`
- new `Pre-Release Tier` state: `evidence_partially_collected`
- newly collected slot: `drain_rebalance_drill`
- total collected slots: `3`
- `Capacity Tier` state: `template_only_pending_execution`

## Artifact Contract

- artifact path: `artifacts/perf/step-11/pre-release/drain-rebalance/drill.json`
- collected metric snapshot: `drillDurationMs = 0.983`
- required fields preserved:
  - `runId`
  - `drainCompletionSeconds`
  - `routeMigrationSuccessRate`
  - `rebalanceP95Ms`
- supporting fields:
  - `expectedRouteCount`
  - `migratedRouteCount`
  - `deliveredEventCount`
  - `deliveryPreserved`
  - `sourceBaselinePath`
  - `sourceTestPath`
  - `sourceReviewId`

## Boundary

- `drainCompletionSeconds = 0.000983`, `routeMigrationSuccessRate = 1.0`, and `rebalanceP95Ms = 0.983` are derived from the published local drain drill evidence.
- This artifact is a truthful partial collection record, not full `Pre-Release Tier` sign-off.
- It does not change `Capacity Tier`.

