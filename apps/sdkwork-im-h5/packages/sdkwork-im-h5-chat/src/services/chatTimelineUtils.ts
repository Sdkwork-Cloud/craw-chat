import type { TimelineViewEntry } from "@sdkwork/im-sdk";

export interface TimelinePaginationState {
  hasMore: boolean;
  nextAfterSeq: number;
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
  return Array.from(byId.values()).sort((left, right) => left.messageSeq - right.messageSeq);
}

export function pickTimelinePagination(response: {
  hasMore?: boolean;
  nextAfterSeq?: number | null;
}): TimelinePaginationState {
  return {
    hasMore: Boolean(response.hasMore),
    nextAfterSeq: typeof response.nextAfterSeq === "number" ? response.nextAfterSeq : 0,
  };
}
