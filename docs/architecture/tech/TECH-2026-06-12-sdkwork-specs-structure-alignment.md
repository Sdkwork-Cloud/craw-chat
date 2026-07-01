> Migrated from `docs/superpowers/plans/2026-06-12-sdkwork-specs-structure-alignment.md` on 2026-06-24.
> Owner: SDKWork maintainers
>
> **Status (2026-06-30):** Structural alignment tasks 1–6 are implemented. Repository standards
> verification passes via `node scripts/run-sdkwork-im-standards-verification.mjs`. Step-11
> performance evidence collection remains a pre-launch operator action (`check:commercial-readiness`
> exit code 2 until tier evidence artifacts are collected).

# SDKWork Specs Structure Alignment Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Align `sdkwork-im` with the new `../sdkwork-specs` workspace and PC application architecture rules without breaking existing package paths.

**Architecture:** Add compatibility-first SDKWORK dictionary files, root capability placeholders, a root `pnpm-workspace.yaml` authority, and a deterministic Node.js static verification command. Preserve current source changes and defer package renaming to a later migration.

**Tech Stack:** Markdown, `pnpm-workspace.yaml`, Node.js ESM static tests, SDKWork repository specs.

---

### Task 1: Repository And App Dictionary Files

**Files:**
- Create: `apis/README.md`
- Create: `jobs/README.md`
- Create: `plugins/README.md`
- Create: `examples/README.md`
- Create: `configs/README.md`
- Create: `tests/README.md`
- Create: `apps/sdkwork-im-pc/AGENTS.md`
- Create: `apps/sdkwork-im-pc/CODEX.md`
- Create: `apps/sdkwork-im-pc/CLAUDE.md`
- Create: `apps/sdkwork-im-pc/GEMINI.md`
- Create: `apps/sdkwork-im-pc/.sdkwork/README.md`
- Create: `apps/sdkwork-im-pc/.sdkwork/.gitignore`
- Create: `apps/sdkwork-im-pc/.sdkwork/skills/README.md`
- Create: `apps/sdkwork-im-pc/.sdkwork/plugins/README.md`
- Modify: `.sdkwork/.gitignore`
- Modify: `.gitignore`

- [x] **Step 1: Add missing directory README placeholders**

Write focused README files that explain purpose, owner, allowed content, forbidden content, related specs, and verification.

- [x] **Step 2: Add PC app-root agent entrypoints**

Add app-root SDKWORK dictionary files that reference `../../../sdkwork-specs` and delegate compatibility shims to `AGENTS.md`.

- [x] **Step 3: Ignore and untrack local SDKWORK cache paths**

Add `.sdkwork/dart/pub-cache/` ignore rules, then remove currently tracked cache files from the Git
index with `git rm --cached` while preserving local files on disk.

### Task 2: Root Workspace Authority

**Files:**
- Create: `pnpm-workspace.yaml`
- Modify: `specs/README.md`
- Modify: `apps/sdkwork-im-pc/specs/README.md`

- [x] **Step 1: Create root `pnpm-workspace.yaml`**

Copy the app-local packages/catalog into a root workspace file and include `apps/sdkwork-im-pc/packages/*`.

- [x] **Step 2: Document compatibility**

Document that the app-local workspace file is retained for existing app-root commands while the root workspace is the repository-level authority.

### Task 3: Static Structure Verification

**Files:**
- Create: `scripts/sdkwork-workspace-structure-standard.test.mjs`
- Modify: `package.json`
- Modify: `scripts/commercial-gates-governance-node-test-catalog.mjs`
- Modify: `scripts/commercial-gates-governance-node-test-catalog.test.mjs`

- [x] **Step 1: Write failing static verification**

Add a Node.js test that checks repository directories, root/app `.sdkwork` dictionaries, spec path resolution, root `pnpm-workspace.yaml` coverage, documented legacy package exceptions, and rejects tracked `.sdkwork/dart/pub-cache` cache state.

- [x] **Step 2: Run the new test and observe failure before implementation is complete**

Run: `node scripts/sdkwork-workspace-structure-standard.test.mjs`
Expected: fail until all required files and docs exist.

Resumed checkpoint note: the current workspace already contains the required files, so the resumed
verification validates the final green state rather than re-running the historical incomplete state.

- [x] **Step 3: Implement required files and script logic**

