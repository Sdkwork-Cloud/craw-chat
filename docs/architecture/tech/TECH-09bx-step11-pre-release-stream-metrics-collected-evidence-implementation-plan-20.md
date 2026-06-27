> Migrated from `docs/架构/09BX-step11-pre-release-stream-metrics-collected-evidence-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 11 Pre-Release Stream Metrics Collected Evidence Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Materialize the last truthful `Pre-Release Tier` slot as `stream/metrics.json` and move the tier into `evidence_collected_gate_blocked`.

**Architecture:** Reuse the existing CP11-2 doc-capture pattern, add one stream artifact, then refresh the shared evidence index, manifests, and concise backwrite docs. The resulting tier is complete for collected evidence but still not full gate sign-off.

**Tech Stack:** JSON artifacts, Markdown backwrite docs, Rust regression tests in `performance_drill_catalog_test.rs`

---

### Task 1: Stream Artifact

**Files:**
- Create: `artifacts/perf/step-11/pre-release/stream/metrics.json`
- Modify: `services/sdkwork-im-cloud-gateway/tests/performance_drill_catalog_test.rs`

- [ ] Write the failing test for `test_continuous_optimization_materializes_pre_release_stream_metrics_collected_evidence`.
- [ ] Run the exact test and confirm it fails because `stream/metrics.json` is missing.
- [ ] Add `stream/metrics.json` with `frameP95Ms = 0.117`, `framesPerSecond = 10613.071`, and `frameP95Ms <- appendP95Ms`.
- [ ] Re-run the exact test and confirm the next failure moves to shared index/docs.

### Task 2: Shared Index

**Files:**
- Modify: `artifacts/perf/step-11/pre-release/pre-release-tier-evidence-index.json`
- Modify: `artifacts/perf/step-11/pre-release/checksum-manifest.txt`
- Modify: `artifacts/perf/step-11/pre-release/artifact-file-list.txt`
- Modify: `artifacts/perf/step-11/pre-release/README.md`
- Modify: `artifacts/perf/step-11/README.md`

- [ ] Move `Pre-Release Tier` to `evidence_collected_gate_blocked`.
- [ ] Update `collectionSummary` to `7 / 0`.
- [ ] Mark `stream_metrics` as collected with real `sizeBytes` and `checksumSha256`.
- [ ] Remove the final pending placeholder comment for `stream/metrics.json`.

### Task 3: Backwrite Docs

**Files:**
- Create: `docs/review/continuous-optimization-step11-pre-release-stream-metrics-collected-evidence-2026-04-09.md`
- Create: `docs/step/continuous-optimization-step11-pre-release-stream-metrics-collected-evidence-2026-04-09.md`
- Create: `docs/架构/150BX-step11-pre-release-stream-metrics-collected-evidence-design-2026-04-09.md`
- Modify: `docs/step/README.md`
- Modify: `docs/review/README.md`
- Modify: `docs/架构/README.md`

- [ ] Add concise stream collected-evidence review/step/design docs.
- [ ] Index the new docs in the public README files.
- [ ] State explicitly that the tier is `evidence_collected_gate_blocked`, `Capacity Tier` stays `template_only_pending_execution`, and this is not full gate sign-off.

### Task 4: Historical Corrections

**Files:**
- Modify: `docs/review/continuous-optimization-step11-closure-claim-supersession-2026-04-09.md`
- Modify: `docs/step/continuous-optimization-step11-closure-claim-supersession-2026-04-09.md`
- Modify: `docs/架构/09BV-step11-closure-claim-supersession-implementation-plan-2026-04-09.md`
- Modify: `docs/架构/150BV-step11-closure-claim-supersession-design-2026-04-09.md`
- Modify: `docs/架构/09-实施计划.md`

- [ ] Replace the stale `remaining slot: stream_metrics` wording.
- [ ] Record that `stream_metrics` was collected on `2026-04-09`.
- [ ] Record that all truthful `Pre-Release Tier` slots are now materialized but the tier is still not full gate sign-off.

### Task 5: Verification

**Files:**
- Verify: `services/sdkwork-im-cloud-gateway/tests/performance_drill_catalog_test.rs`

- [ ] Run the exact stream test.
- [ ] Run `cargo fmt --all --check`.
- [ ] Run `cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture`.
- [ ] Run `cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture`.

