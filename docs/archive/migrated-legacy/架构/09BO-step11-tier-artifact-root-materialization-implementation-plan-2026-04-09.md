# Step 11 Tier Artifact-Root Materialization Implementation Plan

**Goal:** Materialize the documented Step 11 high-tier artifact roots so the published `artifactRoot` contract resolves to real repository paths.

**Architecture:** Reuse the existing tier gate templates as the source of truth. Add only checked-in artifact-root guidance, not fake evidence payloads.

**Tech Stack:** Rust regression test, Markdown artifact-root READMEs

---

### Task 1: Freeze the missing-root bug

**Files:**
- Modify: `services/sdkwork-im-cloud-gateway/tests/performance_drill_catalog_test.rs`

- [ ] Add a regression test that requires both high-tier `artifactRoot` directories and README guidance to exist.
- [ ] Run:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_step11_tier_artifact_roots_in_repo -- --exact --nocapture
```

- [ ] Confirm red because the repo has no `artifacts/perf/step-11/pre-release` root yet.

### Task 2: Materialize the roots

**Files:**
- Create: `artifacts/perf/step-11/README.md`
- Create: `artifacts/perf/step-11/pre-release/README.md`
- Create: `artifacts/perf/step-11/capacity/README.md`

- [ ] Add concise guidance with:
  - stable `artifactRoot`
  - source `gateTemplate`
  - current state `template_only_pending_execution`
  - slot state `pending_collection`
  - naming rule `artifactPath = artifactRoot + "/" + suggestedRelativePath`

### Task 3: Backwrite the closure

**Files:**
- Modify: `docs/step/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md`
- Create: `docs/review/continuous-optimization-step11-tier-artifact-root-materialization-2026-04-09.md`
- Create: `docs/step/continuous-optimization-step11-tier-artifact-root-materialization-2026-04-09.md`
- Create: `docs/架构/150BO-step11-tier-artifact-root-materialization-design-2026-04-09.md`
- Modify: `docs/review/README.md`
- Modify: `docs/step/README.md`
- Modify: `docs/架构/README.md`

- [ ] Run:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_step11_tier_artifact_roots_in_repo -- --exact --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```
