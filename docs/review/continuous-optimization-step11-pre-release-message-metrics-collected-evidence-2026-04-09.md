# Continuous Optimization: Step 11 Pre-Release Message Metrics Collected Evidence

## Context

- `Pre-Release Tier` already carried five truthful local artifacts after the connection backfill loop.
- `message_metrics` still stayed placeholder-only even though published local CP11-2 quantitative evidence already existed.

## Confirmed Gap

- `artifacts/perf/step-11/pre-release/message/metrics.json` was missing.
- `pre-release-tier-evidence-index.json` still under-reported the real collected-evidence surface.

## Root Cause

- Earlier loops froze the `message` metrics contract but never promoted the published local CP11-2 baseline into the high-tier artifact root.
- The gate contract expects `messageP95Ms` and `messagesPerSecond`, while the published source exposed `postP95Ms` and `messageTps`.

## Decision

- Materialize `artifacts/perf/step-11/pre-release/message/metrics.json`.
- Record the mapping explicitly: `messageP95Ms <- postP95Ms`, `messagesPerSecond <- messageTps`.
- Keep `Pre-Release Tier` at `evidence_partially_collected`, not gate-ready.
- Keep `Capacity Tier` at `template_only_pending_execution`.

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_pre_release_message_metrics_collected_evidence -- --exact --nocapture
```

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_pre_release_message_metrics_collected_evidence -- --exact --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Result

- `Pre-Release Tier` now carries `message/metrics.json`.
- `collectionSummary` moves to `collectedSlots = 6`, `pendingSlots = 1`.
- collected metric snapshot: `messageP95Ms = 0.152`
- mapping record: `messageP95Ms <- postP95Ms`, `messagesPerSecond <- messageTps`
- throughput snapshot: `messagesPerSecond = 7745.652`
- `Capacity Tier` still stays `template_only_pending_execution`.

## Next Gap

- 2026-04-09 addendum: `stream/metrics.json` was collected later the same day, so `Pre-Release Tier` is now `evidence_collected_gate_blocked`, not full gate sign-off.
