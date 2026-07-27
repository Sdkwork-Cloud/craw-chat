import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const appRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const chatServiceText = readFileSync(
  join(appRoot, 'packages/sdkwork-im-pc-chat/src/services/ChatService.ts'),
  'utf8',
);

const startDirectChatMatch = chatServiceText.match(
  /\n  async startDirectChat\([\s\S]*?\n  async startAgentChat/u,
);

assert.ok(
  startDirectChatMatch,
  'ChatService must expose startDirectChat before startAgentChat',
);

const startDirectChatSource = startDirectChatMatch[0];

assert.notEqual(
  startDirectChatSource.indexOf('conversations.bindDirectChat'),
  -1,
  'startDirectChat must call conversations.bindDirectChat',
);
assert.notEqual(
  startDirectChatSource.indexOf('conversations.updatePreferences'),
  -1,
  'startDirectChat must unhide the bound direct conversation',
);
assert.doesNotMatch(
  startDirectChatSource,
  /if \(contactConversationId\) \{[\s\S]*?return \{/u,
  'startDirectChat must not treat a contact conversation id as proof of active conversation membership',
);
assert.doesNotMatch(
  startDirectChatSource,
  /conversations\.updatePreferences\(contactConversationId/u,
  'startDirectChat must update preferences only on the conversation returned by the binding operation',
);
assert.match(
  startDirectChatSource,
  /const result = await[\s\S]*?conversations\.bindDirectChat\([\s\S]*?const boundConversationId = result\.conversationId;[\s\S]*?conversations\.updatePreferences\(boundConversationId,\s*\{\s*isHidden:\s*false\s*\}\)/u,
  'startDirectChat must always resolve normalized direct-chat membership before unhiding the returned server-owned conversation id',
);

console.log('sdkwork im pc direct chat binding contract passed.');
