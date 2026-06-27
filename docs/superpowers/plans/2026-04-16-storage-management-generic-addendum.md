# Storage Management Generic Execution Addendum

This addendum updates the implementation order for storage management so future work continues from the generic layers instead of rebuilding sdkwork-im-specific storage logic.

## Revised Execution Order

1. Stabilize `im-storage-contracts` as the only shared definition of storage provider schema, scope, config, secret redaction, effective resolution, validation, and audit types.
2. Stabilize `im-storage-runtime` as the reusable Rust orchestration layer for save/delete/resolve/validate/audit operations.
3. Keep `sdkwork-im-contract-admin` as a compatibility re-export surface instead of a second source of truth.
4. Adapt `sdkwork-api-product-runtime` admin sandbox and any future production admin backend to the generic runtime/contracts.
5. Adapt `apps/control-plane` admin API packages, UI modules, and route manifests to consume the generic storage contracts.
6. Align SDK upload flows and storage-management docs to the same storage contract terminology.

## Current Completed Milestones

- Shared storage contract crate created: `crates/im-storage-contracts`
- Shared storage runtime crate created: `crates/im-storage-runtime`
- Admin compatibility re-export added: `crates/sdkwork-im-contract-admin::storage`
- Generic admin sandbox storage wiring added in `crates/sdkwork-api-product-runtime/src/admin_sandbox.rs`
- Dev admin sandbox storage routes added in `apps/control-plane/dev/admin-sandbox.mjs`
- Admin TypeScript storage API/types modules added
- Admin storage UI package, route manifest, and shell registration added in `apps/control-plane/packages/sdkwork-control-plane-storage`
- Mode-aware credential field metadata and validation added across storage contracts and admin form composition
- Shared storage upsert parsing plus provider- and credential-mode-aware validation added to `crates/im-storage-runtime`
- Typed `StorageConfigUpsertInput` request contract added to `crates/im-storage-contracts` so Rust adapters can decode storage admin payloads without ad-hoc JSON maps
- Shared `StorageDomainSnapshotStore` trait added to `crates/im-storage-contracts`
- `crates/im-storage-runtime` now supports `StorageRuntimeState::from_domain_snapshot`, `domain_snapshot`, `load_from_store`, and `save_to_store` as the initial persistence seam for real backends
- Rust admin sandbox storage save routes now consume the shared runtime parser instead of duplicating storage schema logic
- Rust admin sandbox now deserializes shared typed storage input contracts before handing requests to the generic runtime
- JS dev sandbox validation now matches runtime behavior for unsupported credential modes and provider-config payload validation
- Worktree-safe `@sdkwork/ui-pc-react` type resolution restored through `apps/control-plane/tsconfig.json` and `src/types/sdkwork-ui-pc-react-shim.d.ts`
- UI declaration resolution regression coverage added in `apps/control-plane/tests/admin-ui-resolution.test.mjs`

## Remaining High-Value Work

- Replace any remaining provider/credential-based storage handling with storage-specific adapters
- Update the product and API docs to describe the generic storage module architecture
- Add build/dev verification for the admin app once the broader control-plane pipeline is ready
