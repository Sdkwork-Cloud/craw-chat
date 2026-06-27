# Storage Management

The repository has already moved the core storage-configuration contract and runtime logic onto a
reusable module instead of keeping it as `sdkwork-im`-specific admin glue. What remains is broader
production adoption on top of the shared boundary.

The goal is a single contract and runtime boundary that can be consumed by:

- admin UI and admin APIs
- app and control-plane backends
- media and upload issuance flows
- future Rust services and SDK-oriented upload helpers

## Current Module Boundary

| Layer | Responsibility |
| --- | --- |
| `crates/im-storage-contracts` | Shared storage catalog, provider schema, typed upsert input, redacted secret summary, effective resolution, domain snapshot, and store trait |
| `crates/im-storage-runtime` | Save, delete, validate, resolve, audit, snapshot import/export, and store-backed hydration logic |
| `crates/sdkwork-im-contract-admin` | Compatibility re-export for admin consumers that still anchor on the sdkwork-im admin contract surface |
| `crates/sdkwork-api-product-runtime` | Current Rust admin sandbox entry point that now delegates storage parsing and validation to the generic runtime |
| `apps/sdkwork-im-admin` | Consumer-facing admin API types, form composition, and route wiring for storage management |
| `adapters/local-memory` | In-memory adapter baseline, including `MemoryStorageDomainSnapshotStore` for generic storage runtime hydration |
| `adapters/local-disk` | File-backed reference adapter, including `FileStorageDomainSnapshotStore` for durable local persistence and runtime rehydration |

This split matters because provider schema, validation rules, and tenant fallback rules now live in
one place instead of being duplicated across sandbox code, frontend form logic, and future backend
handlers.

## Resolution And Override Rules

The storage module currently follows two rules that downstream services must preserve:

1. Global configuration is the default baseline for a storage domain such as `object-storage`.
2. Tenant configuration is a whole-record override. If a tenant record exists, it fully replaces
   the global record for that tenant. If it does not exist, resolution falls back to global.

That behavior is expressed by `StorageRuntimeState` and `StorageDomainSnapshot::effective_config`.
Consumers should not reimplement field-by-field merge logic.

## Secret Handling Contract

Storage writes accept typed secret input through `StorageConfigUpsertInput`. Stored secrets remain in
`StorageSecretRecord`, but read-facing responses expose only `StorageSecretSummary`.

That means:

- read APIs can show whether credentials are configured
- audit and diagnostics can expose the credential mode and fingerprint
- admin and SDK consumers never need raw secret material on read paths

Any future backend API or SDK surface should keep this redaction rule intact.

## Persistence Seam

The current reusable persistence boundary is intentionally small:

| Type | Purpose |
| --- | --- |
| `StorageDomainSnapshot` | Durable domain payload containing catalog, bindings, configs, and secrets |
| `StorageDomainSnapshotStore` | Store trait for loading or saving a domain snapshot |
| `StorageRuntimeState::load_from_store(...)` | Hydrate runtime state from a store-backed snapshot |
| `StorageRuntimeState::save_to_store(...)` | Persist current runtime state through the shared store boundary |
| `StoreBackedStorageRuntime<S>` | Reusable load-mutate-persist manager that keeps store-backed persistence at the runtime edge |

This seam is designed so adapters can be implemented without copying provider-validation or
resolution logic into database, file, or memory layers.

## Current Adapter Status

The storage module now has two concrete reference adapters:

- `MemoryStorageDomainSnapshotStore` in `adapters/local-memory`
- `FileStorageDomainSnapshotStore` in `adapters/local-disk`

Together they establish the baseline semantics that future database-backed adapters must preserve:

- loading an unknown storage domain returns `None`
- saving the same domain overwrites the previous snapshot
- snapshots are isolated by `snapshot.catalog.domain`
- file-backed snapshots survive reopen and process restart

