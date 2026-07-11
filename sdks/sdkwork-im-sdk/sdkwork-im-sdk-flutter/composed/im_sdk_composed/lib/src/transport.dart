/// Generic transport abstraction layer.
///
/// Mirrors the TypeScript `transport.ts`. Defines a transport-protocol-agnostic
/// connection interface so that the upper-layer CCP state machine in
/// `im_realtime.dart` can be reused across WebSocket/TCP/UDP implementations.
///
/// Design goals:
/// - Browser environments can only supply a WebSocket transport (API limit).
/// - Native Dart/Flutter runtimes can supply WebSocket/TCP/UDP via `dart:io`.
/// - The realtime layer only depends on [ImTransportConnection], never on a
///   concrete transport implementation.
library;

/// Transport protocol kind identifier, aligned with the server-side
/// `TransportBinding` enum.
enum ImTransportKind { websocket, tcp, udp }

/// CCP binding identifier used in the protocol envelope `binding` field,
/// aligned with the server-side `ccp-core` `TransportBinding.protocol_id`.
enum ImCcpBindingId { ws1, tcp1, udp1, quic1 }

/// Transport connection lifecycle state.
enum ImTransportConnectionState { connecting, open, closing, closed }

/// Transport capability descriptor, used for transport selection and protocol
/// behavior decisions.
class ImTransportCapabilities {
  const ImTransportCapabilities({
    required this.supportsFraming,
    required this.supportsDatagram,
    required this.reliable,
    required this.orderedDelivery,
    required this.supportsBackpressure,
    required this.maxFrameBytes,
    required this.supportsUpgradeAuth,
    required this.ccpBinding,
  });

  /// Whether the transport preserves frame boundaries (WebSocket/TCP/QUIC true,
  /// UDP false).
  final bool supportsFraming;

  /// Whether the transport supports datagram semantics (UDP true, others false).
  final bool supportsDatagram;

  /// Whether the transport is reliable (WebSocket/TCP/QUIC true, UDP false).
  final bool reliable;

  /// Whether delivery is ordered (WebSocket/TCP/QUIC true, UDP false).
  final bool orderedDelivery;

  /// Whether the transport supports backpressure (TCP/QUIC true; WebSocket
  /// depends on implementation; UDP false).
  final bool supportsBackpressure;

  /// Maximum bytes per frame (TCP/QUIC 512KB, UDP 64KB, WebSocket impl-defined).
  final int maxFrameBytes;

  /// Whether HTTP-upgrade auth is supported (WebSocket true, TCP/UDP false).
  final bool supportsUpgradeAuth;

  /// Corresponding CCP binding identifier.
  final ImCcpBindingId ccpBinding;
}

/// Default capability descriptors per transport kind, aligned with the
/// server-side `ccp-core` `TransportBinding`.
const transportCapabilities = <ImTransportKind, ImTransportCapabilities>{
  ImTransportKind.websocket: ImTransportCapabilities(
    supportsFraming: true,
    supportsDatagram: false,
    reliable: true,
    orderedDelivery: true,
    supportsBackpressure: false,
    maxFrameBytes: 512 * 1024,
    supportsUpgradeAuth: true,
    ccpBinding: ImCcpBindingId.ws1,
  ),
  ImTransportKind.tcp: ImTransportCapabilities(
    supportsFraming: true,
    supportsDatagram: false,
    reliable: true,
    orderedDelivery: true,
    supportsBackpressure: true,
    maxFrameBytes: 512 * 1024,
    supportsUpgradeAuth: false,
    ccpBinding: ImCcpBindingId.tcp1,
  ),
  ImTransportKind.udp: ImTransportCapabilities(
    supportsFraming: false,
    supportsDatagram: true,
    reliable: false,
    orderedDelivery: false,
    supportsBackpressure: false,
    maxFrameBytes: 64 * 1024,
    supportsUpgradeAuth: false,
    ccpBinding: ImCcpBindingId.udp1,
  ),
};

/// A single transport frame.
class ImTransportFrame {
  const ImTransportFrame({required this.data, required this.isBinary});

  /// Frame payload. Either a UTF-8 string (CCP JSON) or binary bytes (CCP
  /// CBOR, reserved).
  final Object data;

  /// Whether [data] is binary.
  final bool isBinary;
}

/// Transport connection close event.
class ImTransportCloseEvent {
  const ImTransportCloseEvent({
    required this.code,
    required this.reason,
    required this.wasClean,
  });

  final int code;
  final String reason;
  final bool wasClean;
}

/// Transport connection error event.
class ImTransportErrorEvent {
  const ImTransportErrorEvent({required this.error, this.code});

  final Object error;
  final String? code;
}

