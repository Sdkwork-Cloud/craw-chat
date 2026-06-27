# Storage Management Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add first-class storage configuration management for global and tenant object-storage binding, provider-specific schemas, secret handling, validation, and admin UX, while keeping the workspace-local admin sandbox and docs aligned with the production `/backend/v3/api/admin/storage/*` contract.

**Architecture:** Keep the storage domain isolated from generic provider credentials. Model config, secrets, and effective resolution as separate records; expose them through the local admin sandbox first so the admin UI can render against live workspace data; then add a dedicated admin module package and shell route that reads from the admin workspace snapshot. Treat the SDK upload/session alignment as a companion integration so docs, upload helpers, and media terminology stay consistent with the storage contract. The repository does not contain the live production admin backend, so the workspace-local sandbox is the executable verification surface and the production `/backend/v3/api/admin/storage/*` service is the contract consumer.

**Tech Stack:** Rust, Axum, TypeScript, React, Node.js, pnpm, Vite, Node test runner, workspace JSON sandbox, Markdown docs

---

## File Structure

### Existing files to modify

- `crates/sdkwork-im-contract-admin/src/lib.rs`
- `crates/sdkwork-api-product-runtime/src/admin_sandbox.rs`
- `crates/sdkwork-api-product-runtime/src/lib.rs` if the sandbox dispatch needs route plumbing
- `apps/control-plane/dev/admin-sandbox-seed.json`
- `apps/control-plane/packages/sdkwork-control-plane-admin-api/src/index.ts`
- `apps/control-plane/packages/sdkwork-control-plane-types/src/index.ts`
- `apps/control-plane/packages/sdkwork-control-plane-core/src/routes.ts`
- `apps/control-plane/packages/sdkwork-control-plane-core/src/routePaths.ts`
- `apps/control-plane/packages/sdkwork-control-plane-core/src/routeManifest.ts`
- `apps/control-plane/packages/sdkwork-control-plane-core/src/workbenchSnapshot.ts`
- `apps/control-plane/packages/sdkwork-control-plane-core/src/workbench.tsx`
- `apps/control-plane/packages/sdkwork-control-plane-system/src/index.tsx`
- `apps/control-plane/packages/sdkwork-control-plane-shell/src/application/router/AppRoutes.tsx`
- `apps/control-plane/packages/sdkwork-control-plane-shell/src/application/router/routePrefetch.ts`
- `apps/control-plane/package.json`
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/media-upload-runtime.ts`
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/types.ts`
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md`
- `docs/sites/.vitepress/api-reference-sidebar.mjs`
- `docs/sites/api-reference/index.md`
- `docs/sites/api-reference/control-plane-api.md`
- `docs/sites/api-reference/control-plane/providers.md`
- `docs/sites/sdk/index.md`
- `docs/sites/sdk/app-sdk.md`
- `docs/sites/sdk/typescript-sdk.md`
- `docs/sites/api-reference/app/media.md`

### New files to create

- `crates/sdkwork-im-contract-admin/src/storage.rs`
- `crates/sdkwork-im-contract-admin/tests/storage_contract_test.rs`
- `crates/sdkwork-api-product-runtime/tests/admin_storage_sandbox_test.rs`
- `apps/control-plane/packages/sdkwork-control-plane-admin-api/src/storage.ts`
- `apps/control-plane/packages/sdkwork-control-plane-types/src/storage.ts`
- `apps/control-plane/packages/sdkwork-control-plane-storage/package.json`
- `apps/control-plane/packages/sdkwork-control-plane-storage/src/index.tsx`
- `apps/control-plane/packages/sdkwork-control-plane-storage/src/providerSchemas.ts`
- `apps/control-plane/packages/sdkwork-control-plane-storage/src/storagePageModel.ts`
- `apps/control-plane/tests/admin-storage-routing.test.mjs`
- `apps/control-plane/tests/admin-storage-page.test.mjs`
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/test/media-upload-runtime.test.ts`
- `docs/sites/api-reference/control-plane/storage.md`
- `docs/sites/api-reference/operations/control-plane/storage/get-storage-providers.md`
- `docs/sites/api-reference/operations/control-plane/storage/get-storage-config.md`
- `docs/sites/api-reference/operations/control-plane/storage/upsert-storage-config.md`
- `docs/sites/api-reference/operations/control-plane/storage/validate-storage-config.md`

---

### Task 1: Define The Storage Contract And Visibility Model

**Files:**
- Create: `crates/sdkwork-im-contract-admin/src/storage.rs`
- Modify: `crates/sdkwork-im-contract-admin/src/lib.rs`
- Create: `crates/sdkwork-im-contract-admin/tests/storage_contract_test.rs`

- [ ] **Step 1: Write the failing contract tests**

