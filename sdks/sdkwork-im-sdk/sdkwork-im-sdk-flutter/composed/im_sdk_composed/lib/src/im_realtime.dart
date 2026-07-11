import 'dart:async';
import 'dart:convert';

import 'package:web_socket_channel/io.dart';

import 'ccp_wire.dart';
import 'transport.dart';

typedef ImSubscription = void Function();

class ImLiveConnectionState {
  const ImLiveConnectionState({required this.status, this.reason});

  final String status;
  final String? reason;
}

class ImConnectOptions {
  const ImConnectOptions({
    this.deviceId,
    this.subscriptions,
    this.connectionTimeoutMs = 15000,
    this.authTimeoutMs = 15000,
  });

  final String? deviceId;
  final ImConnectSubscriptions? subscriptions;
  final int connectionTimeoutMs;
  final int authTimeoutMs;
}

class ImConnectSubscriptions {
  const ImConnectSubscriptions({
    this.conversations = const [],
    this.scopes = const [],
  });

  final List<String> conversations;
  final List<ImRealtimeScopeSubscription> scopes;
}

class ImRealtimeScopeSubscription {
  const ImRealtimeScopeSubscription({
    required this.scopeType,
    required this.scopeId,
    this.eventTypes = const [],
  });

  final String scopeType;
  final String scopeId;
  final List<String> eventTypes;
}

const inboxRealtimeEventTypes = <String>[
  'message.posted',
  'conversation.updated',
  'conversation.created',
  'conversation.member_joined',
  'conversation.member_role_changed',
  'conversation.member_removed',
  'conversation.member_left',
  'conversation.owner_transferred',
];

class ImLiveConnection {
  ImLiveConnection._({
    required this.disconnect,
    required this.events,
    required this.messages,
    required this.subscriptions,
    required this.lifecycle,
  });

  final void Function([int code, String reason]) disconnect;
  final ImLiveConnectionEvents events;
  final ImLiveConnectionMessages messages;
  final ImLiveConnectionSubscriptions subscriptions;
  final ImLiveConnectionLifecycle lifecycle;
}

class ImLiveConnectionEvents {
  ImLiveConnectionEvents({required this.onScope});

  final ImSubscription Function(
    String scopeType,
    String scopeId,
    void Function(Map<String, dynamic> event) handler,
  ) onScope;
}

class ImLiveConnectionMessages {
  ImLiveConnectionMessages({required this.onConversation});

  final ImSubscription Function(
    String conversationId,
    void Function(Map<String, dynamic> message) handler,
  ) onConversation;
}

class ImLiveConnectionSubscriptions {
  ImLiveConnectionSubscriptions({
    required this.syncConversations,
    required this.syncScopes,
  });

  final void Function(List<String> conversationIds) syncConversations;
  final void Function(List<ImRealtimeScopeSubscription> scopes) syncScopes;
}

class ImLiveConnectionLifecycle {
  ImLiveConnectionLifecycle({required this.onStateChange});

  final ImSubscription Function(void Function(ImLiveConnectionState state) handler) onStateChange;
}

class ImCreateLiveConnectionParams {
  const ImCreateLiveConnectionParams({
    required this.websocketBaseUrl,
    this.accessToken,
    this.authToken,
    this.headers = const {},
    this.options = const ImConnectOptions(),
    this.transport,
  });

  final String websocketBaseUrl;
  final String? accessToken;
  final String? authToken;
  final Map<String, String> headers;
  final ImConnectOptions options;

  /// Optional pre-built transport connection. When provided, the realtime
  /// layer uses this transport instead of creating a new WebSocket. This
  /// enables TCP/UDP/custom transports. When null, falls back to the
  /// existing WebSocket path (backward compatible).
  final ImTransportConnection? transport;
}

