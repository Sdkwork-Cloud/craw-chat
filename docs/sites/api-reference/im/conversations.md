# Conversations and Handoff

<p class="api-page-intro">
  Conversation endpoints expose the unified inbox/list read, conversation creation, agent-dialog
  creation, system channels, and the full agent-handoff state machine.
</p>

<div class="api-link-list">
  <a href="/api-reference/im/membership-and-read-state"><code>Membership</code> Roster mutations and read cursors are documented separately</a>
  <a href="/api-reference/im/messages"><code>Messages</code> Message history reads and message mutation flows live on their own page</a>
  <a href="/sdk/app-sdk"><code>SDK</code> <code>@sdkwork/im-sdk</code> and <code>im_sdk</code> map these routes into conversation helpers such as <code>sdk.conversations</code></a>
</div>

## Recommended SDK Mapping

For the TypeScript app SDK, creation and handoff routes map into `sdk.conversations`:

- `sdk.conversations.create(...)`
- `sdk.conversations.createAgentDialog(...)`
- `sdk.conversations.createAgentHandoff(...)`
- `sdk.conversations.createSystemChannel(...)`
- `sdk.conversations.get(...)`
- `sdk.conversations.getAgentHandoffState(...)`
- `sdk.conversations.acceptAgentHandoff(...)`
- `sdk.conversations.resolveAgentHandoff(...)`
- `sdk.conversations.closeAgentHandoff(...)`

The sibling pages for membership/read state and messages are still part of the same TypeScript
module surface:

- `sdk.conversations.listMembers(...)`
- `sdk.conversations.addMember(...)`
- `sdk.conversations.removeMember(...)`
- `sdk.conversations.transferOwner(...)`
- `sdk.conversations.changeMemberRole(...)`
- `sdk.conversations.leave(...)`
- `sdk.conversations.getReadCursor(...)`
- `sdk.conversations.updateReadCursor(...)`
- `sdk.conversations.listMessages(...)`
- `sdk.conversations.postMessage(...)`
- `sdk.conversations.postText(...)`
- `sdk.conversations.publishSystemMessage(...)`
- `sdk.conversations.publishSystemText(...)`

Inbox is exposed through the TypeScript semantic module and generated transport boundary as
`sdk.conversations.list()` and `sdk.transport.chat.inbox.list()`. This is the only unified
conversation list for direct, group, system, handoff, and agent-dialog conversations.
`POST /im/v3/api/chat/conversations/agent_dialogs` is a create/idempotency command only; clients
must not use it as a list, retrieve, or reuse endpoint.

<a id="inbox-list"></a>
<section class="api-op">

## `GET /im/v3/api/chat/inbox`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/im/v3/api/chat/inbox</code>
  <span class="api-op-id">operationId: inbox.list</span>
</div>

Returns the cursor-paginated unified conversation list for the current principal. The list is served
by `projection-service` from the maintained inbox projection and includes ordinary human chats,
groups, system conversations, handoffs, and agent dialogs visible to the caller.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.transport.chat.inbox.list()`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 SdkWorkPageData<ConversationInboxEntry>`</span></div>
</div>

### Response `200`

`SdkWorkApiResponse.data.items` contains `ConversationInboxEntry` records and
`data.pageInfo.mode` is `cursor`.

<ApiSchemaTable schema="ConversationInboxEntry" />
<ApiSchemaTable schema="PageInfo" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | AppContext projection is missing or invalid. |
| `403` | `40301` | The caller is not allowed to access the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the read or handshake flow. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
<a id="create-conversation"></a>
<section class="api-op">

## `POST /im/v3/api/chat/conversations`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/chat/conversations</code>
  <span class="api-op-id">operationId: createConversation</span>
</div>

Creates a regular conversation.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.conversations`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201 CreateConversationResult in data.item`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="CreateConversationRequest" />

### Response `201`

<ApiSchemaTable schema="CreateConversationResult" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The request payload or parameters are invalid. |
| `401` | `40101` | AppContext projection is missing or invalid. |
| `403` | `40301` | The caller is not allowed to mutate the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the mutation. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
<a id="create-agent-dialog"></a>
<section class="api-op">

## `POST /im/v3/api/chat/conversations/agent_dialogs`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/chat/conversations/agent_dialogs</code>
  <span class="api-op-id">operationId: createAgentDialog</span>
</div>

Creates a one-to-one conversation with a specific agent. Before calling this command, app clients
should use `sdk.conversations.list()` / `GET /im/v3/api/chat/inbox` to find and reuse an existing
agent-dialog conversation. Duplicate creates with the same idempotency key replay the original
result; conflicting reuse of an idempotency key returns `40901`.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.conversations`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201 CreateConversationResult in data.item`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="CreateAgentDialogRequest" />

### Response `201`

<ApiSchemaTable schema="CreateConversationResult" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The request payload or parameters are invalid. |
| `401` | `40101` | AppContext projection is missing or invalid. |
| `403` | `40301` | The caller is not allowed to mutate the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the mutation. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
<a id="create-agent-handoff"></a>
<section class="api-op">

