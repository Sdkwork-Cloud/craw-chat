import {
  getImSdkClient,
  type ImSdkClient,
} from "@sdkwork/im-h5-core/sdk";
import type {
  ConversationInboxPage,
  ConversationMessageListResponse,
  MessageHistoryListParams,
  PostMessageResult,
  QueryParams,
} from "@sdkwork/im-h5-core/sdk";
import { uuid } from "@sdkwork/utils";

export interface ListMessagesOptions {
  params?: MessageHistoryListParams;
}

export interface ChatConversationSdkPort {
  conversations: Pick<
    ImSdkClient["conversations"],
    "list" | "listMessages" | "postText" | "updatePreferences" | "updateReadCursor"
  >;
}

export function getChatImSdkClient(): ImSdkClient {
  return getImSdkClient();
}

export function createChatConversationService(
  resolveClient: () => ChatConversationSdkPort = getChatImSdkClient,
) {
  return {
    listInbox(params?: QueryParams): Promise<ConversationInboxPage> {
      return resolveClient().conversations.list(params);
    },

    listMessages(
      conversationId: string,
      options: ListMessagesOptions = {},
    ): Promise<ConversationMessageListResponse> {
      return resolveClient().conversations.listMessages(
        conversationId,
        options.params,
      );
    },

    postText(
      conversationId: string,
      text: string,
    ): Promise<PostMessageResult> {
      return resolveClient().conversations.postText(conversationId, text, {
        clientMsgId: uuid(),
      });
    },

    async markConversationRead(
      conversationId: string,
      readSeq: number,
    ): Promise<void> {
      if (!Number.isSafeInteger(readSeq) || readSeq < 0) {
        throw new RangeError("Conversation read sequence must be a non-negative safe integer.");
      }
      const client = resolveClient();
      await client.conversations.updateReadCursor(conversationId, { readSeq });
      await client.conversations.updatePreferences(conversationId, {
        isMarkedUnread: false,
      });
    },
  };
}

const chatConversationService = createChatConversationService();

export async function listInbox(
  params?: QueryParams,
): Promise<ConversationInboxPage> {
  return chatConversationService.listInbox(params);
}

export async function listMessages(
  conversationId: string,
  options: ListMessagesOptions = {},
): Promise<ConversationMessageListResponse> {
  return chatConversationService.listMessages(conversationId, options);
}

export async function postText(
  conversationId: string,
  text: string,
): Promise<PostMessageResult> {
  return chatConversationService.postText(conversationId, text);
}

export async function markConversationRead(
  conversationId: string,
  readSeq: number,
): Promise<void> {
  return chatConversationService.markConversationRead(conversationId, readSeq);
}
