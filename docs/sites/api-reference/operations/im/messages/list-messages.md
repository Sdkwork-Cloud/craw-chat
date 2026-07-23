# `GET /im/v3/api/chat/conversations/{conversationId}/messages`

<p class="api-page-intro">
  Exact request and response contract for <strong>Messages</strong> in the <strong>IM Standard API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/im/messages"><code>Messages</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/im-api"><code>IM Standard API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, resolved request-context, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

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
