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
assert.match(
  startDirectChatSource,
  /const projectedConversationId = user\.conversationId\?\.trim\(\);[\s\S]*?if \(projectedConversationId\) \{[\s\S]*?return \{/u,
  'startDirectChat must reuse the server-projected contact conversation id and avoid privileged rebinding when it already exists',
);
assert.match(
  startDirectChatSource,
  /if \(projectedConversationId\) \{[\s\S]*?conversations\.updatePreferences\(projectedConversationId,\s*\{\s*isHidden:\s*false\s*\}\)/u,
  'startDirectChat must unhide the server-projected contact conversation instead of synthesizing a local id',
);
assert.match(
  startDirectChatSource,
  /const result = await[\s\S]*?conversations\.bindDirectChat\([\s\S]*?const boundConversationId = result\.conversationId;[\s\S]*?conversations\.updatePreferences\(boundConversationId,\s*\{\s*isHidden:\s*false\s*\}\)/u,
  'startDirectChat must bind missing direct-chat projections and unhide the returned server-owned conversation id',
);

console.log('sdkwork im pc direct chat binding contract passed.');
