# SDKWork IM HTTP API Inventory

Status: active
Owner: `im-platform`
Generated: yes
Generator: `docs/sites/scripts/generate-contract-inventories.mjs`
Specs: `API_SPEC.md`, `DOCUMENTATION_SPEC.md`

This inventory contains only HTTP APIs owned by this repository. Sibling platform and product
dependencies mounted by a gateway are intentionally excluded. Authored OpenAPI under `apis/` is
the contract authority; SDK-family OpenAPI under `sdks/` is a deterministic materialization.

## Surface Summary

| Surface | Prefix | Operations | Authored authority | SDK authority | SDK family |
| --- | --- | ---: | --- | --- | --- |
| Open API | `/im/v3/api` | 125 | `apis/open-api/im/sdkwork-im-im.openapi.yaml` | `sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml` | `sdkwork-im-sdk` |
| App API | `/app/v3/api` | 25 | `apis/app-api/communication/sdkwork-im-app-api.openapi.yaml` | `sdks/sdkwork-im-app-sdk/openapi/sdkwork-im-app-api.openapi.yaml` | `sdkwork-im-app-sdk` |
| Backend API | `/backend/v3/api` | 111 | `apis/backend-api/communication/sdkwork-im-backend-api.openapi.yaml` | `sdks/sdkwork-im-backend-sdk/openapi/sdkwork-im-backend-api.openapi.yaml` | `sdkwork-im-backend-sdk` |
| **Total** | - | **261** | - | - | - |

## Operation Inventory

Each row is extracted from the authored OpenAPI `paths` object. Method, path, and `operationId`
are public contract identifiers and must change at the OpenAPI source before this file is regenerated.

### Open API (125)