Complete the files from Tasks 1 and 2 and wire `package.json` script `test:sdkwork-workspace-structure-standard`.

- [x] **Step 4: Run focused verification**

Run: `node scripts/sdkwork-workspace-structure-standard.test.mjs`
Expected: `SDKWork workspace structure standard passed`

Run: `pnpm run test:sdkwork-workspace-structure-standard`
Expected: same pass output through pnpm.

### Task 4: Governance Verification

**Files:**
- Verify: `scripts/run-commercial-gates-governance-node-tests.mjs`
- Verify: `package.json`

- [x] **Step 1: Run catalog test**

Run: `node --test --experimental-test-isolation=none scripts/commercial-gates-governance-node-test-catalog.test.mjs`
Expected: pass with the new workspace structure test in the governed catalog.

- [x] **Step 2: Run new workspace structure test**

Run: `pnpm run test:sdkwork-workspace-structure-standard`
Expected: pass.

- [x] **Step 3: Run affected governance suite**

Run: `pnpm run test:workflow-commercial-gates`
Expected: pass, unless unrelated pre-existing worktree changes surface; report exact output.

## Execution Checkpoint 2026-06-12

Status: Tasks 1 through 4 are implemented and verified in the current worktree. This checkpoint does
not change UI layout, visual styling, runtime behavior, generated SDK output, or package names.

Verified commands:

- `node scripts/sdkwork-workspace-structure-standard.test.mjs`
  - Result: passed with `SDKWork workspace structure standard passed`.
- `pnpm run test:sdkwork-workspace-structure-standard`
  - Result: passed with `SDKWork workspace structure standard passed` through the pnpm script.
- `node --test --experimental-test-isolation=none scripts/commercial-gates-governance-node-test-catalog.test.mjs`
  - Result: passed, 2/2 tests.
- `pnpm run test:workflow-commercial-gates`
  - Result: passed, 39/39 tests.
- `pnpm run check:commercial-readiness`
  - Result: all code/build/test gates reached in this pass succeeded, then the release gate stopped
    on the truthful `Capacity Tier` evidence blocker.

Remaining blocker:

- `artifacts/perf/step-11/capacity/capacity-tier-evidence-index.json` remains
  `template_only_pending_execution` with 7 pending required slots:
  `connection_capacity`, `message_capacity`, `stream_capacity`, `restore_recovery_recovery`,
  `failover_recovery`, `capacity_report`, and `recovery_report`.
- Do not fabricate these artifacts. They must be collected from a real `capacity-dedicated`
  environment and placed under `artifacts/perf/step-11/capacity/`, then refreshed with
  `pnpm run perf:refresh-step-11-capacity-evidence-index` before rerunning
  `pnpm run check:commercial-readiness`.

## Continuation Checkpoint 2026-06-12 - Capacity Release Gate Hardening

Status: commercial readiness capacity assessment now fails closed on the per-slot evidence state,
not only on the index summary state. Missing required capacity evidence blockers include the exact
drop paths from `artifacts/perf/step-11/capacity/capacity-tier-evidence-index.json`, and the release
README documents that `exit code 2` output must point operators to those drop paths. No UI layout,
visual styling, runtime SDK output, or package names were changed.

Verified commands:

- `node --test --experimental-test-isolation=none scripts/release/commercial-readiness.test.mjs`
  - Result: passed, 10/10 tests.
- `node --test --experimental-test-isolation=none scripts/perf/refresh-step-11-capacity-evidence-index.test.mjs`
  - Result: passed, 4/4 tests.
- `node scripts/perf/refresh-step-11-capacity-evidence-index.mjs --dry-run`
  - Result: `template_only_pending_execution`, `collected=0`, `pending=7`, process exit code `2`.

Additional verification note:

- `pnpm.cmd run test:workflow-commercial-gates` passed once with 42/42 tests after the capacity
  blocker-path change. A later rerun became blocked after a local `pnpm install` verification attempt
  removed the app root `node_modules` links and the sandbox prevented a full reinstall of sibling
  workspace packages. The latest failure is dependency-state/environmental: missing `vite`, `tsx`,
  and desktop Tauri CLI links under `apps/sdkwork-im-pc/node_modules`, not a release gate logic
  assertion failure.

## Continuation Checkpoint 2026-06-12 - Dependency State Recovery

