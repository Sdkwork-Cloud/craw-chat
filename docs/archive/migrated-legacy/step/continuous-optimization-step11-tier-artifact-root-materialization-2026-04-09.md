# Step: Step 11 Tier Artifact-Root Materialization

## Goal

Turn the published Step 11 high-tier `artifactRoot` paths into real repository paths with minimal operator guidance.

## Red

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_step11_tier_artifact_roots_in_repo -- --exact --nocapture
```

- The repo did not contain `artifacts/perf/step-11/pre-release`.

## Green

- Materialized `artifacts/perf/step-11/pre-release` and `artifacts/perf/step-11/capacity`.
- Added README guidance that points back to the tier gate templates.
- Kept the state explicit: `template_only_pending_execution` and `pending_collection`.

## Verify

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_materializes_step11_tier_artifact_roots_in_repo -- --exact --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Next

- Add one machine-readable high-tier evidence index template or one real collected high-tier sample.
