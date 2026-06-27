# `GET /app/v3/api/portal/workspace`

<p class="api-page-intro">
  Exact request and response contract for <strong>Portal Access</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/portal-access"><code>Portal Access</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, AppContext projection, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/app/v3/api/portal/workspace</code>
  <span class="api-op-id">operationId: getWorkspace</span>
</div>

Reads the current authenticated tenant workspace summary used by the portal shell.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.portal.workspace.retrieve()`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalWorkspaceView`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalWorkspaceView" />

### Example Response

```json
{
  "name": "Nebula Commerce IM",
  "slug": "nebula-commerce-im",
  "tier": "Enterprise",
  "region": "CN-East / Multi-AZ",
  "supportPlan": "Platinum",
  "seats": 84,
  "activeBrands": 12,
  "uptime": "99.983%"
}
```

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `portal_access_denied`, `permission_denied` | The principal cannot access the workspace summary. |
| `503` | `*_unavailable` | The workspace snapshot source is unavailable. |

</section>
