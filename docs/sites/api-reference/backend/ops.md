# Operations

<p class="api-page-intro">
  Backend operations report health, topology, authoritative runtime lag, commercial readiness,
  runtime-directory state, provider-binding state, and bounded diagnostics. No endpoint rebuilds
  business state or exposes a state-reconstruction control plane.
</p>

<div class="api-link-list">
  <a href="/deployment/runtime-operations"><code>Runtime Ops</code> Deployment-owned inspection and recovery workflows</a>
  <a href="/reference/runtime-directory"><code>Runtime Dir</code> Managed runtime-directory contract</a>
  <a href="/sdk/index"><code>SDK</code> Generated Backend SDK family and client construction</a>
</div>

<a id="get-ops-health"></a>
<section class="api-op">

## `GET /backend/v3/api/ops/health`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/ops/health</code>
  <span class="api-op-id">operationId: health.retrieve</span>
</div>

Returns current service, dependency, database, relay, and readiness health.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / ops</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`ops.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SdkWorkApiResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="SdkWorkApiResponse" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `ops.read`. |
| `503` | `50301` | Operational diagnostics are temporarily unavailable. |

</section>
<a id="get-ops-cluster"></a>
<section class="api-op">

## `GET /backend/v3/api/ops/cluster`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/ops/cluster</code>
  <span class="api-op-id">operationId: cluster.retrieve</span>
</div>

Returns bounded cluster topology and authoritative route totals visible to the current node.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / ops</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`ops.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SdkWorkApiResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="SdkWorkApiResponse" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `ops.read`. |
| `503` | `50301` | Operational diagnostics are temporarily unavailable. |

</section>
<a id="get-ops-lag"></a>
<section class="api-op">

## `GET /backend/v3/api/ops/lag`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/ops/lag</code>
  <span class="api-op-id">operationId: lag.retrieve</span>
</div>

Lists real operational lag measurements from registered runtime components.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / ops</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`ops.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 LagListResponse`</span></div>
</div>

### Query Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `page_size` | integer | No | Page size from 1 through 200 |
| `cursor` | string | No | Opaque continuation cursor |

### Response `200`

<ApiSchemaTable schema="LagListResponse" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `ops.read`. |
| `503` | `50301` | Operational diagnostics are temporarily unavailable. |

</section>
<a id="get-ops-commercial-readiness"></a>
<section class="api-op">

## `GET /backend/v3/api/ops/commercial_readiness`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/ops/commercial_readiness</code>
  <span class="api-op-id">operationId: commercialReadiness.retrieve</span>
</div>

Returns current release-gate evidence without converting missing evidence into a successful state.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / ops</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`ops.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SdkWorkApiResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="SdkWorkApiResponse" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `ops.read`. |
| `503` | `50301` | Operational diagnostics are temporarily unavailable. |

</section>
<a id="get-ops-runtime-dir"></a>
<section class="api-op">

## `GET /backend/v3/api/ops/runtime_dir`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/ops/runtime_dir</code>
  <span class="api-op-id">operationId: runtimeDir.retrieve</span>
</div>

Returns redacted runtime-directory inspection results for the active node.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / ops</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`ops.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SdkWorkApiResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="SdkWorkApiResponse" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `ops.read`. |
| `503` | `50301` | Operational diagnostics are temporarily unavailable. |

</section>
<a id="get-ops-provider-bindings"></a>
<section class="api-op">

## `GET /backend/v3/api/ops/provider_bindings`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/ops/provider_bindings</code>
  <span class="api-op-id">operationId: ops.providerBindings.list</span>
</div>

Lists the bounded effective provider-binding state available to the current node.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / ops</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`ops.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ProviderBindingSnapshotListResponse`</span></div>
</div>

### Query Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `page_size` | integer | No | Page size from 1 through 200 |
| `cursor` | string | No | Opaque continuation cursor |

### Response `200`

<ApiSchemaTable schema="ProviderBindingSnapshotListResponse" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `ops.read`. |
| `503` | `50301` | Operational diagnostics are temporarily unavailable. |

</section>
<a id="get-ops-provider-binding-drift"></a>
<section class="api-op">

## `GET /backend/v3/api/ops/provider_bindings/drift`

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
<a id="get-ops-diagnostics"></a>
<section class="api-op">

## `GET /backend/v3/api/ops/diagnostics`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/ops/diagnostics</code>
  <span class="api-op-id">operationId: diagnostics.retrieve</span>
</div>

Returns a redacted, bounded diagnostic bundle for the current node.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / ops</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`ops.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SdkWorkApiResponse`</span></div>
</div>

### Response `200`

<ApiSchemaTable schema="SdkWorkApiResponse" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `ops.read`. |
| `503` | `50301` | Operational diagnostics are temporarily unavailable. |

</section>
