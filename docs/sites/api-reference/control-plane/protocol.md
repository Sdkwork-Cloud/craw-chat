# Control Plane Protocol Governance

<p class="api-page-intro">
  Protocol governance endpoints expose the control-plane health probe, protocol registry inventory,
  and the effective governance snapshot applied to compatible clients.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane/providers"><code>Providers</code> Provider registry, bindings, preview, and rollback flows are documented separately</a>
  <a href="/sdk/backend-sdk"><code>Backend SDK</code> Read the backend SDK page for control module boundaries and release-state limits</a>
</div>

<a id="get-control-healthz"></a>
<section class="api-op">

## `GET /healthz`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/healthz</code>
  <span class="api-op-id">operationId: getControlPlaneHealthz</span>
</div>

Returns the liveness state of the control-plane process.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>Open endpoint</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.protocol</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Not required</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ControlPlaneHealthResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ControlPlaneHealthResponse" />

</section>

<a id="get-protocol_registry"></a>
<section class="api-op">

## `GET /backend/v3/api/control/protocol_registry`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/protocol_registry</code>
  <span class="api-op-id">operationId: getProtocolRegistry</span>
</div>

Returns the active protocol registry snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.protocol</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProtocolRegistryResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ProtocolRegistryResponse" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | Query or path parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `503` | `*_unavailable` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="get-protocol_governance"></a>
<section class="api-op">

## `GET /backend/v3/api/control/protocol_governance`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/protocol_governance</code>
  <span class="api-op-id">operationId: getProtocolGovernance</span>
</div>

Returns the effective control-plane governance snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.protocol</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProtocolGovernanceResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ProtocolGovernanceResponse" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | Query or path parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `503` | `*_unavailable` | The governance snapshot or provider runtime is unavailable. |

</section>
