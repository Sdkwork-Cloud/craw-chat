> Migrated from `docs/sites/sdk/swift-sdk.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Swift SDK

The Swift workspace is a Tier B member of the `im-sdk` business SDK family and is currently
standardized around generated transport delivery.

## Current Delivery Reality

This page describes the current checked-in repo contract validated on 2026-04-16. It does not by
itself claim Swift Package Index publication or a shipped semantic Swift client.

Today the verified Swift entrypoint is the generated package under `generated/server-openapi`.
Swift-specific workspace normalization keeps that package named `ImSdkGenerated`, while the
semantic Swift package remains reserved under `composed`.

## Package Contract

| Concern | Value |
| --- | --- |
| Maturity tier | Tier B |
| Generated transport package | `ImSdkGenerated` |
| Raw generated import | `import ImSdkGenerated` |
| Raw generated client | `ImTransportClient` |
| Reserved semantic package | `ImSdk` |
| Target business client | `ImSdkClient` |
| Generator-owned boundary | `generated/server-openapi` |
| Manual semantic boundary | `composed` |

## What Ships Today

- live-schema generation from the Sdkwork IM OpenAPI 3.x export
- Swift workspace normalization that preserves the generated package name `ImSdkGenerated`
- verified generated package boundary and assembly metadata
- a raw generated transport client named `ImTransportClient`

For exact generated installation and raw API examples, use
`generated/server-openapi/README.md` as the transport reference.

## Raw Generated Client

If you are integrating Swift today, start from `ImSdkGenerated` and `ImTransportClient`.

- generated package: `ImSdkGenerated`
- raw generated client: `ImTransportClient`
- reserved semantic package: `ImSdk`

That is the checked-in Swift surface the workspace verifies today.

## API Reference Map

Use `generated/server-openapi/README.md` together with `ImTransportClient` when you need the
exact Swift route-group names and DTO entrypoints. Use the map below to jump from transport
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

This keeps the Swift page precise: the repo-standard delivery today is transport-first, so the API
reference plus `generated/server-openapi/README.md` remains the exact route authority until a
future semantic `ImSdkClient` is implemented under `composed`.

## What Is Not Shipped Yet

- no checked-in semantic Swift package that already exposes `ImSdkClient`
- no handwritten message-first layer above generated route groups
- no delivered websocket live runtime abstraction comparable to the TypeScript SDK

Treat this page as a repo contract for current Swift delivery, not as a claim of semantic parity.

## When To Use `composed`

Use `composed` only when you are intentionally implementing the future semantic Swift layer:

- `ImSdkClient`
- business-friendly wrappers above generated transport
- message-first helpers
- live runtime orchestration

Do not hand-edit generated Swift files under `generated/server-openapi`.

## Generate And Verify

Root workspace:

```powershell
powershell -ExecutionPolicy Bypass -File .\sdks\sdkwork-im-sdk\bin\generate-sdk.ps1 -Languages swift
node .\sdks\sdkwork-im-sdk\bin\verify-sdk.mjs --language swift
```

Swift workspace wrappers:

```powershell
.\sdks\sdkwork-im-sdk\sdkwork-im-sdk-swift\bin\sdk-gen.ps1
.\sdks\sdkwork-im-sdk\sdkwork-im-sdk-swift\bin\sdk-verify.ps1
```

```bash
./sdks/sdkwork-im-sdk/sdkwork-im-sdk-swift/bin/sdk-gen.sh
./sdks/sdkwork-im-sdk/sdkwork-im-sdk-swift/bin/sdk-verify.sh
```

## When To Choose Swift

- Choose Swift when you need a verified Apple-platform transport package generated from the Sdkwork
  IM schema.
- Choose Swift when you can work directly against generated request or response models and transport
  route groups.
- Choose TypeScript or Flutter when you need a checked-in semantic client rather than a
  transport-standardized package.

## What To Read Next

- Read [Language Support](/sdk/language-support) when you need the full cross-language maturity
  matrix and current Swift delivery status.
- Read [Generator Boundary](/sdk/generator-boundary) when you need the exact ownership split
  between `generated/server-openapi` and `composed`.
- Read [TypeScript SDK](/sdk/typescript-sdk) when you need the current semantic baseline rather
  than the Swift transport-first workspace.
- Read [Portal Access](/api-reference/app/portal-access), [Conversations](/api-reference/im/conversations),
  and [Messages](/api-reference/im/messages) when you need the exact HTTP contract behind the
  generated Swift transport.
- Read [Realtime Presence](/api-reference/im/session-and-realtime) and [Calls](/api-reference/im/calls)
  when you need the route-level transport contract for live coordination and call workflows.

