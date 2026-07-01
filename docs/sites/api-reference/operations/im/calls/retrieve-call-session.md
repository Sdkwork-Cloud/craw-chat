# `GET /im/v3/api/calls/sessions/{rtcSessionId}`

<p class="api-page-intro">
  Exact request and response contract for <strong>Calls</strong> in the <strong>IM Standard API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/im/calls"><code>Calls</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/im-api"><code>IM Standard API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, AppContext projection, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/im/v3/api/calls/sessions/{rtcSessionId}</code>
  <span class="api-op-id">operationId: calls.sessions.retrieve</span>
</div>

Retrieves current IM call signaling session state for reconnect and backfill.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.calls`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; conversation call scope is validated by IM.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RtcSession`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `rtcSessionId` | `string` | Yes | IM call signaling session identifier. |

### Response `200`

<ApiSchemaTable schema="RtcSession" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101`, `40102` | AppContext projection is missing or invalid. |
| `403` | `40301`, `40302` | The caller is not allowed to access the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the read or handshake flow. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