ImLiveConnection createImLiveConnection(ImCreateLiveConnectionParams params) {
  final stateHandlers = <void Function(ImLiveConnectionState)>[];
  final messageHandlers = <String, Set<void Function(Map<String, dynamic>)>>{};
  final eventHandlers = <String, Set<void Function(Map<String, dynamic>)>>{};
  final subscriptionConversations = <String>{...params.options.subscriptions?.conversations ?? const []};
  final subscriptionScopes = <ImRealtimeScopeSubscription>[
    ...params.options.subscriptions?.scopes ?? const [],
  ];
  // Replay the complete desired snapshot after every handshake, including an
  // empty snapshot. A resumed server session may otherwise retain old scopes.
  var subscriptionDirty = true;

  var currentState = const ImLiveConnectionState(status: 'connecting');
  final transport = params.transport;
  final usingTransport = transport != null;
  // Determine the CCP binding id from the transport capabilities, defaulting
  // to the WebSocket binding for backward compatibility.
  final ccpBinding = transport != null
      ? _ccpBindingLabelFromId(transport.capabilities.ccpBinding)
      : ccpWsBinding;
  // TCP/UDP have no HTTP upgrade auth; credentials are passed via the CCP
  // auth_bind frame. WebSocket still uses headers to decide frameAuthRequired.
  final isNonWebSocketTransport =
      usingTransport && transport.kind != ImTransportKind.websocket;
  final headers = _buildWebSocketHeaders(
    accessToken: params.accessToken,
    authToken: params.authToken,
    headers: params.headers,
  );
  final frameAuthRequired =
      isNonWebSocketTransport ? false : !_hasUpgradeAuthHeaders(headers);
  var connectionPhase = frameAuthRequired ? 'gateway_auth' : 'ccp_hello_ack';
  ImCcpAuthBindContext? pendingBindContext;
  var resumeNegotiated = false;
  Timer? authTimeout;
  Timer? connectionTimeout;
  Timer? heartbeatTimer;
  var heartbeatCounter = 0;
  var lastInboundAt = DateTime.now();
  var suppressNextClosedState = false;
  var authInitSent = false;
  var openNotified = false;
  var closeNotified = false;
  var closeRequested = false;

  // Socket abstraction: either transport-backed (TCP/UDP/custom) or the
  // existing WebSocket path. Both expose the same send/close/subscribe
  // interface so the CCP state machine is transport-agnostic.
  late final void Function(String data) socketSend;
  late final void Function(int code, String reason) socketClose;
  late final void Function(
    void Function(String event) onData,
    void Function() onDone,
    void Function(Object error) onError,
  ) socketSubscribe;
  late final void Function(void Function() onOpen) socketOnOpen;

  if (transport != null) {
    socketSend = (data) {
      transport.send(ImTransportFrame(data: data, isBinary: false));
    };
    socketClose = (code, reason) {
      transport.close(code: code, reason: reason);
    };
    socketSubscribe = (onData, onDone, onError) {
      transport.onMessage((frame) {
        if (frame.data is String) {
          onData(frame.data as String);
        }
      });
      transport.onClose((_) => onDone());
      transport.onError((event) => onError(event.error));
    };
    socketOnOpen = (onOpen) {
      // 如果传输已关闭（极端竞态），不注册 open 回调；
      // close 事件会通过 socketSubscribe 的 onDone 派发。
      if (transport.state == ImTransportConnectionState.closed) {
        return;
      }
      // 如果传输已经 open，立即调度回调（late-registration）。
      // transport.onOpen 内部有 _openDispatched 检查，但 constructor 的
      // scheduleMicrotask 可能在 onOpen 注册前就已完成。
      if (transport.state == ImTransportConnectionState.open) {
        scheduleMicrotask(() {
          if (transport.state == ImTransportConnectionState.open) {
            onOpen();
          }
        });
        return;
      }
      transport.onOpen(onOpen);
    };
  } else {
    final uri = _buildWebSocketUri(params.websocketBaseUrl, params.options.deviceId);
    final channel = IOWebSocketChannel.connect(
      uri,
      headers: headers,
      protocols: [imCcpWebSocketSubprotocol],
    );
    socketSend = (data) {
      channel.sink.add(data);
    };
    socketClose = (code, reason) {
      channel.sink.close(code, reason);
    };
    socketSubscribe = (onData, onDone, onError) {
      channel.stream.listen(
        (event) {
          if (event is String) {
            onData(event);
          }
        },
        onDone: onDone,
        onError: (e) => onError(e),
      );
    };
    socketOnOpen = (onOpen) {
      // IOWebSocketChannel.ready completes when the WebSocket handshake
      // succeeds. Use it to drive the open callback.
      channel.ready.then(
        (_) => onOpen(),
        onError: (_) {
          // Swallow: the stream's onError listener already surfaces errors.
        },
      );
    };
  }

  void emitState(ImLiveConnectionState state) {
    currentState = state;
    for (final handler in List.of(stateHandlers)) {
      handler(state);
    }
  }

  void clearTimers() {
    authTimeout?.cancel();
    authTimeout = null;
    connectionTimeout?.cancel();
    connectionTimeout = null;
    heartbeatTimer?.cancel();
    heartbeatTimer = null;
  }

  void closeSocket(int code, String reason) {
    if (closeRequested) {
      return;
    }
    closeRequested = true;
    clearTimers();
    try {
      socketClose(code, reason);
    } catch (error) {
      if (!closeNotified) {
        closeNotified = true;
        emitState(ImLiveConnectionState(status: 'closed', reason: '$error'));
      }
    }
  }

  void sendSocket(String data) {
    if (closeRequested) {
      return;
    }
    try {
      socketSend(data);
    } catch (error) {
      emitState(ImLiveConnectionState(status: 'error', reason: '$error'));
      closeSocket(4000, 'transport_send_failed');
    }
  }

  void failAuth(String code, String message) {
    connectionPhase = 'gateway_auth';
    clearTimers();
    emitState(ImLiveConnectionState(status: 'error', reason: message));
    closeSocket(4401, code);
  }

  void failCcpHandshake(String code, String message) {
    connectionPhase = 'gateway_auth';
    clearTimers();
    emitState(ImLiveConnectionState(status: 'error', reason: message));
    closeSocket(4401, code);
  }

  void scheduleAuthTimeout(void Function() onTimeout) {
    authTimeout?.cancel();
    authTimeout = Timer(Duration(milliseconds: params.options.authTimeoutMs), onTimeout);
  }

  void beginCcpHandshake({Map<String, dynamic>? authOk}) {
    final bindContext = resolveCcpAuthBindContext(
      accessToken: params.accessToken,
      authOk: authOk,
      deviceId: params.options.deviceId,
    );
    if (bindContext == null) {
      failCcpHandshake(
        'websocket_ccp_auth_bind_unavailable',
        'websocket CCP auth_bind context is unavailable',
      );
      return;
    }
    pendingBindContext = bindContext;
    connectionPhase = 'ccp_hello_ack';
    sendSocket(encodeCcpHelloFrame(binding: ccpBinding));
    scheduleAuthTimeout(() => failCcpHandshake(
      'websocket_ccp_handshake_timeout',
      'websocket CCP handshake was not completed before timeout',
    ));
  }

  void startHeartbeat() {
    heartbeatTimer?.cancel();
    heartbeatTimer = Timer.periodic(const Duration(seconds: 30), (_) {
      if (heartbeatCounter > 0 &&
          DateTime.now().difference(lastInboundAt) > const Duration(seconds: 75)) {
        emitState(const ImLiveConnectionState(
          status: 'error',
          reason: 'websocket heartbeat response was not received before timeout',
        ));
        closeSocket(4408, 'websocket_heartbeat_timeout');
        return;
      }
      heartbeatCounter += 1;
      sendSocket(encodeCcpHeartbeatFrame(heartbeatCounter, binding: ccpBinding));
    });
  }

  void flushSubscriptionSync() {
    if (!subscriptionDirty || connectionPhase != 'ready') {
      return;
    }
    subscriptionDirty = false;
    final conversationScopes = subscriptionConversations
        .map((conversationId) => <String, dynamic>{
          'scopeType': 'conversation',
          'scopeId': conversationId,
          'eventTypes': <String>['message.posted'],
        })
        .toList();
    final scopeItems = subscriptionScopes
        .map((scope) => <String, dynamic>{
          'scopeType': scope.scopeType,
          'scopeId': scope.scopeId,
          'eventTypes': scope.eventTypes,
        })
        .toList();
    sendSocket(
      encodeCcpBusinessFrame(
        'cc.realtime.subscriptions.sync.v1',
        'cmd',
        <String, dynamic>{
          'type': 'subscriptions.sync',
          'items': <dynamic>[...conversationScopes, ...scopeItems],
        },
        binding: ccpBinding,
      ),
    );
  }

  String realtimeScopeKey(String scopeType, String scopeId) => '$scopeType:$scopeId';

  void emitOpenAndSyncSubscriptions() {
    clearTimers();
    connectionPhase = 'ready';
    emitState(const ImLiveConnectionState(status: 'open'));
    startHeartbeat();
    flushSubscriptionSync();
  }

  void beginSessionResumeIfNeeded() {
    final sessionId = pendingBindContext?.sessionId;
    if (!resumeNegotiated || sessionId == null || sessionId.isEmpty) {
      pendingBindContext = null;
      emitOpenAndSyncSubscriptions();
      return;
    }
    connectionPhase = 'ccp_session_resume';
    sendSocket(encodeCcpSessionResumeFrame(sessionId, binding: ccpBinding));
    scheduleAuthTimeout(() => failCcpHandshake(
      'websocket_ccp_handshake_timeout',
      'websocket CCP handshake was not completed before timeout',
    ));
  }

  void sendAuthInit() {
    if (authInitSent) {
      return;
    }
    authInitSent = true;
    if (params.accessToken == null || params.authToken == null) {
      failAuth('websocket_auth_tokens_not_ready', 'websocket auth tokens are not ready');
      return;
    }
    sendSocket(jsonEncode(<String, dynamic>{
      'type': 'auth.init',
      'authToken': params.authToken,
      'accessToken': params.accessToken,
      if (params.options.deviceId != null) 'deviceId': params.options.deviceId,
    }));
    scheduleAuthTimeout(() => failAuth(
      'websocket_auth_timeout',
      'websocket auth.ok was not received before timeout',
    ));
  }

  void handleInbound(String raw) {
    lastInboundAt = DateTime.now();
    if (frameAuthRequired && connectionPhase == 'gateway_auth' && !authInitSent) {
      sendAuthInit();
    }

    if (connectionPhase == 'gateway_auth') {
      final frame = _parseControlFrame(raw);
      if (_pickString(frame?['type']) == 'auth.ok') {
        authTimeout?.cancel();
        beginCcpHandshake(authOk: frame);
        return;
      }
      final error = _parseControlError(raw);
      if (error != null) {
        failAuth(error['code'] as String, error['message'] as String);
      }
      return;
    }
    if (connectionPhase == 'ccp_hello_ack') {
      if (isCcpHelloAckEnvelope(raw)) {
        resumeNegotiated = ccpHelloAckNegotiatesSessionResume(raw);
        final bindContext = pendingBindContext;
        if (bindContext == null) {
          failCcpHandshake(
            'websocket_ccp_auth_bind_unavailable',
            'websocket CCP auth_bind context is unavailable',
          );
          return;
        }
        connectionPhase = 'ccp_auth_ok';
        sendSocket(encodeCcpAuthBindFrame(bindContext, binding: ccpBinding));
        return;
      }
      final error = _parseControlError(raw);
      if (error != null) {
        failCcpHandshake(error['code'] as String, error['message'] as String);
      }
      return;
    }
    if (connectionPhase == 'ccp_auth_ok') {
      if (isCcpAuthOkEnvelope(raw)) {
        beginSessionResumeIfNeeded();
        return;
      }
      final error = _parseControlError(raw);
      if (error != null) {
        failCcpHandshake(error['code'] as String, error['message'] as String);
      }
      return;
    }
    if (connectionPhase == 'ccp_session_resume') {
      if (isCcpSessionResumedEnvelope(raw)) {
        pendingBindContext = null;
        emitOpenAndSyncSubscriptions();
        return;
      }
      final error = _parseControlError(raw);
      if (error != null) {
        failCcpHandshake(error['code'] as String, error['message'] as String);
      }
      return;
    }

    final inboundFrame = unwrapInboundRealtimeFrame(raw);
    final controlError = _parseControlError(inboundFrame);
    if (controlError != null && _isFatalControlError(controlError['code'] as String)) {
      clearTimers();
      emitState(ImLiveConnectionState(
        status: 'error',
        reason: controlError['message'] as String,
      ));
      closeSocket(4401, controlError['code'] as String);
      return;
    }

    for (final message in _parseRealtimeMessages(inboundFrame)) {
      final conversationId = _pickString([
        message['conversationId'],
        message['scopeId'],
      ]);
      if (conversationId == null) {
        continue;
      }
      final handlers = messageHandlers[conversationId];
      if (handlers == null) {
        continue;
      }
      for (final handler in List.of(handlers)) {
        handler(message);
      }
    }

    for (final event in _parseRealtimeEvents(inboundFrame)) {
      final scopeType = _pickString([event['scopeType'], event['scope']]);
      final scopeId = _pickString([event['scopeId'], event['conversationId']]);
      if (scopeType == null || scopeId == null) {
        continue;
      }
      final handlers = eventHandlers[realtimeScopeKey(scopeType, scopeId)];
      if (handlers == null) {
        continue;
      }
      for (final handler in List.of(handlers)) {
        handler(event);
      }
    }
  }

  connectionTimeout = Timer(
    Duration(milliseconds: params.options.connectionTimeoutMs),
    () {
      suppressNextClosedState = true;
      emitState(const ImLiveConnectionState(
        status: 'error',
        reason: 'websocket connection was not established before timeout',
      ));
      closeSocket(4408, 'websocket_connect_timeout');
    },
  );

  socketSubscribe(
    (event) {
      handleInbound(event);
    },
    () {
      if (closeNotified) {
        return;
      }
      closeNotified = true;
      closeRequested = true;
      clearTimers();
      if (suppressNextClosedState) {
        suppressNextClosedState = false;
        return;
      }
      emitState(const ImLiveConnectionState(status: 'closed'));
    },
    (error) {
      if (closeNotified || closeRequested) {
        return;
      }
      clearTimers();
      emitState(ImLiveConnectionState(status: 'error', reason: '$error'));
      closeSocket(4000, 'transport_error');
    },
  );

  // 使用 open 事件（而非固定延迟）启动握手，确保传输层真正就绪后再发送 CCP 帧。
  // 对 WebSocket：channel.ready 完成后触发；对 TCP/UDP：transport.onOpen 触发。
  socketOnOpen(() {
    if (openNotified || closeRequested || closeNotified) {
      return;
    }
    openNotified = true;
    connectionTimeout?.cancel();
    connectionTimeout = null;
    if (frameAuthRequired) {
      sendAuthInit();
    } else {
      beginCcpHandshake();
    }
  });

  return ImLiveConnection._(
    disconnect: ([int code = 1000, String reason = 'client disconnect']) {
      closeSocket(code, reason);
    },
    events: ImLiveConnectionEvents(
      onScope: (scopeType, scopeId, handler) {
        final key = realtimeScopeKey(scopeType, scopeId);
        final handlers = eventHandlers.putIfAbsent(
          key,
          () => <void Function(Map<String, dynamic>)>{},
        );
        handlers.add(handler);
        subscriptionScopes.removeWhere(
          (scope) => scope.scopeType == scopeType && scope.scopeId == scopeId,
        );
        subscriptionScopes.add(ImRealtimeScopeSubscription(
          scopeType: scopeType,
          scopeId: scopeId,
          eventTypes: inboxRealtimeEventTypes,
        ));
        subscriptionDirty = true;
        if (connectionPhase == 'ready') {
          flushSubscriptionSync();
        }
        return () {
          handlers.remove(handler);
          if (handlers.isEmpty) {
            eventHandlers.remove(key);
            subscriptionScopes.removeWhere(
              (scope) => scope.scopeType == scopeType && scope.scopeId == scopeId,
            );
            subscriptionDirty = true;
            if (connectionPhase == 'ready') {
              flushSubscriptionSync();
            }
          }
        };
      },
    ),
    messages: ImLiveConnectionMessages(
      onConversation: (conversationId, handler) {
        final handlers = messageHandlers.putIfAbsent(
          conversationId,
          () => <void Function(Map<String, dynamic>)>{},
        );
        handlers.add(handler);
        subscriptionConversations.add(conversationId);
        subscriptionDirty = true;
        if (connectionPhase == 'ready') {
          flushSubscriptionSync();
        }
        return () {
          handlers.remove(handler);
          if (handlers.isEmpty) {
            messageHandlers.remove(conversationId);
            subscriptionConversations.remove(conversationId);
            subscriptionDirty = true;
            if (connectionPhase == 'ready') {
              flushSubscriptionSync();
            }
          }
        };
      },
    ),
    subscriptions: ImLiveConnectionSubscriptions(
      syncConversations: (conversationIds) {
        subscriptionConversations
          ..clear()
          ..addAll(conversationIds);
        subscriptionDirty = true;
        if (connectionPhase == 'ready') {
          flushSubscriptionSync();
        }
      },
      syncScopes: (scopes) {
        subscriptionScopes
          ..clear()
          ..addAll(scopes);
        subscriptionDirty = true;
        if (connectionPhase == 'ready') {
          flushSubscriptionSync();
        }
      },
    ),
    lifecycle: ImLiveConnectionLifecycle(
      onStateChange: (handler) {
        stateHandlers.add(handler);
        handler(currentState);
        return () => stateHandlers.remove(handler);
      },
    ),
  );
}

