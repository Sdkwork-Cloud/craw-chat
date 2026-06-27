# Sdkwork IM Multilanguage SDK Generator Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Expand `sdks/sdkwork-im-sdk` into a verified multi-language SDK workspace that standardizes generation, normalization, assembly, verification, and docs for `typescript`, `flutter`, `rust`, `java`, `csharp`, `swift`, `kotlin`, `go`, and `python`, using the live Sdkwork IM OpenAPI 3.x schema as the source of truth.

**Architecture:** Keep OpenAPI generation isolated under each language workspace's `generated/server-openapi` boundary and treat all live runtime, message ergonomics, RTC helpers, and higher-level product-facing workflows as non-generated semantic code. In the current writable scope, implementation focuses on the Sdkwork IM workspace itself: root wrappers, language workspace skeletons, capability metadata, verification, and docs. Any generator-core fixes outside `apps/sdkwork-im` are captured as workspace-level contract failures and follow-up backlog rather than edited directly here.

**Tech Stack:** Node.js, PowerShell, POSIX shell, OpenAPI 3.0.3, existing `sdkwork-sdk-generator` CLI integration, TypeScript verification scripts, VitePress docs, language-native package metadata formats

---

## File Structure

### Existing files to modify

- `sdks/sdkwork-im-sdk/README.md`
- `sdks/sdkwork-im-sdk/.sdkwork-assembly.json`
- `sdks/sdkwork-im-sdk/bin/generate-sdk.ps1`
- `sdks/sdkwork-im-sdk/bin/generate-sdk.sh`
- `sdks/sdkwork-im-sdk/bin/verify-sdk.mjs`
- `sdks/sdkwork-im-sdk/bin/verify-sdk.ps1`
- `sdks/sdkwork-im-sdk/bin/verify-sdk.sh`
- `sdks/sdkwork-im-sdk/bin/assemble-sdk.mjs`
- `sdks/sdkwork-im-sdk/bin/normalize-generated-auth-surface.mjs`
- `docs/sites/sdk/index.md`
- `docs/sites/sdk/language-support.md`
- `docs/sites/README.md`

### New language workspaces to create

- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/`
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-java/`
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-csharp/`
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-swift/`
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-kotlin/`
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-go/`
- `sdks/sdkwork-im-sdk/sdkwork-im-sdk-python/`

### New verification and maintainer files to create

- `sdks/sdkwork-im-sdk/bin/verify-rust-workspace.mjs`
- `sdks/sdkwork-im-sdk/bin/verify-java-workspace.mjs`
- `sdks/sdkwork-im-sdk/bin/verify-csharp-workspace.mjs`
- `sdks/sdkwork-im-sdk/bin/verify-swift-workspace.mjs`
- `sdks/sdkwork-im-sdk/bin/verify-kotlin-workspace.mjs`
- `sdks/sdkwork-im-sdk/bin/verify-go-workspace.mjs`
- `sdks/sdkwork-im-sdk/bin/verify-python-workspace.mjs`
- `sdks/sdkwork-im-sdk/docs/multilanguage-generator-standard.md`
- `sdks/sdkwork-im-sdk/docs/multilanguage-capability-matrix.md`
- `sdks/sdkwork-im-sdk/docs/multilanguage-audit-report.md`

### New public SDK docs to create

- `docs/sites/sdk/rust-sdk.md`
- `docs/sites/sdk/java-sdk.md`
- `docs/sites/sdk/csharp-sdk.md`
- `docs/sites/sdk/swift-sdk.md`
- `docs/sites/sdk/kotlin-sdk.md`
- `docs/sites/sdk/go-sdk.md`
- `docs/sites/sdk/python-sdk.md`
- `docs/sites/sdk/generator-boundary.md`

### Workspace skeleton files to create per new language

- `<language-workspace>/README.md`
- `<language-workspace>/bin/sdk-gen.ps1`
- `<language-workspace>/bin/sdk-gen.sh`
- `<language-workspace>/bin/sdk-verify.ps1`
- `<language-workspace>/bin/sdk-verify.sh`
- `<language-workspace>/generated/server-openapi/.gitkeep` or equivalent placeholder
- `<language-workspace>/composed/README.md` or equivalent semantic-reserve marker

---

### Task 1: Lock the full language set into root workspace automation

