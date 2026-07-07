import '../http/client.dart';
import '../models.dart';

import 'paths.dart';
import 'response_helpers.dart';


class PortalApi {
  final HttpClient _client;

  PortalApi(this._client);

  /// Read the tenant portal sign-in snapshot
  Future<AccessRetrieveResponse?> accessRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/portal/access'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : AccessRetrieveResponse.fromJson(map);
    })();
  }

  /// Read the tenant automation snapshot
  Future<AutomationRetrieveResponse?> automationRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/portal/automation'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : AutomationRetrieveResponse.fromJson(map);
    })();
  }

  /// Read the tenant conversations snapshot
  Future<ConversationSnapshotRetrieveResponse?> conversationSnapshotRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/portal/conversations'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationSnapshotRetrieveResponse.fromJson(map);
    })();
  }

  /// Read the tenant dashboard snapshot
  Future<DashboardRetrieveResponse?> dashboardRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/portal/dashboard'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : DashboardRetrieveResponse.fromJson(map);
    })();
  }

  /// Read the tenant governance snapshot
  Future<GovernanceRetrieveResponse?> governanceRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/portal/governance'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : GovernanceRetrieveResponse.fromJson(map);
    })();
  }

  /// Read the tenant portal home snapshot
  Future<HomeRetrieveResponse?> homeRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/portal/home'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : HomeRetrieveResponse.fromJson(map);
    })();
  }

  /// Read the tenant media snapshot
  Future<MediaRetrieveResponse?> mediaRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/portal/media'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MediaRetrieveResponse.fromJson(map);
    })();
  }

  /// Read the tenant realtime snapshot
  Future<RealtimeRetrieveResponse?> realtimeRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/portal/realtime'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RealtimeRetrieveResponse.fromJson(map);
    })();
  }

  /// Read the current tenant workspace snapshot
  Future<WorkspaceRetrieveResponse?> workspaceRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/portal/workspace'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : WorkspaceRetrieveResponse.fromJson(map);
    })();
  }
}
