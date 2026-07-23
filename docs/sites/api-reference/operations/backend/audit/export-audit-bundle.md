# `GET /backend/v3/api/audit/export`

<p class="api-page-intro">
  Exact request and response contract for <strong>Audit</strong> in the <strong>Backend API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/backend/audit"><code>Audit</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/backend-api"><code>Backend API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, resolved request-context, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/audit/export</code>
  <span class="api-op-id">operationId: export.retrieve</span>
</div>

Exports an audit bundle containing the visible records at the time of the request.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / audit</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`audit.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 AuditExportBundle`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="AuditExportBundle" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `audit.read`. |

</section>
