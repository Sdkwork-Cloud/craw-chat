# Automation

<p class="api-page-intro">
  Automation endpoints request asynchronous executions and inspect execution state after the job has
  been accepted by the runtime.
</p>

<div class="api-link-list">
  <a href="/api-reference/app/notifications"><code>Notifications</code> Notification work items are documented separately</a>
  <a href="/api-reference/backend/audit"><code>Backend Audit</code> Audit evidence and export flows are documented separately</a>
  <a href="/sdk/app-sdk"><code>App SDK</code> Use <code>sdkwork-im-app-sdk</code> and <code>SdkworkAppClient.automation</code></a>
</div>

<a id="request-automation-execution"></a>
<section class="api-op">

## `POST /app/v3/api/automation/executions`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/app/v3/api/automation/executions</code>
  <span class="api-op-id">operationId: automation.executions.create</span>
</div>

Requests a new automation execution.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.automation.executions.create(body)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`automation.execute`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201 AutomationExecution in data.item`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="RequestAutomationExecution" />

### Response `201`

<ApiSchemaTable schema="AutomationExecution" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The automation execution request is invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `automation.execute`. |
| `409` | `40901` | The execution id conflicts with an existing request. |
| `503` | `50301` | Automation persistence is unavailable. |

</section>

<a id="start-agent-response"></a>
<section class="api-op">

## `POST /app/v3/api/automation/agent_responses`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/app/v3/api/automation/agent_responses</code>
  <span class="api-op-id">operationId: automation.agentResponses.create</span>
</div>

Starts an agent response stream for an automation execution.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.automation.agentResponses.create(body)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`automation.execute`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201 StreamSession in data.item`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="StartAgentResponseRequest" />

### Response `201`

<ApiSchemaTable schema="StreamSession" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The automation execution request is invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `automation.execute`. |
| `409` | `40901` | The execution id conflicts with an existing request. |
| `503` | `50301` | Automation persistence is unavailable. |

</section>

<a id="append-agent-response-frame"></a>
<section class="api-op">

## `POST /app/v3/api/automation/agent_responses/{streamId}/frames`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/app/v3/api/automation/agent_responses/{streamId}/frames</code>
  <span class="api-op-id">operationId: automation.agentResponses.frames.create</span>
</div>

Appends a frame to an active agent response stream.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.automation.agentResponses.frames.create(streamId, body)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`automation.execute`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201 StreamFrame in data.item`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `streamId` | `string` | Yes | Agent response stream identifier. |

### Request Body

<ApiSchemaTable schema="AppendAgentResponseDeltaRequest" />

### Response `201`

<ApiSchemaTable schema="StreamFrame" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The automation execution request is invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `automation.execute`. |
| `409` | `40901` | The execution id conflicts with an existing request. |
| `503` | `50301` | Automation persistence is unavailable. |

</section>

<a id="complete-agent-response"></a>
<section class="api-op">

## `POST /app/v3/api/automation/agent_responses/{streamId}/complete`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/app/v3/api/automation/agent_responses/{streamId}/complete</code>
  <span class="api-op-id">operationId: automation.agentResponses.complete</span>
</div>

Completes an agent response stream and returns the final stream session state.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.automation.agentResponses.complete(streamId, body)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`automation.execute`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 StreamSession`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `streamId` | `string` | Yes | Agent response stream identifier. |

### Request Body

<ApiSchemaTable schema="CompleteAgentResponseRequest" />

### Response `200`

<ApiSchemaTable schema="StreamSession" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The automation execution request is invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `automation.execute`. |
| `409` | `40901` | The execution id conflicts with an existing request. |
| `503` | `50301` | Automation persistence is unavailable. |

</section>

<a id="request-agent-tool-call"></a>
<section class="api-op">

## `POST /app/v3/api/automation/agent_tool_calls`

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

<a id="complete-agent-tool-call"></a>
<section class="api-op">

## `POST /app/v3/api/automation/executions/{executionId}/agent_tool_calls/{toolCallId}/complete`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/app/v3/api/automation/executions/{executionId}/agent_tool_calls/{toolCallId}/complete</code>
  <span class="api-op-id">operationId: automation.agentToolCalls.complete</span>
</div>

Completes a pending agent tool call for a specific automation execution.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-app-sdk` / `client.automation.agentToolCalls.complete(executionId, toolCallId, body)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`automation.execute`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 AgentToolCall`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `executionId` | `string` | Yes | Automation execution identifier. |
| `toolCallId` | `string` | Yes | Agent tool-call identifier. |

### Request Body

<ApiSchemaTable schema="CompleteAgentToolCallRequest" />

### Response `200`

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
<a id="get-automation-execution"></a>
<section class="api-op">

## `GET /app/v3/api/automation/executions/{executionId}`

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
