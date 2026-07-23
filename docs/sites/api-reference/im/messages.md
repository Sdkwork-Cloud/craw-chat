# Messages

<p class="api-page-intro">
  Message endpoints expose paged message history reads, regular and system-channel submission, and message
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
  <span class="api-op-id">operationId: conversations.messages.create</span>
</div>

Posts a regular conversation message.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.messages`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation-bound write access.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201 PostMessageResult in data.item`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `conversation_id` | `string` | Yes | Conversation identifier. |

### Request Body

<ApiSchemaTable schema="PostMessageRequest" />

### Response `201`

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
| `400` | `40001` | The request payload or parameters are invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller is not allowed to mutate the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the mutation. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
<a id="list-messages"></a>
<section class="api-op">

## `GET /im/v3/api/chat/conversations/{conversationId}/messages`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/im/v3/api/chat/conversations/{conversationId}/messages</code>
  <span class="api-op-id">operationId: conversations.messages.list</span>
</div>

Lists a durable, normalized Message history window owned by
`sdkwork-comms-conversation-service`. Inbox, Conversation summaries, read cursors, Message search,
pins, visibility, and interaction summaries are read from their typed normalized repositories.

Production deployments read from `message_store.read_window` on `PostgresMessageStore` with
`message_seq > afterSeq`, ascending sequence order, and `page_size + 1` fetch-ahead. Startup is
fail-closed outside dev/test when Postgres is not configured, so a `50301` on this route means the
conversation runtime's required durable dependency is unavailable, not that the GET API is missing.
The dev/test in-memory message-log fallback is only for explicit in-memory runtime tests and must
not be treated as the production history store.

The handler never performs unbounded in-process pagination for this path. Normal app clients request
one page, render it, and call `sdk.conversations.listMessages(...)` again with the returned cursor
only when the user explicitly loads more history.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.messages`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Active conversation member.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ConversationMessageListResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `conversation_id` | `string` | Yes | Conversation identifier. |

### Query Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `afterSeq` | `integer` | No | Return messages with `messageSeq` greater than this value. Defaults to `0`. |
| `page_size` | `integer` | No | Page size. Defaults to `20`; maximum is `200`. |

### Response `200`

<ApiSchemaTable schema="ConversationMessageListResponse" />

The response uses the SDKWork list envelope: `data.items`, `data.pageInfo`, and
`data.highWatermark`. `data.pageInfo.mode` is `cursor`; callers continue by passing the last
returned `messageSeq` as `afterSeq`.

`ConversationMessageEntry` is the message-history DTO for this route. It contains message identity,
sequence, sender, message body, summary, type, delivery mode, and timestamps. It intentionally does
not inline `reactionCounts` or `pin` state; clients must not issue hidden per-message
`interaction_summary` requests while loading history. Reactions, pins, and other interaction state
remain explicit typed Message operations.


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller is not allowed to access the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the read or handshake flow. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
<a id="publish-system-channel-message"></a>
<section class="api-op">

## `POST /im/v3/api/chat/conversations/{conversationId}/system_channel/publish`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/chat/conversations/{conversationId}/system_channel/publish</code>
  <span class="api-op-id">operationId: conversations.systemChannel.publish</span>
</div>

Publishes a system message to the conversation's system channel.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
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
| `400` | `40001` | The request payload or parameters are invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller is not allowed to mutate the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the mutation. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
<a id="edit-message"></a>
<section class="api-op">

## `POST /im/v3/api/chat/messages/{messageId}/edit`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/chat/messages/{messageId}/edit</code>
  <span class="api-op-id">operationId: messages.edit</span>
</div>

Edits a previously posted message.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
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
| `400` | `40001` | The request payload or parameters are invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller is not allowed to mutate the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the mutation. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
<a id="recall-message"></a>
<section class="api-op">

## `POST /im/v3/api/chat/messages/{messageId}/recall`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/chat/messages/{messageId}/recall</code>
  <span class="api-op-id">operationId: messages.recall</span>
</div>

Recalls a message. This operation does not require a JSON request body.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
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
| `400` | `40001` | The request payload or parameters are invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller is not allowed to mutate the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the mutation. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