**Files:**
- Modify: `sdks/sdkwork-im-sdk/bin/generate-sdk.ps1`
- Modify: `sdks/sdkwork-im-sdk/bin/generate-sdk.sh`
- Modify: `sdks/sdkwork-im-sdk/bin/verify-sdk.mjs`
- Modify: `sdks/sdkwork-im-sdk/bin/verify-sdk.ps1`
- Modify: `sdks/sdkwork-im-sdk/bin/verify-sdk.sh`
- Modify: `sdks/sdkwork-im-sdk/README.md`
- Test: `sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs`
- Test: `sdks/sdkwork-im-sdk/bin/verify-powershell-wrapper-args.mjs`

- [ ] **Step 1: Write the failing automation assertions**

Extend the workspace guardrail scripts so the documented language set must include:

```js
[
  'typescript',
  'flutter',
  'rust',
  'java',
  'csharp',
  'swift',
  'kotlin',
  'go',
  'python',
]
```

The guardrails should fail until:

- root generation examples mention the expanded language set
- root verification examples mention the expanded language set
- README language-workspace lists mention the expanded language set

- [ ] **Step 2: Run the guardrails and verify failure**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs
node sdks/sdkwork-im-sdk/bin/verify-powershell-wrapper-args.mjs
```

Expected: FAIL because current wrappers and docs still only recognize `typescript` and `flutter`.

- [ ] **Step 3: Implement minimal root wrapper support**

Update root wrapper parsing so the official language set is recognized consistently by:

- `generate-sdk.ps1`
- `generate-sdk.sh`
- `verify-sdk.mjs`
- `verify-sdk.ps1`
- `verify-sdk.sh`

Do not add generator-core logic here yet. This task only locks the root workspace contract so all
languages are official inputs.

- [ ] **Step 4: Re-run the guardrails**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs
node sdks/sdkwork-im-sdk/bin/verify-powershell-wrapper-args.mjs
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add sdks/sdkwork-im-sdk/bin/generate-sdk.ps1 sdks/sdkwork-im-sdk/bin/generate-sdk.sh sdks/sdkwork-im-sdk/bin/verify-sdk.mjs sdks/sdkwork-im-sdk/bin/verify-sdk.ps1 sdks/sdkwork-im-sdk/bin/verify-sdk.sh sdks/sdkwork-im-sdk/README.md
git commit -m "feat(sdk): register full multilanguage workspace set"
```

### Task 2: Scaffold all missing language workspaces with a uniform boundary

**Files:**
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/README.md`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/bin/sdk-gen.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/bin/sdk-gen.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/bin/sdk-verify.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/bin/sdk-verify.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/generated/server-openapi/.gitkeep`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/README.md`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-java/README.md`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-java/bin/sdk-gen.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-java/bin/sdk-gen.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-java/bin/sdk-verify.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-java/bin/sdk-verify.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-java/generated/server-openapi/.gitkeep`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-java/composed/README.md`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-csharp/README.md`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-csharp/bin/sdk-gen.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-csharp/bin/sdk-gen.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-csharp/bin/sdk-verify.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-csharp/bin/sdk-verify.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-csharp/generated/server-openapi/.gitkeep`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-csharp/composed/README.md`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-swift/README.md`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-swift/bin/sdk-gen.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-swift/bin/sdk-gen.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-swift/bin/sdk-verify.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-swift/bin/sdk-verify.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-swift/generated/server-openapi/.gitkeep`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-swift/composed/README.md`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-kotlin/README.md`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-kotlin/bin/sdk-gen.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-kotlin/bin/sdk-gen.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-kotlin/bin/sdk-verify.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-kotlin/bin/sdk-verify.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-kotlin/generated/server-openapi/.gitkeep`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-kotlin/composed/README.md`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-go/README.md`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-go/bin/sdk-gen.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-go/bin/sdk-gen.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-go/bin/sdk-verify.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-go/bin/sdk-verify.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-go/generated/server-openapi/.gitkeep`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-go/composed/README.md`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-python/README.md`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-python/bin/sdk-gen.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-python/bin/sdk-gen.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-python/bin/sdk-verify.ps1`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-python/bin/sdk-verify.sh`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-python/generated/server-openapi/.gitkeep`
- Create: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-python/composed/README.md`
- Test: `sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs`

- [ ] **Step 1: Add a failing workspace-boundary contract**

Extend the automation guardrail so each official language must have:

```js
requirePath(`${workspace}/README.md`);
requirePath(`${workspace}/bin/sdk-gen.ps1`);
requirePath(`${workspace}/bin/sdk-verify.ps1`);
requirePath(`${workspace}/generated/server-openapi`);
requirePath(`${workspace}/composed`);
```

- [ ] **Step 2: Run the guardrail to verify failure**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs
```

