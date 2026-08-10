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
  me: Pick<ImSdkClient["chat"]["me"], "welcome">;
}

export function getChatImSdkClient(): ImSdkClient {
  return getImSdkClient();
}

/**
 * Adapter exposing the narrow `ChatConversationSdkPort` surface. The IM SDK
 * exposes `welcome` under `client.chat.me`; the port flattens it so service
 * callers keep `client.me.welcome` without requiring the full client shape.
 */
export function getChatConversationSdkPort(): ChatConversationSdkPort {
  const client = getImSdkClient();
  return {
    conversations: client.conversations,
    me: client.chat.me,
  };
}

export function createChatConversationService(
  resolveClient: () => ChatConversationSdkPort = getChatConversationSdkPort,
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

    /**
     * 幂等触发系统智能体 Welcome 检查：后端在用户未收到过 Welcome 且
     * 没有过对话时发送系统消息，否则跳过。注册/登录后 fire-and-forget 调用。
     */
    async ensureWelcome(): Promise<void> {
      const client = resolveClient();
      await client.me.welcome.ensure();
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

/**
 * 幂等触发系统智能体 Welcome 检查（fire-and-forget 语义由调用方决定；
 * 服务端保证不重复发送）。注册/登录后调用一次即可。
 */
export async function ensureChatWelcomeMessage(): Promise<void> {
  return chatConversationService.ensureWelcome();
}
