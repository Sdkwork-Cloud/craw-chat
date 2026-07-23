# `GET /backend/v3/api/ops/provider_bindings/drift`

<p class="api-page-intro">
  Exact request and response contract for <strong>Operations</strong> in the <strong>Backend API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/backend/ops"><code>Operations</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/backend-api"><code>Backend API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, resolved request-context, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/ops/provider_bindings/drift</code>
  <span class="api-op-id">operationId: ops.providerBindings.drift.retrieve</span>
</div>

Lists bounded tenant drift against the governed provider-binding baseline.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / ops</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`ops.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderBindingDriftListResponse`</span></div>
</div>

### Query Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `page_size` | integer | No | Page size from 1 through 200 |
| `cursor` | string | No | Opaque continuation cursor |

### Response `200`

<ApiSchemaTable schema="ProviderBindingDriftListResponse" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `ops.read`. |
| `503` | `50301` | Operational diagnostics are temporarily unavailable. |

</section>
