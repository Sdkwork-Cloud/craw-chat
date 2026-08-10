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
    friendships?: Partial<ContactsSdkPort["social"]["friendships"]>;
    users?: Partial<ContactsSdkPort["social"]["users"]>;
  };
}

function createSdk(overrides: ContactsSdkOverrides = {}): ContactsSdkPort {
  return {
    conversations: {
      create: async () => ({ conversationId: "conversation-1", eventId: "event-1" }),
      addMember: async () => ({}),
      ...overrides.conversations,
    },
    social: {
      contacts: {
        list: async () => ({
          items: [],
          pageInfo: { mode: "cursor", hasMore: false },
        }),
        preferences: {
          retrieve: async () => {
            throw new Error("Unexpected contact preferences retrieval");
          },
          update: async () => {
            throw new Error("Unexpected contact preferences update");
          },
        },
        ...overrides.social?.contacts,
      },
      friendRequests: {
        list: async () => ({
          items: [],
          pageInfo: { mode: "cursor", hasMore: false },
        }),
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
        accept: async () => { throw new Error("Unexpected friend request acceptance"); },
        decline: async () => { throw new Error("Unexpected friend request decline"); },
        cancel: async () => { throw new Error("Unexpected friend request cancellation"); },
        pendingCount: async () => ({ count: 0 }),
        ...overrides.social?.friendRequests,
      },
      friendships: {
        remove: async () => { throw new Error("Unexpected friendship removal"); },
        ...overrides.social?.friendships,
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

test("lists and resolves friend requests through the injected IM SDK", async () => {
  const calls: string[] = [];
  let listParams: unknown;
  const service = createContactService(() => createSdk({
    social: {
      friendRequests: {
        list: async (params) => {
          listParams = params;
          return {
            items: [{
              tenantId: "tenant-1",
              friendRequestId: "request-1",
              requesterUserId: "user-1",
              targetUserId: "current-user",
              status: "pending",
              createdAt: "2026-07-31T00:00:00Z",
              updatedAt: "2026-07-31T00:00:00Z",
            }],
            pageInfo: { mode: "cursor", hasMore: true, nextCursor: "next" },
          };
        },
        pendingCount: async () => ({ count: 1 }),
        accept: async (requestId) => {
          calls.push(`accept:${requestId}`);
          return {} as Awaited<ReturnType<ContactsSdkPort["social"]["friendRequests"]["accept"]>>;
        },
        decline: async (requestId) => {
          calls.push(`decline:${requestId}`);
          return {} as Awaited<ReturnType<ContactsSdkPort["social"]["friendRequests"]["decline"]>>;
        },
      },
    },
  }));

  const page = await service.listFriendRequests("incoming", "cursor-1");
  assert.deepEqual(listParams, {
    cursor: "cursor-1",
    direction: "incoming",
    pageSize: 50,
    status: "pending",
  });
  assert.equal(page.items[0]?.friendRequestId, "request-1");
  assert.equal(page.nextCursor, "next");
  assert.equal(await service.getPendingFriendRequestCount(), 1);

  await service.acceptFriendRequest(" request-1 ");
  await service.declineFriendRequest(" request-2 ");
  assert.deepEqual(calls, ["accept:request-1", "decline:request-2"]);
});

test("starts a direct conversation with a client id and attaches the member", async () => {
  let request: unknown;
  let memberRequest: unknown;
  const service = createContactService(() => createSdk({
    conversations: {
      create: async (body) => {
        request = body;
        return { conversationId: (body as { conversationId?: string }).conversationId ?? "conversation-1", eventId: "event-1" };
      },
      addMember: async (_conversationId, body) => {
        memberRequest = body;
      },
    },
  }));

  const conversationId = await service.startDirectConversation("user-1");

  assert.match(conversationId, /^direct-/u);
  assert.deepEqual(
    (request as { conversationId?: string }).conversationId,
    conversationId,
  );
  assert.equal((request as { memberUserIds?: string[] }).memberUserIds, undefined);
  assert.equal((request as { clientRequestKey?: string }).clientRequestKey, undefined);
  assert.deepEqual(memberRequest, {
    principalId: "user-1",
    principalKind: "user",
    role: "member",
  });
});

test("removes a friendship through the resolved contact friendship id", async () => {
  let removedFriendshipId: string | undefined;
  const service = createContactService(() => createSdk({
    social: {
      contacts: {
        list: async () => ({
          items: [{
            tenantId: "tenant-1",
            ownerUserId: "current-user",
            targetUserId: "user-1",
            displayName: "Alice",
            avatarUrl: "",
            contactType: "friend",
            relationshipState: "friends",
            friendshipId: "friendship-9",
            establishedAt: "2026-07-29T00:00:00Z",
            lastInteractionAt: "2026-07-29T00:00:00Z",
            isStarred: false,
            isBlocked: false,
            updatedAt: "2026-07-29T00:00:00Z",
          }],
          pageInfo: { mode: "cursor", hasMore: false },
        }),
      },
      friendships: {
        remove: async (friendshipId) => {
          removedFriendshipId = friendshipId;
          return { friendshipId, deleted: true };
        },
      },
    },
  }));

  await service.removeFriend("user-1");

  assert.equal(removedFriendshipId, "friendship-9");
});

test("rejects removing a friendship without a resolved friendship id", async () => {
  let removeCalls = 0;
  const service = createContactService(() => createSdk({
    social: {
      friendships: {
        remove: async () => {
          removeCalls += 1;
          return {};
        },
      },
    },
  }));

  await assert.rejects(service.removeFriend("user-1"), /Friendship not found/);
  assert.equal(removeCalls, 0);
});

test("blocks a contact through contact preferences", async () => {
  let receivedBody: unknown;
  const service = createContactService(() => createSdk({
    social: {
      contacts: {
        preferences: {
          retrieve: async () => {
            throw new Error("Unexpected preferences retrieval");
          },
          update: async (targetUserId, body) => {
            receivedBody = body;
            return {
              tenantId: "tenant-1",
              ownerUserId: "current-user",
              targetUserId,
              isStarred: false,
              remark: "",
              isBlocked: true,
              updatedAt: "2026-07-29T00:00:00Z",
            };
          },
        },
      },
    },
  }));

  const preferences = await service.blockContact(" user-1 ");

  assert.deepEqual(receivedBody, { isBlocked: true });
  assert.equal(preferences.isBlocked, true);
});

test("retrieves and updates contact preferences through the SDK", async () => {
  const calls: string[] = [];
  const service = createContactService(() => createSdk({
    social: {
      contacts: {
        preferences: {
          retrieve: async (targetUserId) => {
            calls.push(`retrieve:${targetUserId}`);
            return {
              tenantId: "tenant-1",
              ownerUserId: "current-user",
              targetUserId,
              isStarred: true,
              remark: "Preferred",
              isBlocked: false,
              updatedAt: "2026-07-29T00:00:00Z",
            };
          },
          update: async (targetUserId, body) => {
            calls.push(`update:${targetUserId}`);
            return {
              tenantId: "tenant-1",
              ownerUserId: "current-user",
              targetUserId,
              isStarred: body.isStarred ?? false,
              remark: body.remark ?? "",
              isBlocked: false,
              updatedAt: "2026-07-29T00:00:00Z",
            };
          },
        },
      },
    },
  }));

  const retrieved = await service.getContactPreferences("user-1");
  assert.equal(retrieved.remark, "Preferred");
  assert.equal(retrieved.isStarred, true);

  const updated = await service.updateContactPreferences("user-1", { isStarred: false });
  assert.equal(updated.isStarred, false);

  assert.deepEqual(calls, ["retrieve:user-1", "update:user-1"]);
});
