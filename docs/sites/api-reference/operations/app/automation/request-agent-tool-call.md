# `POST /app/v3/api/automation/agent_tool_calls`

<p class="api-page-intro">
  Exact request and response contract for <strong>Automation</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/automation"><code>Automation</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, resolved request-context, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/app/v3/api/automation/agent_tool_calls</code>
  <span class="api-op-id">operationId: automation.agentToolCalls.create</span>
</div>

Requests a tool call as part of an automation execution.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.automation.agentToolCalls.create(body)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`automation.execute`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201 AgentToolCall in data.item`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="RequestAgentToolCallRequest" />

### Response `201`

<ApiSchemaTable schema="AgentToolCall" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The automation execution request is invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `automation.execute`. |
| `409` | `40901` | The execution id conflicts with an existing request. |
| `503` | `50301` | Automation persistence is unavailable. |

</section>
