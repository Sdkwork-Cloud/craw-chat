import type { ConversationInboxPage } from "@sdkwork/im-sdk";
import { getImSdkClient } from "@sdkwork/im-h5-core";

import { MAX_INBOX_ENTRIES } from "./chatInboxUtils";
import { readCursorPageInfo } from "./chatMessageHistoryUtils";

const INBOX_PAGE_SIZE = 20;

function normalizeInboxPageSize(pageSize: number | undefined): number {
  if (pageSize === undefined) {
    return INBOX_PAGE_SIZE;
  }
  const normalized = Math.floor(pageSize);
  if (!Number.isFinite(normalized) || normalized <= 0) {
    return INBOX_PAGE_SIZE;
  }
  return Math.min(normalized, MAX_INBOX_ENTRIES);
}

export async function fetchChatInbox(
  pageSize = INBOX_PAGE_SIZE,
  q?: string,
): Promise<ConversationInboxPage> {
  return fetchChatInboxPage({ pageSize, q });
}

export async function fetchChatInboxPage(options?: {
  pageSize?: number;
  cursor?: string;
  q?: string;
}): Promise<ConversationInboxPage> {
  const q = options?.q?.trim();
  const cursor = options?.cursor;
  return getImSdkClient().conversations.list({
    pageSize: normalizeInboxPageSize(options?.pageSize),
    ...(typeof cursor === "string" && cursor.length > 0 ? { cursor } : {}),
    ...(q ? { q } : {}),
  });
}

export function readInboxPageState(response: ConversationInboxPage) {
  return readCursorPageInfo(response.pageInfo);
}

export async function markConversationRead(
  conversationId: string,
  options?: { readSeq?: number },
): Promise<void> {
  const client = getImSdkClient();
  const readSeq = options?.readSeq ?? 0;
  if (readSeq > 0) {
    await client.conversations.updateReadCursor(conversationId, { readSeq });
  }
  await client.conversations.updatePreferences(conversationId, { isMarkedUnread: false });
}
