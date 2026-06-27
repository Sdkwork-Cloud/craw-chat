# Step 11 Pre-Release Message Metrics Collected Evidence Design

## Decision

- Promote one already published CP11-2 local `message` result into the `Pre-Release Tier` artifact root.
- Do not fake the remaining `stream` artifact.

## State Model

- previous `Pre-Release Tier` state: `evidence_partially_collected`
- new `Pre-Release Tier` state: `evidence_partially_collected`
- newly collected slot: `message_metrics`
- total collected slots: `6`
- 2026-04-09 addendum: `stream_metrics` was collected later the same day, so `Pre-Release Tier` is now `evidence_collected_gate_blocked`, not full gate sign-off.
- `Capacity Tier` state: `template_only_pending_execution`

## Artifact Contract

- artifact path: `artifacts/perf/step-11/pre-release/message/metrics.json`
- collected metric snapshot: `messageP95Ms = 0.152`
- supporting metric: `messagesPerSecond = 7745.652`
- explicit mapping:
  - `messageP95Ms <- postP95Ms`
  - `messagesPerSecond <- messageTps`
- required fields preserved:
  - `runId`
  - `messageCount`
  - `successCount`
  - `messageP95Ms`
  - `messagesPerSecond`
- supporting fields:
  - `totalDurationMs`
  - `sourceBaselinePath`
  - `sourceTestPath`
  - `sourceReviewId`

## Boundary

- `messageP95Ms = 0.152` is doc-captured from the published local CP11-2 message evidence.
- This artifact is a truthful partial collection record, not full `Pre-Release Tier` sign-off.
- It does not change `Capacity Tier`.
