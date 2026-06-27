import 'dart:convert';
import '../http/client.dart';
import '../models.dart';

import 'paths.dart';
import 'response_helpers.dart';


class ControlApi {
  final HttpClient _client;

  ControlApi(this._client);

  /// Activate a realtime node and clear drain state.
  Future<RouteNodeLifecycle?> nodesActivate(String nodeId) async {
    final response = await _client.post(ApiPaths.backendPath('/control/nodes/${serializePathParameter(nodeId, const PathParameterSpec('nodeId', 'simple', false))}/activate'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RouteNodeLifecycle.fromJson(map);
    })();
  }

  /// Mark a realtime node as draining.
  Future<RouteNodeLifecycle?> nodesDrain(String nodeId) async {
    final response = await _client.post(ApiPaths.backendPath('/control/nodes/${serializePathParameter(nodeId, const PathParameterSpec('nodeId', 'simple', false))}/drain'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RouteNodeLifecycle.fromJson(map);
    })();
  }

  /// Migrate owned routes from the source node to the target node.
  Future<RouteMigrationResult?> nodesRoutesMigrate(String nodeId, MigrateRoutesRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/nodes/${serializePathParameter(nodeId, const PathParameterSpec('nodeId', 'simple', false))}/routes/migrate'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RouteMigrationResult.fromJson(map);
    })();
  }

  /// Read the control-plane protocol governance snapshot.
  Future<ProtocolGovernanceResponse?> protocolGovernanceRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/control/protocol_governance'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ProtocolGovernanceResponse.fromJson(map);
    })();
  }

  /// Read the control-plane protocol registry snapshot.
  Future<ProtocolRegistryResponse?> protocolRegistryRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/control/protocol_registry'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ProtocolRegistryResponse.fromJson(map);
    })();
  }

  /// Read provider policy history.
  Future<ProviderPolicyHistoryResponse?> providerPoliciesList() async {
    final response = await _client.get(ApiPaths.backendPath('/control/provider_policies'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ProviderPolicyHistoryResponse.fromJson(map);
    })();
  }

  /// Read provider policy diff between two versions.
  Future<ProviderPolicyDiffResponse?> providerPoliciesDiffList(int fromVersion, int toVersion) async {
    final query = buildQueryString([
      QueryParameterSpec('fromVersion', fromVersion, 'form', true, false, null),
      QueryParameterSpec('toVersion', toVersion, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/control/provider_policies/diff'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ProviderPolicyDiffResponse.fromJson(map);
    })();
  }

  /// Preview the effective provider policy result before commit.
  Future<ProviderBindingCommitResponse?> providerPoliciesPreview(UpsertProviderBindingPolicyRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/provider_policies/preview'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ProviderBindingCommitResponse.fromJson(map);
    })();
  }

  /// Rollback provider policy history to a target version.
  Future<ProviderBindingCommitResponse?> providerPoliciesRollback(ProviderPolicyRollbackRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/provider_policies/rollback'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ProviderBindingCommitResponse.fromJson(map);
    })();
  }

  /// Read the provider registry snapshot.
  Future<ProviderRegistrySnapshotResponse?> providerRegistryRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/control/provider_registry'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ProviderRegistrySnapshotResponse.fromJson(map);
    })();
  }

  /// Read effective provider bindings.
  Future<ProviderBindingsResponse?> providerBindingsList([String? tenantId]) async {
    final query = buildQueryString([
      QueryParameterSpec('tenantId', tenantId, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/control/provider_bindings'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ProviderBindingsResponse.fromJson(map);
    })();
  }

  /// Upsert a provider binding policy.
  Future<ProviderBindingCommitResponse?> providerBindingsCreate(UpsertProviderBindingPolicyRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/provider_bindings'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ProviderBindingCommitResponse.fromJson(map);
    })();
  }

  /// Bind a direct chat to a conversation.
  Future<SocialDirectChatCommitResponse?> socialDirectChatsBindingsCreate(BindDirectChatRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/social/direct_chats/bindings'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialDirectChatCommitResponse.fromJson(map);
    })();
  }

  /// Read a direct chat snapshot.
  Future<SocialDirectChatSnapshotResponse?> socialDirectChatsRetrieve(String directChatId) async {
    final response = await _client.get(ApiPaths.backendPath('/control/social/direct_chats/${serializePathParameter(directChatId, const PathParameterSpec('directChatId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialDirectChatSnapshotResponse.fromJson(map);
    })();
  }

  /// Establish an external collaboration connection.
  Future<SocialExternalConnectionCommitResponse?> socialExternalConnectionsCreate(EstablishExternalConnectionRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/social/external_connections'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialExternalConnectionCommitResponse.fromJson(map);
    })();
  }

  /// Read an external connection snapshot.
  Future<SocialExternalConnectionSnapshotResponse?> socialExternalConnectionsRetrieve(String connectionId) async {
    final response = await _client.get(ApiPaths.backendPath('/control/social/external_connections/${serializePathParameter(connectionId, const PathParameterSpec('connectionId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialExternalConnectionSnapshotResponse.fromJson(map);
    })();
  }

  /// Bind an external member link.
  Future<SocialExternalMemberLinkCommitResponse?> socialExternalMemberLinksCreate(BindExternalMemberLinkRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/social/external_member_links'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialExternalMemberLinkCommitResponse.fromJson(map);
    })();
  }

  /// Read an external member link snapshot.
  Future<SocialExternalMemberLinkSnapshotResponse?> socialExternalMemberLinksRetrieve(String linkId) async {
    final response = await _client.get(ApiPaths.backendPath('/control/social/external_member_links/${serializePathParameter(linkId, const PathParameterSpec('linkId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialExternalMemberLinkSnapshotResponse.fromJson(map);
    })();
  }

  /// Submit a friend request event.
  Future<SocialFriendRequestCommitResponse?> socialFriendRequestsCreate(SubmitFriendRequestRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/social/friend_requests'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendRequestCommitResponse.fromJson(map);
    })();
  }

  /// Read a friend request snapshot.
  Future<SocialFriendRequestSnapshotResponse?> socialFriendRequestsRetrieve(String requestId) async {
    final response = await _client.get(ApiPaths.backendPath('/control/social/friend_requests/${serializePathParameter(requestId, const PathParameterSpec('requestId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendRequestSnapshotResponse.fromJson(map);
    })();
  }

  /// Accept a friend request.
  Future<SocialFriendRequestCommitResponse?> socialFriendRequestsAccept(String requestId, AcceptFriendRequestRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/social/friend_requests/${serializePathParameter(requestId, const PathParameterSpec('requestId', 'simple', false))}/accept'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendRequestCommitResponse.fromJson(map);
    })();
  }

  /// Decline a friend request.
  Future<SocialFriendRequestCommitResponse?> socialFriendRequestsDecline(String requestId, DeclineFriendRequestRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/social/friend_requests/${serializePathParameter(requestId, const PathParameterSpec('requestId', 'simple', false))}/decline'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendRequestCommitResponse.fromJson(map);
    })();
  }

  /// Cancel a friend request.
  Future<SocialFriendRequestCommitResponse?> socialFriendRequestsCancel(String requestId, CancelFriendRequestRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/social/friend_requests/${serializePathParameter(requestId, const PathParameterSpec('requestId', 'simple', false))}/cancel'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendRequestCommitResponse.fromJson(map);
    })();
  }

  /// Activate a friendship event.
  Future<SocialFriendshipCommitResponse?> socialFriendshipsCreate(ActivateFriendshipRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/social/friendships'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendshipCommitResponse.fromJson(map);
    })();
  }

  /// Read a friendship snapshot.
  Future<SocialFriendshipSnapshotResponse?> socialFriendshipsRetrieve(String friendshipId) async {
    final response = await _client.get(ApiPaths.backendPath('/control/social/friendships/${serializePathParameter(friendshipId, const PathParameterSpec('friendshipId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendshipSnapshotResponse.fromJson(map);
    })();
  }

  /// Remove a friendship.
  Future<SocialFriendshipCommitResponse?> socialFriendshipsRemove(String friendshipId, RemoveFriendshipRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/social/friendships/${serializePathParameter(friendshipId, const PathParameterSpec('friendshipId', 'simple', false))}/remove'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendshipCommitResponse.fromJson(map);
    })();
  }

  /// Claim selected pending shared-channel sync entries.
  Future<SocialSharedChannelSyncPendingClaimResponse?> socialRuntimeClaimPendingSharedChannelSyncTargetedCreate(SocialSharedChannelSyncPendingTargetedClaimRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/social/runtime/claim_pending_shared_channel_sync_targeted'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialSharedChannelSyncPendingClaimResponse.fromJson(map);
    })();
  }

  /// Read the dead-letter shared-channel sync queue.
  Future<SocialSharedChannelSyncDeadLetterInventoryResponse?> socialRuntimeDeadLetterSharedChannelSyncList() async {
    final response = await _client.get(ApiPaths.backendPath('/control/social/runtime/dead_letter_shared_channel_sync'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialSharedChannelSyncDeadLetterInventoryResponse.fromJson(map);
    })();
  }

  /// Read the delivered shared-channel sync ledger.
  Future<SocialSharedChannelSyncDeliveredInventoryResponse?> socialRuntimeDeliveredSharedChannelSyncList() async {
    final response = await _client.get(ApiPaths.backendPath('/control/social/runtime/delivered_shared_channel_sync'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialSharedChannelSyncDeliveredInventoryResponse.fromJson(map);
    })();
  }

  /// Read merged shared-channel sync delivery state.
  Future<SocialSharedChannelSyncDeliveryStateInventoryResponse?> socialRuntimeDeliveryStateSharedChannelSyncList() async {
    final response = await _client.get(ApiPaths.backendPath('/control/social/runtime/delivery_state_shared_channel_sync'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialSharedChannelSyncDeliveryStateInventoryResponse.fromJson(map);
    })();
  }

  /// Read the pending shared-channel sync queue.
  Future<SocialSharedChannelSyncPendingInventoryResponse?> socialRuntimePendingSharedChannelSyncList() async {
    final response = await _client.get(ApiPaths.backendPath('/control/social/runtime/pending_shared_channel_sync'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialSharedChannelSyncPendingInventoryResponse.fromJson(map);
    })();
  }

  /// Reclaim stale shared-channel sync pending ownership.
  Future<SocialSharedChannelSyncPendingStaleReclaimResponse?> socialRuntimeReclaimStalePendingSharedChannelSyncCreate() async {
    final response = await _client.post(ApiPaths.backendPath('/control/social/runtime/reclaim_stale_pending_shared_channel_sync'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialSharedChannelSyncPendingStaleReclaimResponse.fromJson(map);
    })();
  }

  /// Release selected pending shared-channel sync entries.
  Future<SocialSharedChannelSyncPendingReleaseResponse?> socialRuntimeReleasePendingSharedChannelSyncTargetedCreate(SocialSharedChannelSyncPendingTargetedReleaseRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/social/runtime/release_pending_shared_channel_sync_targeted'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialSharedChannelSyncPendingReleaseResponse.fromJson(map);
    })();
  }

  /// Repair the persisted social runtime derived snapshot.
  Future<SocialRuntimeRepairResponse?> socialRuntimeRepairDerivedSnapshotCreate() async {
    final response = await _client.post(ApiPaths.backendPath('/control/social/runtime/repair_derived_snapshot'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialRuntimeRepairResponse.fromJson(map);
    })();
  }

  /// Repair shared-channel sync backlog state.
  Future<SocialSharedChannelSyncRepairResponse?> socialRuntimeRepairSharedChannelSyncCreate() async {
    final response = await _client.post(ApiPaths.backendPath('/control/social/runtime/repair_shared_channel_sync'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialSharedChannelSyncRepairResponse.fromJson(map);
    })();
  }

  /// Republish selected pending shared-channel sync entries.
  Future<SocialSharedChannelSyncTargetedRepublishResponse?> socialRuntimeRepublishPendingSharedChannelSyncTargetedCreate(SocialSharedChannelSyncTargetedRepublishRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/social/runtime/republish_pending_shared_channel_sync_targeted'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialSharedChannelSyncTargetedRepublishResponse.fromJson(map);
    })();
  }

  /// Requeue all dead-letter shared-channel sync entries.
  Future<SocialSharedChannelSyncDeadLetterRequeueResponse?> socialRuntimeRequeueDeadLetterSharedChannelSyncCreate() async {
    final response = await _client.post(ApiPaths.backendPath('/control/social/runtime/requeue_dead_letter_shared_channel_sync'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialSharedChannelSyncDeadLetterRequeueResponse.fromJson(map);
    })();
  }

  /// Requeue selected dead-letter shared-channel sync entries.
  Future<SocialSharedChannelSyncDeadLetterTargetedRequeueResponse?> socialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreate(SocialSharedChannelSyncDeadLetterTargetedRequeueRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialSharedChannelSyncDeadLetterTargetedRequeueResponse.fromJson(map);
    })();
  }

  /// Take over selected pending shared-channel sync entries.
  Future<SocialSharedChannelSyncPendingTakeoverResponse?> socialRuntimeTakeoverPendingSharedChannelSyncTargetedCreate(SocialSharedChannelSyncPendingTargetedTakeoverRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/social/runtime/takeover_pending_shared_channel_sync_targeted'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialSharedChannelSyncPendingTakeoverResponse.fromJson(map);
    })();
  }

  /// Apply a shared-channel policy.
  Future<SocialSharedChannelPolicyCommitResponse?> socialSharedChannelPoliciesCreate(ApplySharedChannelPolicyRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/social/shared_channel_policies'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialSharedChannelPolicyCommitResponse.fromJson(map);
    })();
  }

  /// Read a shared-channel policy snapshot.
  Future<SocialSharedChannelPolicySnapshotResponse?> socialSharedChannelPoliciesRetrieve(String policyId) async {
    final response = await _client.get(ApiPaths.backendPath('/control/social/shared_channel_policies/${serializePathParameter(policyId, const PathParameterSpec('policyId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialSharedChannelPolicySnapshotResponse.fromJson(map);
    })();
  }

  /// Block a user in the social graph.
  Future<SocialUserBlockCommitResponse?> socialUserBlocksCreate(BlockUserRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/control/social/user_blocks'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialUserBlockCommitResponse.fromJson(map);
    })();
  }

  /// Read a user block snapshot.
  Future<SocialUserBlockSnapshotResponse?> socialUserBlocksRetrieve(String blockId) async {
    final response = await _client.get(ApiPaths.backendPath('/control/social/user_blocks/${serializePathParameter(blockId, const PathParameterSpec('blockId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialUserBlockSnapshotResponse.fromJson(map);
    })();
  }
}

