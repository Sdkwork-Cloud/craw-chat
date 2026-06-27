# Sdkwork IM Admin SDK Standardization Design

## Goal

Build `sdkwork-control-plane-sdk` into the single professional admin SDK family for Sdkwork IM, aligned with the mature `sdkwork-im-sdk` standard:

- real OpenAPI 3.x schema fetched from a running admin or control-plane service
- strict `generated` versus `composed` ownership
- language workspaces for TypeScript and Flutter
- root generation, verification, assembly, and documentation workflows
- direct consumer adoption by `apps/control-plane` instead of maintaining a second handwritten API SDK

## Scope

This round covers:

- standardizing `sdks/sdkwork-control-plane-sdk` as a real SDK workspace
- fetching the latest admin or control-plane OpenAPI 3.x schema from a running service before generation
- introducing the same root `openapi/`, `bin/`, verification, and assembly model already used by `sdkwork-im-sdk`
- generating TypeScript and Flutter admin transport SDKs under `generated/server-openapi`
- adding a manual-owned `composed` product SDK layer for both languages
- making `apps/control-plane` consume the new admin SDK directly for real control-plane access
- retiring `apps/control-plane/packages/sdkwork-control-plane-admin-api` as a standalone handwritten SDK boundary

This round does not cover:

- inventing admin websocket adapters that do not exist in the real contract
- keeping dual long-term client abstractions for the same admin HTTP surface
- speculative mobile admin UI work beyond the SDK package boundary

## Design Principles

- The admin SDK must have one authoritative client boundary, not a generated SDK plus a second handwritten "admin-api SDK" in the app.
- Generated code must live only under `generated/server-openapi`.
- Manual business-facing ergonomics must live only under `composed`.
- The generator must consume a real OpenAPI 3.x schema fetched from a running service, then preserve a normalized checked-in authority snapshot for repeatability and review.
- `apps/control-plane` must validate the SDK through real consumption, not only through synthetic package smoke tests.
- Package naming, client naming, folder layout, verification, and docs must align with the existing `sdkwork-im-sdk` standard wherever the admin contract allows it.

## Decisions

- Workspace family: `sdks/sdkwork-control-plane-sdk` becomes the only official admin SDK workspace.
- Runtime contract source: start the admin or control-plane service, fetch the latest OpenAPI 3.x schema, normalize it, and store the normalized result under the admin SDK workspace.
- Authority contract: keep a checked-in admin OpenAPI authority snapshot in `sdks/sdkwork-control-plane-sdk/openapi/`, but treat it as a normalized runtime snapshot rather than a manually invented contract.
- Generator input: derive generator-friendly sdkgen inputs from the checked-in normalized authority snapshot.
- TypeScript public package names:
  - generated: `@sdkwork/control-plane-backend-sdk`
  - composed: `@sdkwork/control-plane-sdk`
- TypeScript client naming:
  - composed entrypoint: `ControlPlaneSdkClient`
- Flutter public package names:
  - generated: `control_plane_backend_sdk`
  - composed: `control_plane_sdk`
- App adoption: `apps/control-plane` will consume the formal admin SDK directly. The existing `sdkwork-control-plane-admin-api` package will not survive as a parallel API SDK.
- UI integration boundary: if the admin app still needs thin UI-facing adapters such as React query helpers, loaders, or sandbox bridges, they stay app-local and must depend on the formal admin SDK instead of re-defining transport, DTOs, or auth logic.

## Architecture

The admin SDK root mirrors the app SDK root:

```text
sdks/sdkwork-control-plane-sdk/
  openapi/
  bin/
  sdkwork-control-plane-sdk-typescript/
  sdkwork-control-plane-sdk-flutter/
  README.md
  .sdkwork-assembly.json
```

The root owns:

- runtime schema capture
- authority snapshot normalization
- derived sdkgen input generation
- unified language generation wrappers
- post-generation assembly metadata
- verification orchestration
- workspace-level documentation

Each language workspace follows the same split:

- `generated/server-openapi`
  Generator-owned transport SDK output
- `composed`
  Manual-owned ergonomic admin SDK layer
- `bin/`
  Thin forwarding scripts back to the root wrappers
- `README.md`
  Consumer-facing language documentation

## Package Ownership Model

### Root Workspace

The root workspace owns:

- `openapi/control-plane.openapi.yaml`
  The normalized checked-in authority snapshot fetched from the running service.
- `openapi/control-plane.sdkgen.yaml`
  The derived generator input.
- `bin/prepare-openapi-source.mjs`
  Refreshes the derived sdkgen input from the authority snapshot.
- `bin/fetch-openapi-source.mjs`
  Starts or targets the correct runtime endpoint, fetches the live admin OpenAPI schema, validates it, and writes the normalized authority snapshot.
- `bin/generate-sdk.ps1` and `bin/generate-sdk.sh`
  The only supported regeneration entrypoints.
- `bin/verify-sdk.mjs`
  Runs the admin SDK verification chain.
- `bin/assemble-sdk.mjs`
  Refreshes `.sdkwork-assembly.json` for the workspace.

### TypeScript Workspace

The TypeScript workspace owns:

- `sdkwork-control-plane-sdk-typescript/generated/server-openapi`
  Generator-owned transport package.
- `sdkwork-control-plane-sdk-typescript/composed`
  Manual-owned product SDK package exposing `ControlPlaneSdkClient`.

The composed TypeScript package should provide:

