> Migrated from `docs/sites/api-reference/operations/app/portal-access/get-home.md` on 2026-06-24.
> Owner: SDKWork maintainers

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
  <code>/app/v3/api/portal/home</code>
  <span class="api-op-id">operationId: getHome</span>
</div>

Reads the public tenant-portal home snapshot used for the landing experience.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.portal.home.retrieve()`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PortalSnapshot`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="PortalSnapshot" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `503` | `*_unavailable` | The portal snapshot source is unavailable. |

</section>

