# Rooms

<p class="api-page-intro">
  Room endpoints bind <code>live</code>, <code>chat</code>, or <code>game</code> business identities to
  group conversations. Message delivery, realtime fanout, and membership state still use the conversation
  kernel; rooms add orchestration, capacity policy, and self-serve enter/leave.
</p>

<div class="api-link-list">
  <a href="/api-reference/im/conversations"><code>Conversations</code> Group conversation creation and inbox flows</a>
  <a href="/api-reference/im/messages"><code>Messages</code> Post room messages on the bound <code>conversationId</code></a>
  <a href="/api-reference/im/session-and-realtime"><code>Realtime</code> Subscribe to <code>message.posted</code> on the bound conversation</a>
  <a href="/sdk/typescript-sdk"><code>@sdkwork/im-sdk</code> TypeScript exposes <code>sdk.rooms</code></a>
  <a href="/sdk/flutter-sdk"><code>im_sdk</code> Flutter consumers use the generated <code>client.chat.rooms</code> route group</a>
</div>

## Recommended SDK Mapping

- `sdk.rooms.create(...)` creates a room and binds it to a group conversation.
- `sdk.rooms.get(...)` returns active member count and capacity metadata.
- `sdk.rooms.enter(...)` self-joins the authenticated principal without admin invite.
- `sdk.rooms.leave(...)` removes the current principal from the room roster.

Example:

```ts
const room = await sdk.rooms.create({
  conversationId: 'c_live_001',
  roomId: 'room_live_001',
  roomKind: 'live',
});

await sdk.rooms.enter(room.roomId);

await sdk.conversations.postText(room.conversationId, 'hello room');

const view = await sdk.rooms.get(room.roomId);
console.log(view.activeMemberCount, view.maxMembers);

await sdk.rooms.leave(room.roomId);
```

<a id="create-room"></a>
<section class="api-op">

## `POST /im/v3/api/chat/rooms`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/chat/rooms</code>
  <span class="api-op-id">operationId: rooms.create</span>
</div>

Creates a live, chat, or game room bound to a group conversation.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.rooms.create(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 CreateConversationResult`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="CreateRoomRequest" />

### Response `200`

<ApiSchemaTable schema="CreateConversationResult" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>

<a id="get-room"></a>
<section class="api-op">

## `GET /im/v3/api/chat/rooms/{roomId}`

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

<a id="enter-room"></a>
<section class="api-op">

## `POST /im/v3/api/chat/rooms/{roomId}/enter`

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

<a id="leave-room"></a>
<section class="api-op">

## `POST /im/v3/api/chat/rooms/{roomId}/leave`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/chat/rooms/{roomId}/leave</code>
  <span class="api-op-id">operationId: rooms.leave</span>
</div>

Leaves the room as the authenticated principal.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.rooms.leave(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Active room member.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 EnterRoomResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `roomId` | `string` | Yes | Room to leave. |

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

<ApiSchemaTable schema="EnterRoomResponse" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested room does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
