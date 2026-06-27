import '../http/client.dart';

import 'paths.dart';


class AutomationApi {
  final HttpClient _client;

  AutomationApi(this._client);

  /// Retrieve automation governance
  Future<Map<String, dynamic>?> governanceRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/automation/governance'));
    return response;
  }
}