## `POST /im/v3/api/chat/conversations/agent_handoffs`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/chat/conversations/agent_handoffs</code>
  <span class="api-op-id">operationId: createAgentHandoff</span>
</div>

Creates a handoff conversation and initializes handoff state.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.conversations`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201 CreateConversationResult in data.item`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="CreateAgentHandoffRequest" />

### Response `201`

<ApiSchemaTable schema="CreateConversationResult" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The request payload or parameters are invalid. |
| `401` | `40101` | AppContext projection is missing or invalid. |
| `403` | `40301` | The caller is not allowed to mutate the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the mutation. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
<a id="create-system-channel"></a>
<section class="api-op">

## `POST /im/v3/api/chat/conversations/system_channels`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/chat/conversations/system_channels</code>
  <span class="api-op-id">operationId: createSystemChannel</span>
</div>

Creates a system channel for the specified subscriber principal.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.conversations`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201 CreateConversationResult in data.item`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="CreateSystemChannelRequest" />

### Response `201`

<ApiSchemaTable schema="CreateConversationResult" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The request payload or parameters are invalid. |
| `401` | `40101` | AppContext projection is missing or invalid. |
| `403` | `40301` | The caller is not allowed to mutate the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the mutation. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
<a id="get-conversation-summary"></a>
<section class="api-op">

## `GET /im/v3/api/chat/conversations/{conversationId}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/im/v3/api/chat/conversations/{conversationId}</code>
  <span class="api-op-id">operationId: getConversationSummary</span>
</div>

Reads the conversation summary projection.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.conversations`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Active conversation member.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ConversationSummaryView`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `conversation_id` | `string` | Yes | Conversation identifier. |

### Response `200`

<ApiSchemaTable schema="ConversationSummaryView" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | AppContext projection is missing or invalid. |
| `403` | `40301` | The caller is not allowed to access the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the read or handshake flow. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>

<a id="get-agent-handoff-state"></a>
<section class="api-op">

## `GET /im/v3/api/chat/conversations/{conversationId}/agent_handoff`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/im/v3/api/chat/conversations/{conversationId}/agent_handoff</code>
  <span class="api-op-id">operationId: getAgentHandoffState</span>
</div>

Reads the current handoff state for the conversation.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.conversations`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Active conversation member.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 AgentHandoffStateView`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `conversation_id` | `string` | Yes | Conversation identifier. |

### Response `200`

<ApiSchemaTable schema="AgentHandoffStateView" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | AppContext projection is missing or invalid. |
| `403` | `40301` | The caller is not allowed to access the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the read or handshake flow. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
<a id="accept-agent-handoff"></a>
<section class="api-op">

## `POST /im/v3/api/chat/conversations/{conversationId}/agent_handoff/accept`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/chat/conversations/{conversationId}/agent_handoff/accept</code>
  <span class="api-op-id">operationId: acceptAgentHandoff</span>
</div>

Accepts the handoff from the target side. No JSON request body is required.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.conversations`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Active conversation member.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 AgentHandoffStateView`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `conversation_id` | `string` | Yes | Conversation identifier. |

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

<ApiSchemaTable schema="AgentHandoffStateView" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The request payload or parameters are invalid. |
| `401` | `40101` | AppContext projection is missing or invalid. |
| `403` | `40301` | The caller is not allowed to mutate the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the mutation. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
<a id="resolve-agent-handoff"></a>
<section class="api-op">

## `POST /im/v3/api/chat/conversations/{conversationId}/agent_handoff/resolve`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/chat/conversations/{conversationId}/agent_handoff/resolve</code>
  <span class="api-op-id">operationId: resolveAgentHandoff</span>
</div>

Marks the handoff as resolved. No JSON request body is required.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.conversations`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Active conversation member.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 AgentHandoffStateView`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `conversation_id` | `string` | Yes | Conversation identifier. |

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

<ApiSchemaTable schema="AgentHandoffStateView" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The request payload or parameters are invalid. |
| `401` | `40101` | AppContext projection is missing or invalid. |
| `403` | `40301` | The caller is not allowed to mutate the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the mutation. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
<a id="close-agent-handoff"></a>
<section class="api-op">

## `POST /im/v3/api/chat/conversations/{conversationId}/agent_handoff/close`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/chat/conversations/{conversationId}/agent_handoff/close</code>
  <span class="api-op-id">operationId: closeAgentHandoff</span>
</div>

Closes the handoff. No JSON request body is required.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.conversations`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Active conversation member.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 AgentHandoffStateView`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `conversation_id` | `string` | Yes | Conversation identifier. |

### Request Body

None. This operation does not accept a JSON request body.

### Response `200`

<ApiSchemaTable schema="AgentHandoffStateView" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The request payload or parameters are invalid. |
| `401` | `40101` | AppContext projection is missing or invalid. |
| `403` | `40301` | The caller is not allowed to mutate the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the mutation. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
