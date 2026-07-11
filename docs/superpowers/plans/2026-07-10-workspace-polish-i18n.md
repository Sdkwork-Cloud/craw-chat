# PC Workbench Polish And I18n Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the PC workbench a usable, keyboard-accessible launcher and recent-document surface with complete zh-CN/en-US localization and verifiable fallback behavior.

**Architecture:** Keep the existing App PC React package boundary. `src/index.tsx` remains a thin public re-export; `WorkspaceView.tsx` composes the view and `components/WorkspaceShortcutDialog.tsx` owns modal focus behavior. The workspace service remains the injected app-SDK/Drive-SDK orchestration boundary, owns local shortcut preferences, and reports remote/fallback status so the UI can show a retryable degraded state. Package-local locale fragments remain under `src/i18n/<locale>/communication/im-pc-workspace/home.json`, with the existing host language event and i18next instance as the runtime bridge.

**Tech Stack:** React, TypeScript, i18next/react-i18next, generated app/Drive SDK facades, Tailwind CSS utilities, Node contract tests, Playwright browser smoke verification.

---

### Task 1: Establish failing workbench and locale contracts

**Files:**
- Create: `apps/sdkwork-im-pc/scripts/workspace-workbench-contract.test.mjs`
- Modify: `scripts/dev/sdkwork-im-pc-i18n.test.mjs`
- Create: `apps/sdkwork-im-pc/e2e/fixtures/workspace-harness.html`
- Create: `apps/sdkwork-im-pc/e2e/fixtures/workspace-harness.tsx`
- Create: `apps/sdkwork-im-pc/e2e/workspace.spec.ts`

- [ ] **Step 1: Add failing assertions for the missing locale keys and capability wiring**
  - Assert both locale fragments contain the same `apps` keys for the commercial workbench catalog (`knowledge`, `community`, `voice`, `shop`, `orders`).
  - Assert the workspace source contains a real search handler, management dialog semantics, keyboard shortcut listener, retry/empty state hooks, and no fake `about:blank?doc=` action.
  - Add a browser fixture/spec that tests search, Cmd/Ctrl+K focus, modal focus containment, shortcut save, retryable degraded state, Drive navigation, and host language event fallback.

- [ ] **Step 2: Run the narrow contracts and confirm they fail for the current implementation**
  - Run: `pnpm test:sdkwork-im-pc-i18n`
  - Run: `node apps/sdkwork-im-pc/scripts/workspace-workbench-contract.test.mjs`
  - Expected: the new workbench assertions fail because the current view has no search/management behavior and the locale fragments lack catalog keys.

### Task 2: Make workspace service data and shortcut preferences explicit

**Files:**
- Modify: `apps/sdkwork-im-pc/packages/sdkwork-im-pc-workspace/src/services/WorkspaceService.ts`
- Create: `apps/sdkwork-im-pc/scripts/workspace-service-contract.test.ts`
- Create: `apps/sdkwork-im-pc/scripts/stubs/workspace-core.stub.ts`
- Create: `apps/sdkwork-im-pc/scripts/stubs/workspace-shell.stub.ts`
- Create: `apps/sdkwork-im-pc/scripts/workspace-service-test.tsconfig.json`

- [ ] **Step 1: Write the failing service behavior tests**
  - Use an injected fake app SDK and Drive SDK plus an in-memory `localStorage` implementation.
  - Verify the service returns the enabled commercial catalog with stable icon metadata, defaults all available apps to pinned, persists `savePinnedAppIds`, keeps the required Notary shortcut pinned, filters stale non-commercial shortcuts, maps a bounded recent Drive page without full-download pagination, and reports remote/fallback status.

- [ ] **Step 2: Run the service test and confirm the expected missing-method/type failure**
  - Run: `pnpm --dir apps/sdkwork-im-pc exec node ../../scripts/dev/run-tsx-cli.mjs --tsconfig scripts/workspace-service-test.tsconfig.json scripts/workspace-service-contract.test.ts`

- [ ] **Step 3: Implement the smallest service changes**
  - Add a versioned local pinned-app storage key and `pinned`/`required` view metadata.
  - Add `savePinnedAppIds(ids)` and return explicit remote/fallback source status for app and document reads.
  - Preserve injected SDK clients and the existing Drive page-size contract; do not add raw HTTP or generated SDK edits.

- [ ] **Step 4: Run the service test until green**
  - Run the same `run-tsx-cli.mjs` command and confirm all assertions pass.

