import { MAX_LIST_PAGE_SIZE, uuid } from "@sdkwork/utils";
import type {
  ConversationInboxEntry,
  ConversationInboxPage,
  ConversationMember,
  ConversationMessageEntry,
  ConversationMessageListResponse,
  ConversationPreferencesView,
  ConversationProfileView,
  CreateConversationRequest,
  CreateConversationResult,
  EditMessageRequest,
  FavoriteMessageRequest,
  FavoriteMessagesResponse,
  ListMembersResponse,
  MessageFavoriteView,
  MessageMutationResult,
  MessagePinMutationResult,
  MessageReplyReference,
  MessageSearchPage,
  MessageSearchParams,
  PinnedMessagesResponse,
  PostMessageResult,
  UpdateConversationPreferencesRequest,
  UpdateConversationProfileRequest,
} from "@sdkwork/im-h5-core/sdk";
import {
  getCmsAppSdkClient,
  type CmsAppSdkClient,
  type CmsFavoriteType,
} from "@sdkwork/im-h5-core/sdk";
import type { Chat, Message, User } from "@sdkwork/im-h5-types";
import i18next from "i18next";

import { getChatImSdkClient } from "./chatConversationService";
import type { ImSdkClient } from "@sdkwork/im-h5-core/sdk";
import { createChatMediaDownloadUrl, uploadChatMedia, type ChatMediaUpload } from "./chatMediaUploadService";

const LEGACY_CHAT_PAGE_SIZE = 50;
const MAX_SEARCH_MESSAGE_LOOKUP_PAGES = 10;
const MAX_MEMBER_LOOKUP_PAGES = 20;

/**
 * Wire-level view of a conversation member. The conversation runtime returns
 * `attributes` (displayName/avatarUrl enrichment) alongside the generated
 * member fields; the local view keeps the service layer type-safe without
 * changing generated SDK output.
 */
type EnrichedConversationMember = ListMembersResponse["items"][number] & {
  attributes?: Record<string, string>;
};

function memberDisplayName(member: EnrichedConversationMember): string | undefined {
  const attributes = member.attributes;
  const displayName = attributes?.["displayName"] ?? attributes?.["display_name"];
  return typeof displayName === "string" && displayName.trim() ? displayName.trim() : undefined;
}

/**
 * Derive the favorites page content shape from the message parts so the
 * favorites filter tabs (chat/image/file/voice/link) receive real data.
 */
function deriveMessageFavoriteType(message: ConversationMessageEntry): CmsFavoriteType {
  const mediaPart = message.body.parts.find((part) => part.kind === "media");
  if (mediaPart && mediaPart.kind === "media") {
    const mediaKind = mediaPart.resource.kind ?? mediaPart.resource.mediaKind;
    if (mediaKind === "image" || mediaKind === "video") {
      return "image";
    }
    if (mediaKind === "voice" || mediaKind === "audio") {
      return "voice";
    }
    if (mediaKind === "link") {
      return "link";
    }
    if (mediaKind === "file" || mediaKind === "document") {
      return "file";
    }
  }
  const text = message.body.text ?? message.body.summary ?? message.summary ?? "";
  if (/(https?:\/\/|www\.)/i.test(text)) {
    return "link";
  }
  return "chat";
}

export interface ChatPage {
  items: Chat[];
  hasMore: boolean;
  nextCursor?: string;
}

