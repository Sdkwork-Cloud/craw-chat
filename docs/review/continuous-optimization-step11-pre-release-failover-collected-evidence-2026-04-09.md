# Continuous Optimization: Step 11 Pre-Release Failover Collected Evidence

## Context

- Step 11 high-tier roots and machine-readable indexes already existed.
- The repo still lacked any real collected high-tier evidence payload.
- Existing Step 11 review material already published one local failover drill result.

## Confirmed Gap

- `artifacts/perf/step-11/pre-release` had no collected artifact, so `pre-release-tier-evidence-index.json` stayed template-only.
- Operators still had to read review docs instead of consuming one co-located artifact under the tier root.

## Root Cause

- Earlier loops froze the collection contract but never materialized one truthful artifact from the already published CP11-3 local failover evidence.

## Decision

- Materialize one collected `Pre-Release Tier` failover artifact at `artifacts/perf/step-11/pre-release/failover/drill.json`.
- Move `pre-release-tier-evidence-index.json` from `template_only_pending_execution` to `evidence_partially_collected`.
- Keep the boundary honest: this closes only one `Pre-Release Tier` slot; `Capacity Tier` and the remaining Pre-Release slots are still pending.

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_pre_release_failover_collected_evidence -- --exact --nocapture
```

- Failed because `artifacts/perf/step-11/pre-release/failover/drill.json` was missing.

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_pre_release_failover_collected_evidence -- --exact --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Result

- `Pre-Release Tier` now carries one real collected `failover` artifact.
- `pre-release-tier-evidence-index.json` can now surface `evidence_partially_collected`.
- collected metric snapshot: `takeoverDurationMs = 0.553`
- `Capacity Tier` still stays `template_only_pending_execution`.

## Next Gap

- Materialize the next truthful high-tier artifact, preferably `restore-recovery` for `Pre-Release Tier` or one real `Capacity Tier` report/output.
