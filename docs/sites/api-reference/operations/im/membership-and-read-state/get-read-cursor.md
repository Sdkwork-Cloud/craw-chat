# `GET /im/v3/api/chat/conversations/{conversationId}/read_cursor`

<p class="api-page-intro">
  Exact request and response contract for <strong>Membership and Read State</strong> in the <strong>IM Standard API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/im/membership-and-read-state"><code>Membership and Read State</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/im-api"><code>IM Standard API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, AppContext projection, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/im/v3/api/chat/conversations/{conversationId}/read_cursor</code>
  <span class="api-op-id">operationId: getReadCursor</span>
</div>

Returns the read cursor for the current principal in the target conversation.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.conversations`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Active conversation member.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 ConversationReadCursorView`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `conversation_id` | `string` | Yes | Conversation identifier. |

### Response `200`

<ApiSchemaTable schema="ConversationReadCursorView" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | AppContext projection is missing or invalid. |
| `403` | `40301` | The caller is not allowed to access the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the read or handshake flow. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
