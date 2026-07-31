import { MAX_LIST_PAGE_SIZE, uuid } from "@sdkwork/utils";
import type {
  ConversationInboxEntry,
  ConversationInboxPage,
  ConversationMessageEntry,
  ConversationMessageListResponse,
  CreateConversationRequest,
  CreateConversationResult,
  FavoriteMessageRequest,
  FavoriteMessagesResponse,
  MessageFavoriteView,
  PostMessageResult,
  UpdateConversationPreferencesRequest,
} from "@sdkwork/im-h5-core/sdk";
import type { Chat, Message, User } from "@sdkwork/im-h5-types";

import { getChatImSdkClient } from "./chatConversationService";

const LEGACY_CHAT_PAGE_SIZE = 50;

export interface ChatSdkPort {
  conversations: {
    addMember(
      conversationId: string,
      body: {
        principalId: string;
        principalKind: string;
        role: string;
        attributes?: Record<string, unknown>;
      },
    ): Promise<unknown>;
    create(body: CreateConversationRequest): Promise<CreateConversationResult>;
    list(params?: { cursor?: string; pageSize?: number; q?: string }): Promise<ConversationInboxPage>;
    listMessages(
      conversationId: string,
      params?: { cursor?: string; pageSize?: number },
    ): Promise<ConversationMessageListResponse>;
    postText(
      conversationId: string,
      text: string,
      body?: { clientMsgId?: string | null },
    ): Promise<PostMessageResult>;
    updatePreferences(
      conversationId: string,
      body: UpdateConversationPreferencesRequest,
    ): Promise<unknown>;
    updateReadCursor(conversationId: string, body: { readSeq: number }): Promise<unknown>;
  };
  messages: {
    deleteForMe(messageId: string): Promise<void>;
    favorites: {
      create(messageId: string, body: FavoriteMessageRequest): Promise<MessageFavoriteView>;
      delete(favoriteId: string): Promise<unknown>;
      list(params?: { pageSize?: number }): Promise<FavoriteMessagesResponse>;
    };
  };
}

export class ChatCapabilityUnavailableError extends Error {
  constructor(capability: string) {
    super(`${capability} is not exposed by the generated IM SDK.`);
    this.name = "ChatCapabilityUnavailableError";
  }
}

export function createChatService(
  resolveClient: () => ChatSdkPort = getChatImSdkClient,
) {
  const listInbox = async (q?: string): Promise<ConversationInboxPage> => {
    const page = await resolveClient().conversations.list({
      pageSize: Math.min(LEGACY_CHAT_PAGE_SIZE, MAX_LIST_PAGE_SIZE),
      ...(q ? { q } : {}),
    });
    assertCursorPage(page.pageInfo, "IM inbox");
    return page;
  };

  const findMessageById = async (
    conversationId: string,
    messageId: string,
  ): Promise<ConversationMessageEntry | undefined> => {
    const page = await resolveClient().conversations.listMessages(conversationId, {
      pageSize: Math.min(LEGACY_CHAT_PAGE_SIZE, MAX_LIST_PAGE_SIZE),
    });
    assertCursorPage(page.pageInfo, "IM message history");
    return page.items.find((message) => message.messageId === messageId);
  };

  return {
    async getChats(): Promise<Chat[]> {
      const page = await listInbox();
      return page.items.map(mapInboxEntry);
    },

    async getChatById(_conversationId: string): Promise<Chat | undefined> {
      throw new ChatCapabilityUnavailableError("Conversation retrieval by ID");
    },

    async getMessages(conversationId: string): Promise<Message[]> {
      const page = await resolveClient().conversations.listMessages(conversationId, {
        pageSize: Math.min(LEGACY_CHAT_PAGE_SIZE, MAX_LIST_PAGE_SIZE),
      });
      assertCursorPage(page.pageInfo, "IM message history");
      return page.items.map(mapMessageEntry);
    },

    async searchChatHistory(
      _conversationId: string,
      _query: string,
    ): Promise<Message[]> {
      throw new ChatCapabilityUnavailableError("Message history search");
    },

    async sendMessage(
      conversationId: string,
      _senderId: string,
      content: string,
      type: Message["type"] = "text",
      metadata?: unknown,
    ): Promise<Message> {
      if (type !== "text" || metadata !== undefined) {
        throw new ChatCapabilityUnavailableError(`Legacy ${type} message composition`);
      }
      const result = await resolveClient().conversations.postText(conversationId, content, {
        clientMsgId: uuid(),
      });
      const storedMessage = await findMessageById(conversationId, result.messageId);
      if (!storedMessage) {
        throw new Error("The accepted message was not returned by IM history.");
      }
      return mapMessageEntry(storedMessage);
    },

    async searchChats(query: string): Promise<Chat[]> {
      const normalizedQuery = query.trim();
      if (!normalizedQuery) {
        return [];
      }
      const page = await listInbox(normalizedQuery);
      return page.items.map(mapInboxEntry);
    },

    async createDirectChat(user: User, greeting?: string): Promise<Chat> {
      const result = await resolveClient().conversations.create({
        clientRequestKey: uuid(),
        conversationType: "direct",
        memberUserIds: [user.id],
      });
      if (greeting?.trim()) {
        await resolveClient().conversations.postText(result.conversationId, greeting.trim(), {
          clientMsgId: uuid(),
        });
      }
      return {
        id: result.conversationId,
        type: "direct",
        participants: [user],
        unreadCount: 0,
      };
    },

    async createGroupChat(name: string, participantIds: string[]): Promise<Chat> {
      const result = await resolveClient().conversations.create({
        clientRequestKey: uuid(),
        conversationType: "group",
        groupName: name.trim() || undefined,
        memberUserIds: Array.from(new Set(participantIds)),
      });
      return {
        id: result.conversationId,
        type: "group",
        participants: [],
        unreadCount: 0,
        ...(name.trim() ? { name: name.trim() } : {}),
      };
    },

    async addParticipants(
      conversationId: string,
      participantIds: string[],
    ): Promise<Chat | undefined> {
      for (const participantId of new Set(participantIds)) {
        await resolveClient().conversations.addMember(conversationId, {
          principalId: participantId,
          principalKind: "user",
          role: "member",
        });
      }
      return undefined;
    },

    async joinOrCreateGroupChat(_name: string): Promise<Chat> {
      throw new ChatCapabilityUnavailableError(
        "Entitlement-authorized group conversation join",
      );
    },

    async updateChatSettings(
      _conversationId: string,
      _settings: Partial<Chat["settings"]>,
    ): Promise<Chat | undefined> {
      throw new ChatCapabilityUnavailableError("Legacy chat display settings");
    },

    async pinChat(conversationId: string, isPinned: boolean): Promise<void> {
      await resolveClient().conversations.updatePreferences(conversationId, { isPinned });
    },

    async markAsUnread(conversationId: string): Promise<void> {
      await resolveClient().conversations.updatePreferences(conversationId, {
        isMarkedUnread: true,
      });
    },

    async markAsRead(conversationId: string): Promise<void> {
      const page = await resolveClient().conversations.listMessages(conversationId, {
        pageSize: 1,
      });
      assertCursorPage(page.pageInfo, "IM message history");
      await resolveClient().conversations.updateReadCursor(conversationId, {
        readSeq: page.highWatermark,
      });
      await resolveClient().conversations.updatePreferences(conversationId, {
        isMarkedUnread: false,
      });
    },

    async deleteChat(conversationId: string): Promise<void> {
      await resolveClient().conversations.updatePreferences(conversationId, {
        isHidden: true,
      });
    },

    async clearChatHistory(_conversationId: string): Promise<void> {
      throw new ChatCapabilityUnavailableError("Clear conversation history");
    },

    async deleteMessage(_conversationId: string, messageId: string): Promise<void> {
      await resolveClient().messages.deleteForMe(messageId);
    },

    async starMessage(
      conversationId: string,
      messageId: string,
      isStarred: boolean,
    ): Promise<void> {
      if (isStarred) {
        const message = await findMessageById(conversationId, messageId);
        if (!message) {
          throw new Error(`Message not found: ${messageId}`);
        }
        const preview = message.body.text ?? message.summary ?? "";
        await resolveClient().messages.favorites.create(messageId, {
          contentPreview: preview,
          conversationId,
          favoriteType: "chat",
          sourceDisplayName: message.sender.displayName ?? message.sender.id,
          title: preview.slice(0, 80) || message.messageType,
        });
        return;
      }

      throw new ChatCapabilityUnavailableError(
        `Favorite retrieval for message ${messageId} in conversation ${conversationId}`,
      );
    },

    async getEmojis(): Promise<string[]> {
      return ["😀", "😂", "😍", "🙏", "👍", "🎉", "❤️", "🔥"];
    },
  };
}

