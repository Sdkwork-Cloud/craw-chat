import assert from "node:assert/strict";
import test from "node:test";

import {
  ChatCapabilityUnavailableError,
  createChatService,
  type ChatSdkPort,
} from "./ChatService";

interface ChatSdkOverrides {
  conversations?: Partial<ChatSdkPort["conversations"]>;
  messages?: {
    deleteForMe?: ChatSdkPort["messages"]["deleteForMe"];
    favorites?: Partial<ChatSdkPort["messages"]["favorites"]>;
  };
}

function createSdk(overrides: ChatSdkOverrides = {}): ChatSdkPort {
  return {
    conversations: {
      getPreferences: async () => ({
        tenantId: "tenant-1",
        conversationId: "conversation-1",
        principalKind: "user",
        principalId: "current-user",
        isPinned: false,
        isMuted: false,
        isMarkedUnread: false,
        isHidden: false,
        updatedAt: "2026-07-29T00:00:00Z",
      }),
      getProfile: async () => ({
        tenantId: "tenant-1",
        conversationId: "conversation-1",
        displayName: "Conversation",
        avatarUrl: "",
        notice: "",
        updatedAt: "2026-07-29T00:00:00Z",
      }),
      listMembers: async () => ({
        items: [],
        pageInfo: { mode: "cursor", hasMore: false },
      }),
      addMember: async () => ({}),
      create: async () => ({ conversationId: "conversation-created", eventId: "event-1" }),
      list: async () => ({
        items: [],
        pageInfo: { mode: "cursor", hasMore: false },
      }),
      listMessages: async () => ({
        items: [],
        pageInfo: { mode: "cursor", hasMore: false },
        highWatermark: 0,
      }),
      postText: async () => ({
        deliveryStatus: "applied",
        eventId: "event-message",
        messageId: "message-1",
        messageSeq: 1,
      }),
      updatePreferences: async () => ({}),
      updateReadCursor: async () => ({}),
      ...overrides.conversations,
    },
    messages: {
      deleteForMe: async () => undefined,
      favorites: {
        create: async () => ({
          tenantId: "tenant-1",
          principalKind: "user",
          principalId: "user-1",
          favoriteId: "favorite-1",
          favoriteType: "chat",
          conversationId: "conversation-1",
          messageId: "message-1",
          messageSeq: 1,
          title: "Hello",
          contentPreview: "Hello",
          sourceDisplayName: "User",
          favoritedAt: "2026-07-29T00:00:00Z",
        }),
        delete: async () => ({}),
        list: async () => ({
          items: [],
          pageInfo: { mode: "cursor", hasMore: false },
        }),
        ...overrides.messages?.favorites,
      },
    },
  };
}

test("lists one bounded server cursor page", async () => {
  let receivedParams: unknown;
  const service = createChatService(() => createSdk({
    conversations: {
      list: async (params) => {
        receivedParams = params;
        return {
          items: [{
            tenantId: "tenant-1",
            conversationId: "conversation-1",
            conversationType: "direct",
            displayName: "Support",
            lastActivityAt: "2026-07-29T00:00:00Z",
            messageCount: 0,
            lastMessageSeq: 0,
            unreadCount: 0,
          }],
          pageInfo: { mode: "cursor", hasMore: false },
        };
      },
    },
  }));

  const chats = await service.getChats();

  assert.deepEqual(receivedParams, { pageSize: 50 });
  assert.equal(chats[0]?.id, "conversation-1");
});

test("rejects incomplete cursor metadata", async () => {
  const service = createChatService(() => createSdk({
    conversations: {
      list: async () => ({
        items: [],
        pageInfo: { mode: "cursor", hasMore: true },
      }),
    },
  }));

  await assert.rejects(service.getChats(), /hasMore without nextCursor/);
});

test("returns the server message after posting text", async () => {
  let postOptions: unknown;
  const service = createChatService(() => createSdk({
    conversations: {
      postText: async (_conversationId, _text, options) => {
        postOptions = options;
        return {
          deliveryStatus: "applied",
          eventId: "event-message",
          messageId: "message-1",
          messageSeq: 1,
        };
      },
      listMessages: async () => ({
        items: [{
          tenantId: "tenant-1",
          conversationId: "conversation-1",
          messageId: "message-1",
          messageSeq: 1,
          sender: { id: "user-1", kind: "user" },
          body: { text: "Hello", parts: [] },
          messageType: "standard",
          deliveryMode: "persistent",
          occurredAt: "2026-07-29T00:00:00Z",
        }],
        pageInfo: { mode: "cursor", hasMore: false },
        highWatermark: 1,
      }),
    },
  }));

  const message = await service.sendMessage("conversation-1", "user-1", "Hello");

  assert.equal(message.id, "message-1");
  assert.equal(message.timestamp, Date.parse("2026-07-29T00:00:00Z"));
  assert.match(
    String((postOptions as { clientMsgId?: string }).clientMsgId),
    /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/iu,
  );
});

test("uses one UUID request key and de-duplicates group members", async () => {
  let request: unknown;
  const service = createChatService(() => createSdk({
    conversations: {
      create: async (body) => {
        request = body;
        return { conversationId: "group-1", eventId: "event-1" };
      },
    },
  }));

  await service.createGroupChat("Team", ["user-1", "user-1", "user-2"]);

  assert.match(
    String((request as { clientRequestKey?: string }).clientRequestKey),
    /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/iu,
  );
  assert.deepEqual((request as { memberUserIds?: string[] }).memberUserIds, ["user-1", "user-2"]);
});

test("fails closed when the generated SDK has no history-search operation", async () => {
  const service = createChatService(() => createSdk());
  await assert.rejects(
    service.searchChatHistory("conversation-1", "hello"),
    ChatCapabilityUnavailableError,
  );
});

test("retrieves conversation profile, members, and preferences through the SDK", async () => {
  const service = createChatService(() => createSdk());
  const chat = await service.getChatById("conversation-1");
  assert.equal(chat?.id, "conversation-1");
  assert.equal(chat?.type, "direct");
});

test("fails closed instead of creating an unauthorized group by name", async () => {
  let createCalls = 0;
  const service = createChatService(() => createSdk({
    conversations: {
      create: async () => {
        createCalls += 1;
        return { conversationId: "group-1", eventId: "event-1" };
      },
    },
  }));

  await assert.rejects(
    service.joinOrCreateGroupChat("Paid community"),
    ChatCapabilityUnavailableError,
  );
  assert.equal(createCalls, 0);
});

test("marks a conversation read at the server history high watermark", async () => {
  let readCursor: unknown;
  const service = createChatService(() => createSdk({
    conversations: {
      listMessages: async () => ({
        items: [],
        pageInfo: { mode: "cursor", hasMore: false },
        highWatermark: 42,
      }),
      updateReadCursor: async (_conversationId, body) => {
        readCursor = body;
        return {};
      },
    },
  }));

  await service.markAsRead("conversation-1");
  assert.deepEqual(readCursor, { readSeq: 42 });
});

test("scans cursor-paginated favorites before failing to unstar", async () => {
  let listCalls = 0;
  const service = createChatService(() => createSdk({
    messages: {
      favorites: {
        list: async () => {
          listCalls += 1;
          return {
            items: [],
            pageInfo: { mode: "cursor", hasMore: true, nextCursor: "next" },
          };
        },
      },
    },
  }));

  await assert.rejects(service.starMessage("conversation-1", "message-1", false), /repeated cursor/);
  assert.equal(listCalls, 2);
});