Status: the local `apps/sdkwork-im-pc/node_modules` state has been restored enough for the governed
Node contract suite to pass again. Recovery used filtered pnpm install attempts limited to the PC app
workspace plus current-repository Tauri package links for the desktop subpackage. The sandbox still
does not allow the package manager to relink sibling workspace package internals under paths such as
`../sdkwork-aiot/.../node_modules`, so full install-style release checks must account for that
workspace permission boundary.

Verified commands:

- `node --test --experimental-test-isolation=none scripts/dev/sdkwork-im-pc-dev-command.test.mjs scripts/dev/sdkwork-im-pc-sdk-integration.test.mjs scripts/dev/sdkwork-im-sdk-websocket-contract-node.test.mjs`
  - Result: passed, 3/3 tests.
- `pnpm.cmd run test:workflow-commercial-gates`
  - Result: passed, 42/42 tests.

## Continuation Checkpoint 2026-06-12 - Commercial Readiness Install Strategy

Status: commercial readiness no longer runs a mutating full `pnpm install` as its first PC app gate.
The first gate now runs `pnpm install --lockfile-only --frozen-lockfile --ignore-scripts` from
`apps/sdkwork-im-pc`, preserving root workspace and lockfile authority while avoiding sandboxed
cross-repository `node_modules` relinks. The later `pc-lint`, `pc-build`, and contract tests remain
the evidence that installed dependencies are usable. No UI layout, visual styling, runtime SDK
output, or package names were changed.

Implementation notes:

- `scripts/release/commercial-readiness.mjs` uses frozen lockfile verification for `pc-install` and
  keeps `CI=true` plus `npm_config_update_notifier=false` for package script checks.
- Tailwind CSS integration follows `sdkwork-specs/TAILWIND_CSS_INTEGRATION_SPEC.md`: the host shell
  owns the single `@import "tailwindcss"` bootstrap, host-composed feature CSS must not re-bootstrap,
  and Vite must not alias bare specifier `tailwindcss`.
- PC app install/build steps run from `apps/sdkwork-im-pc` so the app-local `.npmrc` is applied during install and build.
- `scripts/dev/run-esbuild-cli.mjs` and `scripts/dev/run-tsx-cli.mjs` route app scripts through
  repository-managed CLI wrappers when local `.bin` shims are unavailable.
- `apps/sdkwork-im-pc/package.json` runs the production server bundle through
  `run-esbuild-cli.mjs` and the QR scan standard contract through `run-tsx-cli.mjs`.

Verified commands:

- `pnpm.cmd install --lockfile-only --frozen-lockfile --ignore-scripts` from
  `apps/sdkwork-im-pc`
  - Result: passed, scoped all 68 workspace projects without rebuilding `node_modules`.
- `node --test --experimental-test-isolation=none scripts/release/commercial-readiness.test.mjs`
  - Result after implementation: passed, 10/10 tests. The new lockfile-only expectation failed
    before the implementation change.
- `pnpm.cmd run lint` from `apps/sdkwork-im-pc`
  - Result: passed after local virtual store repair.
- `node --test --experimental-test-isolation=none scripts/dev/sdkwork-im-pc-sdk-integration.test.mjs`
  - Result after implementation: passed, 1/1 test. The esbuild wrapper expectation failed before the
    package script and wrapper change.
- `pnpm.cmd run build` from `apps/sdkwork-im-pc`
  - Result: passed; Vite built the renderer and `run-esbuild-cli.mjs` emitted `dist/server.cjs`.
- `pnpm.cmd run test:qr-scan-standard` from `apps/sdkwork-im-pc`
  - Result: passed; the TS contract ran through `run-tsx-cli.mjs`.
- `pnpm.cmd run test:workflow-commercial-gates`
  - Result: passed, 42/42 tests.
- `pnpm.cmd run check:commercial-readiness`
  - Result: code/build/test gates passed through `performance-drill-catalog`, then exited with code
    `2` on the truthful `Capacity Tier` evidence blocker.

Remaining blocker:

- `artifacts/perf/step-11/capacity/capacity-tier-evidence-index.json` is still
  `template_only_pending_execution` with 7 missing required evidence paths:
  `artifacts/perf/step-11/capacity/connection/capacity.json`,
  `artifacts/perf/step-11/capacity/message/capacity.json`,
  `artifacts/perf/step-11/capacity/stream/capacity.json`,
  `artifacts/perf/step-11/capacity/restore-recovery/recovery.json`,
  `artifacts/perf/step-11/capacity/failover/recovery.json`,
  `artifacts/perf/step-11/capacity/reports/capacity-report.md`, and
  `artifacts/perf/step-11/capacity/reports/recovery-report.md`.

