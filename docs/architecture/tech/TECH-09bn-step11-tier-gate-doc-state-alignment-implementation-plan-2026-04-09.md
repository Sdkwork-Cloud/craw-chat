> Migrated from `docs/架构/09BN-step11-tier-gate-doc-state-alignment-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 11 Tier-Gate Doc State Alignment Implementation Plan

**Goal:** Remove Step 11 doc-state drift by making the optimization step doc reflect the already-shipped tier-level `artifactRoot` contract.

**Architecture:** Treat documentation as a tested public contract. The catalog and schema stay unchanged; only the stale step narrative is corrected and then frozen with an automated regression test.

**Tech Stack:** Rust integration tests, Markdown backwrites

---

### Task 1: Prove the doc drift

**Files:**
- Modify: `services/sdkwork-im-cloud-gateway/tests/performance_drill_catalog_test.rs`

- [ ] Add a regression test that requires the Step 11 step doc to say the catalog gap is closed.
- [ ] Run:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_step11_step_doc_marks_artifact_root_gap_closed -- --exact --nocapture
```

- [ ] Confirm red because the addendum is missing.

### Task 2: Correct the public step doc

**Files:**
- Modify: `docs/step/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md`

- [ ] Add an explicit addendum saying:
  - the catalog already exposes `gateTemplate` and `artifactRoot`
  - the remaining gap is real evidence collection

### Task 3: Backwrite the closure

**Files:**
- Create: `docs/review/continuous-optimization-step11-tier-gate-doc-state-alignment-2026-04-09.md`
- Create: `docs/step/continuous-optimization-step11-tier-gate-doc-state-alignment-2026-04-09.md`
- Create: `docs/架构/150BN-step11-tier-gate-doc-state-alignment-design-2026-04-09.md`
- Modify: `docs/review/README.md`
- Modify: `docs/step/README.md`
- Modify: `docs/架构/README.md`

- [ ] Run:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_step11_step_doc_marks_artifact_root_gap_closed -- --exact --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture
```

