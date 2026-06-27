> Migrated from `docs/review/continuous-optimization-principal-profile-external-catalog-missing-catalog-unavailable-contract-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: User-Module External Missing-Catalog Unavailable Contract

## Context

- Current loop: continue provider/runtime alignment after the `principal-profile-external-catalog` default bootstrap fix.
- Surface: default runtime bootstrap for `SDKWORK_IM_PRINCIPAL_PROFILE_PROVIDER=external`
- Contract: external provider config errors must be explicitly exposed instead of crashing the whole app before business and ops surfaces can react.

## Confirmed Bug

- `SDKWORK_IM_PRINCIPAL_PROFILE_PROVIDER=external` without `SDKWORK_IM_PRINCIPAL_PROFILE_EXTERNAL_CATALOG_PATH` panicked during app assembly.
- The same provider already modeled unreadable or invalid catalogs as `ContractError::Unavailable` and `ProviderHealthSnapshot.status=unavailable`.
- That created a split-brain failure model:
  - missing catalog path -> startup panic
  - unreadable/invalid catalog file -> structured unavailable provider

## Why It Matters

- A leading operator surface should preserve boot and expose explicit unavailability, not turn a single external-directory config miss into a process crash.
- The repo already maps `ContractError::Unavailable -> 503 provider_unavailable`.
- The missing-path branch bypassed that standard and made the runtime less observable than the rest of the provider system.

## Decision

- Keep `SDKWORK_IM_PRINCIPAL_PROFILE_PROVIDER=local|external` unchanged.
- Keep invalid provider mode values as fast rejection.
- Only convert the missing external catalog-path branch into an `UnavailablePrincipalProfileProvider`.
- Reuse the existing `provider_unavailable` API path instead of inventing a new error family.

## Changed Files

- `services/sdkwork-im-cloud-gateway/src/node/principal_profile.rs`
- `services/sdkwork-im-cloud-gateway/tests/principal_profile_provider_runtime_selection_test.rs`
- `docs/review/continuous-optimization-principal-profile-external-catalog-missing-catalog-unavailable-contract-2026-04-09.md`
- `docs/step/continuous-optimization-principal-profile-external-catalog-missing-catalog-unavailable-contract-2026-04-09.md`
- `docs/架构/09BJ-principal-profile-external-catalog-missing-catalog-unavailable-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150BJ-principal-profile-external-catalog-missing-catalog-unavailable-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline missing_catalog_path_and_returns_provider_unavailable -- --nocapture
```

- Failed before the patch because app assembly panicked at:
  - `SDKWORK_IM_PRINCIPAL_PROFILE_EXTERNAL_CATALOG_PATH is required when SDKWORK_IM_PRINCIPAL_PROFILE_PROVIDER=external`

Green:

```powershell
cargo test -p sdkwork-im-cloud-gateway --offline missing_catalog_path_and_returns_provider_unavailable -- --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline --test principal_profile_provider_runtime_selection_test -- --nocapture
cargo test -p sdkwork-im-cloud-gateway --offline -- --nocapture
cargo fmt --all --check
```

## Remaining Gap

- Missing catalog path now respects unavailable semantics.
- Remaining follow-up items are:
  - decide whether invalid provider mode should stay panic-fast or also become structured bootstrap rejection
  - expose principal-profile provider health through an operator-facing surface
  - continue deeper provider/runtime work beyond principal-profile

