# Continuous Optimization: Step 11 Pre-Release Upgrade-Rollback Collected Evidence

## Context

- `Pre-Release Tier` already had truthful `failover`, `restore-recovery`, and `drain-rebalance` artifacts.
- `upgrade_rollback_drill` still stayed placeholder-only even though the repo already published a local CP11-3 upgrade-rollback result.

## Confirmed Gap

- `artifacts/perf/step-11/pre-release/upgrade-rollback/drill.json` was missing.
- `pre-release-tier-evidence-index.json` still under-reported the real collected-evidence surface.

## Root Cause

- Earlier loops froze the `upgrade-rollback` contract but never promoted the published local drill into the high-tier artifact root.

## Decision

- Materialize `artifacts/perf/step-11/pre-release/upgrade-rollback/drill.json`.
- Keep `Pre-Release Tier` at `evidence_partially_collected`, not gate-ready.
- Keep `Capacity Tier` at `template_only_pending_execution`.

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_pre_release_upgrade_rollback_collected_evidence -- --exact --nocapture
```

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_pre_release_upgrade_rollback_collected_evidence -- --exact --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Result

- `Pre-Release Tier` now carries `upgrade-rollback/drill.json`.
- `collectionSummary` moves to `collectedSlots = 4`, `pendingSlots = 3`.
- collected metric snapshot: `rollbackActivationMs = 0.007`
- `rollbackActivationSeconds = 0.000007` is a documented derivation from the published drill evidence.
- `Capacity Tier` still stays `template_only_pending_execution`.

## Next Gap

- Materialize the next truthful high-tier artifact, preferably one `Pre-Release Tier` quantitative metric artifact such as `connection/metrics.json`, or one real `Capacity Tier` report/output.
