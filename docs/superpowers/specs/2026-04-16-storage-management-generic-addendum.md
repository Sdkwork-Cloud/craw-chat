# Storage Management Generic Architecture Addendum

This addendum supersedes the sdkwork-im-specific storage ownership assumption in the earlier design note.

## Updated Ownership

- `im-storage-contracts` is the canonical shared contract layer for storage scope, provider schema, binding, config, secret redaction, effective resolution, validation, and audit records.
- `im-storage-runtime` is the canonical reusable Rust runtime layer for storage save/delete flows, tenant override fallback, validation, and audit recording.
- `im-storage-runtime` also owns adapter-facing JSON upsert parsing plus provider- and credential-mode-aware validation so admin/product entry points do not duplicate storage schema rules.
- Shared storage HTTP/input payloads should be expressed as typed storage contract structs instead of ad-hoc JSON maps wherever Rust backends or adapters own request decoding.
- Shared storage persistence boundaries should be expressed through generic storage snapshot contracts and store traits so future admin/control-plane backends can hydrate runtime state without re-encoding provider rules.
- `sdkwork-im-contract-admin` is an adapter entry point that re-exports the generic storage contracts for admin consumers that still anchor on the sdkwork-im admin package.
- `apps/control-plane` is a consumer of the storage domain through typed API and view-model adapters. It does not define a separate storage model.
- `sdkwork-api-product-runtime` and any future admin/control-plane backends should consume the generic runtime/contracts instead of encoding storage rules directly in sandbox or product-specific handlers.

## Updated Boundary Rules

- Storage is a first-class reusable platform module, not a branch of generic provider credentials.
- Tenant override resolution remains whole-record replacement only. No field-level merge is allowed.
- Provider schemas remain provider-specific and must not be flattened into a fake one-size-fits-all credential form.
- Credential field contracts may be mode-aware. A provider schema must be able to say which credential fields apply to which credential modes instead of exposing a flat credential field bag with ambiguous required flags.
- Unsupported provider/credential-mode combinations must fail explicitly with provider-specific errors instead of silently skipping required-field validation.
- Read paths must expose only redacted secret summaries, never raw secret payloads.
- Application-specific admin routes and SDK helpers are adapter surfaces that consume the generic storage contracts.
- Worktree-scoped admin apps that consume `@sdkwork/ui-pc-react` must resolve against the current `dist` declaration layout (`dist/index.d.ts`, `dist/theme/index.d.ts`, `dist/*`) rather than legacy `dist/src/*` paths.

## Initial Implementation Baseline

The current implementation baseline in this branch is:

- `crates/im-storage-contracts`
- `crates/im-storage-runtime`
- `crates/sdkwork-im-contract-admin` re-export compatibility for `storage`
- `crates/sdkwork-api-product-runtime/src/admin_sandbox.rs` generic storage sandbox wiring
- `apps/control-plane/dev/admin-sandbox.mjs` storage route support
- `apps/control-plane/packages/sdkwork-control-plane-types/src/storage.ts`
- `apps/control-plane/packages/sdkwork-control-plane-admin-api/src/storage.ts`
- `apps/control-plane/packages/sdkwork-control-plane-storage` UI package and route registration
- Mode-aware credential field schema added across Rust contracts, sandbox catalogs, and admin form logic
- Shared mode-aware secret validation and API payload parsing added to `crates/im-storage-runtime`, with Rust admin sandbox refactored to consume that shared entry point
- Typed `StorageConfigUpsertInput` contract added to `crates/im-storage-contracts`, matching the admin TypeScript storage input shape and giving Rust backends a reusable request model
- `StorageDomainSnapshotStore` plus runtime snapshot import/export helpers now form the first reusable persistence seam for generic storage state
- JS dev admin sandbox validation updated to match shared runtime semantics for unsupported credential modes and provider-config payload shape
- `apps/control-plane/tests/admin-ui-resolution.test.mjs` regression coverage for worktree UI declaration resolution

The remaining work is to finish persistence adapters, tenant/global configuration management UX refinement, SDK upload integration, and public documentation for the generic storage module.
