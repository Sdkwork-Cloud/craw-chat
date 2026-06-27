> Migrated from `docs/架构/09BY-step11-tier-artifact-root-current-state-assertion-alignment-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 11 Tier Artifact Root Current-State Assertion Alignment Implementation Plan

## Goal

- Make the shared Step 11 artifact-root regression test reflect the real current `Pre-Release Tier` state.

## Steps

- Reproduce the failure in `test_continuous_optimization_materializes_step11_tier_artifact_roots_in_repo`.
- Compare the failing assertion with `artifacts/perf/step-11/pre-release/README.md`.
- Replace the stale shared-state expectation with the current gate-blocked state and seven-artifact surface.
- Add a negative assertion that forbids stale `pending_collection` text in the shared README.
- Re-run the targeted test, the full catalog test file, formatting, and the `sdkwork-im-server` offline test suite.

## Boundary

- Do not rewrite historical per-artifact docs that still describe intermediate partial states.
- Do not change `Capacity Tier` semantics in this loop.

