# Step 11 Pre-Release Stream Metrics Collected Evidence Design

## Decision

- Promote one already published CP11-2 local `stream` result into the `Pre-Release Tier` artifact root.
- Do not fake gate readiness after the last slot lands.

## State Model

- previous `Pre-Release Tier` state: `evidence_partially_collected`
- new `Pre-Release Tier` state: `evidence_collected_gate_blocked`
- newly collected slot: `stream_metrics`
- total collected slots: `7`
- pending slots: `0`
- `Capacity Tier` state: `template_only_pending_execution`

## Artifact Contract

- artifact path: `artifacts/perf/step-11/pre-release/stream/metrics.json`
- collected metric snapshot: `frameP95Ms = 0.117`
- supporting metric: `framesPerSecond = 10613.071`
- explicit mapping:
  - `frameP95Ms <- appendP95Ms`
- required fields preserved:
  - `runId`
  - `frameCount`
  - `successCount`
  - `frameP95Ms`
  - `framesPerSecond`
- supporting fields:
  - `totalDurationMs`
  - `sourceBaselinePath`
  - `sourceTestPath`
  - `sourceReviewId`

## Boundary

- `frameP95Ms = 0.117` is doc-captured from the published local CP11-2 stream evidence.
- The tier is `evidence_collected_gate_blocked`, not full gate sign-off.
- It does not change `Capacity Tier`.
