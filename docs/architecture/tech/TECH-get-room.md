> Migrated from `docs/sites/api-reference/operations/im/rooms/get-room.md` on 2026-06-24.
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
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/im/v3/api/chat/rooms/{roomId}</code>
  <span class="api-op-id">operationId: rooms.get</span>
</div>

Returns room metadata and the active member count.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.rooms.get(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RoomView`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `roomId` | `string` | Yes | Stable room identifier bound through conversation business metadata. |

### Response `200`

<ApiSchemaTable schema="RoomView" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | The caller is not allowed to access the target resource. |
| `404` | `*_not_found` | The requested room does not exist. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>

