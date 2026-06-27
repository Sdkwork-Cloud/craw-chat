# Continuous Optimization: Step 11 Tier Artifact Root Current-State Assertion Alignment

## Goal

- Align the shared Step 11 artifact-root regression test with the current `Pre-Release Tier` truth.

## Scope

- Touch only `services/sdkwork-im-cloud-gateway/tests/performance_drill_catalog_test.rs`.
- Keep historical partial-state backwrite docs unchanged.
- Keep `Capacity Tier` expectations unchanged.

## Implementation

- Replace the stale shared-state assertion `evidence_partially_collected` with `evidence_collected_gate_blocked`.
- Require the current collected slot ids and all seven collected artifact paths.
- Add a guard that rejects stale `pending_collection` text in the shared `Pre-Release Tier` README.

## Expected State

- shared README state: `evidence_collected_gate_blocked`
- shared README boundary: `all seven truthful local artifacts`
- shared README pending placeholders: none
- `Capacity Tier`: still `template_only_pending_execution`

## Boundary

- This loop fixes only the shared current-state assertion drift.
- It does not rewrite historical docs that intentionally describe earlier partial collection stages.
