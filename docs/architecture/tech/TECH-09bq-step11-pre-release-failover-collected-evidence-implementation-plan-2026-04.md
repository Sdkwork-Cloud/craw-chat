> Migrated from `docs/架构/09BQ-step11-pre-release-failover-collected-evidence-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 11 Pre-Release Failover Collected Evidence Implementation Plan

**Goal:** Materialize one truthful `Pre-Release Tier` failover artifact and move the tier evidence index into `evidence_partially_collected` state.

**Architecture:** Reuse already published CP11-3 local failover evidence, store it under the Pre-Release artifact root, and update the co-located machine-readable index plus manifest files. Keep the rest of the tier and all of Capacity Tier explicitly pending.

**Capacity Tier:** `template_only_pending_execution`

**Tech Stack:** Rust regression tests, JSON artifact contracts, Markdown review backwrites

---

### Task 1: Freeze the collected-evidence contract

**Files:**
- Modify: `services/sdkwork-im-cloud-gateway/tests/performance_drill_catalog_test.rs`
- Create: `artifacts/perf/step-11/pre-release/failover/drill.json`
- Modify: `artifacts/perf/step-11/pre-release/pre-release-tier-evidence-index.json`
- Modify: `artifacts/perf/step-11/pre-release/checksum-manifest.txt`
- Modify: `artifacts/perf/step-11/pre-release/artifact-file-list.txt`
- Modify: `artifacts/perf/step-11/pre-release/README.md`

- [ ] Write the failing regression test for one collected `failover_drill` slot.
- [ ] Run the focused test and confirm it fails because the artifact is missing.
- [ ] Add the minimal collected artifact and partial-collected index state.
- [ ] Record the collected metric snapshot `takeoverDurationMs = 0.553`.
- [ ] Re-run the focused test and confirm it passes.

### Task 2: Backwrite concise docs

**Files:**
- Create: `docs/review/continuous-optimization-step11-pre-release-failover-collected-evidence-2026-04-09.md`
- Create: `docs/step/continuous-optimization-step11-pre-release-failover-collected-evidence-2026-04-09.md`
- Create: `docs/架构/150BQ-step11-pre-release-failover-collected-evidence-design-2026-04-09.md`
- Modify: `docs/step/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md`
- Modify: `docs/review/README.md`
- Modify: `docs/step/README.md`
- Modify: `docs/架构/README.md`

- [ ] Record root cause, boundary, and next gap.
- [ ] Mark that `Pre-Release Tier` now has one collected artifact.
- [ ] Keep `Capacity Tier` explicitly pending.

### Task 3: Verify the loop

**Files:**
- Verify only

- [ ] Run `cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_pre_release_failover_collected_evidence -- --exact --nocapture`
- [ ] Run `cargo fmt --all --check`
- [ ] Run `cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture`
- [ ] Run `cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture`

