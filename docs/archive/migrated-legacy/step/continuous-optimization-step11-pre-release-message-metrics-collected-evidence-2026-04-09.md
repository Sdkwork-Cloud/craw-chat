# Continuous Optimization: Step 11 Pre-Release Message Metrics Collected Evidence

## Goal

- Close the next real Step 11 gap by moving `message_metrics` from placeholder-only to collected evidence.

## Scope

- Collect only `message/metrics.json`.
- Keep `Pre-Release Tier` partial.
- Keep `Capacity Tier` unchanged at `template_only_pending_execution`.

## Implementation

- Add `artifacts/perf/step-11/pre-release/message/metrics.json`.
- Preserve the field mapping explicitly: `messageP95Ms <- postP95Ms`, `messagesPerSecond <- messageTps`.
- Update `pre-release-tier-evidence-index.json` to `collectedSlots = 6`, `pendingSlots = 1`.
- Recompute `artifact-file-list.txt` and `checksum-manifest.txt`.
- Backwrite the change into concise review and architecture docs.

## Expected State

- `Pre-Release Tier`: `evidence_partially_collected`
- collected artifact: `artifacts/perf/step-11/pre-release/message/metrics.json`
- key metric: `messageP95Ms = 0.152`
- supporting metric: `messagesPerSecond = 7745.652`
- 2026-04-09 addendum: `stream_metrics` was collected later the same day, so `Pre-Release Tier` is now `evidence_collected_gate_blocked`, not full gate sign-off.
- `Capacity Tier`: `template_only_pending_execution`

## Boundary

- This is one more truthful partial collection record promoted from published CP11-2 local message evidence.
- It does not complete `Pre-Release Tier`.
