# Calls

<p class="api-page-intro">
  Calls endpoints manage IM-owned call signaling sessions, invite lifecycle, signal delivery,
  and RTC media participant credential handoff. RTC provider media/runtime remains in
  <code>@sdkwork/rtc-sdk</code>; signaling is owned by IM.
</p>

<div class="api-link-list">
  <a href="/api-reference/im/session-and-realtime"><code>Realtime</code> WebSocket subscription and event acknowledgement</a>
  <a href="/sdk/rtc-sdk"><code>RTC SDK</code> Provider media/runtime bridge without signaling ownership</a>
  <a href="/sdk/typescript-sdk"><code>@sdkwork/im-sdk</code> TypeScript exposes <code>sdk.calls</code></a>
  <a href="/sdk/flutter-sdk"><code>im_sdk</code> Flutter consumers use the generated <code>client.calls</code> route group</a>
</div>

## Recommended SDK Mapping

- `sdk.calls.start(...)` creates an IM call signaling session.
- `sdk.calls.invite(...)`, `sdk.calls.accept(...)`, `sdk.calls.reject(...)`, and `sdk.calls.end(...)` drive the call lifecycle.
- `sdk.calls.sendSignal(...)` posts SDP/ICE or provider-specific signaling payloads through IM.
- `sdk.calls.watchIncoming(...)` watches incoming call invites over the IM realtime WebSocket.
- `sdk.calls.issueParticipantCredential(...)` issues RTC media join credentials after IM authorization.

Example:

```ts
const session = await sdk.calls.start({
  rtcSessionId: 'call-1',
  conversationId: 'conversation-1',
  rtcMode: 'video',
});

await sdk.calls.invite(session.rtcSessionId, {
  signalingStreamId: 'call-signal-1',
});

await sdk.calls.sendSignal(session.rtcSessionId, {
  signalingStreamId: 'call-signal-1',
  signalType: 'rtc.offer',
  payload: JSON.stringify({ sdp: 'v=0...' }),
});

const incoming = await sdk.calls.watchIncoming({
  conversationIds: ['conversation-1'],
});

if (incoming) {
  await sdk.calls.accept(incoming.rtcSessionId);
}
```

<a id="create-call-session"></a>
<section class="api-op">

## `POST /im/v3/api/calls/sessions`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/calls/sessions</code>
  <span class="api-op-id">operationId: calls.sessions.create</span>
</div>

Creates an IM-owned call signaling session.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.calls`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `call.create` capability when the session is bound to a conversation.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RtcSessionMutationResponse`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="CreateRtcSessionRequest" />

### Response `200`

<ApiSchemaTable schema="RtcSessionMutationResponse" />

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

<a id="retrieve-call-session"></a>
<section class="api-op">

## `GET /im/v3/api/calls/sessions/{rtcSessionId}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/im/v3/api/calls/sessions/{rtcSessionId}</code>
  <span class="api-op-id">operationId: calls.sessions.retrieve</span>
</div>

Retrieves current IM call signaling session state for reconnect and backfill.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.calls`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Authenticated principal; conversation call scope is validated by IM.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RtcSession`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `rtcSessionId` | `string` | Yes | IM call signaling session identifier. |

### Response `200`

<ApiSchemaTable schema="RtcSession" />

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | The caller is not allowed to access the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the read or handshake flow. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>

<a id="invite-call-session"></a>
<section class="api-op">

## `POST /im/v3/api/calls/sessions/{rtcSessionId}/invite`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/calls/sessions/{rtcSessionId}/invite</code>
  <span class="api-op-id">operationId: calls.sessions.invite</span>
</div>

Sends an IM call invite and publishes the corresponding realtime signal message.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.calls`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `call.invite` capability.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RtcSessionMutationResponse`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `rtcSessionId` | `string` | Yes | IM call signaling session identifier. |

### Request Body

<ApiSchemaTable schema="InviteRtcSessionRequest" />

### Response `200`

<ApiSchemaTable schema="RtcSessionMutationResponse" />

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

<a id="accept-call-session"></a>
<section class="api-op">

## `POST /im/v3/api/calls/sessions/{rtcSessionId}/accept`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/calls/sessions/{rtcSessionId}/accept</code>
  <span class="api-op-id">operationId: calls.sessions.accept</span>
</div>

Accepts an incoming IM call signaling session.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.calls`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `call.accept` capability.</span></div>
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
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>

<a id="reject-call-session"></a>
<section class="api-op">

## `POST /im/v3/api/calls/sessions/{rtcSessionId}/reject`

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
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>

<a id="end-call-session"></a>
<section class="api-op">

## `POST /im/v3/api/calls/sessions/{rtcSessionId}/end`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/calls/sessions/{rtcSessionId}/end</code>
  <span class="api-op-id">operationId: calls.sessions.end</span>
</div>

Ends an active IM call signaling session.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.calls`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `call.end` capability.</span></div>
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
| `400` | `invalid_request`, `validation_error` | The request payload or parameters are invalid. |
| `401` | `app_context_missing`, `app_context_invalid` | AppContext projection is missing or invalid. |
| `403` | `conversation_permission_denied`, `permission_denied` | The caller is not allowed to mutate the target resource. |
| `404` | `*_not_found` | The requested resource does not exist. |
| `409` | `reconnect_required`, `disconnect_fence_conflict`, `conflict` | Current runtime state blocks the mutation. |
| `503` | `*_unavailable` | A required subsystem or provider is unavailable. |

</section>

<a id="post-call-signal"></a>
<section class="api-op">

## `POST /im/v3/api/calls/sessions/{rtcSessionId}/signals`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/calls/sessions/{rtcSessionId}/signals</code>
  <span class="api-op-id">operationId: calls.sessions.signals.create</span>
</div>

Posts an IM call signaling payload. The realtime payload includes the full message body so SDK
call watchers can parse invite, accept, reject, end, SDP, ICE, and provider-specific signal parts.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.calls`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `call.signal` capability.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RtcSignalEvent`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `rtcSessionId` | `string` | Yes | IM call signaling session identifier. |

### Request Body

<ApiSchemaTable schema="PostRtcSignalRequest" />

### Response `200`

<ApiSchemaTable schema="RtcSignalEvent" />

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

<a id="issue-call-credential"></a>
<section class="api-op">

## `POST /im/v3/api/calls/sessions/{rtcSessionId}/credentials`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/im/v3/api/calls/sessions/{rtcSessionId}/credentials</code>
  <span class="api-op-id">operationId: calls.sessions.credentials.create</span>
</div>

Issues an RTC media participant credential after IM authorization.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + AppContext</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`@sdkwork/im-sdk` / `sdk.calls`</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>Conversation `call.issue_credential` capability.</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200 RtcParticipantCredential`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `rtcSessionId` | `string` | Yes | IM call signaling session identifier. |

### Request Body

<ApiSchemaTable schema="IssueRtcParticipantCredentialRequest" />

### Response `200`

<ApiSchemaTable schema="RtcParticipantCredential" />

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
