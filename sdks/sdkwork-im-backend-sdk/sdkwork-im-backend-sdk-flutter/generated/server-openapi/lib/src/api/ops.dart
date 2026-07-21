import 'dart:convert';
import '../http/client.dart';
import '../models.dart';

import 'paths.dart';
import 'response_helpers.dart';


class OpsApi {
  final HttpClient _client;

  OpsApi(this._client);

  /// Retrieve ops health
  Future<HealthRetrieveResponse?> healthRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/ops/health'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : HealthRetrieveResponse.fromJson(map);
    })();
  }

  /// Retrieve cluster state
  Future<ClusterRetrieveResponse?> clusterRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/ops/cluster'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ClusterRetrieveResponse.fromJson(map);
    })();
  }

  /// Retrieve projection lag
  Future<LagListResponse?> lagRetrieve([int? pageSize, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/ops/lag'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : LagListResponse.fromJson(map);
    })();
  }

  /// Retrieve replay status
  Future<ReplayStatusRetrieveResponse?> replayStatusRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/ops/replay_status'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ReplayStatusRetrieveResponse.fromJson(map);
    })();
  }

  /// Retrieve commercial readiness
  Future<CommercialReadinessRetrieveResponse?> commercialReadinessRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/ops/commercial_readiness'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : CommercialReadinessRetrieveResponse.fromJson(map);
    })();
  }

  /// Inspect runtime directory
  Future<RuntimeDirRetrieveResponse?> runtimeDirRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/ops/runtime_dir'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RuntimeDirRetrieveResponse.fromJson(map);
    })();
  }

  /// List provider bindings
  Future<ProviderBindingSnapshotListResponse?> providerBindingsList([int? pageSize, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/ops/provider_bindings'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ProviderBindingSnapshotListResponse.fromJson(map);
    })();
  }

  /// Retrieve provider binding drift
  Future<ProviderBindingDriftListResponse?> providerBindingsDriftRetrieve([int? pageSize, String? cursor]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/ops/provider_bindings/drift'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ProviderBindingDriftListResponse.fromJson(map);
    })();
  }

  /// Retrieve diagnostics
  Future<DiagnosticsRetrieveResponse?> diagnosticsRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/ops/diagnostics'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : DiagnosticsRetrieveResponse.fromJson(map);
    })();
  }
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
