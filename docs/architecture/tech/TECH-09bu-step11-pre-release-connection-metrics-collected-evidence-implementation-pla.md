> Migrated from `docs/架构/09BU-step11-pre-release-connection-metrics-collected-evidence-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 11 Pre-Release Connection Metrics Collected Evidence Implementation Plan

## Goal

- Promote one already published CP11-2 local `connection` metric snapshot into the `Pre-Release Tier` artifact root and keep the state model honest.

## Plan

- Add `artifacts/perf/step-11/pre-release/connection/metrics.json`.
- Update `pre-release-tier-evidence-index.json`, `checksum-manifest.txt`, `artifact-file-list.txt`, and root READMEs.
- Recompute `collectionSummary` to `collectedSlots = 5` and `pendingSlots = 2`.
- Backwrite concise docs under `docs/review/`, `docs/step/`, and `docs/架构/`.

## Boundary

- `Pre-Release Tier` remains `evidence_partially_collected`.
- `Capacity Tier` remains `template_only_pending_execution`.
- `connectP95Ms = 15.108` is doc-captured from published local CP11-2 quantitative evidence.

