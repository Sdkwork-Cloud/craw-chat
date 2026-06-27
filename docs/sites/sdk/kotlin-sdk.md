# Kotlin SDK

The Kotlin workspace is a Tier B member of the `im-sdk` business SDK family and currently
ships as a transport-standardized JVM lane.

## Current Delivery Reality

This page describes the current checked-in repo contract validated on 2026-04-16. It does not by
itself claim a published semantic Kotlin artifact.

Today the verified Kotlin entrypoint is the generated artifact under `generated/server-openapi`.
The semantic Kotlin artifact remains reserved under `composed`.

## Package Contract

| Concern | Value |
| --- | --- |
| Maturity tier | Tier B |
| Generated transport artifact | `com.sdkwork:im-sdk-generated` |
| Raw generated package root | `com.sdkwork.im.generated` |
| Raw generated client | `ImTransportClient` |
| Reserved semantic artifact | `com.sdkwork:im-sdk` |
| Target business client | `ImSdkClient` |
| Generator-owned boundary | `generated/server-openapi` |
| Manual semantic boundary | `composed` |

## What Ships Today

- live-schema generation from the Sdkwork IM OpenAPI 3.x export
- verified generated artifact naming and assembly metadata
- a stable split between `generated/server-openapi` and `composed`
- a raw generated transport client named `ImTransportClient`

For exact installation and raw API examples, use `generated/server-openapi/README.md` as the
transport reference.

## Raw Generated Client

If you are integrating Kotlin today, start from the generated artifact and
`com.sdkwork.im.generated.ImTransportClient`.

- generated artifact: `com.sdkwork:im-sdk-generated`
- raw generated client: `ImTransportClient`
- reserved semantic artifact: `com.sdkwork:im-sdk`

That is the checked-in Kotlin surface the workspace verifies today.

## API Reference Map

Use `generated/server-openapi/README.md` together with `ImTransportClient` when you need the
exact Kotlin route-group names and DTO entrypoints. Use the map below to jump from transport
concern to the matching HTTP reference:

| Transport concern | Generated transport focus today | Exact API reference |
| --- | --- | --- |
| SDKWork appbase credential pass-through and portal reads | generated token hooks and portal route groups on `ImTransportClient` | [Portal Access](/api-reference/app/portal-access) |
| Conversation lifecycle and handoff | conversation route groups on `ImTransportClient` | [Conversations](/api-reference/im/conversations) |
| Membership and read cursors | conversation membership and read-state route groups | [Membership and Read State](/api-reference/im/membership-and-read-state) |
| Message send payloads and timeline schemas | message route groups and DTOs | [Messages](/api-reference/im/messages) |
| Upload and attachment lifecycle | media route groups and DTOs | [Media](/api-reference/im/media) |
| Realtime Presence, presence, and realtime coordination | session, presence, and realtime route groups | [Realtime Presence](/api-reference/im/session-and-realtime) |
| IM call lifecycle and signaling-side HTTP operations | calls route groups | [Calls](/api-reference/im/calls) |
| Stream ingestion and checkpoints | stream route groups | [Streams](/api-reference/im/streams) |

This keeps the Kotlin page precise: the repo-standard delivery today is transport-first, so the
API reference plus `generated/server-openapi/README.md` remains the exact route authority until a
future semantic `ImSdkClient` is implemented under `composed`.

## What Is Not Shipped Yet

- no checked-in semantic Kotlin artifact that already exposes `ImSdkClient`
- no handwritten message-first business layer above generated route groups
- no delivered websocket live runtime abstraction above the generated transport artifact

Treat this page as a repo contract for current Kotlin delivery, not as a claim of TypeScript-level
semantic completeness.

## When To Use `composed`

Use `composed` only when you are intentionally building the future semantic Kotlin layer:

- `ImSdkClient`
- business wrappers above raw route groups
- higher-level message helpers
- live runtime abstractions

Do not hand-edit generated Kotlin files under `generated/server-openapi`.

## Generate And Verify

Root workspace:

```powershell
powershell -ExecutionPolicy Bypass -File .\sdks\sdkwork-im-sdk\bin\generate-sdk.ps1 -Languages kotlin
node .\sdks\sdkwork-im-sdk\bin\verify-sdk.mjs --language kotlin
```

Kotlin workspace wrappers:

```powershell
.\sdks\sdkwork-im-sdk\sdkwork-im-sdk-kotlin\bin\sdk-gen.ps1
.\sdks\sdkwork-im-sdk\sdkwork-im-sdk-kotlin\bin\sdk-verify.ps1
```

```bash
./sdks/sdkwork-im-sdk/sdkwork-im-sdk-kotlin/bin/sdk-gen.sh
./sdks/sdkwork-im-sdk/sdkwork-im-sdk-kotlin/bin/sdk-verify.sh
```

## When To Choose Kotlin

- Choose Kotlin when you need a verified JVM-side transport artifact generated from the Sdkwork IM
  app schema.
- Choose Kotlin when you can work directly against generated request or response models and
  transport-level route groups.
- Choose TypeScript when you need the richest checked-in semantic SDK today.

## What To Read Next

- Read [Language Support](/sdk/language-support) when you need the full cross-language maturity
  matrix and current Kotlin delivery status.
- Read [Generator Boundary](/sdk/generator-boundary) when you need the exact ownership split
  between `generated/server-openapi` and `composed`.
- Read [TypeScript SDK](/sdk/typescript-sdk) when you need the current semantic baseline rather
  than the Kotlin transport-first workspace.
- Read [Portal Access](/api-reference/app/portal-access), [Conversations](/api-reference/im/conversations),
  and [Messages](/api-reference/im/messages) when you need the exact HTTP contract behind the
  generated Kotlin transport.
- Read [Realtime Presence](/api-reference/im/session-and-realtime) and [Calls](/api-reference/im/calls)
  when you need the route-level transport contract for live coordination and call workflows.
