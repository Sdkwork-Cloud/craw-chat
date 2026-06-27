# Rust SDK

The Rust workspace is a Tier A app-SDK lane and now ships both a generated transport crate and a
checked-in composed crate for application code.

## Current Delivery Reality

This page describes the current checked-in repo contract validated on 2026-04-19. It is a
statement of the repo contract, not a claim that crates.io publication or TypeScript-level parity
already exists.

Today the normal Rust consumption boundary is the composed crate under `composed`, exposed as
`im-sdk` with the public client `ImSdkClient`. The generated transport crate under
`generated/server-openapi` remains the raw route-level fallback.

## Package Contract

| Concern | Value |
| --- | --- |
| Maturity tier | Tier A |
| Generated transport crate package | `sdkwork-im-sdk-generated` |
| Generated Rust import crate | `sdkwork_im_sdk_generated` |
| Raw generated client | `ImTransportClient` |
| Composed semantic crate package | `im-sdk` |
| Composed semantic crate import | `im_sdk` |
| Primary business client | `ImSdkClient` |
| Generator-owned boundary | `generated/server-openapi` |
| Manual semantic boundary | `composed` |

## What Ships Today

- live-schema generation from the same Sdkwork IM OpenAPI 3.x export as every official language
- verified generated-versus-semantic ownership split between `generated/server-openapi` and `composed`
- a checked-in `ImSdkClient` with route-aligned modules for session, presence, realtime, inbox,
  conversations, messages, media, streams, and calls
- builder helpers for text messages, text stream frames, and JSON RTC signals
- re-exported generated transport types so application code can still drop down to `ImTransportClient`
  when it needs transport-level token hooks, portal, or DTO-level access

## Normal Client Entry

Use `im-sdk` and `ImSdkClient` for normal application code:

```rust
use im_sdk::ImSdkClient;

let client = ImSdkClient::new_with_base_url("http://127.0.0.1:18079")?;
client.set_auth_token(token);
```

## Boundary Rules

- Start from `ImSdkClient` when you want route-aligned Rust helpers above the generated transport.
- Drop to `ImTransportClient` or the generated DTO exports re-exported by `im_sdk` when you need
  transport-level token hooks, portal, or exact route-group access.
- Use `generated/server-openapi/README.md` when you need the raw transport installation contract
  and exact generated route-group examples.
- Do not hand-edit generated Rust files under `generated/server-openapi`.

## API Reference Map

Use the map below to jump from the Rust surface you are using to the matching HTTP reference:

| Concern | Preferred Rust surface today | Exact API reference |
| --- | --- | --- |
| SDKWork appbase credential pass-through and portal reads | `ImTransportClient` fallback via generated exports or `client.transport_client()` | [Portal Access](/api-reference/app/portal-access) |
| Realtime presence and coordination | `ImSdkClient::presence()`, `realtime()` | [Realtime And Presence](/api-reference/im/session-and-realtime) |
| Inbox and conversation lifecycle | `ImSdkClient::inbox()`, `conversations()` | [Conversations](/api-reference/im/conversations) |
| Membership and read cursors | `ImSdkClient::conversations()` plus generated DTOs | [Membership and Read State](/api-reference/im/membership-and-read-state) |
| Message posting and mutation helpers | `ImSdkClient::conversations()`, `messages()`, `build_text_message(...)` | [Messages](/api-reference/im/messages) |
| Upload and attachment lifecycle | `ImSdkClient::media()` | [Media](/api-reference/im/media) |
| Stream ingestion and checkpoints | `ImSdkClient::streams()`, `build_text_stream_frame(...)` | [Streams](/api-reference/im/streams) |
| IM call lifecycle and signaling helpers | `ImSdkClient::calls()`, `calls.sendSignal(...)` | [Calls](/api-reference/im/calls) |

When you need the exact generated route groups or transport-level DTO usage examples, pair this
page with `generated/server-openapi/README.md`.

## What Is Not Shipped Yet

Rust now ships a composed client, but it still trails the TypeScript baseline in several areas:

- no delivered websocket live runtime abstraction above HTTP coordination
- no TypeScript-style `sdk.connect(...)`, `createXxxMessage(...)`, `send(...)`, or `decodeMessage()` surface
- auth and portal remain generated-client-first rather than dedicated semantic modules on `ImSdkClient`

## When To Use `generated/server-openapi`

Use the generated crate directly when you need:

- transport-level token hooks or portal route groups
- exact generated DTOs and low-level route-group calls
- transport-level debugging against the generated boundary

Use `composed` and `ImSdkClient` for normal application integration.

## Generate And Verify

Root workspace:

```powershell
powershell -ExecutionPolicy Bypass -File .\sdks\sdkwork-im-sdk\bin\generate-sdk.ps1 -Languages rust
node .\sdks\sdkwork-im-sdk\bin\verify-sdk.mjs --language rust
```

Rust workspace wrappers:

```powershell
.\sdks\sdkwork-im-sdk\sdkwork-im-sdk-rust\bin\sdk-gen.ps1
.\sdks\sdkwork-im-sdk\sdkwork-im-sdk-rust\bin\sdk-verify.ps1
```

```bash
./sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/bin/sdk-gen.sh
./sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/bin/sdk-verify.sh
```

## When To Choose Rust

- Choose Rust when you want a checked-in IM consumer Rust client with route-aligned helpers above
  the generated transport.
- Choose Rust when service-side or systems-side integration benefits from a strongly typed generated
  fallback under the same crate family.
- Choose TypeScript instead when you need the richest checked-in live runtime and message-first app
  surface today.

## What To Read Next

- Read [Language Support](/sdk/language-support) when you need the full cross-language maturity
  matrix and Tier A versus Tier B boundaries.
- Read [Generator Boundary](/sdk/generator-boundary) when you need the exact ownership split
  between `generated/server-openapi` and `composed`.
- Read [TypeScript SDK](/sdk/typescript-sdk) when you need the current semantic baseline with the
  richest checked-in live runtime.
- Read [Portal Access](/api-reference/app/portal-access), [Conversations](/api-reference/im/conversations),
  and [Messages](/api-reference/im/messages) when you need the exact HTTP contract behind the Rust
  semantic and generated layers.
- Read [Realtime Presence](/api-reference/im/session-and-realtime) and [Calls](/api-reference/im/calls)
  when you need the route-level contract for live coordination and call workflows.
