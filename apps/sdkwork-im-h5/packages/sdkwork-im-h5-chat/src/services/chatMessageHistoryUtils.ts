import type { ConversationMessageEntry, SdkWorkListPageInfo } from "@sdkwork/im-sdk";

export const MAX_MESSAGE_HISTORY_ENTRIES = 500;

export interface MessageHistoryPaginationState {
  hasMore: boolean;
  nextCursor?: string;
}

export type MessageHistoryWindowDirection = "older" | "newer";

export interface MessageHistoryPageMergeResult {
  entries: ConversationMessageEntry[];
  incomingPageRetained: boolean;
}

export function readCursorPageInfo(
  pageInfo?: SdkWorkListPageInfo,
): { hasMore: boolean; nextCursor?: string } {
  const nextCursor = pageInfo?.nextCursor;
  const hasMore = pageInfo?.hasMore === true
    && typeof nextCursor === "string"
    && nextCursor.length > 0;
  return {
    hasMore,
    nextCursor: hasMore ? nextCursor : undefined,
  };
}

export function resolveLatestMessageSeq(entries: readonly ConversationMessageEntry[]): number {
  return entries.reduce((max, entry) => Math.max(max, entry.messageSeq ?? 0), 0);
}

export function mergeConversationMessagePage(
  existing: readonly ConversationMessageEntry[],
  incoming: readonly ConversationMessageEntry[],
  direction: MessageHistoryWindowDirection,
): MessageHistoryPageMergeResult {
  const existingIds = new Set(existing.map((entry) => entry.messageId));
  const byId = new Map<string, ConversationMessageEntry>();
  for (const entry of existing) {
    byId.set(entry.messageId, entry);
  }
  for (const entry of incoming) {
    byId.set(entry.messageId, entry);
  }
  const sorted = Array.from(byId.values())
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
    });
  const entries = direction === "older"
    ? sorted.slice(0, MAX_MESSAGE_HISTORY_ENTRIES)
    : sorted.slice(-MAX_MESSAGE_HISTORY_ENTRIES);
  const retainedIds = new Set(entries.map((entry) => entry.messageId));
  const incomingPageRetained = incoming.every(
    (entry) => existingIds.has(entry.messageId) || retainedIds.has(entry.messageId),
  );

  return { entries, incomingPageRetained };
}

export function mergeConversationMessageEntries(
  existing: readonly ConversationMessageEntry[],
  incoming: readonly ConversationMessageEntry[],
  direction: MessageHistoryWindowDirection = "newer",
): ConversationMessageEntry[] {
  return mergeConversationMessagePage(existing, incoming, direction).entries;
}

export function pickMessageHistoryPagination(response: {
  pageInfo?: SdkWorkListPageInfo;
}): MessageHistoryPaginationState {
  return readCursorPageInfo(response.pageInfo);
}
