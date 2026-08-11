import assert from "node:assert/strict";
import test from "node:test";

import type {
  CmsAppSdkClient,
  ConversationMessageEntry,
} from "@sdkwork/im-h5-core/sdk";

import {
  ChatCapabilityUnavailableError,
  createChatService,
  type ChatSdkPort,
} from "./ChatService";

interface ChatSdkOverrides {
  conversations?: Partial<ChatSdkPort["conversations"]>;
  messages?: {
    deleteForMe?: ChatSdkPort["messages"]["deleteForMe"];
    search?: ChatSdkPort["messages"]["search"];
    recall?: ChatSdkPort["messages"]["recall"];
    edit?: ChatSdkPort["messages"]["edit"];
    favorites?: Partial<ChatSdkPort["messages"]["favorites"]>;
  };
}

interface CmsSdkOverrides {
  favorites?: {
    create?: CmsAppSdkClient["favorites"]["create"];
    list?: CmsAppSdkClient["favorites"]["list"];
    delete?: CmsAppSdkClient["favorites"]["delete"];
  };
}

function createCmsSdk(overrides: CmsSdkOverrides = {}): CmsAppSdkClient {
  return {
    favorites: {
      create: async () => ({ item: {
        id: "1",
        favoriteId: "fav-1",
        favoriteType: "chat",
        targetType: "im_message",
        targetId: "message-1",
        targetUuid: null,
        targetUrl: null,
        title: "Hello",
        summary: "Hello",
        sourceDisplayName: "User",
        media: null,
        favoritedAt: "2026-07-29T00:00:00Z",
      } }),
      list: async () => ({
        items: [],
        pageInfo: { mode: "cursor", nextCursor: null, hasMore: false },
      }),
      delete: async () => ({ deleted: true }),
      ...overrides.favorites,
    },
  } as unknown as CmsAppSdkClient;
}

