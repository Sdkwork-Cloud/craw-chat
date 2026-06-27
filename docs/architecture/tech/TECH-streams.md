> Migrated from `docs/sites/api-reference/im/streams.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Streams

<p class="api-page-intro">
  Stream endpoints expose the transport used for long-running structured payload delivery,
  call-related streaming payloads, and app-business streaming workflows. The wire model follows the current
  `streaming-service`.
</p>

<div class="api-link-list">
  <a href="/api-reference/im/calls"><code>Calls</code> IM call lifecycle and signaling resources are documented separately</a>
  <a href="/sdk/app-sdk"><code>SDK</code> <code>@sdkwork/im-sdk</code> currently exposes stream routes through the generated transport boundary; Flutter consumers access the same contract through <code>im_sdk</code></a>
</div>

## Recommended SDK Mapping

Stream transport is currently generated-first in the TypeScript SDK:

- `sdk.generated.stream.open(...)`
- `sdk.generated.stream.listStreamFrames(...)`
- `sdk.generated.stream.appendStreamFrame(...)`
- `sdk.generated.stream.checkpoint(...)`
- `sdk.generated.stream.complete(...)`
- `sdk.generated.stream.abort(...)`

Example:

```ts
const stream = await sdk.generated.stream.open({
  streamId: 'stream-demo-1',
  streamType: 'custom.delta.text',
  scopeKind: 'conversation',
  scopeId: 'conversation-1',
  durabilityClass: 'durableSession',
  schemaRef: 'custom.delta.text.v1',
});

await sdk.generated.stream.appendStreamFrame(stream.streamId, {
  frameType: 'delta',
  encoding: 'utf-8',
  payload: 'hello world',
});

const frames = await sdk.generated.stream.listStreamFrames(stream.streamId);
console.log(frames.items.length);
```

<a id="open-stream"></a>
<section class="api-op">

## `POST /im/v3/api/streams`

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
<a id="append-stream-frame"></a>
<section class="api-op">

## `POST /im/v3/api/streams/{streamId}/frames`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/streams/{streamId}/frames</code>
  <span class="api-op-id">operationId: appendStreamFrame</span>
</div>

Appends a frame to an open stream.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.generated.stream.appendStreamFrame(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `stream.append` capability.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 StreamFrame`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `stream_id` | `string` | Yes | Stream identifier. |

### Request Body

<ApiSchemaTable schema="AppendStreamFrameRequest" />

### Response `200`

<ApiSchemaTable schema="StreamFrame" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="list-stream-frames"></a>
<section class="api-op">

## `GET /im/v3/api/streams/{streamId}/frames`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/im/v3/api/streams/{streamId}/frames</code>
  <span class="api-op-id">operationId: listStreamFrames</span>
</div>

Reads a paged window of frames for a stream.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.generated.stream.listStreamFrames(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation member or stream owner scope.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 StreamFrameWindow`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `stream_id` | `string` | Yes | Stream identifier. |

### Query Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `afterFrameSeq` | `uint64 \| null` | No | Return frames strictly after this sequence number. |
| `limit` | `uint64 \| null` | No | Window size. |

### Response `200`

<ApiSchemaTable schema="StreamFrameWindow" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | The caller is not allowed to access the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the read or handshake flow. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="checkpoint-stream"></a>
<section class="api-op">

## `POST /im/v3/api/streams/{streamId}/checkpoint`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/streams/{streamId}/checkpoint</code>
  <span class="api-op-id">operationId: checkpointStream</span>
</div>

Updates the consumer checkpoint for the stream.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.generated.stream.checkpoint(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `stream.checkpoint` capability.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 StreamSession`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `stream_id` | `string` | Yes | Stream identifier. |

### Request Body

<ApiSchemaTable schema="CheckpointStreamRequest" />

### Response `200`

<ApiSchemaTable schema="StreamSession" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="complete-stream"></a>
<section class="api-op">

## `POST /im/v3/api/streams/{streamId}/complete`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/streams/{streamId}/complete</code>
  <span class="api-op-id">operationId: completeStream</span>
</div>

Marks the stream as completed.


<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.generated.stream.complete(...)`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `stream.complete` capability.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 StreamSession`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `stream_id` | `string` | Yes | Stream identifier. |

### Request Body

<ApiSchemaTable schema="CompleteStreamRequest" />

### Response `200`

<ApiSchemaTable schema="StreamSession" />


### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>
<a id="abort-stream"></a>
<section class="api-op">

## `POST /im/v3/api/streams/{streamId}/abort`

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

