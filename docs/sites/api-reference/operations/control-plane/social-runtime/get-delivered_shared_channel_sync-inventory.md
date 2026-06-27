# `GET /backend/v3/api/control/social/runtime/delivered_shared_channel_sync`

<p class="api-page-intro">
  Exact request and response contract for <strong>Social Runtime</strong> in the <strong>Backend API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane/social-runtime"><code>Social Runtime</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/backend-api"><code>Backend API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, AppContext projection, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/social/runtime/delivered_shared_channel_sync</code>
  <span class="api-op-id">operationId: getDeliveredSharedChannelSyncInventory</span>
</div>

Read the delivered shared-channel sync ledger.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Response `200`

`SocialSharedChannelSyncDeliveredInventoryResponse` is currently modeled as an open-ended runtime
inventory payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The queue read request is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested inventory source does not exist. |
| `409` | `*_conflict` | Current runtime state blocks the read. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
