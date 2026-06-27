> Migrated from `docs/review/continuous-optimization-step11-tier-machine-readable-evidence-index-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: Step 11 Tier Machine-Readable Evidence Index

## Context

- Step 11 already had machine-readable tier gate templates in `tools/perf/`.
- The repo now materializes `artifactRoot` directories for `Pre-Release Tier` and `Capacity Tier`.
- Operators still had no co-located machine-readable evidence entrypoint inside those roots.

## Confirmed Bug

- High-tier evidence collection still required jumping from `artifactRoot` back to `tools/perf/...gate.json`.
- The repo lacked a co-located schema, tier evidence index, checksum manifest placeholder, and artifact file list placeholder.

## Root Cause

- The Step 11 machine-readable contract stopped at gate templates.
- Artifact-root materialization fixed filesystem presence, but not operator-local evidence indexing.
- Earlier Step 11 review notes already flagged the likely missing unified evidence index / artifact schema.

## Decision

- Add one shared Step 11 tier evidence-index schema under `artifacts/perf/step-11/schemas/`.
- Add one co-located tier evidence index per high-tier artifact root.
- Add placeholder `checksum-manifest.txt` and `artifact-file-list.txt` files.
- Keep the state honest: `template_only_pending_execution` and `pending_collection`.

## Changed Files

- `artifacts/perf/step-11/schemas/step-11-tier-evidence-index.schema.json`
- `artifacts/perf/step-11/pre-release/pre-release-tier-evidence-index.json`
- `artifacts/perf/step-11/capacity/capacity-tier-evidence-index.json`
- `artifacts/perf/step-11/pre-release/checksum-manifest.txt`
- `artifacts/perf/step-11/pre-release/artifact-file-list.txt`
- `artifacts/perf/step-11/capacity/checksum-manifest.txt`
- `artifacts/perf/step-11/capacity/artifact-file-list.txt`
- `services/sdkwork-im-cloud-gateway/tests/performance_drill_catalog_test.rs`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_co_locates_step11_tier_evidence_indexes_with_artifact_roots -- --exact --nocapture
```

- Failed because `artifacts/perf/step-11/schemas/step-11-tier-evidence-index.schema.json` was missing.

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_co_locates_step11_tier_evidence_indexes_with_artifact_roots -- --exact --nocapture
cargo fmt --all --check
cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
```

## Next Gap

- The machine-readable high-tier contract is now co-located with `artifactRoot`.
- The remaining gap is real collected high-tier evidence, not index discoverability.

