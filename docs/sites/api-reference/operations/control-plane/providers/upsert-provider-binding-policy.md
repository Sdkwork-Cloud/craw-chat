# `POST /backend/v3/api/control/provider_bindings`

<p class="api-page-intro">
  Exact request and response contract for <strong>Provider Governance</strong> in the <strong>Backend API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane/providers"><code>Provider Governance</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/backend-api"><code>Backend API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, AppContext projection, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/provider_bindings</code>
  <span class="api-op-id">operationId: upsertProviderBindingPolicy</span>
</div>

Writes a deployment-level or tenant-level provider binding policy entry.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.providers</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderBindingCommitResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="UpsertProviderBindingPolicyRequest" />

### Response `200`

<ApiSchemaTable schema="ProviderBindingCommitResponse" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `409` | `provider_policy_version_conflict` | `expectedBaseVersion` does not match the latest policy version. |
| `404` | `provider_plugin_not_found` | The referenced plugin is not present in the registry. |

</section>
