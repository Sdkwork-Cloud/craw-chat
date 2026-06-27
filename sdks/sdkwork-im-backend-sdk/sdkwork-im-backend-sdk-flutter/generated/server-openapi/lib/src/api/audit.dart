import '../http/client.dart';

import 'paths.dart';


class AuditApi {
  final HttpClient _client;

  AuditApi(this._client);

  /// List audit records
  Future<Map<String, dynamic>?> recordsList() async {
    final response = await _client.get(ApiPaths.backendPath('/audit/records'));
    return response;
  }

  /// Record audit anchor
  Future<Map<String, dynamic>?> recordsCreate() async {
    final response = await _client.post(ApiPaths.backendPath('/audit/records'));
    return response;
  }

  /// Export audit bundle
  Future<Map<String, dynamic>?> exportRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/audit/export'));
    return response;
  }
}
