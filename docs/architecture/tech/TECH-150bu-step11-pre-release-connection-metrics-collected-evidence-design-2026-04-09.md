> Migrated from `docs/架构/150BU-step11-pre-release-connection-metrics-collected-evidence-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 11 Pre-Release Connection Metrics Collected Evidence Design

## Decision

- Promote one already published CP11-2 local `connection` result into the `Pre-Release Tier` artifact root.
- Do not fake any missing `message` or `stream` artifacts.

## State Model

- previous `Pre-Release Tier` state: `evidence_partially_collected`
- new `Pre-Release Tier` state: `evidence_partially_collected`
- newly collected slot: `connection_metrics`
- total collected slots: `5`
- `Capacity Tier` state: `template_only_pending_execution`

## Artifact Contract

- artifact path: `artifacts/perf/step-11/pre-release/connection/metrics.json`
- collected metric snapshot: `connectP95Ms = 15.108`
- supporting metric: `connectionsPerSecond = 1802.431`
- required fields preserved:
  - `runId`
  - `connectionCount`
  - `successCount`
  - `connectP95Ms`
  - `connectionsPerSecond`
- supporting fields:
  - `totalDurationMs`
  - `sourceBaselinePath`
  - `sourceTestPath`
  - `sourceReviewId`

## Boundary

- `connectP95Ms = 15.108` is doc-captured from the published local CP11-2 connection evidence.
- This artifact is a truthful partial collection record, not full `Pre-Release Tier` sign-off.
- It does not change `Capacity Tier`.

