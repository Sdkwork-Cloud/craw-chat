# Sdkwork IM Next-Generation SDK Standard Design

## Goal

Define the long-term public SDK standard for the new Sdkwork IM application, with no backward-compatibility constraints and no transport-first public API compromises.

The target is an industry-grade Chat IM SDK system with these properties:

- one official consumer package per language
- one primary client class per package family
- OpenAPI 3.x as the generated HTTP contract source
- handwritten domain layers for product-grade messaging, realtime, RTC, and session ergonomics
- consistent message, signal, and event models across languages
- documentation that teaches workflows first, not route groups first

The first landing implementation is TypeScript for browser and Node.js, but the standards in this document are intended to govern all future language SDKs.

## Non-Goals

- preserve compatibility with the current public TypeScript SDK surface
- expose generated REST route-group clients as the primary integration API
- treat WebSocket receive handling as a generated OpenAPI concern
- optimize public API naming around current backend module names when those names are transport-oriented

## Canonical Inputs

- TypeScript SDK workspace: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript`
- SDK root workspace: `sdks/sdkwork-im-sdk`
- Current live schema export endpoint: `/im/v3/openapi.json`
- Checked-in authority schema: `sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml`
- Current live schema refresh script: `sdks/sdkwork-im-sdk/bin/refresh-live-openapi-source.mjs`
- Current TypeScript client implementation: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/sdk.ts`
- Current TypeScript public types: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/types.ts`
- Current SDK docs: `docs/sites/sdk/*.md`

## Problem Statement

The current SDK direction is materially better than a raw generated client, but it still carries several signs of an implementation-first system instead of a polished public SDK standard.

Current gaps that should not survive into the new application standard:

- client bootstrap still reflects transport setup details rather than a clean product-facing lifecycle
- current naming still leaks backend structure in places such as `portal`, `realtime`, and `receiver.pull()`
- receive callbacks still feel too event-envelope-centric for the main public API
- root-level message builders make the primary client surface busier than necessary
- generated versus handwritten ownership exists conceptually but is not yet elevated into a hard cross-language SDK standard
- documentation still has to explain too much backend context before teaching the best integration path

The new application should not continue refining those compromises. It should define a cleaner standard and pull the implementation toward it.

## Chosen Direction

Adopt a domain-first, single-package SDK standard:

1. OpenAPI 3.x generated code owns only the HTTP contract layer.
2. Handwritten business modules own the public SDK experience.
3. The default public entrypoint is a single client class derived from the package business name.
4. Construction is synchronous and side-effect free.
5. Runtime side effects start only when explicit actions such as login, connect, send, upload, or RTC control are invoked.
6. Messaging is message-first.
7. Receiving is context-first.
8. Live push and durable catch-up are separate concepts in the public API.
9. RTC is a first-class domain, not an afterthought on top of raw event plumbing.
10. Documentation and examples must use only the public semantic layer unless a page is explicitly about the generated layer.

This gives the SDK a stable architecture that can scale across languages and product surfaces without reducing the developer experience to generated route wrappers.

## Package Standard

### Naming

Package naming is derived from the SDK workspace root name.

Example:

- SDK workspace root: `sdkwork-im-sdk`
- official npm package: `@sdkwork/im-sdk`
- primary client class: `ImSdkClient`

The same derivation rule should apply in other ecosystems:

- Flutter package: `im_sdk`
- Java artifact: `sdkwork-im-sdk`
- Rust crate: `im_sdk`

The public client class or main entry type should always use the business name, not the generated transport name.

### Single-Package Rule

Each language should ship one official consumer package for normal application integration.

Inside that package:

- generated code must live under a dedicated generated namespace or directory
- handwritten semantic code must live outside that generated namespace
- advanced consumers may access generated exports from the same package
- normal consumers should never need to install a second package just to get the semantic SDK

For TypeScript, the standard is:

- package: `@sdkwork/im-sdk`
- generated layer location: `src/generated/**`
- public semantic layer location: `src/**` outside `src/generated/**`

## Architecture Standard

The SDK must be split into two hard layers.

### Layer 1: Generated Contract Layer

This layer is owned by `sdkwork-sdk-generator` and generated from OpenAPI 3.x.

Responsibilities:

- generated DTOs
- generated request and response types
- generated HTTP route-group clients
- generated transport config adapters
- generated OpenAPI-aligned error and auth primitives where applicable

Non-responsibilities:

- message builder ergonomics
- WebSocket connection lifecycle
- durable event replay orchestration
- RTC signal domain helpers
- application-facing module naming

### Layer 2: Semantic Domain Layer

This layer is handwritten and package-owned.

Responsibilities:

- public client class
- domain modules
- session and connection lifecycle orchestration
- message construction and decoding
- live receive runtime
- RTC signal helpers and receive routing
- rich error normalization
- environment-specific runtime adapters
- workflow-first documentation examples

The public SDK experience is defined entirely by this semantic layer.

## Directory Standard

### TypeScript

The TypeScript package should converge on the following structure:

```text
sdkwork-im-sdk-typescript/
  package.json
  src/
    index.ts
    generated/
      api/
      types/
      runtime/
    core/
      client.ts
      config.ts
      errors.ts
      transport.ts
    auth/
    app/
    conversations/
    messages/
    media/
    live/
    sync/
    rtc/
    internal/
  test/
```

Rules:

- `src/generated/**` is generator-owned
- every other `src/**` directory is manual-owned
- handwritten code must not be mixed back into `src/generated/**`
- generated code must never import handwritten business modules
- handwritten modules may depend on generated exports

### Cross-Language Invariant

For every language, there must be an equivalent split between:

- generated transport contract
- public semantic SDK

Directory names may vary by ecosystem, but the ownership rule must not vary.

## Public Client Standard

### Construction Model

The new standard should not make client construction asynchronous.

Preferred shape:

```ts
import { ImSdkClient } from '@sdkwork/im-sdk';

const sdk = new ImSdkClient({
  baseUrl: import.meta.env.VITE_SDKWORK_IM_BASE_URL,
  authToken: window.localStorage.getItem('sdkwork-im-token')!,
});
```

Why:

- construction should be pure configuration assembly
- it should not hide network I/O
- it reads more naturally in both browser and Node.js code
- it makes testing and dependency injection easier

Network activity begins only with explicit methods such as:

- `sdk.auth.useToken(...)`
- `sdk.connect(...)`
- `sdk.messages.send(...)`
- `sdk.media.upload(...)`
- `sdk.calls.start(...)`

### Configuration Model

The top-level config should optimize for the common case while still allowing split origins and advanced adapters.

Required standard fields:

- `baseUrl`
- `apiBaseUrl`
- `websocketBaseUrl`
- `authToken`
- `tokenProvider`
- `fetch`
- `webSocketFactory`
- `logger`
- `retry`

Rules:

- `baseUrl` is the simple default for both HTTP and WebSocket derivation
- `apiBaseUrl` overrides HTTP origin when different from `baseUrl`
- `websocketBaseUrl` overrides realtime origin when different from `baseUrl`
- `authToken` is the simplest static token path
- `tokenProvider` is the professional path for rotating credentials
- browser consumers should work without providing `fetch` or `webSocketFactory`
- Node.js consumers may provide adapters where needed

The public config should not require a nested `backendConfig` object. That shape is transport-oriented and adds friction without adding clarity.

## Public Module Standard

The root client should stay intentionally small and expose business domains, not transport buckets.

Required public modules:

- `sdk.auth`
- `sdk.app`
- `sdk.conversations`
- `sdk.messages`
- `sdk.media`
- `sdk.live`
- `sdk.sync`
- `sdk.calls`
- `sdk.generated`

Optional root convenience methods:

- `sdk.connect(...)`
- `sdk.disconnectAll()`

Guiding rule:

- if a capability is part of normal product integration, it belongs in a semantic domain module
- if a capability exists mainly for low-level debugging or special-case transport access, it belongs under `sdk.generated` or clearly advanced APIs

## Authentication Standard

Authentication and IAM context are issued by `sdkwork-appbase`, not by Sdkwork IM SDK packages.
The public Sdkwork IM SDK auth surface is token consumption only.

Required public shape:

- `sdk.auth.useToken(token)`
- `sdk.auth.clearToken()`
- `authToken` constructor config

Current-user, tenant, organization, login, refresh, and dual-token lifecycle flows stay in appbase.
Sdkwork IM receives an appbase bearer token and applies it to IM HTTP and realtime requests.

## Connection Standard

The main realtime lifecycle entrypoint for a new application should be `connect()`.

Preferred shape:

```ts
const sdk = new ImSdkClient({
  baseUrl: 'http://127.0.0.1:18090',
  authToken: token,
});

const live = await sdk.connect({
  clientRouteId: 'web-chrome-01',
  subscriptions: {
    conversations: ['conversation-support-001'],
    rtcSessions: ['rtc-support-001'],
  },
});
```

`connect()` should orchestrate:

1. token availability validation
2. client route registration when needed
3. session resume
4. optional durable catch-up before going live
5. WebSocket connection establishment
6. subscription synchronization
7. reconnect handling
8. state reporting

This is the correct abstraction boundary for a professional IM SDK. Application developers should not have to manually reason about `resume`, `listRealtimeEvents`, `ackRealtimeEvents`, and WebSocket wiring just to receive messages correctly.

## Live Receive Standard

### Main Principle

The mainstream receive API must be semantic-first and context-first.

The preferred public design is:

```ts
live.onMessage((ctx) => {
  console.log(ctx.message.type, ctx.message.summary, ctx.sequence);
});
```

Do not make `(message, event)` the long-term standard callback shape.

Reasons:

- one context object is easier to extend safely
- the public API should not duplicate `message` and `event.message`
- the name `event` is too transport-leaning for the main business callback
- one-argument callbacks scale better across languages and event systems

### Required Live Handlers

Main semantic handlers:

- `live.onMessage((ctx) => {})`
- `live.onData((ctx) => {})`
- `live.onSignal((ctx) => {})`
- `live.onConversationMessage(conversationId, (ctx) => {})`

Low-level handlers:

- `live.onRawEvent((ctx) => {})`
- `live.onStateChange((state) => {})`
- `live.onError((error) => {})`

### Receive Context Standard

Required receive context fields:

- `message` or `data` or `signal`
- `conversationId`
- `sender`
- `sequence`
- `receivedAt`
- `source`
- `rawEvent`
- `ack()` where manual ack mode applies

Recommended `source` values:

- `live`
- `catch_up`
- `replay`

This makes receive handling readable in product code while still preserving transport insight when advanced consumers need it.

## Durable Catch-Up Standard

The public SDK must separate live push from durable catch-up.

The current term `receiver.pull()` is not suitable for the long-term standard because it is too easy to misread as a live receive mechanism.

The new standard should express intent directly:

- `sdk.sync.catchUp(...)`
- `sdk.sync.ack(...)`
- `sdk.sync.replayConversation(...)`

Rules:

- `sdk.connect(...)` should hide catch-up for the mainstream path whenever possible
- `sdk.sync` exists for advanced durability, repair, replay, and operational recovery paths
- docs must explicitly explain that durable catch-up is HTTP replay, not WebSocket receive

## Messaging Standard

### Core Rule

Message construction must be explicit and message sending must accept message objects.

Preferred shape:

```ts
const text = sdk.messages.createText({
  conversationId: 'conversation-1',
  text: 'hello world',
});

await sdk.messages.send(text);
```

This is preferable to teaching application developers to construct transport payloads directly.

### Message Object Rule

Message objects should be immutable value objects that represent business intent, not raw HTTP payload fragments.

The semantic layer is responsible for:

- building message objects
- encoding them into backend request bodies
- decoding inbound backend messages into the same semantic family when possible

### Standard Message Families

The SDK must natively support the common IM families plus AI-era additions.

Required message families:

- `text`
- `image`
- `audio`
- `video`
- `file`
- `location`
- `link`
- `card`
- `music`
- `sticker`
- `custom`
- `ai_text`
- `ai_image_generation`
- `ai_video_generation`
- `agent_state`
- `agent_handoff`
- `tool_result`
- `workflow_event`

Each family should have:

- a typed builder
- a decoded inbound representation
- a stable summary strategy
- a documented fallback text strategy where relevant

### Custom Message Standard

Custom messages are required and must not be treated as second-class.

Preferred shape:

```ts
const custom = sdk.messages.createCustom({
  conversationId: 'conversation-1',
  customType: 'order-status',
  payload: {
    orderId: 'order-1001',
    status: 'packed',
  },
  summary: 'Order #1001 packed',
});
```

Standard rule:

- custom types use stable, namespaced identifiers
- the SDK should preserve a machine-readable custom type and payload
- the SDK should also require or derive a human-readable summary

If URN-style identifiers are used internally, the semantic layer should still expose a simple developer-facing API.

## Media Standard

Media upload is a first-class workflow and should not depend on application developers learning storage contract details.

Required shape:

- `sdk.media.upload(...)`
- `sdk.messages.createImage(...)`
- `sdk.messages.createVideo(...)`
- `sdk.messages.createFile(...)`
- `sdk.messages.uploadAndSend(...)`

The SDK should preserve the existing backend rule that outbound media messages need resolved media asset identity, but the public API should make the correct path obvious and easy.

## RTC Standard

RTC must be treated as a domain, not as miscellaneous REST plus raw signal events.

Required module responsibilities:

- `sdk.calls.start(...)`
- `sdk.calls.invite(...)`
- `sdk.calls.accept(...)`
- `sdk.calls.reject(...)`
- `sdk.calls.end(...)`
- `sdk.calls.sendSignal(...)`
- participant credential issuance helpers
- recording artifact helpers where the backend supports them

Incoming RTC signaling should land in the same live runtime through:

- `live.onSignal((ctx) => {})`

RTC signaling should use one normalized signal envelope across send and receive paths.

## Error Standard

The public SDK should normalize low-level generated transport errors into product-readable domain errors.

Required error categories:

- auth error
- validation error
- permission error
- transport error
- timeout error
- retryable realtime connection error
- message encode or decode error
- RTC signal error

The generated layer may expose raw HTTP failures, but the semantic layer should map the common cases into typed SDK errors with stable codes.

## Generated Export Standard

The official package should still expose the generated layer for advanced consumers, but it must be clearly secondary.

Preferred export shape:

```ts
import { ImSdkClient, generated } from '@sdkwork/im-sdk';
```

`generated` should expose:

- generated API classes
- generated DTO types
- generated config and transport types

The package documentation must mark this as advanced usage, not the default onboarding path.

## Generation Pipeline Standard

The generator pipeline must always use the newest service contract, not a stale local assumption.

Required pipeline:

1. start or verify the target service instance
2. fetch the live OpenAPI 3.x schema from `/im/v3/openapi.json`
3. validate that the fetched document is OpenAPI 3.x
4. refresh the checked-in authority snapshot
5. derive generator-specific normalized inputs where needed
6. generate the language-specific contract layer into the language package's generated namespace
7. assemble the handwritten semantic package layout
8. run language-specific smoke tests and contract verification
9. run documentation example verification where possible

This preserves the current important rule already reflected in the workspace scripts: generation must be grounded in a running service schema export.

## Generator Responsibility Standard

`sdkwork-sdk-generator` must support OpenAPI 3.x broadly across products, not only Sdkwork IM.

The generator should standardize these capabilities:

- derive output package names from SDK workspace naming
- derive primary client names from business package naming
- generate into a fixed generated namespace
- preserve a stable non-generated namespace for semantic code
- support browser and Node.js packaging for TypeScript
- support contract refresh from a live schema endpoint before generation
- support post-generation assembly hooks for semantic layers
- support verification hooks so generated output is not considered complete until smoke tests pass

The generator must not attempt to auto-generate all of the semantic IM experience. That is a package-owned concern because it depends on product semantics, realtime runtime behavior, and language ergonomics.

## TypeScript Public API Draft

The TypeScript standard should converge toward this usage model:

```ts
import { ImSdkClient } from '@sdkwork/im-sdk';

const sdk = new ImSdkClient({
  apiBaseUrl: import.meta.env.VITE_SDKWORK_IM_API_BASE_URL,
  websocketBaseUrl: import.meta.env.VITE_SDKWORK_IM_WS_BASE_URL,
  authToken: window.localStorage.getItem('sdkwork-im-token')!,
});

const live = await sdk.connect({
  clientRouteId: 'web-chrome-01',
  subscriptions: {
    conversations: ['conversation-support-001'],
  },
});

live.onMessage((ctx) => {
  console.log(ctx.message.type, ctx.message.summary, ctx.sequence);
});

const text = sdk.messages.createText({
  conversationId: 'conversation-support-001',
  text: 'hello world',
});

await sdk.messages.send(text);
```

This should become the primary documented path for both browser and Node.js consumers.

## Documentation Standard

The documentation set must be workflow-first and must present the semantic layer as the product.

Required primary documentation flow:

1. quick start
2. initialize the client
3. authenticate
4. connect and receive live messages
5. send text and rich messages
6. send media
7. handle custom and AI messages
8. integrate RTC
9. handle errors and reconnection
10. drop down into generated APIs only when necessary

Required editorial rules:

- examples must use the public semantic client, not generated route groups
- examples must use the long-term standard naming, not compatibility aliases
- examples must use context-first receive handlers
- docs must explain durable catch-up separately from live WebSocket receive
- docs must explicitly identify generated versus semantic ownership

## Implementation Phases

### Phase 1: Finalize TypeScript Public Standard

- replace compatibility-shaped docs examples with the new standard usage
- converge client construction on `new ImSdkClient(...)`
- move message builders under `sdk.messages`
- define `sdk.connect(...)` and a standard live connection object
- rename durable replay APIs into `sdk.sync`
- move receive APIs to context-first signatures

### Phase 2: Harden the TypeScript Runtime

- implement typed receive contexts
- standardize reconnect and state callbacks
- normalize RTC signal send and receive
- finalize message family coverage and custom message rules
- add smoke tests for browser and Node.js flows

### Phase 3: Lock the Generator Contract

- formalize generated namespace output rules
- formalize package and client naming derivation
- formalize live schema refresh as a required step
- formalize post-generation assembly hooks
- formalize verification gates

### Phase 4: Replicate the Standard Across Languages

- apply the same semantic layer boundaries to Flutter and future languages
- keep naming, message families, receive contexts, and lifecycle semantics aligned
- allow language-idiomatic syntax, but not semantic drift

## Acceptance Criteria

The new standard is successful when all of the following are true:

- a new consumer can initialize the SDK with a simple top-level config object
- the main live path is `connect()`, not a collection of transport steps
- outbound messaging is always message-first
- inbound receiving is always context-first
- custom, media, RTC, and AI-era message types are all first-class
- durable catch-up is clearly separated from live receive
- generated code is isolated under a fixed generated namespace
- docs, examples, and package exports all describe the same public standard
- the generator can reproduce the generated layer from a running OpenAPI 3.x service schema

## Decision Summary

The new Sdkwork IM application should standardize on a professional semantic SDK architecture:

- single package
- single primary client
- generated HTTP contract layer
- handwritten semantic product layer
- explicit `connect()` lifecycle
- `sdk.messages.createXxx()` plus `sdk.messages.send(...)`
- `live.onMessage((ctx) => {})`
- `sdk.sync` for durable replay
- `sdk.calls` for RTC domain workflows
- OpenAPI 3.x live-schema-driven generation

That is the correct target for an industry-standard, product-grade Chat IM SDK system.
