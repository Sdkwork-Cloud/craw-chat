# Sdkwork IM Admin Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `apps/control-plane` as a standalone IM operator workspace that fully mirrors the `sdkwork-routes-admin` architecture standard while replacing router business modules with professional IM admin modules.

**Architecture:** Start by codifying the target contract with failing architecture tests, then copy the reference workspace skeleton, rename the package graph, and recompose the shell, auth, route manifest, and module surfaces around the IM domain. Keep the root app thin and force all business access through `sdkwork-control-plane-admin-api` and the admin SDK boundary.

**Tech Stack:** React 19, TypeScript, Vite, Tailwind CSS v4, Tauri 2, Node built-in test runner, `@sdkwork/ui-pc-react`, `sdkwork-control-plane-sdk`

---

## File Structure

### Root Workspace

- Create: `apps/control-plane/package.json`
- Create: `apps/control-plane/pnpm-workspace.yaml`
- Create: `apps/control-plane/turbo.json`
- Create: `apps/control-plane/tsconfig.json`
- Create: `apps/control-plane/vite.config.ts`
- Create: `apps/control-plane/README.md`
- Create: `apps/control-plane/src/main.tsx`
- Create: `apps/control-plane/src/App.tsx`
- Create: `apps/control-plane/src/theme.css`
- Create: `apps/control-plane/src/vite-env.d.ts`
- Create: `apps/control-plane/src/types/*.d.ts`

### Foundation Packages

- Create: `apps/control-plane/packages/sdkwork-control-plane-types/**`
- Create: `apps/control-plane/packages/sdkwork-control-plane-core/**`
- Create: `apps/control-plane/packages/sdkwork-control-plane-shell/**`
- Create: `apps/control-plane/packages/sdkwork-control-plane-admin-api/**`
- Create: `apps/control-plane/packages/sdkwork-control-plane-auth/**`

### IM Business Packages

- Create: `apps/control-plane/packages/sdkwork-control-plane-overview/**`
- Create: `apps/control-plane/packages/sdkwork-control-plane-tenants/**`
- Create: `apps/control-plane/packages/sdkwork-control-plane-users/**`
- Create: `apps/control-plane/packages/sdkwork-control-plane-conversations/**`
- Create: `apps/control-plane/packages/sdkwork-control-plane-messages/**`
- Create: `apps/control-plane/packages/sdkwork-control-plane-groups/**`
- Create: `apps/control-plane/packages/sdkwork-control-plane-moderation/**`
- Create: `apps/control-plane/packages/sdkwork-control-plane-automation/**`
- Create: `apps/control-plane/packages/sdkwork-control-plane-announcements/**`
- Create: `apps/control-plane/packages/sdkwork-control-plane-realtime/**`
- Create: `apps/control-plane/packages/sdkwork-control-plane-system/**`
- Create: `apps/control-plane/packages/sdkwork-control-plane-settings/**`

### Host And Verification

- Create: `apps/control-plane/src-tauri/**`
- Create: `apps/control-plane/tests/admin-architecture.test.mjs`
- Create: `apps/control-plane/tests/admin-product-experience.test.mjs`
- Create: `apps/control-plane/tests/admin-auth-surface.test.mjs`
- Create: `apps/control-plane/tests/admin-sdk-boundary.test.mjs`

### Supporting Docs

- Create: `apps/control-plane/docs/superpowers/specs/2026-04-09-control-plane-design.md`
- Create: `apps/control-plane/docs/superpowers/plans/2026-04-09-control-plane-implementation.md`

### Task 1: Lock The Workspace Contract With Failing Tests

**Files:**
- Create: `apps/control-plane/tests/admin-architecture.test.mjs`
- Create: `apps/control-plane/tests/admin-product-experience.test.mjs`
- Create: `apps/control-plane/tests/admin-auth-surface.test.mjs`
- Create: `apps/control-plane/tests/admin-sdk-boundary.test.mjs`

