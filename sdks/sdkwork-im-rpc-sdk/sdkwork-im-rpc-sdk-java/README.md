# SdkworkImRpc

SdkworkImRpc is an SDKWork RPC SDK scaffold generated from proto packages and an SDKWork RPC manifest.

## Proto packages

- sdkwork.communication.app.v3
- sdkwork.communication.backend.v3
- sdkwork.communication.internal.v1

## Service catalog

- sdkwork.communication.app.v3.PresenceService (app)
  - CreatePresenceHeartbeat: presence.heartbeat.create, unary, auth=app-session, idempotency=optional
  - RetrieveMyPresence: presence.me.retrieve, unary, auth=app-session, idempotency=none
  - WatchPresence: presence.watch, server, auth=app-session, idempotency=none
- sdkwork.communication.app.v3.RealtimeService (app)
  - SyncRealtimeSubscriptions: realtime.subscriptions.sync, unary, auth=app-session, idempotency=optional
  - AckRealtimeEvents: realtime.events.ack, unary, auth=app-session, idempotency=optional
  - ListRealtimeEvents: realtime.events.list, unary, auth=app-session, idempotency=none
  - WatchRealtimeEvents: realtime.events.watch, server, auth=app-session, idempotency=none
- sdkwork.communication.app.v3.ConversationService (app)
  - CreateConversation: conversations.create, unary, auth=app-session, idempotency=required
  - CreateAgentDialog: conversations.agentDialogs.create, unary, auth=app-session, idempotency=required
  - CreateAgentHandoff: conversations.agentHandoffs.create, unary, auth=app-session, idempotency=required
  - CreateSystemChannel: conversations.systemChannels.create, unary, auth=app-session, idempotency=required
  - CreateThread: conversations.threads.create, unary, auth=app-session, idempotency=required
  - BindDirectChat: conversations.directChats.bind, unary, auth=app-session, idempotency=required
  - RetrieveConversation: conversations.retrieve, unary, auth=app-session, idempotency=none
  - RetrieveInbox: inbox.retrieve, unary, auth=app-session, idempotency=none
  - ListConversationMembers: conversations.members.list, unary, auth=app-session, idempotency=none
  - AddConversationMember: conversations.members.add, unary, auth=app-session, idempotency=required
  - RemoveConversationMember: conversations.members.remove, unary, auth=app-session, idempotency=required
  - TransferConversationOwner: conversations.members.transferOwner, unary, auth=app-session, idempotency=required
  - ChangeConversationMemberRole: conversations.members.changeRole, unary, auth=app-session, idempotency=required
  - LeaveConversation: conversations.members.leave, unary, auth=app-session, idempotency=required
  - RetrieveConversationPreferences: conversations.preferences.retrieve, unary, auth=app-session, idempotency=none
  - UpdateConversationPreferences: conversations.preferences.update, unary, auth=app-session, idempotency=optional
  - RetrieveConversationProfile: conversations.profile.retrieve, unary, auth=app-session, idempotency=none
  - UpdateConversationProfile: conversations.profile.update, unary, auth=app-session, idempotency=optional
  - RetrieveReadCursor: conversations.readCursor.retrieve, unary, auth=app-session, idempotency=none
  - UpdateReadCursor: conversations.readCursor.update, unary, auth=app-session, idempotency=optional
  - ListConversationMemberDirectory: conversations.memberDirectory.list, unary, auth=app-session, idempotency=none
  - ListPinnedMessages: conversations.pins.list, unary, auth=app-session, idempotency=none
- sdkwork.communication.app.v3.ContactService (app)
  - ListContacts: contacts.list, unary, auth=app-session, idempotency=none
  - ListContactTags: social.contacts.tags.list, unary, auth=app-session, idempotency=none
  - CreateContactTag: social.contacts.tags.create, unary, auth=app-session, idempotency=required
  - UpdateContactTag: social.contacts.tags.update, unary, auth=app-session, idempotency=optional
  - DeleteContactTag: social.contacts.tags.delete, unary, auth=app-session, idempotency=required
  - CreateContactRecommendation: social.contacts.recommendations.create, unary, auth=app-session, idempotency=required
  - RetrieveContactPreferences: social.contacts.preferences.retrieve, unary, auth=app-session, idempotency=none
  - UpdateContactPreferences: social.contacts.preferences.update, unary, auth=app-session, idempotency=optional
