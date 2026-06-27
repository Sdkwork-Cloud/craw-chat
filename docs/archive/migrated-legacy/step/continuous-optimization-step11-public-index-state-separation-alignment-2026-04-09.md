# Continuous Optimization: Step 11 Public Index State Separation Alignment

## Goal

- Make all shared public indexes state the current `Pre-Release Tier` and `Capacity Tier` statuses without ambiguity.

## Scope

- Update only shared index docs and the matching regression test.
- Do not rewrite historical per-artifact docs.

## Implementation

- Add explicit `Capacity Tier` current-state wording to all five public index docs.
- Add a shared boundary line that says only `Capacity Tier` still waits for real collection.
- Add a catalog regression test that requires the split-state summary everywhere.

## Expected State

- `Pre-Release Tier`: `evidence_collected_gate_blocked`
- `Capacity Tier`: `template_only_pending_execution`
- shared boundary: only `Capacity Tier` still waits for real collection

## Boundary

- This loop aligns shared index wording only.
- It does not claim formal capacity evidence exists.
