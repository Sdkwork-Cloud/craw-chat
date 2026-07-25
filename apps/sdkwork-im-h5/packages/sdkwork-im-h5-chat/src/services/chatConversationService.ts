import { getImSdkClient } from '@sdkwork/im-h5-core';
import type { ImSdkClient } from '@sdkwork/im-sdk';

export interface ConversationMessageListResponse {
  messages: ConversationMessage[];
  nextCursor?: string;
  hasMore: boolean;
}

export interface ConversationMessage {
  id: string;
  conversationId: string;
  senderId: string;
  text?: string;
  type: string;
  createdAt: string;
}

export interface PostMessageResult {
  messageId: string;
  conversationId: string;
  createdAt: string;
}

export interface ListMessagesParams {
  cursor?: string;
  limit?: number;
  direction?: 'before' | 'after';
}

async function resolveImSdkClient(): Promise<ImSdkClient> {
  return getImSdkClient();
}

export async function listMessages(
  conversationId: string,
  params?: ListMessagesParams,
): Promise<ConversationMessageListResponse> {
  const client = await resolveImSdkClient();
  const response = await client.conversations.listMessages(conversationId, params);
  return response as unknown as ConversationMessageListResponse;
}

export async function postText(
  conversationId: string,
  text: string,
  body?: Record<string, unknown>,
): Promise<PostMessageResult> {
  const client = await resolveImSdkClient();
  const result = await client.conversations.postText(conversationId, text, body as Parameters<typeof client.conversations.postText>[2]);
  return result as unknown as PostMessageResult;
}

export async function fetchConversationMessages(
  conversationId: string,
  params?: ListMessagesParams,
): Promise<ConversationMessageListResponse> {
  return listMessages(conversationId, params);
}

export async function sendConversationText(
  conversationId: string,
  text: string,
): Promise<PostMessageResult> {
  return postText(conversationId, text);
}