function messageEntry(overrides: Partial<ConversationMessageEntry> = {}): ConversationMessageEntry {
  return {
    tenantId: "tenant-1",
    conversationId: "conversation-1",
    messageId: "message-1",
    messageSeq: 1,
    summary: "Hello",
    sender: { id: "user-1", kind: "user", displayName: "User" },
    body: { text: "Hello", parts: [] },
    messageType: "standard",
    deliveryMode: "chat",
    occurredAt: "2026-07-29T00:00:00Z",
    ...overrides,
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
      search: async () => ({
        items: [],
        pageInfo: { mode: "cursor", hasMore: false },
      }),
      ...(overrides.messages?.deleteForMe ? { deleteForMe: overrides.messages.deleteForMe } : {}),
      ...(overrides.messages?.search ? { search: overrides.messages.search } : {}),
      ...(overrides.messages?.recall ? { recall: overrides.messages.recall } : {}),
      ...(overrides.messages?.edit ? { edit: overrides.messages.edit } : {}),
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

test("forwards conversation type filter to the server page", async () => {
  let receivedParams: unknown;
  const service = createChatService(() => createSdk({
    conversations: {
      list: async (params) => {
        receivedParams = params;
        return {
          items: [{
            tenantId: "tenant-1",
            conversationId: "group-1",
            conversationType: "group",
            displayName: "Team",
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

  const chats = await service.listChatPage(undefined, undefined, "group");

  assert.deepEqual(receivedParams, { pageSize: 50, conversationType: "group" });
  assert.equal(chats.items[0]?.type, "group");
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

test("creates a direct chat with a client id and attaches the member", async () => {
  let createRequest: unknown;
  let memberBody: unknown;
  let memberConversationId: unknown;
  const service = createChatService(() => createSdk({
    conversations: {
      create: async (body) => {
        createRequest = body;
        return { conversationId: (body as { conversationId?: string }).conversationId ?? "direct-1", eventId: "event-1" };
      },
      addMember: async (conversationId, body) => {
        memberConversationId = conversationId;
        memberBody = body;
      },
    },
  }));

  const chat = await service.createDirectChat({ id: "user-1", name: "Alice" } as never);

  assert.equal(chat.type, "direct");
  assert.match(chat.id, /^direct-/u);
  assert.equal(
    (createRequest as { conversationId?: string }).conversationId,
    chat.id,
  );
  assert.equal((createRequest as { memberUserIds?: string[] }).memberUserIds, undefined);
  assert.equal((createRequest as { clientRequestKey?: string }).clientRequestKey, undefined);
  assert.equal(memberConversationId, chat.id);
  assert.deepEqual(memberBody, {
    principalId: "user-1",
    principalKind: "user",
    role: "member",
  });
});

test("searches history through the SDK and maps hits to messages", async () => {
  let receivedParams: unknown;
  const service = createChatService(() => createSdk({
    messages: {
      search: async (params) => {
        receivedParams = params;
        return {
          items: [{ conversationId: "conversation-1", messageId: "message-42", messageSeq: 5 }],
          pageInfo: { mode: "cursor", hasMore: false },
        };
      },
    },
    conversations: {
      listMessages: async () => ({
        items: [{
          tenantId: "tenant-1",
          conversationId: "conversation-1",
          messageId: "message-42",
          messageSeq: 5,
          sender: { id: "user-1", kind: "user" },
          body: {
            parts: [{ kind: "text", text: "Hello world" }],
          },
          summary: "Hello world",
          messageType: "standard",
          deliveryMode: "at_least_once",
          occurredAt: "2026-07-29T00:00:00Z",
        }],
        pageInfo: { mode: "cursor", hasMore: false },
        highWatermark: 5,
      }),
    },
  }));
  const results = await service.searchChatHistory("conversation-1", "hello");
  assert.equal(results.length, 1);
  assert.equal(results[0].id, "message-42");
  assert.equal(results[0].chatId, "conversation-1");
  assert.equal(results[0].content, "Hello world");
  assert.equal((receivedParams as { q?: string }).q, "hello");
  assert.equal((receivedParams as { conversationId?: string }).conversationId, "conversation-1");
});

test("searches history with a trimmed query and skips hits without history entries", async () => {
  const service = createChatService(() => createSdk({
    messages: {
      search: async () => ({
        items: [
          { conversationId: "conversation-1", messageId: "message-42", messageSeq: 5 },
          { conversationId: "conversation-1", messageId: "message-99", messageSeq: 9 },
        ],
        pageInfo: { mode: "cursor", hasMore: false },
      }),
    },
    conversations: {
      listMessages: async () => ({
        items: [{
          tenantId: "tenant-1",
          conversationId: "conversation-1",
          messageId: "message-42",
          messageSeq: 5,
          sender: { id: "user-1", kind: "user" },
          body: {
            parts: [{ kind: "text", text: "Hello world" }],
          },
          summary: "Hello world",
          messageType: "standard",
          deliveryMode: "at_least_once",
          occurredAt: "2026-07-29T00:00:00Z",
        }],
        pageInfo: { mode: "cursor", hasMore: false },
        highWatermark: 9,
      }),
    },
  }));
  const results = await service.searchChatHistory("conversation-1", "  hello  ");
  assert.equal(results.length, 1);
  assert.equal(results[0].id, "message-42");
});

test("returns no results for an empty search query", async () => {
  let searchCalls = 0;
  const service = createChatService(() => createSdk({
    messages: {
      search: async () => {
        searchCalls += 1;
        return { items: [], pageInfo: { mode: "cursor", hasMore: false } };
      },
    },
  }));
  const results = await service.searchChatHistory("conversation-1", "   ");
  assert.deepEqual(results, []);
  assert.equal(searchCalls, 0);
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

test("stars a message through the CMS favorites surface with derived type", async () => {
  let createdBody: Parameters<CmsAppSdkClient["favorites"]["create"]>[0] | undefined;
  const service = createChatService(
    () => createSdk({
      conversations: {
        listMessages: async () => ({
          items: [
            messageEntry({
              body: {
                text: "https://github.com/facebook/react",
                parts: [],
              },
            }),
          ],
          pageInfo: { mode: "cursor", hasMore: false },
          highWatermark: 1,
        }),
      },
    }),
    () => createCmsSdk({
      favorites: {
        create: async (body) => {
          createdBody = body;
          return { item: {
            id: "1",
            favoriteId: "fav-1",
            favoriteType: "link",
            targetType: "im_message",
            targetId: "message-1",
            targetUuid: null,
            targetUrl: null,
            title: "https://github.com/facebook/react",
            summary: "https://github.com/facebook/react",
            sourceDisplayName: "User",
            media: null,
            favoritedAt: "2026-07-29T00:00:00Z",
          } };
        },
      },
    }),
  );

  await service.starMessage("conversation-1", "message-1", true);
  assert.equal(createdBody?.targetType, "im_message");
  assert.equal(createdBody?.targetId, "message-1");
  assert.equal(createdBody?.favoriteType, "link");
  assert.equal(createdBody?.sourceDisplayName, "User");
});

test("derives image favorite type from media message parts", async () => {
  let createdBody: Parameters<CmsAppSdkClient["favorites"]["create"]>[0] | undefined;
  const service = createChatService(
    () => createSdk({
      conversations: {
        listMessages: async () => ({
          items: [
            messageEntry({
              body: {
                text: "",
                parts: [
                  {
                    kind: "media",
                    drive: { driveUri: "drive://space-1/node-1", spaceId: "space-1", nodeId: "node-1" },
                    resource: {
                      source: "drive",
                      uri: "drive://node-1",
                      mediaKind: "image",
                    },
                  },
                ],
              },
            }),
          ],
          pageInfo: { mode: "cursor", hasMore: false },
          highWatermark: 1,
        }),
      },
    }),
    () => createCmsSdk({
      favorites: {
        create: async (body) => {
          createdBody = body;
          return { item: {
            id: "1",
            favoriteId: "fav-1",
            favoriteType: "image",
            targetType: "im_message",
            targetId: "message-1",
            targetUuid: null,
            targetUrl: null,
            title: "image",
            summary: "",
            sourceDisplayName: "User",
            media: null,
            favoritedAt: "2026-07-29T00:00:00Z",
          } };
        },
      },
    }),
  );

  await service.starMessage("conversation-1", "message-1", true);
  assert.equal(createdBody?.favoriteType, "image");
});

test("scans cursor-paginated CMS favorites before failing to unstar", async () => {
  let listCalls = 0;
  const service = createChatService(
    () => createSdk(),
    () => createCmsSdk({
      favorites: {
        list: async () => {
          listCalls += 1;
          return {
            items: [],
            pageInfo: { mode: "cursor", hasMore: true, nextCursor: "next" },
          };
        },
      },
    }),
  );

  await assert.rejects(service.starMessage("conversation-1", "message-1", false), /repeated cursor/);
  assert.equal(listCalls, 2);
});

test("unstars a message by deleting the matching CMS im_message favorite", async () => {
  let deletedId: string | undefined;
  const service = createChatService(
    () => createSdk(),
    () => createCmsSdk({
      favorites: {
        list: async () => ({
          items: [
            {
              id: "1",
              favoriteId: "fav-9",
              favoriteType: "chat",
              targetType: "im_message",
              targetId: "message-1",
              targetUuid: null,
              targetUrl: null,
              title: "Hello",
              summary: "Hello",
              sourceDisplayName: "User",
              media: null,
              favoritedAt: "2026-07-29T00:00:00Z",
            },
          ],
          pageInfo: { mode: "cursor", nextCursor: null, hasMore: false },
        }),
        delete: async (favoriteId) => {
          deletedId = favoriteId;
          return { deleted: true };
        },
      },
    }),
  );

  await service.starMessage("conversation-1", "message-1", false);
  assert.equal(deletedId, "fav-9");
});

test("maps a system-type message with a text part to a system message", async () => {
  const service = createChatService(() => createSdk({
    conversations: {
      listMessages: async () => ({
        items: [
          messageEntry({
            messageId: "welcome-1",
            messageType: "system",
            sender: { id: "system", kind: "system", displayName: "System" },
            summary: "欢迎使用 SDKWork 即时通讯！",
            body: {
              // The server message body carries text parts, not a text field.
              parts: [{ kind: "text", text: "欢迎使用 SDKWork 即时通讯！" }],
            },
          }),
        ],
        pageInfo: { mode: "cursor", hasMore: false },
        highWatermark: 1,
      }),
    },
  }));

  const [message] = await service.getMessages("conversation-1");

  assert.equal(message?.type, "system");
  assert.equal(message?.senderId, "system");
  assert.equal(message?.content, "欢迎使用 SDKWork 即时通讯！");
});

test("maps a data-part message to a system message with a derived summary", async () => {
  const service = createChatService(() => createSdk({
    conversations: {
      listMessages: async () => ({
        items: [
          messageEntry({
            messageId: "signal-1",
            messageType: "standard",
            sender: { id: "user-1", kind: "user" },
            summary: "Call started",
            body: {
              parts: [{
                kind: "data",
                schemaRef: "urn:sdkwork:sdkwork-im:message:call",
                encoding: "json",
                payload: "{}",
              }],
            },
          }),
        ],
        pageInfo: { mode: "cursor", hasMore: false },
        highWatermark: 1,
      }),
    },
  }));

  const [message] = await service.getMessages("conversation-1");

  assert.equal(message?.type, "system");
  assert.equal(message?.content, "Call started");
});

test("keeps media messages media-typed even when the server declares them system", async () => {
  const service = createChatService(() => createSdk({
    conversations: {
      listMessages: async () => ({
        items: [
          messageEntry({
            messageId: "media-1",
            messageType: "system",
            sender: { id: "system", kind: "system" },
            summary: "image",
            body: {
              parts: [{
                kind: "media",
                drive: { driveUri: "drive://space-1/node-1", spaceId: "space-1", nodeId: "node-1" },
                resource: {
                  source: "drive",
                  uri: "drive://node-1",
                  kind: "image",
                },
              }],
            },
          }),
        ],
        pageInfo: { mode: "cursor", hasMore: false },
        highWatermark: 1,
      }),
    },
  }));

  const [message] = await service.getMessages("conversation-1");

  assert.equal(message?.type, "image");
});

test("recalls a message through the IM SDK", async () => {
  const calls: string[] = [];
  const service = createChatService(() => createSdk({
    messages: {
      recall: async (messageId) => {
        calls.push(messageId);
        return { conversationId: "conversation-1", eventId: "event-1", messageId, messageSeq: 1 };
      },
    },
  }));

  await service.recallMessage("conversation-1", "message-1");

  assert.deepEqual(calls, ["message-1"]);
});

test("edits a text message and re-reads it from history", async () => {
  let editedBody: unknown;
  const service = createChatService(() => createSdk({
    conversations: {
      listMessages: async () => ({
        items: [
          messageEntry({
            messageId: "message-1",
            summary: "Edited content",
            body: { text: "Edited content", parts: [] },
          }),
        ],
        pageInfo: { mode: "cursor", hasMore: false },
        highWatermark: 1,
      }),
    },
    messages: {
      edit: async (_messageId, body) => {
        editedBody = body;
        return { conversationId: "conversation-1", eventId: "event-1", messageId: "message-1", messageSeq: 2 };
      },
    },
  }));

  const message = await service.editMessage("conversation-1", "message-1", "  Edited content  ");

  assert.deepEqual(editedBody, { text: "Edited content" });
  assert.equal(message.content, "Edited content");
});

test("rejects editing with empty content before calling the SDK", async () => {
  let calls = 0;
  const service = createChatService(() => createSdk({
    messages: {
      edit: async () => {
        calls += 1;
        return { conversationId: "conversation-1", eventId: "event-1", messageId: "message-1", messageSeq: 2 };
      },
    },
  }));

  await assert.rejects(
    service.editMessage("conversation-1", "message-1", "   "),
    /required/,
  );
  assert.equal(calls, 0);
});

test("updates the conversation profile through updateProfile", async () => {
  let receivedBody: unknown;
  const service = createChatService(() => createSdk({
    conversations: {
      updateProfile: async (_conversationId, body) => {
        receivedBody = body;
        return {
          tenantId: "tenant-1",
          conversationId: "conversation-1",
          displayName: "New name",
          avatarUrl: "",
          notice: "",
          updatedAt: "2026-07-29T00:00:00Z",
        };
      },
    },
  }));

  await service.updateChatProfile("conversation-1", { displayName: "New name" });

  assert.deepEqual(receivedBody, { displayName: "New name" });
});

test("resolves the current member role for group management", async () => {
  const service = createChatService(() => createSdk({
    conversations: {
      getCurrentMember: async () => ({
        tenantId: "tenant-1",
        conversationId: "conversation-1",
        memberId: "member-1",
        principalId: "current-user",
        principalKind: "user",
        role: "owner",
        state: "joined",
        joinedAt: "2026-07-29T00:00:00Z",
      }),
    },
  }));

  assert.equal(await service.getMyConversationRole("conversation-1"), "owner");
});

test("removes a group member by resolving its member id", async () => {
  const calls: Array<[string, unknown]> = [];
  const service = createChatService(() => createSdk({
    conversations: {
      listMembers: async () => ({
        items: [
          {
            tenantId: "tenant-1",
            conversationId: "conversation-1",
            memberId: "member-2",
            principalId: "user-2",
            principalKind: "user",
            role: "member",
            state: "joined",
            joinedAt: "2026-07-29T00:00:00Z",
          },
        ],
        pageInfo: { mode: "cursor", hasMore: false },
      }),
      removeMember: async (conversationId, body) => {
        calls.push([conversationId, body]);
      },
    },
  }));

  await service.removeGroupMember("conversation-1", "user-2");

  assert.deepEqual(calls, [["conversation-1", { memberId: "member-2" }]]);
});

test("leaves a group conversation through the members endpoint", async () => {
  let leftConversationId: string | undefined;
  const service = createChatService(() => createSdk({
    conversations: {
      leave: async (conversationId) => {
        leftConversationId = conversationId;
      },
    },
  }));

  await service.leaveGroupChat("conversation-1");

  assert.equal(leftConversationId, "conversation-1");
});
