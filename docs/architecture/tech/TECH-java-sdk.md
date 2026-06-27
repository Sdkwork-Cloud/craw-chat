> Migrated from `docs/sites/sdk/java-sdk.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Java SDK

The Java workspace is a Tier B member of the `im-sdk` business SDK family and is currently
standardized around generated transport delivery first.

## Current Delivery Reality

This page describes the current checked-in repo contract validated on 2026-04-16. It does not by
itself claim Maven Central publication or a shipped semantic Java client.

Today the real Java consumption boundary is the generated artifact in `generated/server-openapi`.
The semantic Java artifact remains reserved under `composed`.

## Package Contract

| Concern | Value |
| --- | --- |
| Maturity tier | Tier B |
| Generated transport artifact | `com.sdkwork:im-sdk-generated` |
| Raw generated client | `ImTransportClient` |
| Generated Java package root | `com.sdkwork.im.generated` |
| Reserved semantic artifact | `com.sdkwork:im-sdk` |
| Target business client | `ImSdkClient` |
| Generator-owned boundary | `generated/server-openapi` |
| Manual semantic boundary | `composed` |

## What Ships Today

- live-schema generation from the Sdkwork IM OpenAPI 3.x export
- verified generated artifact naming and assembly metadata
- a stable split between `generated/server-openapi` and `composed`
- a raw generated transport client named `ImTransportClient`

For installation snippets, raw route-group usage, and DTO examples, use
`generated/server-openapi/README.md` as the exact transport reference.

## Raw Generated Client

If you are integrating Java today, start from the generated transport artifact and
`com.sdkwork.im.generated.ImTransportClient`.

- generated artifact: `com.sdkwork:im-sdk-generated`
- raw generated client: `ImTransportClient`
- semantic artifact reserved for later: `com.sdkwork:im-sdk`

This is the only checked-in Java entrypoint that is verified as shipped in the repo today.

## API Reference Map

Use `generated/server-openapi/README.md` together with `ImTransportClient` when you need the
exact Java route-group names and DTO entrypoints. Use the map below to jump from transport concern
to the matching HTTP reference:

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

This keeps the Java page precise: the repo-standard delivery today is transport-first, so the API
reference plus `generated/server-openapi/README.md` remains the exact route authority until a
future semantic `ImSdkClient` is implemented under `composed`.

## What Is Not Shipped Yet

- no checked-in semantic Java artifact that already exposes `ImSdkClient`
- no handwritten message-first business layer
- no delivered websocket live runtime layer above the generated transport artifact

Treat the page as a repo contract for current Java delivery, not as evidence that Java already has
TypeScript-level semantics.

## When To Use `composed`

Use `composed` only when you are intentionally building the future semantic Java surface:

- `ImSdkClient`
- business-oriented wrappers above raw route groups
- higher-level message helpers
- live runtime abstractions

Do not hand-edit generator output under `generated/server-openapi`.

## Generate And Verify

Root workspace:

```powershell
powershell -ExecutionPolicy Bypass -File .\sdks\sdkwork-im-sdk\bin\generate-sdk.ps1 -Languages java
node .\sdks\sdkwork-im-sdk\bin\verify-sdk.mjs --language java
```

Java workspace wrappers:

```powershell
.\sdks\sdkwork-im-sdk\sdkwork-im-sdk-java\bin\sdk-gen.ps1
.\sdks\sdkwork-im-sdk\sdkwork-im-sdk-java\bin\sdk-verify.ps1
```

```bash
./sdks/sdkwork-im-sdk/sdkwork-im-sdk-java/bin/sdk-gen.sh
./sdks/sdkwork-im-sdk/sdkwork-im-sdk-java/bin/sdk-verify.sh
```

## When To Choose Java

- Choose Java when you need a verified JVM transport artifact generated from the Sdkwork IM schema.
- Choose Java when your integration can work directly against generated request or response models
  and route-group methods.
- Choose TypeScript or Flutter when you need a checked-in semantic SDK rather than a
  transport-first workspace contract.

## What To Read Next

- Read [Language Support](/sdk/language-support) when you need the full cross-language maturity
  matrix and current Java delivery status.
- Read [Generator Boundary](/sdk/generator-boundary) when you need the exact ownership split
  between `generated/server-openapi` and `composed`.
- Read [TypeScript SDK](/sdk/typescript-sdk) when you need the current semantic baseline rather
  than the Java transport-first workspace.
- Read [Portal Access](/api-reference/app/portal-access), [Conversations](/api-reference/im/conversations),
  and [Messages](/api-reference/im/messages) when you need the exact HTTP contract behind the
  generated Java transport.
- Read [Realtime Presence](/api-reference/im/session-and-realtime) and [Calls](/api-reference/im/calls)
  when you need the route-level transport contract for live coordination and call workflows.

