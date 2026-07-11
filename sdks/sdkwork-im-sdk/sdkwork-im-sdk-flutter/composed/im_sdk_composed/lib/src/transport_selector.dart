/// Transport selection policy implementation.
///
/// Mirrors the TypeScript `transport-selector.ts`. Selects the optimal
/// transport based on runtime capabilities, with support for:
/// - Auto-detection of available transports
/// - Priority-based selection with automatic fallback
/// - Manual override (caller forces a specific transport kind)
/// - URL conversion from a base URL to a transport-specific endpoint
library;

import 'transport.dart';
import 'transports/tcp_transport.dart';
import 'transports/udp_transport.dart';
import 'transports/websocket_transport.dart';
import 'ccp_wire.dart';

/// Detects available transport kinds from the given factory map.
///
/// Native Dart/Flutter: websocket + tcp + udp.
/// Browser-like: websocket only (if `dart:io` is unavailable).
List<ImTransportKind> detectAvailableTransports(
  Map<ImTransportKind, ImTransportFactory> factories,
) {
  final available = <ImTransportKind>[];
  for (final kind in [
    ImTransportKind.websocket,
    ImTransportKind.tcp,
    ImTransportKind.udp,
  ]) {
    final factory = factories[kind];
    if (factory?.isAvailable() ?? false) {
      available.add(kind);
    }
  }
  return available;
}

/// Selects a transport factory.
///
/// Selection logic:
/// 1. If [preferred] is specified and available, use it directly.
/// 2. Otherwise, try each kind in [ImTransportSelectionPolicy.preferred] order.
/// 3. If `autoFallback` is false and the preferred kind is unavailable, throw.
/// 4. If all are unavailable, throw.
ImTransportFactory selectTransportFactory(
  Map<ImTransportKind, ImTransportFactory> factories, [
  ImTransportSelectionPolicy policy = defaultTransportSelectionPolicy,
  ImTransportKind? preferred,
]) {
  if (preferred != null) {
    final factory = factories[preferred];
    if (factory != null && factory.isAvailable()) {
      return factory;
    }
    if (!policy.autoFallback) {
      throw StateError(
        'Preferred transport "$preferred" is not available in the current environment.',
      );
    }
  }
  for (final kind in policy.preferred) {
    final factory = factories[kind];
    if (factory != null && factory.isAvailable()) {
      return factory;
    }
  }
  throw StateError(
    'No transport is available in the current environment. '
    'Provide a custom ImTransportFactory or run in a supported runtime.',
  );
}

/// Builds a transport endpoint from a base URL and transport kind.
///
/// - websocket: `ws://host:port/im/v3/api/realtime/ws`
/// - tcp: `tcp://host:port`
/// - udp: `udp://host:port`
ImTransportEndpoint buildTransportEndpoint(
  String baseUrl,
  ImTransportKind kind, {
  String? deviceId,
}) {
  final urlKind = parseTransportKindFromUrl(baseUrl);

  // If the URL already specifies the matching transport kind, use it directly.
  if (urlKind == kind && kind != ImTransportKind.websocket) {
    return ImTransportEndpoint(
      kind: kind,
      url: baseUrl,
      deviceId: deviceId,
      protocols: kind == ImTransportKind.websocket
          ? [imCcpWebSocketSubprotocol]
          : null,
    );
  }

  // Otherwise, convert the base URL to the transport-specific URL.
  final url = _convertBaseUrlForTransport(baseUrl, kind, deviceId);
  return ImTransportEndpoint(
    kind: kind,
    url: url,
    deviceId: deviceId,
    protocols: kind == ImTransportKind.websocket
        ? [imCcpWebSocketSubprotocol]
        : null,
  );
}

