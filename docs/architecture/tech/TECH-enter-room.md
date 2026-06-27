> Migrated from `docs/sites/api-reference/operations/im/rooms/enter-room.md` on 2026-06-24.
> Owner: SDKWork maintainers

<p class="api-page-intro">
  Exact request and response contract for <strong>Rooms</strong> in the <strong>IM Standard API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/im/rooms"><code>Rooms</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/im-api"><code>IM Standard API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, AppContext projection, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/chat/rooms/{roomId}/enter</code>
  <span class="api-op-id">operationId: rooms.enter</span>
</div>

Enters the room as the authenticated principal.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.rooms.enter(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; room capacity and enter policy are enforced.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 EnterRoomResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `roomId` | `string` | Yes | Room to enter. |

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

<ApiSchemaTable schema="EnterRoomResponse" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | Enter is blocked by room policy or capacity. |
| `404` | `*_not_found` | The requested room does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>

