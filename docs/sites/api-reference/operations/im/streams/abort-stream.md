# `POST /im/v3/api/streams/{streamId}/abort`

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
  <code>/im/v3/api/streams/{streamId}/abort</code>
  <span class="api-op-id">operationId: abortStream</span>
</div>

Aborts the stream lifecycle.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.generated.stream.abort(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `stream.abort` capability.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 StreamSession`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `stream_id` | `string` | Yes | Stream identifier. |

### Request Body

<ApiSchemaTable schema="AbortStreamRequest" />

### Response `200`

<ApiSchemaTable schema="StreamSession" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `404` | `stream_not_found` | The target stream does not exist. |
| `409` | `invalid_stream_state` | The stream lifecycle does not permit the requested transition. |

</section>