Uri _buildWebSocketUri(String websocketBaseUrl, String? deviceId) {
  final parsed = Uri.parse(websocketBaseUrl);
  final path = parsed.path.endsWith(imRealtimeWsPath)
      ? parsed.path
      : '${parsed.path.replaceAll(RegExp(r'/+$'), '')}$imRealtimeWsPath';
  final query = Map<String, String>.from(parsed.queryParameters);
  if (deviceId != null && deviceId.isNotEmpty) {
    query['deviceId'] = deviceId;
  }
  return parsed.replace(path: path, queryParameters: query);
}

/// Maps an [ImCcpBindingId] enum to its wire-protocol string label.
String _ccpBindingLabelFromId(ImCcpBindingId id) {
  switch (id) {
    case ImCcpBindingId.ws1:
      return ccpWsBinding;
    case ImCcpBindingId.tcp1:
      return ccpTcpBinding;
    case ImCcpBindingId.udp1:
      return ccpUdpBinding;
    case ImCcpBindingId.quic1:
      return 'Quic1';
  }
}

Map<String, String> _buildWebSocketHeaders({
  String? accessToken,
  String? authToken,
  Map<String, String> headers = const {},
}) {
  final resolved = Map<String, String>.from(headers);
  if (authToken != null && authToken.isNotEmpty) {
    resolved['Authorization'] = authToken.startsWith('Bearer ') ? authToken : 'Bearer $authToken';
  }
  if (accessToken != null && accessToken.isNotEmpty) {
    resolved['Access-Token'] = accessToken;
  }
  return resolved;
}