- sdkwork.communication.app.v3.MessageService (app)
  - ListConversationMessages: conversations.messages.list, unary, auth=app-session, idempotency=none
  - CreateConversationMessage: conversations.messages.create, unary, auth=app-session, idempotency=required
  - PublishSystemChannelMessage: conversations.systemChannel.publish, unary, auth=app-session, idempotency=required
  - RetrieveMessageInteractionSummary: conversations.messages.interactionSummary.retrieve, unary, auth=app-session, idempotency=none
  - EditMessage: messages.edit, unary, auth=app-session, idempotency=required
  - RecallMessage: messages.recall, unary, auth=app-session, idempotency=required
  - ListFavoriteMessages: messages.favorites.list, unary, auth=app-session, idempotency=none
  - CreateMessageFavorite: messages.favorites.create, unary, auth=app-session, idempotency=required
  - DeleteMessageFavorite: messages.favorites.delete, unary, auth=app-session, idempotency=required
  - DeleteMessageVisibility: messages.visibility.delete, unary, auth=app-session, idempotency=required
  - CreateMessageReaction: messages.reactions.create, unary, auth=app-session, idempotency=required
  - DeleteMessageReaction: messages.reactions.delete, unary, auth=app-session, idempotency=required
  - PinMessage: messages.pin.create, unary, auth=app-session, idempotency=required
  - UnpinMessage: messages.pin.delete, unary, auth=app-session, idempotency=required
- sdkwork.communication.app.v3.SocialService (app)
  - ListSocialUsers: social.users.list, unary, auth=app-session, idempotency=none
  - ListFriendRequests: social.friendRequests.list, unary, auth=app-session, idempotency=none
  - CreateFriendRequest: social.friendRequests.create, unary, auth=app-session, idempotency=required
  - AcceptFriendRequest: social.friendRequests.accept, unary, auth=app-session, idempotency=required
  - DeclineFriendRequest: social.friendRequests.decline, unary, auth=app-session, idempotency=required
  - CancelFriendRequest: social.friendRequests.cancel, unary, auth=app-session, idempotency=required
  - RemoveFriendship: social.friendships.remove, unary, auth=app-session, idempotency=required
- sdkwork.communication.app.v3.StreamService (app)
  - CreateStream: streams.create, unary, auth=app-session, idempotency=required
  - ListStreamFrames: streams.frames.list, unary, auth=app-session, idempotency=none
  - AppendStreamFrame: streams.frames.create, unary, auth=app-session, idempotency=required
  - CreateStreamCheckpoint: streams.checkpoint.create, unary, auth=app-session, idempotency=required
  - CompleteStream: streams.complete, unary, auth=app-session, idempotency=required
  - AbortStream: streams.abort, unary, auth=app-session, idempotency=required
  - WatchStreamFrames: streams.frames.watch, server, auth=app-session, idempotency=none
- sdkwork.communication.app.v3.CallService (app)
  - CreateCallSession: calls.sessions.create, unary, auth=app-session, idempotency=required
  - RetrieveCallSession: calls.sessions.retrieve, unary, auth=app-session, idempotency=none
  - InviteCallSession: calls.sessions.invite, unary, auth=app-session, idempotency=required
  - AcceptCallSession: calls.sessions.accept, unary, auth=app-session, idempotency=required
  - RejectCallSession: calls.sessions.reject, unary, auth=app-session, idempotency=required
  - EndCallSession: calls.sessions.end, unary, auth=app-session, idempotency=required
  - CreateCallSignal: calls.sessions.signals.create, unary, auth=app-session, idempotency=required
  - CreateCallCredential: calls.sessions.credentials.create, unary, auth=app-session, idempotency=required
  - WatchCallSignals: calls.sessions.signals.watch, server, auth=app-session, idempotency=none
- sdkwork.communication.app.v3.NotificationService (app)
  - ListNotifications: notifications.list, unary, auth=app-session, idempotency=none
  - CreateNotificationRequest: notifications.requests.create, unary, auth=app-session, idempotency=required
  - RetrieveNotification: notifications.retrieve, unary, auth=app-session, idempotency=none
  - WatchNotifications: notifications.watch, server, auth=app-session, idempotency=none
- sdkwork.communication.app.v3.AutomationService (app)
  - CreateAutomationExecution: automation.executions.create, unary, auth=app-session, idempotency=required
  - RetrieveAutomationExecution: automation.executions.retrieve, unary, auth=app-session, idempotency=none
  - CreateAgentResponse: automation.agentResponses.create, unary, auth=app-session, idempotency=required
  - CompleteAgentResponse: automation.agentResponses.complete, unary, auth=app-session, idempotency=required
  - CreateAgentResponseFrame: automation.agentResponses.frames.create, unary, auth=app-session, idempotency=required
  - RequestAgentToolCall: automation.agentToolCalls.create, unary, auth=app-session, idempotency=required
  - CompleteAgentToolCall: automation.agentToolCalls.complete, unary, auth=app-session, idempotency=required
