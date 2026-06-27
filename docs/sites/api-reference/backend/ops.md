# Operations

<p class="api-page-intro">
  Operator endpoints expose service health, cluster topology, lag, replay status, runtime directory
  inspection, provider binding mirrors, and diagnostic bundles from the active node.
</p>

<div class="api-link-list">
  <a href="/deployment/runtime-operations"><code>Runtime Ops</code> Local runtime inspection, repair, preview, and restore workflows are documented separately</a>
  <a href="/reference/runtime-directory"><code>Runtime Dir</code> Managed state files and restore semantics are documented in the reference section</a>
  <a href="/sdk/index"><code>SDK</code> Treat these routes as HTTP-first operational surfaces unless a repo consumer layer is documented elsewhere</a>
</div>

<a id="get-ops-health"></a>
<section class="api-op">

## `GET /backend/v3/api/ops/health`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/ops/health</code>
  <span class="api-op-id">operationId: getOpsHealth</span>
</div>

Returns service-level health and projection-plane health.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / ops</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`ops.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 OpsHealthResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="OpsHealthResponse" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `ops.read`. |
| `503` | `*_unavailable` | Operational diagnostics are temporarily unavailable. |

</section>
<a id="get-ops-cluster"></a>
<section class="api-op">

## `GET /backend/v3/api/ops/cluster`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/ops/cluster</code>
  <span class="api-op-id">operationId: getOpsCluster</span>
</div>

Returns the cluster topology as seen by the current node.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / ops</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`ops.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ClusterView`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ClusterView" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `ops.read`. |
| `503` | `*_unavailable` | Operational diagnostics are temporarily unavailable. |

</section>
<a id="get-ops-lag"></a>
<section class="api-op">

## `GET /backend/v3/api/ops/lag`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/ops/lag</code>
  <span class="api-op-id">operationId: getOpsLag</span>
</div>

Returns lag measurements for runtime components.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / ops</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`ops.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 LagView`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="LagView" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `ops.read`. |
| `503` | `*_unavailable` | Operational diagnostics are temporarily unavailable. |

</section>
<a id="get-ops-replay-status"></a>
<section class="api-op">

## `GET /backend/v3/api/ops/replay_status`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/ops/replay_status</code>
  <span class="api-op-id">operationId: getOpsReplayStatus</span>
</div>

Returns projection replay state and replay lag metrics.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / ops</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`ops.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProjectionReplayStatusView`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ProjectionReplayStatusView" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `ops.read`. |
| `503` | `*_unavailable` | Operational diagnostics are temporarily unavailable. |

</section>
<a id="get-ops-runtime-dir"></a>
<section class="api-op">

## `GET /backend/v3/api/ops/runtime_dir`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/ops/runtime_dir</code>
  <span class="api-op-id">operationId: getOpsRuntimeDir</span>
</div>

Returns runtime directory inspection results.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / ops</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`ops.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RuntimeDirInspectionView`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="RuntimeDirInspectionView" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `ops.read`. |
| `503` | `*_unavailable` | Operational diagnostics are temporarily unavailable. |

</section>
<a id="get-ops-provider-bindings"></a>
<section class="api-op">

## `GET /backend/v3/api/ops/provider_bindings`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/ops/provider_bindings</code>
  <span class="api-op-id">operationId: getOpsProviderBindings</span>
</div>

Returns the node-local mirror of provider binding snapshots.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / ops</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`ops.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderBindingsView`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ProviderBindingsView" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `ops.read`. |
| `503` | `*_unavailable` | Operational diagnostics are temporarily unavailable. |

</section>
<a id="get-ops-provider-binding-drift"></a>
<section class="api-op">

## `GET /backend/v3/api/ops/provider_bindings/drift`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/ops/provider_bindings/drift</code>
  <span class="api-op-id">operationId: getOpsProviderBindingDrift</span>
</div>

Returns tenant drift relative to the baseline provider binding selection.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / ops</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`ops.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderBindingDriftView`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="ProviderBindingDriftView" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `ops.read`. |
| `503` | `*_unavailable` | Operational diagnostics are temporarily unavailable. |

</section>
<a id="get-ops-diagnostics"></a>
<section class="api-op">

## `GET /backend/v3/api/ops/diagnostics`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/ops/diagnostics</code>
  <span class="api-op-id">operationId: getOpsDiagnostics</span>
</div>

Returns the aggregated diagnostic bundle for the current node.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / ops</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`ops.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 DiagnosticBundle`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="DiagnosticBundle" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `ops.read`. |
| `503` | `*_unavailable` | Operational diagnostics are temporarily unavailable. |

</section>
