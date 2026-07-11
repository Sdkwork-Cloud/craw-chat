/// WebSocket transport implementation.
///
/// Adapts `IOWebSocketChannel` to the [ImTransportConnection] interface. This
/// is the only transport available in browser-like environments; native
/// Dart/Flutter runtimes can also use TCP/UDP.
library;

import 'dart:async';
import 'dart:typed_data';

import 'package:web_socket_channel/io.dart';

import '../transport.dart';

/// WebSocket transport connection, adapting [IOWebSocketChannel] to
/// [ImTransportConnection].
class ImWebSocketTransportConnection implements ImTransportConnection {
  ImWebSocketTransportConnection(this._channel) {
    _channel.stream.listen(
      (event) {
        if (_state != ImTransportConnectionState.open) {
          return;
        }
        if (event is String) {
          final frame = ImTransportFrame(data: event, isBinary: false);
          for (final handler in List.of(_messageHandlers)) {
            handler(frame);
          }
        } else if (event is List<int>) {
          final frame = ImTransportFrame(
            data: Uint8List.fromList(event),
            isBinary: true,
          );
          for (final handler in List.of(_messageHandlers)) {
            handler(frame);
          }
        }
      },
      onError: (Object error) {
        final errorEvent = ImTransportErrorEvent(
          error: error,
          code: 'websocket_stream_error',
        );
        for (final handler in List.of(_errorHandlers)) {
          handler(errorEvent);
        }
        // Ensure close is triggered after error: cancelOnError=true usually
        // triggers onDone, but proactively close to guarantee state consistency.
        if (_state != ImTransportConnectionState.closed) {
          try {
            _channel.sink.close(4000, 'websocket_error');
          } catch (_) {
            // Ignore repeated close
          }
        }
      },
      onDone: () {
        final code = _channel.closeCode ?? 1000;
        final reason = _channel.closeReason ?? '';
        _finalizeClose(_requestedCloseEvent ?? ImTransportCloseEvent(
          code: code,
          reason: reason,
          wasClean: code >= 1000 && code < 1004,
        ));
      },
      cancelOnError: true,
    );

    // The `ready` future completes once the WebSocket handshake succeeds. Use
    // it to drive the connecting -> open transition. Errors are surfaced via
    // the stream's onError callback above.
    _channel.ready.then(
      (_) {
        if (_state != ImTransportConnectionState.connecting) return;
        _state = ImTransportConnectionState.open;
        // Defer open callback to next microtask to ensure callers can register
        // onOpen after connect() resolves.
        scheduleMicrotask(() {
          if (_state != ImTransportConnectionState.open) return;
          _openDispatched = true;
          for (final handler in List.of(_openHandlers)) {
            handler();
          }
        });
      },
      onError: (Object error) {
        // Swallow: the stream's onError listener already surfaces the error to
        // the realtime layer. Handling it here would double-report.
      },
    );
  }

  final IOWebSocketChannel _channel;
  ImTransportConnectionState _state = ImTransportConnectionState.connecting;
  bool _openDispatched = false;
  ImTransportCloseEvent? _closeEvent;
  ImTransportCloseEvent? _requestedCloseEvent;

  final _messageHandlers = <void Function(ImTransportFrame frame)>[];
  final _openHandlers = <void Function()>[];
  final _closeHandlers = <void Function(ImTransportCloseEvent event)>[];
  final _errorHandlers = <void Function(ImTransportErrorEvent event)>[];

  void _finalizeClose(ImTransportCloseEvent event) {
    if (_state == ImTransportConnectionState.closed) return;
    _state = ImTransportConnectionState.closed;
    _closeEvent = event;
    for (final handler in List.of(_closeHandlers)) {
      handler(event);
    }
    _messageHandlers.clear();
    _openHandlers.clear();
    _errorHandlers.clear();
  }

  @override
  ImTransportKind get kind => ImTransportKind.websocket;

  @override
  ImTransportCapabilities get capabilities =>
      transportCapabilities[ImTransportKind.websocket]!;

  @override
  ImTransportConnectionState get state => _state;

  @override
  void send(ImTransportFrame frame) {
    if (_state != ImTransportConnectionState.open) {
      return;
    }
    final data = frame.data;
    if (data is String) {
      _channel.sink.add(data);
    } else if (data is List<int>) {
      _channel.sink.add(data);
    }
  }

  @override
  void close({int? code, String? reason}) {
    if (_state == ImTransportConnectionState.closing ||
        _state == ImTransportConnectionState.closed) {
      return;
    }
    _state = ImTransportConnectionState.closing;
    _requestedCloseEvent = ImTransportCloseEvent(
      code: code ?? 1000,
      reason: reason ?? '',
      wasClean: true,
    );
    unawaited(() async {
      try {
        await _channel.sink.close(code ?? 1000, reason ?? '');
      } catch (_) {
        _finalizeClose(ImTransportCloseEvent(
          code: code ?? 1000,
          reason: reason ?? 'websocket_close_failed',
          wasClean: false,
        ));
      }
    }());
  }

  @override
  void Function() onMessage(void Function(ImTransportFrame frame) handler) {
    _messageHandlers.add(handler);
    return () => _messageHandlers.remove(handler);
  }

  @override
  void Function() onOpen(void Function() handler) {
    _openHandlers.add(handler);
    // Only schedule callback if open event has already been dispatched;
    // otherwise the ready future's microtask will handle it.
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
        if (_closeHandlers.contains(handler)) {
          handler(_closeEvent ?? const ImTransportCloseEvent(
            code: 1000,
            reason: 'websocket_already_closed',
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

/// WebSocket transport factory.
///
/// Wraps [IOWebSocketChannel.connect] and adapts it to [ImTransportFactory].
/// The returned connection is in `connecting` state and transitions to `open`
/// asynchronously via the `ready` future.
class ImWebSocketTransportFactory implements ImTransportFactory {
  const ImWebSocketTransportFactory();

  @override
  ImTransportKind get kind => ImTransportKind.websocket;

  @override
  ImTransportCapabilities get capabilities =>
      transportCapabilities[ImTransportKind.websocket]!;

  @override
  bool isAvailable() => true;

  @override
  Future<ImTransportConnection> connect(
    ImTransportEndpoint endpoint,
    ImTransportConnectOptions options,
  ) async {
    final uri = Uri.parse(endpoint.url);
    final headers = <String, dynamic>{
      ...?options.headers,
      ...?endpoint.headers,
    };
    final protocols =
        endpoint.protocols ?? options.protocols ?? const <String>[];

    final channel = IOWebSocketChannel.connect(
      uri,
      headers: headers.isEmpty ? null : headers,
      protocols: protocols,
      connectTimeout: Duration(milliseconds: options.connectionTimeoutMs),
    );

    return ImWebSocketTransportConnection(channel);
  }
}
