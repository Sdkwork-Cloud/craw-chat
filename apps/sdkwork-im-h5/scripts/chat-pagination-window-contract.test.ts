import assert from "node:assert/strict";

import type {
  ConversationInboxEntry,
  ConversationMessageEntry,
  SdkWorkListPageInfo,
} from "@sdkwork/im-sdk";

import {
  MAX_INBOX_ENTRIES,
  mergeInboxPage,
} from "../packages/sdkwork-im-h5-chat/src/services/chatInboxUtils";
import {
  MAX_MESSAGE_HISTORY_ENTRIES,
  mergeConversationMessagePage,
  readCursorPageInfo,
} from "../packages/sdkwork-im-h5-chat/src/services/chatMessageHistoryUtils";

function message(messageSeq: number): ConversationMessageEntry {
  return {
    messageId: `message-${messageSeq}`,
    messageSeq,
    occurredAt: `2026-07-16T00:${String(messageSeq % 60).padStart(2, "0")}:00.000Z`,
  } as ConversationMessageEntry;
}

function inboxEntry(index: number): ConversationInboxEntry {
  return { conversationId: `conversation-${index}` } as ConversationInboxEntry;
}

const opaqueCursor = "eyJzZWVrIjoiY29udmVyc2F0aW9uLTAwMDAwMSJ9";
const pageInfo = readCursorPageInfo({
  hasMore: true,
  nextCursor: opaqueCursor,
} as SdkWorkListPageInfo);
assert.equal(pageInfo.hasMore, true);
assert.equal(pageInfo.nextCursor, opaqueCursor);

const newestMessages = Array.from(
  { length: MAX_MESSAGE_HISTORY_ENTRIES },
  (_, index) => message(index + 51),
);
const olderMessages = Array.from({ length: 50 }, (_, index) => message(index + 1));
const olderHistoryPage = mergeConversationMessagePage(
  newestMessages,
  olderMessages,
  "older",
);
assert.equal(olderHistoryPage.incomingPageRetained, true);
assert.equal(olderHistoryPage.entries.length, MAX_MESSAGE_HISTORY_ENTRIES);
assert.ok(
  olderMessages.every((entry) => olderHistoryPage.entries.some(
    (retained) => retained.messageId === entry.messageId,
  )),
  "loading an older page must retain every newly fetched message before its cursor advances",
);
assert.ok(
  !olderHistoryPage.entries.some((entry) => entry.messageId === "message-550"),
  "older-page retention evicts the newest end of a full window",
);

const oversizedHistoryPage = mergeConversationMessagePage(
  [],
  Array.from(
    { length: MAX_MESSAGE_HISTORY_ENTRIES + 1 },
    (_, index) => message(index + 1),
  ),
  "older",
);
assert.equal(
  oversizedHistoryPage.incomingPageRetained,
  false,
  "a message page that cannot fit in the bounded window must not be accepted for cursor advancement",
);

const currentInbox = Array.from(
  { length: MAX_INBOX_ENTRIES },
  (_, index) => inboxEntry(index),
);
const olderInbox = Array.from({ length: 20 }, (_, index) => inboxEntry(index + MAX_INBOX_ENTRIES));
const olderInboxPage = mergeInboxPage(currentInbox, olderInbox, "older");
assert.equal(olderInboxPage.incomingPageRetained, true);
assert.equal(olderInboxPage.entries.length, MAX_INBOX_ENTRIES);
assert.ok(
  olderInbox.every((entry) => olderInboxPage.entries.some(
    (retained) => retained.conversationId === entry.conversationId,
  )),
  "loading an older inbox page must retain every newly fetched conversation before its cursor advances",
);
assert.ok(
  !olderInboxPage.entries.some((entry) => entry.conversationId === "conversation-0"),
  "older inbox paging evicts the newest end of a full window",
);

const oversizedInboxPage = mergeInboxPage(
  [],
  Array.from({ length: MAX_INBOX_ENTRIES + 1 }, (_, index) => inboxEntry(index)),
  "older",
);
assert.equal(
  oversizedInboxPage.incomingPageRetained,
  false,
  "a page that cannot fit in the bounded window must not be accepted for cursor advancement",
);

process.stdout.write("sdkwork-im H5 chat pagination window contract passed\n");