Expected: FAIL because the new language workspaces do not exist yet.

- [ ] **Step 3: Create the workspace skeletons**

For every missing language workspace:

- add root README with generated/manual ownership wording
- add thin forwarding scripts back to root wrappers
- create generated placeholder directory
- create semantic reserve or composed README

The forwarding PowerShell script shape should stay minimal:

```powershell
& (Join-Path $PSScriptRoot '..\..\bin\generate-sdk.ps1') -Languages rust @args
```

The forwarding shell script shape should stay minimal:

```bash
bash "${SCRIPT_DIR}/../../bin/generate-sdk.sh" --language rust "$@"
```

- [ ] **Step 4: Re-run the guardrail**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust sdks/sdkwork-im-sdk/sdkwork-im-sdk-java sdks/sdkwork-im-sdk/sdkwork-im-sdk-csharp sdks/sdkwork-im-sdk/sdkwork-im-sdk-swift sdks/sdkwork-im-sdk/sdkwork-im-sdk-kotlin sdks/sdkwork-im-sdk/sdkwork-im-sdk-go sdks/sdkwork-im-sdk/sdkwork-im-sdk-python
git commit -m "feat(sdk): scaffold multilanguage workspace boundaries"
```

### Task 3: Extend assembly metadata and root README for the full SDK family

**Files:**
- Modify: `sdks/sdkwork-im-sdk/.sdkwork-assembly.json`
- Modify: `sdks/sdkwork-im-sdk/README.md`
- Create: `sdks/sdkwork-im-sdk/docs/multilanguage-capability-matrix.md`
- Test: `sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs`

- [ ] **Step 1: Write the failing metadata assertions**

Make the automation checks require:

- an assembly entry for each official language
- package naming fields for generated and semantic layers where applicable
- client naming fields for the business-facing SDK
- maturity tier and verification status markers

Example expected metadata fragment:

```json
{
  "language": "rust",
  "workspace": "sdkwork-im-sdk-rust",
  "maturityTier": "tier-a",
  "primaryClient": "ImSdkClient"
}
```

- [ ] **Step 2: Run guardrails to verify failure**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs
```

Expected: FAIL because `.sdkwork-assembly.json` only contains `typescript` and `flutter`.

- [ ] **Step 3: Implement assembly and README expansion**

Update `.sdkwork-assembly.json` so every official language has:

- workspace path
- generated manifest path
- semantic manifest path or semantic reserve marker
- official consumer package naming
- generated transport package naming
- primary client naming
- maturity tier

Update the root README so it documents:

- the official language set
- the difference between Tier A and Tier B
- the generated/manual ownership rule
- the current limitation that generator-core edits remain outside writable scope

- [ ] **Step 4: Re-run the guardrails**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-sdk-automation.mjs
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add sdks/sdkwork-im-sdk/.sdkwork-assembly.json sdks/sdkwork-im-sdk/README.md sdks/sdkwork-im-sdk/docs/multilanguage-capability-matrix.md
git commit -m "docs(sdk): add multilanguage assembly metadata"
```

### Task 4: Add per-language verification entrypoints and root dispatch

**Files:**
- Create: `sdks/sdkwork-im-sdk/bin/verify-rust-workspace.mjs`
- Create: `sdks/sdkwork-im-sdk/bin/verify-java-workspace.mjs`
- Create: `sdks/sdkwork-im-sdk/bin/verify-csharp-workspace.mjs`
- Create: `sdks/sdkwork-im-sdk/bin/verify-swift-workspace.mjs`
- Create: `sdks/sdkwork-im-sdk/bin/verify-kotlin-workspace.mjs`
- Create: `sdks/sdkwork-im-sdk/bin/verify-go-workspace.mjs`
- Create: `sdks/sdkwork-im-sdk/bin/verify-python-workspace.mjs`
- Modify: `sdks/sdkwork-im-sdk/bin/verify-sdk.mjs`
- Modify: `sdks/sdkwork-im-sdk/bin/verify-sdk.ps1`
- Modify: `sdks/sdkwork-im-sdk/bin/verify-sdk.sh`
- Test: `sdks/sdkwork-im-sdk/bin/verify-sdk.mjs`

- [ ] **Step 1: Add failing verify expectations**

Extend the root verification contract so every official language dispatches to a dedicated verify
script. The new workspace verifiers should at minimum check:

- required directories and README exist
- generated directory exists
- semantic reserve exists
- package naming and primary client expectations are declared

- [ ] **Step 2: Run root verification and verify failure**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language rust
node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language java
```

