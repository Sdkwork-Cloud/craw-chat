> Migrated from `docs/superpowers/plans/2026-04-16-craw-chat-admin-sdk-standardization.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Sdkwork IM Admin SDK Standardization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Turn `sdkwork-control-plane-sdk` into the single real admin SDK workspace, generate it from a live OpenAPI 3.x schema fetched from the running admin or control-plane service, and migrate `apps/control-plane` to consume the formal SDK directly instead of the handwritten `sdkwork-control-plane-admin-api` boundary.

**Architecture:** `services/control-plane-api` becomes the runtime contract source. `sdks/sdkwork-control-plane-sdk` mirrors the mature app SDK layout with a checked-in normalized authority snapshot under `openapi/`, root generation and verification wrappers under `bin/`, generator-owned `generated/server-openapi` outputs per language, and manual-owned `composed` client packages. `apps/control-plane` removes the handwritten transport package and depends on the formal TypeScript admin SDK for real control-plane access.

**Tech Stack:** Rust, Axum, OpenAPI 3.x, Node.js, PowerShell, shell wrappers, sdkwork-sdk-generator, TypeScript, Flutter/Dart, React, Vite

---

### Task 1: Lock The New Boundary With Failing Tests And Documentation Targets

**Files:**
- Create: `docs/superpowers/specs/2026-04-16-control-plane-sdk-standardization-design.md`
- Create: `docs/superpowers/plans/2026-04-16-control-plane-sdk-standardization.md`
- Modify: `apps/control-plane/tests/admin-sdk-boundary.test.mjs`
- Modify: `apps/control-plane/tests/admin-architecture.test.mjs`
- Modify: `apps/control-plane/tests/admin-product-experience.test.mjs`
- Modify: `apps/control-plane/README.md`
- Modify: `sdks/sdkwork-control-plane-sdk/README.md`

- [ ] **Step 1: Rewrite the admin boundary tests so they describe the desired end state**

The updated tests must assert that:

- `apps/control-plane` depends on the formal admin SDK instead of `sdkwork-control-plane-admin-api`
- business packages still avoid raw `fetch` or raw control-plane URLs
- the admin app documentation points to the formal SDK boundary

- [ ] **Step 2: Run the boundary tests and confirm they fail for the expected reason**

Run:

```powershell
node .\apps\control-plane\tests\admin-sdk-boundary.test.mjs
node .\apps\control-plane\tests\admin-product-experience.test.mjs
```

Expected:

- failure because `sdkwork-control-plane-admin-api` is still referenced
- failure because the formal admin SDK package is not wired yet

- [ ] **Step 3: Replace the placeholder admin SDK workspace README with the professional English root workspace contract**

Document:

- workspace purpose
- runtime schema fetch requirement
- `generated` versus `composed` ownership
- package names for TypeScript and Flutter
- verification entrypoints

- [ ] **Step 4: Update the admin app README so it names the formal admin SDK as the transport boundary**

### Task 2: Expose And Capture The Real Admin OpenAPI Contract

**Files:**
- Modify: `services/control-plane-api/src/lib.rs`
- Modify: `services/control-plane-api/src/main.rs`
- Create: `services/control-plane-api/tests/admin_openapi_contract_test.rs`
- Create: `sdks/sdkwork-control-plane-sdk/openapi/README.md`
- Create: `sdks/sdkwork-control-plane-sdk/openapi/control-plane.openapi.yaml`
- Create: `sdks/sdkwork-control-plane-sdk/openapi/control-plane.sdkgen.yaml`
- Create: `sdks/sdkwork-control-plane-sdk/bin/fetch-openapi-source.mjs`
- Create: `sdks/sdkwork-control-plane-sdk/bin/prepare-openapi-source.mjs`

- [ ] **Step 1: Add a failing Rust regression test for the admin OpenAPI schema endpoint**

The test should prove that the control-plane service exposes an OpenAPI 3.x document that includes the expected admin route groups.

Run:

```powershell
cargo test -p control-plane-api admin_openapi_contract -- --nocapture
```

Expected:

- failure because the runtime schema endpoint or schema payload is missing or incomplete

- [ ] **Step 2: Implement the runtime OpenAPI schema exposure in the control-plane service**

Use `services/control-plane-api/src/lib.rs` and `services/control-plane-api/src/main.rs` as the runtime boundary. Keep the schema faithful to the currently implemented admin routes instead of inventing undocumented operations.

- [ ] **Step 3: Re-run the Rust test and confirm the control-plane service now exposes the expected OpenAPI 3.x schema**

Run:

```powershell
cargo test -p control-plane-api admin_openapi_contract -- --nocapture
```

Expected:

- PASS

- [ ] **Step 4: Add the root admin SDK fetch script that starts or targets the runtime service and captures the live schema**

`bin/fetch-openapi-source.mjs` must:

- resolve the correct runtime entrypoint
- fetch the live admin schema
- validate the response is OpenAPI 3.x
- normalize unstable fields
- write `openapi/control-plane.openapi.yaml`

- [ ] **Step 5: Add the derived sdkgen preparation script and confirm the authority snapshot can be transformed deterministically**

Run:

```powershell
node .\sdks\sdkwork-control-plane-sdk\bin\fetch-openapi-source.mjs
node .\sdks\sdkwork-control-plane-sdk\bin\prepare-openapi-source.mjs
```

Expected:

- both files are produced under `openapi/`
- repeated runs do not rewrite the files unnecessarily

### Task 3: Build The Admin SDK Root Workspace Automation And Assembly Layer

**Files:**
- Create: `sdks/sdkwork-control-plane-sdk/.gitignore`
- Create: `sdks/sdkwork-control-plane-sdk/.sdkwork-assembly.json`
- Create: `sdks/sdkwork-control-plane-sdk/bin/sdk-generator-root.mjs`
- Create: `sdks/sdkwork-control-plane-sdk/bin/assemble-sdk.mjs`
- Create: `sdks/sdkwork-control-plane-sdk/bin/generate-sdk.ps1`
- Create: `sdks/sdkwork-control-plane-sdk/bin/generate-sdk.sh`
- Create: `sdks/sdkwork-control-plane-sdk/bin/verify-sdk.mjs`
- Create: `sdks/sdkwork-control-plane-sdk/bin/verify-sdk.ps1`
- Create: `sdks/sdkwork-control-plane-sdk/bin/verify-sdk.sh`
- Create: `sdks/sdkwork-control-plane-sdk/bin/verify-sdk-automation.mjs`
- Create: `sdks/sdkwork-control-plane-sdk/bin/verify-powershell-wrapper-args.mjs`

- [ ] **Step 1: Add the root admin SDK workspace metadata and ignore rules**

Mirror the proven `sdkwork-im-sdk` conventions for transient `.sdkwork`, package cache, temp, and assembly artifacts.

- [ ] **Step 2: Add generation wrappers that enforce the live-fetch -> authority snapshot -> derived sdkgen -> generation flow**

The wrappers must not treat generated output as source-of-truth.

- [ ] **Step 3: Add automation guardrails that fail when required scripts, docs, or verification links drift**

- [ ] **Step 4: Run the automation verification before any language-specific generation exists and confirm it fails on the missing language workspaces**

Run:

```powershell
node .\sdks\sdkwork-control-plane-sdk\bin\verify-sdk.mjs --language typescript
```

Expected:

- failure because the TypeScript workspace and verification chain are not built yet

### Task 4: Implement The TypeScript Generated And Composed Admin SDK

**Files:**
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/bin/sdk-gen.ps1`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/bin/sdk-gen.sh`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/bin/sdk-assemble.ps1`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/bin/sdk-assemble.sh`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/bin/sdk-verify.ps1`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/bin/sdk-verify.sh`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/generated/server-openapi/*`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/package.json`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/tsconfig.build.json`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/README.md`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/src/index.ts`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/src/sdk.ts`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/src/sdk-context.ts`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/src/types.ts`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/src/generated-backend-types.ts`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/src/auth-module.ts`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/src/operators-module.ts`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/src/portal-users-module.ts`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/src/tenants-module.ts`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/src/projects-module.ts`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/src/api-keys-module.ts`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/src/routing-module.ts`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/src/providers-module.ts`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/src/models-module.ts`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/src/runtime-module.ts`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/src/storage-module.ts`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/composed/test/control-plane-client.test.mjs`
- Create: `sdks/sdkwork-control-plane-sdk/bin/build-typescript-generated-package.mjs`
- Create: `sdks/sdkwork-control-plane-sdk/bin/verify-typescript-generated-package.mjs`
- Create: `sdks/sdkwork-control-plane-sdk/bin/verify-typescript-public-api-boundary.mjs`
- Create: `sdks/sdkwork-control-plane-sdk/bin/verify-typescript-usage-surface.mjs`
- Create: `sdks/sdkwork-control-plane-sdk/bin/verify-typescript-workspace.mjs`

