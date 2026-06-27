> Migrated from `docs/review/continuous-optimization-step11-tier-artifact-root-materialization-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Step 11 Tier Artifact-Root Materialization

## Context

- `tools/perf/step-11-scenario-catalog.json` and both tier gate templates already exposed stable high-tier `artifactRoot` paths.
- The repository still had no matching `artifacts/perf/step-11/...` directories.
- That made the published operator drop targets non-materialized.

## Confirmed Bug

- `Pre-Release Tier` and `Capacity Tier` artifact roots were documented but absent from the repo.
- Operators had no checked-in landing path or README guidance for future evidence collection.

## Root Cause

- The artifact-root contract was frozen in catalog/template/docs before the repository materialized those roots.
- The earlier loop fixed public wording, but not the physical repo paths behind that wording.

## Decision

- Materialize both high-tier artifact roots in-repo.
- Add README guidance inside each root instead of claiming real evidence already exists.
- Keep the state honest: `template_only_pending_execution` and `pending_collection`.

## Changed Files

- `artifacts/perf/step-11/README.md`
- `artifacts/perf/step-11/pre-release/README.md`
- `artifacts/perf/step-11/capacity/README.md`
- `services/sdkwork-im-cloud-gateway/tests/performance_drill_catalog_test.rs`
- `docs/step/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_step11_tier_artifact_roots_in_repo -- --exact --nocapture
```

- Failed because `artifacts/perf/step-11/pre-release` did not exist.

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_step11_tier_artifact_roots_in_repo -- --exact --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Next Gap

- The roots now exist, but they still contain no real collected `Pre-Release Tier` or `Capacity Tier` evidence payloads.

