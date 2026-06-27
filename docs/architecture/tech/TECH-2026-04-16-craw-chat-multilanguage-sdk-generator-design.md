> Migrated from `docs/superpowers/specs/2026-04-16-craw-chat-multilanguage-sdk-generator-design.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Sdkwork IM Multilanguage SDK Generator Design

## Goal

Define the production standard for expanding `sdkwork-im-sdk` from a TypeScript-first
workspace into a real multi-language SDK family that is generated from the live Sdkwork IM OpenAPI
3.x contract and then assembled into language-appropriate consumer SDKs.

This design treats the current TypeScript SDK as the baseline public standard for:

- one business-facing SDK family per language
- one primary client per SDK family derived from the business package name
- one generated HTTP contract layer derived from OpenAPI 3.x
- one non-generated semantic layer for live runtime, message ergonomics, RTC, and product-facing
  workflows
- one root workspace that owns refresh, generation, normalization, verification, and docs

The immediate objective is to make `sdkwork-im-sdk` the contract and verification hub for
all supported languages, not just `typescript` and `flutter`.

## Current Baseline

The TypeScript SDK is already the strongest implementation in this workspace and therefore defines
the current public standard that other languages should align with.

Accepted TypeScript baseline:

- installable consumer package: `@sdkwork/im-sdk`
- primary client: `ImSdkClient`
- synchronous construction: `new ImSdkClient({...})`
- flat config for normal use: `baseUrl`, `apiBaseUrl`, `websocketBaseUrl`, `authToken`, and
  runtime adapters at the top level
- message-first outbound workflow:
  - `sdk.createTextMessage(...)`
  - `sdk.createImageMessage(...)`
  - `sdk.createCustomMessage(...)`
  - `sdk.send(message)`
- payload-first live receive workflow:
  - `live.messages.on((message, context) => ...)`
  - `live.data.on((data, context) => ...)`
  - `live.signals.on((signal, context) => ...)`
  - `live.events.on((context) => ...)`
  - `live.lifecycle.onStateChange(...)`
- explicit separation of live push and durable replay:
  - `sdk.connect(...)`
  - `sdk.sync.catchUp(...)`
  - `sdk.sync.ack(...)`

This design must not regress the live API back to the older context-first `live.onMessage((ctx) =>
...)` proposal. The current TypeScript baseline is payload-first by domain stream and remains the
cross-language reference point.

## Non-Goals

- preserve the current partial multi-language state as the long-term architecture
- make generated transport the default public integration surface
- force every language to reach full TypeScript semantic depth in the first phase
- move WebSocket receive orchestration into OpenAPI generation
- maintain a local fork of `sdkwork-sdk-generator` inside `apps/sdkwork-im`
- treat checked-in OpenAPI YAML as the source of truth when a live service schema is available

## Constraints

### Writable Scope

The current writable scope is limited to:

- `apps/sdkwork-im/**`

The actual generator source currently lives outside that writable root:

- `sdk/sdkwork-sdk-generator`

This means the first implementation phase can fully standardize the IM SDK workspace,
language workspaces, docs, verification, and integration contracts, but it cannot directly patch
the external generator source in this environment.

### Existing Generator Support

The external generator already advertises support for these languages:

- `typescript`
- `flutter`
- `rust`
- `java`
- `csharp`
- `swift`
- `kotlin`
- `go`
- `python`

The current Sdkwork IM workspace only formally integrates:

- `typescript`
- `flutter`

The design therefore focuses on turning the declared generator language set into a real,
workspace-owned, verified SDK family.

## Design Summary

Adopt a root-workspace-first multi-language SDK standard:

1. `sdkwork-im-sdk` is the only official generation, verification, and docs hub.
2. Every language workspace must expose a hard generated versus non-generated boundary.
3. OpenAPI 3.x generation owns only the HTTP transport contract layer.
4. Semantic SDK behavior such as message builders, live receive, sync replay, RTC helpers, and
   AI-era message families remains non-generated.
5. The root workspace must refresh the live service schema before any generation run.
6. Every language must participate in the same generation, normalization, assembly, verification,
   and docs-sync pipeline.
7. The first phase standardizes every supported language at the workspace level, while only
   `typescript`, `flutter`, and `rust` are expected to reach full semantic-SDK depth.

## Target Language Set

The official language set for `sdkwork-im-sdk` becomes:

- `typescript`
- `flutter`
- `rust`
- `java`
- `csharp`
- `swift`
- `kotlin`
- `go`
- `python`

These languages must be treated as first-class workspace entries in:

- root generation wrappers
- root verification wrappers
- assembly metadata
- internal maintainer docs
- public SDK docs
- capability reporting

## SDK Workspace Standard

### Root Workspace Layout

The root workspace should converge on this shape:

```text
sdkwork-im-sdk/
  openapi/
  bin/
  docs/
  sdkwork-im-sdk-typescript/
  sdkwork-im-sdk-flutter/
  sdkwork-im-sdk-rust/
  sdkwork-im-sdk-java/
  sdkwork-im-sdk-csharp/
  sdkwork-im-sdk-swift/
  sdkwork-im-sdk-kotlin/
  sdkwork-im-sdk-go/
  sdkwork-im-sdk-python/
  .sdkwork-assembly.json
  README.md
```

Rules:

- the root workspace owns the authority OpenAPI snapshot and derived generator inputs
- the root workspace is the only supported place to regenerate all languages
- language workspaces may provide thin forwarding scripts only
- language workspaces must not invent generation rules that bypass the root wrappers

### Per-Language Boundary

Every language workspace must implement the same ownership split even if the exact directory names
vary by ecosystem:

```text
<language-workspace>/
  generated/
    server-openapi/
  composed/ or equivalent manual layer
  bin/
  README.md
```

Rules:

- `generated/server-openapi/**` is generator-owned only
- the semantic layer is manual-owned only
- generated code must not import semantic business modules
- semantic business modules may depend on generated exports
- docs and verification must make this ownership split explicit

## Naming Standard

### Naming Source

Package and client naming are derived from the SDK workspace family name:

- workspace family: `sdkwork-im-sdk`
- business family name: `im-sdk`

The public consumer-facing names must be business-facing, not transport-facing.

### Target Names

The following names define the intended consumer-facing standard:

- TypeScript package: `@sdkwork/im-sdk`
- TypeScript primary client: `ImSdkClient`
- Flutter package: `im_sdk`
- Flutter primary entry type: `ImSdkClient` or the nearest idiomatic equivalent
- Rust crate: `im_sdk`
- Rust primary client: `ImSdkClient` or `SdkworkImClient`
- Java artifact family: must preserve `im-sdk` in the artifact identity
- C# package family: must preserve `ImSdk` in the package identity
- Swift package/product: must preserve `ImSdk`
- Kotlin artifact family: must preserve `im-sdk`
- Go module path: must preserve `im-sdk`
- Python package: `sdkwork_im_sdk` or an equally explicit `im_sdk` package naming
  rule, depending on workspace conventions

Generated transport package names may remain transport-oriented where ecosystem tooling requires it,
but the normal application-facing entrypoint must stay business-oriented.

## Layering Standard

### Generated Contract Layer

This layer is produced from OpenAPI 3.x and owns:

- DTOs and models
- request and response types
- route-group API clients
- transport configuration primitives
- generated auth/config/error primitives where applicable
- generated README and package metadata
- generated smoke-test scaffolding where available

This layer must not own:

- message builders
- `client.send(message)` business workflow
- WebSocket receive runtime
- durable replay orchestration
- RTC domain helpers
- AI-era message families as semantic business objects
- app-facing lifecycle naming

### Semantic SDK Layer

This layer is handwritten and owns:

- the primary SDK client
- semantic modules
- business-facing naming
- message creation helpers
- live receive runtime
- durable replay helpers
- RTC helpers
- custom message and AI message families
- high-level upload and send flows
- docs-first integration examples

This split is non-negotiable across languages.

## Language Delivery Tiers

Not every language should be forced into full semantic parity in the first phase. The workspace
must explicitly distinguish between complete semantic SDKs and standardized transport SDKs.

### Tier A: Complete Semantic SDKs

First-phase complete semantic SDK targets:

- `typescript`
- `flutter`
- `rust`

Required first-phase capabilities:

- primary business client
- message-first send API
- live receive API
- sync replay helpers
- RTC semantic facade
- custom message support
- AI-era message families
- detailed consumer docs

### Tier B: Standardized Transport SDKs With Semantic Reserve

First-phase standardized transport SDK targets:

- `java`
- `csharp`
- `swift`
- `kotlin`
- `go`
- `python`

Required first-phase capabilities:

- standard naming
- standard generated/manual boundary
- generated transport package
- thin business-facing facade or reserved semantic layer structure
- smoke tests
- package metadata validation
- docs that accurately describe the current maturity level

These languages should not fake feature completeness in docs before the semantic layer exists.

## Generation Pipeline Standard

Every generation run must use the live service schema as its input source of truth.

Required root pipeline:

1. Verify or start the target Sdkwork IM service.
2. Fetch the live OpenAPI 3.x schema from `/im/v3/openapi.json`.
3. Refresh the checked-in authority snapshot under `openapi/`.
4. Derive generator-specific normalized inputs from the authority schema.
5. Resolve one unified SDK version for the workspace generation run.
6. Generate each requested language into its `generated/server-openapi` boundary.
7. Run post-generation normalization for package metadata, auth surface, and any
   language-specific generator cleanup.
8. Run language-specific assembly where the publishable package differs from the raw generated
   package.
9. Run language-specific verification.
10. Run docs and contract verification.
11. Refresh assembly and capability metadata.

Rules:

- no language may generate directly from a stale local file when a live schema export is available
- no language may skip post-generation normalization if the generator output is not yet aligned
  with Sdkwork IM standards
- generation is not complete until verification and assembly pass

## Root Wrapper Standard

The root wrappers under `bin/` must be the official generation and verification entrypoints for all
languages.

### Required Root Generation Behavior

`generate-sdk.ps1` and `generate-sdk.sh` must:

- accept the full language set
- validate unknown languages consistently
- refresh the live schema first
- run per-language generation into the correct workspace boundary
- run per-language normalization
- run per-language verification where applicable
- refresh assembly metadata at the end

### Required Root Verification Behavior

`verify-sdk.mjs`, `verify-sdk.ps1`, and `verify-sdk.sh` must:

- accept the full language set
- default to the full supported set or an explicitly documented default subset
- run workspace guardrails first
- dispatch into language-specific verification entrypoints
- run assembly refresh at the end
- fail loudly on drift in package names, client names, docs, generated boundaries, or verification
  contracts

## Validation Matrix

Every language must pass four categories of verification.

### 1. Workspace Contract Verification

Required checks:

- required directories exist
- required forwarding scripts exist
- `generated/server-openapi` is present
- semantic layer or semantic reserve exists
- README documents generated/manual ownership
- root assembly metadata references the workspace correctly

### 2. Generated Transport Verification

Required checks:

- package metadata is valid
- package or artifact naming matches the language contract
- generated primary client naming is normalized where required
- auth surface is aligned with bearer-token app SDK rules
- generated README examples do not contradict the approved SDK standard
- generated package can pass at least build or package-level smoke validation

### 3. Semantic Layer Verification

Required checks:

- semantic code only imports the generated public root exports
- no semantic code depends on generator-private file paths
- primary client and semantic exports exist
- smoke tests prove the consumer package shape is usable
- Tier A languages additionally verify message, live, sync, and RTC contracts

### 4. Docs Contract Verification

Required checks:

- package names in docs match implementation
- primary client names in docs match implementation
- code snippets match actual public exports
- API reference and SDK reference do not contradict each other
- language capability docs match actual maturity level

## Capability Matrix Standard

The workspace must maintain a machine-readable and human-readable capability matrix.

### Machine-Readable

Assembly and capability metadata should continue to live in:

- `.sdkwork-assembly.json`

It must be extended so every language entry records at least:

- language
- workspace root
- generated package manifest
- semantic package manifest or semantic reserve root
- package name
- primary client name
- maturity tier
- generated transport status
- semantic layer status
- verification status
- docs status

### Human-Readable

Maintainer-facing and public docs must summarize at least:

- language
- maturity tier
- generated transport support
- semantic client status
- message builders
- live runtime
- sync replay
- RTC facade
- browser or native runtime coverage
- docs completeness
- release readiness

## Documentation Standard

Docs must describe the real maturity and real boundaries of each language.

### Root Docs

The root workspace docs must add or maintain:

- multi-language workspace overview
- generated versus semantic ownership rules
- generation and verification flow
- language capability matrix
- maintainer notes for generator integration

### Public SDK Docs

The public docs site must add or update:

- a multi-language SDK landing page
- one page per supported language
- a generated versus semantic boundary page
- cross-language standards pages for:
  - message model
  - live receive model
  - sync replay model
  - RTC model
  - custom and AI message families

Editorial rules:

- never document an unimplemented semantic surface as if it already exists
- always distinguish target standard from currently shipped maturity
- prefer workflow-first SDK docs over route-group-first docs
- keep API reference and SDK reference aligned

## Language-Specific First-Phase Deliverables

### TypeScript

Preserve and continue hardening the current standard:

- one installable package
- root semantic SDK plus `src/generated/**`
- browser and Node.js support
- message-first send
- payload-first domain-stream live receive
- sync replay
- RTC helpers
- detailed docs and contract tests

### Flutter

Bring the existing workspace to the same generator-contract standard as TypeScript, while
continuing to use a generated transport package plus composed consumer package model.

### Rust

Promote Rust to a first-class semantic SDK target:

- generated transport crate
- composed semantic crate
- stable business client
- builder helpers
- smoke tests
- verification and docs

### Java, C#, Swift, Kotlin, Go, Python

Standardize the workspaces so they become real generated SDK outputs under the root workspace,
with:

- formal workspace registration
- generated output directories
- thin forwarding scripts
- package metadata and naming validation
- README and docs entries
- semantic reserve directories or thin facades
- explicit maturity reporting

## Implementation Phases

### Phase 1: Standardize the Sdkwork IM Multi-Language Workspace

- extend root scripts to recognize every supported language
- add or scaffold every language workspace
- extend assembly metadata
- add per-language verification entrypoints
- align root README and maintainer docs
- add multi-language docs and capability matrix

### Phase 2: Run Real Generation and Capture Gaps

- run every language against the live Sdkwork IM schema
- record generation and verification results
- identify naming, auth, structure, and docs drift
- turn generator shortcomings into explicit workspace-level problem reports and guardrails

### Phase 3: Deepen Tier A Semantic SDKs

- keep TypeScript aligned and stable
- promote Rust to semantic parity targets
- harden Flutter semantic consistency

### Phase 4: Backport Standards Into The External Generator

Once writable access to `sdkwork-sdk-generator` is available:

- move validated naming rules into the generator
- move validated generated-namespace rules into the generator
- move validated package metadata rules into the generator
- keep Sdkwork IM as the contract-test workspace for regression coverage

## Acceptance Criteria

This design is successful when:

- the root IM SDK workspace can generate all supported languages from the live service
  schema
- every supported language is represented in generation, verification, assembly, and docs
- every language has a documented generated/manual ownership boundary
- naming is business-facing and consistent with `im-sdk`
- the TypeScript baseline remains the reference standard and is not regressed
- verification can identify generation defects instead of letting them silently drift
- docs accurately reflect both the common standard and per-language maturity
- the workspace can serve as a high-quality contract-test harness for future generator work

## Decision

The correct next step is to treat `sdkwork-im-sdk` as the official multi-language SDK
standard workspace for Sdkwork IM, expand it to cover the full generator language set, and use it to
lock package structure, naming, verification, and documentation standards before generator-core
changes are made outside the writable scope.