Expected: FAIL because those verify entrypoints do not exist yet.

- [ ] **Step 3: Implement the verify entrypoints**

Each language verifier should follow the same orchestration pattern as existing workspace verifiers:

```js
run('node', [path.join(scriptDir, 'verify-sdk-automation.mjs')], { cwd: workspaceRoot });
assertWorkspacePathExists(languageWorkspaceRoot);
assertReadmeContainsOwnershipRule(languageWorkspaceReadme);
assertAssemblyEntryMatches(language);
```

Native build checks should be capability-aware:

- report `skipped` when the native toolchain is missing
- fail in strict or release verification modes when the native toolchain is required

- [ ] **Step 4: Re-run verification**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language rust --language java --language csharp --language swift --language kotlin --language go --language python
```

Expected: PASS for structural verification in the current workspace state.

- [ ] **Step 5: Commit**

```bash
git add sdks/sdkwork-im-sdk/bin/verify-rust-workspace.mjs sdks/sdkwork-im-sdk/bin/verify-java-workspace.mjs sdks/sdkwork-im-sdk/bin/verify-csharp-workspace.mjs sdks/sdkwork-im-sdk/bin/verify-swift-workspace.mjs sdks/sdkwork-im-sdk/bin/verify-kotlin-workspace.mjs sdks/sdkwork-im-sdk/bin/verify-go-workspace.mjs sdks/sdkwork-im-sdk/bin/verify-python-workspace.mjs sdks/sdkwork-im-sdk/bin/verify-sdk.mjs sdks/sdkwork-im-sdk/bin/verify-sdk.ps1 sdks/sdkwork-im-sdk/bin/verify-sdk.sh
git commit -m "feat(sdk): add multilanguage workspace verification"
```

### Task 5: Extend generation and normalization dispatch for all languages

**Files:**
- Modify: `sdks/sdkwork-im-sdk/bin/generate-sdk.ps1`
- Modify: `sdks/sdkwork-im-sdk/bin/generate-sdk.sh`
- Modify: `sdks/sdkwork-im-sdk/bin/normalize-generated-auth-surface.mjs`
- Modify: `sdks/sdkwork-im-sdk/bin/assemble-sdk.mjs`
- Test: `sdks/sdkwork-im-sdk/bin/generate-sdk.ps1`
- Test: `sdks/sdkwork-im-sdk/bin/generate-sdk.sh`

- [ ] **Step 1: Write failing dispatch checks**

Add guardrails that require:

- per-language output directory mapping
- per-language package naming
- per-language derived input selection where needed
- per-language post-generation normalization routing
- per-language assembly entries

- [ ] **Step 2: Run focused generation commands and verify failure**

Run:

```powershell
powershell -ExecutionPolicy Bypass -File sdks/sdkwork-im-sdk/bin/generate-sdk.ps1 -Languages rust
```

```bash
./sdks/sdkwork-im-sdk/bin/generate-sdk.sh --language java
```

Expected: FAIL because only `typescript` and `flutter` are currently configured.

- [ ] **Step 3: Implement the generation map**

Add official language configuration entries similar to:

```powershell
rust = @{
  OutputDir = Join-Path $WorkspaceDir "sdkwork-im-sdk-rust\generated\server-openapi"
  PackageName = "sdkwork-im-sdk-generated"
  Input = $PreparedInput
}
```

Equivalent entries are required for:

- `java`
- `csharp`
- `swift`
- `kotlin`
- `go`
- `python`

Do not over-specialize the first implementation. Reuse the default derived input unless a
language-specific derived input is actually required.

- [ ] **Step 4: Re-run focused generation**

Run:

```powershell
powershell -ExecutionPolicy Bypass -File sdks/sdkwork-im-sdk/bin/generate-sdk.ps1 -Languages rust,java,csharp,swift,kotlin,go,python
```

Expected: generation dispatch completes or fails only on real per-language generator issues rather
than unsupported-language wrapper rejection.

- [ ] **Step 5: Commit**

```bash
git add sdks/sdkwork-im-sdk/bin/generate-sdk.ps1 sdks/sdkwork-im-sdk/bin/generate-sdk.sh sdks/sdkwork-im-sdk/bin/normalize-generated-auth-surface.mjs sdks/sdkwork-im-sdk/bin/assemble-sdk.mjs
git commit -m "feat(sdk): extend multilanguage generation dispatch"
```

### Task 6: Run all languages against the live schema and capture the first audit baseline

**Files:**
- Create: `sdks/sdkwork-im-sdk/docs/multilanguage-audit-report.md`
- Modify: `sdks/sdkwork-im-sdk/.sdkwork-assembly.json`
- Test: `sdks/sdkwork-im-sdk/bin/generate-sdk.ps1`
- Test: `sdks/sdkwork-im-sdk/bin/verify-sdk.mjs`

- [ ] **Step 1: Generate the full language family from the live service schema**

Run:

```powershell
powershell -ExecutionPolicy Bypass -File sdks/sdkwork-im-sdk/bin/generate-sdk.ps1 -Languages typescript,flutter,rust,java,csharp,swift,kotlin,go,python
```

Expected: every language is attempted from the same live Sdkwork IM schema export.

- [ ] **Step 2: Run full workspace verification**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language typescript --language flutter --language rust --language java --language csharp --language swift --language kotlin --language go --language python
```