## Continuation Checkpoint 2026-06-12 - Manifest Path And Notary Contract Alignment

Status: root and PC application manifests now agree that the active PC app root is
`apps/sdkwork-im-pc`. The SDKWORK workspace structure and runtime standard checks assert that
`publish.config.workspaceRoot`, `artifacts.installConfig.metadata.workspaceRoot`, and
`devApp.sourceRoot` all point to that existing path. The notary package type/import repairs keep
the existing JSX classes, layout, and visual styling intact; no generated SDK output was hand-edited.

Implementation notes:

- `sdkwork.app.config.json` and `apps/sdkwork-im-pc/sdkwork.app.config.json` now use
  `apps/sdkwork-im-pc` for the manifest path slots that describe the app workspace/source root.
- `scripts/sdkwork-workspace-structure-standard.test.mjs` verifies both manifests and fails if the
  configured workspace/source roots do not resolve to an existing path.
- `scripts/dev/sdkwork-im-runtime-standard.test.mjs` verifies the same manifest path slots as part
  of the runtime standard contract.
- The notary package repairs add the attachment fields consumed by the current UI, read
  `task.createTime` from the declared task shape, keep `PartyDriveModal` imported from the shared
  component boundary, and replace the placeholder QR grid's render-time randomness with a
  deterministic helper so the contract remains stable without visual redesign.
- The SDKWORK App Manifest v3 validator command recommended by `APP_MANIFEST_SPEC.md` remains
  unavailable in this workspace because `apps/scripts/validate-sdkwork-app-standard-v3.mjs` is not
  present here or in the sibling `E:\sdkwork-space\apps\scripts` path.

Verified commands:

- `pnpm.cmd run test:sdkwork-workspace-structure-standard`
  - Result: passed with `SDKWork workspace structure standard passed`.
- `pnpm.cmd run test:runtime-standard`
  - Result: passed with `sdkwork-im runtime standard contract passed`.
- `pnpm.cmd run test:workflow-commercial-gates`
  - Result: passed, 42/42 governed Node tests.
- `pnpm.cmd run lint` from `apps/sdkwork-im-pc`
  - Result: passed; `run-tsc-cli.mjs --noEmit` completed with exit code 0.
- `pnpm.cmd run test:notary-app-sdk-integration` from `apps/sdkwork-im-pc`
  - Result: passed with `sdkwork chat notary app SDK integration contract passed`.
- `pnpm.cmd run check:commercial-readiness`
  - Result: PC install, lint, build, appbase UI contract, notary integration, QR scan contract,
    dependency management, governed workflow tests, control-plane API tests, commercial gate
    contract, session gateway tests, performance quant baseline, and performance drill catalog
    passed. The command correctly exited with exit code `2` because the Capacity Tier still
    lacks real collected evidence.

Remaining blocker:

- `artifacts/perf/step-11/capacity/capacity-tier-evidence-index.json` remains
  `template_only_pending_execution` with 7 pending required slots:
  `artifacts/perf/step-11/capacity/connection/capacity.json`,
  `artifacts/perf/step-11/capacity/message/capacity.json`,
  `artifacts/perf/step-11/capacity/stream/capacity.json`,
  `artifacts/perf/step-11/capacity/restore-recovery/recovery.json`,
  `artifacts/perf/step-11/capacity/failover/recovery.json`,
  `artifacts/perf/step-11/capacity/reports/capacity-report.md`, and
  `artifacts/perf/step-11/capacity/reports/recovery-report.md`.

## Continuation Checkpoint 2026-06-12 - Verification Command And Desktop Asset Entry Alignment

Status: local repository docs and component specs no longer point at the retired
`apps/sdkwork-im/Cargo.toml` manifest. The workspace structure standard now fails when documented
Cargo manifest paths do not exist, and the documented Rust verification command points to the real
root workspace with `cargo test --workspace`. The desktop asset build entrypoint now targets the
current `apps/sdkwork-im-pc` app root instead of retired `apps/control-plane`,
`apps/sdkwork-im-admin`, or `apps/sdkwork-im-portal` roots.

