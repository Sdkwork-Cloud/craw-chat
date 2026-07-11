import type { ConversationInboxEntry, ConversationProfileView } from "@sdkwork/im-sdk";

const CONVERSATION_TITLE_STORAGE_PREFIX = "sdkwork-im-h5:conversation-title:";
const MAX_CONVERSATION_TITLE_LENGTH = 120;

function toRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === "object" && !Array.isArray(value)
    ? value as Record<string, unknown>
    : {};
}

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    if (typeof value !== "string") {
      continue;
    }
    const trimmed = value.trim();
    if (trimmed.length > 0) {
      return trimmed;
    }
  }
  return undefined;
}

function readSessionStorage(): Storage | undefined {
  if (typeof window === "undefined") {
    return undefined;
  }
  try {
    return window.sessionStorage;
  } catch {
    return undefined;
  }
}

function buildStorageKey(conversationId: string): string {
  return `${CONVERSATION_TITLE_STORAGE_PREFIX}${encodeURIComponent(conversationId)}`;
}

export function normalizeConversationDisplayTitle(
  conversationId: string,
  title: unknown,
): string | undefined {
  const normalizedConversationId = conversationId.trim();
  const normalizedTitle = pickString(title);
  if (!normalizedConversationId || !normalizedTitle || normalizedTitle === normalizedConversationId) {
    return undefined;
  }
  return normalizedTitle.length > MAX_CONVERSATION_TITLE_LENGTH
    ? normalizedTitle.slice(0, MAX_CONVERSATION_TITLE_LENGTH)
    : normalizedTitle;
}

export function resolveConversationInboxEntryDisplayTitle(
  entry: ConversationInboxEntry,
): string | undefined {
  const entryRecord = toRecord(entry);
  const peerRecord = toRecord(entryRecord.peer);
  return normalizeConversationDisplayTitle(
    entry.conversationId,
    pickString(
      entryRecord.displayName,
      entryRecord.display_name,
      peerRecord.displayName,
      peerRecord.display_name,
      peerRecord.name,
    ),
  );
}

export function resolveConversationProfileDisplayTitle(
  profile: ConversationProfileView,
): string | undefined {
  const profileRecord = toRecord(profile);
  return normalizeConversationDisplayTitle(
    profile.conversationId,
    pickString(
      profileRecord.displayName,
      profileRecord.display_name,
      profileRecord.name,
      profileRecord.title,
    ),
  );
}

export function rememberConversationTitle(conversationId: string, title: unknown): void {
  const normalizedConversationId = conversationId.trim();
  const displayTitle = normalizeConversationDisplayTitle(normalizedConversationId, title);
  if (!normalizedConversationId) {
    return;
  }
  const storage = readSessionStorage();
  if (!storage) {
    return;
  }
  try {
    if (!displayTitle) {
      storage.removeItem(buildStorageKey(normalizedConversationId));
      return;
    }
    storage.setItem(buildStorageKey(normalizedConversationId), displayTitle);
  } catch {
    // Title memory is a display enhancement; storage failures must not block chat navigation.
  }
}

export function readRememberedConversationTitle(conversationId: string): string | undefined {
  const normalizedConversationId = conversationId.trim();
  if (!normalizedConversationId) {
    return undefined;
  }
  const storage = readSessionStorage();
  if (!storage) {
    return undefined;
  }
  try {
    return normalizeConversationDisplayTitle(
      normalizedConversationId,
      storage.getItem(buildStorageKey(normalizedConversationId)),
    );
  } catch {
    return undefined;
  }
}
