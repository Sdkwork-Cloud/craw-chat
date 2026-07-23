# `GET /im/v3/api/realtime/ws`

<p class="api-page-intro">
  Exact request and response contract for <strong>Realtime Presence</strong> in the <strong>IM Standard API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/im/session-and-realtime"><code>Realtime Presence</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/im-api"><code>IM Standard API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, resolved request-context, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/im/v3/api/realtime/ws</code>
  <span class="api-op-id">operationId: realtime.ws.retrieve</span>
</div>

Upgrades the connection to WebSocket. This page documents the HTTP handshake surface only; it does
not expand the full realtime frame protocol.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.connect(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; active client route is prepared before upgrade.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Security

- SDKWork dual token validated at the appbase boundary, then a server-resolved request context
- Client route ownership and client route binding are validated before upgrade
- Browser runtimes that cannot send upgrade headers should prefer a gateway-issued short-lived
  realtime ticket or pre-signed `wss://` URL instead of a long-lived query credential

### Headers

| Header | Required | Description |
| --- | --- | --- |
| `Sec-WebSocket-Protocol` | No | When set to `ccp/ws/1`, the runtime enters the `CcpJson` subprotocol mode. Otherwise it falls back to the legacy JSON frame mode. |

### Response `200`

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
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller is not allowed to access the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the read or handshake flow. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
