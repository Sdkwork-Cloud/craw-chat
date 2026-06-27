# `POST /app/v3/api/automation/agent_responses`

<p class="api-page-intro">
  Exact request and response contract for <strong>Automation</strong> in the <strong>App API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/automation"><code>Automation</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/app-api"><code>App API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, AppContext projection, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/app/v3/api/automation/agent_responses</code>
  <span class="api-op-id">operationId: startAgentResponse</span>
</div>

Starts an agent response stream for an automation execution.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.automation.agentResponses.create(body)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`automation.execute`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 StreamSession`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="StartAgentResponseRequest" />

### Response `200`

<ApiSchemaTable schema="StreamSession" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The automation execution request is invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `permission_denied` | The caller lacks `automation.execute`. |
| `409` | `automation_execution_conflict` | The execution id conflicts with an existing request. |
| `503` | `automation_store_unavailable`, `journal_unavailable` | Automation persistence is unavailable. |

</section>
