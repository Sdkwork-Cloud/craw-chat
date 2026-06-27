> Migrated from `docs/superpowers/plans/2026-04-10-craw-chat-admin-overview-authenticity.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Sdkwork IM Admin Overview Authenticity Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remove synthetic fallback metrics and seeded placeholder queues from the Sdkwork IM Admin Overview page so the command surface reflects only real snapshot posture.

**Architecture:** Keep the existing Overview route and visual shell, but move all non-trivial derivation into a focused `overviewModel.ts` helper. The React page becomes a thin presenter over explicit metrics, honest empty states, and command-board statuses derived from workspace snapshot data.

**Tech Stack:** React 19, TypeScript, workspace package modules, Node source-contract tests

---

### Task 1: Lock The Product Constraint

**Files:**
- Modify: `apps/control-plane/tests/admin-product-experience.test.mjs`

- [ ] Step 1: Keep the failing Overview authenticity contract that bans numeric fallbacks and seeded placeholder queues.
- [ ] Step 2: Run `node --experimental-test-isolation=none --test apps/control-plane/tests/admin-product-experience.test.mjs`.
- [ ] Step 3: Confirm the failure is caused by synthetic Overview fallback content rather than an unrelated regression.

### Task 2: Extract Snapshot-Derived Overview Model

**Files:**
- Create: `apps/control-plane/packages/sdkwork-control-plane-overview/src/overviewModel.ts`
- Modify: `apps/control-plane/packages/sdkwork-control-plane-overview/src/index.tsx`

- [ ] Step 1: Add focused helper types for metrics, hot conversations, incident watch, tenant load, and command-board cards.
- [ ] Step 2: Derive message throughput, moderation backlog, and online users directly from `AdminWorkspaceSnapshot`.
- [ ] Step 3: Derive hot-conversation and tenant-load lists from snapshot projects, tenants, usage, and billing summaries without fabricated records.
- [ ] Step 4: Derive incident-watch and command-board posture from alerts, runtime health, provider health, credential coverage, quota state, and campaign status.
- [ ] Step 5: Render explicit honest empty states when no snapshot data exists instead of seeded operational fiction.

### Task 3: Localize Any New Honest-State Copy

**Files:**
- Modify: `apps/control-plane/packages/sdkwork-control-plane-core/src/i18n.tsx`

- [ ] Step 1: Add `zh-CN` translations for any new Overview empty-state and command-board strings.
- [ ] Step 2: Reuse existing workspace-status translation keys where they already describe live admin posture accurately.

### Task 4: Verify Overview Regression Surface

**Files:**
- Review only: `apps/control-plane/packages/sdkwork-control-plane-overview/src/index.tsx`
- Review only: `apps/control-plane/packages/sdkwork-control-plane-overview/src/overviewModel.ts`
- Review only: `apps/control-plane/packages/sdkwork-control-plane-core/src/i18n.tsx`

- [ ] Step 1: Run `node --experimental-test-isolation=none --test apps/control-plane/tests/admin-product-experience.test.mjs`.
- [ ] Step 2: Run `node --experimental-test-isolation=none --test apps/control-plane/tests/*.mjs`.
- [ ] Step 3: Run `pnpm.cmd typecheck` from `apps/control-plane`.
- [ ] Step 4: Summarize any residual product gaps if Overview still lacks a first-class live conversation source beyond project traffic summaries.

