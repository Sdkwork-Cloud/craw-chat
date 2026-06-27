# Step 11 Pre-Release Message Metrics Collected Evidence Implementation Plan

## Goal

- Promote one already published CP11-2 local `message` metric snapshot into the `Pre-Release Tier` artifact root and keep the state model honest.

## Plan

- Add `artifacts/perf/step-11/pre-release/message/metrics.json`.
- Preserve source-to-gate mapping explicitly: `messageP95Ms <- postP95Ms`, `messagesPerSecond <- messageTps`.
- Update `pre-release-tier-evidence-index.json`, `checksum-manifest.txt`, `artifact-file-list.txt`, and root READMEs.
- Recompute `collectionSummary` to `collectedSlots = 6` and `pendingSlots = 1`.
- Backwrite concise docs under `docs/review/`, `docs/step/`, and `docs/架构/`.

## Boundary

- `Pre-Release Tier` remains `evidence_partially_collected`.
- `Capacity Tier` remains `template_only_pending_execution`.
- `messageP95Ms = 0.152` is doc-captured from published local CP11-2 `postP95Ms`.