- sdkwork.communication.backend.v3.CommunicationOpsService (backend)
  - RetrieveHealth: health.retrieve, unary, auth=backend-admin, idempotency=none
  - RetrieveCluster: cluster.retrieve, unary, auth=backend-admin, idempotency=none
  - RetrieveLag: lag.retrieve, unary, auth=backend-admin, idempotency=none
  - RetrieveReplayStatus: replayStatus.retrieve, unary, auth=backend-admin, idempotency=none
  - RetrieveCommercialReadiness: commercialReadiness.retrieve, unary, auth=backend-admin, idempotency=none
  - RetrieveRuntimeDir: runtimeDir.retrieve, unary, auth=backend-admin, idempotency=none
  - ListOpsProviderBindings: ops.providerBindings.list, unary, auth=backend-admin, idempotency=none
  - RetrieveProviderBindingDrift: ops.providerBindings.drift.retrieve, unary, auth=backend-admin, idempotency=none
  - RetrieveDiagnostics: diagnostics.retrieve, unary, auth=backend-admin, idempotency=none
- sdkwork.communication.backend.v3.RealtimeNodeAdminService (backend)
  - ActivateRealtimeNode: nodes.activate, unary, auth=backend-admin, idempotency=required
  - DrainRealtimeNode: nodes.drain, unary, auth=backend-admin, idempotency=required
  - MigrateRealtimeNodeRoutes: nodes.routes.migrate, unary, auth=backend-admin, idempotency=required
- sdkwork.communication.backend.v3.CommunicationControlService (backend)
  - RetrieveProtocolGovernance: protocolGovernance.retrieve, unary, auth=backend-admin, idempotency=none
  - RetrieveProtocolRegistry: protocolRegistry.retrieve, unary, auth=backend-admin, idempotency=none
  - ListProviderPolicies: providerPolicies.list, unary, auth=backend-admin, idempotency=none
  - PreviewProviderPolicy: providerPolicies.preview, unary, auth=backend-admin, idempotency=optional
  - RollbackProviderPolicy: providerPolicies.rollback, unary, auth=backend-admin, idempotency=required
  - RetrieveProviderRegistry: providerRegistry.retrieve, unary, auth=backend-admin, idempotency=none
  - ListControlProviderBindings: control.providerBindings.list, unary, auth=backend-admin, idempotency=none
  - CreateControlProviderBinding: control.providerBindings.create, unary, auth=backend-admin, idempotency=required
- sdkwork.communication.backend.v3.SocialAdminService (backend)
  - CreateDirectChatBinding: social.directChats.bindings.create, unary, auth=backend-admin, idempotency=required
  - RetrieveDirectChat: social.directChats.retrieve, unary, auth=backend-admin, idempotency=none
  - CreateExternalConnection: social.externalConnections.create, unary, auth=backend-admin, idempotency=required
  - RetrieveExternalConnection: social.externalConnections.retrieve, unary, auth=backend-admin, idempotency=none
  - CreateExternalMemberLink: social.externalMemberLinks.create, unary, auth=backend-admin, idempotency=required
  - RetrieveExternalMemberLink: social.externalMemberLinks.retrieve, unary, auth=backend-admin, idempotency=none
  - CreateManagedFriendRequest: social.friendRequests.create, unary, auth=backend-admin, idempotency=required
  - RetrieveManagedFriendRequest: social.friendRequests.retrieve, unary, auth=backend-admin, idempotency=none
  - AcceptManagedFriendRequest: social.friendRequests.accept, unary, auth=backend-admin, idempotency=required
  - DeclineManagedFriendRequest: social.friendRequests.decline, unary, auth=backend-admin, idempotency=required
  - CancelManagedFriendRequest: social.friendRequests.cancel, unary, auth=backend-admin, idempotency=required
  - CreateManagedFriendship: social.friendships.create, unary, auth=backend-admin, idempotency=required
  - RetrieveManagedFriendship: social.friendships.retrieve, unary, auth=backend-admin, idempotency=none
  - RemoveManagedFriendship: social.friendships.remove, unary, auth=backend-admin, idempotency=required
  - CreateSharedChannelPolicy: social.sharedChannelPolicies.create, unary, auth=backend-admin, idempotency=required
  - RetrieveSharedChannelPolicy: social.sharedChannelPolicies.retrieve, unary, auth=backend-admin, idempotency=none
  - CreateUserBlock: social.userBlocks.create, unary, auth=backend-admin, idempotency=required
  - RetrieveUserBlock: social.userBlocks.retrieve, unary, auth=backend-admin, idempotency=none