- flat client creation options such as `baseUrl`, `authToken`, `headers`, and `timeout`
- semantic admin modules grouped around the real control-plane domain
- thin re-exports of generated types where useful
- consumer-focused README examples

### Flutter Workspace

The Flutter workspace owns:

- `sdkwork-control-plane-sdk-flutter/generated/server-openapi`
  Generator-owned transport package.
- `sdkwork-control-plane-sdk-flutter/composed`
  Manual-owned product SDK package exposing `ControlPlaneSdkClient`.

The Flutter package should mirror the TypeScript ownership and naming model where practical.

## Runtime Schema Capture Flow

The admin SDK generation process must start from the running service rather than from a stale handwritten file.

Required flow:

1. Start the relevant admin or control-plane service using the approved workspace entrypoint.
2. Request the live OpenAPI 3.x schema from the running service.
3. Validate that the fetched schema is OpenAPI 3.x and contains the expected admin tags and paths.
4. Normalize unstable fields if needed, such as server URLs or generated descriptions that should not drift between runs.
5. Write the normalized result to `openapi/control-plane.openapi.yaml`.
6. Derive `openapi/control-plane.sdkgen.yaml`.
7. Generate TypeScript and Flutter `generated/server-openapi` outputs.
8. Assemble workspace metadata and run verification.

This preserves two required properties at once:

- the contract reflects real runtime behavior
- the repository still keeps an auditable checked-in authority snapshot

## Composed SDK Contract

The generated transport layer is not the preferred app-facing admin surface. The `composed` layer is.

The admin composed SDK should expose:

- `ControlPlaneSdkClient`
- clear module grouping for real admin domains
- stable auth and transport configuration
- generated-type reuse instead of DTO forks
- no fake realtime abstractions that are not present in the admin contract

The composed layer must stay thin. It should not:

- replace generated DTOs with a second handwritten model tree
- re-implement HTTP transport
- create a second authentication system
- drift from the generated route set

## Admin App Migration Strategy

The current `apps/control-plane/packages/sdkwork-control-plane-admin-api` package is a handwritten transport boundary with many route wrappers. That role must be removed.

Migration rules:

- `sdkwork-control-plane-admin-api` stops being the canonical admin transport client.
- `apps/control-plane` business packages should import the formal admin SDK instead of a local handwritten API SDK.
- If some app-local integration helper is still needed, it must be renamed and reduced to a thin UI integration helper that depends on the formal SDK.
- Route URLs, auth headers, response models, and error handling must come from the formal admin SDK rather than remaining duplicated in the admin app.

This removes the long-term risk of:

- duplicated route maps
- divergent DTOs
- mismatched auth behavior
- separate docs for "SDK" and "admin-api"
- false confidence from a generated SDK that no real consumer uses

## Verification Strategy

Completion requires both workspace verification and real consumer verification.

### Root Verification

The admin root verification chain should prove:

- live schema fetch works
- normalized authority snapshot is stable
- derived sdkgen input is regenerated deterministically
- TypeScript and Flutter generation both succeed
- `.sdkwork-assembly.json` reflects the generated and composed packages accurately

### TypeScript Verification

The TypeScript admin workspace should follow the same professional standard as the app SDK:

- generated package build or package verification
- public API boundary checks
- composed package type-check and build
- smoke usage tests for `ControlPlaneSdkClient`
- consumer validation through `apps/control-plane`

### Flutter Verification

The Flutter admin workspace should include:

- generated model and package metadata checks
- public API boundary checks
- composed package parity checks
- optional native Dart or Flutter analysis when explicitly requested

### Consumer Verification

`apps/control-plane` must verify that:

- the old handwritten transport boundary is removed or downgraded to thin app-local integration only
- admin app code compiles against the formal admin SDK
- representative admin workflows still function against the real SDK boundary

## Documentation Strategy

The admin SDK docs should match the app SDK documentation quality bar:

- root workspace README explaining authority snapshot versus derived sdkgen input
- language READMEs for TypeScript and Flutter
- docs site entries that explain admin SDK scope, package names, ownership boundaries, and verification
- explicit statements that the admin SDK reflects real control-plane routes rather than fictional or stale interfaces

## Risks And Controls

- Risk: the running service schema and checked-in authority snapshot drift.
  Control: make live fetch and normalization part of the standard generation flow and verify deterministic output.

- Risk: `apps/control-plane` keeps local route wrappers after the SDK exists.
  Control: remove `sdkwork-control-plane-admin-api` as a transport authority and verify direct SDK consumption.

- Risk: generated and composed layers blur together.
  Control: keep all generated output inside `generated/server-openapi` and all ergonomic code inside `composed`.

- Risk: TypeScript and Flutter package identity diverges from the app SDK family.
  Control: mirror naming, layout, verification, and docs standards already proven in `sdkwork-im-sdk`.

- Risk: schema fetch depends on an undocumented manual runtime setup.
  Control: document and script the service startup and fetch path in root wrappers.

## Success Criteria

This round is complete only when all of the following are true:

- `sdkwork-control-plane-sdk` has the same professional workspace standard as `sdkwork-im-sdk`
- the admin SDK contract is fetched from a running service and normalized into a checked-in authority snapshot
- TypeScript and Flutter both have `generated/server-openapi + composed` layered workspaces
- `ControlPlaneSdkClient` becomes the preferred public client in both languages
- `apps/control-plane` consumes the formal admin SDK instead of maintaining a separate handwritten API SDK boundary
- verification proves both package health and real consumer integration
