# `GET /app/v3/api/automation/executions/{executionId}`

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
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/app/v3/api/automation/executions/{executionId}</code>
  <span class="api-op-id">operationId: automation.executions.retrieve</span>
</div>

Reads an automation execution by identifier.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.automation.executions.retrieve(executionId)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`automation.read`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 AutomationExecution`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `execution_id` | `string` | Yes | Automation execution identifier. |

### Response `200`

<ApiSchemaTable schema="AutomationExecution" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `automation.read`. |
| `404` | `40401` | The requested automation execution does not exist. |
| `503` | `50301` | Automation persistence is unavailable. |

</section>
