import '../http/client.dart';

import 'paths.dart';


class ProviderApi {
  final HttpClient _client;

  ProviderApi(this._client);

  /// Retrieve media provider health
  Future<Map<String, dynamic>?> mediaHealthRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/media/provider_health'));
    return response;
  }

  /// Retrieve principal-profile provider health
  Future<Map<String, dynamic>?> principalProfileHealthRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/principal/profiles/provider_health'));
    return response;
  }
}
