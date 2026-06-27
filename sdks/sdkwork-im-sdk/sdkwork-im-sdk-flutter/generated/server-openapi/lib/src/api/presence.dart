import '../http/client.dart';
import '../models.dart';

import 'paths.dart';
import 'response_helpers.dart';


class PresenceApi {
  final HttpClient _client;

  PresenceApi(this._client);

  /// Publish current client route presence heartbeat
  Future<PresenceView?> heartbeatCreate(PresenceHeartbeatRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.imPath('/presence/heartbeat'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : PresenceView.fromJson(map);
    })();
  }

  /// Retrieve current principal presence
  Future<PresenceView?> meRetrieve() async {
    final response = await _client.get(ApiPaths.imPath('/presence/me'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : PresenceView.fromJson(map);
    })();
  }
}