- [ ] **Step 1: Write the failing architecture and product-surface tests**

Add tests that assert:

- root workspace files exist
- IM admin foundation packages exist
- IM business packages exist
- route manifest exposes IM modules instead of router modules
- auth surface uses Sdkwork IM Admin language
- admin SDK boundary forbids raw admin fetch wrappers

- [ ] **Step 2: Run tests to verify they fail**

Run: `node --test apps/control-plane/tests/*.test.mjs`

Expected: FAIL because the workspace files and packages do not exist yet.

- [ ] **Step 3: Keep the failing tests as the contract**

Do not weaken expectations to match the empty workspace.

- [ ] **Step 4: Commit the test contract**

```bash
git add apps/control-plane/tests
git commit -m "test: define sdkwork im admin workspace contract"
```

### Task 2: Materialize The Reference Workspace Skeleton

**Files:**
- Create or modify: `apps/control-plane/package.json`
- Create or modify: `apps/control-plane/pnpm-workspace.yaml`
- Create or modify: `apps/control-plane/turbo.json`
- Create or modify: `apps/control-plane/tsconfig.json`
- Create or modify: `apps/control-plane/vite.config.ts`
- Create or modify: `apps/control-plane/src/**`
- Create or modify: `apps/control-plane/src-tauri/**`
- Create or modify: `apps/control-plane/packages/**`

- [ ] **Step 1: Copy the `sdkwork-routes-admin` workspace into `apps/control-plane`**

Preserve directory topology:

- `src/`
- `packages/`
- `tests/`
- `src-tauri/`

- [ ] **Step 2: Rename package names, imports, route constants, and workspace identifiers**

Replace `sdkwork-routes-admin` with `sdkwork-control-plane` across:

- `package.json`
- `tsconfig.json`
- package manifests
- import specifiers
- Tauri metadata
- README and product identifiers

- [ ] **Step 3: Run the architecture tests again**

Run: `node --test apps/control-plane/tests/admin-architecture.test.mjs`

Expected: still FAIL, but now on missing IM-specific modules and product language rather than missing workspace structure.

- [ ] **Step 4: Commit the skeleton bootstrap**

```bash
git add apps/control-plane
git commit -m "feat: bootstrap sdkwork im admin workspace skeleton"
```

### Task 3: Recompose Core, Shell, And Auth Around The IM Domain

**Files:**
- Modify: `apps/control-plane/packages/sdkwork-control-plane-types/src/index.ts`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-core/src/routes.ts`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-core/src/routePaths.ts`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-core/src/routeManifest.ts`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-core/src/index.tsx`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-shell/src/application/router/AppRoutes.tsx`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-shell/src/components/AppHeader.tsx`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-shell/src/components/Sidebar.tsx`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-auth/src/index.tsx`
- Modify: `apps/control-plane/src/theme.css`

- [ ] **Step 1: Extend the failing tests with IM-specific route and auth expectations**

Assert route keys and labels for:

- overview
- tenants
- users
- conversations
- messages
- groups
- moderation
- automation
- announcements
- realtime
- system
- settings

- [ ] **Step 2: Run the targeted tests to verify red**

Run: `node --test apps/control-plane/tests/admin-auth-surface.test.mjs apps/control-plane/tests/admin-product-experience.test.mjs`

Expected: FAIL because router-specific routes and copy are still present.

- [ ] **Step 3: Implement the minimal IM route manifest, shell labels, and auth copy**

Keep:

- thin root app
- lazy shell route loading
- isolated auth routes
- route-manifest-driven navigation

Replace:

- router-specific navigation groups
- router-specific header branding
- login messaging and auth copy

- [ ] **Step 4: Re-run the targeted tests**

Run: `node --test apps/control-plane/tests/admin-auth-surface.test.mjs apps/control-plane/tests/admin-product-experience.test.mjs`

Expected: PASS.

