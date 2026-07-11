# `POST /backend/v3/api/control/social/external_connections`

<p class="api-page-intro">
  Exact request and response contract for <strong>Social Graph Control</strong> in the <strong>Backend API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane/social"><code>Social Graph Control</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/backend-api"><code>Backend API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, AppContext projection, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/external_connections</code>
  <span class="api-op-id">operationId: establishExternalConnection</span>
</div>

Establish an external collaboration connection.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.social</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="EstablishExternalConnectionRequest" />

### Response `200`

`SocialExternalConnectionCommitResponse` is currently modeled as an open-ended social commit payload
in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The mutation payload is invalid. |
| `401` | `40101` | AppContext projection is missing or invalid. |
| `403` | `40301` | The caller lacks `control.write`. |
| `404` | `40401` | The requested node, plugin, or target resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the mutation. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
