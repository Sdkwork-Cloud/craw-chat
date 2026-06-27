import 'dart:convert';
import '../http/client.dart';
import '../models.dart';

import 'paths.dart';
import 'response_helpers.dart';


class SocialApi {
  final HttpClient _client;

  SocialApi(this._client);

  /// Search social users
  Future<SocialUserSearchResponse?> usersList([String? q, int? limit, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('q', q, 'form', true, false, null),
      QueryParameterSpec('limit', limit, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/social/users'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialUserSearchResponse.fromJson(map);
    })();
  }

  /// List friend requests
  Future<SocialFriendRequestListResponse?> friendRequestsList([String? direction, String? status, int? limit, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('direction', direction, 'form', true, false, null),
      QueryParameterSpec('status', status, 'form', true, false, null),
      QueryParameterSpec('limit', limit, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/social/friend_requests'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendRequestListResponse.fromJson(map);
    })();
  }

  /// Create a friend request
  Future<SocialFriendRequestMutationResponse?> friendRequestsCreate(SubmitFriendRequestRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/social/friend_requests'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendRequestMutationResponse.fromJson(map);
    })();
  }

  /// Accept a friend request
  Future<SocialFriendRequestAcceptanceResponse?> friendRequestsAccept(String requestId) async {
    final response = await _client.post(ApiPaths.imPath('/social/friend_requests/${serializePathParameter(requestId, const PathParameterSpec('requestId', 'simple', false))}/accept'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendRequestAcceptanceResponse.fromJson(map);
    })();
  }

  /// Decline a friend request
  Future<SocialFriendRequestMutationResponse?> friendRequestsDecline(String requestId) async {
    final response = await _client.post(ApiPaths.imPath('/social/friend_requests/${serializePathParameter(requestId, const PathParameterSpec('requestId', 'simple', false))}/decline'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendRequestMutationResponse.fromJson(map);
    })();
  }

  /// Cancel a friend request
  Future<SocialFriendRequestMutationResponse?> friendRequestsCancel(String requestId) async {
    final response = await _client.post(ApiPaths.imPath('/social/friend_requests/${serializePathParameter(requestId, const PathParameterSpec('requestId', 'simple', false))}/cancel'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendRequestMutationResponse.fromJson(map);
    })();
  }

  /// Remove a friendship
  Future<SocialFriendshipMutationResponse?> friendshipsRemove(String friendshipId) async {
    final response = await _client.post(ApiPaths.imPath('/social/friendships/${serializePathParameter(friendshipId, const PathParameterSpec('friendshipId', 'simple', false))}/remove'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SocialFriendshipMutationResponse.fromJson(map);
    })();
  }

  /// List contact tags
  Future<ContactTagsResponse?> contactsTagsList([int? limit, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('limit', limit, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.imPath('/social/contacts/tags'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ContactTagsResponse.fromJson(map);
    })();
  }

  /// Create a contact tag
  Future<ContactTagView?> contactsTagsCreate(CreateContactTagRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/social/contacts/tags'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ContactTagView.fromJson(map);
    })();
  }

  /// Update a contact tag
  Future<ContactTagView?> contactsTagsUpdate(String tagId, UpdateContactTagRequest body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.imPath('/social/contacts/tags/${serializePathParameter(tagId, const PathParameterSpec('tagId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ContactTagView.fromJson(map);
    })();
  }

  /// Delete a contact tag
  Future<DeleteContactTagResponse?> contactsTagsDelete(String tagId) async {
    final response = await _client.delete(ApiPaths.imPath('/social/contacts/tags/${serializePathParameter(tagId, const PathParameterSpec('tagId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : DeleteContactTagResponse.fromJson(map);
    })();
  }

  /// Create a contact recommendation
  Future<ContactRecommendationView?> contactsRecommendationsCreate(String targetUserId, CreateContactRecommendationRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/social/contacts/${serializePathParameter(targetUserId, const PathParameterSpec('targetUserId', 'simple', false))}/recommendations'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ContactRecommendationView.fromJson(map);
    })();
  }

  /// Retrieve contact preferences
  Future<ContactPreferencesView?> contactsPreferencesRetrieve(String targetUserId) async {
    final response = await _client.get(ApiPaths.imPath('/social/contacts/${serializePathParameter(targetUserId, const PathParameterSpec('targetUserId', 'simple', false))}/preferences'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ContactPreferencesView.fromJson(map);
    })();
  }

  /// Update contact preferences
  Future<ContactPreferencesView?> contactsPreferencesUpdate(String targetUserId, UpdateContactPreferencesRequest body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.imPath('/social/contacts/${serializePathParameter(targetUserId, const PathParameterSpec('targetUserId', 'simple', false))}/preferences'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ContactPreferencesView.fromJson(map);
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
