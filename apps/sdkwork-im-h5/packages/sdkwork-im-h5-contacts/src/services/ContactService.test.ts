import assert from "node:assert/strict";
import test from "node:test";

import {
  createContactService,
  type ContactsSdkPort,
} from "./ContactService";

interface ContactsSdkOverrides {
  conversations?: Partial<ContactsSdkPort["conversations"]>;
  social?: {
    contacts?: Partial<ContactsSdkPort["social"]["contacts"]>;
    friendRequests?: Partial<ContactsSdkPort["social"]["friendRequests"]>;
    users?: Partial<ContactsSdkPort["social"]["users"]>;
  };
}

function createSdk(overrides: ContactsSdkOverrides = {}): ContactsSdkPort {
  return {
    conversations: {
      create: async () => ({ conversationId: "conversation-1", eventId: "event-1" }),
      ...overrides.conversations,
    },
    social: {
      contacts: {
        list: async () => ({
          items: [],
          pageInfo: { mode: "cursor", hasMore: false },
        }),
        ...overrides.social?.contacts,
      },
      friendRequests: {
        create: async (body) => ({
          friendRequest: {
            tenantId: "tenant-1",
            friendRequestId: "request-1",
            requesterUserId: "current-user",
            targetUserId: body.targetUserId,
            status: "pending",
            requestMessage: body.requestMessage,
            createdAt: "2026-07-29T00:00:00Z",
            updatedAt: "2026-07-29T00:00:00Z",
          },
        }),
        ...overrides.social?.friendRequests,
      },
      users: {
        list: async () => ({
          items: [],
          pageInfo: { mode: "cursor", hasMore: false },
        }),
        ...overrides.social?.users,
      },
    },
  };
}

test("lists one bounded cursor page and maps authoritative contact fields", async () => {
  let receivedParams: unknown;
  const service = createContactService(() => createSdk({
    social: {
      contacts: {
        list: async (params) => {
          receivedParams = params;
          return {
            items: [{
              tenantId: "tenant-1",
              ownerUserId: "current-user",
              targetUserId: "user-1",
              displayName: "Display name",
              avatarUrl: "https://cdn.example.test/avatar.png",
              contactType: "friend",
              relationshipState: "friends",
              friendshipId: "friendship-1",
              conversationId: "conversation-1",
              establishedAt: "2026-07-29T00:00:00Z",
              lastInteractionAt: "2026-07-29T00:00:00Z",
              isStarred: false,
              isBlocked: false,
              remark: "Preferred name",
              updatedAt: "2026-07-29T00:00:00Z",
            }],
            pageInfo: { mode: "cursor", hasMore: true, nextCursor: "next" },
          };
        },
      },
    },
  }));

  const page = await service.listContactPage("cursor-1");

  assert.deepEqual(receivedParams, { cursor: "cursor-1", pageSize: 50 });
  assert.equal(page.items[0]?.name, "Preferred name");
  assert.equal(page.items[0]?.conversationId, "conversation-1");
  assert.equal(page.nextCursor, "next");
});

test("rejects incomplete contact cursor metadata", async () => {
  const service = createContactService(() => createSdk({
    social: {
      contacts: {
        list: async () => ({
          items: [],
          pageInfo: { mode: "cursor", hasMore: true },
        }),
      },
    },
  }));

  await assert.rejects(service.listContactPage(), /hasMore without nextCursor/);
});

test("searches social users through one bounded server page", async () => {
  let receivedParams: unknown;
  const service = createContactService(() => createSdk({
    social: {
      users: {
        list: async (params) => {
          receivedParams = params;
          return {
            items: [{
              tenantId: "tenant-1",
              userId: "user-1",
              chatId: "chat-1",
              displayName: "Alice",
              relationshipState: "none",
            }],
            pageInfo: { mode: "cursor", hasMore: false },
          };
        },
      },
    },
  }));

  const results = await service.searchFriends("  Alice  ");

  assert.deepEqual(receivedParams, { pageSize: 20, q: "Alice" });
  assert.equal(results[0]?.id, "user-1");
});

test("creates a real friend request for the selected user ID", async () => {
  let request: unknown;
  const service = createContactService(() => createSdk({
    social: {
      friendRequests: {
        create: async (body) => {
          request = body;
          return {
            friendRequest: {
              tenantId: "tenant-1",
              friendRequestId: "request-1",
              requesterUserId: "current-user",
              targetUserId: body.targetUserId,
              status: "pending",
              createdAt: "2026-07-29T00:00:00Z",
              updatedAt: "2026-07-29T00:00:00Z",
            },
          };
        },
      },
    },
  }));

  await service.addFriend(" user-1 ", " Hello ");
  assert.deepEqual(request, { targetUserId: "user-1", requestMessage: "Hello" });
});

test("starts a direct conversation with a UUID request key", async () => {
  let request: unknown;
  const service = createContactService(() => createSdk({
    conversations: {
      create: async (body) => {
        request = body;
        return { conversationId: "conversation-1", eventId: "event-1" };
      },
    },
  }));

  const conversationId = await service.startDirectConversation("user-1");

  assert.equal(conversationId, "conversation-1");
  assert.deepEqual((request as { memberUserIds?: string[] }).memberUserIds, ["user-1"]);
  assert.match(
    String((request as { clientRequestKey?: string }).clientRequestKey),
    /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/iu,
  );
});