Add tests that require:
- the official provider list includes all object-storage providers already enumerated by the registry
- global and tenant scopes resolve through `tenant override -> global fallback`
- config payloads and audit summaries do not expose raw secret values
- provider-specific schema metadata can be enumerated without binding to a specific UI

- [ ] **Step 2: Run the contract test and confirm the gap**

Run:

```bash
cargo test -p sdkwork-im-contract-admin storage_contract_test -- --exact
```

Expected: FAIL because the storage contract types and helpers do not exist yet.

- [ ] **Step 3: Implement the storage domain types**

Add a dedicated storage module that defines:
- scope types and scope identifiers
- provider schema descriptors
- storage binding records
- storage config records
- storage secret records
- effective config views
- validation result views
- audit summary records

Keep the storage domain separate from generic provider credentials so the new admin surface has one clear contract.

- [ ] **Step 4: Re-run the contract test**

Run:

```bash
cargo test -p sdkwork-im-contract-admin storage_contract_test -- --exact
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/sdkwork-im-contract-admin/src/storage.rs crates/sdkwork-im-contract-admin/src/lib.rs crates/sdkwork-im-contract-admin/tests/storage_contract_test.rs
git commit -m "feat(storage): define admin storage contract"
```

### Task 2: Add Workspace Admin Storage API And Sandbox Persistence

**Files:**
- Modify: `crates/sdkwork-api-product-runtime/src/admin_sandbox.rs`
- Create: `crates/sdkwork-api-product-runtime/tests/admin_storage_sandbox_test.rs`
- Modify: `apps/control-plane/dev/admin-sandbox-seed.json`
- Create: `apps/control-plane/packages/sdkwork-control-plane-admin-api/src/storage.ts`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-admin-api/src/index.ts`
- Create: `apps/control-plane/packages/sdkwork-control-plane-types/src/storage.ts`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-types/src/index.ts`

- [ ] **Step 1: Write the failing sandbox and client tests**

Add tests that require:
- `GET /backend/v3/api/admin/storage/providers`
- `GET /backend/v3/api/admin/storage/config`
- `POST /backend/v3/api/admin/storage/config`
- `GET /backend/v3/api/admin/storage/config/tenants/{tenantId}`
- `POST /backend/v3/api/admin/storage/config/tenants/{tenantId}`
- `GET /backend/v3/api/admin/storage/effective/tenants/{tenantId}`
- `POST /backend/v3/api/admin/storage/validate`
- `GET /backend/v3/api/admin/storage/audit`

The tests should also verify:
- a tenant override hides the global config as the effective source
- removing the tenant override falls back to global
- secret fields are masked in read responses
- validation failures are surfaced with a stable stage and message

- [ ] **Step 2: Run the sandbox test and confirm the gap**

Run:

```bash
cargo test -p sdkwork-api-product-runtime admin_storage_sandbox_test -- --exact
```

Expected: FAIL because the storage routes and persistence fields are not wired yet.

- [ ] **Step 3: Implement the sandbox storage store and API client methods**

Add workspace-local sandbox handling for the storage routes and mirror those endpoints in the admin API package. Keep the sandbox persistence in the JSON store so the admin UI and tests can exercise the complete flow without hand-written HTTP fallbacks.

Make the typed client expose explicit storage helpers instead of forcing storage management through generic provider/credential calls.

- [ ] **Step 4: Re-run the sandbox test**

Run:

```bash
cargo test -p sdkwork-api-product-runtime admin_storage_sandbox_test -- --exact
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/sdkwork-api-product-runtime/src/admin_sandbox.rs crates/sdkwork-api-product-runtime/tests/admin_storage_sandbox_test.rs apps/control-plane/dev/admin-sandbox-seed.json apps/control-plane/packages/sdkwork-control-plane-admin-api/src/storage.ts apps/control-plane/packages/sdkwork-control-plane-admin-api/src/index.ts apps/control-plane/packages/sdkwork-control-plane-types/src/storage.ts apps/control-plane/packages/sdkwork-control-plane-types/src/index.ts
git commit -m "feat(admin): add storage sandbox api"
```

### Task 3: Add A Dedicated Admin Storage Module And Route

