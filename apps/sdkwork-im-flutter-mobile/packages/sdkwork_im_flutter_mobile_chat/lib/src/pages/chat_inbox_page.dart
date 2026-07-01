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
  late Future<InboxResponse?> _inboxFuture;
  bool _liveConnected = false;

  @override
  void initState() {
    super.initState();
    _inboxFuture = widget.inboxService.fetchInbox();
    unawaited(_startInboxRealtime());
  }

  Future<void> _reloadInbox() async {
    setState(() {
      _inboxFuture = widget.inboxService.fetchInbox();
    });
    await _inboxFuture;
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
    return entry.displayName ??
        entry.peer?.displayName ??
        'Conversation ${entry.conversationId}';
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
      body: FutureBuilder<InboxResponse?>(
        future: _inboxFuture,
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return const Center(child: CircularProgressIndicator());
          }
          if (snapshot.hasError) {
            return Center(
              child: Padding(
                padding: const EdgeInsets.all(24),
                child: Text('Failed to load inbox: ${snapshot.error}'),
              ),
            );
          }

          final items = snapshot.data?.items ?? const <ConversationInboxEntry>[];
          if (items.isEmpty) {
            return const Center(child: Text('No conversations yet.'));
          }

          return ListView.separated(
            padding: const EdgeInsets.all(16),
            itemCount: items.length,
            separatorBuilder: (_, __) => const SizedBox(height: 8),
            itemBuilder: (context, index) {
              final entry = items[index];
              final updatedAt = entry.lastMessageAt ?? entry.lastActivityAt;
              return Card(
                child: ListTile(
                  title: Text(_entryTitle(entry)),
                  subtitle: entry.lastSummary == null || entry.lastSummary!.isEmpty
                      ? null
                      : Text(entry.lastSummary!),
                  trailing: Text(formatRelativeTime(updatedAt)),
                  onTap: () {
                    Navigator.of(context).push(
                      MaterialPageRoute<void>(
                        builder: (_) => ChatConversationPage(
                          conversationService: createChatConversationService(widget.imClients),
                          realtimeService: widget.realtimeService,
                          conversationId: entry.conversationId,
                          applicationPublicHttpUrl: widget.applicationPublicHttpUrl,
                          session: widget.session,
                          title: _entryTitle(entry),
                        ),
                      ),
                    );
                  },
                ),
              );
            },
          );
        },
      ),
    );
  }
}