Expected: some languages may fail on real generator-quality issues; the important result is that
the failures are classified and reproducible.

- [ ] **Step 3: Record the audit baseline**

Write `multilanguage-audit-report.md` with one section per language:

- generation status
- verification status
- package naming correctness
- client naming correctness
- auth surface correctness
- README correctness
- semantic reserve correctness
- native toolchain verification status
- action required in workspace
- action required in external generator

- [ ] **Step 4: Refresh capability metadata**

Update `.sdkwork-assembly.json` so the first full-language generation audit is reflected in:

- verification status
- docs status
- maturity tier
- release readiness

- [ ] **Step 5: Commit**

```bash
git add sdks/sdkwork-im-sdk/docs/multilanguage-audit-report.md sdks/sdkwork-im-sdk/.sdkwork-assembly.json
git commit -m "docs(sdk): capture multilanguage generation audit baseline"
```

### Task 7: Expand the public docs site to reflect the real multi-language SDK family

**Files:**
- Modify: `docs/sites/sdk/index.md`
- Modify: `docs/sites/sdk/language-support.md`
- Create: `docs/sites/sdk/rust-sdk.md`
- Create: `docs/sites/sdk/java-sdk.md`
- Create: `docs/sites/sdk/csharp-sdk.md`
- Create: `docs/sites/sdk/swift-sdk.md`
- Create: `docs/sites/sdk/kotlin-sdk.md`
- Create: `docs/sites/sdk/go-sdk.md`
- Create: `docs/sites/sdk/python-sdk.md`
- Create: `docs/sites/sdk/generator-boundary.md`
- Modify: `docs/sites/README.md`
- Test: `docs/sites/scripts/verify-sdk-docs.mjs`
- Test: `sdks/sdkwork-im-sdk/bin/verify-docs-contract-tests.mjs`

- [ ] **Step 1: Add failing docs contract assertions**

Make the docs verification require:

- every official language has an SDK page
- language-support lists all official languages
- docs explicitly distinguish Tier A versus Tier B
- generated versus semantic ownership is documented
- package names and primary client names match assembly metadata

- [ ] **Step 2: Run docs verification and verify failure**

Run:

```bash
node docs/sites/scripts/verify-sdk-docs.mjs
node sdks/sdkwork-im-sdk/bin/verify-docs-contract-tests.mjs
```

Expected: FAIL because only TypeScript and Flutter are currently documented.

- [ ] **Step 3: Implement the docs set**

For every language page, document:

- official package or artifact name
- generated/manual boundary
- current maturity tier
- supported runtime targets
- initialization shape
- current semantic-SDK status
- generated transport escape hatch

For Tier B languages, be explicit that the first phase standardizes:

- generation contract
- package structure
- naming
- transport-level usability

and does not yet claim TypeScript-level live runtime parity.

- [ ] **Step 4: Re-run docs verification**

Run:

```bash
node docs/sites/scripts/verify-sdk-docs.mjs
node sdks/sdkwork-im-sdk/bin/verify-docs-contract-tests.mjs
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add docs/sites/sdk/index.md docs/sites/sdk/language-support.md docs/sites/sdk/rust-sdk.md docs/sites/sdk/java-sdk.md docs/sites/sdk/csharp-sdk.md docs/sites/sdk/swift-sdk.md docs/sites/sdk/kotlin-sdk.md docs/sites/sdk/go-sdk.md docs/sites/sdk/python-sdk.md docs/sites/sdk/generator-boundary.md docs/sites/README.md
git commit -m "docs(sdk): add multilanguage sdk site coverage"
```

