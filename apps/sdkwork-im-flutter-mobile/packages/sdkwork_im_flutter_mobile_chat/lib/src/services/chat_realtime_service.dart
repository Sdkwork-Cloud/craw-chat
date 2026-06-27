import 'dart:async';

import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';

typedef RealtimeRefreshHandler = Future<void> Function();

final _liveHubs = <int, _ChatLiveHub>{};

class _ChatLiveHub {
  _ChatLiveHub(this._bundle);

  final ImSdkClientBundle _bundle;
  ImLiveConnection? _connection;
  ImSubscription? _stateSubscription;
  bool _liveConnected = false;

  final Map<String, Set<RealtimeRefreshHandler>> _inboxHandlers = {};
  final Map<String, Set<RealtimeRefreshHandler>> _conversationHandlers = {};
  final Map<String, ImSubscription> _inboxUnsubs = {};
  final Map<String, ImSubscription> _conversationUnsubs = {};

  bool get isLiveConnected => _liveConnected;

  String _scopeKey(String scopeType, String scopeId) => '$scopeType:$scopeId';

  Future<ImLiveConnection> _ensureConnection() async {
    if (_connection != null) {
      return _connection!;
    }

    final connection = _bundle.composed.connect(
      options: const ImConnectOptions(subscriptions: ImConnectSubscriptions()),
    );
    _connection = connection;
    _stateSubscription = connection.lifecycle.onStateChange((state) {
      _liveConnected = state.status == 'open';
      if (_liveConnected && _connection != null) {
        _syncSubscriptions(_connection!);
      }
      if (state.status == 'closed' || state.status == 'error') {
        _connection = null;
        _liveConnected = false;
      }
    });
    return connection;
  }

  List<ImRealtimeScopeSubscription> _buildScopeSubscriptions() {
    return _inboxHandlers.keys.map((scopeKey) {
      final parts = scopeKey.split(':');
      final scopeType = parts.first;
      final scopeId = parts.sublist(1).join(':');
      return ImRealtimeScopeSubscription(
        scopeType: scopeType,
        scopeId: scopeId,
        eventTypes: inboxRealtimeEventTypes,
      );
    }).toList();
  }

  void _syncSubscriptions(ImLiveConnection connection) {
    if (!_liveConnected) {
      return;
    }
    connection.subscriptions.syncConversations(_conversationHandlers.keys.toList());
    connection.subscriptions.syncScopes(_buildScopeSubscriptions());
  }

  void _teardownIfIdle() {
    if (_inboxHandlers.isNotEmpty || _conversationHandlers.isNotEmpty) {
      return;
    }
    _stateSubscription?.call();
    _stateSubscription = null;
    _connection?.disconnect();
    _connection = null;
    _liveConnected = false;
  }

  Future<void> subscribeInbox({
    required String userId,
    required RealtimeRefreshHandler handler,
  }) async {
    final scopeKey = _scopeKey('user', userId);
    final connection = await _ensureConnection();
    var handlers = _inboxHandlers[scopeKey];
    if (handlers == null) {
      handlers = {};
      _inboxHandlers[scopeKey] = handlers;
      final unsubscribe = connection.events.onScope(
        'user',
        userId,
        (_) {
          for (final activeHandler in handlers ?? {}) {
            unawaited(activeHandler());
          }
        },
      );
      _inboxUnsubs[scopeKey] = unsubscribe;
      _syncSubscriptions(connection);
    }
    handlers.add(handler);
  }

  void unsubscribeInbox({
    required String userId,
    required RealtimeRefreshHandler handler,
  }) {
    final scopeKey = _scopeKey('user', userId);
    final handlers = _inboxHandlers[scopeKey];
    if (handlers == null) {
      return;
    }
    handlers.remove(handler);
    if (handlers.isNotEmpty) {
      return;
    }

    _inboxUnsubs.remove(scopeKey)?.call();
    _inboxHandlers.remove(scopeKey);
    if (_connection != null) {
      _syncSubscriptions(_connection!);
    }
    _teardownIfIdle();
  }

