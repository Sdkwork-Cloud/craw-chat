import 'dart:convert';
import '../http/client.dart';
import '../models.dart';

import 'paths.dart';
import 'response_helpers.dart';


class SocialApi {
  final HttpClient _client;

  SocialApi(this._client);

  /// Search social users
  Future<SocialUsersListResponse?> usersList([String? q, int? pageSize, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('q', q, 'form', true, false, null),
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/social/users'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialUsersListResponse.fromJson(map);
    })();
  }

  /// List friend requests
  Future<SdkWorkListResponse?> friendRequestsList([String? direction, String? status, int? pageSize, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('direction', direction, 'form', true, false, null),
      QueryParameterSpec('status', status, 'form', true, false, null),
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/social/friend_requests'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// Create a friend request
  Future<SocialFriendRequestsCreateResponse201?> friendRequestsCreate(SubmitFriendRequestRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/social/friend_requests'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendRequestsCreateResponse201.fromJson(map);
    })();
  }

  /// Retrieve pending incoming friend request count
  Future<SocialFriendRequestsPendingCountRetrieveResponse?> friendRequestsPendingCountRetrieve() async {
    final response = await _client.get(ApiPaths.imPath('/social/friend_requests/pending/count'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendRequestsPendingCountRetrieveResponse.fromJson(map);
    })();
  }

  /// Accept a friend request
  Future<SocialFriendRequestsAcceptResponse?> friendRequestsAccept(String friendRequestId) async {
    final response = await _client.post(ApiPaths.imPath('/social/friend_requests/${serializePathParameter(friendRequestId, const PathParameterSpec('friendRequestId', 'simple', false))}/accept'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendRequestsAcceptResponse.fromJson(map);
    })();
  }

  /// Decline a friend request
  Future<SocialFriendRequestsDeclineResponse?> friendRequestsDecline(String friendRequestId) async {
    final response = await _client.post(ApiPaths.imPath('/social/friend_requests/${serializePathParameter(friendRequestId, const PathParameterSpec('friendRequestId', 'simple', false))}/decline'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendRequestsDeclineResponse.fromJson(map);
    })();
  }

  /// Cancel a friend request
  Future<SocialFriendRequestsCancelResponse?> friendRequestsCancel(String friendRequestId) async {
    final response = await _client.post(ApiPaths.imPath('/social/friend_requests/${serializePathParameter(friendRequestId, const PathParameterSpec('friendRequestId', 'simple', false))}/cancel'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendRequestsCancelResponse.fromJson(map);
    })();
  }

  /// Remove a friendship
  Future<SocialFriendshipsRemoveResponse?> friendshipsRemove(String friendshipId) async {
    final response = await _client.post(ApiPaths.imPath('/social/friendships/${serializePathParameter(friendshipId, const PathParameterSpec('friendshipId', 'simple', false))}/remove'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendshipsRemoveResponse.fromJson(map);
    })();
  }

  /// Block a social user
  Future<SocialUserBlocksCreateResponse201?> userBlocksCreate(BlockUserRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/social/user_blocks'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialUserBlocksCreateResponse201.fromJson(map);
    })();
  }

  /// Release a social user block
  Future<void> userBlocksDelete(String blockId) async {
    await _client.delete(ApiPaths.imPath('/social/user_blocks/${serializePathParameter(blockId, const PathParameterSpec('blockId', 'simple', false))}'));
  }

  /// List contact tags
  Future<SdkWorkListResponse?> contactsTagsList([int? pageSize, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/social/contacts/tags'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// Create a contact tag
  Future<SocialContactsTagsCreateResponse201?> contactsTagsCreate(CreateContactTagRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/social/contacts/tags'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialContactsTagsCreateResponse201.fromJson(map);
    })();
  }

  /// Update a contact tag
  Future<SocialContactsTagsUpdateResponse?> contactsTagsUpdate(String tagId, UpdateContactTagRequest body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.imPath('/social/contacts/tags/${serializePathParameter(tagId, const PathParameterSpec('tagId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialContactsTagsUpdateResponse.fromJson(map);
    })();
  }

  /// Delete a contact tag
  Future<void> contactsTagsDelete(String tagId) async {
    await _client.delete(ApiPaths.imPath('/social/contacts/tags/${serializePathParameter(tagId, const PathParameterSpec('tagId', 'simple', false))}'));
  }

  /// Create a contact recommendation
  Future<SocialContactsRecommendationsCreateResponse201?> contactsRecommendationsCreate(String targetUserId, CreateContactRecommendationRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/social/contacts/${serializePathParameter(targetUserId, const PathParameterSpec('targetUserId', 'simple', false))}/recommendations'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialContactsRecommendationsCreateResponse201.fromJson(map);
    })();
  }

  /// Retrieve contact preferences
  Future<SocialContactsPreferencesRetrieveResponse?> contactsPreferencesRetrieve(String targetUserId) async {
    final response = await _client.get(ApiPaths.imPath('/social/contacts/${serializePathParameter(targetUserId, const PathParameterSpec('targetUserId', 'simple', false))}/preferences'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialContactsPreferencesRetrieveResponse.fromJson(map);
    })();
  }

  /// Update contact preferences
  Future<SocialContactsPreferencesUpdateResponse?> contactsPreferencesUpdate(String targetUserId, UpdateContactPreferencesRequest body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.imPath('/social/contacts/${serializePathParameter(targetUserId, const PathParameterSpec('targetUserId', 'simple', false))}/preferences'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialContactsPreferencesUpdateResponse.fromJson(map);
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
