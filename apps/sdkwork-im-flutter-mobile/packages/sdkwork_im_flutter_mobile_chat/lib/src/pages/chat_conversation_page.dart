import 'dart:async';

import 'package:flutter/material.dart';
import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';

import '../services/chat_conversation_service.dart';
import '../services/chat_realtime_service.dart';

class ChatConversationPage extends StatefulWidget {
  const ChatConversationPage({
    super.key,
    required this.conversationService,
    required this.realtimeService,
    required this.conversationId,
    this.title,
  });

  final ChatConversationService conversationService;
  final ChatRealtimeService realtimeService;
  final String conversationId;
  final String? title;

  @override
  State<ChatConversationPage> createState() => _ChatConversationPageState();
}

class _ChatConversationPageState extends State<ChatConversationPage> {
  late Future<TimelineResponse?> _timelineFuture;
  final _composerController = TextEditingController();
  bool _sending = false;
  bool _liveConnected = false;

  @override
  void initState() {
    super.initState();
    _timelineFuture = widget.conversationService.fetchTimeline(widget.conversationId);
    unawaited(_startRealtime());
  }

  Future<void> _startRealtime() async {
    try {
      await widget.realtimeService.startConversation(
        conversationId: widget.conversationId,
        onRefresh: _reloadTimeline,
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

  @override
  void dispose() {
    unawaited(widget.realtimeService.stopConversation());
    _composerController.dispose();
    super.dispose();
  }

  Future<void> _reloadTimeline() async {
    setState(() {
      _timelineFuture = widget.conversationService.fetchTimeline(widget.conversationId);
    });
    await _timelineFuture;
  }

  Future<void> _handleSend() async {
    final text = _composerController.text.trim();
    if (text.isEmpty || _sending) {
      return;
    }
    setState(() => _sending = true);
    try {
      await widget.conversationService.sendText(widget.conversationId, text);
      _composerController.clear();
      await _reloadTimeline();
    } catch (error) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Failed to send message: $error')),
        );
      }
    } finally {
      if (mounted) {
        setState(() => _sending = false);
      }
    }
  }

  String _entryLabel(TimelineViewEntry entry) {
    return entry.sender.displayName ?? entry.sender.id;
  }

  String _entryText(TimelineViewEntry entry) {
    return entry.body.text ?? entry.summary ?? '';
  }

  @override
  Widget build(BuildContext context) {
    final heading = widget.title ?? 'Conversation ${widget.conversationId}';

    return Scaffold(
      appBar: AppBar(
        title: Text(heading),
        actions: [
          if (_liveConnected)
            const Padding(
              padding: EdgeInsets.only(right: 12),
              child: Center(
                child: Text('Live', style: TextStyle(fontSize: 12)),
              ),
            ),
        ],
      ),
      body: Column(
        children: [
          Expanded(
            child: FutureBuilder<TimelineResponse?>(
              future: _timelineFuture,
              builder: (context, snapshot) {
                if (snapshot.connectionState == ConnectionState.waiting) {
                  return const Center(child: CircularProgressIndicator());
                }
                if (snapshot.hasError) {
                  return Center(child: Text('Failed to load messages: ${snapshot.error}'));
                }
                final items = snapshot.data?.items ?? const <TimelineViewEntry>[];
                if (items.isEmpty) {
                  return const Center(child: Text('No messages yet.'));
                }
                return ListView.separated(
                  padding: const EdgeInsets.all(16),
                  itemCount: items.length,
                  separatorBuilder: (_, __) => const SizedBox(height: 8),
                  itemBuilder: (context, index) {
                    final entry = items[index];
                    return Card(
                      child: ListTile(
                        title: Text(_entryLabel(entry)),
                        subtitle: Text(_entryText(entry)),
                        trailing: Text(
                          entry.occurredAt,
                          style: Theme.of(context).textTheme.bodySmall,
                        ),
                      ),
                    );
                  },
                );
              },
            ),
          ),
          SafeArea(
            top: false,
            child: Padding(
              padding: const EdgeInsets.all(12),
              child: Row(
                children: [
                  Expanded(
                    child: TextField(
                      controller: _composerController,
                      minLines: 1,
                      maxLines: 4,
                      decoration: const InputDecoration(
                        hintText: 'Type a message',
                        border: OutlineInputBorder(),
                      ),
                      onSubmitted: (_) => _handleSend(),
                    ),
                  ),
                  const SizedBox(width: 8),
                  FilledButton(
                    onPressed: _sending ? null : _handleSend,
                    child: Text(_sending ? 'Sending…' : 'Send'),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
