import 'dart:async';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';

import '../services/chat_conversation_service.dart';
import '../services/chat_media_upload_service.dart';
import '../services/chat_realtime_service.dart';
import '../services/chat_timeline_utils.dart';

class ChatConversationPage extends StatefulWidget {
  const ChatConversationPage({
    super.key,
    required this.conversationService,
    required this.realtimeService,
    required this.conversationId,
    required this.applicationPublicHttpUrl,
    required this.session,
    this.title,
  });

  final ChatConversationService conversationService;
  final ChatRealtimeService realtimeService;
  final String conversationId;
  final String applicationPublicHttpUrl;
  final ImAppSession session;
  final String? title;

  @override
  State<ChatConversationPage> createState() => _ChatConversationPageState();
}

class _ChatConversationPageState extends State<ChatConversationPage> {
  final _scrollController = ScrollController();
  final _composerController = TextEditingController();

  List<TimelineViewEntry> _entries = const [];
  TimelinePaginationState _pagination = const TimelinePaginationState(
    hasMore: false,
    nextAfterSeq: 0,
  );

  bool _loading = true;
  bool _loadingOlder = false;
  bool _uploading = false;
  bool _sending = false;
  bool _liveConnected = false;
  String? _error;
  int _latestSeq = 0;
  bool _loadingOlderGuard = false;

  @override
  void initState() {
    super.initState();
    _scrollController.addListener(_handleScroll);
    unawaited(_loadTimeline());
    unawaited(_startRealtime());
  }

  @override
  void dispose() {
    _scrollController.removeListener(_handleScroll);
    unawaited(widget.realtimeService.stopConversation());
    _scrollController.dispose();
    _composerController.dispose();
    super.dispose();
  }

  void _applyTimelineResponse(
    List<TimelineViewEntry> items,
    TimelinePaginationState pagination, {
    required String mode,
  }) {
    setState(() {
      _entries = mode == 'replace'
          ? items
          : mergeTimelineEntries(_entries, items);
      _pagination = pagination;
      _latestSeq = resolveLatestMessageSeq(_entries);
    });
  }

  Future<void> _loadTimeline({bool silent = false}) async {
    if (!silent) {
      setState(() {
        _loading = true;
        _error = null;
      });
    }

    try {
      final response = await widget.conversationService.fetchTimeline(
        widget.conversationId,
      );
      if (!mounted) {
        return;
      }
      _applyTimelineResponse(
        response?.items ?? const [],
        pickTimelinePagination(response),
        mode: 'replace',
      );
    } catch (error) {
      if (mounted) {
        setState(() => _error = 'Failed to load messages: $error');
      }
    } finally {
      if (mounted && !silent) {
        setState(() => _loading = false);
      }
    }
  }

  Future<void> _appendNewTimelineEntries() async {
    if (_latestSeq <= 0) {
      return;
    }
    try {
      final response = await widget.conversationService.fetchTimelineDelta(
        widget.conversationId,
        _latestSeq,
      );
      final items = response?.items ?? const [];
      if (items.isEmpty || !mounted) {
        return;
      }
      _applyTimelineResponse(
        items,
        pickTimelinePagination(response),
        mode: 'merge',
      );
    } catch (_) {
      // Keep existing timeline visible when incremental sync fails.
    }
  }

