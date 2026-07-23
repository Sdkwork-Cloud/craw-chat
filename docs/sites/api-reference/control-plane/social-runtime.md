# Control Plane Social Runtime

<p class="api-page-intro">
  Social runtime endpoints back <code>sdk.socialRuntime</code> in the admin SDKs. They expose
  pending, delivered, and dead-letter shared-channel sync inventories, plus operational controls for
  claim, release, republish, repair, reclaim, requeue, and targeted takeover flows.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane-api"><code>Control Plane</code> Back to Control Plane overview</a>
  <a href="/sdk/backend-sdk"><code>Backend SDK</code> See the cross-language backend client surface</a>
</div>

The checked-in control-plane authority intentionally leaves current runtime repair and
inventory responses open-ended. Queue-control inputs, route semantics, and permissions are stable;
response bodies should be treated as opaque JSON and consumed through the generated admin SDK
surfaces.

<a id="get-pending_shared_channel_sync-inventory"></a>
<section class="api-op">

## `GET /backend/v3/api/control/social/runtime/pending_shared_channel_sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/social/runtime/pending_shared_channel_sync</code>
  <span class="api-op-id">operationId: social.runtime.pendingSharedChannelSync.list</span>
</div>

Read the pending shared-channel sync queue.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Response `200`

`SocialSharedChannelSyncPendingInventoryResponse` is currently modeled as an open-ended runtime
inventory payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40003` | Query or path parameters are invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks the required control-plane permission. |
| `404` | `40401` | The requested control-plane resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the read. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="get-delivery_state_shared_channel_sync-inventory"></a>
<section class="api-op">

## `GET /backend/v3/api/control/social/runtime/delivery_state_shared_channel_sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/social/runtime/delivery_state_shared_channel_sync</code>
  <span class="api-op-id">operationId: social.runtime.deliveryStateSharedChannelSync.list</span>
</div>

Read merged shared-channel sync delivery state.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Response `200`

`SocialSharedChannelSyncDeliveryStateInventoryResponse` is currently modeled as an open-ended
runtime inventory payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40003` | Query or path parameters are invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks the required control-plane permission. |
| `404` | `40401` | The requested control-plane resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the read. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="get-delivered_shared_channel_sync-inventory"></a>
<section class="api-op">

## `GET /backend/v3/api/control/social/runtime/delivered_shared_channel_sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/social/runtime/delivered_shared_channel_sync</code>
  <span class="api-op-id">operationId: social.runtime.deliveredSharedChannelSync.list</span>
</div>

Read the delivered shared-channel sync ledger.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Response `200`

`SocialSharedChannelSyncDeliveredInventoryResponse` is currently modeled as an open-ended runtime
inventory payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40003` | Query or path parameters are invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks the required control-plane permission. |
| `404` | `40401` | The requested control-plane resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the read. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="get-dead_letter_shared_channel_sync-inventory"></a>
<section class="api-op">

## `GET /backend/v3/api/control/social/runtime/dead_letter_shared_channel_sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/social/runtime/dead_letter_shared_channel_sync</code>
  <span class="api-op-id">operationId: social.runtime.deadLetterSharedChannelSync.list</span>
</div>

Read the dead-letter shared-channel sync queue.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Response `200`

`SocialSharedChannelSyncDeadLetterInventoryResponse` is currently modeled as an open-ended runtime
inventory payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40003` | Query or path parameters are invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks the required control-plane permission. |
| `404` | `40401` | The requested control-plane resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the read. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="requeue-dead_letter_shared_channel_sync"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/runtime/requeue_dead_letter_shared_channel_sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/requeue_dead_letter_shared_channel_sync</code>
  <span class="api-op-id">operationId: social.runtime.requeueDeadLetterSharedChannelSync.create</span>
</div>

Requeue all dead-letter shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201`</span></div>
</div>

### Request Body

None. This operation does not accept a JSON request body.

### Response `201`

