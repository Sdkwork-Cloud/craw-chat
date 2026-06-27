# Continuous Optimization: Step 11 Tier-Gate Doc State Alignment

## Context

- `tools/perf/step-11-scenario-catalog.json` already exposes tier-level `gateTemplate` and `artifactRoot`.
- The existing Step 11 optimization doc still described that exposure as a remaining gap.
- This created operator-doc drift: the code, schema, and tests were ahead of the step narrative.

## Confirmed Bug

- The step doc no longer matched repository truth.
- Review and architecture docs already described the post-fix state, but the step doc still read like a pre-fix note.

## Root Cause

- The catalog change landed first.
- The follow-up step doc retained stale wording instead of recording that the catalog gap had already closed.

## Decision

- Do not change the Step 11 catalog again.
- Add an explicit addendum to the existing step doc.
- Make the addendum explicitly supersede the stale earlier catalog-gap wording.
- Freeze the corrected state with an automated doc regression test.

## Changed Files

- `services/sdkwork-im-cloud-gateway/tests/performance_drill_catalog_test.rs`
- `docs/step/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_step11_step_doc_marks_artifact_root_gap_closed -- --exact --nocapture
```

- Failed because the step doc did not contain the closure addendum.

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_step11_step_doc_marks_artifact_root_gap_closed -- --exact --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture
```

## Next Gap

- The catalog contract is aligned now.
- The remaining truthful Step 11 gaps are formal `Pre-Release Tier` gate execution and `Capacity Tier` evidence collection, not catalog discoverability.