| Method | Path | operationId |
| --- | --- | --- |
| `POST` | `/im/v3/api/calls/sessions` | `calls.sessions.create` |
| `GET` | `/im/v3/api/calls/sessions/{rtcSessionId}` | `calls.sessions.retrieve` |
| `POST` | `/im/v3/api/calls/sessions/{rtcSessionId}/accept` | `calls.sessions.accept` |
| `POST` | `/im/v3/api/calls/sessions/{rtcSessionId}/credentials` | `calls.sessions.credentials.create` |
| `POST` | `/im/v3/api/calls/sessions/{rtcSessionId}/credentials/refresh` | `calls.sessions.credentials.refresh` |
| `POST` | `/im/v3/api/calls/sessions/{rtcSessionId}/end` | `calls.sessions.end` |
| `POST` | `/im/v3/api/calls/sessions/{rtcSessionId}/invite` | `calls.sessions.invite` |
| `POST` | `/im/v3/api/calls/sessions/{rtcSessionId}/reject` | `calls.sessions.reject` |
| `GET` | `/im/v3/api/calls/sessions/{rtcSessionId}/signals` | `calls.sessions.signals.list` |
| `POST` | `/im/v3/api/calls/sessions/{rtcSessionId}/signals` | `calls.sessions.signals.create` |
| `POST` | `/im/v3/api/chat/conversations` | `conversations.create` |
| `GET` | `/im/v3/api/chat/conversations/{conversationId}` | `conversations.retrieve` |
| `GET` | `/im/v3/api/chat/conversations/{conversationId}/agent_handoff` | `conversations.agentHandoff.retrieve` |
| `POST` | `/im/v3/api/chat/conversations/{conversationId}/agent_handoff/accept` | `conversations.agentHandoff.accept` |
| `POST` | `/im/v3/api/chat/conversations/{conversationId}/agent_handoff/close` | `conversations.agentHandoff.close` |
| `POST` | `/im/v3/api/chat/conversations/{conversationId}/agent_handoff/resolve` | `conversations.agentHandoff.resolve` |
| `GET` | `/im/v3/api/chat/conversations/{conversationId}/agents` | `conversations.agents.retrieve` |
| `PUT` | `/im/v3/api/chat/conversations/{conversationId}/agents` | `conversations.agents.update` |
| `GET` | `/im/v3/api/chat/conversations/{conversationId}/member_directory` | `conversations.memberDirectory.list` |
| `GET` | `/im/v3/api/chat/conversations/{conversationId}/members` | `conversations.members.list` |
| `POST` | `/im/v3/api/chat/conversations/{conversationId}/members/accept_invitation` | `conversations.members.acceptInvitation` |
| `POST` | `/im/v3/api/chat/conversations/{conversationId}/members/add` | `conversations.members.add` |
| `POST` | `/im/v3/api/chat/conversations/{conversationId}/members/change_role` | `conversations.members.changeRole` |
| `GET` | `/im/v3/api/chat/conversations/{conversationId}/members/current` | `conversations.members.current.retrieve` |
| `POST` | `/im/v3/api/chat/conversations/{conversationId}/members/leave` | `conversations.members.leave` |
| `POST` | `/im/v3/api/chat/conversations/{conversationId}/members/remove` | `conversations.members.remove` |
| `POST` | `/im/v3/api/chat/conversations/{conversationId}/members/transfer_owner` | `conversations.members.transferOwner` |
| `GET` | `/im/v3/api/chat/conversations/{conversationId}/messages` | `conversations.messages.list` |
| `POST` | `/im/v3/api/chat/conversations/{conversationId}/messages` | `conversations.messages.create` |
| `GET` | `/im/v3/api/chat/conversations/{conversationId}/messages/{messageId}/interaction_summary` | `conversations.messages.interactionSummary.retrieve` |
| `GET` | `/im/v3/api/chat/conversations/{conversationId}/pins` | `conversations.pins.list` |
| `GET` | `/im/v3/api/chat/conversations/{conversationId}/preferences` | `conversations.preferences.retrieve` |
| `PATCH` | `/im/v3/api/chat/conversations/{conversationId}/preferences` | `conversations.preferences.update` |
| `GET` | `/im/v3/api/chat/conversations/{conversationId}/profile` | `conversations.profile.retrieve` |
| `PATCH` | `/im/v3/api/chat/conversations/{conversationId}/profile` | `conversations.profile.update` |
| `GET` | `/im/v3/api/chat/conversations/{conversationId}/read_cursor` | `conversations.readCursor.retrieve` |
| `PATCH` | `/im/v3/api/chat/conversations/{conversationId}/read_cursor` | `conversations.readCursor.update` |
| `POST` | `/im/v3/api/chat/conversations/{conversationId}/system_channel/publish` | `conversations.systemChannel.publish` |
| `POST` | `/im/v3/api/chat/conversations/agent_dialogs` | `conversations.agentDialogs.create` |
| `POST` | `/im/v3/api/chat/conversations/agent_handoffs` | `conversations.agentHandoffs.create` |
| `POST` | `/im/v3/api/chat/conversations/direct_chats/bindings` | `conversations.directChats.bindings.create` |
| `POST` | `/im/v3/api/chat/conversations/system_channels` | `conversations.systemChannels.create` |
| `POST` | `/im/v3/api/chat/conversations/threads` | `conversations.threads.create` |
| `GET` | `/im/v3/api/chat/inbox` | `inbox.list` |
| `POST` | `/im/v3/api/chat/messages/{messageId}/edit` | `messages.edit` |
| `POST` | `/im/v3/api/chat/messages/{messageId}/favorites` | `messages.favorites.create` |
| `POST` | `/im/v3/api/chat/messages/{messageId}/pin` | `messages.pin` |
| `POST` | `/im/v3/api/chat/messages/{messageId}/reactions` | `messages.reactions.create` |
| `POST` | `/im/v3/api/chat/messages/{messageId}/reactions/remove` | `messages.reactions.remove` |
| `POST` | `/im/v3/api/chat/messages/{messageId}/recall` | `messages.recall` |
| `POST` | `/im/v3/api/chat/messages/{messageId}/unpin` | `messages.unpin` |
| `DELETE` | `/im/v3/api/chat/messages/{messageId}/visibility` | `messages.visibility.delete` |
| `GET` | `/im/v3/api/chat/messages/favorites` | `messages.favorites.list` |
| `DELETE` | `/im/v3/api/chat/messages/favorites/{favoriteId}` | `messages.favorites.delete` |
| `POST` | `/im/v3/api/chat/rooms` | `rooms.create` |
| `GET` | `/im/v3/api/chat/rooms/{roomId}` | `rooms.retrieve` |
| `POST` | `/im/v3/api/chat/rooms/{roomId}/enter` | `rooms.enter` |
| `POST` | `/im/v3/api/chat/rooms/{roomId}/leave` | `rooms.leave` |
| `POST` | `/im/v3/api/presence/heartbeat` | `presence.heartbeat` |
| `GET` | `/im/v3/api/presence/me` | `presence.me.retrieve` |
| `GET` | `/im/v3/api/realtime/events` | `realtime.events.list` |
| `POST` | `/im/v3/api/realtime/events/ack` | `realtime.events.ack` |
| `POST` | `/im/v3/api/realtime/subscriptions/sync` | `realtime.subscriptions.sync` |
| `GET` | `/im/v3/api/realtime/ws` | `realtime.ws.retrieve` |
| `GET` | `/im/v3/api/social/contacts` | `social.contacts.list` |
| `GET` | `/im/v3/api/social/contacts/{targetUserId}/preferences` | `social.contacts.preferences.retrieve` |
| `PATCH` | `/im/v3/api/social/contacts/{targetUserId}/preferences` | `social.contacts.preferences.update` |
| `POST` | `/im/v3/api/social/contacts/{targetUserId}/recommendations` | `social.contacts.recommendations.create` |
| `GET` | `/im/v3/api/social/contacts/tags` | `social.contacts.tags.list` |
| `POST` | `/im/v3/api/social/contacts/tags` | `social.contacts.tags.create` |
| `PATCH` | `/im/v3/api/social/contacts/tags/{tagId}` | `social.contacts.tags.update` |
| `DELETE` | `/im/v3/api/social/contacts/tags/{tagId}` | `social.contacts.tags.delete` |
| `GET` | `/im/v3/api/social/friend_requests` | `social.friendRequests.list` |
| `POST` | `/im/v3/api/social/friend_requests` | `social.friendRequests.create` |
| `POST` | `/im/v3/api/social/friend_requests/{friendRequestId}/accept` | `social.friendRequests.accept` |
| `POST` | `/im/v3/api/social/friend_requests/{friendRequestId}/cancel` | `social.friendRequests.cancel` |
| `POST` | `/im/v3/api/social/friend_requests/{friendRequestId}/decline` | `social.friendRequests.decline` |
| `GET` | `/im/v3/api/social/friend_requests/pending/count` | `social.friendRequests.pending.count.retrieve` |
| `POST` | `/im/v3/api/social/friendships/{friendshipId}/remove` | `social.friendships.remove` |
| `POST` | `/im/v3/api/social/user_blocks` | `social.userBlocks.create` |
| `DELETE` | `/im/v3/api/social/user_blocks/{blockId}` | `social.userBlocks.delete` |
| `GET` | `/im/v3/api/social/users` | `social.users.list` |
| `GET` | `/im/v3/api/spaces` | `spaces.list` |
| `POST` | `/im/v3/api/spaces` | `spaces.create` |
| `GET` | `/im/v3/api/spaces/{spaceId}` | `spaces.retrieve` |
| `PATCH` | `/im/v3/api/spaces/{spaceId}` | `spaces.update` |
| `DELETE` | `/im/v3/api/spaces/{spaceId}` | `spaces.delete` |
| `GET` | `/im/v3/api/spaces/{spaceId}/bans` | `spaces.bans.list` |
| `POST` | `/im/v3/api/spaces/{spaceId}/bans` | `spaces.bans.create` |
| `GET` | `/im/v3/api/spaces/{spaceId}/bans/{userId}` | `spaces.bans.retrieve` |
| `DELETE` | `/im/v3/api/spaces/{spaceId}/bans/{userId}` | `spaces.bans.delete` |
| `GET` | `/im/v3/api/spaces/{spaceId}/channels` | `spaces.channels.list` |
| `POST` | `/im/v3/api/spaces/{spaceId}/channels` | `spaces.channels.create` |
| `GET` | `/im/v3/api/spaces/{spaceId}/channels/{channelId}` | `spaces.channels.retrieve` |
| `PATCH` | `/im/v3/api/spaces/{spaceId}/channels/{channelId}` | `spaces.channels.update` |
| `DELETE` | `/im/v3/api/spaces/{spaceId}/channels/{channelId}` | `spaces.channels.delete` |
| `GET` | `/im/v3/api/spaces/{spaceId}/channels/{channelId}/access_rules` | `spaces.channels.accessRules.list` |
| `POST` | `/im/v3/api/spaces/{spaceId}/channels/{channelId}/access_rules` | `spaces.channels.accessRules.create` |
| `DELETE` | `/im/v3/api/spaces/{spaceId}/channels/{channelId}/access_rules/{ruleId}` | `spaces.channels.accessRules.delete` |
| `GET` | `/im/v3/api/spaces/{spaceId}/groups` | `spaces.groups.list` |
| `POST` | `/im/v3/api/spaces/{spaceId}/groups` | `spaces.groups.create` |
| `GET` | `/im/v3/api/spaces/{spaceId}/groups/{groupId}` | `spaces.groups.retrieve` |
| `PATCH` | `/im/v3/api/spaces/{spaceId}/groups/{groupId}` | `spaces.groups.update` |
| `DELETE` | `/im/v3/api/spaces/{spaceId}/groups/{groupId}` | `spaces.groups.delete` |
| `GET` | `/im/v3/api/spaces/{spaceId}/groups/{groupId}/members` | `spaces.groups.members.list` |
| `POST` | `/im/v3/api/spaces/{spaceId}/groups/{groupId}/members` | `spaces.groups.members.create` |
| `GET` | `/im/v3/api/spaces/{spaceId}/groups/{groupId}/members/{userId}` | `spaces.groups.members.retrieve` |
| `PATCH` | `/im/v3/api/spaces/{spaceId}/groups/{groupId}/members/{userId}` | `spaces.groups.members.update` |
| `DELETE` | `/im/v3/api/spaces/{spaceId}/groups/{groupId}/members/{userId}` | `spaces.groups.members.delete` |
| `GET` | `/im/v3/api/spaces/{spaceId}/invites` | `spaces.invites.list` |
| `POST` | `/im/v3/api/spaces/{spaceId}/invites` | `spaces.invites.create` |
| `GET` | `/im/v3/api/spaces/{spaceId}/invites/{inviteCode}` | `spaces.invites.retrieve` |
| `DELETE` | `/im/v3/api/spaces/{spaceId}/invites/{inviteCode}` | `spaces.invites.delete` |
| `POST` | `/im/v3/api/spaces/{spaceId}/invites/{inviteCode}/accept` | `spaces.invites.accept` |
| `GET` | `/im/v3/api/spaces/{spaceId}/members` | `spaces.members.list` |
| `POST` | `/im/v3/api/spaces/{spaceId}/members` | `spaces.members.create` |
| `GET` | `/im/v3/api/spaces/{spaceId}/members/{userId}` | `spaces.members.retrieve` |
| `PATCH` | `/im/v3/api/spaces/{spaceId}/members/{userId}` | `spaces.members.update` |
| `DELETE` | `/im/v3/api/spaces/{spaceId}/members/{userId}` | `spaces.members.delete` |
| `POST` | `/im/v3/api/streams` | `streams.create` |
| `POST` | `/im/v3/api/streams/{streamId}/abort` | `streams.abort` |
| `POST` | `/im/v3/api/streams/{streamId}/checkpoint` | `streams.checkpoint` |
| `POST` | `/im/v3/api/streams/{streamId}/complete` | `streams.complete` |
| `GET` | `/im/v3/api/streams/{streamId}/frames` | `streams.frames.list` |
| `POST` | `/im/v3/api/streams/{streamId}/frames` | `streams.frames.create` |

