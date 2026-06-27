> Migrated from `docs/sites/sdk/typescript-sdk.md` on 2026-06-24.
> Owner: SDKWork maintainers

# TypeScript SDK

The official TypeScript consumer package in the IM SDK family is `@sdkwork/im-sdk`.

This package is the primary IM consumer SDK for browser and Node.js and follows one package rule:

- one installable package for normal consumers
- one primary client class: `ImSdkClient`
- one generated transport boundary assembled under `src/generated/**`
- one semantic SDK surface at the package root

Use `ImSdkClient` for application code. The root client exposes semantic modules such as
`sdk.conversations`, `sdk.messages`, `sdk.calls`, and `sdk.connect(...)`. When you need exact
OpenAPI transport control, use `sdk.transport.presence`, `sdk.transport.realtime`,
`sdk.transport.chat`, and `sdk.transport.streams`.

## Current Delivery Reality

The TypeScript standard is now intentionally narrow and explicit:

- one public consumer package: `@sdkwork/im-sdk`
- one primary public client: `ImSdkClient`
- one internal generated layer assembled from `generated/server-openapi`
- one generator-owned authoring boundary under `generated/server-openapi`
- no second backend-named public companion package

## Package Contract

| Concern | Value |
| --- | --- |
| Official package | `@sdkwork/im-sdk` |
| Primary client | `ImSdkClient` |
| Runtime targets | Browser and Node.js |
| Route-aligned transport modules | `sdk.transport.presence`, `sdk.transport.realtime`, `sdk.transport.chat`, `sdk.transport.streams` |
| Generated source boundary | `src/generated/**` |
| Generator-owned authoring boundary | `generated/server-openapi` |

## Quick Start

### Browser

```ts
import { ImSdkClient } from '@sdkwork/im-sdk';

const sdk = new ImSdkClient({
  baseUrl: import.meta.env.VITE_SDKWORK_IM_BASE_URL,
  authToken: window.localStorage.getItem('sdkwork-im-token') ?? undefined,
});
```

### Node.js

```ts
import { ImSdkClient } from '@sdkwork/im-sdk';

const sdk = new ImSdkClient({
  baseUrl: process.env.SDKWORK_IM_BASE_URL!,
  authToken: process.env.SDKWORK_IM_TOKEN,
});
```

### Split API And WebSocket Origins

If HTTP and realtime upgrades terminate on different origins, keep the config flat:

```ts
const sdk = new ImSdkClient({
  apiBaseUrl: 'https://api.example.com',
  websocketBaseUrl: 'wss://realtime.example.com',
  authToken: window.localStorage.getItem('sdkwork-im-token') ?? undefined,
});
```

This is the preferred configuration model. You do not need a nested transport-config wrapper for
normal use.

## Authentication

Authentication is issued by `sdkwork-appbase`. The IM SDK passes the resulting appbase-issued credential
through constructor `authToken`; Sdkwork IM receives the verified AppContext projection and does not implement login, token refresh,
tenant, organization, or current-account resolution.

```ts
const sdk = new ImSdkClient({
  baseUrl,
  authToken: appbaseAccessToken,
});
```

Behavior:

- `authToken` is passed through to the generated HTTP transport and live helpers
- transport-level code can still call `setAuthToken(...)` on the generated client when it is not using `ImSdkClient`

## Portal Snapshots

Portal snapshot reads belong to the app API SDK family, not the IM SDK root client. Keep portal
bootstrap in the app SDK composition layer, then pass the resulting SDKWork credential into
`ImSdkClient`.

```ts
const sdk = new ImSdkClient({
  baseUrl: 'https://chat.example.com',
  authToken: appbaseCredential,
});

const inbox = await sdk.conversations.list();
console.log(inbox.items.length);
```

Use `sdkwork-im-app-sdk` when you need app-owned portal snapshots.

## Recommended Integration Order

For a new application, build against the SDK in this order:

1. Construct `ImSdkClient`
2. Authenticate with `authToken`
3. Send a text message with `sdk.conversations.postText(...)`
4. Add live push with `sdk.connect(...)`
5. Add durable HTTP coordination with `sdk.transport.realtime.events.list(...)` and
   `sdk.transport.realtime.events.ack(...)`
6. Add Drive-backed media message references
7. Add custom, AI, and agent/workflow messages
8. Add IM call lifecycle and signaling

That order keeps the core app lifecycle stable before richer message and transport features are
added.

## API Reference Map

Use the TypeScript SDK as the default application surface. Jump to the route-level reference below
when you need exact OpenAPI operations, request bodies, or transport DTO details:

| App domain | Primary TypeScript SDK surface | Exact API reference |
| --- | --- | --- |
| SDKWork appbase credential pass-through | `authToken` | [Auth And Client Init](/sdk/auth-and-client-init) |
| Portal shell and workspace snapshots | `sdkwork-im-app-sdk` dependency | [App SDK](/sdk/app-sdk) |
| Conversation lifecycle and handoff | `sdk.conversations.create`, `sdk.conversations.createAgentDialog`, `sdk.conversations.bindDirectChat` | [Conversations](/api-reference/im/conversations) |
| Live, chat, and game rooms | `sdk.rooms.create`, `sdk.rooms.enter`, `sdk.rooms.get`, `sdk.rooms.leave` | [Rooms](/api-reference/im/rooms) |
| Membership and read cursors | `sdk.conversations.listMembers`, `sdk.conversations.addMember`, `sdk.conversations.updateReadCursor` | [Membership and Read State](/api-reference/im/membership-and-read-state) |
| Message schemas and semantic send ergonomics | `sdk.conversations.postText(...)`, `sdk.conversations.postMessage(...)` | [Messages](/api-reference/im/messages) |
| Drive-backed media message references | `sdkwork-drive` for file lifecycle, then `sdk.conversations.postMessage(...)` with `ContentPart.drive` and `MediaResource` | [Media](/api-reference/im/media) |
| Realtime presence, live subscriptions, and durable replay | `sdk.connect(...)`, `sdk.transport.presence`, `sdk.transport.realtime` | [Realtime And Presence](/api-reference/im/session-and-realtime) |
| IM call lifecycle and signaling-side HTTP calls | `sdk.calls.start(...)`, `sdk.calls.sendSignal(...)`, `sdk.calls.issueParticipantCredential(...)`, `sdk.calls.watchIncoming(...)` | [Calls](/api-reference/im/calls) |
| Stream transport and checkpointing | `sdk.transport.streams.create(...)`, `sdk.transport.streams.frames.create(...)`, `sdk.transport.streams.checkpoint.create(...)`, `sdk.transport.streams.complete(...)` | [Streams](/api-reference/im/streams) |

## Conversations

`sdk.conversations` is the route-aligned domain for conversation lifecycle, membership changes,
read cursors, direct timeline access, and system-channel publishing. Use it when you want a
near-1:1 mapping with the public OpenAPI routes. Use the root message-first path
`sdk.createXxxMessage(...)` plus `sdk.send(...)` when you want the default builder-first
experience. Use `sdk.messages` when you want the same behavior through a namespaced module surface.

### Conversation Lifecycle And Handoff

| Task | Method |
| --- | --- |
| Create a regular conversation | `sdk.conversations.create(...)` |
| Create a one-to-one agent dialog | `sdk.conversations.createAgentDialog(...)` |
| Create an agent handoff conversation | `sdk.conversations.createAgentHandoff(...)` |
| Create a system channel conversation | `sdk.conversations.createSystemChannel(...)` |
| Read the current summary projection | `sdk.conversations.get(...)` |
| Read handoff state | `sdk.conversations.getAgentHandoffState(...)` |
| Accept a handoff | `sdk.conversations.acceptAgentHandoff(...)` |
| Resolve a handoff | `sdk.conversations.resolveAgentHandoff(...)` |
| Close a handoff | `sdk.conversations.closeAgentHandoff(...)` |

