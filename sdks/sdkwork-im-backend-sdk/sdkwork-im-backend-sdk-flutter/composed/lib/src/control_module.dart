import 'package:im_backend_api_generated/im_backend_api_generated.dart';

import 'context.dart';

class ImBackendControlModule {
  final ImBackendSdkContext context;

  ImBackendControlModule(this.context);

  ControlApi get raw => context.transportClient.control;

  Future<RouteNodeLifecycle?> activateNode(String nodeId) {
    return nodesActivate(nodeId);
  }

  Future<RouteNodeLifecycle?> drainNode(String nodeId) {
    return nodesDrain(nodeId);
  }

  Future<RouteMigrationResult?> migrateRoutes(
    String nodeId,
    MigrateRoutesRequest body,
  ) {
    return nodesRoutesMigrate(nodeId, body);
  }

  Future<ProtocolGovernanceResponse?> protocolGovernance() {
    return protocolGovernanceRetrieve();
  }

  Future<ProtocolRegistryResponse?> protocolRegistry() {
    return protocolRegistryRetrieve();
  }

  Future<ProviderPolicyHistoryResponse?> providerPolicies() {
    return providerPoliciesList();
  }

  Future<ProviderPolicyDiffResponse?> providerPoliciesDiff(
    int fromVersion,
    int toVersion,
  ) {
    return providerPoliciesDiffList(fromVersion, toVersion);
  }

  Future<ProviderBindingCommitResponse?> previewProviderPolicy(
    UpsertProviderBindingPolicyRequest body,
  ) {
    return providerPoliciesPreview(body);
  }

  Future<ProviderBindingCommitResponse?> rollbackProviderPolicy(
    ProviderPolicyRollbackRequest body,
  ) {
    return providerPoliciesRollback(body);
  }

  Future<ProviderRegistrySnapshotResponse?> providerRegistry() {
    return providerRegistryRetrieve();
  }

  Future<ProviderBindingsResponse?> providerBindings([String? tenantId]) {
    return providerBindingsList(tenantId);
  }

  Future<ProviderBindingCommitResponse?> upsertProviderBinding(
    UpsertProviderBindingPolicyRequest body,
  ) {
    return providerBindingsCreate(body);
  }

  Future<RouteNodeLifecycle?> nodesActivate(String nodeId) {
    return context.transportClient.control.nodesActivate(nodeId);
  }

  Future<RouteNodeLifecycle?> nodesDrain(String nodeId) {
    return context.transportClient.control.nodesDrain(nodeId);
  }

  Future<RouteMigrationResult?> nodesRoutesMigrate(
    String nodeId,
    MigrateRoutesRequest body,
  ) {
    return context.transportClient.control.nodesRoutesMigrate(nodeId, body);
  }

  Future<ProtocolGovernanceResponse?> protocolGovernanceRetrieve() {
    return context.transportClient.control.protocolGovernanceRetrieve();
  }

  Future<ProtocolRegistryResponse?> protocolRegistryRetrieve() {
    return context.transportClient.control.protocolRegistryRetrieve();
  }

  Future<ProviderPolicyHistoryResponse?> providerPoliciesList() {
    return context.transportClient.control.providerPoliciesList();
  }

  Future<ProviderPolicyDiffResponse?> providerPoliciesDiffList(
    int fromVersion,
    int toVersion,
  ) {
    return context.transportClient.control.providerPoliciesDiffList(
      fromVersion,
      toVersion,
    );
  }

  Future<ProviderBindingCommitResponse?> providerPoliciesPreview(
    UpsertProviderBindingPolicyRequest body,
  ) {
    return context.transportClient.control.providerPoliciesPreview(body);
  }

  Future<ProviderBindingCommitResponse?> providerPoliciesRollback(
    ProviderPolicyRollbackRequest body,
  ) {
    return context.transportClient.control.providerPoliciesRollback(body);
  }

  Future<ProviderRegistrySnapshotResponse?> providerRegistryRetrieve() {
    return context.transportClient.control.providerRegistryRetrieve();
  }

  Future<ProviderBindingsResponse?> providerBindingsList([String? tenantId]) {
    return context.transportClient.control.providerBindingsList(tenantId);
  }