### App API (25)

| Method | Path | operationId |
| --- | --- | --- |
| `POST` | `/app/v3/api/automation/agent_responses` | `automation.agentResponses.create` |
| `POST` | `/app/v3/api/automation/agent_responses/{streamId}/complete` | `automation.agentResponses.complete` |
| `POST` | `/app/v3/api/automation/agent_responses/{streamId}/frames` | `automation.agentResponses.frames.create` |
| `POST` | `/app/v3/api/automation/agent_tool_calls` | `automation.agentToolCalls.create` |
| `POST` | `/app/v3/api/automation/executions` | `automation.executions.create` |
| `GET` | `/app/v3/api/automation/executions/{executionId}` | `automation.executions.retrieve` |
| `POST` | `/app/v3/api/automation/executions/{executionId}/agent_tool_calls/{toolCallId}/complete` | `automation.agentToolCalls.complete` |
| `POST` | `/app/v3/api/chat/conversations/{conversationId}/archive` | `conversations.archive` |
| `GET` | `/app/v3/api/chat/conversations/{conversationId}/knowledgebase` | `conversations.knowledgebase.retrieve` |
| `POST` | `/app/v3/api/chat/conversations/{conversationId}/knowledgebase` | `conversations.knowledgebase.create` |
| `POST` | `/app/v3/api/chat/conversations/{conversationId}/knowledgebase/launch` | `conversations.knowledgebase.launch` |
| `GET` | `/app/v3/api/media/provider_health` | `mediaHealth.retrieve` |
| `GET` | `/app/v3/api/notifications` | `notifications.list` |
| `GET` | `/app/v3/api/notifications/{notificationId}` | `notifications.retrieve` |
| `POST` | `/app/v3/api/notifications/requests` | `notifications.requests.create` |
| `GET` | `/app/v3/api/portal/access` | `access.retrieve` |
| `GET` | `/app/v3/api/portal/automation` | `automation.retrieve` |
| `GET` | `/app/v3/api/portal/conversations` | `conversationSnapshot.retrieve` |
| `GET` | `/app/v3/api/portal/dashboard` | `dashboard.retrieve` |
| `GET` | `/app/v3/api/portal/governance` | `governance.retrieve` |
| `GET` | `/app/v3/api/portal/home` | `home.retrieve` |
| `GET` | `/app/v3/api/portal/media` | `media.retrieve` |
| `GET` | `/app/v3/api/portal/realtime` | `realtime.retrieve` |
| `GET` | `/app/v3/api/portal/workspace` | `workspace.retrieve` |
| `GET` | `/app/v3/api/principal/profiles/provider_health` | `principalProfileHealth.retrieve` |

