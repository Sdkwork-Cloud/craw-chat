import assert from "node:assert/strict";
import test from "node:test";

import {
  createChatConversationService,
  type ChatConversationSdkPort,
} from "./chatConversationService";

function createSdk(): ChatConversationSdkPort {
  return {
    conversations: {
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
        deliveryStatus: "applied" as const,
        eventId: "event-1",
        messageId: "message-1",
        messageSeq: 1,
      }),
      updatePreferences: async () => ({
        tenantId: "tenant-1",
        conversationId: "conversation-1",
        principalKind: "user",
        principalId: "user-1",
        isPinned: false,
        isMuted: false,
        isMarkedUnread: false,
        isHidden: false,
        updatedAt: "2026-07-31T00:00:00Z",
      }),
      updateReadCursor: async () => ({
        tenantId: "tenant-1",
        conversationId: "conversation-1",
        principalKind: "user",
        principalId: "user-1",
        readSeq: 0,
        updatedAt: "2026-07-31T00:00:00Z",
      }),
    },
  };
}

test("posts text with a unique client message id", async () => {
  let options: { clientMsgId?: string | null } | undefined;
  const sdk = createSdk();
  sdk.conversations.postText = async (_conversationId, _text, body) => {
    options = body;
    return {
      deliveryStatus: "applied" as const,
      eventId: "event-1",
      messageId: "message-1",
      messageSeq: 1,
    };
  };
  const service = createChatConversationService(() => sdk);

  await service.postText("conversation-1", "Hello");

  assert.match(
    options?.clientMsgId ?? "",
    /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/iu,
  );
});

test("commits the server high watermark and clears marked-unread preference", async () => {
  const calls: unknown[] = [];
  const sdk = createSdk();
  sdk.conversations.updateReadCursor = async (conversationId, body) => {
    calls.push(["cursor", conversationId, body]);
    return {
      tenantId: "tenant-1",
      conversationId,
      principalKind: "user",
      principalId: "user-1",
      readSeq: body.readSeq,
      updatedAt: "2026-07-31T00:00:00Z",
    };
  };
  sdk.conversations.updatePreferences = async (conversationId, body) => {
    calls.push(["preferences", conversationId, body]);
    return {
      tenantId: "tenant-1",
      conversationId,
      principalKind: "user",
      principalId: "user-1",
      isPinned: false,
      isMuted: false,
      isMarkedUnread: false,
      isHidden: false,
      updatedAt: "2026-07-31T00:00:00Z",
    };
  };
  const service = createChatConversationService(() => sdk);

  await service.markConversationRead("conversation-1", 42);

  assert.deepEqual(calls, [
    ["cursor", "conversation-1", { readSeq: 42 }],
    ["preferences", "conversation-1", { isMarkedUnread: false }],
  ]);
});

test("rejects invalid read sequence before calling the SDK", async () => {
  let calls = 0;
  const sdk = createSdk();
  sdk.conversations.updateReadCursor = async () => {
    calls += 1;
    return {} as never;
  };
  const service = createChatConversationService(() => sdk);

  await assert.rejects(
    service.markConversationRead("conversation-1", Number.NaN),
    RangeError,
  );
  assert.equal(calls, 0);
});
