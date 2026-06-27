# Generator Boundary

The IM SDK family uses one cross-language rule set for generated code and handwritten SDK
behavior.

## Source Of Truth

Every official language starts from the same live contract export:

- endpoint: `/im/v3/openapi.json`
- checked-in authority snapshot: `sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml`

The root wrappers refresh that authority snapshot before generation. Language-specific SDK delivery
then starts from the same contract instead of diverging per workspace.

## What Generation Owns

The generator-owned boundary is always:

- `generated/server-openapi`

That boundary owns:

- HTTP route groups
- DTOs and models
- generated package metadata
- generated transport configuration
- generated low-level README and smoke scaffolding

Do not hand-edit `generated/server-openapi`.

## What The Semantic Layer Owns

The manual-owned semantic boundary is:

- `composed` for every non-TypeScript language
- root `src/**` outside `src/generated/**` for TypeScript

That layer owns:

- business-facing client names
- message-first or workflow-first helpers
- live runtime orchestration
- durable replay helpers
- RTC helpers
- any future semantic wrappers that hide raw route-group calls

## TypeScript Special Case

TypeScript keeps the generator authoring source in `generated/server-openapi`, but the public
consumer package assembles that transport into `src/generated/**`.

That is why the TypeScript consumer contract is still a single package:

- `@sdkwork/im-sdk`
- `ImSdkClient`

Consumers use one package, while maintainers still keep a hard generator boundary under
`generated/server-openapi`.

## WebSocket And Live Runtime Rule

WebSocket receive, live runtime orchestration, and other non-HTTP behaviors are manual-owned. They
do not belong in the OpenAPI-generated transport layer.

Examples:

- TypeScript `sdk.connect(...)`
- TypeScript `live.messages.on(...)`
- any future semantic WebSocket or live runtime for Rust, Java, C#, Swift, Kotlin, Go, or Python

OpenAPI generation can model the HTTP coordination surfaces around realtime delivery. The actual
live runtime remains a manual semantic concern.

## Verification Ownership

The generator boundary is also reflected in how verification is split:

- root automation verification checks wrapper wiring, workspace assembly metadata, and required
  normalization steps
- language workspace verification checks language-specific README contracts, package structure, and
  public boundary promises
- docs-site verification checks that the published SDK and API pages stay synchronized with the
  checked-in workspace contract

That split is deliberate. It avoids putting the same TypeScript or Flutter README wording checks in
multiple verifiers while still keeping the live-schema generation chain tightly enforced.

## Why This Boundary Exists

This split keeps the SDK family stable:

- generated transport can be refreshed from the live schema without rewriting IM consumer design
- semantic SDKs can improve ergonomics without forking raw OpenAPI output
- docs can be honest about what is route-generated and what is handwritten
- verification can detect generator drift separately from semantic design drift

## Use This Rule When Extending A Language

When a new language grows beyond transport-standardized delivery, keep the rule intact:

1. generate transport into `generated/server-openapi`
2. build semantic helpers above it under `composed`
3. document the public business-facing client separately from the raw generated transport

## What To Read Next

- Read [App SDK](/sdk/app-sdk) when you need the product-facing runtime contract above the
  generator-owned transport layer.
- Read [TypeScript SDK](/sdk/typescript-sdk) when you need the reference single-package assembly
  that exposes generated transport under `src/generated/**`.
- Read [Language Support](/sdk/language-support) when you need the full cross-language maturity
  matrix and current delivery status.
- Read [Backend SDK](/sdk/backend-sdk) when your consumer boundary is governance, control-plane, operator, or admin
  rather than the public app runtime.
