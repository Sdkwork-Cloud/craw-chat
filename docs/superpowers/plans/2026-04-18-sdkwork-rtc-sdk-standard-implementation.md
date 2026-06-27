# SDKWork RTC SDK Standard Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Materialize a new `sdkwork-rtc-sdk` workspace that follows a JDBC-style provider standard, aligns with the existing provider registry, ships a verified TypeScript baseline, and establishes multi-language workspace standards without introducing fake runtime promises.

**Architecture:** Treat `sdkwork-rtc-sdk` as a provider-standard workspace, not an OpenAPI-generation workspace. The root workspace owns docs, assembly metadata, and structural verification. The TypeScript workspace is the executable reference implementation with provider-neutral core contracts, a driver manager, a data source model, and built-in adapters for `volcengine`, `aliyun`, and `tencent`. Other official language workspaces are materialized as standards-governed skeletons with README and package-boundary placeholders so the matrix stays explicit and verifiable.

**Tech Stack:** Node.js, TypeScript, PowerShell, shell wrappers, Markdown, JSON

---

## File Structure

### Existing files to modify

- `sdks/README.md` only if the new workspace must be added to the root overview

### New files to create

- `docs/superpowers/specs/2026-04-18-sdkwork-rtc-sdk-jdbc-style-standard-design.md`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/README.md`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/.sdkwork-assembly.json`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/docs/README.md`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/docs/package-standards.md`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/docs/provider-adapter-standard.md`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/docs/multilanguage-capability-matrix.md`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/docs/verification-matrix.md`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/bin/verify-sdk.mjs`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/bin/verify-sdk.ps1`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/bin/verify-sdk.sh`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/test/verify-sdk-automation.test.mjs`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/package.json`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/tsconfig.build.json`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/README.md`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/bin/package-task.mjs`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/index.ts`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/errors.ts`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/types.ts`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/capabilities.ts`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/driver.ts`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/driver-manager.ts`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/data-source.ts`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/client.ts`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/providers/volcengine.ts`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/providers/aliyun.ts`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/providers/tencent.ts`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/test/driver-manager.test.mjs`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/test/data-source.test.mjs`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/test/built-in-providers.test.mjs`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-flutter/README.md`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-rust/README.md`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-java/README.md`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-csharp/README.md`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-swift/README.md`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-kotlin/README.md`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-go/README.md`
- `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-python/README.md`

---

### Task 1: Materialize The Root RTC Workspace Contract

**Files:**
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/README.md`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/.sdkwork-assembly.json`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/docs/README.md`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/docs/package-standards.md`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/docs/provider-adapter-standard.md`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/docs/multilanguage-capability-matrix.md`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/docs/verification-matrix.md`
- Modify: `sdks/README.md`

- [ ] **Step 1: Write the failing root automation test**

Add `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/test/verify-sdk-automation.test.mjs` assertions that require:
- the RTC workspace is listed in `sdks/README.md`
- the root docs files exist
- `.sdkwork-assembly.json` declares all official languages
- the default provider is `volcengine`

- [ ] **Step 2: Run the test to confirm the gap**

Run:

```bash
node <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/test/verify-sdk-automation.test.mjs
```

Expected: FAIL because the workspace contract files do not exist yet.

- [ ] **Step 3: Create the root workspace docs and assembly metadata**

Write the README, internal standards docs, and assembly snapshot so the workspace has a real
machine-readable and human-readable contract.

- [ ] **Step 4: Re-run the root automation test**

Run:

```bash
node <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/test/verify-sdk-automation.test.mjs
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add sdks/README.md <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk
git commit -m "feat(rtc-sdk): add workspace contract"
```

### Task 2: Add Root Verification Entry Points

**Files:**
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/bin/verify-sdk.mjs`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/bin/verify-sdk.ps1`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/bin/verify-sdk.sh`

- [ ] **Step 1: Extend the failing root automation test**

Add assertions that require:
- `bin/verify-sdk.mjs` exists
- PowerShell and shell wrappers exist
- the verifier rejects missing required docs or language workspace directories

- [ ] **Step 2: Run the automation test and confirm it fails**

Run:

```bash
node <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/test/verify-sdk-automation.test.mjs
```

Expected: FAIL because the verifier and wrappers are missing.

- [ ] **Step 3: Implement the root verifier and wrappers**

Write `verify-sdk.mjs` so it validates:
- required root docs
- required language directories
- assembly default provider and official language list
- required TypeScript workspace files

