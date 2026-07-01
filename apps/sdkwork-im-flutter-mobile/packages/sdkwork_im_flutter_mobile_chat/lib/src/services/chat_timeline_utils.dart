import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';

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
  return merged;
}

TimelinePaginationState pickTimelinePagination(TimelineResponse? response) {
  return TimelinePaginationState(
    hasMore: response?.hasMore ?? false,
    nextAfterSeq: response?.nextAfterSeq ?? 0,
  );
}