Implementation notes:

- `scripts/sdkwork-workspace-structure-standard.test.mjs` extracts documented
  `cargo test --manifest-path ...` commands from `README.md`, `specs/README.md`, and
  `specs/component.spec.json`; it fails if any referenced manifest path is missing.
- `README.md`, `specs/README.md`, and `specs/component.spec.json` now document
  `cargo test --workspace` for the repository Rust workspace.
- `scripts/build-sdkwork-im-desktop-assets.mjs` is importable again without retired portal sources
  and builds/checks `apps/sdkwork-im-pc/dist/index.html`.
- `scripts/build-sdkwork-im-desktop-assets.test.mjs` proves the desktop asset script is aligned to
  `apps/sdkwork-im-pc`, rejects retired app-root references in that script, and validates the
  expected PC dist readiness check.
- `scripts/commercial-gates-governance-node-test-catalog.mjs` includes the new desktop asset
  contract test so the governed workflow suite keeps this entrypoint covered.

TDD evidence:

- `node scripts\sdkwork-workspace-structure-standard.test.mjs` failed first on the three retired
  `apps/sdkwork-im/Cargo.toml` references, then passed after the docs/spec command update.
- `node --test --experimental-test-isolation=none scripts\build-sdkwork-im-desktop-assets.test.mjs`
  failed first because `scripts/build-sdkwork-im-desktop-assets.mjs` referenced retired app roots and
  could not import `apps/sdkwork-im-portal/scripts/lib/build-dist.mjs`, then passed after the script
  was realigned to `apps/sdkwork-im-pc`.

Verified commands:

- `node --test --experimental-test-isolation=none scripts\build-sdkwork-im-desktop-assets.test.mjs`
  - Result: passed, 2/2 tests.
- `node --test --experimental-test-isolation=none scripts\commercial-gates-governance-node-test-catalog.test.mjs`
  - Result: passed, 2/2 tests.
- `pnpm.cmd run test:sdkwork-workspace-structure-standard`
  - Result: passed with `SDKWork workspace structure standard passed`.
- `node --check scripts\build-sdkwork-im-desktop-assets.mjs`
  - Result: passed.
- `pnpm.cmd run test:workflow-commercial-gates`
  - Result: passed, 44/44 governed Node tests.

## Continuation Checkpoint 2026-06-12 - Standard Directory README Contract

Status: the repository structure standard now verifies that every standard top-level directory
README carries the SDKWORK directory dictionary fields required by `SDKWORK_WORKSPACE_SPEC.md`.
`apps/README.md` and `sdks/README.md` have been brought into that shape without changing runtime
behavior, generated SDK output, UI layout, or visual styling.

Implementation notes:

- `scripts/sdkwork-workspace-structure-standard.test.mjs` now checks each standard top-level
  directory README for Purpose, Owner, Allowed Content, Forbidden Content, Related Specs, and
  Verification sections, and requires a link to `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`.
- `apps/README.md` now documents app-root purpose, ownership, allowed content, forbidden content,
  related specs, and verification for `apps/sdkwork-im-pc` and future app surfaces.
- `sdks/README.md` keeps the existing SDK workspace details and adds the standard directory README
  fields at the top.

TDD evidence:

- `node scripts\sdkwork-workspace-structure-standard.test.mjs` failed first because
  `apps/README.md` was empty and `sdks/README.md` did not expose the standard directory README
  headings, then passed after the documentation updates.

Verified commands:

- `pnpm.cmd run test:sdkwork-workspace-structure-standard`
  - Result: passed with `SDKWork workspace structure standard passed`.
- `npm.cmd run docs:verify` from `docs/sites`
  - Result: passed; standardized 145 API markdown files, generated 118 operation pages, verified 18
    source API pages, 118 operation pages, 118 sidebar entries, and SDK site docs.
- `pnpm.cmd run test:workflow-commercial-gates`
  - Result: passed, 44/44 governed Node tests.
- `pnpm.cmd run lint` from `apps/sdkwork-im-pc`
  - Result: passed; TypeScript checking completed with exit code 0.
- `pnpm.cmd run build` from `apps/sdkwork-im-pc`
  - Result: passed; Vite/esbuild emitted the PC dist and `dist/server.cjs`. Vite still reports the
    existing large chunk warning, but the command exits successfully.