- [ ] **Step 5: Commit the shell and auth conversion**

```bash
git add apps/control-plane
git commit -m "feat: convert admin shell and auth to sdkwork im domain"
```

### Task 4: Stand Up IM Product Modules With Operator-Grade Surfaces

**Files:**
- Modify: `apps/control-plane/packages/sdkwork-control-plane-overview/src/index.tsx`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-tenants/src/index.tsx`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-users/src/index.tsx`
- Create or modify: `apps/control-plane/packages/sdkwork-control-plane-conversations/src/index.tsx`
- Create or modify: `apps/control-plane/packages/sdkwork-control-plane-messages/src/index.tsx`
- Create or modify: `apps/control-plane/packages/sdkwork-control-plane-groups/src/index.tsx`
- Create or modify: `apps/control-plane/packages/sdkwork-control-plane-moderation/src/index.tsx`
- Create or modify: `apps/control-plane/packages/sdkwork-control-plane-automation/src/index.tsx`
- Create or modify: `apps/control-plane/packages/sdkwork-control-plane-announcements/src/index.tsx`
- Create or modify: `apps/control-plane/packages/sdkwork-control-plane-realtime/src/index.tsx`
- Create or modify: `apps/control-plane/packages/sdkwork-control-plane-system/src/index.tsx`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-settings/src/index.tsx`

- [ ] **Step 1: Strengthen the product-surface tests**

Write assertions that the pages expose operator-grade IM concepts:

- throughput and hotspots on overview
- tenant governance
- user/client route posture
- conversation lifecycle
- message audit
- moderation queues
- automation runs
- realtime session posture
- protocol governance and compatibility matrix

- [ ] **Step 2: Run the product-surface tests to verify red**

Run: `node --test apps/control-plane/tests/admin-product-experience.test.mjs`

Expected: FAIL because the copied router pages still expose router business language.

- [ ] **Step 3: Implement the minimal IM module pages**

Each module should render credible operator surfaces, not placeholder headings only.

- [ ] **Step 4: Re-run the product-surface tests**

Run: `node --test apps/control-plane/tests/admin-product-experience.test.mjs`

Expected: PASS.

- [ ] **Step 5: Commit the IM module surface pass**

```bash
git add apps/control-plane
git commit -m "feat: add first-pass im admin product modules"
```

### Task 5: Enforce The Admin SDK Boundary And Verification Flow

**Files:**
- Modify: `apps/control-plane/packages/sdkwork-control-plane-admin-api/src/index.ts`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-admin-api/src/transport.ts`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-core/src/workbench.tsx`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-core/src/workbenchActions.ts`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-core/src/workbenchSnapshot.ts`
- Modify: `apps/control-plane/tests/admin-sdk-boundary.test.mjs`

- [ ] **Step 1: Write failing SDK boundary assertions**

Assert that:

- business packages do not call raw admin URLs
- `admin-api` is the single control-plane access package inside the workspace
- package source references the IM admin SDK boundary

- [ ] **Step 2: Run the SDK boundary tests to verify red**

Run: `node --test apps/control-plane/tests/admin-sdk-boundary.test.mjs`

Expected: FAIL until `admin-api` and workbench files stop carrying router-admin references and raw transport assumptions.

- [ ] **Step 3: Implement the minimal admin API boundary**

Keep transport and service seams clean enough that future SDK wiring can slot in without product-package rewrites.

- [ ] **Step 4: Run the complete targeted verification set**

Run: `node --test apps/control-plane/tests/*.test.mjs`

Expected: PASS.

- [ ] **Step 5: Run typecheck and build**

Run: `pnpm --dir apps/control-plane typecheck`

Expected: PASS.

Run: `pnpm --dir apps/control-plane build`

Expected: PASS.

- [ ] **Step 6: Commit the admin API boundary and final verification pass**

```bash
git add apps/control-plane
git commit -m "feat: finalize sdkwork im admin workspace"
```
