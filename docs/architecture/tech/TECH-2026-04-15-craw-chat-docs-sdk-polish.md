> Migrated from `docs/superpowers/plans/2026-04-15-craw-chat-docs-sdk-polish.md` on 2026-06-24.
> Owner: SDKWork maintainers

# IM SDK Docs Polish Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Upgrade the `docs/sites` SDK documentation so it reads like a production-grade developer portal while staying strictly aligned with the current Sdkwork IM repository state.

**Architecture:** Reframe the SDK section around consumer workflows first, then document generated/manual ownership, contract sources, and release-state caveats. Add language-specific pages for TypeScript and Flutter, keep admin coverage precise, and wire the new pages into VitePress navigation.

**Tech Stack:** VitePress, Markdown, local SDK workspace READMEs, OpenAPI authority contract, release catalog JSON

---

### Task 1: Restructure the SDK site information architecture

**Files:**
- Modify: `docs/sites/.vitepress/config.ts`
- Modify: `docs/sites/sdk/index.md`
- Test: `docs/sites/package.json`

- [ ] **Step 1: Define the new navigation shape**
- [ ] **Step 2: Add language-specific entries for TypeScript and Flutter**
- [ ] **Step 3: Reframe the SDK overview around consumer entry points and delivery reality**
- [ ] **Step 4: Build the docs site to verify sidebar and page resolution**

### Task 2: Rewrite App SDK documentation around real consumer flows

**Files:**
- Modify: `docs/sites/sdk/app-sdk.md`
- Create: `docs/sites/sdk/typescript-sdk.md`
- Create: `docs/sites/sdk/flutter-sdk.md`
- Test: `docs/sites/sdk/app-sdk.md`

- [ ] **Step 1: Document the real package names and entrypoints**
- [ ] **Step 2: Add installation, authentication, and quick-start sections**
- [ ] **Step 3: Add capability mapping for portal/auth, session, conversations, media, streams, and RTC**
- [ ] **Step 4: Cross-check examples against current SDK source exports**

### Task 3: Tighten support and admin boundary documentation

**Files:**
- Modify: `docs/sites/sdk/language-support.md`
- Modify: `docs/sites/sdk/admin-sdk.md`
- Modify: `docs/sites/index.md`
- Test: `docs/sites/sdk/language-support.md`

- [ ] **Step 1: Make language support reflect checked-in workspaces and consumer packages**
- [ ] **Step 2: Clarify why Admin SDK docs stop at boundary and source-of-truth rules**
- [ ] **Step 3: Update homepage messaging so SDK docs are discoverable**
- [ ] **Step 4: Verify links and claims against the release catalog and repo state**

### Task 4: Run docs verification and fix any structural issues

**Files:**
- Modify: `docs/sites/*` as needed

- [ ] **Step 1: Run `npm run docs:build`**
- [ ] **Step 2: Run `npm run docs:verify`**
- [ ] **Step 3: Fix any broken links or missing page targets**
- [ ] **Step 4: Re-run verification and capture the final status**