- `node scripts\build-sdkwork-im-desktop-assets.mjs`
  - Result: passed; it built the PC app from `apps/sdkwork-im-pc` and verified the desktop web
    asset output.

## Continuation Checkpoint 2026-06-12 - Docs RTC SDK Path Alignment

Status: docs-site SDK verification now reflects the current SDKWORK sibling dependency layout for
`sdkwork-rtc`. RTC SDK examples point to `../sdkwork-rtc/sdks/sdkwork-rtc-sdk` from the repository
root instead of materializing RTC SDK families under this repository's `sdks/` tree or using over-deep
relative paths.
`../../../../sdkwork-rtc` path. This is documentation and verification alignment only; no UI layout
or rendered visual design was changed.

Implementation notes:

- `docs/sites/reference/cli-and-scripts.md`, `docs/sites/sdk/index.md`,
  `docs/sites/sdk/rtc-sdk.md`, and `docs/sites/sdk/language-support.md` now use the same sibling RTC
  SDK path.
- `docs/sites/scripts/verify-docs-site.mjs`, `docs/sites/scripts/verify-sdk-docs.mjs`, and
  `docs/sites/sdk/verify-sdk-site-docs.mjs` now assert the sibling RTC SDK verification command.
- The change follows `DEPENDENCY_MANAGEMENT_SPEC.md`: sibling SDKWork source paths remain relative
  native workspace paths and are not copied into this repository as a local SDK family.

Verified commands:

- `node scripts\verify-docs-site.mjs` from `docs/sites`
  - Result: passed with docs navigation, CLI docs, and optional admin README alignment verified.
- `node scripts\verify-sdk-docs.mjs` from `docs/sites`
  - Result: passed with `Verified SDK documentation contract pages`.
- `node sdk\verify-sdk-site-docs.mjs` from `docs/sites`
  - Result: passed with `[docs/sites/sdk] SDK site docs verification passed`.
- `npm.cmd run docs:verify` from `docs/sites`
  - Result: passed; standardized 145 API markdown files, generated 118 operation pages, verified 18
    source API pages, 118 operation pages, 118 sidebar entries, and SDK site docs.

Environment note:

- `npm run docs:verify` without `.cmd` is blocked in this PowerShell environment by script
  execution policy for `npm.ps1`; `npm.cmd run docs:verify` is the working command here.

## Continuation Checkpoint 2026-06-12 - RTC Verifier Command Root Alignment

Status: repository SDK documentation now uses the same sibling `sdkwork-rtc` verifier command when
the command is meant to run from the Sdkwork IM repository root. Existing markdown links that are
relative to their own document locations remain unchanged when they already resolve correctly. This
is documentation and static verification alignment only; no UI layout, visual styling, runtime SDK
output, or package names were changed.

Implementation notes:

- `sdks/README.md` now documents
  `node ..\sdkwork-rtc\sdks\sdkwork-rtc-sdk\bin\verify-sdk.mjs` for the independent RTC provider
  SDK verifier.
- The IM/RTC integration guide's executable RTC verification commands now use the same repository
  root sibling path as the IM commands in that section.
- `scripts/sdkwork-workspace-structure-standard.test.mjs` now fails when repository-root RTC
  verifier examples use over-deep `../../sdkwork-rtc` or `../../../sdkwork-rtc` command paths.

TDD evidence:

- `node scripts\sdkwork-workspace-structure-standard.test.mjs` failed first on the stale
  `sdks/README.md` RTC verifier command, then passed after the README command was realigned.
- The same test failed first on the stale executable RTC verifier command in the IM/RTC integration
  guide, then passed after the command was realigned.

Verified commands:

- `pnpm.cmd run test:sdkwork-workspace-structure-standard`
  - Result: passed with `SDKWork workspace structure standard passed`.
- `npm.cmd run docs:verify` from `docs/sites`
  - Result: passed; standardized 145 API markdown files, generated 118 operation pages, verified 18
    source API pages, 118 operation pages, 118 sidebar entries, and SDK site docs.
- `pnpm.cmd run test:workflow-commercial-gates`
  - Result: passed, 44/44 governed Node tests.

## Continuation Checkpoint 2026-06-12 - Appbase Cargo Sibling Path Alignment

