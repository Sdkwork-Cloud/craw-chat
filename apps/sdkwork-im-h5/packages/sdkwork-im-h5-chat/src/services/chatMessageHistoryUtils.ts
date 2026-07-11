import type { ConversationMessageEntry, SdkWorkListPageInfo } from "@sdkwork/im-sdk";

export const MAX_MESSAGE_HISTORY_ENTRIES = 500;

export interface MessageHistoryPaginationState {
  hasMore: boolean;
  nextCursor?: string;
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

export function readSeqPageInfo(pageInfo?: SdkWorkListPageInfo): MessageHistoryPaginationState {
  return readCursorPageInfo(pageInfo);
}

export function resolveLatestMessageSeq(entries: readonly ConversationMessageEntry[]): number {
  return entries.reduce((max, entry) => Math.max(max, entry.messageSeq ?? 0), 0);
}

export function mergeConversationMessageEntries(
  existing: readonly ConversationMessageEntry[],
  incoming: readonly ConversationMessageEntry[],
): ConversationMessageEntry[] {
  const byId = new Map<string, ConversationMessageEntry>();
  for (const entry of existing) {
    byId.set(entry.messageId, entry);
  }
  for (const entry of incoming) {
    byId.set(entry.messageId, entry);
  }
  return Array.from(byId.values())
    .sort((left, right) => {
      const sequenceDifference = (left.messageSeq ?? 0) - (right.messageSeq ?? 0);
      if (sequenceDifference !== 0) {
        return sequenceDifference;
      }
      const occurredAtDifference = Date.parse(left.occurredAt) - Date.parse(right.occurredAt);
      if (Number.isFinite(occurredAtDifference) && occurredAtDifference !== 0) {
        return occurredAtDifference;
      }
      return left.messageId.localeCompare(right.messageId);
    })
    .slice(-MAX_MESSAGE_HISTORY_ENTRIES);
}

export function pickMessageHistoryPagination(response: {
  pageInfo?: SdkWorkListPageInfo;
}): MessageHistoryPaginationState {
  return readSeqPageInfo(response.pageInfo);
}
