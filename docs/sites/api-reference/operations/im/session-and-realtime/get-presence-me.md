# `GET /im/v3/api/presence/me`

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
  <code>/im/v3/api/presence/me</code>
  <span class="api-op-id">operationId: presence.me.retrieve</span>
</div>

Reads the current principal's presence snapshot.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.presence.getPresenceMe()`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; Client route ownership and client route binding are enforced where required.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PresenceSnapshotView`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PresenceSnapshotView" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller is not allowed to access the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the read or handshake flow. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
