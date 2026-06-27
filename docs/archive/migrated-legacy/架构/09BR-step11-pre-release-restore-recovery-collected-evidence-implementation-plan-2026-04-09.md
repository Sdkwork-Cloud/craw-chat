# Step 11 Pre-Release Restore-Recovery Collected Evidence Implementation Plan

## Goal

- Promote one already published CP11-3 local `restore-recovery` drill into the `Pre-Release Tier` artifact root and keep the state model honest.

## Plan

- Add `artifacts/perf/step-11/pre-release/restore-recovery/drill.json`.
- Update `pre-release-tier-evidence-index.json`, `checksum-manifest.txt`, `artifact-file-list.txt`, and root READMEs.
- Recompute `collectionSummary` to `collectedSlots = 2` and `pendingSlots = 5`.
- Backwrite concise docs under `docs/review/`, `docs/step/`, and `docs/架构/`.

## Boundary

- `Pre-Release Tier` remains `evidence_partially_collected`.
- `Capacity Tier` remains `template_only_pending_execution`.
- `restoreRtoSeconds` is a documented derivation from the published `restoreDurationMs = 17.983` evidence.