bool _hasUpgradeAuthHeaders(Map<String, String> headers) {
  for (final entry in headers.entries) {
    final key = entry.key.toLowerCase();
    final value = entry.value.trim();
    if (value.isEmpty) {
      continue;
    }
    if (key == 'authorization' || key == 'access-token') {
      return true;
    }
  }
  return false;
}

Map<String, dynamic>? _parseControlFrame(String raw) {
  try {
    final parsed = jsonDecode(raw);
    if (parsed is Map) {
      return Map<String, dynamic>.from(parsed);
    }
  } catch (_) {}
  return null;
}

Map<String, dynamic>? _parseControlError(String raw) {
  final frame = _parseControlFrame(raw);
  if (_pickString(frame?['type']) != 'error') {
    return null;
  }
  return <String, dynamic>{
    'code': _pickString(frame?['code']) ?? 'websocket_error',
    'message': _pickString([frame?['message'], frame?['detail']]) ?? 'websocket error',
  };
}

bool _isFatalControlError(String code) {
  if (code == 'reconnect_required') {
    return true;
  }
  return RegExp(r'^websocket_(?:auth|upstream|connect)').hasMatch(code);
}

List<Map<String, dynamic>> _parseRealtimeEvents(String raw) {
  try {
    final parsed = jsonDecode(raw);
    if (parsed is! Map) {
      return const [];
    }
    final frame = Map<String, dynamic>.from(parsed);
    if (_pickString([frame['type']]) == 'event.window') {
      final window = frame['window'];
      if (window is! Map) {
        return const [];
      }
      final items = window['items'];
      if (items is! List) {
        return const [];
      }
      return items
          .whereType<Map>()
          .map((item) => Map<String, dynamic>.from(item))
          .toList();
    }
    return [frame];
  } catch (_) {
    return const [];
  }
}

