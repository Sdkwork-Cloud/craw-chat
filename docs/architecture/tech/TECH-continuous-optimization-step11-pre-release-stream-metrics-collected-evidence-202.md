> Migrated from `docs/review/continuous-optimization-step11-pre-release-stream-metrics-collected-evidence-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Step 11 Pre-Release Stream Metrics Collected Evidence

## Context

- `Pre-Release Tier` already carried six truthful local artifacts after the message loop.
- `stream_metrics` still stayed placeholder-only even though published local CP11-2 quantitative evidence already existed.

## Confirmed Gap

- `artifacts/perf/step-11/pre-release/stream/metrics.json` was missing.
- `pre-release-tier-evidence-index.json` still under-reported the real collected-evidence surface.

## Root Cause

- Earlier loops froze the `stream` metrics contract but never promoted the published local CP11-2 baseline into the high-tier artifact root.
- The gate contract expects `frameP95Ms`, while the published source exposed `appendP95Ms`.

## Decision

- Materialize `artifacts/perf/step-11/pre-release/stream/metrics.json`.
- Record the mapping explicitly: `frameP95Ms <- appendP95Ms`.
- Move `Pre-Release Tier` to `evidence_collected_gate_blocked`.
- Keep `Capacity Tier` at `template_only_pending_execution`.

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_pre_release_stream_metrics_collected_evidence -- --exact --nocapture
```

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_pre_release_stream_metrics_collected_evidence -- --exact --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Result

- `Pre-Release Tier` now carries `stream/metrics.json`.
- `collectionSummary` moves to `collectedSlots = 7`, `pendingSlots = 0`.
- collected metric snapshot: `frameP95Ms = 0.117`
- mapping record: `frameP95Ms <- appendP95Ms`
- throughput snapshot: `framesPerSecond = 10613.071`
- `Pre-Release Tier` is now `evidence_collected_gate_blocked`, not full gate sign-off.
- `Capacity Tier` still stays `template_only_pending_execution`.

## Next Gap

- Keep the current truth explicit: all `Pre-Release Tier` slots are collected, but formal gate execution is still blocked and `Capacity Tier` remains template-only.

