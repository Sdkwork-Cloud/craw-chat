import { MAX_LIST_PAGE_SIZE, uuid } from "@sdkwork/utils";
import type {
  ConversationInboxEntry,
  ConversationInboxPage,
  ConversationMessageEntry,
  ConversationMessageListResponse,
  ConversationPreferencesView,
  ConversationProfileView,
  CreateConversationRequest,
  CreateConversationResult,
  FavoriteMessageRequest,
  FavoriteMessagesResponse,
  ListMembersResponse,
  MessageFavoriteView,
  MessageReplyReference,
  MessageSearchPage,
  MessageSearchParams,
  PostMessageResult,
  UpdateConversationPreferencesRequest,
} from "@sdkwork/im-h5-core/sdk";
import type { Chat, Message, User } from "@sdkwork/im-h5-types";

import { getChatImSdkClient } from "./chatConversationService";
import type { ImSdkClient } from "@sdkwork/im-h5-core/sdk";
import { createChatMediaDownloadUrl, uploadChatMedia, type ChatMediaUpload } from "./chatMediaUploadService";

const LEGACY_CHAT_PAGE_SIZE = 50;
const MAX_SEARCH_MESSAGE_LOOKUP_PAGES = 10;

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
    getSummary?(conversationId: string): Promise<{ conversationId: string; messageCount: number; lastMessageSeq: number; lastSummary?: string | null; lastMessageAt?: string | null }>;
    list(params?: { cursor?: string; pageSize?: number; q?: string }): Promise<ConversationInboxPage>;
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
    updatePreferences(
      conversationId: string,
      body: UpdateConversationPreferencesRequest,
    ): Promise<unknown>;
    updateReadCursor(conversationId: string, body: { readSeq: number }): Promise<unknown>;
  };
  messages: {
    deleteForMe(messageId: string): Promise<void>;
    search(params: MessageSearchParams): Promise<MessageSearchPage>;
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
    async listChatPage(cursor?: string, q?: string): Promise<ChatPage> {
      const page = await resolveClient().conversations.list({
        pageSize: Math.min(LEGACY_CHAT_PAGE_SIZE, MAX_LIST_PAGE_SIZE),
        ...(cursor ? { cursor } : {}),
        ...(q?.trim() ? { q: q.trim() } : {}),
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
        resolveClient().conversations.listMembers(conversationId, {
          pageSize: Math.min(LEGACY_CHAT_PAGE_SIZE, MAX_LIST_PAGE_SIZE),
        }),
        resolveClient().conversations.getPreferences(conversationId),
        resolveClient().conversations.list({ pageSize: Math.min(LEGACY_CHAT_PAGE_SIZE, MAX_LIST_PAGE_SIZE) }),
      ]);
      assertCursorPage(members.pageInfo, "IM conversation members");
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
        throw new Error("The accepted message was not returned by IM history.");
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
      if (!storedMessage) throw new Error("The accepted media message was not returned by IM history.");
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

      let cursor: string | undefined;
      const visitedCursors = new Set<string>();
      do {
        const page = await resolveClient().messages.favorites.list({
          pageSize: Math.min(LEGACY_CHAT_PAGE_SIZE, MAX_LIST_PAGE_SIZE),
          ...(cursor ? { cursor } : {}),
          favoriteType: "chat",
        });
        assertCursorPage(page.pageInfo, "IM message favorites");
        const favorite = page.items.find((item) => item.messageId === messageId && item.conversationId === conversationId);
        if (favorite) {
          await resolveClient().messages.favorites.delete(favorite.favoriteId);
          return;
        }
        cursor = page.pageInfo.hasMore ? page.pageInfo.nextCursor ?? undefined : undefined;
        if (cursor) {
          if (visitedCursors.has(cursor)) {
            throw new Error("IM message favorites returned a repeated cursor.");
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
