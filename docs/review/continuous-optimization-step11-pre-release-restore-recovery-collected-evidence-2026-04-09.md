# Continuous Optimization: Step 11 Pre-Release Restore-Recovery Collected Evidence

## Context

- `Pre-Release Tier` already had one truthful `failover` artifact.
- `restore_recovery_drill` still stayed placeholder-only even though the repo already published a local CP11-3 restore drill result.

## Confirmed Gap

- `artifacts/perf/step-11/pre-release/restore-recovery/drill.json` was missing.
- `pre-release-tier-evidence-index.json` still under-reported the real collected-evidence surface.

## Root Cause

- Earlier loops froze the `restore-recovery` contract but never promoted the published local drill into the high-tier artifact root.

## Decision

- Materialize `artifacts/perf/step-11/pre-release/restore-recovery/drill.json`.
- Keep `Pre-Release Tier` at `evidence_partially_collected`, not gate-ready.
- Keep `Capacity Tier` at `template_only_pending_execution`.

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_pre_release_restore_recovery_collected_evidence -- --exact --nocapture
```

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_pre_release_restore_recovery_collected_evidence -- --exact --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Result

- `Pre-Release Tier` now carries `restore-recovery/drill.json`.
- `collectionSummary` moves to `collectedSlots = 2`, `pendingSlots = 5`.
- collected metric snapshot: `restoreDurationMs = 17.983`
- `restoreRtoSeconds = 0.017983` is a documented derivation from the published `restoreDurationMs` evidence.
- `Capacity Tier` still stays `template_only_pending_execution`.

## Next Gap

- Materialize the next truthful high-tier artifact, preferably `drain-rebalance` or `upgrade-rollback` for `Pre-Release Tier`, or one real `Capacity Tier` report/output.
