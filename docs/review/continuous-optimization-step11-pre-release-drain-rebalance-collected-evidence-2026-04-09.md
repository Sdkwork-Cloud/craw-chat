# Continuous Optimization: Step 11 Pre-Release Drain-Rebalance Collected Evidence

## Context

- `Pre-Release Tier` already had truthful `failover` and `restore-recovery` artifacts.
- `drain_rebalance_drill` still stayed placeholder-only even though the repo already published a local CP11-3 drain drill result.

## Confirmed Gap

- `artifacts/perf/step-11/pre-release/drain-rebalance/drill.json` was missing.
- `pre-release-tier-evidence-index.json` still under-reported the real collected-evidence surface.

## Root Cause

- Earlier loops froze the `drain-rebalance` contract but never promoted the published local drill into the high-tier artifact root.

## Decision

- Materialize `artifacts/perf/step-11/pre-release/drain-rebalance/drill.json`.
- Keep `Pre-Release Tier` at `evidence_partially_collected`, not gate-ready.
- Keep `Capacity Tier` at `template_only_pending_execution`.

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_pre_release_drain_rebalance_collected_evidence -- --exact --nocapture
```

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_pre_release_drain_rebalance_collected_evidence -- --exact --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Result

- `Pre-Release Tier` now carries `drain-rebalance/drill.json`.
- `collectionSummary` moves to `collectedSlots = 3`, `pendingSlots = 4`.
- collected metric snapshot: `drillDurationMs = 0.983`
- `drainCompletionSeconds = 0.000983` and `routeMigrationSuccessRate = 1.0` are documented derivations from the published drill evidence.
- `Capacity Tier` still stays `template_only_pending_execution`.

## Next Gap

- Materialize the next truthful high-tier artifact, preferably `upgrade-rollback` for `Pre-Release Tier`, or one real `Capacity Tier` report/output.
