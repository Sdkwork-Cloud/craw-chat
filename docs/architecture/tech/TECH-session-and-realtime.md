> Migrated from `docs/sites/api-reference/im/session-and-realtime.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Realtime And Presence

<p class="api-page-intro">
  Realtime endpoints cover health probes, client route presence heartbeats, realtime subscription
  sync, event polling, ACK tracking, and WebSocket upgrade. The
  recommended TypeScript SDK model separates live push from durable replay: use
  <code>sdk.connect(...)</code> for live push and <code>sdk.sync.catchUp(...)</code> for durable
  catch-up.
</p>

<div class="api-note">
  Health probes are open endpoints. The WebSocket entry on this page documents the handshake route
  and protocol boundary. Generation still owns only the HTTP contract, while the semantic
  TypeScript SDK additionally ships a handwritten live runtime behind <code>sdk.connect(...)</code>.
</div>

<div class="api-link-list">
  <a href="/sdk/typescript-sdk"><code>SDK</code> <code>@sdkwork/im-sdk</code> and Flutter package <code>im_sdk</code> are the official app-consumer SDKs for realtime bootstrap, live receive, replay, and SDKWork appbase credential pass-through</a>
</div>

## Recommended SDK Mapping

| Need | SDK entry |
| --- | --- |
| SDKWork appbase credential pass-through | constructor `authToken`; generated transport `setAuthToken(...)` when working at transport level |
| Live push receive | `sdk.connect(...)` |
| Durable replay and ACK | `sdk.sync.catchUp(...)`, `sdk.sync.ack(...)` |
| Client route bootstrap before connect | `sdk.connect({ clientRouteId, subscriptions })` |
| Presence heartbeat and snapshot | `sdk.generated.presence.heartbeat(...)`, `sdk.generated.presence.getPresenceMe()` |
| Route-level subscription sync and polling | `sdk.generated.realtime.syncRealtimeSubscriptions(...)`, `sdk.generated.realtime.listRealtimeEvents(...)`, `sdk.generated.realtime.ackRealtimeEvents(...)` |
| Health probes | Direct HTTP `GET /healthz` and `GET /readyz` when you need infrastructure probes |

On the live path, register `live.messages.onConversation(...)`,
`live.events.onConversation(...)`, `live.lifecycle.onStateChange(...)`, and
`live.lifecycle.onError(...)` after `sdk.connect(...)`. The live runtime is payload-first by
domain stream: your callback receives the final `message` object first, then
the operational receive context second. Each receive context exposes `context.ack()` for per-event
acknowledgement. When you want to advance the durable replay cursor explicitly, use
`sdk.sync.ack(...)`.

For conversation-scoped receive, prefer `live.messages.onConversation(...)`. For IM call invites
and signals, use `sdk.calls.subscribe(...)` with `sdk.calls.watchIncoming(...)`; call subscriptions
are delivered through the same conversation realtime stream.

When you need exact transport-level control, the semantic runtime and the generated route groups are
designed to coexist:

```ts
const live = await sdk.connect({
  clientRouteId: 'web-chrome-01',
  subscriptions: {
    conversations: ['conversation-1'],
  },
});

await sdk.generated.realtime.syncRealtimeSubscriptions({
  clientRouteId: 'web-chrome-01',
  items: [
    {
      scopeType: 'conversation',
      scopeId: 'conversation-1',
      eventTypes: ['message.created', 'message.updated', 'message.recalled'],
    },
  ],
});

const window = await sdk.generated.realtime.listRealtimeEvents({
  limit: 50,
});

await sdk.generated.realtime.ackRealtimeEvents({
  ackedSeq: window.ackedThroughSeq ?? 0,
});
```

<a id="get-healthz"></a>
<section class="api-op">

## `GET /healthz`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/healthz</code>
  <span class="api-op-id">operationId: getHealthz</span>
</div>

Returns process liveness for the app runtime.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Open endpoint</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>Direct HTTP probe</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Not required</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 HealthResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="HealthResponse" />

</section>

<a id="get-readyz"></a>
<section class="api-op">

## `GET /readyz`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/readyz</code>
  <span class="api-op-id">operationId: getReadyz</span>
</div>

Returns process readiness for the app runtime.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Open endpoint</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>Direct HTTP probe</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Not required</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 HealthResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="HealthResponse" />

</section>

<a id="heartbeat-presence"></a>
<section class="api-op">

## `POST /im/v3/api/presence/heartbeat`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/presence/heartbeat</code>
  <span class="api-op-id">operationId: heartbeatPresence</span>
</div>

Refreshes the presence heartbeat for the current client route.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.generated.presence.heartbeat(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; Client route ownership and client route binding are enforced where required.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PresenceSnapshotView`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="PresenceHeartbeatRequest" />