### Backend API (111)

| Method | Path | operationId |
| --- | --- | --- |
| `GET` | `/backend/v3/api/admin/api_key_groups` | `apiKeyGroups.list` |
| `POST` | `/backend/v3/api/admin/api_key_groups` | `apiKeyGroups.create` |
| `PATCH` | `/backend/v3/api/admin/api_key_groups/{groupId}` | `apiKeyGroups.update` |
| `DELETE` | `/backend/v3/api/admin/api_key_groups/{groupId}` | `apiKeyGroups.delete` |
| `POST` | `/backend/v3/api/admin/api_key_groups/{groupId}/status` | `apiKeyGroups.status` |
| `GET` | `/backend/v3/api/admin/api_keys` | `apiKeys.list` |
| `POST` | `/backend/v3/api/admin/api_keys` | `apiKeys.create` |
| `PUT` | `/backend/v3/api/admin/api_keys/{hashedKey}` | `apiKeys.update` |
| `DELETE` | `/backend/v3/api/admin/api_keys/{hashedKey}` | `apiKeys.delete` |
| `POST` | `/backend/v3/api/admin/api_keys/{hashedKey}/status` | `apiKeys.status` |
| `GET` | `/backend/v3/api/admin/billing/events` | `billing.events.list` |
| `GET` | `/backend/v3/api/admin/billing/events/summary` | `billing.events.summary.retrieve` |
| `GET` | `/backend/v3/api/admin/billing/summary` | `billing.summary.retrieve` |
| `GET` | `/backend/v3/api/admin/channel_models` | `channelModels.list` |
| `POST` | `/backend/v3/api/admin/channel_models` | `channelModels.create` |
| `DELETE` | `/backend/v3/api/admin/channel_models/{channelId}/models/{modelId}` | `channelModels.models.delete` |
| `GET` | `/backend/v3/api/admin/channels` | `channels.list` |
| `POST` | `/backend/v3/api/admin/channels` | `channels.create` |
| `DELETE` | `/backend/v3/api/admin/channels/{channelId}` | `channels.delete` |
| `GET` | `/backend/v3/api/admin/credentials` | `credentials.list` |
| `POST` | `/backend/v3/api/admin/credentials` | `credentials.create` |
| `DELETE` | `/backend/v3/api/admin/credentials/{tenantId}/providers/{providerId}/keys/{keyReference}` | `credentials.providers.keys.delete` |
| `POST` | `/backend/v3/api/admin/extensions/runtime_reloads` | `extensions.runtimeReloads.create` |
| `GET` | `/backend/v3/api/admin/extensions/runtime_statuses` | `extensions.runtimeStatuses.list` |
| `GET` | `/backend/v3/api/admin/gateway/rate_limit_policies` | `gateway.rateLimitPolicies.list` |
| `POST` | `/backend/v3/api/admin/gateway/rate_limit_policies` | `gateway.rateLimitPolicies.create` |
| `GET` | `/backend/v3/api/admin/gateway/rate_limit_windows` | `gateway.rateLimitWindows.list` |
| `GET` | `/backend/v3/api/admin/marketing/campaigns` | `marketing.campaigns.list` |
| `POST` | `/backend/v3/api/admin/marketing/campaigns` | `marketing.campaigns.create` |
| `POST` | `/backend/v3/api/admin/marketing/campaigns/{marketingCampaignId}/status` | `marketing.campaigns.status` |
| `GET` | `/backend/v3/api/admin/model_prices` | `modelPrices.list` |
| `POST` | `/backend/v3/api/admin/model_prices` | `modelPrices.create` |
| `DELETE` | `/backend/v3/api/admin/model_prices/{channelId}/models/{modelId}/providers/{proxyProviderId}` | `modelPrices.models.providers.delete` |
| `GET` | `/backend/v3/api/admin/models` | `models.list` |
| `POST` | `/backend/v3/api/admin/models` | `models.create` |
| `DELETE` | `/backend/v3/api/admin/models/{externalName}/providers/{providerId}` | `models.providers.delete` |
| `GET` | `/backend/v3/api/admin/providers` | `providers.list` |
| `POST` | `/backend/v3/api/admin/providers` | `providers.create` |
| `DELETE` | `/backend/v3/api/admin/providers/{providerId}` | `providers.delete` |
| `GET` | `/backend/v3/api/admin/routing/decision_logs` | `routing.decisionLogs.list` |
| `GET` | `/backend/v3/api/admin/routing/health_snapshots` | `routing.healthSnapshots.retrieve` |
| `GET` | `/backend/v3/api/admin/routing/profiles` | `routing.profiles.list` |
| `POST` | `/backend/v3/api/admin/routing/profiles` | `routing.profiles.create` |
| `GET` | `/backend/v3/api/admin/routing/snapshots` | `routing.snapshots.list` |
| `GET` | `/backend/v3/api/admin/storage/audit` | `storage.audit.list` |
| `GET` | `/backend/v3/api/admin/storage/config` | `storage.config.retrieve` |
| `POST` | `/backend/v3/api/admin/storage/config` | `storage.config.create` |
| `GET` | `/backend/v3/api/admin/storage/config/tenants/{tenantId}` | `storage.config.tenants.retrieve` |
| `POST` | `/backend/v3/api/admin/storage/config/tenants/{tenantId}` | `storage.config.tenants.create` |
| `DELETE` | `/backend/v3/api/admin/storage/config/tenants/{tenantId}` | `storage.config.tenants.delete` |
| `GET` | `/backend/v3/api/admin/storage/effective/tenants/{tenantId}` | `storage.effective.tenants.retrieve` |
| `GET` | `/backend/v3/api/admin/storage/providers` | `storage.providers.list` |
| `POST` | `/backend/v3/api/admin/storage/validate` | `storage.validation.create` |
| `POST` | `/backend/v3/api/admin/storage/validate/tenants/{tenantId}` | `storage.validation.tenants.create` |
| `GET` | `/backend/v3/api/admin/usage/records` | `usage.records.list` |
| `GET` | `/backend/v3/api/admin/usage/summary` | `usage.summary.retrieve` |
| `GET` | `/backend/v3/api/audit/export` | `export.retrieve` |
| `GET` | `/backend/v3/api/audit/records` | `records.list` |
| `POST` | `/backend/v3/api/audit/records` | `records.create` |
| `GET` | `/backend/v3/api/automation/governance` | `governance.retrieve` |
| `POST` | `/backend/v3/api/control/nodes/{nodeId}/activate` | `nodes.activate` |
| `POST` | `/backend/v3/api/control/nodes/{nodeId}/drain` | `nodes.drain` |
| `POST` | `/backend/v3/api/control/nodes/{nodeId}/routes/migrate` | `nodes.routes.migrate` |
| `GET` | `/backend/v3/api/control/protocol_governance` | `protocolGovernance.retrieve` |
| `GET` | `/backend/v3/api/control/protocol_registry` | `protocolRegistry.retrieve` |
| `GET` | `/backend/v3/api/control/provider_bindings` | `control.providerBindings.list` |
| `POST` | `/backend/v3/api/control/provider_bindings` | `control.providerBindings.create` |
| `GET` | `/backend/v3/api/control/provider_policies` | `providerPolicies.list` |
| `GET` | `/backend/v3/api/control/provider_policies/diff` | `providerPolicies.diff.list` |
| `POST` | `/backend/v3/api/control/provider_policies/preview` | `providerPolicies.preview` |
| `POST` | `/backend/v3/api/control/provider_policies/rollback` | `providerPolicies.rollback` |
| `GET` | `/backend/v3/api/control/provider_registry` | `providerRegistry.retrieve` |
| `GET` | `/backend/v3/api/control/social/direct_chats/{directChatId}` | `social.directChats.retrieve` |
| `POST` | `/backend/v3/api/control/social/direct_chats/bindings` | `social.directChats.bindings.create` |
| `POST` | `/backend/v3/api/control/social/external_connections` | `social.externalConnections.create` |
| `GET` | `/backend/v3/api/control/social/external_connections/{connectionId}` | `social.externalConnections.retrieve` |
| `POST` | `/backend/v3/api/control/social/external_member_links` | `social.externalMemberLinks.create` |
| `GET` | `/backend/v3/api/control/social/external_member_links/{linkId}` | `social.externalMemberLinks.retrieve` |
| `POST` | `/backend/v3/api/control/social/friend_requests` | `social.friendRequests.create` |
| `GET` | `/backend/v3/api/control/social/friend_requests/{requestId}` | `social.friendRequests.retrieve` |
| `POST` | `/backend/v3/api/control/social/friend_requests/{requestId}/accept` | `social.friendRequests.accept` |
| `POST` | `/backend/v3/api/control/social/friend_requests/{requestId}/cancel` | `social.friendRequests.cancel` |
| `POST` | `/backend/v3/api/control/social/friend_requests/{requestId}/decline` | `social.friendRequests.decline` |
| `POST` | `/backend/v3/api/control/social/friendships` | `social.friendships.create` |
| `GET` | `/backend/v3/api/control/social/friendships/{friendshipId}` | `social.friendships.retrieve` |
| `POST` | `/backend/v3/api/control/social/friendships/{friendshipId}/remove` | `social.friendships.remove` |
| `POST` | `/backend/v3/api/control/social/runtime/claim_pending_shared_channel_sync_targeted` | `social.runtime.claimPendingSharedChannelSyncTargeted.create` |
| `GET` | `/backend/v3/api/control/social/runtime/dead_letter_shared_channel_sync` | `social.runtime.deadLetterSharedChannelSync.list` |
| `GET` | `/backend/v3/api/control/social/runtime/delivered_shared_channel_sync` | `social.runtime.deliveredSharedChannelSync.list` |
| `GET` | `/backend/v3/api/control/social/runtime/delivery_state_shared_channel_sync` | `social.runtime.deliveryStateSharedChannelSync.list` |
| `GET` | `/backend/v3/api/control/social/runtime/pending_shared_channel_sync` | `social.runtime.pendingSharedChannelSync.list` |
| `POST` | `/backend/v3/api/control/social/runtime/reclaim_stale_pending_shared_channel_sync` | `social.runtime.reclaimStalePendingSharedChannelSync.create` |
| `POST` | `/backend/v3/api/control/social/runtime/release_pending_shared_channel_sync_targeted` | `social.runtime.releasePendingSharedChannelSyncTargeted.create` |
| `POST` | `/backend/v3/api/control/social/runtime/repair_derived_snapshot` | `social.runtime.repairDerivedSnapshot.create` |
| `POST` | `/backend/v3/api/control/social/runtime/repair_shared_channel_sync` | `social.runtime.repairSharedChannelSync.create` |
| `POST` | `/backend/v3/api/control/social/runtime/republish_pending_shared_channel_sync_targeted` | `social.runtime.republishPendingSharedChannelSyncTargeted.create` |
| `POST` | `/backend/v3/api/control/social/runtime/requeue_dead_letter_shared_channel_sync` | `social.runtime.requeueDeadLetterSharedChannelSync.create` |
| `POST` | `/backend/v3/api/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted` | `social.runtime.requeueDeadLetterSharedChannelSyncTargeted.create` |
| `POST` | `/backend/v3/api/control/social/runtime/takeover_pending_shared_channel_sync_targeted` | `social.runtime.takeoverPendingSharedChannelSyncTargeted.create` |
| `POST` | `/backend/v3/api/control/social/shared_channel_policies` | `social.sharedChannelPolicies.create` |
| `GET` | `/backend/v3/api/control/social/shared_channel_policies/{policyId}` | `social.sharedChannelPolicies.retrieve` |
| `POST` | `/backend/v3/api/control/social/user_blocks` | `social.userBlocks.create` |
| `GET` | `/backend/v3/api/control/social/user_blocks/{blockId}` | `social.userBlocks.retrieve` |
| `GET` | `/backend/v3/api/ops/cluster` | `cluster.retrieve` |
| `GET` | `/backend/v3/api/ops/commercial_readiness` | `commercialReadiness.retrieve` |
| `GET` | `/backend/v3/api/ops/diagnostics` | `diagnostics.retrieve` |
| `GET` | `/backend/v3/api/ops/health` | `health.retrieve` |
| `GET` | `/backend/v3/api/ops/lag` | `lag.retrieve` |
| `GET` | `/backend/v3/api/ops/provider_bindings` | `ops.providerBindings.list` |
| `GET` | `/backend/v3/api/ops/provider_bindings/drift` | `ops.providerBindings.drift.retrieve` |
| `GET` | `/backend/v3/api/ops/runtime_dir` | `runtimeDir.retrieve` |

## Regeneration And Verification

```bash
node docs/sites/scripts/generate-contract-inventories.mjs --write
node docs/sites/scripts/generate-contract-inventories.mjs --check
pnpm test:apis-authority-standard
```
