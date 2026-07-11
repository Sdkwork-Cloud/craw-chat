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
    required this.nextAfterSeq,
  });

  final bool hasMore;
  final int nextAfterSeq;
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

List<ConversationMessageEntry> mergeConversationMessageEntries(
  List<ConversationMessageEntry> existing,
  List<ConversationMessageEntry> incoming,
) {
  final byId = <String, ConversationMessageEntry>{};
  for (final entry in existing) {
    byId[entry.messageId] = entry;
  }
  for (final entry in incoming) {
    byId[entry.messageId] = entry;
  }
  final merged = byId.values.toList()
    ..sort((left, right) => left.messageSeq.compareTo(right.messageSeq));
  if (merged.length <= maxMessageHistoryEntries) {
    return merged;
  }
  return merged.sublist(merged.length - maxMessageHistoryEntries);
}

MessageHistoryPaginationState readSeqPageInfo(PageInfo? pageInfo) {
  final hasMore = pageInfo?.hasMore == true;
  final parsedCursor = hasMore ? int.tryParse(pageInfo?.nextCursor ?? '') : null;
  return MessageHistoryPaginationState(
    hasMore: hasMore,
    nextAfterSeq: parsedCursor != null && parsedCursor > 0 ? parsedCursor : 0,
  );
}

MessageHistoryPaginationState pickMessageHistoryPagination(
  ChatMessageHistoryResult? response,
) {
  return response?.pagination ??
      const MessageHistoryPaginationState(hasMore: false, nextAfterSeq: 0);
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
    pagination: readSeqPageInfo(pageInfo),
  );
}

PostMessageResult? readPostMessageResultFromSdkResponse(
  ConversationsMessagesCreateResponse201? response,
) {
  return readItemFromSdkData(response?.data, PostMessageResult.fromJson);
}
