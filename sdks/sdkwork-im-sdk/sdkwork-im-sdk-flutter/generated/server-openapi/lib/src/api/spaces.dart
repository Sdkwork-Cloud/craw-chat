import 'dart:convert';
import '../http/client.dart';
import '../models.dart';

import 'paths.dart';
import 'response_helpers.dart';


class SpacesApi {
  final HttpClient _client;

  SpacesApi(this._client);

  /// Create a space
  Future<SpacesCreateResponse201?> create(SpaceCreateRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/spaces'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesCreateResponse201.fromJson(map);
    })();
  }

  /// List spaces
  Future<SpacesListResponse?> list([int? pageSize, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/spaces'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesListResponse.fromJson(map);
    })();
  }

  /// Retrieve a space
  Future<SpacesRetrieveResponse?> retrieve(String spaceId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesRetrieveResponse.fromJson(map);
    })();
  }

  /// Update a space
  Future<SpacesUpdateResponse?> update(String spaceId, SpaceUpdateRequest body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesUpdateResponse.fromJson(map);
    })();
  }

  /// Delete a space
  Future<void> delete(String spaceId) async {
    await _client.delete(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}'));
  }

  /// List spaces members
  Future<SpacesMembersListResponse?> membersList(String spaceId, [int? pageSize, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/members'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesMembersListResponse.fromJson(map);
    })();
  }

  /// Create spaces members
  Future<SpacesMembersCreateResponse201?> membersCreate(String spaceId, SpaceMemberCreateRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/members'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesMembersCreateResponse201.fromJson(map);
    })();
  }

  /// retrieve spaces members
  Future<SpacesMembersRetrieveResponse?> membersRetrieve(String spaceId, String userId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/members/${serializePathParameter(userId, const PathParameterSpec('userId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesMembersRetrieveResponse.fromJson(map);
    })();
  }

  /// Update spaces members
  Future<SpacesMembersUpdateResponse?> membersUpdate(String spaceId, String userId, SpaceMemberUpdateRequest body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/members/${serializePathParameter(userId, const PathParameterSpec('userId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesMembersUpdateResponse.fromJson(map);
    })();
  }

  /// Delete spaces members
  Future<void> membersDelete(String spaceId, String userId) async {
    await _client.delete(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/members/${serializePathParameter(userId, const PathParameterSpec('userId', 'simple', false))}'));
  }

  /// List spaces groups
  Future<SpacesGroupsListResponse?> groupsList(String spaceId, [int? pageSize, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesGroupsListResponse.fromJson(map);
    })();
  }

  /// Create spaces groups
  Future<SpacesGroupsCreateResponse201?> groupsCreate(String spaceId, SpaceGroupCreateRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesGroupsCreateResponse201.fromJson(map);
    })();
  }

  /// retrieve spaces groups
  Future<SpacesGroupsRetrieveResponse?> groupsRetrieve(String spaceId, String groupId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesGroupsRetrieveResponse.fromJson(map);
    })();
  }

  /// Update spaces groups
  Future<SpacesGroupsUpdateResponse?> groupsUpdate(String spaceId, String groupId, SpaceGroupUpdateRequest body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesGroupsUpdateResponse.fromJson(map);
    })();
  }

  /// Delete spaces groups
  Future<void> groupsDelete(String spaceId, String groupId) async {
    await _client.delete(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}'));
  }

  /// List spaces groups members
  Future<SpacesGroupsMembersListResponse?> groupsMembersList(String spaceId, String groupId, [int? pageSize, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}/members'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesGroupsMembersListResponse.fromJson(map);
    })();
  }

  /// Create spaces groups members
  Future<SpacesGroupsMembersCreateResponse201?> groupsMembersCreate(String spaceId, String groupId, SpaceGroupMemberCreateRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}/members'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesGroupsMembersCreateResponse201.fromJson(map);
    })();
  }

  /// retrieve spaces groups members
  Future<SpacesGroupsMembersRetrieveResponse?> groupsMembersRetrieve(String spaceId, String groupId, String userId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}/members/${serializePathParameter(userId, const PathParameterSpec('userId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesGroupsMembersRetrieveResponse.fromJson(map);
    })();
  }

  /// Update spaces groups members
  Future<SpacesGroupsMembersUpdateResponse?> groupsMembersUpdate(String spaceId, String groupId, String userId, SpaceGroupMemberUpdateRequest body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}/members/${serializePathParameter(userId, const PathParameterSpec('userId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesGroupsMembersUpdateResponse.fromJson(map);
    })();
  }

  /// Delete spaces groups members
  Future<void> groupsMembersDelete(String spaceId, String groupId, String userId) async {
    await _client.delete(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}/members/${serializePathParameter(userId, const PathParameterSpec('userId', 'simple', false))}'));
  }

  /// List spaces channels
  Future<SpacesChannelsListResponse?> channelsList(String spaceId, [int? pageSize, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/channels'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesChannelsListResponse.fromJson(map);
    })();
  }

  /// Create spaces channels
  Future<SpacesChannelsCreateResponse201?> channelsCreate(String spaceId, SpaceChannelCreateRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/channels'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesChannelsCreateResponse201.fromJson(map);
    })();
  }

  /// retrieve spaces channels
  Future<SpacesChannelsRetrieveResponse?> channelsRetrieve(String spaceId, String channelId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/channels/${serializePathParameter(channelId, const PathParameterSpec('channelId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesChannelsRetrieveResponse.fromJson(map);
    })();
  }

  /// Update spaces channels
  Future<SpacesChannelsUpdateResponse?> channelsUpdate(String spaceId, String channelId, SpaceChannelUpdateRequest body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/channels/${serializePathParameter(channelId, const PathParameterSpec('channelId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesChannelsUpdateResponse.fromJson(map);
    })();
  }

  /// Delete spaces channels
  Future<void> channelsDelete(String spaceId, String channelId) async {
    await _client.delete(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/channels/${serializePathParameter(channelId, const PathParameterSpec('channelId', 'simple', false))}'));
  }

  /// List spaces channels access Rules
  Future<SpacesChannelsAccessRulesListResponse?> channelsAccessRulesList(String spaceId, String channelId, [int? pageSize, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/channels/${serializePathParameter(channelId, const PathParameterSpec('channelId', 'simple', false))}/access_rules'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesChannelsAccessRulesListResponse.fromJson(map);
    })();
  }

  /// Create spaces channels access Rules
  Future<SpacesChannelsAccessRulesCreateResponse201?> channelsAccessRulesCreate(String spaceId, String channelId, SpaceChannelAccessRuleCreateRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/channels/${serializePathParameter(channelId, const PathParameterSpec('channelId', 'simple', false))}/access_rules'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesChannelsAccessRulesCreateResponse201.fromJson(map);
    })();
  }

  /// Delete spaces channels access Rules
  Future<void> channelsAccessRulesDelete(String spaceId, String channelId, String ruleId) async {
    await _client.delete(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/channels/${serializePathParameter(channelId, const PathParameterSpec('channelId', 'simple', false))}/access_rules/${serializePathParameter(ruleId, const PathParameterSpec('ruleId', 'simple', false))}'));
  }

  /// List spaces invites
  Future<SpacesInvitesListResponse?> invitesList(String spaceId, [String? status, int? pageSize, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('status', status, 'form', true, false, null),
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/invites'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesInvitesListResponse.fromJson(map);
    })();
  }

  /// Create spaces invites
  Future<SpacesInvitesCreateResponse201?> invitesCreate(String spaceId, SpaceInviteCreateRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/invites'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesInvitesCreateResponse201.fromJson(map);
    })();
  }

  /// retrieve spaces invites
  Future<SpacesInvitesRetrieveResponse?> invitesRetrieve(String spaceId, String inviteCode) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/invites/${serializePathParameter(inviteCode, const PathParameterSpec('inviteCode', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesInvitesRetrieveResponse.fromJson(map);
    })();
  }

  /// Delete spaces invites
  Future<void> invitesDelete(String spaceId, String inviteCode) async {
    await _client.delete(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/invites/${serializePathParameter(inviteCode, const PathParameterSpec('inviteCode', 'simple', false))}'));
  }

  /// Accept spaces invites
  Future<SdkWorkCommandResponse?> invitesAccept(String spaceId, String inviteCode) async {
    final response = await _client.post(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/invites/${serializePathParameter(inviteCode, const PathParameterSpec('inviteCode', 'simple', false))}/accept'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkCommandResponse.fromJson(map);
    })();
  }

  /// List spaces bans
  Future<SpacesBansListResponse?> bansList(String spaceId, [int? pageSize, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/bans'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesBansListResponse.fromJson(map);
    })();
  }

  /// Create spaces bans
  Future<SpacesBansCreateResponse201?> bansCreate(String spaceId, SpaceBanCreateRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/bans'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesBansCreateResponse201.fromJson(map);
    })();
  }

  /// retrieve spaces bans
  Future<SpacesBansRetrieveResponse?> bansRetrieve(String spaceId, String userId) async {
    final response = await _client.get(ApiPaths.imPath('/spaces/${serializePathParameter(spaceId, const PathParameterSpec('spaceId', 'simple', false))}/bans/${serializePathParameter(userId, const PathParameterSpec('userId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SpacesBansRetrieveResponse.fromJson(map);
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
