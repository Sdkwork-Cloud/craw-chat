import type { InboxResponse } from "@sdkwork/im-sdk";
import { getImSdkClient } from "@sdkwork/im-h5-core";

const INBOX_PAGE_SIZE = 20;

export async function fetchChatInbox(pageSize = INBOX_PAGE_SIZE): Promise<InboxResponse> {
  return fetchChatInboxPage({ pageSize });
}

export async function fetchChatInboxPage(options?: {
  pageSize?: number;
  cursor?: string;
}): Promise<InboxResponse> {
  return getImSdkClient().conversations.list({
    pageSize: options?.pageSize ?? INBOX_PAGE_SIZE,
    ...(options?.cursor ? { cursor: options.cursor } : {}),
  });
}
