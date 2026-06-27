# Control Plane Provider Governance

<p class="api-page-intro">
  Provider governance endpoints expose the provider registry, effective bindings, policy history,
  diffs, previews, and rollback operations.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane/protocol"><code>Protocol</code> Protocol registry and governance snapshots are documented separately</a>
  <a href="/api-reference/control-plane/nodes"><code>Nodes</code> Drain, activate, and route migration are documented separately</a>
  <a href="/sdk/backend-sdk"><code>Backend SDK</code> Backend SDK docs explain the generated control module boundary</a>
</div>

<a id="get-provider_registry"></a>
<section class="api-op">

## `GET /backend/v3/api/control/provider_registry`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/provider_registry</code>
  <span class="api-op-id">operationId: getProviderRegistry</span>
</div>

Returns the provider registry snapshot, including installed plugins and the effective global
bindings resolved by the registry.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.providers</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderRegistrySnapshotResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ProviderRegistrySnapshotResponse" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | Query or path parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `503` | `*_unavailable` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="get-provider-bindings"></a>
<section class="api-op">

## `GET /backend/v3/api/control/provider_bindings`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/provider_bindings</code>
  <span class="api-op-id">operationId: getProviderBindings</span>
</div>

Reads effective provider bindings for the deployment scope or a tenant override scope.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.providers</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderBindingsResponse`</span></div>
</div>

### Query Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `tenantId` | `string \| null` | No | Tenant identifier for override scope. Omit for deployment-level bindings. |

### Response `200`

<ApiSchemaTable schema="ProviderBindingsResponse" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | Query or path parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `503` | `*_unavailable` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="upsert-provider-binding-policy"></a>
<section class="api-op">

## `POST /backend/v3/api/control/provider_bindings`

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

<a id="get-provider-policy-history"></a>
<section class="api-op">

## `GET /backend/v3/api/control/provider_policies`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/provider_policies</code>
  <span class="api-op-id">operationId: getProviderPolicyHistory</span>
</div>

Returns provider policy history.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.providers</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderPolicyHistoryResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ProviderPolicyHistoryResponse" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | Query or path parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `503` | `*_unavailable` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="get-provider-policy-diff"></a>
<section class="api-op">

## `GET /backend/v3/api/control/provider_policies/diff`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/provider_policies/diff</code>
  <span class="api-op-id">operationId: getProviderPolicyDiff</span>
</div>

Compares two provider policy versions.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.providers</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderPolicyDiffResponse`</span></div>
</div>

### Query Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `fromVersion` | `uint64` | Yes | Base version. |
| `toVersion` | `uint64` | Yes | Target version. |

### Response `200`

<ApiSchemaTable schema="ProviderPolicyDiffResponse" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | Query or path parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `503` | `*_unavailable` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="preview-provider-policy"></a>
<section class="api-op">

## `POST /backend/v3/api/control/provider_policies/preview`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/provider_policies/preview</code>
  <span class="api-op-id">operationId: previewProviderPolicy</span>
</div>

Previews a provider policy mutation without persisting it.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.providers</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderPolicyPreview`</span></div>
</div>

### Request Body

Uses the same request schema as `POST /backend/v3/api/control/provider_bindings`.

<ApiSchemaTable schema="UpsertProviderBindingPolicyRequest" />

### Response `200`

<ApiSchemaTable schema="ProviderPolicyPreview" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `invalid_provider_policy` | The mutation payload is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found`, `provider_plugin_not_found` | The requested node, plugin, or target resource does not exist. |
| `409` | `*_conflict`, `provider_policy_conflict` | Current control-plane state blocks the mutation. |
| `503` | `*_unavailable` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="rollback-provider-policy"></a>
<section class="api-op">

## `POST /backend/v3/api/control/provider_policies/rollback`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/provider_policies/rollback</code>
  <span class="api-op-id">operationId: rollbackProviderPolicy</span>
</div>

Rolls back the provider policy history to a specific version.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.providers</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderPolicyHistoryResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="ProviderPolicyRollbackRequest" />

### Response `200`

<ApiSchemaTable schema="ProviderPolicyHistoryResponse" />

### Response Notes

- On rollback responses, `status` is returned as `rolled_back`.


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `invalid_provider_policy` | The mutation payload is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found`, `provider_plugin_not_found` | The requested node, plugin, or target resource does not exist. |
| `409` | `*_conflict`, `provider_policy_conflict` | Current control-plane state blocks the mutation. |
| `503` | `*_unavailable` | The governance snapshot or provider runtime is unavailable. |

</section>
