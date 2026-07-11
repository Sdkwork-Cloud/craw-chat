import 'dart:async';

import 'package:im_sdk_generated/im_client.dart';

import 'ccp_wire.dart';
import 'im_realtime.dart';
import 'transport.dart';
import 'transport_selector.dart';

Future<void> _waitForTransportOpen(
  ImTransportConnection transport,
  int timeoutMs,
) {
  if (transport.state == ImTransportConnectionState.open) {
    return Future<void>.value();
  }
  if (transport.state == ImTransportConnectionState.closed ||
      transport.state == ImTransportConnectionState.closing) {
    return Future<void>.error(
      StateError('Transport "${transport.kind}" closed before it became ready.'),
    );
  }

  final completer = Completer<void>();
  Timer? timer;
  void Function()? unsubscribeOpen;
  void Function()? unsubscribeClose;
  void Function()? unsubscribeError;

  void cleanup() {
    timer?.cancel();
    timer = null;
    unsubscribeOpen?.call();
    unsubscribeClose?.call();
    unsubscribeError?.call();
    unsubscribeOpen = null;
    unsubscribeClose = null;
    unsubscribeError = null;
  }

  void complete() {
    if (completer.isCompleted) return;
    cleanup();
    completer.complete();
  }

  void fail(Object error, [StackTrace? stackTrace]) {
    if (completer.isCompleted) return;
    cleanup();
    completer.completeError(error, stackTrace);
  }

  unsubscribeOpen = transport.onOpen(complete);
  unsubscribeClose = transport.onClose((event) {
    fail(StateError(
      'Transport "${transport.kind}" closed before open '
      '(${event.code}: ${event.reason}).',
    ));
  });
  unsubscribeError = transport.onError((event) => fail(event.error));
  final effectiveTimeout = timeoutMs > 0 ? timeoutMs : 15000;
  timer = Timer(Duration(milliseconds: effectiveTimeout), () {
    fail(StateError(
      'Transport "${transport.kind}" probe timed out after ${effectiveTimeout}ms.',
    ));
  });

  // Re-check after listener registration to close the transition race.
  if (transport.state == ImTransportConnectionState.open) {
    complete();
  } else if (transport.state == ImTransportConnectionState.closed ||
      transport.state == ImTransportConnectionState.closing) {
    fail(StateError('Transport "${transport.kind}" closed before it became ready.'));
  }
  return completer.future;
}

class ImSdkComposedClient {
  ImSdkComposedClient({
    required this.transport,
    required this.websocketBaseUrl,
    this.accessToken,
    this.authToken,
    this.headers = const {},
    this.transportKind,
    this.transportFactories,
    this.transportPolicy,
  });

  final SdkworkImClient transport;
  final String websocketBaseUrl;
  String? accessToken;
  String? authToken;
  final Map<String, String> headers;

  /// Manual transport override. When set, enables the multi-transport path.
  final ImTransportKind? transportKind;

  /// Custom transport factory map. When null, default factories are used.
  final Map<ImTransportKind, ImTransportFactory>? transportFactories;

  /// Transport selection policy (auto-detect + fallback).
  final ImTransportSelectionPolicy? transportPolicy;

  /// Connects to the realtime server.
  ///
  /// When [transportKind], [transportFactories], or [transportPolicy] is set,
  /// uses the multi-transport path with automatic fallback. Otherwise, falls
  /// back to the legacy WebSocket path (backward compatible).
  ///
  /// When [realtimeTransport] is provided directly, it is used as-is without
  /// invoking factory connect logic.
  Future<ImLiveConnection> connect({
    ImConnectOptions options = const ImConnectOptions(),
    ImTransportConnection? realtimeTransport,
  }) async {
    // Pre-built transport: use directly (backward compatible with callers
    // that manage transport lifecycle externally).
    if (realtimeTransport != null) {
      return _buildLiveConnection(realtimeTransport, options);
    }

    final useMultiTransport = transportKind != null ||
        transportFactories != null ||
        transportPolicy != null;

    if (!useMultiTransport) {
      // Legacy WebSocket path
      return _buildLiveConnection(null, options);
    }

    final factories = transportFactories ?? defaultTransportFactories;
    final policy = transportPolicy ?? defaultTransportSelectionPolicy;
    final selector = ImTransportSelector(factories, policy);
    final connectionTimeoutMs = options.connectionTimeoutMs;
    final connectOptions = ImTransportConnectOptions(
      connectionTimeoutMs: connectionTimeoutMs,
      headers: headers,
      protocols: [imCcpWebSocketSubprotocol],
    );

    final candidates =
        selector.buildCandidateList(preferredKind: transportKind);

    // Check preferred transport availability when autoFallback is false
    if (transportKind != null && !policy.autoFallback) {
      final preferredFactory = factories[transportKind!];
      if (preferredFactory == null || !preferredFactory.isAvailable()) {
        throw StateError(
          'Preferred transport "$transportKind" is not available '
          'in the current environment.',
        );
      }
    }

    return _connectWithFallback(
      candidates,
      selector,
      options,
      connectOptions,
      policy.autoFallback,
      policy.probeTimeoutMs,
    );
  }

  /// Recursively tries each candidate transport, falling back on failure.
  ///
  /// Only covers factory.connect() failures (transport layer). CCP handshake
  /// failures (e.g. auth errors) do not trigger fallback, as they are likely
  /// credential issues rather than transport unavailability.
  Future<ImLiveConnection> _connectWithFallback(
    List<ImTransportKind> candidates,
    ImTransportSelector selector,
    ImConnectOptions options,
    ImTransportConnectOptions connectOptions,
    bool autoFallback,
    int probeTimeoutMs,
    [Object? lastError]
  ) async {
    if (candidates.isEmpty) {
      if (lastError != null) throw lastError;
      throw StateError('No transport is available in the current environment.');
    }

    final kind = candidates.first;
    final rest = candidates.sublist(1);
    final factory = selector.getFactory(kind);
    if (factory == null) {
      return _connectWithFallback(
        rest, selector, options, connectOptions, autoFallback, probeTimeoutMs,
        lastError,
      );
    }
    final endpoint = selector.buildEndpoint(
      kind,
      websocketBaseUrl,
      deviceId: options.deviceId,
    );

    ImTransportConnection? transport;
    try {
      transport = await factory.connect(endpoint, connectOptions);
      await _waitForTransportOpen(transport, probeTimeoutMs);
    } catch (error) {
      try {
        transport?.close(code: 4008, reason: 'transport_probe_failed');
      } catch (_) {
        // The transport may already have closed itself.
      }
      if (!autoFallback) {
        rethrow;
      }
      return _connectWithFallback(
        rest, selector, options, connectOptions, autoFallback, probeTimeoutMs,
        error,
      );
    }

    try {
      return _buildLiveConnection(transport, options);
    } catch (_) {
      transport.close(code: 4000, reason: 'live_connection_init_failed');
      rethrow;
    }
  }

  ImLiveConnection _buildLiveConnection(
    ImTransportConnection? transport,
    ImConnectOptions options,
  ) {
    final resolvedDeviceId =
        options.deviceId ?? deviceIdFromAccessToken(accessToken);
    return createImLiveConnection(
      ImCreateLiveConnectionParams(
        websocketBaseUrl: websocketBaseUrl,
        accessToken: accessToken,
        authToken: authToken,
        headers: headers,
        options: ImConnectOptions(
          deviceId: resolvedDeviceId,
          subscriptions: options.subscriptions,
          connectionTimeoutMs: options.connectionTimeoutMs,
          authTimeoutMs: options.authTimeoutMs,
        ),
        transport: transport,
      ),
    );
  }
}