Status: the Cargo workspace now follows the current sibling `sdkwork-appbase` Rust crate layout
under `../sdkwork-appbase/crates/` instead of the retired
`../sdkwork-appbase/packages/native-rust/...` paths. The dependency-management standard now fails
early when root `Cargo.toml` SDKWork sibling path dependencies do not resolve to a real
  `Cargo.toml`, when the same SDKWork sibling Cargo path is declared by more than one root
  workspace key, and when a referenced sibling Cargo crate inherits `*.workspace = true` dependency
  keys that the consuming root workspace has not declared. No UI layout, visual styling, frontend
  runtime behavior, or generated SDK output was changed.

Implementation notes:

- `Cargo.toml` keeps the local dependency aliases `sdkwork_id` and `sdkwork_http_context`, but
  points them at the current appbase packages `sdkwork-platform-id-service` and
  `sdkwork-platform-http-context-service` through Cargo `package = ...` aliases.
- `Cargo.toml` keeps `sdkwork_iam_context_service` as the single root workspace key for
  `../sdkwork-iam/crates/sdkwork-iam-context-service` and removes the unused old
  `sdkwork_iam_core` alias so the sibling source path is declared exactly once.
- `Cargo.toml` also declares `sdkwork_iam_context_service`, `async-trait`, and `getrandom` at the
  workspace root because `sdkwork-platform-http-context-service` inherits those entries with Cargo
  workspace dependency syntax.
- `scripts/dependency-management-standard.test.mjs` now validates SDKWork sibling Cargo paths and
  the inherited workspace dependency keys needed by directly referenced sibling Cargo crates.

TDD evidence:

- `node scripts\dependency-management-standard.test.mjs` failed first on the three missing retired
  appbase paths:
  `sdkwork-id-rust`, `sdkwork-iam-core-rust`, and `sdkwork-http-context-rust`.
- After path aliases were updated, `cargo test -p sdkwork-im-runtime-id --tests` failed on the
  missing inherited `sdkwork_iam_context_service.workspace = true` root declaration.
- The dependency-management test was then tightened to detect that missing inherited workspace key,
  failed on it, and passed after the root workspace declarations were completed.
- The dependency-management test then failed on duplicate root declarations of the
  `sdkwork-iam-context-service` sibling path and passed after the unused `sdkwork_iam_core` alias
  was removed.

Verified commands:

- `node scripts\dependency-management-standard.test.mjs`
  - Result: passed with `Dependency management standard passed`.
- `pnpm.cmd run check:dependency-management`
  - Result: passed with `Dependency management standard passed`; rerun after removing the duplicate
    `sdkwork_iam_core` path alias also passed.
- `cargo metadata --no-deps --format-version 1`
  - Result: passed; Cargo resolved the root workspace metadata with the appbase sibling crate paths,
    including after the duplicate appbase IAM alias was removed.
- `cargo test -p sdkwork-im-runtime-id --tests`
  - Result: passed, 4/4 integration tests.
- `cargo test -p im-app-context --tests`
  - Result: passed, 11/11 integration tests.
- `cargo test -p control-plane-api --tests`
  - Result: passed; the previously blocking Cargo dependency load failure is resolved.
- `pnpm.cmd run test:workflow-commercial-gates`
  - Result: passed, 44/44 governed Node tests.
- `pnpm.cmd run check:commercial-readiness`
  - Result: PC install, lint, build, appbase UI contract, notary integration, QR scan contract,
    dependency management, governed workflow tests, control-plane API tests, commercial gate
    contract, session gateway tests, performance quant baseline, and performance drill catalog
    passed. The command exited with exit code `2` on the truthful Capacity Tier evidence
    blocker, not on the earlier appbase Cargo path failure.

Remaining blocker:

- `artifacts/perf/step-11/capacity/capacity-tier-evidence-index.json` remains
  `template_only_pending_execution` with 7 pending required slots:
  `artifacts/perf/step-11/capacity/connection/capacity.json`,
  `artifacts/perf/step-11/capacity/message/capacity.json`,
  `artifacts/perf/step-11/capacity/stream/capacity.json`,
  `artifacts/perf/step-11/capacity/restore-recovery/recovery.json`,
  `artifacts/perf/step-11/capacity/failover/recovery.json`,
  `artifacts/perf/step-11/capacity/reports/capacity-report.md`, and
  `artifacts/perf/step-11/capacity/reports/recovery-report.md`.