```ts
const conversation = await sdk.conversations.create({
  conversationId: 'conversation-order-1',
  conversationType: 'group',
});

await sdk.conversations.createAgentDialog({
  conversationId: 'conversation-agent-1',
  agentId: 'assistant-1',
});

const handoffConversation = await sdk.conversations.createAgentHandoff({
  conversationId: 'conversation-handoff-1',
  targetId: 'billing-specialist',
  targetKind: 'agent',
  handoffSessionId: 'handoff-1',
  handoffReason: 'invoice_exception',
});

await sdk.conversations.createSystemChannel({
  conversationId: 'conversation-system-1',
  subscriberId: '1',
});

const summary = await sdk.conversations.get(conversation.conversationId);
const handoffState = await sdk.conversations.getAgentHandoffState(
  handoffConversation.conversationId,
);

await sdk.conversations.acceptAgentHandoff(handoffConversation.conversationId);
await sdk.conversations.resolveAgentHandoff(handoffConversation.conversationId);
await sdk.conversations.closeAgentHandoff(handoffConversation.conversationId);

console.log(summary.conversationId, handoffState.status);
```

### Rooms

`sdk.rooms` binds live, chat, and game room identities to group conversations. Message delivery still uses `sdk.conversations` on the returned `conversationId`.

| Task | Method |
| --- | --- |
| Create a room | `sdk.rooms.create(...)` |
| Read room capacity metadata | `sdk.rooms.get(...)` |
| Self-join a room | `sdk.rooms.enter(...)` |
| Leave a room | `sdk.rooms.leave(...)` |

```ts
const room = await sdk.rooms.create({
  conversationId: 'c_live_001',
  roomId: 'room_live_001',
  roomKind: 'live',
});

await sdk.rooms.enter(room.roomId);
await sdk.conversations.postText(room.conversationId, 'hello room');
await sdk.rooms.leave(room.roomId);
```

### Membership, Read Cursor, And Direct Posting

| Task | Method |
| --- | --- |
| List visible members | `sdk.conversations.listMembers(...)` |
| Add a member | `sdk.conversations.addMember(...)` |
| Remove a member | `sdk.conversations.removeMember(...)` |
| Transfer ownership | `sdk.conversations.transferOwner(...)` |
| Change a member role | `sdk.conversations.changeMemberRole(...)` |
| Leave the conversation | `sdk.conversations.leave(...)` |
| Read the current principal cursor | `sdk.conversations.getReadCursor(...)` |
| Advance the cursor | `sdk.conversations.updateReadCursor(...)` |
| Read the route-level message timeline | `sdk.conversations.listMessages(...)` |
| Post a raw message body | `sdk.conversations.postMessage(...)` |
| Post plain text quickly | `sdk.conversations.postText(...)` |
| Publish a raw system message | `sdk.conversations.publishSystemMessage(...)` |
| Publish system text quickly | `sdk.conversations.publishSystemText(...)` |

```ts
const conversationId = conversation.conversationId;

await sdk.conversations.addMember(conversationId, {
  principalId: '3',
  principalKind: 'user',
  role: 'member',
});

const members = await sdk.conversations.listMembers(conversationId);

await sdk.conversations.changeMemberRole(conversationId, {
  memberId: members.items[0]?.memberId ?? 'member-1',
  role: 'admin',
});

const cursor = await sdk.conversations.getReadCursor(conversationId);

await sdk.conversations.updateReadCursor(conversationId, {
  readSeq: cursor.readSeq,
});

const timeline = await sdk.conversations.listMessages(conversationId);

await sdk.conversations.postText(conversationId, 'Route-aligned text delivery', {
  summary: 'Route-aligned text delivery',
});

await sdk.conversations.publishSystemText(
  conversationId,
  'System maintenance starts in 5 minutes',
  {
    summary: 'Maintenance window',
  },
);

await sdk.conversations.transferOwner(conversationId, {
  memberId: members.items[0]?.memberId ?? 'member-1',
});

await sdk.conversations.removeMember(conversationId, {
  memberId: 'member-2',
});

await sdk.conversations.leave(conversationId);

console.log(timeline.items.length);
```

