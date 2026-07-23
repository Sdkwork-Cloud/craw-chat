# `POST /im/v3/api/chat/conversations/agent_handoffs`

<p class="api-page-intro">
  Exact request and response contract for <strong>Conversations and Handoff</strong> in the <strong>IM Standard API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/im/conversations"><code>Conversations and Handoff</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/im-api"><code>IM Standard API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, resolved request-context, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/chat/conversations/agent_handoffs</code>
  <span class="api-op-id">operationId: conversations.agentHandoffs.create</span>
</div>

Creates a handoff conversation and initializes handoff state.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
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
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller is not allowed to mutate the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the mutation. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
