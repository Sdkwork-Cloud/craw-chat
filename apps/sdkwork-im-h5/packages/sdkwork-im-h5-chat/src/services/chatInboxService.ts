import type { ConversationInboxPage } from "@sdkwork/im-sdk";
import { getImSdkClient } from "@sdkwork/im-h5-core";

import { readCursorPageInfo } from "./chatMessageHistoryUtils";

const INBOX_PAGE_SIZE = 20;

export async function fetchChatInbox(pageSize = INBOX_PAGE_SIZE): Promise<ConversationInboxPage> {
  return fetchChatInboxPage({ pageSize });
}

export async function fetchChatInboxPage(options?: {
  pageSize?: number;
  cursor?: string;
}): Promise<ConversationInboxPage> {
  return getImSdkClient().conversations.list({
    pageSize: options?.pageSize ?? INBOX_PAGE_SIZE,
    ...(options?.cursor ? { cursor: options.cursor } : {}),
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
