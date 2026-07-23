# `POST /backend/v3/api/control/social/runtime/requeue_dead_letter_shared_channel_sync`

<p class="api-page-intro">
  Exact request and response contract for <strong>Social Runtime</strong> in the <strong>Backend API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane/social-runtime"><code>Social Runtime</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/backend-api"><code>Backend API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, resolved request-context, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/requeue_dead_letter_shared_channel_sync</code>
  <span class="api-op-id">operationId: social.runtime.requeueDeadLetterSharedChannelSync.create</span>
</div>

Requeue all dead-letter shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201`</span></div>
</div>

### Request Body

None. This operation does not accept a JSON request body.

### Response `201`

`SocialSharedChannelSyncDeadLetterRequeueResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The mutation payload is invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `control.write`. |
| `404` | `40401` | The requested node, plugin, or target resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the mutation. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