`SocialSharedChannelSyncDeadLetterRequeueResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in control-plane authority. Treat it as opaque JSON.

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
<a id="requeue-dead_letter_shared_channel_sync-targeted"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted</code>
  <span class="api-op-id">operationId: social.runtime.requeueDeadLetterSharedChannelSyncTargeted.create</span>
</div>

Requeue selected dead-letter shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SocialSharedChannelSyncDeadLetterTargetedRequeueRequest" />

### Response `201`

`SocialSharedChannelSyncDeadLetterTargetedRequeueResponse` is currently modeled as an open-ended
runtime operation payload in the checked-in control-plane authority. Treat it as opaque JSON.

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
<a id="repair-social-runtime-snapshot"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/runtime/repair_derived_snapshot`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/repair_derived_snapshot</code>
  <span class="api-op-id">operationId: social.runtime.repairDerivedSnapshot.create</span>
</div>

Repair the persisted social runtime derived snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201`</span></div>
</div>

### Request Body

None. This operation does not accept a JSON request body.

### Response `201`

`SocialRuntimeRepairResponse` is currently modeled as an open-ended runtime repair payload in the
checked-in control-plane authority. Treat it as opaque JSON.

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
<a id="repair_shared_channel_sync"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/runtime/repair_shared_channel_sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/repair_shared_channel_sync</code>
  <span class="api-op-id">operationId: social.runtime.repairSharedChannelSync.create</span>
</div>

Repair shared-channel sync backlog state.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201`</span></div>
</div>

### Request Body

None. This operation does not accept a JSON request body.

### Response `201`

`SocialSharedChannelSyncRepairResponse` is currently modeled as an open-ended runtime repair
payload in the checked-in control-plane authority. Treat it as opaque JSON.

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
<a id="claim-pending_shared_channel_sync-targeted"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/runtime/claim_pending_shared_channel_sync_targeted`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/claim_pending_shared_channel_sync_targeted</code>
  <span class="api-op-id">operationId: social.runtime.claimPendingSharedChannelSyncTargeted.create</span>
</div>

Claim selected pending shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SocialSharedChannelSyncPendingTargetedClaimRequest" />

### Response `201`

`SocialSharedChannelSyncPendingClaimResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in control-plane authority. Treat it as opaque JSON.

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
<a id="release-pending_shared_channel_sync-targeted"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/runtime/release_pending_shared_channel_sync_targeted`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/release_pending_shared_channel_sync_targeted</code>
  <span class="api-op-id">operationId: social.runtime.releasePendingSharedChannelSyncTargeted.create</span>
</div>

Release selected pending shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SocialSharedChannelSyncPendingTargetedReleaseRequest" />

### Response `201`

`SocialSharedChannelSyncPendingReleaseResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in control-plane authority. Treat it as opaque JSON.

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
<a id="reclaim-stale-pending_shared_channel_sync"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/runtime/reclaim_stale_pending_shared_channel_sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/reclaim_stale_pending_shared_channel_sync</code>
  <span class="api-op-id">operationId: social.runtime.reclaimStalePendingSharedChannelSync.create</span>
</div>

Reclaim stale shared-channel sync pending ownership.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201`</span></div>
</div>

### Request Body

None. This operation does not accept a JSON request body.

### Response `201`

`SocialSharedChannelSyncPendingStaleReclaimResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in control-plane authority. Treat it as opaque JSON.

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
<a id="republish-pending_shared_channel_sync-targeted"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/runtime/republish_pending_shared_channel_sync_targeted`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/republish_pending_shared_channel_sync_targeted</code>
  <span class="api-op-id">operationId: social.runtime.republishPendingSharedChannelSyncTargeted.create</span>
</div>

Republish selected pending shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SocialSharedChannelSyncTargetedRepublishRequest" />

### Response `201`

`SocialSharedChannelSyncTargetedRepublishResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in control-plane authority. Treat it as opaque JSON.

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
<a id="takeover-pending_shared_channel_sync-targeted"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/runtime/takeover_pending_shared_channel_sync_targeted`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/takeover_pending_shared_channel_sync_targeted</code>
  <span class="api-op-id">operationId: social.runtime.takeoverPendingSharedChannelSyncTargeted.create</span>
</div>

Take over selected pending shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SocialSharedChannelSyncPendingTargetedTakeoverRequest" />

### Response `201`

`SocialSharedChannelSyncPendingTakeoverResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in control-plane authority. Treat it as opaque JSON.

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
