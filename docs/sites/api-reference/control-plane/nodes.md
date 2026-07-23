# Control Plane Node Operations

<p class="api-page-intro">
  Node operation endpoints manage drain state, reactivation, and realtime route migration between
  cluster nodes.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane/protocol"><code>Protocol</code> Protocol governance snapshots are documented separately</a>
  <a href="/api-reference/control-plane/providers"><code>Providers</code> Provider registry and binding policy flows are documented separately</a>
  <a href="/sdk/backend-sdk"><code>Backend SDK</code> Backend SDK docs explain the current control-plane consumer boundary</a>
</div>

<a id="drain-node"></a>
<section class="api-op">

## `POST /backend/v3/api/control/nodes/{nodeId}/drain`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/nodes/{nodeId}/drain</code>
  <span class="api-op-id">operationId: nodes.drain</span>
</div>

Marks the node as draining.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.nodes</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RouteNodeLifecycle`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `node_id` | `string` | Yes | Node identifier. |

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

<ApiSchemaTable schema="RouteNodeLifecycle" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The mutation payload is invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `control.write`. |
| `404` | `40401` | The requested node, plugin, or target resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the mutation. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="activate-node"></a>
<section class="api-op">

## `POST /backend/v3/api/control/nodes/{nodeId}/activate`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/nodes/{nodeId}/activate</code>
  <span class="api-op-id">operationId: nodes.activate</span>
</div>

Reactivates a previously drained node.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.nodes</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RouteNodeLifecycle`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `node_id` | `string` | Yes | Node identifier. |

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

<ApiSchemaTable schema="RouteNodeLifecycle" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The mutation payload is invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `control.write`. |
| `404` | `40401` | The requested node, plugin, or target resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the mutation. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="migrate-node-routes"></a>
<section class="api-op">

## `POST /backend/v3/api/control/nodes/{nodeId}/routes/migrate`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/nodes/{nodeId}/routes/migrate</code>
  <span class="api-op-id">operationId: nodes.routes.migrate</span>
</div>

Migrates owned realtime routes from the source node to a target node.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.nodes</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RouteMigrationResult`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `node_id` | `string` | Yes | Source node identifier. |

### Request Body

<ApiSchemaTable schema="MigrateRoutesRequest" />

### Response `200`

<ApiSchemaTable schema="RouteMigrationResult" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The mutation payload is invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `control.write`. |
| `404` | `40401` | The requested node, plugin, or target resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the mutation. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
