import {
  getImSdkClient,
  type ImSdkClient,
} from "@sdkwork/im-h5-core";
import type {
  ConversationInboxPage,
  ConversationMessageListResponse,
  MessageHistoryListParams,
  PostMessageResult,
  QueryParams,
} from "@sdkwork/im-sdk";

export interface ListMessagesOptions {
  params?: MessageHistoryListParams;
}

export function getChatImSdkClient(): ImSdkClient {
  return getImSdkClient();
}

export async function listInbox(
  params?: QueryParams,
): Promise<ConversationInboxPage> {
  return getChatImSdkClient().conversations.list(params);
}

export async function listMessages(
  conversationId: string,
  options: ListMessagesOptions = {},
): Promise<ConversationMessageListResponse> {
  return getChatImSdkClient().conversations.listMessages(
    conversationId,
    options.params,
  );
}

export async function postText(
  conversationId: string,
  text: string,
): Promise<PostMessageResult> {
  return getChatImSdkClient().conversations.postText(conversationId, text);
}
