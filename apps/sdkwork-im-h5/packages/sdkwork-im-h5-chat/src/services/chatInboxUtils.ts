import type { ConversationInboxEntry } from "@sdkwork/im-sdk";

/** Maximum inbox rows retained in memory while paginating. */
export const MAX_INBOX_ENTRIES = 200;

export function mergeInboxEntries(
  existing: readonly ConversationInboxEntry[],
  incoming: readonly ConversationInboxEntry[],
): ConversationInboxEntry[] {
  const byId = new Map<string, ConversationInboxEntry>();
  for (const entry of existing) {
    byId.set(entry.conversationId, entry);
  }
  for (const entry of incoming) {
    byId.set(entry.conversationId, entry);
  }
  const merged = Array.from(byId.values());
  if (merged.length <= MAX_INBOX_ENTRIES) {
    return merged;
  }
  return merged.slice(0, MAX_INBOX_ENTRIES);
}

export function mergeLatestInboxEntries(
  existing: readonly ConversationInboxEntry[],
  latestPage: readonly ConversationInboxEntry[],
): ConversationInboxEntry[] {
  const latestIds = new Set(latestPage.map((entry) => entry.conversationId));
  return [
    ...latestPage,
    ...existing.filter((entry) => !latestIds.has(entry.conversationId)),
  ].slice(0, MAX_INBOX_ENTRIES);
}
