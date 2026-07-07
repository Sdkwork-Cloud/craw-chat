import type { SdkWorkListPageInfo, TimelineViewEntry } from "@sdkwork/im-sdk";

export const MAX_TIMELINE_ENTRIES = 500;

export interface TimelinePaginationState {
  hasMore: boolean;
  nextAfterSeq: number;
}

export function readCursorPageInfo(
  pageInfo?: SdkWorkListPageInfo,
): { hasMore: boolean; nextCursor?: string } {
  const hasMore = pageInfo?.hasMore === true;
  return {
    hasMore,
    nextCursor: hasMore ? (pageInfo?.nextCursor ?? undefined) : undefined,
  };
}

export function readSeqPageInfo(pageInfo?: SdkWorkListPageInfo): TimelinePaginationState {
  const hasMore = pageInfo?.hasMore === true;
  const parsed = hasMore && pageInfo?.nextCursor
    ? Number.parseInt(pageInfo.nextCursor, 10)
    : 0;
  return {
    hasMore,
    nextAfterSeq: Number.isFinite(parsed) && parsed > 0 ? parsed : 0,
  };
}

export function resolveLatestMessageSeq(entries: readonly TimelineViewEntry[]): number {
  return entries.reduce((max, entry) => Math.max(max, entry.messageSeq ?? 0), 0);
}

export function mergeTimelineEntries(
  existing: readonly TimelineViewEntry[],
  incoming: readonly TimelineViewEntry[],
): TimelineViewEntry[] {
  const byId = new Map<string, TimelineViewEntry>();
  for (const entry of existing) {
    byId.set(entry.messageId, entry);
  }
  for (const entry of incoming) {
    byId.set(entry.messageId, entry);
  }
  return Array.from(byId.values())
    .sort((left, right) => left.messageSeq - right.messageSeq)
    .slice(-MAX_TIMELINE_ENTRIES);
}

export function pickTimelinePagination(response: {
  pageInfo?: SdkWorkListPageInfo;
}): TimelinePaginationState {
  return readSeqPageInfo(response.pageInfo);
}