  Future<ProviderBindingCommitResponse?> providerBindingsCreate(
    UpsertProviderBindingPolicyRequest body,
  ) {
    return context.transportClient.control.providerBindingsCreate(body);
  }

  Future<SocialDirectChatCommitResponse?> socialDirectChatsBindingsCreate(
    BindDirectChatRequest body,
  ) {
    return context.transportClient.control
        .socialDirectChatsBindingsCreate(body);
  }

  Future<SocialDirectChatSnapshotResponse?> socialDirectChatsRetrieve(
    String directChatId,
  ) {
    return context.transportClient.control.socialDirectChatsRetrieve(
      directChatId,
    );
  }

  Future<SocialExternalConnectionCommitResponse?>
      socialExternalConnectionsCreate(EstablishExternalConnectionRequest body) {
    return context.transportClient.control
        .socialExternalConnectionsCreate(body);
  }

  Future<SocialExternalConnectionSnapshotResponse?>
      socialExternalConnectionsRetrieve(String connectionId) {
    return context.transportClient.control.socialExternalConnectionsRetrieve(
      connectionId,
    );
  }

  Future<SocialExternalMemberLinkCommitResponse?>
      socialExternalMemberLinksCreate(
    BindExternalMemberLinkRequest body,
  ) {
    return context.transportClient.control
        .socialExternalMemberLinksCreate(body);
  }

  Future<SocialExternalMemberLinkSnapshotResponse?>
      socialExternalMemberLinksRetrieve(String linkId) {
    return context.transportClient.control.socialExternalMemberLinksRetrieve(
      linkId,
    );
  }

  Future<SocialFriendRequestCommitResponse?> socialFriendRequestsCreate(
    SubmitFriendRequestRequest body,
  ) {
    return context.transportClient.control.socialFriendRequestsCreate(body);
  }

  Future<SocialFriendRequestSnapshotResponse?> socialFriendRequestsRetrieve(
    String requestId,
  ) {
    return context.transportClient.control
        .socialFriendRequestsRetrieve(requestId);
  }

  Future<SocialFriendRequestCommitResponse?> socialFriendRequestsAccept(
    String requestId,
    AcceptFriendRequestRequest body,
  ) {
    return context.transportClient.control.socialFriendRequestsAccept(
      requestId,
      body,
    );
  }

  Future<SocialFriendRequestCommitResponse?> socialFriendRequestsDecline(
    String requestId,
    DeclineFriendRequestRequest body,
  ) {
    return context.transportClient.control.socialFriendRequestsDecline(
      requestId,
      body,
    );
  }

  Future<SocialFriendRequestCommitResponse?> socialFriendRequestsCancel(
    String requestId,
    CancelFriendRequestRequest body,
  ) {
    return context.transportClient.control.socialFriendRequestsCancel(
      requestId,
      body,
    );
  }

  Future<SocialFriendshipCommitResponse?> socialFriendshipsCreate(
    ActivateFriendshipRequest body,
  ) {
    return context.transportClient.control.socialFriendshipsCreate(body);
  }

  Future<SocialFriendshipSnapshotResponse?> socialFriendshipsRetrieve(
    String friendshipId,
  ) {
    return context.transportClient.control
        .socialFriendshipsRetrieve(friendshipId);
  }

  Future<SocialFriendshipCommitResponse?> socialFriendshipsRemove(
    String friendshipId,
    RemoveFriendshipRequest body,
  ) {
    return context.transportClient.control.socialFriendshipsRemove(
      friendshipId,
      body,
    );
  }

  Future<SocialSharedChannelSyncPendingClaimResponse?>
      socialRuntimeClaimPendingSharedChannelSyncTargetedCreate(
    SocialSharedChannelSyncPendingTargetedClaimRequest body,
  ) {
    return context.transportClient.control
        .socialRuntimeClaimPendingSharedChannelSyncTargetedCreate(body);
  }

  Future<SocialSharedChannelSyncDeadLetterInventoryResponse?>
      socialRuntimeDeadLetterSharedChannelSyncList() {
    return context.transportClient.control
        .socialRuntimeDeadLetterSharedChannelSyncList();
  }