### Inbox

Inbox is exposed through the conversations semantic module and the generated transport:

```ts
const inbox = await sdk.conversations.list();
const exactInbox = await sdk.transport.chat.inbox.retrieve();
console.log(inbox.items.length);
```

## Messages

The outbound experience is message-first at the client root:

1. create a message with `sdk.createXxxMessage(...)`
2. send it with `sdk.send(message)`

The same builders remain available on `sdk.messages` when you want a namespaced module surface.
The primary message entrypoints are `sdk.createTextMessage(...)`, `sdk.send(...)`, and
`sdk.decodeMessage(...)`.

### Text

```ts
const message = sdk.createTextMessage({
  conversationId: 'conversation-1',
  text: 'hello world',
  summary: 'Greeting',
  renderHints: { tone: 'friendly' },
});

await sdk.send(message);
```

### Media

```ts
const image = sdk.createImageMessage({
  conversationId: 'conversation-1',
  drive: {
    driveUri: 'drive://spaces/space_app_upload_demo/nodes/node_storefront_png',
    spaceId: 'space_app_upload_demo',
    nodeId: 'node_storefront_png',
    nodeVersion: '1',
  },
  resource: {
    id: 'node_storefront_png',
    kind: 'image',
    source: 'provider_asset',
    uri: 'drive://spaces/space_app_upload_demo/nodes/node_storefront_png',
    fileName: 'storefront.png',
    mimeType: 'image/png',
  },
  mediaRole: 'attachment',
  text: 'Latest storefront concept',
  summary: 'Storefront concept',
});

await sdk.send(image);
```

Upload and file access are owned by `sdkwork-drive`. After Drive returns a node, the TypeScript IM
SDK sends that node as `ContentPart.drive` using a `DriveReference`; the neighboring
`MediaResource` describes how the message uses the media.

### Standard Message Families

The semantic layer includes common IM, custom business, and AI-era message families.

| Family | Method |
| --- | --- |
| Text | `createTextMessage` |
| Image / video / audio / file | `createImageMessage`, `createVideoMessage`, `createAudioMessage`, `createFileMessage` |
| Location / link / card / music / contact | `createLocationMessage`, `createLinkMessage`, `createCardMessage`, `createMusicMessage`, `createContactMessage` |
| Sticker / voice | `createStickerMessage`, `createVoiceMessage` |
| Custom business payload | `createCustomMessage` |
| Structured data / signal / stream reference | `createDataMessage`, `createSignalMessage`, `createStreamReferenceMessage` |
| AI text / AI image generation / AI video generation | `createAiTextMessage`, `createAiImageGenerationMessage`, `createAiVideoGenerationMessage` |
| Agent / agent state / handoff | `createAgentMessage`, `createAgentStateMessage`, `createAgentHandoffMessage` |
| Tool / workflow | `createToolResultMessage`, `createWorkflowEventMessage` |

Examples:

```ts
const custom = sdk.createCustomMessage({
  conversationId: 'conversation-1',
  customType: 'order.card',
  text: 'Order ready for review',
  data: {
    orderId: 'order-1001',
    amount: 128.5,
  },
});

const aiText = sdk.createAiTextMessage({
  conversationId: 'conversation-1',
  text: 'Assistant answer',
  prompt: 'summarize the last order',
  model: 'gpt-5.4',
  status: 'completed',
});

const agent = sdk.createAgentMessage({
  conversationId: 'conversation-1',
  text: 'Primary support agent joined',
  agentId: 'assistant-1',
  agentName: 'Assistant',
  stage: 'active',
  status: 'online',
  capabilities: ['summarize', 'route'],
});

const handoff = sdk.createAgentHandoffMessage({
  conversationId: 'conversation-1',
  text: 'Escalating to billing specialist',
  fromAgentId: 'router',
  toAgentId: 'billing-specialist',
  reason: 'invoice_exception',
  status: 'pending',
});

const workflow = sdk.createWorkflowEventMessage({
  conversationId: 'conversation-1',
  text: 'Workflow advanced to fulfillment',
  workflowId: 'wf-1',
  eventName: 'state.changed',
  stage: 'fulfillment',
  status: 'success',
  data: {
    orderId: 'order-1001',
  },
});
```