### Response `200`

<ApiSchemaTable schema="PresenceSnapshotView" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="get-presence-me"></a>
<section class="api-op">

## `GET /im/v3/api/presence/me`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/im/v3/api/presence/me</code>
  <span class="api-op-id">operationId: getPresenceMe</span>
</div>

Reads the current principal's presence snapshot.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.presence.getPresenceMe()`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; Client route ownership and client route binding are enforced where required.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PresenceSnapshotView`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PresenceSnapshotView" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | The caller is not allowed to access the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the read or handshake flow. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="sync-realtime-subscriptions"></a>
<section class="api-op">

## `POST /im/v3/api/realtime/subscriptions/sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/realtime/subscriptions/sync</code>
  <span class="api-op-id">operationId: syncRealtimeSubscriptions</span>
</div>

Replaces the realtime subscription set for the current client route.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.connect(...)`, `sdk.generated.realtime.syncRealtimeSubscriptions(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; Client route ownership and client route binding are enforced where required.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RealtimeSubscriptionSnapshot`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SyncRealtimeSubscriptionsRequest" />

### Response `200`

<ApiSchemaTable schema="RealtimeSubscriptionSnapshot" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="list-realtime-events"></a>
<section class="api-op">

## `GET /im/v3/api/realtime/events`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/im/v3/api/realtime/events</code>
  <span class="api-op-id">operationId: listRealtimeEvents</span>
</div>

Fetches realtime events from the realtime event window.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.sync.catchUp(...)`, `sdk.generated.realtime.listRealtimeEvents(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; Client route ownership and client route binding are enforced where required.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RealtimeEventWindow`</span></div>
</div>

### Query Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `afterSeq` | `uint64 \| null` | No | Continue reading after this realtime sequence. |
| `limit` | `uint64 \| null` | No | Maximum number of events to return. The current default is `100`. |

### Response `200`

<ApiSchemaTable schema="RealtimeEventWindow" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | The caller is not allowed to access the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the read or handshake flow. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="ack-realtime-events"></a>
<section class="api-op">

## `POST /im/v3/api/realtime/events/ack`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/realtime/events/ack</code>
  <span class="api-op-id">operationId: ackRealtimeEvents</span>
</div>

Acknowledges the highest realtime event sequence consumed by the client.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.sync.ack(...)`, `context.ack()`, `sdk.generated.realtime.ackRealtimeEvents(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; Client route ownership and client route binding are enforced where required.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RealtimeAckState`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="AckRealtimeEventsRequest" />

### Response `200`

<ApiSchemaTable schema="RealtimeAckState" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="realtime-websocket"></a>
<section class="api-op">

## `GET /im/v3/api/realtime/ws`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/im/v3/api/realtime/ws</code>
  <span class="api-op-id">operationId: realtimeWebsocket</span>
</div>

Upgrades the connection to WebSocket. This page documents the HTTP handshake surface only; it does
not expand the full realtime frame protocol.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.connect(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; active client route is prepared before upgrade.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`101 Switching Protocols`</span></div>
</div>

### Security

- SDKWork dual token validated at the appbase boundary, then AppContext projection headers
- Client route ownership and client route binding are validated before upgrade
- Browser runtimes that cannot send upgrade headers should prefer a gateway-issued short-lived
  realtime ticket or pre-signed `wss://` URL instead of a long-lived query credential

### Headers

| Header | Required | Description |
| --- | --- | --- |
| `Sec-WebSocket-Protocol` | No | When set to `ccp/ws/1`, the runtime enters the `CcpJson` subprotocol mode. Otherwise it falls back to the legacy JSON frame mode. |

### Response `101`

| Output | Type | Description |
| --- | --- | --- |
| `Upgrade` | `header` | Returned as `websocket` when the handshake succeeds. |
| `Connection` | `header` | Returned as `Upgrade` for the switching-protocols handshake. |
| `Sec-WebSocket-Accept` | `header` | Standard RFC 6455 handshake proof derived from the client key. |
| `Sec-WebSocket-Protocol` | `header \| null` | Echoed when the server accepts a negotiated subprotocol such as `ccp/ws/1`. |

### Response Notes

- Status code is `101 Switching Protocols`.
- After the handshake, the connection leaves the request-response lifecycle and enters realtime transport mode.
- For TypeScript consumers, the standard SDK entrypoint for that transport is `sdk.connect(...)`.
- When browser auth requires a query credential, prefer gateway token exchange plus short-lived
  credentials and pass the final browser-safe URL through `sdk.connect({ url })`.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `409` | `client_route_scope_conflict` | The client route key is already bound to another principal. |
| `409` | `disconnect_fence_conflict` | Routing or session state blocks the upgrade. |

</section>

