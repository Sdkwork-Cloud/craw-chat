# Control Plane Social Graph Control

<p class="api-page-intro">
  Social graph control endpoints back <code>sdk.social</code> in the admin SDKs. They let operators
  bind direct chats, establish external collaboration topology, manage friendship aggregates, apply
  shared-channel policies, and enforce user blocks.
</p>

<div class="api-link-list">
  <a href="/api-reference/control-plane-api"><code>Control Plane</code> Back to Control Plane overview</a>
  <a href="/sdk/backend-sdk"><code>Backend SDK</code> See the cross-language backend client surface</a>
</div>

The checked-in control-plane authority keeps current social response payloads open-ended on
purpose. Mutation inputs, route semantics, and permission boundaries are stable; response bodies
should be treated as opaque JSON and consumed through the generated admin SDK surfaces.

<a id="bind-direct-chat"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/direct_chats/bindings`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/direct_chats/bindings</code>
  <span class="api-op-id">operationId: social.directChats.bindings.create</span>
</div>

Bind a direct chat to a conversation.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.social</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="BindDirectChatRequest" />

### Response `201`

`SocialDirectChatCommitResponse` is currently modeled as an open-ended social commit payload in the
checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The mutation payload is invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `control.write`. |
| `404` | `40401` | The requested node, plugin, or target resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the mutation. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="submit-friend-request"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/friend_requests`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/friend_requests</code>
  <span class="api-op-id">operationId: social.friendRequests.create</span>
</div>

Submit a friend request event.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.social</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="SubmitFriendRequestRequest" />

### Response `201`

`SocialFriendRequestCommitResponse` is currently modeled as an open-ended social commit payload in
the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The mutation payload is invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `control.write`. |
| `404` | `40401` | The requested node, plugin, or target resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the mutation. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="get-friend-request-snapshot"></a>
<section class="api-op">

## `GET /backend/v3/api/control/social/friend_requests/{requestId}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/social/friend_requests/{requestId}</code>
  <span class="api-op-id">operationId: social.friendRequests.retrieve</span>
</div>

Read a friend request snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.social</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `request_id` | `string` | Yes | Friend request aggregate identifier. |

### Response `200`

`SocialFriendRequestSnapshotResponse` is currently modeled as an open-ended social snapshot payload
in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40003` | Query or path parameters are invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks the required control-plane permission. |
| `404` | `40401` | The requested control-plane resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the read. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="activate-friendship"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/friendships`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/friendships</code>
  <span class="api-op-id">operationId: social.friendships.create</span>
</div>

Activate a friendship event.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.social</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="ActivateFriendshipRequest" />

### Response `201`

`SocialFriendshipCommitResponse` is currently modeled as an open-ended social commit payload in the
checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The mutation payload is invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `control.write`. |
| `404` | `40401` | The requested node, plugin, or target resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the mutation. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="get-friendship-snapshot"></a>
<section class="api-op">

## `GET /backend/v3/api/control/social/friendships/{friendshipId}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/social/friendships/{friendshipId}</code>
  <span class="api-op-id">operationId: social.friendships.retrieve</span>
</div>

Read a friendship snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.social</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `friendship_id` | `string` | Yes | Friendship aggregate identifier. |

### Response `200`

`SocialFriendshipSnapshotResponse` is currently modeled as an open-ended social snapshot payload in
the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40003` | Query or path parameters are invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks the required control-plane permission. |
| `404` | `40401` | The requested control-plane resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the read. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="apply-shared-channel-policy"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/shared_channel_policies`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/shared_channel_policies</code>
  <span class="api-op-id">operationId: social.sharedChannelPolicies.create</span>
</div>

Apply a shared-channel policy.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.social</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="ApplySharedChannelPolicyRequest" />

### Response `201`

`SocialSharedChannelPolicyCommitResponse` is currently modeled as an open-ended social commit
payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The mutation payload is invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `control.write`. |
| `404` | `40401` | The requested node, plugin, or target resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the mutation. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="get-shared-channel-policy-snapshot"></a>
<section class="api-op">

## `GET /backend/v3/api/control/social/shared_channel_policies/{policyId}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/social/shared_channel_policies/{policyId}</code>
  <span class="api-op-id">operationId: social.sharedChannelPolicies.retrieve</span>
</div>

Read a shared-channel policy snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.social</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `policy_id` | `string` | Yes | Shared-channel policy aggregate identifier. |

### Response `200`

`SocialSharedChannelPolicySnapshotResponse` is currently modeled as an open-ended social snapshot
payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40003` | Query or path parameters are invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks the required control-plane permission. |
| `404` | `40401` | The requested control-plane resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the read. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="block-user"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/user_blocks`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/user_blocks</code>
  <span class="api-op-id">operationId: social.userBlocks.create</span>
</div>

Block a user in the social graph.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.social</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="BlockUserRequest" />

### Response `201`

