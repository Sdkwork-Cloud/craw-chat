import '../http/client.dart';
import '../models.dart';

import 'paths.dart';
import 'response_helpers.dart';


class SpacesApi {
  final HttpClient _client;

  SpacesApi(this._client);

  /// Create a space
  Future<SpaceView?> create(SpaceCreateRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/spaces'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceView.fromJson(map);
    })();
  }

  /// List spaces
  Future<SpaceListResponse?> list() async {
    final response = await _client.get(ApiPaths.imPath('/spaces'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceListResponse.fromJson(map);
    })();
  }

  /// Get a space
  Future<SpaceView?> get_(String spaceId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceView.fromJson(map);
    })();
  }

  /// Update a space
  Future<SpaceView?> update(String spaceId, SpaceUpdateRequest body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceView.fromJson(map);
    })();
  }

  /// Delete a space
  Future<void> delete(String spaceId) async {
    await _client.delete(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}'));
  }

  /// List spaces members
  Future<SpaceMemberListResponse?> membersList(String spaceId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/members'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceMemberListResponse.fromJson(map);
    })();
  }

  /// Create spaces members
  Future<SpaceMemberView?> membersCreate(String spaceId, SpaceMemberCreateRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/members'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceMemberView.fromJson(map);
    })();
  }

  /// Get spaces members
  Future<SpaceMemberView?> membersGet(String spaceId, String userId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/members/${serializePathParameter(userId, const PathParameterSpec('userId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceMemberView.fromJson(map);
    })();
  }

  /// Update spaces members
  Future<SpaceMemberView?> membersUpdate(String spaceId, String userId, SpaceMemberUpdateRequest body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/members/${serializePathParameter(userId, const PathParameterSpec('userId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceMemberView.fromJson(map);
    })();
  }

  /// Delete spaces members
  Future<void> membersDelete(String spaceId, String userId) async {
    await _client.delete(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/members/${serializePathParameter(userId, const PathParameterSpec('userId', 'simple', false))}'));
  }

  /// List spaces groups
  Future<SpaceGroupListResponse?> groupsList(String spaceId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceGroupListResponse.fromJson(map);
    })();
  }

  /// Create spaces groups
  Future<SpaceGroupView?> groupsCreate(String spaceId, SpaceGroupCreateRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceGroupView.fromJson(map);
    })();
  }

  /// Get spaces groups
  Future<SpaceGroupView?> groupsGet(String spaceId, String groupId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceGroupView.fromJson(map);
    })();
  }

  /// Update spaces groups
  Future<SpaceGroupView?> groupsUpdate(String spaceId, String groupId, SpaceGroupUpdateRequest body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceGroupView.fromJson(map);
    })();
  }

  /// Delete spaces groups
  Future<void> groupsDelete(String spaceId, String groupId) async {
    await _client.delete(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}'));
  }

  /// List spaces groups members
  Future<SpaceGroupMemberListResponse?> groupsMembersList(String spaceId, String groupId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}/members'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceGroupMemberListResponse.fromJson(map);
    })();
  }

  /// Create spaces groups members
  Future<SpaceGroupMemberView?> groupsMembersCreate(String spaceId, String groupId, SpaceGroupMemberCreateRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}/members'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceGroupMemberView.fromJson(map);
    })();
  }

  /// Get spaces groups members
  Future<SpaceGroupMemberView?> groupsMembersGet(String spaceId, String groupId, String userId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}/members/${serializePathParameter(userId, const PathParameterSpec('userId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceGroupMemberView.fromJson(map);
    })();
  }

  /// Update spaces groups members
  Future<void> groupsMembersUpdate(String spaceId, String groupId, String userId, SpaceGroupMemberUpdateRequest body) async {
    final payload = body.toJson();
    await _client.patch(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}/members/${serializePathParameter(userId, const PathParameterSpec('userId', 'simple', false))}'), body: payload, contentType: 'application/json');
  }

  /// Delete spaces groups members
  Future<void> groupsMembersDelete(String spaceId, String groupId, String userId) async {
    await _client.delete(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}/members/${serializePathParameter(userId, const PathParameterSpec('userId', 'simple', false))}'));
  }

  /// List spaces channels
  Future<SpaceChannelListResponse?> channelsList(String spaceId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/channels'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceChannelListResponse.fromJson(map);
    })();
  }

  /// Create spaces channels
  Future<SpaceChannelView?> channelsCreate(String spaceId, SpaceChannelCreateRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/channels'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceChannelView.fromJson(map);
    })();
  }

  /// Get spaces channels
  Future<SpaceChannelView?> channelsGet(String spaceId, String channelId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/channels/${serializePathParameter(channelId, const PathParameterSpec('channelId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceChannelView.fromJson(map);
    })();
  }

  /// Update spaces channels
  Future<SpaceChannelView?> channelsUpdate(String spaceId, String channelId, SpaceChannelUpdateRequest body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/channels/${serializePathParameter(channelId, const PathParameterSpec('channelId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceChannelView.fromJson(map);
    })();
  }

  /// Delete spaces channels
  Future<void> channelsDelete(String spaceId, String channelId) async {
    await _client.delete(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/channels/${serializePathParameter(channelId, const PathParameterSpec('channelId', 'simple', false))}'));
  }

  /// List spaces channels access Rules
  Future<SpaceChannelAccessRuleListResponse?> channelsAccessRulesList(String spaceId, String channelId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/channels/${serializePathParameter(channelId, const PathParameterSpec('channelId', 'simple', false))}/access_rules'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceChannelAccessRuleListResponse.fromJson(map);
    })();
  }

  /// Create spaces channels access Rules
  Future<SpaceChannelAccessRuleView?> channelsAccessRulesCreate(String spaceId, String channelId, SpaceChannelAccessRuleCreateRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/channels/${serializePathParameter(channelId, const PathParameterSpec('channelId', 'simple', false))}/access_rules'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceChannelAccessRuleView.fromJson(map);
    })();
  }

  /// Delete spaces channels access Rules
  Future<void> channelsAccessRulesDelete(String spaceId, String channelId, String ruleId) async {
    await _client.delete(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/channels/${serializePathParameter(channelId, const PathParameterSpec('channelId', 'simple', false))}/access_rules/${serializePathParameter(ruleId, const PathParameterSpec('ruleId', 'simple', false))}'));
  }

  /// List spaces invites
  Future<SpaceInviteListResponse?> invitesList(String spaceId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/invites'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceInviteListResponse.fromJson(map);
    })();
  }

  /// Create spaces invites
  Future<SpaceInviteView?> invitesCreate(String spaceId, SpaceInviteCreateRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/invites'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceInviteView.fromJson(map);
    })();
  }

  /// Get spaces invites
  Future<SpaceInviteView?> invitesGet(String spaceId, String inviteCode) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/invites/${serializePathParameter(inviteCode, const PathParameterSpec('inviteCode', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceInviteView.fromJson(map);
    })();
  }

  /// Revoke spaces invites
  Future<void> invitesRevoke(String spaceId, String inviteCode) async {
    await _client.delete(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/invites/${serializePathParameter(inviteCode, const PathParameterSpec('inviteCode', 'simple', false))}'));
  }

  /// Accept spaces invites
  Future<void> invitesAccept(String spaceId, String inviteCode) async {
    await _client.post(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/invites/${serializePathParameter(inviteCode, const PathParameterSpec('inviteCode', 'simple', false))}/accept'));
  }

  /// List spaces bans
  Future<SpaceBanListResponse?> bansList(String spaceId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/bans'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceBanListResponse.fromJson(map);
    })();
  }

  /// Create spaces bans
  Future<SpaceBanView?> bansCreate(String spaceId, SpaceBanCreateRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/bans'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceBanView.fromJson(map);
    })();
  }

  /// Get spaces bans
  Future<SpaceBanView?> bansGet(String spaceId, String userId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/bans/${serializePathParameter(userId, const PathParameterSpec('userId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpaceBanView.fromJson(map);
    })();
  }

  /// Delete spaces bans
  Future<void> bansDelete(String spaceId, String userId) async {
    await _client.delete(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/bans/${serializePathParameter(userId, const PathParameterSpec('userId', 'simple', false))}'));
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
