# TypeScript Generated Package Install Safety Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the generated TypeScript transport package install-safe so downstream `pnpm i` does not fail because the generated package runs stale local build hooks or broken `.bin` tool links.

**Architecture:** Add a regression verifier that enforces an install-safe generated package manifest and a runnable build command, then normalize the generated package manifest onto a root-owned build entrypoint and harden the root build script so polluted generated `node_modules` are rebuilt safely. Keep the fix inside the IM SDK workspace so regeneration preserves it.

**Tech Stack:** Node.js scripts, pnpm/npm package manifests, TypeScript generated SDK workspace, existing IM SDK verification scripts

---

### Task 1: Add a failing regression verifier for generated package install safety

**Files:**
- Create: `sdks/sdkwork-im-sdk/bin/verify-typescript-generated-package-install-safety.mjs`
- Modify: `sdks/sdkwork-im-sdk/bin/verify-typescript-workspace.mjs`

- [ ] **Step 1: Write a verifier that fails when the generated package has install-time lifecycle hooks or an unsafe build script**
- [ ] **Step 2: Run the verifier directly and confirm it fails against the current generated package**
- [ ] **Step 3: Wire the verifier into the TypeScript workspace verification flow**

### Task 2: Normalize the generated package manifest onto a safe build contract

**Files:**
- Create: `sdks/sdkwork-im-sdk/bin/normalize-typescript-generated-package-manifest.mjs`
- Modify: `sdks/sdkwork-im-sdk/bin/generate-sdk.ps1`
- Modify: `sdks/sdkwork-im-sdk/bin/generate-sdk.sh`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/generated/server-openapi/package.json`

- [ ] **Step 1: Add a normalization script that rewrites the generated package manifest to remove install-time lifecycle hooks and route builds through the root builder**
- [ ] **Step 2: Call that normalization step from the TypeScript generation pipeline**
- [ ] **Step 3: Update the checked-in generated package manifest to the normalized contract**

### Task 3: Harden the root generated-package builder against polluted local tooling

**Files:**
- Modify: `sdks/sdkwork-im-sdk/bin/build-typescript-generated-package.mjs`

- [ ] **Step 1: Detect unsafe generated `node_modules` tool links that point outside the generated workspace**
- [ ] **Step 2: Remove unsafe local tooling state before reinstalling build dependencies**
- [ ] **Step 3: Keep the builder behavior deterministic and compatible with the existing verification flow**

### Task 4: Verify the regression is fixed end to end

**Files:**
- Modify: `sdks/sdkwork-im-sdk/*` as needed

- [ ] **Step 1: Run the new install-safety verifier and confirm it passes**
- [ ] **Step 2: Run the TypeScript workspace verifier and confirm it passes**
- [ ] **Step 3: Run the generated package build through its public `npm run build` entrypoint and confirm it succeeds**