### Task 3: Rebuild the workbench interactions and states

**Files:**
- Modify: `apps/sdkwork-im-pc/packages/sdkwork-im-pc-workspace/src/index.tsx`
- Create: `apps/sdkwork-im-pc/packages/sdkwork-im-pc-workspace/src/WorkspaceView.tsx`
- Create: `apps/sdkwork-im-pc/packages/sdkwork-im-pc-workspace/src/components/WorkspaceShortcutDialog.tsx`

- [ ] **Step 1: Implement query and navigation behavior**
  - Add a controlled search field that filters localized app labels and recent document names, supports Cmd/Ctrl+K focus outside modal state, and exposes an accessible clear action.
  - Make “View all” navigate to Drive through `onAppSelect`; until Drive exposes a resource-level handoff, recent-document rows must honestly label the action as opening Drive rather than a specific document.

- [ ] **Step 2: Implement shortcut management**
  - Replace the placeholder app-center flow with a `role="dialog"` management surface using native checkboxes, Escape/backdrop close, focus trap, focus return, Cancel, and Save.
  - Persist changed pinned IDs through the service and render only pinned apps in the default launcher view; search still searches the full catalog.

- [ ] **Step 3: Complete loading, empty, retry, and localization-aware formatting states**
  - Add explicit loading skeleton/aria-busy state, app/search/document empty states, and a retry action for both explicit degraded remote data and unexpected load failures.
  - Derive greeting and date/time labels from locale-aware `Intl.DateTimeFormat`; do not reload server data merely because the language changes.
  - Add missing icon mappings and accessible labels/tooltips; remove the fake new-tab URL and non-functional “more” action.

- [ ] **Step 4: Run the workbench contract test and TypeScript build check**
  - Run: `node apps/sdkwork-im-pc/scripts/workspace-workbench-contract.test.mjs`
  - Run: `pnpm --dir apps/sdkwork-im-pc exec tsc -p tsconfig.app.json --noEmit`

### Task 4: Finish locale fragments and runtime fallback checks

**Files:**
- Modify: `apps/sdkwork-im-pc/packages/sdkwork-im-pc-workspace/src/i18n/index.ts`
- Modify: `apps/sdkwork-im-pc/packages/sdkwork-im-pc-workspace/src/i18n/zh-CN/communication/im-pc-workspace/home.json`
- Modify: `apps/sdkwork-im-pc/packages/sdkwork-im-pc-workspace/src/i18n/en-US/communication/im-pc-workspace/home.json`
- Modify: `scripts/dev/sdkwork-im-pc-i18n.test.mjs`

- [ ] **Step 1: Add complete matching zh-CN/en-US keys**
  - Cover catalog names, search result labels, management dialog actions, retry/error/empty states, and locale-aware time labels.
  - Preserve interpolation names exactly across locales and keep the fragment path/layout canonical.

- [ ] **Step 2: Harden the workspace i18next instance**
  - Declare supported languages, explicit fallback, and current-only loading; reuse the canonical host language normalization/event contract.

- [ ] **Step 3: Run i18n static and regression checks**
  - Run: `pnpm test:sdkwork-im-pc-i18n`
  - Run: `node ..\sdkwork-specs\tools/check-i18n-standard.mjs --root .`

### Task 5: Verify the composed PC surface

**Files:**
- No additional source files unless verification exposes a defect.

- [ ] **Step 1: Run focused checks**
  - Run the service contract, workbench contract, i18n regression, package TypeScript check, `node ../sdkwork-specs/tools/check-i18n-standard.mjs --root .`, `node ../sdkwork-specs/tools/check-application-layering.mjs --root apps/sdkwork-im-pc`, `node ../sdkwork-specs/tools/check-frontend-composition.mjs --root apps/sdkwork-im-pc`, `node ../sdkwork-specs/tools/check-api-operation-patterns.mjs --workspace .`, `node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .`, and `node ../sdkwork-specs/tools/check-pagination.mjs --workspace .`.

- [ ] **Step 2: Start the PC dev server and perform browser verification**
  - Start `pnpm --dir apps/sdkwork-im-pc dev` on an available port.
  - Verify the workbench at desktop and narrow/tablet widths: language switching, search, keyboard focus, dialog close/save, empty states, Drive navigation, and no overlapping text.

- [ ] **Step 3: Run the package build**
  - Run: `pnpm --dir apps/sdkwork-im-pc build`
  - Record exact command output and any residual risks caused by unrelated dirty-worktree changes.