List<Map<String, dynamic>> _parseRealtimeMessages(String raw) {
  try {
    final parsed = jsonDecode(raw);
    if (parsed is! Map) {
      return const [];
    }
    final frame = Map<String, dynamic>.from(parsed);
    if (_pickString(frame['type']) == 'event.window') {
      final window = frame['window'];
      if (window is! Map) {
        return const [];
      }
      final items = window['items'];
      if (items is! List) {
        return const [];
      }
      final messages = <Map<String, dynamic>>[];
      for (final item in items) {
        if (item is! Map) {
          continue;
        }
        final event = Map<String, dynamic>.from(item);
        final eventType = _pickString([event['eventType'], event['type']]);
        if (eventType != null && eventType != 'message.posted') {
          continue;
        }
        final payload = _parseRecordPayload(event['payload']) ?? event;
        final conversationId = _pickString([
          payload['conversationId'],
          event['scopeId'],
          event['conversationId'],
        ]);
        if (conversationId == null) {
          continue;
        }
        messages.add(<String, dynamic>{
          ...payload,
          'conversationId': conversationId,
          'messageId': _pickString([payload['messageId'], event['messageId'], event['eventId']]),
          'occurredAt': _pickString([payload['occurredAt'], event['occurredAt']]),
        });
      }
      return messages;
    }

    final payload = _parseRecordPayload(frame['payload']) ?? frame;
    final conversationId = _pickString([
      payload['conversationId'],
      frame['scopeId'],
      frame['conversationId'],
    ]);
    if (conversationId == null) {
      return const [];
    }
    return <Map<String, dynamic>>[
      <String, dynamic>{
        ...payload,
        'conversationId': conversationId,
        'messageId': _pickString([payload['messageId'], frame['messageId'], frame['eventId']]),
        'occurredAt': _pickString([payload['occurredAt'], frame['occurredAt']]),
      },
    ];
  } catch (_) {
    return const [];
  }
}

Map<String, dynamic>? _parseRecordPayload(dynamic value) {
  if (value is Map) {
    return Map<String, dynamic>.from(value);
  }
  if (value is String && value.trim().isNotEmpty) {
    try {
      final parsed = jsonDecode(value);
      if (parsed is Map) {
        return Map<String, dynamic>.from(parsed);
      }
    } catch (_) {}
  }
  return null;
}

String? _pickString(List<dynamic> values) {
  for (final value in values) {
    if (value is String && value.trim().isNotEmpty) {
      return value.trim();
    }
  }
  return null;
}

String resolveImWebSocketBaseUrl(String httpBaseUrl) {
  final trimmed = httpBaseUrl.trim().replaceAll(RegExp(r'/+$'), '');
  if (trimmed.startsWith('ws://') || trimmed.startsWith('wss://')) {
    return trimmed;
  }
  if (trimmed.startsWith('https://')) {
    return trimmed.replaceFirst('https://', 'wss://');
  }
  if (trimmed.startsWith('http://')) {
    return trimmed.replaceFirst('http://', 'ws://');
  }
  return trimmed;
}