class PathParameterSpec {
  final String name;
  final String style;
  final bool explode;

  const PathParameterSpec(this.name, this.style, this.explode);
}

String serializePathParameter(dynamic value, PathParameterSpec spec) {
  if (value == null) return '';
  final style = spec.style.trim().isEmpty ? 'simple' : spec.style;
  if (value is Iterable) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (value is Map) {
    return serializePathObject(spec.name, value, style, spec.explode);
  }
  return pathPrimitivePrefix(spec.name, style) + Uri.encodeComponent(value.toString());
}

String serializePathArray(String name, Iterable values, String style, bool explode) {
  final serialized = values.where((item) => item != null).map((item) => Uri.encodeComponent(item.toString())).toList();
  if (serialized.isEmpty) return pathPrefix(name, style);
  if (style == 'matrix') {
    if (explode) {
      return serialized.map((item) => ';$name=$item').join();
    }
    return ';$name=${serialized.join(',')}';
  }
  final separator = explode ? '.' : ',';
  return pathPrefix(name, style) + serialized.join(separator);
}

String serializePathObject(String name, Map values, String style, bool explode) {
  final entries = <String>[];
  final exploded = <String>[];
  values.forEach((key, value) {
    if (value == null) return;
    final escapedKey = Uri.encodeComponent(key.toString());
    final escapedValue = Uri.encodeComponent(value.toString());
    if (explode) {
      if (style == 'matrix') {
        exploded.add(';$escapedKey=$escapedValue');
      } else {
        exploded.add('$escapedKey=$escapedValue');
      }
    } else {
      entries.add(escapedKey);
      entries.add(escapedValue);
    }
  });
  if (style == 'matrix') {
    if (explode) return exploded.join();
    return ';$name=${entries.join(',')}';
  }
  if (explode) {
    final separator = style == 'label' ? '.' : ',';
    return pathPrefix(name, style) + exploded.join(separator);
  }
  return pathPrefix(name, style) + entries.join(',');
}

