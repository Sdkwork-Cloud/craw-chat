> Owner: SDKWork maintainers

## `GET /im/v3/api/realtime/ws`

Upgrades the connection to WebSocket. This page documents the HTTP handshake surface only; frame-level
authentication and CCP protocol details live in `docs/superpowers/specs/2026-06-25-websocket-login-verification-standard-design.md`.

### Security

- **HTTP upgrade is anonymous** for browser clients. Dual-token validation happens in the first `auth.init` JSON text
  frame after `101 Switching Protocols`.
- Node/Tauri/Flutter native runtimes may supply `Authorization` and `Access-Token` headers during upgrade and skip `auth.init`.
- Server routing uses `has_websocket_upgrade_auth_headers` from `im-app-context` (checks `Authorization`, `access-token`, and `Access-Token`).
- Tokens must **not** appear in URL query parameters.
- Shared gate: `crates/sdkwork-im-websocket-auth-gate` (used by cloud gateway + session-gateway compat path).
- Upgrade header detection: `im-app-context::has_websocket_upgrade_auth_headers`.

### Headers

| Header | Required | Description |
| --- | --- | --- |
| `Sec-WebSocket-Protocol` | Recommended | Negotiate `sdkwork-im.ccp.ws.v1` for the current CCP JSON wire mode. |
| `Authorization` | Native only | `Bearer <authToken>` when the runtime can send upgrade headers (Node/Tauri/Flutter `IOWebSocketChannel`). |
| `Access-Token` | Native only | Access token when the runtime can send upgrade headers. |

### Handshake flow (browser)

1. `GET /im/v3/api/realtime/ws?deviceId=<id>` returns `101 Switching Protocols`.
2. Client sends `auth.init` with `authToken`, `accessToken`, and `deviceId`.
3. Gateway or embedded session-gateway validates through IAM and returns `auth.ok`.
4. Client completes CCP `hello` / `hello_ack` / `auth_bind` / `auth_ok` and enters `ready`.
5. Client sends `subscriptions.sync` (`kind: cmd`) only after `ready`. Sending business frames during the CCP handshake returns `CCP_CONTROL_REQUIRED` and closes the connection.

### Handshake flow (native / mobile / Node factory)

1. `GET /im/v3/api/realtime/ws?deviceId=<id>` with dual-token upgrade headers returns `101`.
2. Server validates IAM from headers, resolves `deviceId` from query when absent in token claims, and skips `auth.init`.
3. Client sends CCP `hello` immediately after the socket is open (no `auth.ok` frame).
4. Client sends `subscriptions.sync` only after CCP `auth_ok` (`ready` phase).

See the full runtime matrix in `docs/superpowers/specs/2026-06-25-websocket-login-verification-standard-design.md` §12.1.

### Response `101`

| Output | Type | Description |
| --- | --- | --- |
| `Upgrade` | header | `websocket` |
| `Connection` | header | `Upgrade` |
| `Sec-WebSocket-Accept` | header | RFC 6455 proof |
| `Sec-WebSocket-Protocol` | header \| null | Echoed when `sdkwork-im.ccp.ws.v1` is negotiated |

### Error Responses

| HTTP | `code` / body | Description |
| --- | --- | --- |
| `426` | `no upgrade state was present` | Websocket route was incorrectly wrapped by HTTP middleware or dispatched through `oneshot`. This is a server wiring defect, not an auth failure. |
| `401` | `websocket_auth_failed` (frame) | Invalid or expired dual-token session after `auth.init`. |
| `409` | `client_route_scope_conflict` | Client route key already bound to another principal. |
| `503` | `websocket_overloaded` | Connection semaphore saturated. |

### SDK

- TypeScript: `@sdkwork/im-sdk` → `ImSdkClient.connect()` / `createImLiveConnection()`
- Flutter: `createImLiveConnection()` in `im_sdk_composed`

### PC client connection model

`sdkwork-im-pc` keeps **one shared WebSocket per authenticated browser session** through
`@sdkwork/im-pc-core/sdk/pcRealtimeConnectionManager`:

- Chat inbox, conversation messages, friend-request scopes, and incoming-call watch all multiplex on the same
  `ImLiveConnection`.
- In-flight `connect()` attempts are deduplicated with `sharedConnectionPromise`.
- `recoverPcLiveConnection()` only restarts when status is not `open` or `connecting`.
- Reconnect uses exponential backoff with jitter and a circuit breaker after repeated fatal failures.
- Wire subscription sync runs on lifecycle `open` only (not when `connect()` resolves).
- `CallService` passes the shared connection to `calls.watchIncoming({ connection })` and registers leased
  conversation ids so `subscriptions.syncConversations` stays aligned.

Do not call `ImSdkClient.connect()` directly from PC feature services; subscribe through the manager instead.

### H5 client connection model

`sdkwork-im-h5` uses one shared browser WebSocket through
`packages/sdkwork-im-h5-chat/src/services/chatRealtimeService.ts`:

- Inbox refresh and conversation live messages multiplex on the same `ImLiveConnection`.
- Subscription snapshots sync on lifecycle `open` and when handlers change while the connection is already open.
- Do not call `ImSdkClient.connect()` from page components; use `subscribeConversationLiveMessages` /
  `subscribeInboxLiveRefresh`.

### Routing implementation

- `session_gateway::build_realtime_websocket_router` serves the upgrade route without `WebFrameworkLayer`.
- `sdkwork-routes-im-realtime-open-api` wraps only HTTP realtime/presence routes with IAM interceptors.
- Unified gateways merge the websocket router before the wrapped business router and exclude `REALTIME_WS` from embedded
  oneshot dispatch.
