import '../http/client.dart';
import '../models.dart';

import 'paths.dart';
import 'response_helpers.dart';


class PortalApi {
  final HttpClient _client;

  PortalApi(this._client);

  /// Read the tenant portal sign-in snapshot
  Future<Map<String, dynamic>?> accessRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/portal/access'));
    return response;
  }

  /// Read the tenant automation snapshot
  Future<Map<String, dynamic>?> automationRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/portal/automation'));
    return response;
  }

  /// Read the tenant conversations snapshot
  Future<Map<String, dynamic>?> conversationSnapshotRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/portal/conversations'));
    return response;
  }

  /// Read the tenant dashboard snapshot
  Future<Map<String, dynamic>?> dashboardRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/portal/dashboard'));
    return response;
  }

  /// Read the tenant governance snapshot
  Future<Map<String, dynamic>?> governanceRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/portal/governance'));
    return response;
  }

  /// Read the tenant portal home snapshot
  Future<Map<String, dynamic>?> homeRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/portal/home'));
    return response;
  }

  /// Read the tenant media snapshot
  Future<Map<String, dynamic>?> mediaRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/portal/media'));
    return response;
  }

  /// Read the tenant realtime snapshot
  Future<Map<String, dynamic>?> realtimeRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/portal/realtime'));
    return response;
  }

  /// Read the current tenant workspace snapshot
  Future<PortalWorkspaceView?> workspaceRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/portal/workspace'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : PortalWorkspaceView.fromJson(map);
    })();
  }
}