String pathPrefix(String name, String style) {
  if (style == 'label') return '.';
  if (style == 'matrix') return ';$name';
  return '';
}

String pathPrimitivePrefix(String name, String style) {
  return style == 'matrix' ? ';$name=' : pathPrefix(name, style);
}
class QueryParameterSpec {
  final String name;
  final dynamic value;
  final String style;
  final bool explode;
  final bool allowReserved;
  final String? contentType;

  const QueryParameterSpec(
    this.name,
    this.value,
    this.style,
    this.explode,
    this.allowReserved,
    this.contentType,
  );
}

String buildQueryString(List<QueryParameterSpec> parameters) {
  final pairs = <String>[];
  for (final parameter in parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

void appendSerializedParameter(List<String> pairs, QueryParameterSpec parameter) {
  final value = parameter.value;
  if (value == null) return;

  final contentType = parameter.contentType;
  if (contentType != null && contentType.trim().isNotEmpty) {
    pairs.add('${urlEncode(parameter.name)}=${encodeQueryValue(jsonEncode(value), parameter.allowReserved)}');
    return;
  }

  final style = parameter.style.trim().isEmpty ? 'form' : parameter.style;
  if (style == 'deepObject' && value is Map) {
    appendDeepObjectParameter(pairs, parameter.name, value, parameter.allowReserved);
    return;
  }
  if (value is Iterable) {
    appendArrayParameter(pairs, parameter.name, value, style, parameter.explode, parameter.allowReserved);
    return;
  }
  if (value is Map) {
    appendObjectParameter(pairs, parameter.name, value, style, parameter.explode, parameter.allowReserved);
    return;
  }
  pairs.add('${urlEncode(parameter.name)}=${encodeQueryValue(value.toString(), parameter.allowReserved)}');
}

void appendArrayParameter(
  List<String> pairs,
  String name,
  Iterable values,
  String style,
  bool explode,
  bool allowReserved,
) {
  final serialized = values.where((item) => item != null).map((item) => item.toString()).toList();
  if (serialized.isEmpty) return;
  if (style == 'form' && explode) {
    for (final item in serialized) {
      pairs.add('${urlEncode(name)}=${encodeQueryValue(item, allowReserved)}');
    }
    return;
  }
  pairs.add('${urlEncode(name)}=${encodeQueryValue(serialized.join(','), allowReserved)}');
}

void appendObjectParameter(
  List<String> pairs,
  String name,
  Map values,
  String style,
  bool explode,
  bool allowReserved,
) {
  final serialized = <String>[];
  values.forEach((key, value) {
    if (value == null) return;
    if (style == 'form' && explode) {
      pairs.add('${urlEncode(key.toString())}=${encodeQueryValue(value.toString(), allowReserved)}');
      return;
    }
    serialized.add(key.toString());
    serialized.add(value.toString());
  });
  if (serialized.isNotEmpty) {
    pairs.add('${urlEncode(name)}=${encodeQueryValue(serialized.join(','), allowReserved)}');
  }
}

void appendDeepObjectParameter(List<String> pairs, String name, Map values, bool allowReserved) {
  values.forEach((key, value) {
    if (value != null) {
      pairs.add('${urlEncode('$name[$key]')}=${encodeQueryValue(value.toString(), allowReserved)}');
    }
  });
}

String encodeQueryValue(String value, bool allowReserved) {
  var encoded = urlEncode(value);
  if (!allowReserved) return encoded;
  const replacements = <String, String>{
    '%3A': ':',
    '%2F': '/',
    '%3F': '?',
    '%23': '#',
    '%5B': '[',
    '%5D': ']',
    '%40': '@',
    '%21': '!',
    '%24': r'$',
    '%26': '&',
    '%27': "'",
    '%28': '(',
    '%29': ')',
    '%2A': '*',
    '%2B': '+',
    '%2C': ',',
    '%3B': ';',
    '%3D': '=',
  };
  replacements.forEach((escaped, reserved) {
    encoded = encoded.replaceAll(escaped, reserved);
  });
  return encoded;
}

String urlEncode(String value) => Uri.encodeQueryComponent(value);
