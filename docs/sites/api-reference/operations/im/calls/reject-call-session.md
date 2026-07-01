# `POST /im/v3/api/calls/sessions/{rtcSessionId}/reject`

<p class="api-page-intro">
  Exact request and response contract for <strong>Calls</strong> in the <strong>IM Standard API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/im/calls"><code>Calls</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/im-api"><code>IM Standard API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, AppContext projection, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/calls/sessions/{rtcSessionId}/reject</code>
  <span class="api-op-id">operationId: calls.sessions.reject</span>
</div>

Rejects an incoming IM call signaling session.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.calls`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `call.reject` capability.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RtcSessionMutationResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `rtcSessionId` | `string` | Yes | IM call signaling session identifier. |

### Request Body

<ApiSchemaTable schema="UpdateRtcSessionRequest" />

### Response `200`

<ApiSchemaTable schema="RtcSessionMutationResponse" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The request payload or parameters are invalid. |
| `401` | `40101`, `40102` | AppContext projection is missing or invalid. |
| `403` | `40301`, `40302` | The caller is not allowed to mutate the target resource. |
| `404` | `40401` | The requested resource does not exist. |
| `409` | `40901` | Current runtime state blocks the mutation. |
| `503` | `50301` | A required subsystem or provider is unavailable. |

</section>