- [ ] **Step 1: Write the failing TypeScript smoke test for `ControlPlaneSdkClient`**

The smoke test should demonstrate:

- flat client creation
- generated transport injection fallback
- semantic module access through the composed client

Run:

```powershell
node .\sdks\sdkwork-control-plane-sdk\sdkwork-control-plane-sdk-typescript\composed\test\control-plane-client.test.mjs
```

Expected:

- failure because the composed client and transport package do not exist yet

- [ ] **Step 2: Generate the TypeScript transport package under `generated/server-openapi`**

- [ ] **Step 3: Implement the composed TypeScript package around `ControlPlaneSdkClient` and the admin domain modules**

Reuse generated models and generated API clients directly. Do not fork DTOs into a second handwritten tree.

- [ ] **Step 4: Add the TypeScript verification chain and make it part of the root workspace verification**

- [ ] **Step 5: Run the TypeScript workspace verification until the generated and composed package both pass**

Run:

```powershell
node .\sdks\sdkwork-control-plane-sdk\bin\verify-sdk.mjs --language typescript
```

Expected:

- PASS

### Task 5: Implement The Flutter Generated And Composed Admin SDK

**Files:**
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/bin/sdk-gen.ps1`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/bin/sdk-gen.sh`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/bin/sdk-assemble.ps1`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/bin/sdk-assemble.sh`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/bin/sdk-verify.ps1`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/bin/sdk-verify.sh`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/generated/server-openapi/*`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/composed/pubspec.yaml`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/composed/pubspec_overrides.yaml`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/composed/README.md`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/composed/lib/control_plane_sdk.dart`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/composed/lib/src/context.dart`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/composed/lib/src/types.dart`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/composed/lib/src/auth_module.dart`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/composed/lib/src/operators_module.dart`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/composed/lib/src/portal_users_module.dart`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/composed/lib/src/tenants_module.dart`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/composed/lib/src/projects_module.dart`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/composed/lib/src/api_keys_module.dart`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/composed/lib/src/routing_module.dart`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/composed/lib/src/providers_module.dart`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/composed/lib/src/models_module.dart`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/composed/lib/src/runtime_module.dart`
- Create: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/composed/lib/src/storage_module.dart`
- Create: `sdks/sdkwork-control-plane-sdk/bin/verify-flutter-generated-models.mjs`
- Create: `sdks/sdkwork-control-plane-sdk/bin/verify-flutter-public-api-boundary.mjs`
- Create: `sdks/sdkwork-control-plane-sdk/bin/verify-flutter-package-metadata.mjs`
- Create: `sdks/sdkwork-control-plane-sdk/bin/verify-flutter-workspace.mjs`

- [ ] **Step 1: Create a failing Flutter parity check for `ControlPlaneSdkClient`**

Run:

```powershell
node .\sdks\sdkwork-control-plane-sdk\bin\verify-sdk.mjs --language flutter
```

Expected:

- failure because the Flutter generated and composed layers are still missing

- [ ] **Step 2: Generate the Flutter transport package under `generated/server-openapi`**

- [ ] **Step 3: Implement the composed Flutter package with `ControlPlaneSdkClient` and aligned domain modules**

- [ ] **Step 4: Add the Flutter verification chain and wire it into the root workspace verification**

- [ ] **Step 5: Run the Flutter workspace verification until the generated and composed package both pass**

Run:

```powershell
node .\sdks\sdkwork-control-plane-sdk\bin\verify-sdk.mjs --language flutter
```

Expected:

- PASS

### Task 6: Migrate The Admin App To The Formal TypeScript Admin SDK

**Files:**
- Modify: `apps/control-plane/package.json`
- Modify: `apps/control-plane/vite.config.ts`
- Modify: `apps/control-plane/tsconfig.json`
- Modify: `apps/control-plane/README.md`
- Modify: `apps/control-plane/tests/admin-sdk-boundary.test.mjs`
- Modify: `apps/control-plane/tests/admin-architecture.test.mjs`
- Modify: `apps/control-plane/tests/admin-product-experience.test.mjs`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-core/package.json`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-core/src/operatorErrorStatus.ts`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-core/src/workbench.tsx`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-core/src/workbenchActions.ts`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-core/src/workbenchSnapshot.ts`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-storage/package.json`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-storage/src/StoragePage.tsx`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-automation/package.json`
- Delete: `apps/control-plane/packages/sdkwork-control-plane-admin-api/package.json`
- Delete: `apps/control-plane/packages/sdkwork-control-plane-admin-api/src/index.ts`
- Delete: `apps/control-plane/packages/sdkwork-control-plane-admin-api/src/storage.ts`
- Delete: `apps/control-plane/packages/sdkwork-control-plane-admin-api/src/transport.ts`

- [ ] **Step 1: Update the admin app boundary tests to expect the formal admin SDK package names and imports**

Run:

```powershell
node .\apps\control-plane\tests\admin-sdk-boundary.test.mjs
```

Expected:

- failure because the admin app still depends on the handwritten package

- [ ] **Step 2: Swap package dependencies, TS path aliases, and Vite aliases from `sdkwork-control-plane-admin-api` to the formal TypeScript admin SDK**

- [ ] **Step 3: Replace the remaining imports in the core and storage packages with the formal SDK entrypoints**

- [ ] **Step 4: Delete the handwritten `sdkwork-control-plane-admin-api` transport package**

- [ ] **Step 5: Run the admin app test and build pipeline until the direct SDK consumption path is green**

Run:

```powershell
pnpm --dir .\apps\control-plane verify
```

Expected:

- PASS

### Task 7: Complete Documentation, Assembly, And Cross-Language Verification

**Files:**
- Modify: `sdks/sdkwork-control-plane-sdk/README.md`
- Modify: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/README.md`
- Modify: `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/README.md`
- Modify: `docs/sites/.vitepress/config.ts`
- Modify: `docs/sites/sdk/index.md`
- Modify: `docs/sites/sdk/language-support.md`
- Create: `docs/sites/sdk/admin-sdk.md`
- Create: `docs/sites/sdk/admin-typescript-sdk.md`
- Create: `docs/sites/sdk/admin-flutter-sdk.md`
- Modify: `docs/sites/tests/docs-runtime.test.mjs`

- [ ] **Step 1: Add docs runtime assertions for the admin SDK pages and package names**

Run:

```powershell
node .\docs\sites\tests\docs-runtime.test.mjs
```

Expected:

- failure because the new admin SDK pages and links do not exist yet

- [ ] **Step 2: Write the docs pages and navigation entries so the admin SDK mirrors the app SDK documentation quality bar**

- [ ] **Step 3: Refresh assembly metadata and workspace READMEs to match the generated and composed package layout**

- [ ] **Step 4: Run the final admin SDK verification for both languages plus the docs and admin app verification**

Run:

```powershell
node .\sdks\sdkwork-control-plane-sdk\bin\verify-sdk.mjs --language typescript --language flutter
node .\docs\sites\tests\docs-runtime.test.mjs
pnpm --dir .\apps\control-plane verify
```

Expected:

- all commands exit with code `0`
- admin SDK root assembly is refreshed
- docs and real consumer integration both validate the final boundary

