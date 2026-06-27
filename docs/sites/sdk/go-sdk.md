# Go SDK

The Go workspace is a Tier B member of the `im-sdk` business SDK family and currently ships
as a transport-standardized module.

## Current Delivery Reality

This page describes the current checked-in repo contract validated on 2026-04-16. It does not by
itself claim module proxy publication or a shipped semantic Go client.

Today the verified Go entrypoint is the generated module under `generated/server-openapi`. The
semantic Go module remains reserved under `composed`.

## Package Contract

| Concern | Value |
| --- | --- |
| Maturity tier | Tier B |
| Generated transport module | `github.com/sdkwork/im-sdk-generated` |
| Raw generated client | `ImTransportClient` |
| Reserved semantic module | `github.com/sdkwork/im-sdk` |
| Target business client | `ImSdkClient` |
| Generator-owned boundary | `generated/server-openapi` |
| Manual semantic boundary | `composed` |

## What Ships Today

- live-schema generation from the Sdkwork IM OpenAPI 3.x export
- verified generated module naming and assembly metadata
- a stable split between `generated/server-openapi` and `composed`
- a raw generated transport client named `ImTransportClient`

For exact import examples and raw transport usage, use `generated/server-openapi/README.md` as the
transport reference.

## Raw Generated Client

If you are integrating Go today, start from the generated module and `ImTransportClient`.

- generated module: `github.com/sdkwork/im-sdk-generated`
- raw generated client: `ImTransportClient`
- reserved semantic module: `github.com/sdkwork/im-sdk`

That is the checked-in Go surface the workspace verifies today.

## API Reference Map

Use `generated/server-openapi/README.md` together with `ImTransportClient` when you need the
exact Go route-group names and DTO entrypoints. Use the map below to jump from transport concern
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

This keeps the Go page precise: the repo-standard delivery today is transport-first, so the API
reference plus `generated/server-openapi/README.md` remains the exact route authority until a
future semantic `ImSdkClient` is implemented under `composed`.

## What Is Not Shipped Yet

- no checked-in semantic Go module that already exposes `ImSdkClient`
- no handwritten message-first business layer above generated route groups
- no delivered websocket live runtime abstraction above the generated transport module

Treat this page as a repo contract for current Go delivery, not as a claim of semantic parity.

## When To Use `composed`

Use `composed` only when you are intentionally implementing the future semantic Go layer:

- `ImSdkClient`
- business wrappers above raw route groups
- message-first helpers
- live runtime orchestration above transport-level coordination

Do not hand-edit generated Go files under `generated/server-openapi`.

## Generate And Verify

Root workspace:

```powershell
powershell -ExecutionPolicy Bypass -File .\sdks\sdkwork-im-sdk\bin\generate-sdk.ps1 -Languages go
node .\sdks\sdkwork-im-sdk\bin\verify-sdk.mjs --language go
```

Go workspace wrappers:

```powershell
.\sdks\sdkwork-im-sdk\sdkwork-im-sdk-go\bin\sdk-gen.ps1
.\sdks\sdkwork-im-sdk\sdkwork-im-sdk-go\bin\sdk-verify.ps1
```

```bash
./sdks/sdkwork-im-sdk/sdkwork-im-sdk-go/bin/sdk-gen.sh
./sdks/sdkwork-im-sdk/sdkwork-im-sdk-go/bin/sdk-verify.sh
```

## When To Choose Go

- Choose Go when you need a verified transport module generated from the Sdkwork IM schema for
  services or automation.
- Choose Go when you can work directly against generated request or response models and
  transport-level route groups.
- Choose TypeScript or Flutter when you need a checked-in semantic SDK rather than a
  transport-standardized boundary.

## What To Read Next

- Read [Language Support](/sdk/language-support) when you need the full cross-language maturity
  matrix and current Go delivery status.
- Read [Generator Boundary](/sdk/generator-boundary) when you need the exact ownership split
  between `generated/server-openapi` and `composed`.
- Read [TypeScript SDK](/sdk/typescript-sdk) when you need the current semantic baseline rather
  than the Go transport-first workspace.
- Read [Portal Access](/api-reference/app/portal-access), [Conversations](/api-reference/im/conversations),
  and [Messages](/api-reference/im/messages) when you need the exact HTTP contract behind the
  generated Go transport.
- Read [Realtime Presence](/api-reference/im/session-and-realtime) and [Calls](/api-reference/im/calls)
  when you need the route-level transport contract for live coordination and call workflows.
