import type { ConversationInboxEntry } from "@sdkwork/im-sdk";

/** Maximum inbox rows retained in memory while paginating. */
export const MAX_INBOX_ENTRIES = 200;

export type InboxWindowDirection = "older" | "newer";

export interface InboxPageMergeResult {
  entries: ConversationInboxEntry[];
  incomingPageRetained: boolean;
}

export function mergeInboxPage(
  existing: readonly ConversationInboxEntry[],
  incoming: readonly ConversationInboxEntry[],
  direction: InboxWindowDirection,
): InboxPageMergeResult {
  const existingIds = new Set(existing.map((entry) => entry.conversationId));
  const incomingById = new Map(
    incoming.map((entry) => [entry.conversationId, entry] as const),
  );
  const incomingEntries = Array.from(incomingById.values());
  const merged = direction === "older"
    ? [
      ...existing.map((entry) => incomingById.get(entry.conversationId) ?? entry),
      ...incomingEntries.filter((entry) => !existingIds.has(entry.conversationId)),
    ]
    : [
      ...incomingEntries,
      ...existing.filter((entry) => !incomingById.has(entry.conversationId)),
    ];
  const entries = direction === "older"
    ? merged.slice(-MAX_INBOX_ENTRIES)
    : merged.slice(0, MAX_INBOX_ENTRIES);
  const retainedIds = new Set(entries.map((entry) => entry.conversationId));
  const incomingPageRetained = incomingEntries.every(
    (entry) => existingIds.has(entry.conversationId) || retainedIds.has(entry.conversationId),
  );

  return { entries, incomingPageRetained };
}

export function mergeInboxEntries(
  existing: readonly ConversationInboxEntry[],
  incoming: readonly ConversationInboxEntry[],
): ConversationInboxEntry[] {
  return mergeInboxPage(existing, incoming, "older").entries;
}

export function mergeLatestInboxEntries(
  existing: readonly ConversationInboxEntry[],
  latestPage: readonly ConversationInboxEntry[],
): ConversationInboxEntry[] {
  return mergeInboxPage(existing, latestPage, "newer").entries;
}
