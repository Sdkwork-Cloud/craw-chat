# Continuous Optimization: Step 11 Pre-Release Restore-Recovery Collected Evidence

## Goal

- Close the next real Step 11 gap by moving `restore_recovery_drill` from placeholder-only to collected evidence.

## Scope

- Collect only `restore-recovery/drill.json`.
- Keep `Pre-Release Tier` partial.
- Keep `Capacity Tier` unchanged at `template_only_pending_execution`.

## Implementation

- Add `artifacts/perf/step-11/pre-release/restore-recovery/drill.json`.
- Update `pre-release-tier-evidence-index.json` to `collectedSlots = 2`, `pendingSlots = 5`.
- Recompute `artifact-file-list.txt` and `checksum-manifest.txt`.
- Backwrite the change into concise review and architecture docs.

## Expected State

- `Pre-Release Tier`: `evidence_partially_collected`
- collected artifact: `artifacts/perf/step-11/pre-release/restore-recovery/drill.json`
- key metric: `restoreDurationMs = 17.983`
- derived metric: `restoreRtoSeconds = 0.017983`
- `Capacity Tier`: `template_only_pending_execution`

## Boundary

- This is one more truthful partial collection record promoted from published CP11-3 local restore evidence.
- It does not complete `Pre-Release Tier`.
