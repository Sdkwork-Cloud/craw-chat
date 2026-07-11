/// UDP transport implementation (native Dart/Flutter only).
///
/// Uses `dart:io RawDatagramSocket` for connectionless datagram communication.
/// Each datagram carries a single complete CCP envelope (JSON string bytes),
/// aligned with the server-side `sdkwork-im-ccp-binding-udp`
/// (`CCP_UDP_MAX_DATAGRAM_BYTES=64KB`).
///
/// UDP is unreliable and unordered; it is primarily suitable for:
/// - CCP control frame handshake (hello/auth_bind/auth_ok)
/// - Heartbeat keepalive
/// - Low-latency lightweight commands
///
/// Business message delivery should use WebSocket/TCP. The server-side
/// `serve_udp_datagram` handles each datagram independently without
/// maintaining long session state.
///
/// Only available in native Dart/Flutter runtimes that expose `dart:io`.
library;

import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import '../transport.dart';

/// Maximum datagram size (64KB), aligned with the server-side
/// `CCP_UDP_MAX_DATAGRAM_BYTES`.
const int _udpMaxDatagramBytes = 64 * 1024;

/// Parses `udp://host:port` into a host/port pair.
_HostPort _parseUdpEndpoint(String url) {
  final parsed = Uri.parse(url);
  final host = parsed.host.isEmpty ? '127.0.0.1' : parsed.host;
  final port = parsed.port;
  if (port <= 0 || port > 65535) {
    throw ArgumentError('Invalid UDP port in URL: $url');
  }
  return _HostPort(host, port);
}

class _HostPort {
  const _HostPort(this.host, this.port);
  final String host;
  final int port;
}

/// UDP transport connection.
///
/// UDP is a connectionless protocol, but to adapt it to the
/// [ImTransportConnection] interface we treat the datagram socket as
/// "connected" once created. Each [send] dispatches a single datagram to the
/// server; [onMessage] receives datagrams from the server.
class ImUdpTransportConnection implements ImTransportConnection {
  ImUdpTransportConnection(this._socket, this._serverHost, this._serverPort) {
    _socket.listen(
      (event) {
        if (event == RawSocketEvent.read) {
          final datagram = _socket.receive();
          if (datagram == null) {
            return;
          }
          final text = utf8.decode(datagram.data);
          final frame = ImTransportFrame(data: text, isBinary: false);
          for (final handler in List.of(_messageHandlers)) {
            handler(frame);
          }
        } else if (event == RawSocketEvent.closed) {
          if (_state == ImTransportConnectionState.closed) return;
          _state = ImTransportConnectionState.closed;
          final closeEvent = ImTransportCloseEvent(
            code: 1000,
            reason: 'udp_socket_closed',
            wasClean: true,
          );
          for (final handler in List.of(_closeHandlers)) {
            handler(closeEvent);
          }
        }
      },
      onError: (Object error) {
        final errorEvent = ImTransportErrorEvent(
          error: error,
          code: 'udp_socket_error',
        );
        for (final handler in List.of(_errorHandlers)) {
          handler(errorEvent);
        }
        // 确保 error 后触发 close：RawDatagramSocket error 后可能不会自动关闭
        if (_state != ImTransportConnectionState.closed) {
          try {
            _socket.close();
          } catch (_) {
            // 忽略重复关闭
          }
        }
      },
    );

    // UDP is connectionless; the socket is ready to send immediately, so we
    // treat it as open right away. Defer open callback to next microtask to
    // ensure callers can register onOpen after connect() resolves.
    _state = ImTransportConnectionState.open;
    scheduleMicrotask(() {
      if (_state != ImTransportConnectionState.open) return;
      _openDispatched = true;
      for (final handler in List.of(_openHandlers)) {
        handler();
      }
    });
  }

  final RawDatagramSocket _socket;
  final String _serverHost;
  final int _serverPort;
  ImTransportConnectionState _state = ImTransportConnectionState.connecting;
  bool _openDispatched = false;

  final _messageHandlers = <void Function(ImTransportFrame frame)>[];
  final _openHandlers = <void Function()>[];
  final _closeHandlers = <void Function(ImTransportCloseEvent event)>[];
  final _errorHandlers = <void Function(ImTransportErrorEvent event)>[];

