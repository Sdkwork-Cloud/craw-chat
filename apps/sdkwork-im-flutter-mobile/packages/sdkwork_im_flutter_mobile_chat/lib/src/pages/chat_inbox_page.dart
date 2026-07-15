import 'dart:async';

import 'package:flutter/material.dart';
import 'package:sdkwork_im_flutter_mobile_commons/sdkwork_im_flutter_mobile_commons.dart';
import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';
import 'package:sdkwork_im_flutter_mobile_shell/sdkwork_im_flutter_mobile_shell.dart';

import '../services/chat_conversation_service.dart';
import '../services/chat_inbox_service.dart';
import '../services/chat_realtime_service.dart';
import 'chat_conversation_page.dart';

class ChatInboxPage extends StatefulWidget {
  const ChatInboxPage({
    super.key,
    required this.inboxService,
    required this.imClients,
    required this.realtimeService,
    required this.userId,
    required this.applicationPublicHttpUrl,
    required this.session,
  });

  final ChatInboxService inboxService;
  final ImSdkClientBundle imClients;
  final ChatRealtimeService realtimeService;
  final String userId;
  final String applicationPublicHttpUrl;
  final ImAppSession session;

  @override
  State<ChatInboxPage> createState() => _ChatInboxPageState();
}

class _ChatInboxPageState extends State<ChatInboxPage> {
  final List<ConversationInboxEntry> _entries = [];
  String? _nextCursor;
  bool _hasMore = true;
  bool _loading = false;
  bool _initialLoadComplete = false;
  bool _liveConnected = false;
  bool _pendingRealtimeRefresh = false;
  String? _loadError;

  @override
  void initState() {
    super.initState();
    unawaited(_loadInboxPage(reset: true));
    unawaited(_startInboxRealtime());
  }

  Future<void> _loadInboxPage({required bool reset}) async {
    if (_loading) {
      return;
    }
    if (!reset && !_hasMore) {
      return;
    }
    setState(() {
      _loading = true;
      if (reset) {
        _loadError = null;
      }
    });
    try {
      final response = await widget.inboxService
          .fetchInboxPage(cursor: reset ? null : _nextCursor);
      final pageItems = response.items;
      final pageInfo = response.pageInfo;
      if (!mounted) {
        return;
      }
      setState(() {
        if (reset) {
          _entries
            ..clear()
            ..addAll(pageItems);
        } else {
          final merged = mergeConversationInboxEntries(_entries, pageItems);
          _entries
            ..clear()
            ..addAll(merged);
        }
        _hasMore = pageInfo.hasMore ?? false;
        _nextCursor = pageInfo.nextCursor;
        _initialLoadComplete = true;
        _loadError = null;
      });
    } catch (_) {
      if (!mounted) {
        return;
      }
      setState(() {
        _loadError = 'Unable to load conversations.';
        _initialLoadComplete = true;
      });
    } finally {
      if (mounted) {
        var shouldRefresh = false;
        setState(() {
          _loading = false;
          shouldRefresh = _pendingRealtimeRefresh;
          _pendingRealtimeRefresh = false;
        });
        if (shouldRefresh) {
          unawaited(_loadInboxPage(reset: true));
        }
      }
    }
  }

  Future<void> _reloadInbox() async {
    if (_loading) {
      _pendingRealtimeRefresh = true;
      return;
    }
    await _loadInboxPage(reset: true);
  }

  Future<void> _startInboxRealtime() async {
    try {
      await widget.realtimeService.startInbox(
        userId: widget.userId,
        onRefresh: _reloadInbox,
      );
      if (mounted) {
        setState(() => _liveConnected = widget.realtimeService.isLiveConnected);
      }
    } catch (_) {
      if (mounted) {
        setState(() => _liveConnected = false);
      }
    }
  }

  String _entryTitle(ConversationInboxEntry entry) {
    return resolveConversationInboxTitle(entry);
  }