### Task 8: Stabilize the maintainer docs and strict verification story

**Files:**
- Create: `sdks/sdkwork-im-sdk/docs/multilanguage-generator-standard.md`
- Modify: `sdks/sdkwork-im-sdk/README.md`
- Modify: `docs/superpowers/specs/2026-04-16-sdkwork-im-multilanguage-sdk-generator-design.md`
- Modify: `docs/superpowers/plans/2026-04-16-sdkwork-im-multilanguage-sdk-generator.md`
- Test: `sdks/sdkwork-im-sdk/bin/verify-internal-docs.mjs`

- [ ] **Step 1: Write the missing maintainer-doc assertions**

Add verification expectations so internal docs must cover:

- live schema refresh as a required generation step
- generated/manual boundary rules
- Tier A versus Tier B maturity definition
- current writable-scope limitation for generator-core changes
- audit-report maintenance rules

- [ ] **Step 2: Run internal-doc verification and verify failure**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-internal-docs.mjs
```

Expected: FAIL until the maintainer docs mention the new standard.

- [ ] **Step 3: Implement the maintainer docs**

Write `multilanguage-generator-standard.md` so a future maintainer can answer:

- how a language becomes official in this workspace
- which files must be updated
- which verification entrypoint must be added
- how capability metadata is refreshed
- how to tell workspace gaps from generator-core gaps

- [ ] **Step 4: Re-run internal-doc verification**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-internal-docs.mjs
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add sdks/sdkwork-im-sdk/docs/multilanguage-generator-standard.md sdks/sdkwork-im-sdk/README.md docs/superpowers/specs/2026-04-16-sdkwork-im-multilanguage-sdk-generator-design.md docs/superpowers/plans/2026-04-16-sdkwork-im-multilanguage-sdk-generator.md
git commit -m "docs(sdk): formalize multilanguage generator standard"
```

### Task 9: Run the full multilanguage verification baseline

**Files:**
- Modify: `sdks/sdkwork-im-sdk/docs/multilanguage-audit-report.md`
- Modify: `sdks/sdkwork-im-sdk/.sdkwork-assembly.json`

- [ ] **Step 1: Run root generation from the live service**

Run:

```powershell
powershell -ExecutionPolicy Bypass -File sdks/sdkwork-im-sdk/bin/generate-sdk.ps1 -Languages typescript,flutter,rust,java,csharp,swift,kotlin,go,python
```

Expected: the live service schema is refreshed and all supported languages are attempted.

- [ ] **Step 2: Run root verification serially**

Run:

```bash
node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language typescript
node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language flutter
node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language rust
node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language java
node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language csharp
node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language swift
node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language kotlin
node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language go
node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language python
```

Expected: PASS where the language meets the current workspace standard, or a reproducible
classification of generator or workspace defects where it does not.

- [ ] **Step 3: Run docs verification**

Run:

```bash
node docs/sites/scripts/verify-sdk-docs.mjs
node sdks/sdkwork-im-sdk/bin/verify-docs-contract-tests.mjs
node sdks/sdkwork-im-sdk/bin/verify-internal-docs.mjs
```

Expected: PASS

- [ ] **Step 4: Refresh the audit report and capability matrix**

Update the audit report and assembly metadata to reflect the final baseline after all fixes in this
plan.

- [ ] **Step 5: Commit**

```bash
git add sdks/sdkwork-im-sdk/docs/multilanguage-audit-report.md sdks/sdkwork-im-sdk/.sdkwork-assembly.json
git commit -m "test(sdk): record multilanguage workspace verification baseline"
```

## Notes For The Implementer

- Do not edit `generated/server-openapi/**` manually.
- Use the live schema refresh flow as the source of truth before generation.
- Keep TypeScript behavior as the approved public baseline. Do not regress it toward older,
  context-first callback proposals.
- When a generated language output is structurally wrong but the fix belongs in the external
  generator, record the issue in the audit report and enforce the expectation through workspace
  verification rather than inventing hidden ad hoc fixes.
- Native language verification should distinguish `passed`, `failed`, and `skipped`; do not treat
  missing toolchains as invisible success.
