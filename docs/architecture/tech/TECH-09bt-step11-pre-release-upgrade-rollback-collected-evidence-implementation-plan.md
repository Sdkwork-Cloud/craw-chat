> Migrated from `docs/架构/09BT-step11-pre-release-upgrade-rollback-collected-evidence-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 11 Pre-Release Upgrade-Rollback Collected Evidence Implementation Plan

## Goal

- Promote one already published CP11-3 local `upgrade-rollback` drill into the `Pre-Release Tier` artifact root and keep the state model honest.

## Plan

- Add `artifacts/perf/step-11/pre-release/upgrade-rollback/drill.json`.
- Update `pre-release-tier-evidence-index.json`, `checksum-manifest.txt`, `artifact-file-list.txt`, and root READMEs.
- Recompute `collectionSummary` to `collectedSlots = 4` and `pendingSlots = 3`.
- Backwrite concise docs under `docs/review/`, `docs/step/`, and `docs/架构/`.

## Boundary

- `Pre-Release Tier` remains `evidence_partially_collected`.
- `Capacity Tier` remains `template_only_pending_execution`.
- `rollbackActivationSeconds = 0.000007` is a documented derivation from the published `rollbackActivationMs = 0.007` evidence.

