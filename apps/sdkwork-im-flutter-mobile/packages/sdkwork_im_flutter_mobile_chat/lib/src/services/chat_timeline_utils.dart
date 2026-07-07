import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';

import 'chat_sdk_response_utils.dart';

/// Maximum timeline entries retained in memory per conversation (aligned with H5).
const int maxTimelineEntries = 500;

class ChatTimelineResult {
  const ChatTimelineResult({
    required this.items,
    required this.pagination,
  });

  final List<TimelineViewEntry> items;
  final TimelinePaginationState pagination;
}

class TimelinePaginationState {
  const TimelinePaginationState({
    required this.hasMore,
    required this.nextAfterSeq,
  });

  final bool hasMore;
  final int nextAfterSeq;
}

int resolveLatestMessageSeq(List<TimelineViewEntry> entries) {
  var maxSeq = 0;
  for (final entry in entries) {
    if (entry.messageSeq > maxSeq) {
      maxSeq = entry.messageSeq;
    }
  }
  return maxSeq;
}

List<TimelineViewEntry> mergeTimelineEntries(
  List<TimelineViewEntry> existing,
  List<TimelineViewEntry> incoming,
) {
  final byId = <String, TimelineViewEntry>{};
  for (final entry in existing) {
    byId[entry.messageId] = entry;
  }
  for (final entry in incoming) {
    byId[entry.messageId] = entry;
  }
  final merged = byId.values.toList()
    ..sort((left, right) => left.messageSeq.compareTo(right.messageSeq));
  if (merged.length <= maxTimelineEntries) {
    return merged;
  }
  return merged.sublist(merged.length - maxTimelineEntries);
}

TimelinePaginationState readSeqPageInfo(PageInfo? pageInfo) {
  final hasMore = pageInfo?.hasMore == true;
  final parsedCursor = hasMore ? int.tryParse(pageInfo?.nextCursor ?? '') : null;
  return TimelinePaginationState(
    hasMore: hasMore,
    nextAfterSeq: parsedCursor != null && parsedCursor > 0 ? parsedCursor : 0,
  );
}

TimelinePaginationState pickTimelinePagination(ChatTimelineResult? response) {
  return response?.pagination ??
      const TimelinePaginationState(hasMore: false, nextAfterSeq: 0);
}

ChatTimelineResult readTimelinePageFromSdkResponse(
  ConversationsMessagesListResponse? response,
) {
  final pageInfo = readPageInfoFromSdkData(response?.data);
  return ChatTimelineResult(
    items: readItemsFromSdkData(
      response?.data,
      TimelineViewEntry.fromJson,
    ),
    pagination: readSeqPageInfo(pageInfo),
  );
}

PostedMessageResponse? readPostedMessageFromSdkResponse(
  ConversationsMessagesCreateResponse201? response,
) {
  return readItemFromSdkData(response?.data, PostedMessageResponse.fromJson);
}
