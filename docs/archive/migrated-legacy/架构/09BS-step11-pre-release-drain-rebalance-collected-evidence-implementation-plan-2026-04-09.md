# Step 11 Pre-Release Drain-Rebalance Collected Evidence Implementation Plan

## Goal

- Promote one already published CP11-3 local `drain-rebalance` drill into the `Pre-Release Tier` artifact root and keep the state model honest.

## Plan

- Add `artifacts/perf/step-11/pre-release/drain-rebalance/drill.json`.
- Update `pre-release-tier-evidence-index.json`, `checksum-manifest.txt`, `artifact-file-list.txt`, and root READMEs.
- Recompute `collectionSummary` to `collectedSlots = 3` and `pendingSlots = 4`.
- Backwrite concise docs under `docs/review/`, `docs/step/`, and `docs/架构/`.

## Boundary

- `Pre-Release Tier` remains `evidence_partially_collected`.
- `Capacity Tier` remains `template_only_pending_execution`.
- `drainCompletionSeconds = 0.000983` and `routeMigrationSuccessRate = 1.0` are documented derivations from the published `drillDurationMs = 0.983` and `migratedRouteCount = 1` evidence.
