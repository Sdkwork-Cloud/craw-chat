# `POST /im/v3/api/streams`

<p class="api-page-intro">
  Exact request and response contract for <strong>Streams</strong> in the <strong>IM Standard API</strong>.
</p>

<div class="api-link-list">
  <a href="/api-reference/im/streams"><code>Streams</code> Return to the group page for workflow context and related operations</a>
  <a href="/api-reference/im-api"><code>IM Standard API</code> Return to the domain overview</a>
  <a href="/api-reference/auth-and-errors"><code>Auth</code> SDKWork dual-token, AppContext projection, and error-envelope rules</a>
</div>

<section class="api-op api-op-single">

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/streams</code>
  <span class="api-op-id">operationId: openStream</span>
</div>

Opens a new stream session.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.generated.stream.open(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `stream.open` capability.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 StreamSession`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="OpenStreamRequest" />

### Response `200`

<ApiSchemaTable schema="StreamSession" />

### Example Request

```json
{
  "streamId": "stream_demo_001",
  "streamType": "custom.delta.text",
  "scopeKind": "conversation",
  "scopeId": "conv_demo_001",
  "durabilityClass": "durableSession",
  "schemaRef": "custom.delta.text.v1"
}
```


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error`, `aiot_stream_scope_unsupported` | The request payload or parameters are invalid, or the stream scope is owned by sdkwork-aiot. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