/// Generic transport connection interface.
///
/// All transport implementations (WebSocket/TCP/UDP) adapt to this interface so
/// that the upper realtime layer is agnostic of the underlying protocol.
abstract class ImTransportConnection {
  /// Current connection state.
  ImTransportConnectionState get state;

  /// Transport capability descriptor.
  ImTransportCapabilities get capabilities;

  /// Transport kind.
  ImTransportKind get kind;

  /// Sends a single frame. Behavior when not `open` is implementation-defined
  /// (buffered or dropped).
  void send(ImTransportFrame frame);

  /// Actively closes the connection.
  void close({int? code, String? reason});

  /// Registers a message handler. Returns an unsubscribe function.
  void Function() onMessage(void Function(ImTransportFrame frame) handler);

  /// Registers an open handler. Returns an unsubscribe function.
  void Function() onOpen(void Function() handler);

  /// Registers a close handler. Returns an unsubscribe function.
  void Function() onClose(void Function(ImTransportCloseEvent event) handler);

  /// Registers an error handler. Returns an unsubscribe function.
  void Function() onError(void Function(ImTransportErrorEvent event) handler);
}

/// Transport endpoint descriptor.
class ImTransportEndpoint {
  const ImTransportEndpoint({
    required this.kind,
    required this.url,
    this.headers,
    this.protocols,
    this.deviceId,
  });

  /// Transport kind.
  final ImTransportKind kind;

  /// Endpoint URL.
  ///
  /// - websocket: `ws://host:port/im/v3/api/realtime/ws` or `wss://...`
  /// - tcp: `tcp://host:port`
  /// - udp: `udp://host:port`
  final String url;

  /// Additional HTTP headers (WebSocket upgrade only; ignored by TCP/UDP).
  final Map<String, String>? headers;

  /// WebSocket subprotocols (WebSocket only).
  final List<String>? protocols;

  /// Device ID used for routing binding.
  final String? deviceId;
}

/// Transport connection options.
class ImTransportConnectOptions {
  const ImTransportConnectOptions({
    required this.connectionTimeoutMs,
    this.headers,
    this.protocols,
  });

  /// Connection timeout in milliseconds.
  final int connectionTimeoutMs;

  /// Additional HTTP headers.
  final Map<String, String>? headers;

  /// WebSocket subprotocols.
  final List<String>? protocols;
}

/// Transport factory interface.
///
/// Each transport (WebSocket/TCP/UDP) implements a factory that declares its
/// availability via [isAvailable] and produces connections via [connect].
abstract class ImTransportFactory {
  /// Transport kind.
  ImTransportKind get kind;

  /// Transport capability descriptor.
  ImTransportCapabilities get capabilities;

  /// Whether the current runtime supports this transport.
  bool isAvailable();

  /// Establishes a transport connection.
  ///
  /// Returns a connection whose state is `connecting` or `open`.
  Future<ImTransportConnection> connect(
    ImTransportEndpoint endpoint,
    ImTransportConnectOptions options,
  );
}

/// Transport selection policy.
class ImTransportSelectionPolicy {
  const ImTransportSelectionPolicy({
    required this.preferred,
    required this.autoFallback,
    required this.probeTimeoutMs,
  });

  /// Preferred transport priority list, tried in order.
  final List<ImTransportKind> preferred;

  /// Whether to automatically fall back to the next transport when the
  /// preferred one is unavailable.
  final bool autoFallback;

  /// Connection probe timeout in milliseconds; after which the next transport
  /// is tried.
  final int probeTimeoutMs;
}

/// Default selection policy: prefer WebSocket, auto-fallback.
const defaultTransportSelectionPolicy = ImTransportSelectionPolicy(
  preferred: [ImTransportKind.websocket, ImTransportKind.tcp, ImTransportKind.udp],
  autoFallback: true,
  probeTimeoutMs: 15000,
);

/// Returns the CCP binding identifier for a transport kind.
ImCcpBindingId ccpBindingForTransport(ImTransportKind kind) {
  return transportCapabilities[kind]!.ccpBinding;
}

/// Parses the transport kind implied by a URL scheme.
///
/// Returns `null` when the scheme does not map to a known transport.
ImTransportKind? parseTransportKindFromUrl(String url) {
  if (RegExp(r'^wss?://', caseSensitive: false).hasMatch(url)) {
    return ImTransportKind.websocket;
  }
  if (RegExp(r'^tcp://', caseSensitive: false).hasMatch(url)) {
    return ImTransportKind.tcp;
  }
  if (RegExp(r'^udp://', caseSensitive: false).hasMatch(url)) {
    return ImTransportKind.udp;
  }
  return null;
}
