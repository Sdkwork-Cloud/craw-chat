> Migrated from `docs/sites/api-reference/operations/control-plane/social/block-user.md` on 2026-06-24.
> Owner: SDKWork maintainers

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
  <code>/backend/v3/api/control/social/user_blocks</code>
  <span class="api-op-id">operationId: blockUser</span>
</div>

Block a user in the social graph.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.social</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="BlockUserRequest" />

### Response `200`

`SocialUserBlockCommitResponse` is currently modeled as an open-ended social commit payload in the
checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The user-block payload is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | A referenced user, direct chat, or block aggregate does not exist. |
| `409` | `*_conflict` | Current social graph state blocks the mutation. |
| `503` | `*_unavailable` | The social graph runtime or persistence dependency is unavailable. |

</section>

