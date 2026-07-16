import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';

import 'chat_sdk_response_utils.dart';

const int inboxPageSize = 20;
const int maxInboxSyncPages = 10;
const int maxInboxEntries = 200;

enum InboxWindowDirection { older, newer }

class InboxPageMergeResult {
  const InboxPageMergeResult({
    required this.items,
    required this.incomingPageRetained,
  });

  final List<ConversationInboxEntry> items;
  final bool incomingPageRetained;
}

class ChatInboxResult {
  const ChatInboxResult({
    required this.items,
    required this.pageInfo,
  });

  final List<ConversationInboxEntry> items;
  final PageInfo pageInfo;
}

String resolveConversationInboxTitle(
  ConversationInboxEntry entry, {
  String fallback = 'Conversation',
}) {
  final displayName = entry.displayName?.trim();
  if (displayName != null && displayName.isNotEmpty) {
    return displayName;
  }
  final peerDisplayName = entry.peer?.displayName?.trim();
  if (peerDisplayName != null && peerDisplayName.isNotEmpty) {
    return peerDisplayName;
  }
  return fallback;
}

InboxPageMergeResult mergeConversationInboxPage(
  Iterable<ConversationInboxEntry> current,
  Iterable<ConversationInboxEntry> incoming, {
  required InboxWindowDirection direction,
}) {
  final currentEntries = current.toList(growable: false);
  final incomingById = <String, ConversationInboxEntry>{
    for (final entry in incoming) entry.conversationId: entry,
  };
  final incomingEntries = incomingById.values.toList(growable: false);
  final currentIds =
      currentEntries.map((entry) => entry.conversationId).toSet();
  final merged = direction == InboxWindowDirection.older
      ? <ConversationInboxEntry>[
          for (final entry in currentEntries)
            incomingById[entry.conversationId] ?? entry,
          for (final entry in incomingEntries)
            if (!currentIds.contains(entry.conversationId)) entry,
        ]
      : <ConversationInboxEntry>[
          ...incomingEntries,
          for (final entry in currentEntries)
            if (!incomingById.containsKey(entry.conversationId)) entry,
        ];
  final items = direction == InboxWindowDirection.older
      ? merged.length <= maxInboxEntries
          ? merged
          : merged.sublist(merged.length - maxInboxEntries)
      : merged.take(maxInboxEntries).toList(growable: false);
  final retainedIds = items.map((entry) => entry.conversationId).toSet();
  final incomingPageRetained = incomingEntries.every(
    (entry) =>
        currentIds.contains(entry.conversationId) ||
        retainedIds.contains(entry.conversationId),
  );
  return InboxPageMergeResult(
    items: items,
    incomingPageRetained: incomingPageRetained,
  );
}

List<ConversationInboxEntry> mergeConversationInboxEntries(
  Iterable<ConversationInboxEntry> current,
  Iterable<ConversationInboxEntry> incoming, {
  InboxWindowDirection direction = InboxWindowDirection.older,
}) {
  return mergeConversationInboxPage(
    current,
    incoming,
    direction: direction,
  ).items;
}

int _normalizeInboxPageSize(int pageSize) {
  if (pageSize <= 0) {
    return inboxPageSize;
  }
  return pageSize > maxInboxEntries ? maxInboxEntries : pageSize;
}

class ChatInboxService {
  ChatInboxService(this._client);

  final SdkworkImClient _client;

  Future<ChatInboxResult> fetchInbox({int pageSize = inboxPageSize}) {
    return fetchInboxPage(pageSize: pageSize);
  }

  Future<ChatInboxResult> fetchInboxPage({
    int pageSize = inboxPageSize,
    String? cursor,
  }) async {
    final response = await _client.chat.inboxList(
      _normalizeInboxPageSize(pageSize),
      cursor,
    );
    return readInboxPageFromSdkResponse(response);
  }

  Future<List<ConversationInboxEntry>> fetchInboxEntries({
    int pageSize = inboxPageSize,
    int maxPages = maxInboxSyncPages,
  }) async {
    final items = <ConversationInboxEntry>[];
    String? cursor;
    for (var page = 0;
        page < maxPages && items.length < maxInboxEntries;
        page += 1) {
      final remaining = maxInboxEntries - items.length;
      final requestedPageSize = _normalizeInboxPageSize(pageSize);
      final response = await fetchInboxPage(
        pageSize: requestedPageSize > remaining ? remaining : requestedPageSize,
        cursor: cursor,
      );
      final pageItems = response.items;
      if (pageItems.length > remaining) {
        throw StateError('Inbox page exceeds the remaining bounded window.');
      }
      items.addAll(pageItems);
      final pageInfo = response.pageInfo;
      final hasMore = pageInfo.hasMore ?? false;
      final nextCursor = pageInfo.nextCursor;
      if (!hasMore || nextCursor == null || nextCursor.isEmpty) {
        break;
      }
      cursor = nextCursor;
    }
    return List<ConversationInboxEntry>.unmodifiable(items);
  }

  Future<void> markConversationRead(
    String conversationId, {
    int readSeq = 0,
  }) async {
    if (readSeq > 0) {
      await _client.chat.conversationsReadCursorUpdate(
        conversationId,
        UpdateReadCursorRequest(readSeq: readSeq),
      );
    }
    await _client.chat.conversationsPreferencesUpdate(
      conversationId,
      UpdateConversationPreferencesRequest(isMarkedUnread: false),
    );
  }
}

ChatInboxService createChatInboxService(ImSdkClientBundle bundle) {
  return ChatInboxService(bundle.imSdk);
}

ChatInboxResult readInboxPageFromSdkResponse(InboxListResponse? response) {
  return ChatInboxResult(
    items: readItemsFromSdkData(
      response?.data,
      ConversationInboxEntry.fromJson,
    ),
    pageInfo: readPageInfoFromSdkData(response?.data),
  );
}