  Future<SocialSharedChannelSyncDeliveredInventoryResponse?>
      socialRuntimeDeliveredSharedChannelSyncList() {
    return context.transportClient.control
        .socialRuntimeDeliveredSharedChannelSyncList();
  }

  Future<SocialSharedChannelSyncDeliveryStateInventoryResponse?>
      socialRuntimeDeliveryStateSharedChannelSyncList() {
    return context.transportClient.control
        .socialRuntimeDeliveryStateSharedChannelSyncList();
  }

  Future<SocialSharedChannelSyncPendingInventoryResponse?>
      socialRuntimePendingSharedChannelSyncList() {
    return context.transportClient.control
        .socialRuntimePendingSharedChannelSyncList();
  }

  Future<SocialSharedChannelSyncPendingStaleReclaimResponse?>
      socialRuntimeReclaimStalePendingSharedChannelSyncCreate() {
    return context.transportClient.control
        .socialRuntimeReclaimStalePendingSharedChannelSyncCreate();
  }

  Future<SocialSharedChannelSyncPendingReleaseResponse?>
      socialRuntimeReleasePendingSharedChannelSyncTargetedCreate(
    SocialSharedChannelSyncPendingTargetedReleaseRequest body,
  ) {
    return context.transportClient.control
        .socialRuntimeReleasePendingSharedChannelSyncTargetedCreate(body);
  }

  Future<SocialRuntimeRepairResponse?>
      socialRuntimeRepairDerivedSnapshotCreate() {
    return context.transportClient.control
        .socialRuntimeRepairDerivedSnapshotCreate();
  }

  Future<SocialSharedChannelSyncRepairResponse?>
      socialRuntimeRepairSharedChannelSyncCreate() {
    return context.transportClient.control
        .socialRuntimeRepairSharedChannelSyncCreate();
  }

  Future<SocialSharedChannelSyncTargetedRepublishResponse?>
      socialRuntimeRepublishPendingSharedChannelSyncTargetedCreate(
    SocialSharedChannelSyncTargetedRepublishRequest body,
  ) {
    return context.transportClient.control
        .socialRuntimeRepublishPendingSharedChannelSyncTargetedCreate(body);
  }

  Future<SocialSharedChannelSyncDeadLetterRequeueResponse?>
      socialRuntimeRequeueDeadLetterSharedChannelSyncCreate() {
    return context.transportClient.control
        .socialRuntimeRequeueDeadLetterSharedChannelSyncCreate();
  }

  Future<SocialSharedChannelSyncDeadLetterTargetedRequeueResponse?>
      socialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreate(
    SocialSharedChannelSyncDeadLetterTargetedRequeueRequest body,
  ) {
    return context.transportClient.control
        .socialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreate(body);
  }

  Future<SocialSharedChannelSyncPendingTakeoverResponse?>
      socialRuntimeTakeoverPendingSharedChannelSyncTargetedCreate(
    SocialSharedChannelSyncPendingTargetedTakeoverRequest body,
  ) {
    return context.transportClient.control
        .socialRuntimeTakeoverPendingSharedChannelSyncTargetedCreate(body);
  }

  Future<SocialSharedChannelPolicyCommitResponse?>
      socialSharedChannelPoliciesCreate(ApplySharedChannelPolicyRequest body) {
    return context.transportClient.control
        .socialSharedChannelPoliciesCreate(body);
  }

  Future<SocialSharedChannelPolicySnapshotResponse?>
      socialSharedChannelPoliciesRetrieve(String policyId) {
    return context.transportClient.control.socialSharedChannelPoliciesRetrieve(
      policyId,
    );
  }

  Future<SocialUserBlockCommitResponse?> socialUserBlocksCreate(
    BlockUserRequest body,
  ) {
    return context.transportClient.control.socialUserBlocksCreate(body);
  }

  Future<SocialUserBlockSnapshotResponse?> socialUserBlocksRetrieve(
    String blockId,
  ) {
    return context.transportClient.control.socialUserBlocksRetrieve(blockId);
  }
}
