> Migrated from `docs/superpowers/plans/2026-04-09-craw-chat-sdk-generation.md` on 2026-06-24.
> Owner: SDKWork maintainers

# IM SDK Generation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build OpenAPI-driven TypeScript and Flutter app SDK workspaces for `sdkwork-im` and generate both SDKs through `sdkwork-sdk-generator`.

**Architecture:** The workspace root owns the authority OpenAPI contract, the derived generator input, and regeneration wrappers. TypeScript and Flutter each receive a layered workspace whose generator-owned output is isolated under `generated/server-openapi`, with manual docs and scripts outside the generated boundary.

**Tech Stack:** OpenAPI 3.x, Node.js, PowerShell, shell wrappers, SDKWORK Generator, TypeScript, Flutter/Dart

---

### Task 1: Establish Workspace And Documentation Skeleton

**Files:**
- Create: `docs/superpowers/specs/2026-04-09-im-sdk-generation-design.md`
- Create: `docs/superpowers/plans/2026-04-09-im-sdk-generation.md`
- Modify: `sdks/sdkwork-im-sdk/README.md`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/README.md`
- Create: `sdks/sdkwork-im-sdk/openapi/README.md`

- [ ] Step 1: Write the workspace design and execution docs.
- [ ] Step 2: Replace placeholder README content with clear English regeneration docs and ownership boundaries.
- [ ] Step 3: Add the root `openapi/README.md` describing authority and derived spec ownership.
- [ ] Step 4: Inspect the workspace tree to confirm the documentation targets exist as expected.

### Task 2: Author The OpenAPI Authority Contract

**Files:**
- Create: `sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml`

- [ ] Step 1: Map the app-facing path list from `services/local-minimal-node/src/node/build.rs`.
- [ ] Step 2: Encode bearer auth, shared responses, and tag taxonomy in the authority contract.
- [ ] Step 3: Add request and response schemas derived from Rust domain models and service request structs.
- [ ] Step 4: Add websocket path documentation for `/im/v3/api/realtime/ws` without promising generated websocket transport.
- [ ] Step 5: Validate the YAML parses cleanly through the generator loader path.

### Task 3: Add The Derived sdkgen Input And Regeneration Scripts

**Files:**
- Create: `sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.sdkgen.yaml`
- Create: `sdks/sdkwork-im-sdk/bin/prepare-openapi-source.mjs`
- Create: `sdks/sdkwork-im-sdk/bin/assemble-sdk.mjs`
- Create: `sdks/sdkwork-im-sdk/bin/generate-sdk.ps1`
- Create: `sdks/sdkwork-im-sdk/bin/generate-sdk.sh`

- [ ] Step 1: Add a derived `sdkgen` input file based on the authority contract.
- [ ] Step 2: Implement `prepare-openapi-source.mjs` so the derived file can be refreshed deterministically from the authority spec.
- [ ] Step 3: Implement `assemble-sdk.mjs` to emit stable workspace metadata after generation.
- [ ] Step 4: Implement root generation wrappers that resolve one SDK version and generate TypeScript plus Flutter into `generated/server-openapi`.
- [ ] Step 5: Run the preparation step alone to confirm the derived file is produced and stable.

### Task 4: Create The TypeScript And Flutter Layered Workspace Shells

**Files:**
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/bin/sdk-gen.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/bin/sdk-gen.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/bin/sdk-assemble.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/bin/sdk-assemble.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/bin/sdk-gen.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/bin/sdk-gen.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/bin/sdk-assemble.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/bin/sdk-assemble.sh`

- [ ] Step 1: Add thin per-language forwarding scripts back to the root wrappers.
- [ ] Step 2: Create stable directories for `generated/server-openapi`.
- [ ] Step 3: Confirm the workspace shape matches the layered family standard from the IM SDK references.

### Task 5: Generate Both SDKs

**Files:**
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/generated/server-openapi/*`
- Modify: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/generated/server-openapi/*`

- [ ] Step 1: Run the root generation wrapper for TypeScript and Flutter.
- [ ] Step 2: Inspect generated package manifests and top-level exports.
- [ ] Step 3: Re-run generation to verify the result is idempotent.

### Task 6: Verify Generated Outputs

**Files:**
- Review only: generated TypeScript and Flutter outputs plus root metadata

- [ ] Step 1: Run the generator on the final specs and wrappers and confirm exit code `0`.
- [ ] Step 2: Inspect TypeScript generated `package.json`, entrypoints, and build metadata for correctness.
- [ ] Step 3: Inspect Flutter generated `pubspec.yaml`, entrypoints, and generated library surface for correctness.
- [ ] Step 4: Run TypeScript validation if generated package exposes a build or type-check command.
- [ ] Step 5: Run Dart or Flutter analyzer if the local toolchain is available.
- [ ] Step 6: Summarize any residual limitations, especially the websocket adapter boundary.