### System Channel

Use `channel: 'system'` and send through the same `sdk.send(...)` entrypoint.

```ts
const notice = sdk.createTextMessage({
  conversationId: 'conversation-system-1',
  channel: 'system',
  text: 'Deployment starts in 5 minutes',
  summary: 'Maintenance window',
});

await sdk.send(notice);
```

### Edit, Recall, And Decode

```ts
await sdk.editTextMessage('msg-1', 'Updated content');
await sdk.recallMessage('msg-1');

const decoded = sdk.decodeMessage(aiText.body);
console.log(decoded.type, decoded.summary);
```

## Media

The IM SDK does not own file lifecycle work. Use `sdkwork-drive` to create, version, authorize, and
retrieve files, then use the IM SDK to send a message that references the Drive node. This keeps the
storage authority in Drive and the IM contract focused on message semantics.

| Task | Method |
| --- | --- |
| Upload bytes, choose version, authorize file access | `sdkwork-drive` |
| Send an image reference | `sdk.createImageMessage(...)` |
| Send video, audio, or document references | `sdk.createVideoMessage(...)`, `sdk.createAudioMessage(...)`, `sdk.createFileMessage(...)` |
| Decode received message media references | `sdk.decodeMessage(...)` |

```ts
const drive = {
  driveUri: 'drive://spaces/space_app_upload_demo/nodes/node_project_brief_pdf',
  spaceId: 'space_app_upload_demo',
  nodeId: 'node_project_brief_pdf',
  nodeVersion: '3',
};

const fileMessage = sdk.createFileMessage({
  conversationId: 'conversation-1',
  drive,
  resource: {
    id: drive.nodeId,
    kind: 'document',
    source: 'provider_asset',
    uri: drive.driveUri,
    fileName: 'brief.pdf',
    mimeType: 'application/pdf',
    sizeBytes: String(file.size),
  },
  mediaRole: 'attachment',
  text: 'Project brief',
  summary: 'Project brief',
});

await sdk.send(fileMessage);

const decoded = sdk.decodeMessage(fileMessage.body);
console.log(decoded.attachments[0]?.drive?.driveUri);
```

`DriveReference` is the durable pointer, and `MediaResource` is the normalized usage snapshot on the
message. The canonical Drive URI shape is `drive://spaces/{spaceId}/nodes/{nodeId}`.

## Realtime Model

The SDK intentionally separates live push from durable catch-up.

| Need | Use |
| --- | --- |
| Live WebSocket push | `sdk.connect(...)` |
| Durable resume / background replay / explicit ACK | `sdk.sync.catchUp(...)` and `sdk.sync.ack(...)` |

### Live Push

```ts
const live = await sdk.connect({
  clientRouteId: 'web-chrome-01',
  subscriptions: {
    conversations: ['conversation-1'],
  },
});

live.messages.onConversation('conversation-1', (message, context) => {
  console.log(context.conversationId, message.type, context.sequence);
  void context.ack();
});

live.lifecycle.onStateChange((state) => {
  console.log(state.status);
});

live.lifecycle.onError((context) => {
  console.log(context);
});
```

The recommended receive surface is payload-first by domain stream. Your callback receives the final
semantic object first and the operational receive context second. That keeps rendering and business
logic focused on `message`, while `context` remains available for sequencing,
sender metadata, raw-event inspection, and `context.ack()`.

Each context gives you:

- the semantic payload: `message`
- `sequence`
- `receivedAt`
- `sender`
- `rawEvent`
- `context.ack()`

Use the live domain streams this way:

- `live.messages.onConversation(...)` for conversation-scoped message handling
- `live.events.onConversation(...)` for raw conversation realtime event handling
- `live.lifecycle.onStateChange(...)` for `connected`, `error`, and `closed` transitions
- `live.lifecycle.onError(...)` for realtime protocol and socket-level failures

