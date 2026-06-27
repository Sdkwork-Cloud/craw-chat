> Migrated from `docs/step/continuous-optimization-step11-tier-machine-readable-evidence-index-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step: Step 11 Tier Machine-Readable Evidence Index

## Goal

Co-locate a machine-readable evidence index with each Step 11 high-tier artifact root.

## Red

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_co_locates_step11_tier_evidence_indexes_with_artifact_roots -- --exact --nocapture
```

- The repo had no `artifacts/perf/step-11/schemas/step-11-tier-evidence-index.schema.json`.

## Green

- Added a shared Step 11 tier evidence-index schema.
- Added `pre-release-tier-evidence-index.json` and `capacity-tier-evidence-index.json`.
- Added placeholder `checksum-manifest.txt` and `artifact-file-list.txt` files beside each index.

## Verify

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_co_locates_step11_tier_evidence_indexes_with_artifact_roots -- --exact --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Next

- Add one real `Pre-Release Tier` or `Capacity Tier` evidence payload and update the new index from placeholder to collected state.

