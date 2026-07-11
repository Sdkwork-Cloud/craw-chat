import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import {
  readRememberedConversationTitle,
  rememberConversationTitle,
  resolveConversationInboxEntryDisplayTitle,
} from "../packages/sdkwork-im-h5-chat/src/services/chatConversationTitleStore";

const appRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function readText(...segments: string[]) {
  return fs.readFileSync(path.join(appRoot, ...segments), "utf8");
}

const storageValues = new Map<string, string>();
Object.defineProperty(globalThis, "window", {
  configurable: true,
  value: {
    sessionStorage: {
      getItem: (key: string) => storageValues.get(key) ?? null,
      removeItem: (key: string) => {
        storageValues.delete(key);
      },
      setItem: (key: string, value: string) => {
        storageValues.set(key, value);
      },
    },
  },
});

assert.equal(
  resolveConversationInboxEntryDisplayTitle({
    conversationId: "conversation.peer",
    conversationType: "single",
    displayName: null,
    lastActivityAt: new Date(0).toISOString(),
    lastMessageSeq: 0,
    messageCount: 0,
    peer: {
      displayName: "Alice Chen",
      principalId: "user.alice",
      principalKind: "user",
    },
    tenantId: "tenant.demo",
    unreadCount: 0,
  }),
  "Alice Chen",
  "H5 inbox title resolution must prefer peer displayName when the conversation displayName is absent.",
);

rememberConversationTitle("conversation.contract.title", "Product Design");
assert.equal(
  readRememberedConversationTitle("conversation.contract.title"),
  "Product Design",
  "H5 conversation title store must keep the clicked inbox title for the detail route.",
);

rememberConversationTitle("conversation.raw.id", "conversation.raw.id");
assert.equal(
  readRememberedConversationTitle("conversation.raw.id"),
  undefined,
  "H5 conversation title store must not preserve raw conversation ids as display titles.",
);

rememberConversationTitle("conversation.stale.title", "Old Title");
rememberConversationTitle("conversation.stale.title", "conversation.stale.title");
assert.equal(
  readRememberedConversationTitle("conversation.stale.title"),
  undefined,
  "H5 conversation title store must clear stale display titles when the latest title is the raw conversation id.",
);

const inboxPageSource = readText(
  "packages",
  "sdkwork-im-h5-chat",
  "src",
  "pages",
  "ChatInboxPage.tsx",
);
const conversationPageSource = readText(
  "packages",
  "sdkwork-im-h5-chat",
  "src",
  "pages",
  "ChatConversationPage.tsx",
);
const conversationServiceSource = readText(
  "packages",
  "sdkwork-im-h5-chat",
  "src",
  "services",
  "chatConversationService.ts",
);

assert.match(
  inboxPageSource,
  /rememberConversationTitle\(String\(conversationId\),\s*displayTitle\)/u,
  "ChatInboxPage must remember the clicked conversation display title before navigating to the detail route.",
);

assert.match(
  conversationPageSource,
  /readRememberedConversationTitle\(conversationId\)/u,
  "ChatConversationPage must read the remembered inbox title before falling back to the raw conversation id.",
);

assert.match(
  conversationPageSource,
  /fetchConversationProfile\(conversationId\)/u,
  "ChatConversationPage must hydrate the conversation profile title for direct detail-route entry.",
);

assert.match(
  conversationServiceSource,
  /conversations\.getProfile\(conversationId\)/u,
  "H5 conversation profile hydration must use the generated IM SDK conversation profile method.",
);

process.stdout.write("sdkwork-im-h5 chat conversation title contract passed\n");