- [ ] **Step 4: Re-run the automation test and the root verifier**

Run:

```bash
node <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/test/verify-sdk-automation.test.mjs
node <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/bin/verify-sdk.mjs
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/bin <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/test/verify-sdk-automation.test.mjs
git commit -m "feat(rtc-sdk): add root verification"
```

### Task 3: Implement The TypeScript Core Contract With TDD

**Files:**
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/package.json`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/tsconfig.build.json`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/README.md`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/bin/package-task.mjs`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/index.ts`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/errors.ts`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/types.ts`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/capabilities.ts`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/driver.ts`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/driver-manager.ts`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/data-source.ts`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/client.ts`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/test/driver-manager.test.mjs`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/test/data-source.test.mjs`

- [ ] **Step 1: Write the failing driver-manager and data-source tests**

Require these behaviors:
- registering a driver by provider key
- resolving a driver by provider URL and explicit provider key
- defaulting provider selection to `volcengine`
- throwing stable `driver_not_found` and `invalid_provider_url` errors
- exposing metadata and capability information through the created client

- [ ] **Step 2: Run the tests and confirm the gap**

Run:

```bash
node <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/test/driver-manager.test.mjs
node <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/test/data-source.test.mjs
```

Expected: FAIL because the TypeScript workspace files do not exist yet.

- [ ] **Step 3: Implement the minimal TypeScript core contract**

Create the package metadata, build config, and source files for:
- normalized error handling
- provider descriptors and capability sets
- the provider driver SPI
- the driver manager registry and selection rules
- the data source abstraction

- [ ] **Step 4: Re-run the tests**

Run:

```bash
node <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/test/driver-manager.test.mjs
node <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/test/data-source.test.mjs
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript
git commit -m "feat(rtc-sdk): add typescript core contract"
```

### Task 4: Add Built-In TypeScript Provider Adapters

**Files:**
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/providers/volcengine.ts`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/providers/aliyun.ts`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/providers/tencent.ts`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/test/built-in-providers.test.mjs`

- [ ] **Step 1: Write the failing built-in provider tests**

Require:
- each built-in provider exposes a stable `pluginId`, `providerKey`, `driverId`, and display name
- `volcengine` is marked default-selected
- all three providers advertise the required capability baseline
- `unwrap()` returns the vendor-native client instance the adapter created

- [ ] **Step 2: Run the built-in provider test and confirm the gap**

Run:

```bash
node <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/test/built-in-providers.test.mjs
```

Expected: FAIL because the built-in provider modules do not exist yet.

- [ ] **Step 3: Implement the provider adapters**

Build adapters that wrap a caller-supplied native client factory rather than bundling vendor SDK
implementations directly. Keep the adapters explicit, typed, and one-provider-only.

- [ ] **Step 4: Re-run the built-in provider test and TypeScript package build**

Run:

```bash
node <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/test/built-in-providers.test.mjs
node <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/bin/package-task.mjs test
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/providers <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/test/built-in-providers.test.mjs
git commit -m "feat(rtc-sdk): add built-in provider adapters"
```

### Task 5: Materialize The Official Multi-Language Skeletons

**Files:**
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-flutter/README.md`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-rust/README.md`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-java/README.md`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-csharp/README.md`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-swift/README.md`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-kotlin/README.md`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-go/README.md`
- Create: `<workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-python/README.md`

- [ ] **Step 1: Extend the root automation test**

Add assertions that require every official language directory to exist and contain a README naming:
- the language
- the planned public package name
- the control/runtime support boundary

- [ ] **Step 2: Run the root automation test and confirm the gap**

Run:

```bash
node <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/test/verify-sdk-automation.test.mjs
```

Expected: FAIL because the language skeletons do not exist yet.

- [ ] **Step 3: Create the official language skeleton READMEs**

Each README must document:
- language role
- planned package name
- whether runtime bridge is shipped now or reserved
- the provider adapter standard and capability matrix references

- [ ] **Step 4: Re-run the automation test and full root verification**

Run:

```bash
node <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/test/verify-sdk-automation.test.mjs
node <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/bin/verify-sdk.mjs
node <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/bin/package-task.mjs test
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add <workspace-root>/sdkwork-rtc/sdks/sdkwork-rtc-sdk
git commit -m "feat(rtc-sdk): add official language skeletons"
```