  Future<void> subscribeConversation({
    required String conversationId,
    required RealtimeRefreshHandler handler,
  }) async {
    final connection = await _ensureConnection();
    var handlers = _conversationHandlers[conversationId];
    if (handlers == null) {
      handlers = {};
      _conversationHandlers[conversationId] = handlers;
      final unsubscribe = connection.messages.onConversation(
        conversationId,
        (_) {
          for (final activeHandler in handlers ?? {}) {
            unawaited(activeHandler());
          }
        },
      );
      _conversationUnsubs[conversationId] = unsubscribe;
      _syncSubscriptions(connection);
    }
    handlers.add(handler);
  }

  void unsubscribeConversation({
    required String conversationId,
    required RealtimeRefreshHandler handler,
  }) {
    final handlers = _conversationHandlers[conversationId];
    if (handlers == null) {
      return;
    }
    handlers.remove(handler);
    if (handlers.isNotEmpty) {
      return;
    }

    _conversationUnsubs.remove(conversationId)?.call();
    _conversationHandlers.remove(conversationId);
    if (_connection != null) {
      _syncSubscriptions(_connection!);
    }
    _teardownIfIdle();
  }

  Future<void> dispose() async {
    for (final unsubscribe in _inboxUnsubs.values) {
      unsubscribe();
    }
    for (final unsubscribe in _conversationUnsubs.values) {
      unsubscribe();
    }
    _inboxUnsubs.clear();
    _conversationUnsubs.clear();
    _inboxHandlers.clear();
    _conversationHandlers.clear();
    _stateSubscription?.call();
    _stateSubscription = null;
    _connection?.disconnect();
    _connection = null;
    _liveConnected = false;
  }
}

_ChatLiveHub _hubForBundle(ImSdkClientBundle bundle) {
  return _liveHubs.putIfAbsent(
    identityHashCode(bundle.composed),
    () => _ChatLiveHub(bundle),
  );
}

class ChatRealtimeService {
  ChatRealtimeService(this._bundle);

  final ImSdkClientBundle _bundle;
  _ChatLiveHub get _hub => _hubForBundle(_bundle);

  RealtimeRefreshHandler? _inboxHandler;
  RealtimeRefreshHandler? _conversationHandler;
  String? _inboxUserId;
  String? _conversationId;

  bool get isLiveConnected => _hub.isLiveConnected;

  Future<void> startConversation({
    required String conversationId,
    required RealtimeRefreshHandler onRefresh,
  }) async {
    await stopConversation();
    _conversationId = conversationId;
    _conversationHandler = onRefresh;
    await _hub.subscribeConversation(
      conversationId: conversationId,
      handler: onRefresh,
    );
  }

  Future<void> startInbox({
    required String userId,
    required RealtimeRefreshHandler onRefresh,
  }) async {
    await stopInbox();
    _inboxUserId = userId;
    _inboxHandler = onRefresh;
    await _hub.subscribeInbox(userId: userId, handler: onRefresh);
  }

  Future<void> stopInbox() async {
    final userId = _inboxUserId;
    final handler = _inboxHandler;
    _inboxUserId = null;
    _inboxHandler = null;
    if (userId == null || handler == null) {
      return;
    }
    _hub.unsubscribeInbox(userId: userId, handler: handler);
  }

  Future<void> stopConversation() async {
    final conversationId = _conversationId;
    final handler = _conversationHandler;
    _conversationId = null;
    _conversationHandler = null;
    if (conversationId == null || handler == null) {
      return;
    }
    _hub.unsubscribeConversation(
      conversationId: conversationId,
      handler: handler,
    );
  }

  Future<void> stop() async {
    await stopInbox();
    await stopConversation();
  }
}

ChatRealtimeService createChatRealtimeService(ImSdkClientBundle bundle) {
  return ChatRealtimeService(bundle);
}

Future<void> disposeChatRealtimeHub(ImSdkClientBundle bundle) async {
  final hub = _liveHubs.remove(identityHashCode(bundle.composed));
  await hub?.dispose();
}
