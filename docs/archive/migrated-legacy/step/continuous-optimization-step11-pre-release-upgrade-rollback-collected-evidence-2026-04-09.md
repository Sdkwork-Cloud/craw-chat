# Continuous Optimization: Step 11 Pre-Release Upgrade-Rollback Collected Evidence

## Goal

- Close the next real Step 11 gap by moving `upgrade_rollback_drill` from placeholder-only to collected evidence.

## Scope

- Collect only `upgrade-rollback/drill.json`.
- Keep `Pre-Release Tier` partial.
- Keep `Capacity Tier` unchanged at `template_only_pending_execution`.

## Implementation

- Add `artifacts/perf/step-11/pre-release/upgrade-rollback/drill.json`.
- Update `pre-release-tier-evidence-index.json` to `collectedSlots = 4`, `pendingSlots = 3`.
- Recompute `artifact-file-list.txt` and `checksum-manifest.txt`.
- Backwrite the change into concise review and architecture docs.

## Expected State

- `Pre-Release Tier`: `evidence_partially_collected`
- collected artifact: `artifacts/perf/step-11/pre-release/upgrade-rollback/drill.json`
- key metric: `rollbackActivationMs = 0.007`
- derived metric: `rollbackActivationSeconds = 0.000007`
- `Capacity Tier`: `template_only_pending_execution`

## Boundary

- This is one more truthful partial collection record promoted from published CP11-3 local upgrade-rollback evidence.
- It does not complete `Pre-Release Tier`.
