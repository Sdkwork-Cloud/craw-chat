/// TCP transport implementation (native Dart/Flutter only).
///
/// Uses `dart:io Socket` to establish a long-lived connection with 4-byte
/// big-endian length-prefix framing, aligned with the server-side
/// `sdkwork-im-ccp-binding-tcp` (`CCP_TCP_FRAME_HEADER_BYTES=4`,
/// `CCP_TCP_MAX_FRAME_BYTES=512KB`).
///
/// Only available in native Dart/Flutter runtimes that expose `dart:io`.
library;

import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import '../transport.dart';

/// 4-byte big-endian length prefix, aligned with the server-side
/// `CCP_TCP_FRAME_HEADER_BYTES`.
const int _tcpFrameHeaderBytes = 4;

/// Maximum frame payload size (512KB), aligned with the server-side
/// `CCP_TCP_MAX_FRAME_BYTES`.
const int _tcpMaxFrameBytes = 512 * 1024;

/// Parses `tcp://host:port` into a host/port pair.
_HostPort _parseTcpEndpoint(String url) {
  final parsed = Uri.parse(url);
  final host = parsed.host.isEmpty ? '127.0.0.1' : parsed.host;
  final port = parsed.port;
  if (port <= 0 || port > 65535) {
    throw ArgumentError('Invalid TCP port in URL: $url');
  }
  return _HostPort(host, port);
}

class _HostPort {
  const _HostPort(this.host, this.port);
  final String host;
  final int port;
}

/// Encodes a payload with a 4-byte big-endian length prefix.
///
/// Frame format: `[4-byte big-endian length][payload]`.
/// The payload is the UTF-8 byte sequence of a CCP envelope JSON string.
Uint8List _encodeTcpFrame(List<int> payloadBytes) {
  if (payloadBytes.length > _tcpMaxFrameBytes) {
    throw RangeError(
      'TCP frame payload exceeds max $_tcpMaxFrameBytes bytes: got ${payloadBytes.length}',
    );
  }
  final frame = Uint8List(_tcpFrameHeaderBytes + payloadBytes.length);
  final view = ByteData.sublistView(frame);
  view.setUint32(0, payloadBytes.length, Endian.big);
  frame.setRange(_tcpFrameHeaderBytes, frame.length, payloadBytes);
  return frame;
}

/// 4-byte big-endian length-prefix frame decoder.
///
/// State machine: read 4 header bytes to obtain the payload length, then read
/// that many payload bytes, then reset and wait for the next frame.
class _TcpFrameDecoder {
  final _buffer = <int>[];
  var _state = _DecoderState.header;
  var _expectedPayloadLength = 0;

  /// Pushes new data and returns the list of complete decoded frames.
  List<Uint8List> push(List<int> data) {
    _buffer.addAll(data);
    final frames = <Uint8List>[];
    while (true) {
      if (_state == _DecoderState.header) {
        if (_buffer.length < _tcpFrameHeaderBytes) {
          break;
        }
        final header = Uint8List.fromList(_buffer.sublist(0, _tcpFrameHeaderBytes));
        _buffer.removeRange(0, _tcpFrameHeaderBytes);
        final view = ByteData.sublistView(header);
        _expectedPayloadLength = view.getUint32(0, Endian.big);
        if (_expectedPayloadLength > _tcpMaxFrameBytes) {
          throw RangeError(
            'TCP frame length $_expectedPayloadLength exceeds max $_tcpMaxFrameBytes',
          );
        }
        _state = _DecoderState.payload;
      }
      if (_state == _DecoderState.payload) {
        if (_buffer.length < _expectedPayloadLength) {
          break;
        }
        final payload =
            Uint8List.fromList(_buffer.sublist(0, _expectedPayloadLength));
        _buffer.removeRange(0, _expectedPayloadLength);
        frames.add(payload);
        _state = _DecoderState.header;
        _expectedPayloadLength = 0;
      }
    }
    return frames;
  }

  /// 清理缓冲区，释放内存。连接关闭时调用。
  void reset() {
    _buffer.clear();
    _state = _DecoderState.header;
    _expectedPayloadLength = 0;
  }
}

enum _DecoderState { header, payload }

/// TCP transport connection, wrapping `dart:io Socket` with length-prefix
/// framing and adapting it to [ImTransportConnection].
class ImTcpTransportConnection implements ImTransportConnection {
  ImTcpTransportConnection(this._socket) {
    _decoder = _TcpFrameDecoder();
    _socket.listen(
      (data) {
        // 连接已关闭时不再处理数据，避免 decoder 状态混乱
        if (_state == ImTransportConnectionState.closing ||
            _state == ImTransportConnectionState.closed) {
          return;
        }
        try {
          final frames = _decoder.push(data);
          for (final frame in frames) {
            final text = utf8.decode(frame);
            final transportFrame = ImTransportFrame(data: text, isBinary: false);
            for (final handler in List.of(_messageHandlers)) {
              handler(transportFrame);
            }
          }
        } catch (error) {
          _hadError = true;
          final errorEvent = ImTransportErrorEvent(
            error: error,
            code: 'tcp_frame_decode_error',
          );
          for (final handler in List.of(_errorHandlers)) {
            handler(errorEvent);
          }
          _socket.destroy();
        }
      },
      onError: (Object error) {
        _hadError = true;
        final errorEvent = ImTransportErrorEvent(
          error: error,
          code: 'tcp_socket_error',
        );
        for (final handler in List.of(_errorHandlers)) {
          handler(errorEvent);
        }
        // 确保 error 后触发 close：cancelOnError=true 通常会触发 onDone，
        // 但主动 destroy 确保状态一致
        try {
          _socket.destroy();
        } catch (_) {
          // 忽略重复 destroy
        }
      },
      onDone: () {
        if (_state == ImTransportConnectionState.closed) return;
        final hadError = _hadError;
        _state = ImTransportConnectionState.closed;
        // 清理 decoder 缓冲区，释放内存
        _decoder.reset();
        final closeEvent = ImTransportCloseEvent(
          code: hadError ? 4000 : 1000,
          reason: hadError ? 'tcp_connection_error' : 'tcp_connection_closed',
          wasClean: !hadError,
        );
        for (final handler in List.of(_closeHandlers)) {
          handler(closeEvent);
        }
      },
      cancelOnError: true,
    );
  }

