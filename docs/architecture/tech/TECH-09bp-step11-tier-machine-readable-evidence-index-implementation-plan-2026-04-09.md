> Migrated from `docs/架构/09BP-step11-tier-machine-readable-evidence-index-implementation-plan-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 11 Tier Machine-Readable Evidence Index Implementation Plan

**Goal:** Make Step 11 high-tier evidence collection discoverable from `artifactRoot` itself, not only from `tools/perf/` gate templates.

**Architecture:** Reuse the existing tier gate templates as the source contract, then mirror that contract into co-located artifact-root evidence indexes plus placeholder manifest/list entrypoints.

**Tech Stack:** Rust regression test, JSON schema, JSON evidence index templates, Markdown READMEs

---

### Task 1: Freeze the missing co-located index bug

**Files:**
- Modify: `services/sdkwork-im-cloud-gateway/tests/performance_drill_catalog_test.rs`

- [ ] Add a regression test that requires:
  - `artifacts/perf/step-11/schemas/step-11-tier-evidence-index.schema.json`
  - one high-tier evidence index per artifact root
  - placeholder checksum/file-list files
- [ ] Run:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_co_locates_step11_tier_evidence_indexes_with_artifact_roots -- --exact --nocapture
```

- [ ] Confirm red because the schema and co-located evidence indexes are missing.

### Task 2: Materialize the operator-local contract

**Files:**
- Create: `artifacts/perf/step-11/schemas/step-11-tier-evidence-index.schema.json`
- Create: `artifacts/perf/step-11/pre-release/pre-release-tier-evidence-index.json`
- Create: `artifacts/perf/step-11/capacity/capacity-tier-evidence-index.json`
- Create: `artifacts/perf/step-11/pre-release/checksum-manifest.txt`
- Create: `artifacts/perf/step-11/pre-release/artifact-file-list.txt`
- Create: `artifacts/perf/step-11/capacity/checksum-manifest.txt`
- Create: `artifacts/perf/step-11/capacity/artifact-file-list.txt`

- [ ] Keep `collectionSummary` and `evidenceSlots` aligned with the existing tier gate templates.
- [ ] Keep the state honest:
  - `template_only_pending_execution`
  - `pending_collection`

### Task 3: Backwrite the closure

**Files:**
- Modify: `artifacts/perf/step-11/README.md`
- Modify: `artifacts/perf/step-11/pre-release/README.md`
- Modify: `artifacts/perf/step-11/capacity/README.md`
- Modify: `docs/step/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md`
- Create: `docs/review/continuous-optimization-step11-tier-machine-readable-evidence-index-2026-04-09.md`
- Create: `docs/step/continuous-optimization-step11-tier-machine-readable-evidence-index-2026-04-09.md`
- Create: `docs/架构/150BP-step11-tier-machine-readable-evidence-index-design-2026-04-09.md`
- Modify: `docs/review/README.md`
- Modify: `docs/step/README.md`
- Modify: `docs/架构/README.md`

- [ ] Run:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_co_locates_step11_tier_evidence_indexes_with_artifact_roots -- --exact --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

