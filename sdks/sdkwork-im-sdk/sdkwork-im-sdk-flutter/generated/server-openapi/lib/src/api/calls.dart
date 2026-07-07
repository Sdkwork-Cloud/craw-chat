import '../http/client.dart';
import '../models.dart';

import 'paths.dart';
import 'response_helpers.dart';


class CallsApi {
  final HttpClient _client;

  CallsApi(this._client);

  /// Create an IM call signaling session
  Future<CallsSessionsCreateResponse201?> sessionsCreate(CreateRtcSessionRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/calls/sessions'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : CallsSessionsCreateResponse201.fromJson(map);
    })();
  }

  /// Retrieve IM call signaling session state
  Future<CallsSessionsRetrieveResponse?> sessionsRetrieve(String rtcSessionId) async {
    final response = await _client.get(ApiPaths.imPath('/calls/sessions/${serializePathParameter(rtcSessionId, const PathParameterSpec('rtcSessionId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : CallsSessionsRetrieveResponse.fromJson(map);
    })();
  }

  /// Invite participants into an IM call signaling session
  Future<CallsSessionsInviteResponse?> sessionsInvite(String rtcSessionId, InviteRtcSessionRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/calls/sessions/${serializePathParameter(rtcSessionId, const PathParameterSpec('rtcSessionId', 'simple', false))}/invite'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : CallsSessionsInviteResponse.fromJson(map);
    })();
  }

  /// Accept an IM call signaling session
  Future<CallsSessionsAcceptResponse?> sessionsAccept(String rtcSessionId, UpdateRtcSessionRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/calls/sessions/${serializePathParameter(rtcSessionId, const PathParameterSpec('rtcSessionId', 'simple', false))}/accept'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : CallsSessionsAcceptResponse.fromJson(map);
    })();
  }

  /// Reject an IM call signaling session
  Future<CallsSessionsRejectResponse?> sessionsReject(String rtcSessionId, UpdateRtcSessionRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/calls/sessions/${serializePathParameter(rtcSessionId, const PathParameterSpec('rtcSessionId', 'simple', false))}/reject'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : CallsSessionsRejectResponse.fromJson(map);
    })();
  }

  /// End an IM call signaling session
  Future<CallsSessionsEndResponse?> sessionsEnd(String rtcSessionId, UpdateRtcSessionRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/calls/sessions/${serializePathParameter(rtcSessionId, const PathParameterSpec('rtcSessionId', 'simple', false))}/end'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : CallsSessionsEndResponse.fromJson(map);
    })();
  }

  /// Post an IM call signaling event
  Future<CallsSessionsSignalsCreateResponse201?> sessionsSignalsCreate(String rtcSessionId, PostRtcSignalRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/calls/sessions/${serializePathParameter(rtcSessionId, const PathParameterSpec('rtcSessionId', 'simple', false))}/signals'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : CallsSessionsSignalsCreateResponse201.fromJson(map);
    })();
  }

  /// Issue an RTC media participant credential for an IM call
  Future<CallsSessionsCredentialsCreateResponse201?> sessionsCredentialsCreate(String rtcSessionId, IssueRtcParticipantCredentialRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/calls/sessions/${serializePathParameter(rtcSessionId, const PathParameterSpec('rtcSessionId', 'simple', false))}/credentials'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : CallsSessionsCredentialsCreateResponse201.fromJson(map);
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