- sdkwork.communication.backend.v3.SocialRuntimeAdminService (backend)
  - ClaimPendingSharedChannelSyncTargeted: social.runtime.claimPendingSharedChannelSyncTargeted.create, unary, auth=backend-admin, idempotency=required
  - ListDeadLetterSharedChannelSync: social.runtime.deadLetterSharedChannelSync.list, unary, auth=backend-admin, idempotency=none
  - ListDeliveredSharedChannelSync: social.runtime.deliveredSharedChannelSync.list, unary, auth=backend-admin, idempotency=none
  - ListDeliveryStateSharedChannelSync: social.runtime.deliveryStateSharedChannelSync.list, unary, auth=backend-admin, idempotency=none
  - ListPendingSharedChannelSync: social.runtime.pendingSharedChannelSync.list, unary, auth=backend-admin, idempotency=none
  - ReclaimStalePendingSharedChannelSync: social.runtime.reclaimStalePendingSharedChannelSync.create, unary, auth=backend-admin, idempotency=required
  - ReleasePendingSharedChannelSyncTargeted: social.runtime.releasePendingSharedChannelSyncTargeted.create, unary, auth=backend-admin, idempotency=required
  - RepairDerivedSnapshot: social.runtime.repairDerivedSnapshot.create, unary, auth=backend-admin, idempotency=required
  - RepairSharedChannelSync: social.runtime.repairSharedChannelSync.create, unary, auth=backend-admin, idempotency=required
  - RepublishPendingSharedChannelSyncTargeted: social.runtime.republishPendingSharedChannelSyncTargeted.create, unary, auth=backend-admin, idempotency=required
  - RequeueDeadLetterSharedChannelSync: social.runtime.requeueDeadLetterSharedChannelSync.create, unary, auth=backend-admin, idempotency=required
  - RequeueDeadLetterSharedChannelSyncTargeted: social.runtime.requeueDeadLetterSharedChannelSyncTargeted.create, unary, auth=backend-admin, idempotency=required
  - TakeoverPendingSharedChannelSyncTargeted: social.runtime.takeoverPendingSharedChannelSyncTargeted.create, unary, auth=backend-admin, idempotency=required
- sdkwork.communication.backend.v3.AuditAdminService (backend)
  - ListAuditRecords: records.list, unary, auth=backend-admin, idempotency=none
  - CreateAuditRecord: records.create, unary, auth=backend-admin, idempotency=required
  - RetrieveAuditExport: export.retrieve, unary, auth=backend-admin, idempotency=none
- sdkwork.communication.internal.v1.RuntimeTopologyService (internal)
  - RetrieveRuntimeTopology: internal.runtimeTopology.retrieve, unary, auth=service-mtls, idempotency=none
  - ListRuntimeCapabilities: internal.runtimeCapabilities.list, unary, auth=service-mtls, idempotency=none
  - WatchRuntimeTopology: internal.runtimeTopology.watch, server, auth=service-mtls, idempotency=none
- sdkwork.communication.internal.v1.RouteLeaseService (internal)
  - ClaimRouteLease: internal.routeLeases.claim, unary, auth=service-mtls, idempotency=required
  - RenewRouteLease: internal.routeLeases.renew, unary, auth=service-mtls, idempotency=required
  - ReleaseRouteLease: internal.routeLeases.release, unary, auth=service-mtls, idempotency=required
  - ListRouteLeases: internal.routeLeases.list, unary, auth=service-mtls, idempotency=none
- sdkwork.communication.internal.v1.DomainEventRelayService (internal)
  - PublishDomainEvent: internal.domainEvents.publish, unary, auth=service-mtls, idempotency=required
  - AckDomainEvent: internal.domainEvents.ack, unary, auth=service-mtls, idempotency=required
  - WatchDomainEvents: internal.domainEvents.watch, server, auth=service-mtls, idempotency=none

## Endpoint and TLS/mTLS

Configure the endpoint through application SDK bootstrap. Use TLS for protected remote endpoints and mTLS when the deployment policy requires client certificates.

## Metadata auth

Use metadata providers for authorization, access-token, traceparent, idempotency-key, and x-request-hash. Application code should inject providers through SDK bootstrap instead of assembling raw metadata in business modules.

## Deadline and cancellation

Set a deadline for each RPC call through the generated deadline helpers or the language transport options. Callers should pass cancellation through the platform-native signal when available.

## Unary call example

```ts
import { createRpcIdempotencyMetadata, createStaticMetadataProvider, resolveRpcDeadlineMs } from './src/index.js';

const metadataProvider = createStaticMetadataProvider({
  authorization: 'Bearer <auth-token>',
  'access-token': '<access-token>',
  'idempotency-key': 'create-message-001',
});
const deadlineMs = resolveRpcDeadlineMs({ timeoutMs: 5000 });
const idempotencyMetadata = createRpcIdempotencyMetadata({ idempotencyKey: 'create-message-001' });
// Call PresenceService.CreatePresenceHeartbeat with metadataProvider, idempotencyMetadata, and deadlineMs using the generated protobuf client.
```

## Verification commands

- buf lint
- buf breaking
- sdkgen generate --protocol rpc --dry-run
- run the generated client compile command for this language
