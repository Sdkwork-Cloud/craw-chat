# Sdkwork IM PC Desktop IAM Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `apps/sdkwork-im-pc` as a clean SDKWork-standard Tauri desktop app that integrates appbase IAM auth/user modules and projects verified IAM context into Sdkwork IM SDK clients.

**Architecture:** The desktop app owns only shell, routing, runtime bootstrap, and host integration. IAM UI and user center come from `sdkwork-appbase`; Sdkwork IM business calls go through generated app/IM SDK boundaries with auth/context headers centralized in runtime bootstrap.

**Tech Stack:** Tauri 2, React 19, Vite, TypeScript, appbase IAM runtime/auth/user packages, Sdkwork IM generated TypeScript SDKs, Node contract tests.

---

### Task 1: Standard Desktop Contract Tests

**Files:**
- Create: `apps/sdkwork-im-pc/scripts/desktop-standard-contract.test.mjs`
- Create: `apps/sdkwork-im-pc/tests/runtime-contract.test.mjs`

- [x] Write tests that fail on the current AI Studio scaffold.
- [x] Assert package scripts expose BirdCoder-style desktop commands.
- [x] Assert `src-tauri/tauri.conf.json` exists and uses a fixed localhost dev URL.
- [x] Assert appbase IAM/auth/user workspace packages are referenced.
- [x] Assert old Gemini/AI Studio/mock default path debt is gone.
- [x] Assert runtime projects IAM context into Sdkwork IM headers.
- [x] Assert desktop AppContext projection matches Rust signed AppContext headers.
- [x] Assert desktop mode/env/doctor command matrix is standardized.
- [x] Assert runtime behavior for token headers, AppContext headers, storage, and IAM adapter request flow.
- [x] Assert Sdkwork IM Rust services do not expose SDKWork IAM login/session/current-user routes and keep signed AppContext as the integration boundary.
- [x] Assert package exports and component specs declare the runtime quick-integration entrypoints.
- [x] Assert desktop entrypoint validates `#root` and wraps the app in a standardized error boundary.
- [x] Assert raw HTTP stays inside the IAM authority adapter and token header construction stays inside the AppContext bridge.
- [x] Assert the generated IM SDK receives the same signed AppContext header provider used by the Rust boundary.

### Task 2: Desktop Workspace and Tauri Host

**Files:**
- Modify: `apps/sdkwork-im-pc/package.json`
- Create: `apps/sdkwork-im-pc/pnpm-workspace.yaml`
- Modify: `apps/sdkwork-im-pc/vite.config.ts`
- Modify: `apps/sdkwork-im-pc/tsconfig.json`
- Create: `apps/sdkwork-im-pc/src-tauri/Cargo.toml`
- Create: `apps/sdkwork-im-pc/src-tauri/src/main.rs`
- Create: `apps/sdkwork-im-pc/src-tauri/tauri.conf.json`

- [x] Replace scaffold scripts and dependencies with standard desktop scripts.
- [x] Add workspace references to appbase IAM/auth/user and Sdkwork IM SDKs.
- [x] Configure Vite aliases for appbase and generated SDK source packages.
- [x] Configure minimal Tauri app metadata and build settings.
- [x] Isolate the Tauri host from the Sdkwork IM Rust workspace with an explicit `[workspace]`.
- [x] Add a minimal Tauri capability file for the main window with only `core:default` and `shell:allow-open`.

### Task 3: Runtime Bootstrap

**Files:**
- Create: `apps/sdkwork-im-pc/src/runtime/appContextBridge.ts`
- Create: `apps/sdkwork-im-pc/src/runtime/tokenStore.ts`
- Create: `apps/sdkwork-im-pc/src/runtime/iamRuntime.ts`
- Create: `apps/sdkwork-im-pc/src/runtime/SdkworkImRuntime.ts`
- Create: `apps/sdkwork-im-pc/src/runtime/index.ts`

- [x] Implement pure AppContext-to-header projection.
- [x] Implement localStorage-backed token store for browser/Tauri renderer.
- [x] Implement localStorage-backed AppContext store for browser/Tauri renderer.
- [x] Create IAM runtime using appbase service/runtime APIs.
- [x] Create Sdkwork IM runtime header provider without duplicating token logic.
- [x] Inject the signed AppContext header provider into the generated IM SDK during Sdkwork IM runtime bootstrap.
- [x] Make runtime env access safe outside Vite for behavior tests and future host reuse.
- [x] Make IAM runtime locale access safe outside browser hosts.
- [x] Keep all manual header construction inside runtime bootstrap only.

### Task 4: App Shell and Routes

**Files:**
- Modify: `apps/sdkwork-im-pc/src/App.tsx`
- Modify: `apps/sdkwork-im-pc/src/main.tsx`
- Modify: `apps/sdkwork-im-pc/src/index.css`

- [x] Add auth routes using appbase `SdkworkIamAuthRoutes`.
- [x] Add user center route using appbase `UserCenterPage`.
- [x] Protect chat/admin routes behind runtime session checks.
- [x] Resolve auth state asynchronously so any appbase token store implementation is supported.
- [x] Keep shell focused on routing/providers only.

### Task 5: Debt Removal

**Files:**
- Modify: `apps/sdkwork-im-pc/.env.example`
- Modify: `apps/sdkwork-im-pc/README.md`
- Delete or disconnect default mock/scaffold entry files where safe.

- [x] Remove Gemini and AI Studio copy/env.
- [x] Remove mock API helpers from default runtime path.
- [x] Document standard desktop commands and IAM modes.
- [x] Add machine-readable SDKWork component specs for the React/Tauri desktop assembly and Tauri host.
- [x] Document the Rust IAM boundary verification command and responsibility split.
- [x] Document the `@sdkwork/sdkwork-im-pc-desktop/runtime` quick-integration entrypoint for appbase composition.
- [x] Add a lightweight desktop error boundary and explicit root-element startup guard.

### Task 6: Verification

**Commands:**
- `node apps/sdkwork-im-pc/scripts/desktop-standard-contract.test.mjs`
- `node apps/sdkwork-im-pc/tests/runtime-contract.test.mjs`
- `npm run lint --prefix apps/sdkwork-im-pc`
- `npm run verify --prefix apps/sdkwork-im-pc`
- `cargo check --manifest-path apps/sdkwork-im-pc/src-tauri/Cargo.toml`

- [x] Run contract tests.
- [x] Run runtime behavior tests.
- [x] Run Sdkwork IM runtime authorization header behavior tests.
- [x] Run IAM runtime behavior tests.
- [x] Run user center IAM-backed service behavior tests.
- [x] Run Rust IAM boundary contract tests.
- [x] Run TypeScript typecheck.
- [x] Run Vite production build.
- [x] Run Tauri Rust host cargo check.
- [x] Fix all failures or report exact blockers.

**Verification status:**

- `npm.cmd run verify --prefix apps/sdkwork-im-pc` passes.
- `cargo check --manifest-path apps/sdkwork-im-pc/src-tauri/Cargo.toml` passes.
- `cargo test --manifest-path apps/sdkwork-im-pc/src-tauri/Cargo.toml` passes.
