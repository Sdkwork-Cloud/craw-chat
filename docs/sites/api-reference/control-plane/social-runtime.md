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
  <span class="api-op-id">operationId: getPendingSharedChannelSyncInventory</span>
</div>

Read the pending shared-channel sync queue.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
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
| `400` | `invalid_request` | The queue read request is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested inventory source does not exist. |
| `409` | `*_conflict` | Current runtime state blocks the read. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="get-delivery_state_shared_channel_sync-inventory"></a>
<section class="api-op">

## `GET /backend/v3/api/control/social/runtime/delivery_state_shared_channel_sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/social/runtime/delivery_state_shared_channel_sync</code>
  <span class="api-op-id">operationId: getSharedChannelSyncDeliveryStateInventory</span>
</div>

Read merged shared-channel sync delivery state.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
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
| `400` | `invalid_request` | The queue read request is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested inventory source does not exist. |
| `409` | `*_conflict` | Current runtime state blocks the read. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="get-delivered_shared_channel_sync-inventory"></a>
<section class="api-op">

## `GET /backend/v3/api/control/social/runtime/delivered_shared_channel_sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/social/runtime/delivered_shared_channel_sync</code>
  <span class="api-op-id">operationId: getDeliveredSharedChannelSyncInventory</span>
</div>

Read the delivered shared-channel sync ledger.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
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
| `400` | `invalid_request` | The queue read request is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested inventory source does not exist. |
| `409` | `*_conflict` | Current runtime state blocks the read. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="get-dead_letter_shared_channel_sync-inventory"></a>
<section class="api-op">

## `GET /backend/v3/api/control/social/runtime/dead_letter_shared_channel_sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/social/runtime/dead_letter_shared_channel_sync</code>
  <span class="api-op-id">operationId: getDeadLetterSharedChannelSyncInventory</span>
</div>

Read the dead-letter shared-channel sync queue.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
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
| `400` | `invalid_request` | The queue read request is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks the required control-plane permission. |
| `404` | `*_not_found` | The requested inventory source does not exist. |
| `409` | `*_conflict` | Current runtime state blocks the read. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="requeue-dead_letter_shared_channel_sync"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/runtime/requeue-dead_letter_shared_channel_sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/requeue-dead_letter_shared_channel_sync</code>
  <span class="api-op-id">operationId: requeueDeadLetterSharedChannelSync</span>
</div>

Requeue all dead-letter shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

`SocialSharedChannelSyncDeadLetterRequeueResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The requeue request is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | The requested dead-letter queue does not exist. |
| `409` | `*_conflict` | Current runtime state blocks the requeue. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="requeue-dead_letter_shared_channel_sync-targeted"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/runtime/requeue-dead_letter_shared_channel_sync-targeted`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/requeue-dead_letter_shared_channel_sync-targeted</code>
  <span class="api-op-id">operationId: requeueDeadLetterSharedChannelSyncTargeted</span>
</div>

Requeue selected dead-letter shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SocialSharedChannelSyncDeadLetterTargetedRequeueRequest" />

### Response `200`

`SocialSharedChannelSyncDeadLetterTargetedRequeueResponse` is currently modeled as an open-ended
runtime operation payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The targeted requeue payload is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | One or more targeted request keys do not exist. |
| `409` | `*_conflict` | Current runtime state blocks the requeue. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="repair-social-runtime-snapshot"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/runtime/repair_derived_snapshot`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/repair_derived_snapshot</code>
  <span class="api-op-id">operationId: repairSocialRuntimeSnapshot</span>
</div>

Repair the persisted social runtime derived snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

`SocialRuntimeRepairResponse` is currently modeled as an open-ended runtime repair payload in the
checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The repair request is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | A referenced runtime snapshot or queue does not exist. |
| `409` | `*_conflict` | Current runtime state blocks the repair. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="repair_shared_channel_sync"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/runtime/repair_shared_channel_sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/repair_shared_channel_sync</code>
  <span class="api-op-id">operationId: repairSharedChannelSync</span>
</div>

Repair shared-channel sync backlog state.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

`SocialSharedChannelSyncRepairResponse` is currently modeled as an open-ended runtime repair
payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The repair request is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | A referenced shared-channel sync backlog or queue does not exist. |
| `409` | `*_conflict` | Current runtime state blocks the repair. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="claim-pending_shared_channel_sync-targeted"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/runtime/claim-pending_shared_channel_sync-targeted`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/claim-pending_shared_channel_sync-targeted</code>
  <span class="api-op-id">operationId: claimPendingSharedChannelSyncTargeted</span>
</div>

Claim selected pending shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SocialSharedChannelSyncPendingTargetedClaimRequest" />

### Response `200`

`SocialSharedChannelSyncPendingClaimResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The targeted claim payload is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | One or more targeted request keys do not exist. |
| `409` | `*_conflict` | Current runtime ownership blocks the claim. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="release-pending_shared_channel_sync-targeted"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/runtime/release-pending_shared_channel_sync-targeted`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/release-pending_shared_channel_sync-targeted</code>
  <span class="api-op-id">operationId: releasePendingSharedChannelSyncTargeted</span>
</div>

Release selected pending shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SocialSharedChannelSyncPendingTargetedReleaseRequest" />

### Response `200`

`SocialSharedChannelSyncPendingReleaseResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The targeted release payload is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | One or more targeted request keys do not exist. |
| `409` | `*_conflict` | Current runtime ownership blocks the release. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="reclaim-stale-pending_shared_channel_sync"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/runtime/reclaim-stale-pending_shared_channel_sync`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/reclaim-stale-pending_shared_channel_sync</code>
  <span class="api-op-id">operationId: reclaimStalePendingSharedChannelSync</span>
</div>

Reclaim stale shared-channel sync pending ownership.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

`SocialSharedChannelSyncPendingStaleReclaimResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The reclaim request is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | The requested queue or ownership records do not exist. |
| `409` | `*_conflict` | Current runtime ownership blocks the reclaim. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="republish-pending_shared_channel_sync-targeted"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/runtime/republish-pending_shared_channel_sync-targeted`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/republish-pending_shared_channel_sync-targeted</code>
  <span class="api-op-id">operationId: republishPendingSharedChannelSyncTargeted</span>
</div>

Republish selected pending shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SocialSharedChannelSyncTargetedRepublishRequest" />

### Response `200`

`SocialSharedChannelSyncTargetedRepublishResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The targeted republish payload is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | One or more targeted request keys do not exist. |
| `409` | `*_conflict` | Current runtime ownership blocks the republish. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
<a id="takeover-pending_shared_channel_sync-targeted"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/runtime/takeover-pending_shared_channel_sync-targeted`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/runtime/takeover-pending_shared_channel_sync-targeted</code>
  <span class="api-op-id">operationId: takeoverPendingSharedChannelSyncTargeted</span>
</div>

Take over selected pending shared-channel sync entries.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.socialRuntime</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SocialSharedChannelSyncPendingTargetedTakeoverRequest" />

### Response `200`

`SocialSharedChannelSyncPendingTakeoverResponse` is currently modeled as an open-ended runtime
operation payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request` | The targeted takeover payload is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `control.write`. |
| `404` | `*_not_found` | One or more targeted request keys do not exist. |
| `409` | `*_conflict` | Current runtime ownership blocks the takeover. |
| `503` | `*_unavailable` | The social runtime queue or persistence dependency is unavailable. |

</section>
