import type { InboxResponse } from "@sdkwork/im-sdk";
import { getImSdkClient } from "@sdkwork/im-h5-core";

export async function fetchChatInbox(limit = 30): Promise<InboxResponse> {
  return getImSdkClient().conversations.list({ limit });
}