  Future<void> _loadOlderMessages() async {
    if (_loadingOlderGuard || !_pagination.hasMore) {
      return;
    }
    _loadingOlderGuard = true;
    setState(() => _loadingOlder = true);
    final previousHeight = _scrollController.position.maxScrollExtent;

    try {
      final response = await widget.conversationService.fetchTimeline(
        widget.conversationId,
        afterSeq: _pagination.nextAfterSeq,
        limit: 50,
      );
      if (!mounted) {
        return;
      }
      _applyTimelineResponse(
        response?.items ?? const [],
        pickTimelinePagination(response),
        mode: 'append',
      );
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (!_scrollController.hasClients) {
          return;
        }
        final nextHeight = _scrollController.position.maxScrollExtent;
        _scrollController.jumpTo(nextHeight - previousHeight);
      });
    } catch (error) {
      if (mounted) {
        setState(() => _error = 'Failed to load earlier messages: $error');
      }
    } finally {
      _loadingOlderGuard = false;
      if (mounted) {
        setState(() => _loadingOlder = false);
      }
    }
  }

  void _handleScroll() {
    if (!_scrollController.hasClients || _loadingOlderGuard || !_pagination.hasMore) {
      return;
    }
    if (_scrollController.position.pixels <= 48) {
      unawaited(_loadOlderMessages());
    }
  }

  Future<void> _startRealtime() async {
    try {
      await widget.realtimeService.startConversation(
        conversationId: widget.conversationId,
        onRefresh: _appendNewTimelineEntries,
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

  Future<void> _handleSend() async {
    final text = _composerController.text.trim();
    if (text.isEmpty || _sending) {
      return;
    }
    setState(() => _sending = true);
    try {
      await widget.conversationService.sendText(widget.conversationId, text);
      _composerController.clear();
      await _appendNewTimelineEntries();
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

  Future<void> _handleImageUpload() async {
    if (_uploading) {
      return;
    }

    final picked = await FilePicker.platform.pickFiles(
      type: FileType.image,
      withData: true,
    );
    final file = picked?.files.firstOrNull;
    final bytes = file?.bytes;
    if (bytes == null || bytes.isEmpty) {
      return;
    }

    setState(() => _uploading = true);
    try {
      final upload = await uploadChatMediaBytes(
        applicationPublicHttpUrl: widget.applicationPublicHttpUrl,
        conversationId: widget.conversationId,
        bytes: bytes,
        userId: widget.session.userId,
        accessToken: widget.session.accessToken,
        authToken: widget.session.authToken,
        originalFileName: file?.name,
        contentType: file?.extension == null ? 'image/jpeg' : 'image/${file!.extension}',
      );
      final fileName = file?.name ?? 'image';
      final mimeType = file?.extension == null ? 'image/jpeg' : 'image/${file!.extension}';
      await widget.conversationService.sendImageMessage(
        conversationId: widget.conversationId,
        driveUri: upload.driveUri,
        spaceId: upload.spaceId,
        nodeId: upload.nodeId,
        fileName: fileName,
        mimeType: mimeType,
        sizeBytes: bytes.length,
      );
      await _appendNewTimelineEntries();
    } catch (error) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Failed to upload image: $error')),
        );
      }
    } finally {
      if (mounted) {
        setState(() => _uploading = false);
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
            child: _loading
                ? const Center(child: CircularProgressIndicator())
                : _error != null
                    ? Center(child: Text(_error!))
                    : _entries.isEmpty
                        ? const Center(child: Text('No messages yet.'))
                        : ListView.separated(
                            controller: _scrollController,
                            padding: const EdgeInsets.all(16),
                            itemCount: _entries.length + (_loadingOlder ? 1 : 0),
                            separatorBuilder: (_, __) => const SizedBox(height: 8),
                            itemBuilder: (context, index) {
                              if (_loadingOlder && index == 0) {
                                return const Center(
                                  child: Padding(
                                    padding: EdgeInsets.symmetric(vertical: 8),
                                    child: CircularProgressIndicator(strokeWidth: 2),
                                  ),
                                );
                              }
                              final entryIndex = _loadingOlder ? index - 1 : index;
                              final entry = _entries[entryIndex];
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
                          ),
          ),
          SafeArea(
            top: false,
            child: Padding(
              padding: const EdgeInsets.all(12),
              child: Row(
                children: [
                  IconButton(
                    onPressed: _uploading ? null : _handleImageUpload,
                    icon: _uploading
                        ? const SizedBox(
                            width: 18,
                            height: 18,
                            child: CircularProgressIndicator(strokeWidth: 2),
                          )
                        : const Icon(Icons.image_outlined),
                  ),
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