### Durable Catch-Up

```ts
const batch = await sdk.sync.catchUp({ limit: 50 });

for (const item of batch.items) {
  if (item.kind === 'message') {
    console.log(item.sequence, item.message.type, item.message.summary);
    await item.ack();
  }
}

await sdk.sync.ack(batch);
```

Use `sdk.sync` for resume, replay, reconciliation, worker consumption, or when your application
cannot rely on a continuously open WebSocket connection. `context.ack()` advances acknowledgement
through the current receive context, while `sdk.sync.ack(...)` lets you commit the highest durable
replay position explicitly.

### Browser And Node.js WebSocket Factories

If your realtime gateway needs a custom upgrade strategy, pass `webSocketFactory` at client
construction time.

Important: the default global `WebSocket` constructor cannot attach `Authorization` headers. In
plain browser environments, authenticated realtime should use a browser-safe credential path:

- `ImWebSocketAuthOptions.automatic()` is the standard TypeScript default
- automatic auth resolves to a query credential for the default browser `WebSocket` constructor
- automatic auth resolves to a header credential when a custom `webSocketFactory` is present
- prefer exchanging the primary access token for a short-lived realtime ticket or query credential
- prefer `wss://` plus short-lived credentials over long-lived query credentials
- use `sdk.connect({ url })` when the gateway returns a pre-signed realtime URL

Node.js and custom runtimes can provide header-based upgrades through `webSocketFactory`.

```ts
import { ImSdkClient, ImWebSocketAuthOptions } from '@sdkwork/im-sdk';

const sdk = new ImSdkClient({
  baseUrl: 'https://api.example.com',
  authToken: window.localStorage.getItem('sdkwork-im-token') ?? undefined,
  webSocketAuth: ImWebSocketAuthOptions.queryBearer({
    queryParameterName: 'rt',
    credentialProvider: async ({ authToken }) =>
      issueRealtimeTicket(authToken),
  }),
});

const live = await sdk.connect({
  subscriptions: {
    conversations: ['conversation-1'],
  },
});
```

```ts
const realtimeTicket = await issueRealtimeTicket();

const live = await sdk.connect({
  url: `wss://realtime.example.com/im/v3/api/realtime/ws?rt=${encodeURIComponent(realtimeTicket)}`,
  subscriptions: {
    conversations: ['conversation-1'],
  },
});
```

```ts
import WebSocket from 'ws';

const sdk = new ImSdkClient({
  baseUrl: process.env.SDKWORK_IM_BASE_URL!,
  authToken: process.env.SDKWORK_IM_TOKEN,
  webSocketAuth: ImWebSocketAuthOptions.headerBearer(),
  webSocketFactory: ({ url, protocols, headers }) =>
    new WebSocket(url, protocols, { headers }),
});
```

## Calls

IM owns the call lifecycle and signaling surface through `sdk.calls`. RTC provider SDKs stay focused
on media runtime, native driver, and provider bridge capabilities.

```ts
const session = await sdk.calls.start({
  rtcSessionId: 'rtc-1',
  conversationId: 'conversation-1',
  rtcMode: 'group_call',
});

await sdk.calls.invite(session.rtcSessionId, {
  signalingStreamId: 'rtc-signal-1',
});

await sdk.calls.sendSignal(session.rtcSessionId, {
  signalingStreamId: 'rtc-signal-1',
  signalType: 'offer',
  payload: JSON.stringify({
    sdp: 'v=0...',
  }),
});

const credential = await sdk.calls.issueParticipantCredential(session.rtcSessionId, {
  participantId: '1',
});

const unsubscribeIncoming = sdk.calls.subscribe((incoming) => {
  console.log(incoming.rtcSessionId, incoming.conversationId);
});

await sdk.calls.watchIncoming({
  connection: live,
  conversationIds: ['conversation-1'],
});

