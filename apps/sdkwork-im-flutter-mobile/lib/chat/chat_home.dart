import 'dart:async';

import 'package:flutter/material.dart';
import 'package:sdkwork_im_flutter_mobile_chat/sdkwork_im_flutter_mobile_chat.dart';
import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';

import '../bootstrap/sdk_clients.dart';

class ChatHome extends StatefulWidget {
  const ChatHome({super.key, required this.session});

  final ImAppSession session;

  @override
  State<ChatHome> createState() => _ChatHomeState();
}

class _ChatHomeState extends State<ChatHome> {
  late final ImSdkClientBundle _clientBundle;
  late final ChatRealtimeService _realtimeService;

  @override
  void initState() {
    super.initState();
    _clientBundle = getSdkClients().im;
    _realtimeService = createChatRealtimeService(_clientBundle);
  }

  @override
  void dispose() {
    unawaited(_realtimeService.stop());
    unawaited(disposeChatRealtimeHub(_clientBundle));
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final inboxService = createChatInboxService(_clientBundle);
    return ChatInboxPage(
      inboxService: inboxService,
      imClients: _clientBundle,
      realtimeService: _realtimeService,
      userId: widget.session.userId,
    );
  }
}
