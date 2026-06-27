# Continuous Optimization: Step 11 Tier Artifact Root Current-State Assertion Alignment

## Context

- `stream_metrics` closed the last truthful `Pre-Release Tier` slot.
- The shared artifact root README now exposes `evidence_collected_gate_blocked` with seven collected artifacts.

## Confirmed Bug

- `test_continuous_optimization_materializes_step11_tier_artifact_roots_in_repo` still required `evidence_partially_collected`.
- The same old assertion also required a stale `pending_collection` placeholder in `artifacts/perf/step-11/pre-release/README.md`.

## Root Cause

- The test was authored during the partial-collection phase and never upgraded after the final `stream_metrics` backfill.
- Historical per-artifact docs stayed truthful, but the shared current-state assertion drifted.

## Fix

- Update the shared `Pre-Release Tier` README assertion to require:
  - `evidence_collected_gate_blocked`
  - `connection_metrics`, `message_metrics`, `stream_metrics`
  - all seven collected artifact paths
  - the `all seven truthful local artifacts` boundary text
- Add a negative assertion so the shared README cannot regress back to `pending_collection`.

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_step11_tier_artifact_roots_in_repo -- --exact --nocapture
```

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_step11_tier_artifact_roots_in_repo -- --exact --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Result

- The shared Step 11 artifact-root contract now matches the repo's current truth.
- Historical docs that describe earlier partial states remain unchanged and still truthful.

## Next Gap

- Continue the review loop above Step 11 docs into broader provider/runtime/cross-platform verification for `sdkwork-im-server`.
