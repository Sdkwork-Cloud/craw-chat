# Messages

<p class="api-page-intro">
  Message endpoints expose timeline reads, regular and system-channel submission, and message
  mutations such as edit and recall. The recommended TypeScript SDK surface for these routes is the
  root <code>ImSdkClient</code> message path, not raw route-group calls.
</p>

<div class="api-link-list">
  <a href="/api-reference/im/conversations"><code>Conversations</code> Conversation creation, inbox, and handoff flows are documented separately</a>
  <a href="/api-reference/im/membership-and-read-state"><code>Membership</code> Roster and read-cursor updates live on a separate page</a>
  <a href="/sdk/typescript-sdk"><code>SDK</code> <code>@sdkwork/im-sdk</code> and Flutter package <code>im_sdk</code> expose the recommended message-building and delivery flows for app consumers</a>
</div>

## Recommended SDK Mapping

Use the semantic message layer in this order:

1. create a message with `sdk.createTextMessage(...)`, `createImageMessage(...)`,
   `createCustomMessage(...)`, `createAiTextMessage(...)`, `createAgentHandoffMessage(...)`, and
   the other `createXxxMessage(...)` helpers
2. deliver it with `sdk.send(message)`
3. use `sdkwork-drive` for file lifecycle work, then place the returned `DriveReference` on
   `ContentPart.drive`
4. use `sdk.editMessage(...)`, `sdk.editTextMessage(...)`, and `sdk.recallMessage(...)` for
   mutations
5. use `sdk.decodeMessage(...)` when you need to normalize stored or inbound message bodies

The same functionality also remains available on `sdk.messages` when you want a namespaced module
surface.

Example:

```ts
const message = sdk.createTextMessage({
  conversationId: 'conversation-1',
  text: 'hello world',
  summary: 'Greeting',
});

await sdk.send(message);
```

Drive-backed media send:

```ts
const drive = {
  driveUri: 'drive://spaces/space_app_upload_demo/nodes/node_storefront_png',
  spaceId: 'space_app_upload_demo',
  nodeId: 'node_storefront_png',
  nodeVersion: '1',
};

const image = sdk.createImageMessage({
  conversationId: 'conversation-1',
  drive,
  resource: {
    id: drive.nodeId,
    kind: 'image',
    source: 'provider_asset',
    uri: drive.driveUri,
    fileName: 'storefront.png',
    mimeType: 'image/png',
    sizeBytes: String(file.size),
  },
  mediaRole: 'attachment',
  text: 'Latest storefront concept',
  summary: 'Storefront concept',
});

await sdk.send(image);
```

The same message body carries `ContentPart.drive` as the `DriveReference` and `ContentPart.resource`
as the standardized `MediaResource` snapshot. `drive://spaces/{spaceId}/nodes/{nodeId}` is the
canonical Drive URI shape.

Standard rich builders include `createLocationMessage(...)`, `createLinkMessage(...)`,
`createCardMessage(...)`, `createMusicMessage(...)`, `createContactMessage(...)`,
`createStickerMessage(...)`, `createVoiceMessage(...)`, `createAiImageGenerationMessage(...)`,
`createAiVideoGenerationMessage(...)`, `createAgentMessage(...)`, `createAgentStateMessage(...)`,
`createAgentHandoffMessage(...)`, `createToolResultMessage(...)`, and
`createWorkflowEventMessage(...)`.

If you want route-aligned access instead of builder-first ergonomics, the same transport-facing
operations are also available on `sdk.conversations.listMessages(...)`,
`sdk.conversations.postMessage(...)`, `sdk.conversations.postText(...)`,
`sdk.conversations.publishSystemMessage(...)`, and `sdk.conversations.publishSystemText(...)`.

<a id="post-message"></a>
<section class="api-op">

## `POST /im/v3/api/chat/conversations/{conversationId}/messages`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/chat/conversations/{conversationId}/messages</code>
  <span class="api-op-id">operationId: postMessage</span>
</div>

Posts a regular conversation message.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.messages`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation-bound write access.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PostMessageResult`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `conversation_id` | `string` | Yes | Conversation identifier. |

### Request Body

<ApiSchemaTable schema="PostMessageRequest" />

### Response `200`

<ApiSchemaTable schema="PostMessageResult" />

### Example Request

```json
{
  "clientMsgId": "msg-client-001",
  "summary": "Greeting",
  "text": "hello world"
}
```


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
<a id="get-timeline"></a>
<section class="api-op">

## `GET /im/v3/api/chat/conversations/{conversationId}/messages`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/im/v3/api/chat/conversations/{conversationId}/messages</code>
  <span class="api-op-id">operationId: getTimeline</span>
</div>

Returns the projection-backed message timeline for the conversation.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.messages`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Active conversation member.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 TimelineListResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `conversation_id` | `string` | Yes | Conversation identifier. |

### Response `200`

<ApiSchemaTable schema="TimelineListResponse" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | The caller is not allowed to access the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the read or handshake flow. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="publish-system-channel-message"></a>
<section class="api-op">

## `POST /im/v3/api/chat/conversations/{conversationId}/system_channel/publish`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/chat/conversations/{conversationId}/system_channel/publish</code>
  <span class="api-op-id">operationId: publishSystemChannelMessage</span>
</div>

Publishes a system message to the conversation's system channel.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.messages`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation-bound write access.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 PostMessageResult`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `conversation_id` | `string` | Yes | Conversation identifier. |

### Request Body

Uses the same request schema as regular message submission.

<ApiSchemaTable schema="PostMessageRequest" />

### Response `200`

<ApiSchemaTable schema="PostMessageResult" />


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
<a id="edit-message"></a>
<section class="api-op">

## `POST /im/v3/api/chat/messages/{messageId}/edit`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/chat/messages/{messageId}/edit</code>
  <span class="api-op-id">operationId: editMessage</span>
</div>

Edits a previously posted message.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.messages`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation-bound write access.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 MessageMutationResult`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `message_id` | `string` | Yes | Message identifier. |

### Request Body

<ApiSchemaTable schema="EditMessageRequest" />

### Response `200`

<ApiSchemaTable schema="MessageMutationResult" />


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
<a id="recall-message"></a>
<section class="api-op">

## `POST /im/v3/api/chat/messages/{messageId}/recall`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/chat/messages/{messageId}/recall</code>
  <span class="api-op-id">operationId: recallMessage</span>
</div>

Recalls a message. This operation does not require a JSON request body.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.messages`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation-bound write access.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 MessageMutationResult`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `message_id` | `string` | Yes | Message identifier. |

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

<ApiSchemaTable schema="MessageMutationResult" />


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
