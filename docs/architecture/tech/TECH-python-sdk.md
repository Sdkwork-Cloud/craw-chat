> Migrated from `docs/sites/sdk/python-sdk.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Python SDK

The Python workspace is a Tier B member of the `im-sdk` business SDK family and currently
ships as a transport-standardized package.

## Current Delivery Reality

This page describes the current checked-in repo contract validated on 2026-04-16. It does not by
itself claim PyPI publication or a shipped semantic Python client.

Today the verified Python entrypoint is the generated package under `generated/server-openapi`. The
semantic Python package remains reserved under `composed`.

## Package Contract

| Concern | Value |
| --- | --- |
| Maturity tier | Tier B |
| Generated transport package | `sdkwork-im-sdk-generated` |
| Generated Python import package | `sdkwork_im_sdk_generated` |
| Raw generated client | `ImTransportClient` |
| Reserved semantic package | `sdkwork-im-sdk` |
| Target business client | `ImSdkClient` |
| Generator-owned boundary | `generated/server-openapi` |
| Manual semantic boundary | `composed` |

## What Ships Today

- live-schema generation from the Sdkwork IM OpenAPI 3.x export
- verified generated package naming and assembly metadata
- a stable split between `generated/server-openapi` and `composed`
- a raw generated transport client named `ImTransportClient`

For exact installation and raw transport usage, use `generated/server-openapi/README.md` as the
transport reference.

## Raw Generated Client

If you are integrating Python today, start from the generated package and `ImTransportClient`.

- generated package: `sdkwork-im-sdk-generated`
- import package: `sdkwork_im_sdk_generated`
- raw generated client: `ImTransportClient`
- reserved semantic package: `sdkwork-im-sdk`

That is the checked-in Python surface the workspace verifies today.

## API Reference Map

Use `generated/server-openapi/README.md` together with `ImTransportClient` when you need the
exact Python route-group names and DTO entrypoints. Use the map below to jump from transport
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

This keeps the Python page precise: the repo-standard delivery today is transport-first, so the
API reference plus `generated/server-openapi/README.md` remains the exact route authority until a
future semantic `ImSdkClient` is implemented under `composed`.

## What Is Not Shipped Yet

- no checked-in semantic Python package that already exposes `ImSdkClient`
- no handwritten message-first business layer above generated route groups
- no delivered websocket live runtime abstraction above the generated transport package

Treat this page as a repo contract for current Python delivery, not as a claim of semantic parity.

## When To Use `composed`

Use `composed` only when you are intentionally implementing the future semantic Python layer:

- `ImSdkClient`
- business wrappers above raw route groups
- higher-level message helpers
- live runtime orchestration above transport-level coordination

Do not hand-edit generated Python files under `generated/server-openapi`.

## Generate And Verify

Root workspace:

```powershell
powershell -ExecutionPolicy Bypass -File .\sdks\sdkwork-im-sdk\bin\generate-sdk.ps1 -Languages python
node .\sdks\sdkwork-im-sdk\bin\verify-sdk.mjs --language python
```

Python workspace wrappers:

```powershell
.\sdks\sdkwork-im-sdk\sdkwork-im-sdk-python\bin\sdk-gen.ps1
.\sdks\sdkwork-im-sdk\sdkwork-im-sdk-python\bin\sdk-verify.ps1
```

```bash
./sdks/sdkwork-im-sdk/sdkwork-im-sdk-python/bin/sdk-gen.sh
./sdks/sdkwork-im-sdk/sdkwork-im-sdk-python/bin/sdk-verify.sh
```

## When To Choose Python

- Choose Python when you need a verified transport package for automation or service integration
  generated directly from the Sdkwork IM schema.
- Choose Python when you can work directly against generated request or response models and
  route-group methods.
- Choose TypeScript when you need the richest checked-in semantic SDK today.

## What To Read Next

- Read [Language Support](/sdk/language-support) when you need the full cross-language maturity
  matrix and current Python delivery status.
- Read [Generator Boundary](/sdk/generator-boundary) when you need the exact ownership split
  between `generated/server-openapi` and `composed`.
- Read [TypeScript SDK](/sdk/typescript-sdk) when you need the current semantic baseline rather
  than the Python transport-first workspace.
- Read [Portal Access](/api-reference/app/portal-access), [Conversations](/api-reference/im/conversations),
  and [Messages](/api-reference/im/messages) when you need the exact HTTP contract behind the
  generated Python transport.
- Read [Realtime Presence](/api-reference/im/session-and-realtime) and [Calls](/api-reference/im/calls)
  when you need the route-level transport contract for live coordination and call workflows.

