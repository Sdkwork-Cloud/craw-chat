import '../http/client.dart';

import 'paths.dart';


class OpsApi {
  final HttpClient _client;

  OpsApi(this._client);

  /// Retrieve ops health
  Future<Map<String, dynamic>?> healthRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/ops/health'));
    return response;
  }

  /// Retrieve cluster state
  Future<Map<String, dynamic>?> clusterRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/ops/cluster'));
    return response;
  }

  /// Retrieve projection lag
  Future<Map<String, dynamic>?> lagRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/ops/lag'));
    return response;
  }

  /// Retrieve replay status
  Future<Map<String, dynamic>?> replayStatusRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/ops/replay_status'));
    return response;
  }

  /// Retrieve commercial readiness
  Future<Map<String, dynamic>?> commercialReadinessRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/ops/commercial_readiness'));
    return response;
  }

  /// Inspect runtime directory
  Future<Map<String, dynamic>?> runtimeDirRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/ops/runtime_dir'));
    return response;
  }

  /// List provider bindings
  Future<Map<String, dynamic>?> providerBindingsList() async {
    final response = await _client.get(ApiPaths.backendPath('/ops/provider_bindings'));
    return response;
  }

  /// Retrieve provider binding drift
  Future<Map<String, dynamic>?> providerBindingsDriftRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/ops/provider_bindings/drift'));
    return response;
  }

  /// Retrieve diagnostics
  Future<Map<String, dynamic>?> diagnosticsRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/ops/diagnostics'));
    return response;
  }
}
