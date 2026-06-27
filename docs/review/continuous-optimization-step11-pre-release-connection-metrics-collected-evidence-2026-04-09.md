# Continuous Optimization: Step 11 Pre-Release Connection Metrics Collected Evidence

## Context

- `Pre-Release Tier` already had four truthful drill artifacts.
- `connection_metrics` still stayed placeholder-only even though published local CP11-2 quantitative evidence already existed.

## Confirmed Gap

- `artifacts/perf/step-11/pre-release/connection/metrics.json` was missing.
- `pre-release-tier-evidence-index.json` still under-reported the real collected-evidence surface.

## Root Cause

- Earlier loops froze the `connection` metrics contract but never promoted the published local CP11-2 baseline into the high-tier artifact root.

## Decision

- Materialize `artifacts/perf/step-11/pre-release/connection/metrics.json`.
- Keep `Pre-Release Tier` at `evidence_partially_collected`, not gate-ready.
- Keep `Capacity Tier` at `template_only_pending_execution`.

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_pre_release_connection_metrics_collected_evidence -- --exact --nocapture
```

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_pre_release_connection_metrics_collected_evidence -- --exact --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Result

- `Pre-Release Tier` now carries `connection/metrics.json`.
- `collectionSummary` moves to `collectedSlots = 5`, `pendingSlots = 2`.
- collected metric snapshot: `connectP95Ms = 15.108`
- throughput snapshot: `connectionsPerSecond = 1802.431`
- `Capacity Tier` still stays `template_only_pending_execution`.

## Next Gap

- Historical next-gap note superseded on 2026-04-09 after `message/metrics.json` was collected.
- 2026-04-09 addendum: `stream/metrics.json` was collected later the same day, so `Pre-Release Tier` is now `evidence_collected_gate_blocked`, not full gate sign-off.