  final Socket _socket;
  late final _TcpFrameDecoder _decoder;
  ImTransportConnectionState _state = ImTransportConnectionState.connecting;
  bool _hadError = false;
  bool _openDispatched = false;

  final _messageHandlers = <void Function(ImTransportFrame frame)>[];
  final _openHandlers = <void Function()>[];
  final _closeHandlers = <void Function(ImTransportCloseEvent event)>[];
  final _errorHandlers = <void Function(ImTransportErrorEvent event)>[];

  /// Called by the factory once the socket's `done` future indicates a
  /// successful connection. Drives the connecting -> open transition.
  void _markOpen() {
    if (_state != ImTransportConnectionState.connecting) {
      return;
    }
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
  }

  @override
  ImTransportKind get kind => ImTransportKind.tcp;

  @override
  ImTransportCapabilities get capabilities =>
      transportCapabilities[ImTransportKind.tcp]!;

  @override
  ImTransportConnectionState get state => _state;

  @override
  void send(ImTransportFrame frame) {
    if (_state == ImTransportConnectionState.closing ||
        _state == ImTransportConnectionState.closed) {
      return;
    }
    final List<int> payloadBytes;
    final data = frame.data;
    if (data is String) {
      payloadBytes = utf8.encode(data);
    } else if (data is Uint8List) {
      payloadBytes = data;
    } else if (data is List<int>) {
      payloadBytes = data;
    } else {
      return;
    }
    final encoded = _encodeTcpFrame(payloadBytes);
    try {
      _socket.add(encoded);
    } catch (error) {
      final errorEvent = ImTransportErrorEvent(
        error: error,
        code: 'tcp_write_error',
      );
      for (final handler in List.of(_errorHandlers)) {
        handler(errorEvent);
      }
    }
  }

  @override
  void close({int? code, String? reason}) {
    if (_state == ImTransportConnectionState.closing ||
        _state == ImTransportConnectionState.closed) {
      return;
    }
    _state = ImTransportConnectionState.closing;
    try {
      _socket.destroy();
    } catch (_) {
      // 忽略重复 destroy
    }
    // destroy 后 onDone 可能不立即触发，用 scheduleMicrotask 兜底确保状态最终一致。
    // 如果 onDone 先触发，会设置 closed 并派发 close 事件，这里检查状态跳过。
    scheduleMicrotask(() {
      if (_state != ImTransportConnectionState.closing) return;
      _state = ImTransportConnectionState.closed;
      _decoder.reset();
      final closeEvent = ImTransportCloseEvent(
        code: code ?? 1000,
        reason: reason ?? 'tcp_connection_closed',
        wasClean: true,
      );
      for (final handler in List.of(_closeHandlers)) {
        handler(closeEvent);
      }
    });
  }

  @override
  void Function() onMessage(void Function(ImTransportFrame frame) handler) {
    _messageHandlers.add(handler);
    return () => _messageHandlers.remove(handler);
  }

  @override
  void Function() onOpen(void Function() handler) {
    _openHandlers.add(handler);
    // 仅在 open 事件已经派发后才调度回调；否则 _markOpen 的 microtask 会负责调用
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
            reason: 'tcp_connection_closed',
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

/// TCP transport factory (native Dart/Flutter only).
///
/// Uses `dart:io Socket.connect` to establish a TCP connection. The returned
/// connection is in `connecting` state and transitions to `open` once the
/// socket is established.
class ImTcpTransportFactory implements ImTransportFactory {
  const ImTcpTransportFactory();

  @override
  ImTransportKind get kind => ImTransportKind.tcp;

  @override
  ImTransportCapabilities get capabilities =>
      transportCapabilities[ImTransportKind.tcp]!;

  @override
  bool isAvailable() => true;

  @override
  Future<ImTransportConnection> connect(
    ImTransportEndpoint endpoint,
    ImTransportConnectOptions options,
  ) async {
    final target = _parseTcpEndpoint(endpoint.url);
    final socket = await Socket.connect(
      target.host,
      target.port,
      timeout: Duration(milliseconds: options.connectionTimeoutMs),
    );
    // 启用 TCP_NODELAY，禁用 Nagle 算法，降低小帧延迟。
    // Dart 的 SocketOption 不直接暴露 keepalive 初始延迟，tcpNoDelay 是
    // 跨平台可用的最关键选项；keepalive 由 OS 默认策略负责。
    try {
      socket.setOption(SocketOption.tcpNoDelay, true);
    } catch (_) {
      // 某些平台可能不支持
    }
    final connection = ImTcpTransportConnection(socket);
    connection._markOpen();
    return connection;
  }
}