export interface MessagePage {
  items: Message[];
  hasMore: boolean;
  highWatermark: number;
  nextCursor?: string;
}

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
    getCurrentMember?(conversationId: string): Promise<ConversationMember>;
    getSummary?(conversationId: string): Promise<{ conversationId: string; messageCount: number; lastMessageSeq: number; lastSummary?: string | null; lastMessageAt?: string | null }>;
    leave?(conversationId: string): Promise<unknown>;
    list(params?: { cursor?: string; pageSize?: number; q?: string; conversationType?: string }): Promise<ConversationInboxPage>;
    listPinnedMessages?(conversationId: string): Promise<PinnedMessagesResponse>;
    listMessages(
      conversationId: string,
      params?: { cursor?: string; pageSize?: number },
    ): Promise<ConversationMessageListResponse>;
    getPreferences(conversationId: string): Promise<ConversationPreferencesView>;
    getProfile(conversationId: string): Promise<ConversationProfileView>;
    listMembers(conversationId: string, params?: { cursor?: string; pageSize?: number }): Promise<ListMembersResponse>;
    postText(
      conversationId: string,
      text: string,
      body?: { clientMsgId?: string | null; replyTo?: MessageReplyReference | null },
    ): Promise<PostMessageResult>;
    postMessage?: ImSdkClient["conversations"]["postMessage"];
    removeMember?(conversationId: string, body: { memberId: string }): Promise<unknown>;
    updatePreferences(
      conversationId: string,
      body: UpdateConversationPreferencesRequest,
    ): Promise<unknown>;
    updateProfile?(
      conversationId: string,
      body: UpdateConversationProfileRequest,
    ): Promise<ConversationProfileView>;
    updateReadCursor(conversationId: string, body: { readSeq: number }): Promise<unknown>;
  };
  messages: {
    deleteForMe(messageId: string): Promise<void>;
    edit?(messageId: string, body: EditMessageRequest): Promise<MessageMutationResult>;
    pin?(messageId: string): Promise<MessagePinMutationResult>;
    recall?(messageId: string): Promise<MessageMutationResult>;
    search(params: MessageSearchParams): Promise<MessageSearchPage>;
    unpin?(messageId: string): Promise<MessagePinMutationResult>;
    favorites: {
      create(messageId: string, body: FavoriteMessageRequest): Promise<MessageFavoriteView>;
      delete(favoriteId: string): Promise<unknown>;
      list(params?: { pageSize?: number; cursor?: string; favoriteType?: string }): Promise<FavoriteMessagesResponse>;
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
  resolveCmsClient: () => CmsAppSdkClient = getCmsAppSdkClient,
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

  const findMessageByIdAnywhere = async (
    conversationId: string,
    messageId: string,
  ): Promise<ConversationMessageEntry | undefined> => {
    let cursor: string | undefined;
    for (let depth = 0; depth < MAX_SEARCH_MESSAGE_LOOKUP_PAGES; depth += 1) {
      const page = await resolveClient().conversations.listMessages(conversationId, {
        pageSize: Math.min(LEGACY_CHAT_PAGE_SIZE, MAX_LIST_PAGE_SIZE),
        ...(cursor ? { cursor } : {}),
      });
      assertCursorPage(page.pageInfo, "IM message history");
      const found = page.items.find((message) => message.messageId === messageId);
      if (found) {
        return found;
      }
      if (!page.pageInfo.nextCursor) {
        return undefined;
      }
      cursor = page.pageInfo.nextCursor;
    }
    return undefined;
  };

  return {
    async listChatPage(cursor?: string, q?: string, conversationType?: string): Promise<ChatPage> {
      const page = await resolveClient().conversations.list({
        pageSize: Math.min(LEGACY_CHAT_PAGE_SIZE, MAX_LIST_PAGE_SIZE),
        ...(cursor ? { cursor } : {}),
        ...(q?.trim() ? { q: q.trim() } : {}),
        ...(conversationType ? { conversationType } : {}),
      });
      assertCursorPage(page.pageInfo, "IM inbox");
      return {
        items: page.items.map(mapInboxEntry),
        hasMore: page.pageInfo.hasMore === true,
        ...(page.pageInfo.nextCursor ? { nextCursor: page.pageInfo.nextCursor } : {}),
      };
    },

    async getChats(): Promise<Chat[]> {
      const page = await listInbox();
      return page.items.map(mapInboxEntry);
    },

    async getChatById(conversationId: string): Promise<Chat | undefined> {
      const [profile, members, preferences, inbox] = await Promise.all([
        resolveClient().conversations.getProfile(conversationId),
        fetchAllConversationMembers(resolveClient(), conversationId),
        resolveClient().conversations.getPreferences(conversationId),
        resolveClient().conversations.list({ pageSize: Math.min(LEGACY_CHAT_PAGE_SIZE, MAX_LIST_PAGE_SIZE) }),
      ]);
      assertCursorPage(inbox.pageInfo, "IM inbox");
      const inboxEntry = inbox.items.find((entry) => entry.conversationId === conversationId);
      const participants = members.items
        .filter((member) => member.state === "joined" || member.state === "linked")
        .map((member) => {
          const peer = inboxEntry?.peer;
          const isPeer = peer && (peer.userId ?? peer.principalId) === member.principalId;
          const enriched = member as EnrichedConversationMember;
          const memberName = memberDisplayName(enriched);
          return {
            id: member.principalId,
            name: isPeer
              ? (peer.displayName ?? memberName ?? peer.principalId)
              : (memberName ?? member.principalId),
            ...(isPeer && peer.avatarUrl ? { avatar: peer.avatarUrl } : {}),
            ...(!isPeer && enriched.attributes?.avatarUrl
              ? { avatar: enriched.attributes.avatarUrl }
              : {}),
          };
        });
      const client = resolveClient();
      const summary = client.conversations.getSummary
        ? await client.conversations.getSummary(conversationId)
        : undefined;
      return {
        id: conversationId,
        type: participants.length > 2 ? "group" : "direct",
        participants,
        unreadCount: 0,
        ...(summary?.lastSummary || summary?.lastMessageAt ? { lastMessage: { id: `${conversationId}:${summary.lastMessageSeq}`, chatId: conversationId, content: summary.lastSummary ?? "", senderId: "system", timestamp: parseTimestamp(summary.lastMessageAt ?? new Date().toISOString()), type: "text" as const } } : {}),
        name: profile.displayName || inboxEntry?.displayName || undefined,
        avatar: profile.avatarUrl || inboxEntry?.avatarUrl || undefined,
        ...(profile.notice ? { notice: profile.notice } : {}),
        isPinned: preferences.isPinned,
        settings: { showAvatar: true, cleanMode: false, isMuted: preferences.isMuted, isPinned: preferences.isPinned },
      };
    },

    async getMessagePage(conversationId: string, cursor?: string): Promise<MessagePage> {
      const page = await resolveClient().conversations.listMessages(conversationId, {
        pageSize: Math.min(LEGACY_CHAT_PAGE_SIZE, MAX_LIST_PAGE_SIZE),
        ...(cursor ? { cursor } : {}),
      });
      assertCursorPage(page.pageInfo, "IM message history");
      return {
        items: await Promise.all(page.items.map(mapMessageEntryWithDownloadUrl)),
        highWatermark: page.highWatermark,
        hasMore: page.pageInfo.hasMore === true,
        ...(page.pageInfo.nextCursor ? { nextCursor: page.pageInfo.nextCursor } : {}),
      };
    },

    async getMessages(conversationId: string): Promise<Message[]> {
      const page = await resolveClient().conversations.listMessages(conversationId, {
        pageSize: Math.min(LEGACY_CHAT_PAGE_SIZE, MAX_LIST_PAGE_SIZE),
      });
      assertCursorPage(page.pageInfo, "IM message history");
      return Promise.all(page.items.map(mapMessageEntryWithDownloadUrl));
    },

    async searchChatHistory(
      conversationId: string,
      query: string,
    ): Promise<Message[]> {
      const trimmed = query.trim();
      if (!trimmed) {
        return [];
      }
      const page = await resolveClient().messages.search({
        q: trimmed,
        conversationId,
        pageSize: Math.min(LEGACY_CHAT_PAGE_SIZE, MAX_LIST_PAGE_SIZE),
      });
      assertCursorPage(page.pageInfo, "IM message search");
      const messages = await Promise.all(
        page.items.map(async (hit) => {
          const entry = await findMessageByIdAnywhere(hit.conversationId, hit.messageId);
          return entry ? mapMessageEntryWithDownloadUrl(entry) : undefined;
        }),
      );
      return messages.filter((message): message is Message => message !== undefined);
    },

    async sendMessage(
      conversationId: string,
      _senderId: string,
      content: string,
      type: Message["type"] = "text",
      metadata?: unknown,
      replyTo?: MessageReplyReference,
    ): Promise<Message> {
      if (type !== "text" || metadata !== undefined) {
        throw new ChatCapabilityUnavailableError(`Legacy ${type} message composition`);
      }
      const result = await resolveClient().conversations.postText(conversationId, content, {
        clientMsgId: uuid(),
        ...(replyTo ? { replyTo } : {}),
      });
      const storedMessage = await findMessageById(conversationId, result.messageId);
      if (!storedMessage) {
        // History indexing is eventually consistent: retry across pages once
        // before treating an accepted send as failed.
        const retried = await findMessageByIdAnywhere(conversationId, result.messageId);
        if (!retried) {
          throw new Error("The accepted message was not returned by IM history.");
        }
        return mapMessageEntryWithDownloadUrl(retried);
      }
      return mapMessageEntryWithDownloadUrl(storedMessage);
    },

    async sendMediaMessage(
      conversationId: string,
      file: Parameters<typeof uploadChatMedia>[1],
      kind: ChatMediaUpload["resource"]["kind"],
      options: Parameters<typeof uploadChatMedia>[3] = {},
    ): Promise<Message> {
      const media = await uploadChatMedia(conversationId, file, kind, options);
      const client = resolveClient();
      if (!client.conversations.postMessage) {
        throw new ChatCapabilityUnavailableError("Media message composition");
      }
      const result = await client.conversations.postMessage(conversationId, {
        parts: [{ kind: "media", drive: media.drive, resource: media.resource, mediaRole: "attachment" }],
        clientMsgId: uuid(),
      });
      const storedMessage = await findMessageById(conversationId, result.messageId);
      if (!storedMessage) {
        // History indexing is eventually consistent: retry across pages once
        // before treating an accepted send as failed.
        const retried = await findMessageByIdAnywhere(conversationId, result.messageId);
        if (!retried) throw new Error("The accepted media message was not returned by IM history.");
        return mapMessageEntryWithDownloadUrl(retried);
      }
      return mapMessageEntryWithDownloadUrl(storedMessage);
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
      // Direct conversations accept a client-supplied id and attach members
      // through the member endpoint; memberUserIds is a group-only field.
      const conversationId = `direct-${uuid()}`;
      const result = await resolveClient().conversations.create({
        conversationId,
        conversationType: "direct",
      });
      await resolveClient().conversations.addMember(conversationId, {
        principalId: user.id,
        principalKind: "user",
        role: "member",
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
      conversationId: string,
      settings: Partial<NonNullable<Chat["settings"]>>,
    ): Promise<Chat | undefined> {
      const preferences: UpdateConversationPreferencesRequest = {};
      if (settings.isMuted !== undefined) preferences.isMuted = settings.isMuted;
      if (settings.isPinned !== undefined) preferences.isPinned = settings.isPinned;
      if (Object.keys(preferences).length === 0) return undefined;
      await resolveClient().conversations.updatePreferences(conversationId, preferences);
      return undefined;
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

    async recallMessage(_conversationId: string, messageId: string): Promise<void> {
      const client = resolveClient();
      if (!client.messages.recall) {
        throw new ChatCapabilityUnavailableError("Message recall");
      }
      await client.messages.recall(messageId);
    },

    async editMessage(
      conversationId: string,
      messageId: string,
      text: string,
    ): Promise<Message> {
      const client = resolveClient();
      if (!client.messages.edit) {
        throw new ChatCapabilityUnavailableError("Message editing");
      }
      const trimmed = text.trim();
      if (!trimmed) {
        throw new Error("Edited message content is required.");
      }
      await client.messages.edit(messageId, { text: trimmed });
      const storedMessage = await findMessageByIdAnywhere(conversationId, messageId);
      if (!storedMessage) {
        throw new Error("The edited message was not returned by IM history.");
      }
      return mapMessageEntryWithDownloadUrl(storedMessage);
    },

    async updateChatProfile(
      conversationId: string,
      body: UpdateConversationProfileRequest,
    ): Promise<void> {
      const client = resolveClient();
      if (!client.conversations.updateProfile) {
        throw new ChatCapabilityUnavailableError("Conversation profile update");
      }
      await client.conversations.updateProfile(conversationId, body);
    },

    async getMyConversationRole(conversationId: string): Promise<string | undefined> {
      const client = resolveClient();
      if (!client.conversations.getCurrentMember) {
        return undefined;
      }
      const member = await client.conversations.getCurrentMember(conversationId);
      return member.role;
    },

    async removeGroupMember(conversationId: string, userId: string): Promise<void> {
      const client = resolveClient();
      if (!client.conversations.removeMember) {
        throw new ChatCapabilityUnavailableError("Conversation member removal");
      }
      const page = await client.conversations.listMembers(conversationId, {
        pageSize: Math.min(LEGACY_CHAT_PAGE_SIZE, MAX_LIST_PAGE_SIZE),
      });
      assertCursorPage(page.pageInfo, "IM conversation members");
      const member = page.items.find(
        (item) => item.principalId === userId && item.principalKind === "user",
      );
      if (!member) {
        throw new Error(`Conversation member not found: ${userId}`);
      }
      await client.conversations.removeMember(conversationId, { memberId: member.memberId });
    },

    async leaveGroupChat(conversationId: string): Promise<void> {
      const client = resolveClient();
      if (!client.conversations.leave) {
        throw new ChatCapabilityUnavailableError("Conversation leave");
      }
      await client.conversations.leave(conversationId);
    },

    async listPinnedMessages(conversationId: string): Promise<string[]> {
      const client = resolveClient();
      if (!client.conversations.listPinnedMessages) {
        return [];
      }
      const response = await client.conversations.listPinnedMessages(conversationId);
      return response.items
        .filter((item) => item.pin !== null && item.pin !== undefined)
        .map((item) => item.messageId);
    },

    async pinMessage(_conversationId: string, messageId: string): Promise<void> {
      const client = resolveClient();
      if (!client.messages.pin) {
        throw new ChatCapabilityUnavailableError("Message pinning");
      }
      await client.messages.pin(messageId);
    },

    async unpinMessage(_conversationId: string, messageId: string): Promise<void> {
      const client = resolveClient();
      if (!client.messages.unpin) {
        throw new ChatCapabilityUnavailableError("Message unpinning");
      }
      await client.messages.unpin(messageId);
    },

    async starMessage(
      conversationId: string,
      messageId: string,
      isStarred: boolean,
    ): Promise<void> {
      if (isStarred) {
        const message = await findMessageByIdAnywhere(conversationId, messageId);
        if (!message) {
          throw new Error(`Message not found: ${messageId}`);
        }
        const preview = message.body.text ?? message.summary ?? "";
        await resolveCmsClient().favorites.create({
          targetType: "im_message",
          targetId: messageId,
          favoriteType: deriveMessageFavoriteType(message),
          title: preview.slice(0, 80) || message.messageType,
          summary: preview,
          sourceDisplayName: message.sender.displayName ?? message.sender.id,
        });
        return;
      }

      let cursor: string | undefined;
      const visitedCursors = new Set<string>();
      do {
        const page = await resolveCmsClient().favorites.list({
          pageSize: Math.min(LEGACY_CHAT_PAGE_SIZE, MAX_LIST_PAGE_SIZE),
          ...(cursor ? { cursor } : {}),
        });
        const favorite = page.items.find(
          (item) => item.targetType === "im_message" && item.targetId === messageId,
        );
        if (favorite) {
          await resolveCmsClient().favorites.delete(favorite.favoriteId);
          return;
        }
        cursor = page.pageInfo.hasMore ? page.pageInfo.nextCursor ?? undefined : undefined;
        if (cursor) {
          if (visitedCursors.has(cursor)) {
            throw new Error("CMS message favorites returned a repeated cursor.");
          }
          visitedCursors.add(cursor);
        }
      } while (cursor);
      throw new Error(`Favorite not found for message ${messageId}.`);
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

async function fetchAllConversationMembers(
  client: ChatSdkPort,
  conversationId: string,
): Promise<ListMembersResponse["items"]> {
  const members: ListMembersResponse["items"] = [];
  let cursor: string | undefined;
  for (let depth = 0; depth < MAX_MEMBER_LOOKUP_PAGES; depth += 1) {
    const page = await client.conversations.listMembers(conversationId, {
      pageSize: Math.min(LEGACY_CHAT_PAGE_SIZE, MAX_LIST_PAGE_SIZE),
      ...(cursor ? { cursor } : {}),
    });
    assertCursorPage(page.pageInfo, "IM conversation members");
    members.push(...page.items);
    if (!page.pageInfo.hasMore || !page.pageInfo.nextCursor) {
      break;
    }
    cursor = page.pageInfo.nextCursor;
  }
  return members;
}

function mapInboxEntry(entry: ConversationInboxEntry): Chat {
  if (entry.conversationType !== "direct" && entry.conversationType !== "group") {
    throw new Error(`Unsupported conversation type: ${entry.conversationType}`);
  }
  const peer = entry.peer;
  const participants: User[] = peer
    ? [{
      id: peer.userId ?? peer.principalId,
      name:
        peer.principalKind === "system"
          ? i18next.t("chat.date.system_agent_name", "系统智能体")
          : peer.displayName ?? peer.principalId,
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
    ...(entry.preferences
      ? {
        settings: {
          showAvatar: true,
          cleanMode: false,
          isMuted: entry.preferences.isMuted ?? false,
          isPinned: entry.preferences.isPinned ?? false,
        },
      }
      : {}),
    ...(lastMessage ? { lastMessage } : {}),
  };
}

function mapMessageEntry(message: ConversationMessageEntry): Message {
  const mediaPart = message.body.parts.find((part) => part.kind === "media");
  const media = mediaPart?.kind === "media" ? mediaPart : undefined;
  const mediaKind = media?.resource.kind;
  const messageType: Message["type"] = mediaKind === "image"
    ? "image"
    : mediaKind === "video"
      ? "video"
      : mediaKind === "voice" || mediaKind === "audio"
        ? "voice"
        : mediaKind === "file" || mediaKind === "document"
          ? "file"
          : "text";
  const mediaUrl = media?.resource.publicUrl ?? media?.resource.url ?? media?.resource.uri;
  const replyTo = message.body.replyTo ? {
    id: message.body.replyTo.messageId,
    senderName: message.body.replyTo.senderDisplayName,
    content: message.body.replyTo.contentPreview,
  } : undefined;
  return {
    chatId: message.conversationId,
    content: mediaUrl ?? message.body.text ?? message.summary ?? "",
    id: message.messageId,
    senderId: message.sender.id,
    timestamp: parseTimestamp(message.occurredAt),
    type: messageType,
    ...(replyTo ? { replyTo, metadata: { replyTo: replyTo.id } } : {}),
    ...(media ? { metadata: { ...(replyTo ? { replyTo: replyTo.id } : {}), fileName: media.resource.fileName ?? undefined, mimeType: media.resource.mimeType ?? undefined, size: media.resource.sizeBytes ?? undefined, duration: media.resource.durationSeconds ?? undefined, driveUri: media.drive.driveUri, nodeId: media.drive.nodeId } } : {}),
  };
}

async function mapMessageEntryWithDownloadUrl(message: ConversationMessageEntry): Promise<Message> {
  const mapped = mapMessageEntry(message);
  const nodeId = typeof mapped.metadata?.nodeId === "string" ? mapped.metadata.nodeId : undefined;
  if (!nodeId || mapped.type === "text") return mapped;
  try {
    return { ...mapped, content: await createChatMediaDownloadUrl(nodeId) };
  } catch (error) {
    console.error("Unable to resolve Drive-backed chat media", error);
    return mapped;
  }
}

function parseTimestamp(value: string): number {
  const timestamp = Date.parse(value);
  if (!Number.isFinite(timestamp)) {
    throw new Error(`Invalid IM timestamp: ${value}`);
  }
  return timestamp;
}

export const ChatService = createChatService();