**Files:**
- Modify: `apps/control-plane/packages/sdkwork-control-plane-core/src/routes.ts`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-core/src/routePaths.ts`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-core/src/routeManifest.ts`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-core/src/workbenchSnapshot.ts`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-core/src/workbench.tsx`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-system/src/index.tsx`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-shell/src/application/router/AppRoutes.tsx`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-shell/src/application/router/routePrefetch.ts`
- Modify: `apps/control-plane/package.json`
- Create: `apps/control-plane/packages/sdkwork-control-plane-storage/package.json`
- Create: `apps/control-plane/packages/sdkwork-control-plane-storage/src/index.tsx`
- Create: `apps/control-plane/tests/admin-storage-routing.test.mjs`

- [ ] **Step 1: Write the failing routing and module-loading assertions**

Add tests that require:
- a new `storage` route key and path
- a new product module manifest entry for storage management
- shell prefetch and router loading for the storage package
- the system page to expose a storage shortcut instead of burying the feature in generic settings

- [ ] **Step 2: Run the routing test and confirm the gap**

Run:

```bash
node apps/control-plane/tests/admin-storage-routing.test.mjs
```

Expected: FAIL because the storage route and package do not exist yet.

- [ ] **Step 3: Add the storage package and wire it into the shell**

Create a dedicated storage admin package with one page entry point and register it in the app shell, route manifest, and prefetch map. Keep the storage module separate from `system` so the storage workflow stays first-class and easier to maintain.

- [ ] **Step 4: Re-run the routing test and typecheck**

Run:

```bash
node apps/control-plane/tests/admin-storage-routing.test.mjs
pnpm --dir apps/control-plane typecheck
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add apps/control-plane/packages/sdkwork-control-plane-core/src/routes.ts apps/control-plane/packages/sdkwork-control-plane-core/src/routePaths.ts apps/control-plane/packages/sdkwork-control-plane-core/src/routeManifest.ts apps/control-plane/packages/sdkwork-control-plane-core/src/workbenchSnapshot.ts apps/control-plane/packages/sdkwork-control-plane-core/src/workbench.tsx apps/control-plane/packages/sdkwork-control-plane-system/src/index.tsx apps/control-plane/packages/sdkwork-control-plane-shell/src/application/router/AppRoutes.tsx apps/control-plane/packages/sdkwork-control-plane-shell/src/application/router/routePrefetch.ts apps/control-plane/package.json apps/control-plane/packages/sdkwork-control-plane-storage
git commit -m "feat(admin): add storage navigation"
```

### Task 4: Build The Storage Management Page And Provider-Specific Forms

**Files:**
- Create: `apps/control-plane/packages/sdkwork-control-plane-storage/src/providerSchemas.ts`
- Create: `apps/control-plane/packages/sdkwork-control-plane-storage/src/storagePageModel.ts`
- Create: `apps/control-plane/packages/sdkwork-control-plane-storage/src/index.tsx`
- Create: `apps/control-plane/tests/admin-storage-page.test.mjs`

- [ ] **Step 1: Write the failing page-model tests**

Add tests that require:
- provider-specific field groups are generated from schema metadata
- global and tenant scopes are clearly separated in the model
- the effective config preview shows the correct source
- secret payloads remain masked in all summaries and chips
- validation state is explicit and stable across providers

- [ ] **Step 2: Run the page-model test and confirm the gap**

Run:

```bash
node apps/control-plane/tests/admin-storage-page.test.mjs
```

Expected: FAIL because the storage page model and provider schemas are not implemented yet.

- [ ] **Step 3: Implement the storage page and schema model**

Build the page around a provider schema registry so each provider renders the right bucket/container, region, endpoint, and credential mode fields without forcing a fake one-size-fits-all form. Keep the page layout intentional and operator-friendly:
- global config section
- tenant override section
- effective resolution preview
- validation and health panel
- audit trail summary

- [ ] **Step 4: Re-run the page-model test and typecheck**

Run:

```bash
node apps/control-plane/tests/admin-storage-page.test.mjs
pnpm --dir apps/control-plane typecheck
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add apps/control-plane/packages/sdkwork-control-plane-storage/src/providerSchemas.ts apps/control-plane/packages/sdkwork-control-plane-storage/src/storagePageModel.ts apps/control-plane/packages/sdkwork-control-plane-storage/src/index.tsx apps/control-plane/tests/admin-storage-page.test.mjs
git commit -m "feat(admin): build storage management page"
```

### Task 5: Align Media Upload Helpers And SDK Docs To The Storage Contract

**Files:**
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/media-upload-runtime.ts`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/types.ts`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/test/media-upload-runtime.test.ts`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md`
- Modify: `docs/sites/sdk/app-sdk.md`
- Modify: `docs/sites/sdk/typescript-sdk.md`
- Modify: `docs/sites/api-reference/app/media.md`

- [ ] **Step 1: Write the failing SDK helper tests**

Add tests that require the media upload helper to preserve the stable upload-session shape while presenting the storage contract in the docs and examples:
- `assetId`
- `upload.method`
- `upload.url`
- `upload.headers`
- `upload.expiresAt` or equivalent TTL metadata

- [ ] **Step 2: Run the SDK helper test and confirm the gap**

Run:

```bash
npm --prefix sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript run test
```

Expected: FAIL until the helper and docs agree on the storage-session contract.

- [ ] **Step 3: Implement the upload helper and doc alignment**

Update the TypeScript SDK helper to keep the storage upload flow ergonomic, then sync the public README and SDK docs so they describe the same flow the code actually implements.

- [ ] **Step 4: Re-run the SDK helper test and package checks**

Run:

```bash
npm --prefix sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript run test
npm --prefix sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript run typecheck
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/media-upload-runtime.ts sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/types.ts sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/test/media-upload-runtime.test.ts sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md docs/sites/sdk/app-sdk.md docs/sites/sdk/typescript-sdk.md docs/sites/api-reference/app/media.md
git commit -m "docs(sdk): align media upload helpers with storage contract"
```

### Task 6: Publish The Storage Documentation Surface

**Files:**
- Modify: `docs/sites/.vitepress/api-reference-sidebar.mjs`
- Modify: `docs/sites/api-reference/index.md`
- Modify: `docs/sites/api-reference/control-plane-api.md`
- Modify: `docs/sites/api-reference/control-plane/providers.md`
- Create: `docs/sites/api-reference/control-plane/storage.md`
- Create: `docs/sites/api-reference/operations/control-plane/storage/get-storage-providers.md`
- Create: `docs/sites/api-reference/operations/control-plane/storage/get-storage-config.md`
- Create: `docs/sites/api-reference/operations/control-plane/storage/upsert-storage-config.md`
- Create: `docs/sites/api-reference/operations/control-plane/storage/validate-storage-config.md`
- Modify: `docs/sites/sdk/index.md`

- [ ] **Step 1: Write the failing docs assertions**

Add or extend docs checks so the new storage pages and references are required, including:
- control-plane storage overview
- provider list and effective resolution docs
- storage configuration validation docs
- references from the SDK and API index pages

- [ ] **Step 2: Run the docs verifier and confirm the gap**

Run:

```bash
node docs/sites/scripts/verify-sdk-docs.mjs
node sdks/sdkwork-im-sdk/bin/verify-docs-contract-tests.mjs
```

Expected: FAIL until the new storage docs pages exist and are linked.

- [ ] **Step 3: Implement the docs set**

Document the storage domain in the same professional style as the rest of the site:
- what a global config is
- what a tenant override is
- how effective resolution works
- how provider-specific schemas differ
- how validation and health are reported
- how the SDK upload flow consumes the storage contract

- [ ] **Step 4: Re-run the docs verifier**

Run:

```bash
node docs/sites/scripts/verify-sdk-docs.mjs
node sdks/sdkwork-im-sdk/bin/verify-docs-contract-tests.mjs
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add docs/sites/.vitepress/api-reference-sidebar.mjs docs/sites/api-reference/index.md docs/sites/api-reference/control-plane-api.md docs/sites/api-reference/control-plane/providers.md docs/sites/api-reference/control-plane/storage.md docs/sites/api-reference/operations/control-plane/storage/get-storage-providers.md docs/sites/api-reference/operations/control-plane/storage/get-storage-config.md docs/sites/api-reference/operations/control-plane/storage/upsert-storage-config.md docs/sites/api-reference/operations/control-plane/storage/validate-storage-config.md docs/sites/sdk/index.md
git commit -m "docs(api): add storage management reference"
```

### Task 7: Run The Full Storage Baseline

**Files:**
- No new source files required unless a small verification fix is needed after the baseline run

- [ ] **Step 1: Run the contract and sandbox checks**

Run:

```bash
cargo test -p sdkwork-im-contract-admin
cargo test -p sdkwork-api-product-runtime admin_storage_sandbox_test -- --exact
```

- [ ] **Step 2: Run the admin workspace checks**

Run:

```bash
pnpm --dir apps/control-plane typecheck
node apps/control-plane/tests/admin-storage-routing.test.mjs
node apps/control-plane/tests/admin-storage-page.test.mjs
```

- [ ] **Step 3: Run the SDK and docs checks**

Run:

```bash
npm --prefix sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript run test
npm --prefix sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript run typecheck
node docs/sites/scripts/verify-sdk-docs.mjs
node sdks/sdkwork-im-sdk/bin/verify-docs-contract-tests.mjs
```

- [ ] **Step 4: Refresh any baseline docs or audit notes**

If verification exposes a workspace-owned gap, refresh the relevant audit or guidance docs instead of hiding the failure behind a silent fallback.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "test(storage): record storage management baseline"
```

## Notes For The Implementer

- Keep secret payloads masked in every read path, preview, and audit trail.
- Do not reintroduce field-level inheritance; storage overrides are whole-record overrides only.
- Keep the storage module separate from generic provider credentials so the admin UX stays maintainable.
- The workspace-local admin sandbox is the executable contract surface in this repository; the production admin backend must consume the same storage contract and persistence model.
- Do not edit generated SDK transport surfaces manually; keep the SDK upload helper changes inside the handwritten layer and only synchronize docs with the actual package behavior.
