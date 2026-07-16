import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';

import 'chat_sdk_response_utils.dart';

/// Maximum message history entries retained in memory per conversation (aligned with H5).
const int maxMessageHistoryEntries = 500;

class ChatMessageHistoryResult {
  const ChatMessageHistoryResult({
    required this.items,
    required this.pagination,
  });

  final List<ConversationMessageEntry> items;
  final MessageHistoryPaginationState pagination;
}

class MessageHistoryPaginationState {
  const MessageHistoryPaginationState({
    required this.hasMore,
    required this.nextCursor,
  });

  final bool hasMore;
  final String? nextCursor;
}

enum MessageHistoryWindowDirection { older, newer }

class MessageHistoryPageMergeResult {
  const MessageHistoryPageMergeResult({
    required this.items,
    required this.incomingPageRetained,
  });

  final List<ConversationMessageEntry> items;
  final bool incomingPageRetained;
}

int resolveLatestMessageSeq(List<ConversationMessageEntry> entries) {
  var maxSeq = 0;
  for (final entry in entries) {
    if (entry.messageSeq > maxSeq) {
      maxSeq = entry.messageSeq;
    }
  }
  return maxSeq;
}

MessageHistoryPageMergeResult mergeConversationMessagePage(
  Iterable<ConversationMessageEntry> existing,
  Iterable<ConversationMessageEntry> incoming, {
  required MessageHistoryWindowDirection direction,
}) {
  final existingEntries = existing.toList(growable: false);
  final incomingEntries = incoming.toList(growable: false);
  final existingIds = existingEntries.map((entry) => entry.messageId).toSet();
  final byId = <String, ConversationMessageEntry>{};
  for (final entry in existingEntries) {
    byId[entry.messageId] = entry;
  }
  for (final entry in incomingEntries) {
    byId[entry.messageId] = entry;
  }
  final merged = byId.values.toList()
    ..sort((left, right) {
      final sequenceComparison = left.messageSeq.compareTo(right.messageSeq);
      if (sequenceComparison != 0) {
        return sequenceComparison;
      }
      final occurredAtComparison = left.occurredAt.compareTo(right.occurredAt);
      if (occurredAtComparison != 0) {
        return occurredAtComparison;
      }
      return left.messageId.compareTo(right.messageId);
    });
  final items = direction == MessageHistoryWindowDirection.older
      ? merged.take(maxMessageHistoryEntries).toList(growable: false)
      : merged.length <= maxMessageHistoryEntries
          ? merged
          : merged.sublist(merged.length - maxMessageHistoryEntries);
  final retainedIds = items.map((entry) => entry.messageId).toSet();
  final incomingPageRetained = incomingEntries.every(
    (entry) =>
        existingIds.contains(entry.messageId) ||
        retainedIds.contains(entry.messageId),
  );
  return MessageHistoryPageMergeResult(
    items: items,
    incomingPageRetained: incomingPageRetained,
  );
}

List<ConversationMessageEntry> mergeConversationMessageEntries(
  Iterable<ConversationMessageEntry> existing,
  Iterable<ConversationMessageEntry> incoming, {
  MessageHistoryWindowDirection direction = MessageHistoryWindowDirection.newer,
}) {
  return mergeConversationMessagePage(
    existing,
    incoming,
    direction: direction,
  ).items;
}

MessageHistoryPaginationState readCursorPageInfo(PageInfo? pageInfo) {
  final nextCursor = pageInfo?.nextCursor;
  final hasMore =
      pageInfo?.hasMore == true && nextCursor != null && nextCursor.isNotEmpty;
  return MessageHistoryPaginationState(
    hasMore: hasMore,
    nextCursor: hasMore ? nextCursor : null,
  );
}

MessageHistoryPaginationState pickMessageHistoryPagination(
  ChatMessageHistoryResult? response,
) {
  return response?.pagination ??
      const MessageHistoryPaginationState(hasMore: false, nextCursor: null);
}

ChatMessageHistoryResult readMessageHistoryPageFromSdkResponse(
  ConversationMessageListResponse? response,
) {
  final pageInfo = readPageInfoFromSdkData(response?.data);
  return ChatMessageHistoryResult(
    items: readItemsFromSdkData(
      response?.data,
      ConversationMessageEntry.fromJson,
    ),
    pagination: readCursorPageInfo(pageInfo),
  );
}

PostMessageResult? readPostMessageResultFromSdkResponse(
  ConversationsMessagesCreateResponse201? response,
) {
  return readItemFromSdkData(response?.data, PostMessageResult.fromJson);
}
