import '../http/client.dart';
import '../models.dart';

import 'paths.dart';
import 'response_helpers.dart';


class AutomationApi {
  final HttpClient _client;

  AutomationApi(this._client);

  /// Retrieve automation governance
  Future<GovernanceRetrieveResponse?> governanceRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/automation/governance'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : GovernanceRetrieveResponse.fromJson(map);
    })();
  }
}