String _convertBaseUrlForTransport(
  String baseUrl,
  ImTransportKind kind,
  String? deviceId,
) {
  final trimmed = baseUrl.trim().replaceAll(RegExp(r'/+$'), '');

  if (kind == ImTransportKind.websocket) {
    String wsUrl;
    if (trimmed.startsWith('https://')) {
      wsUrl = 'wss://${trimmed.substring('https://'.length)}';
    } else if (trimmed.startsWith('http://')) {
      wsUrl = 'ws://${trimmed.substring('http://'.length)}';
    } else if (trimmed.startsWith('wss://') || trimmed.startsWith('ws://')) {
      wsUrl = trimmed;
    } else {
      wsUrl = 'ws://$trimmed';
    }
    final parsed = Uri.parse(wsUrl);
    final basePath = parsed.path.replaceAll(RegExp(r'/+$'), '');
    final path = basePath.endsWith(imRealtimeWsPath)
        ? basePath
        : '$basePath$imRealtimeWsPath';
    final query = Map<String, String>.from(parsed.queryParameters);
    if (deviceId != null && deviceId.isNotEmpty) {
      query['deviceId'] = deviceId;
    }
    return parsed
        .replace(path: path, queryParameters: query.isEmpty ? null : query)
        .toString();
  }

  if (kind == ImTransportKind.tcp) {
    return 'tcp://${_extractHostPort(trimmed)}';
  }

  if (kind == ImTransportKind.udp) {
    return 'udp://${_extractHostPort(trimmed)}';
  }

  return trimmed;
}

/// Extracts the `host:port` portion from a URL of any scheme.
String _extractHostPort(String url) {
  final match = RegExp(r'^[a-zA-Z]+://([^/]+)').firstMatch(url);
  if (match != null) {
    return match.group(1)!;
  }
  return url;
}

/// The default set of transport factories for native Dart/Flutter runtimes.
///
/// Includes WebSocket, TCP, and UDP factories. All are backed by `dart:io`.
final Map<ImTransportKind, ImTransportFactory> defaultTransportFactories =
    <ImTransportKind, ImTransportFactory>{
  ImTransportKind.websocket: const ImWebSocketTransportFactory(),
  ImTransportKind.tcp: const ImTcpTransportFactory(),
  ImTransportKind.udp: const ImUdpTransportFactory(),
};

/// Transport selector, encapsulating the full transport selection flow.
class ImTransportSelector {
  ImTransportSelector(
    this.factories, [
    this.policy = defaultTransportSelectionPolicy,
  ]);

  final Map<ImTransportKind, ImTransportFactory> factories;
  final ImTransportSelectionPolicy policy;

  /// Detects available transport kinds in the current environment.
  List<ImTransportKind> detectAvailable() =>
      detectAvailableTransports(factories);

  /// Returns the factory for [kind], or null if not registered.
  ImTransportFactory? getFactory(ImTransportKind kind) => factories[kind];

  /// Builds a prioritized candidate list of available transports.
  ///
  /// If [preferredKind] is specified and available, it is placed first.
  /// Remaining transports are added in policy.preferred order.
  /// Unavailable transports are skipped.
  List<ImTransportKind> buildCandidateList({ImTransportKind? preferredKind}) {
    final candidates = <ImTransportKind>[];
    final seen = <ImTransportKind>{};

    void tryAdd(ImTransportKind kind) {
      if (seen.contains(kind)) return;
      final factory = factories[kind];
      if (factory?.isAvailable() ?? false) {
        candidates.add(kind);
        seen.add(kind);
      }
    }

    if (preferredKind != null) {
      tryAdd(preferredKind);
    }
    for (final kind in policy.preferred) {
      tryAdd(kind);
    }

    return candidates;
  }

  /// Builds a transport endpoint for [kind] from the base URL.
  ImTransportEndpoint buildEndpoint(
    ImTransportKind kind,
    String baseUrl, {
    String? deviceId,
  }) {
    return buildTransportEndpoint(baseUrl, kind, deviceId: deviceId);
  }

  /// Selects a transport and builds the corresponding endpoint.
  ///
  /// Returns a record with the selected [factory] and the constructed
  /// [endpoint].
  ({ImTransportFactory factory, ImTransportEndpoint endpoint}) select(
    String baseUrl, {
    ImTransportKind? preferredKind,
    String? deviceId,
  }) {
    final factory = selectTransportFactory(factories, policy, preferredKind);
    final endpoint = buildTransportEndpoint(
      baseUrl,
      factory.kind,
      deviceId: deviceId,
    );
    return (factory: factory, endpoint: endpoint);
  }
}