The memory adapter is the fast baseline for tests and sandbox flows. The file adapter is the first
durable reference implementation for local runtime and backend persistence experiments.

## How Other Surfaces Should Consume It

The intended flow for backend and product surfaces is:

1. Decode request payloads into `StorageConfigUpsertInput`.
2. Hand those payloads to `im-storage-runtime` for validation and state mutation.
3. Persist and restore state through `StorageDomainSnapshotStore` adapters.
4. Expose only redacted read models to admin, control-plane, or SDK consumers.

The intended flow for upload and media issuance is:

1. Resolve the effective storage config for the current scope.
2. Use the resolved provider binding and redacted metadata to select the storage adapter.
3. Issue presigned upload or download operations from the selected provider implementation.
4. Return upload-facing metadata such as asset id, object key, and final URL without exposing raw
   credential payloads.

## Current Desktop Sandbox Integration

`sdkwork-api-product-runtime` now supports a file-backed storage-management path for the desktop
admin sandbox.

When these conditions are true:

- admin proxying is disabled
- `SDKWORK_ADMIN_SANDBOX` is enabled
- `SDKWORK_ADMIN_SANDBOX_STORAGE_FILE` points to a JSON snapshot file

the sandbox loads storage state through `FileStorageDomainSnapshotStore`, serves admin storage
routes through `StoreBackedStorageRuntime<AdminSandboxStorageStore>`, and persists storage writes
back to the same snapshot file after global saves, tenant saves, and tenant deletes. When no file
is configured, the same runtime abstraction stays in-memory through an ephemeral store instead of
forking a second storage-management code path.

This does not yet make the sandbox a production control plane, but it does provide a real store-
backed path instead of in-memory-only storage behavior.

## Current Admin Route Contract

Read [Admin Storage Contract](/reference/admin-storage-contract) when you need the actual
`/backend/v3/api/admin/storage/*` route catalog, write payload shape, validation semantics, and sandbox
promotion boundary.

That reference is intentionally separate from the OpenAPI-style API section because the storage
routes are currently implemented as a verified admin contract, not as a published control-plane
OpenAPI authority.

## Current Verified Consumption

The current repository state already verifies four concrete consumption paths:

- `apps/sdkwork-im-admin` exposes a first-class storage-management module with provider-catalog
  loading, tenant override editing, validation, effective fallback inspection, audit review, and
  redacted secret summaries.
- `sdkwork-api-product-runtime` serves the admin sandbox storage routes through the same generic
  runtime and persists mutations through either an in-memory or file-backed store, depending on the
  configured sandbox mode.
- `adapters/local-memory` and `adapters/local-disk` both implement the shared snapshot-store seam
  and are covered by persistence and reopen semantics tests.
- `im-storage-runtime`, `im-storage-contracts`, `sdkwork-im-contract-admin`, and
  `sdkwork-api-product-runtime` all ship tests that verify provider schema shape, effective
  resolution, redaction, validation, audit, and store-backed hydration.

## What Is Still Not Done

The generic storage module is established, but several promotion steps remain:

- a non-sandbox admin or control-plane backend that loads and saves storage state through the
  adapter seam
- a database-backed adapter for production-grade persistence beyond local file-backed storage
- media-service and SDK upload issuance wired to the same effective-config resolution path
- client-facing presigned-upload helpers that turn the resolved storage policy into a polished
  `upload(...)` experience for browser, Node.js, and future multi-language SDKs
- public API documentation for storage admin and upload endpoints once those endpoints are promoted
  from sandbox to production surfaces

## Why This Matters

Without this module boundary, storage configuration becomes fragmented across admin forms, sandbox
logic, backend handlers, and SDK upload helpers. With the current contract and runtime split, the
repository now has a single source of truth for:

- provider families and credential modes
- validation rules
- tenant fallback behavior
- secret redaction
- store-backed runtime hydration

That is the baseline required for a reusable storage component that can later be shared by admin,
Rust backend services, and client-facing SDK upload flows.