`SocialUserBlockCommitResponse` is currently modeled as an open-ended social commit payload in the
checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The mutation payload is invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `control.write`. |
| `404` | `40401` | The requested node, plugin, or target resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the mutation. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="get-user-block-snapshot"></a>
<section class="api-op">

## `GET /backend/v3/api/control/social/user_blocks/{blockId}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/social/user_blocks/{blockId}</code>
  <span class="api-op-id">operationId: social.userBlocks.retrieve</span>
</div>

Read a user block snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.social</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `block_id` | `string` | Yes | User block aggregate identifier. |

### Response `200`

`SocialUserBlockSnapshotResponse` is currently modeled as an open-ended social snapshot payload in
the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40003` | Query or path parameters are invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks the required control-plane permission. |
| `404` | `40401` | The requested control-plane resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the read. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="get-direct-chat-snapshot"></a>
<section class="api-op">

## `GET /backend/v3/api/control/social/direct_chats/{directChatId}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/social/direct_chats/{directChatId}</code>
  <span class="api-op-id">operationId: social.directChats.retrieve</span>
</div>

Read a direct chat snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.social</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `direct_chat_id` | `string` | Yes | Direct chat aggregate identifier. |

### Response `200`

`SocialDirectChatSnapshotResponse` is currently modeled as an open-ended social snapshot payload in
the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40003` | Query or path parameters are invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks the required control-plane permission. |
| `404` | `40401` | The requested control-plane resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the read. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="establish-external-connection"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/external_connections`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/external_connections</code>
  <span class="api-op-id">operationId: social.externalConnections.create</span>
</div>

Establish an external collaboration connection.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.social</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="EstablishExternalConnectionRequest" />

### Response `201`

`SocialExternalConnectionCommitResponse` is currently modeled as an open-ended social commit payload
in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The mutation payload is invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `control.write`. |
| `404` | `40401` | The requested node, plugin, or target resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the mutation. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="get-external-connection-snapshot"></a>
<section class="api-op">

## `GET /backend/v3/api/control/social/external_connections/{connectionId}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/social/external_connections/{connectionId}</code>
  <span class="api-op-id">operationId: social.externalConnections.retrieve</span>
</div>

Read an external connection snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.social</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `connection_id` | `string` | Yes | External connection aggregate identifier. |

### Response `200`

`SocialExternalConnectionSnapshotResponse` is currently modeled as an open-ended social snapshot
payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40003` | Query or path parameters are invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks the required control-plane permission. |
| `404` | `40401` | The requested control-plane resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the read. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="bind-external-member-link"></a>
<section class="api-op">

## `POST /backend/v3/api/control/social/external_member_links`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-post">POST</span>
  <code>/backend/v3/api/control/social/external_member_links</code>
  <span class="api-op-id">operationId: social.externalMemberLinks.create</span>
</div>

Bind an external member link.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.social</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`201`</span></div>
</div>

### Request Body

<ApiSchemaTable schema="BindExternalMemberLinkRequest" />

### Response `201`

`SocialExternalMemberLinkCommitResponse` is currently modeled as an open-ended social commit payload
in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40001` | The mutation payload is invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks `control.write`. |
| `404` | `40401` | The requested node, plugin, or target resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the mutation. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
<a id="get-external-member-link-snapshot"></a>
<section class="api-op">

## `GET /backend/v3/api/control/social/external_member_links/{linkId}`

<div class="api-op-header">
  <span class="endpoint-tag endpoint-get">GET</span>
  <code>/backend/v3/api/control/social/external_member_links/{linkId}</code>
  <span class="api-op-id">operationId: social.externalMemberLinks.retrieve</span>
</div>

Read an external member link snapshot.

<div class="api-meta-grid">
  <div class="api-meta-card"><strong>Security</strong><span>SDKWork dual token + resolved request context</span></div>
  <div class="api-meta-card"><strong>SDK</strong><span>`sdkwork-im-backend-sdk` / control.social</span></div>
  <div class="api-meta-card"><strong>Permission</strong><span>`control.read` or `control.write`</span></div>
  <div class="api-meta-card"><strong>Success</strong><span>`200`</span></div>
</div>

### Path Parameters

| Name | Type | Required | Description |
| --- | --- | --- | --- |
| `link_id` | `string` | Yes | External member link aggregate identifier. |

### Response `200`

`SocialExternalMemberLinkSnapshotResponse` is currently modeled as an open-ended social snapshot
payload in the checked-in control-plane authority. Treat it as opaque JSON.

### Error Responses

| HTTP | `code` | Description |
| --- | --- | --- |
| `400` | `40003` | Query or path parameters are invalid. |
| `401` | `40101` | SDKWork authentication or request-context resolution failed. |
| `403` | `40301` | The caller lacks the required control-plane permission. |
| `404` | `40401` | The requested control-plane resource does not exist. |
| `409` | `40901` | Current control-plane state blocks the read. |
| `503` | `50301` | The governance snapshot or provider runtime is unavailable. |

</section>
