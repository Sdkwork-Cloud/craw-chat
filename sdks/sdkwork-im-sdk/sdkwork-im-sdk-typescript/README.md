# @sdkwork/im-sdk

Layered TypeScript SDK for Sdkwork IM IM runtime APIs.

This package exposes the app-facing `ImSdkClient` facade. Generated HTTP
transport is consumed through `@sdkwork/im-sdk-generated`, whose source remains
under `generated/server-openapi`; realtime websocket behavior and semantic
convenience modules live in `src/`.

## Realtime Websocket

`ImSdkClient.connect()` opens `/im/v3/api/realtime/ws` through the authored
websocket adapter. The adapter preserves deployment base paths, for example
`wss://chat.example.com/sdkwork/chat` becomes
`wss://chat.example.com/sdkwork/chat/im/v3/api/realtime/ws`.

Browser runtimes use `globalThis.WebSocket`. Browser websocket APIs cannot set
custom request headers, so authenticated browser connections send the current
dual-token session in the first `auth.init` frame. The gateway validates that
frame through appbase current-session, projects trusted AppContext headers to
session-gateway, responds with `auth.ok`, and only then does the SDK send
subscription frames.

The handshake URL never carries auth tokens or conversation subscriptions. A
browser connection starts with:

```json
{
  "type": "auth.init",
  "requestId": "sdkwork-im-auth-init-1",
  "authToken": "<auth-token>",
  "accessToken": "<access-token>",
  "deviceId": "<device-id>"
}
```

After `auth.ok`, the SDK completes the CCP control handshake (`hello` → `hello_ack` →
`auth_bind` → `auth_ok`) and only then enters lifecycle `open` and sends `subscriptions.sync`
frames as CCP `kind: cmd` business envelopes. `conversationId` values must not be appended to the websocket URL.

Node, Tauri, tests, and other host runtimes can inject a transport:

```ts
import { ImSdkClient, type ImWebSocketFactory } from '@sdkwork/im-sdk';

const webSocketFactory: ImWebSocketFactory = (url, options) => {
  return new NodeWebSocket(url, options.protocols, {
    headers: options.headers,
  });
};

const client = new ImSdkClient({
  authToken,
  accessToken,
  headerProvider: () => ({
    'X-Sdkwork-Tenant-Id': tenantId,
    'X-Sdkwork-User-Id': userId,
    'X-Sdkwork-Device-Id': deviceId,
  }),
  webSocketFactory,
  websocketBaseUrl: 'wss://chat.example.com/sdkwork/chat',
});

const connection = await client.connect({
  connectionTimeoutMs: 15_000,
  deviceId,
  heartbeat: {
    intervalMs: 30_000,
    timeoutMs: 75_000,
  },
  subscriptions: {
    conversations: [conversationId],
  },
});

connection.messages.onConversation(conversationId, async (message, context) => {
  await context.ack();
});

connection.subscriptions.syncConversations([
  conversationId,
  anotherConversationId,
]);
```

The wire mode is CCP JSON over WebSocket subprotocol `sdkwork-im.ccp.ws.v1`:

- Browser mode sends `auth.init` first, waits for `auth.ok`, completes the CCP handshake, and only then reports lifecycle `open`.
- Node/Tauri injected websocket factories may forward `Authorization`, `Access-Token`, and AppContext headers for trusted host runtimes and skip `auth.init`.
- `subscriptions.sync` is deferred until the CCP `ready` phase. Calling `connection.subscriptions.syncConversations(...)` before `open` only marks a dirty snapshot; the SDK flushes it after `auth_ok`.
- A connection that never reaches the native websocket `open` event emits `websocket_connect_timeout` and queues a close for the first safe moment. Application connection managers should treat this as a reconnectable transport error.
- Heartbeats are enabled by default. The SDK sends `ping` frames and treats any inbound frame as liveness; if the socket stays silent past `timeoutMs`, it emits `websocket_heartbeat_timeout` and closes the connection so the app-level connection manager can reconnect. Pass `heartbeat: false` only for tests or runtimes with their own equivalent liveness checks.
- Server `error` control frames are surfaced through `connection.lifecycle.onError(...)`. Fatal websocket auth, session, token, upstream, and connect errors also move the connection into `error` state and close the socket; non-fatal subscription or business errors leave the socket open.
- `event.window` frames are decoded into `messages.onConversation(...)` callbacks.
- `context.ack()` sends `events.ack` with the received realtime sequence.
- Auth tokens are not sent as websocket URL query parameters or subprotocol names.

Do not hand-edit `generated/server-openapi`. Update the OpenAPI authority or
authored facade source, then run:

```bash
node ../bin/generate-sdk.mjs --language typescript
```

## Release Snapshot Boundary

This workspace inherits the current SDK release snapshot from
`artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`.

- `state = template_only_pending_generation`
- `generationStatus = generated`
- `releaseStatus = not_published`
- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`

The release catalog remains the machine-readable source of truth:
`sdk-release-catalog.json`.