  @override
  ImTransportKind get kind => ImTransportKind.udp;

  @override
  ImTransportCapabilities get capabilities =>
      transportCapabilities[ImTransportKind.udp]!;

  @override
  ImTransportConnectionState get state => _state;

  @override
  void send(ImTransportFrame frame) {
    if (_state != ImTransportConnectionState.open) {
      return;
    }
    final List<int> bytes;
    final data = frame.data;
    if (data is String) {
      bytes = utf8.encode(data);
    } else if (data is Uint8List) {
      bytes = data;
    } else if (data is List<int>) {
      bytes = data;
    } else {
      return;
    }
    if (bytes.length > _udpMaxDatagramBytes) {
      final errorEvent = ImTransportErrorEvent(
        error: RangeError(
          'UDP datagram exceeds max $_udpMaxDatagramBytes bytes: got ${bytes.length}',
        ),
        code: 'udp_datagram_too_large',
      );
      for (final handler in List.of(_errorHandlers)) {
        handler(errorEvent);
      }
      return;
    }
    _socket.send(bytes, InternetAddress(_serverHost), _serverPort);
  }

  @override
  void close({int? code, String? reason}) {
    if (_state == ImTransportConnectionState.closing ||
        _state == ImTransportConnectionState.closed) {
      return;
    }
    _state = ImTransportConnectionState.closing;
    try {
      _socket.close();
    } catch (_) {
      // 忽略重复关闭
    }
    _state = ImTransportConnectionState.closed;
    final closeEvent = ImTransportCloseEvent(
      code: code ?? 1000,
      reason: reason ?? 'udp_socket_closed',
      wasClean: true,
    );
    for (final handler in List.of(_closeHandlers)) {
      handler(closeEvent);
    }
  }

  @override
  void Function() onMessage(void Function(ImTransportFrame frame) handler) {
    _messageHandlers.add(handler);
    return () => _messageHandlers.remove(handler);
  }

  @override
  void Function() onOpen(void Function() handler) {
    _openHandlers.add(handler);
    // 仅在 open 事件已经派发后才调度回调；否则 constructor 的 microtask 会负责调用
    if (_state == ImTransportConnectionState.open && _openDispatched) {
      scheduleMicrotask(() {
        if (_openHandlers.contains(handler) &&
            _state == ImTransportConnectionState.open) {
          handler();
        }
      });
    }
    return () => _openHandlers.remove(handler);
  }

  @override
  void Function() onClose(void Function(ImTransportCloseEvent event) handler) {
    _closeHandlers.add(handler);
    // late-registration：如果连接已经关闭，立即调度回调
    if (_state == ImTransportConnectionState.closed) {
      scheduleMicrotask(() {
        if (_closeHandlers.contains(handler) &&
            _state == ImTransportConnectionState.closed) {
          handler(ImTransportCloseEvent(
            code: 1000,
            reason: 'udp_socket_closed',
            wasClean: true,
          ));
        }
      });
    }
    return () => _closeHandlers.remove(handler);
  }

  @override
  void Function() onError(void Function(ImTransportErrorEvent event) handler) {
    _errorHandlers.add(handler);
    return () => _errorHandlers.remove(handler);
  }
}

/// UDP transport factory (native Dart/Flutter only).
///
/// Creates a `RawDatagramSocket` and returns an immediately-open connection.
/// UDP is connectionless, so there is no handshake to await.
class ImUdpTransportFactory implements ImTransportFactory {
  const ImUdpTransportFactory();

  @override
  ImTransportKind get kind => ImTransportKind.udp;

  @override
  ImTransportCapabilities get capabilities =>
      transportCapabilities[ImTransportKind.udp]!;

  @override
  bool isAvailable() => true;

  @override
  Future<ImTransportConnection> connect(
    ImTransportEndpoint endpoint,
    ImTransportConnectOptions options,
  ) async {
    final target = _parseUdpEndpoint(endpoint.url);
    final socket = await RawDatagramSocket.bind(
      InternetAddress.anyIPv4,
      0,
    ).timeout(Duration(milliseconds: options.connectionTimeoutMs));
    return ImUdpTransportConnection(socket, target.host, target.port);
  }
}