  @override
  Widget build(BuildContext context) {
    return ImAppScaffold(
      title: 'Inbox',
      actions: [
        if (_liveConnected)
          const Padding(
            padding: EdgeInsets.only(right: 12),
            child: Center(
              child: Text('Live', style: TextStyle(fontSize: 12)),
            ),
          ),
      ],
      body: !_initialLoadComplete && _loading
          ? const Center(child: CircularProgressIndicator())
          : _loadError != null && _entries.isEmpty
              ? Center(
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Text(_loadError!, textAlign: TextAlign.center),
                      const SizedBox(height: 12),
                      FilledButton(
                        onPressed:
                            _loading ? null : () => _loadInboxPage(reset: true),
                        child: const Text('Retry'),
                      ),
                    ],
                  ),
                )
              : _entries.isEmpty
                  ? const Center(child: Text('No conversations yet.'))
                  : ListView.separated(
                      padding: const EdgeInsets.all(16),
                      itemCount: _entries.length + (_hasMore ? 1 : 0),
                      separatorBuilder: (_, __) => const SizedBox(height: 8),
                      itemBuilder: (context, index) {
                        if (index >= _entries.length) {
                          return TextButton(
                            onPressed: _loading
                                ? null
                                : () => _loadInboxPage(reset: false),
                            child: _loading
                                ? const SizedBox(
                                    width: 18,
                                    height: 18,
                                    child: CircularProgressIndicator(
                                        strokeWidth: 2),
                                  )
                                : const Text('Load more'),
                          );
                        }
                        final entry = _entries[index];
                        final updatedAt =
                            entry.lastMessageAt ?? entry.lastActivityAt;
                        final unreadCount = entry.unreadCount;
                        final isMarkedUnread =
                            entry.preferences?.isMarkedUnread ?? false;
                        final isUnread = unreadCount > 0 || isMarkedUnread;
                        final isMuted = entry.preferences?.isMuted ?? false;
                        return Card(
                          child: ListTile(
                            title: Row(
                              children: [
                                Expanded(child: Text(_entryTitle(entry))),
                                if (isMuted)
                                  const Padding(
                                    padding: EdgeInsets.only(left: 4),
                                    child: Icon(
                                        Icons.notifications_off_outlined,
                                        size: 16),
                                  ),
                              ],
                            ),
                            subtitle: entry.lastSummary == null ||
                                    entry.lastSummary!.isEmpty
                                ? null
                                : Text(entry.lastSummary!),
                            trailing: Column(
                              mainAxisAlignment: MainAxisAlignment.center,
                              crossAxisAlignment: CrossAxisAlignment.end,
                              children: [
                                Text(formatRelativeTime(updatedAt),
                                    style:
                                        Theme.of(context).textTheme.bodySmall),
                                if (isUnread)
                                  Padding(
                                    padding: const EdgeInsets.only(top: 4),
                                    child: CircleAvatar(
                                      radius: isMuted ? 4 : 10,
                                      backgroundColor: Colors.red,
                                      child: isMuted || unreadCount <= 0
                                          ? null
                                          : Text(
                                              unreadCount > 99
                                                  ? '99+'
                                                  : '$unreadCount',
                                              style: const TextStyle(
                                                  color: Colors.white,
                                                  fontSize: 10),
                                            ),
                                    ),
                                  ),
                              ],
                            ),
                            onTap: () {
                              unawaited(
                                widget.inboxService.markConversationRead(
                                  entry.conversationId,
                                  readSeq: entry.lastMessageSeq,
                                ),
                              );
                              Navigator.of(context).push(
                                MaterialPageRoute<void>(
                                  builder: (_) => ChatConversationPage(
                                    conversationService:
                                        createChatConversationService(
                                            widget.imClients),
                                    realtimeService: widget.realtimeService,
                                    conversationId: entry.conversationId,
                                    applicationPublicHttpUrl:
                                        widget.applicationPublicHttpUrl,
                                    session: widget.session,
                                    title: _entryTitle(entry),
                                  ),
                                ),
                              );
                            },
                          ),
                        );
                      },
                    ),
    );
  }
}