unsubscribeIncoming();
```

Handle incoming call invites through `sdk.calls.subscribe(...)` and `sdk.calls.watchIncoming(...)`.
Use `sdk.calls.sendSignal(...)`
for SDP/ICE/business signaling payloads and `sdk.calls.issueParticipantCredential(...)` for provider
join credentials.

## Route-Aligned Transport Modules

Some route groups are intentionally available through the generated transport client because they
already match the public OpenAPI contract cleanly. Use the higher-level semantic modules for chat,
live receive, media, and call workflows. Use `sdk.transport` when you need exact route-group control
for presence, realtime coordination, inbox, or stream transport.

```ts
import { ImSdkClient } from '@sdkwork/im-sdk';

const sdk = new ImSdkClient({
  baseUrl: 'http://127.0.0.1:18079',
  authToken: 'token',
});

await sdk.transport.presence.heartbeat.create({ clientRouteId: 'web-chrome-01' });
await sdk.transport.presence.me.retrieve();
await sdk.transport.realtime.events.list({ limit: 20 });
await sdk.conversations.listMessages('conversation-1');
await sdk.transport.chat.inbox.retrieve();
await sdk.transport.streams.create({
  streamId: 'stream-demo-1',
  streamType: 'custom.delta.text',
  scopeKind: 'conversation',
  scopeId: 'conversation-1',
  durabilityClass: 'durableSession',
  schemaRef: 'custom.delta.text.v1',
});
```

Use `sdk.transport` when you need exact DTOs or route-group control. Reach for
`sdk.transport.presence.heartbeat.create(...)`, `sdk.transport.presence.me.retrieve()`,
`sdk.transport.realtime.events.list(...)`, `sdk.transport.chat.inbox.retrieve()`, and
`sdk.transport.streams.create(...)` when the route group already matches the API cleanly. Use the
semantic domains on `ImSdkClient` for normal application integration.

## Assembly Metadata

The TypeScript workspace publishes its contract into `.sdkwork-assembly.json`.

Use that file when you need the verified package-layer picture instead of guessing from folder
names:

- `generatedAt` tells you whether the assembly content actually changed
- `manifestPath` records the manifest that defined each package layer
- TypeScript records `generated`, `composed`, and `root` package layers together
- the internal generated layer is assembled from `generated/server-openapi` and emitted under `src/generated/**`
- the public consumer layer is the root `@sdkwork/im-sdk` package

## Local Workspace Workflow

When you maintain the checked-in TypeScript workspace locally, use the folders for their exact
roles:

- `generated/server-openapi`
  Generator-owned internal build workspace that materializes the assembled transport layer
- `composed`
  Manual-owned authoring boundary that imports the generated layer through the internal alias
- repository package root
  The assembled single-package consumer output published as `@sdkwork/im-sdk`

If you are debugging package metadata or release layout, start from `.sdkwork-assembly.json` before
editing manifests by hand.

## Verification

Local verification from the TypeScript workspace:

```bash
node ../bin/verify-typescript-workspace.mjs
node ./bin/assemble-single-package.mjs
node ../../../../../sdk/sdkwork-sdk-generator/node_modules/typescript/bin/tsc -p tsconfig.build.json --noEmit
node ./test/sdkwork-im-client.test.mjs
```

From the repository root, the family-level entrypoint is:

```bash
node ./sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language typescript
```

## What To Read Next

- Read [App SDK](/sdk/app-sdk) when you want the product-facing family rules above the
  TypeScript-specific details.
- Read [Flutter SDK](/sdk/flutter-sdk) when you need the current secondary consumer baseline and
  its parity gaps.
- Read [Language Support](/sdk/language-support) when you need the full cross-language maturity
  matrix.
- Read [Generator Boundary](/sdk/generator-boundary) when you need the exact split between
  `src/generated/**`, `generated/server-openapi`, and `composed`.
- Read [Portal Access](/api-reference/app/portal-access) when you need the underlying HTTP
  contract for portal snapshots and SDKWork appbase credential pass-through.
- Read [Messages](/api-reference/im/messages), [Realtime Presence](/api-reference/im/session-and-realtime),
  and [Calls](/api-reference/im/calls) when you need the route-level contract behind the semantic
  TypeScript SDK.