function assertCursorPage(
  pageInfo: { mode: string; hasMore?: boolean; nextCursor?: string | null },
  resource: string,
): void {
  if (pageInfo.mode !== "cursor") {
    throw new Error(`${resource} must use cursor pagination.`);
  }
  if (pageInfo.hasMore && !pageInfo.nextCursor) {
    throw new Error(`${resource} returned hasMore without nextCursor.`);
  }
}

function mapInboxEntry(entry: ConversationInboxEntry): Chat {
  if (entry.conversationType !== "direct" && entry.conversationType !== "group") {
    throw new Error(`Unsupported conversation type: ${entry.conversationType}`);
  }
  const peer = entry.peer;
  const participants: User[] = peer
    ? [{
      id: peer.userId ?? peer.principalId,
      name: peer.displayName ?? peer.principalId,
      ...(peer.avatarUrl ? { avatar: peer.avatarUrl } : {}),
    }]
    : [];
  const lastMessage = entry.lastMessageId
    ? {
      chatId: entry.conversationId,
      content: entry.lastSummary ?? "",
      id: entry.lastMessageId,
      senderId: entry.lastSenderId ?? "system",
      timestamp: parseTimestamp(entry.lastMessageAt ?? entry.lastActivityAt),
      type: "text" as const,
    }
    : undefined;
  return {
    id: entry.conversationId,
    type: entry.conversationType,
    participants,
    unreadCount: entry.unreadCount,
    ...(entry.displayName ? { name: entry.displayName } : {}),
    ...(entry.avatarUrl ? { avatar: entry.avatarUrl } : {}),
    ...(entry.preferences?.isPinned !== undefined
      ? { isPinned: entry.preferences.isPinned }
      : {}),
    ...(lastMessage ? { lastMessage } : {}),
  };
}

function mapMessageEntry(message: ConversationMessageEntry): Message {
  return {
    chatId: message.conversationId,
    content: message.body.text ?? message.summary ?? "",
    id: message.messageId,
    senderId: message.sender.id,
    timestamp: parseTimestamp(message.occurredAt),
    type: "text",
  };
}

function parseTimestamp(value: string): number {
  const timestamp = Date.parse(value);
  if (!Number.isFinite(timestamp)) {
    throw new Error(`Invalid IM timestamp: ${value}`);
  }
  return timestamp;
}

export const ChatService = createChatService();
