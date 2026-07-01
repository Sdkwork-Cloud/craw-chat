import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const read = (relativePath) => fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');

const chatServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService.ts');

assert.match(
  chatServiceSource,
  /export function resolveIncomingCallWatchConversationIds/u,
  'chat service must export incoming call watch conversation id resolution',
);
assert.doesNotMatch(
  chatServiceSource,
  /export function projectDirectChatConversationId/u,
  'incoming call watch must not rely on client-side direct-chat id projection',
);
assert.doesNotMatch(
  chatServiceSource,
  /buildDirectChatStableIds/u,
  'direct chat ids must be server-assigned canonical values, not client-built pc-direct ids',
);
assert.match(
  chatServiceSource,
  /resolveIncomingCallWatchConversationIds\s*\([\s\S]*?contact\.conversationId/u,
  'incoming call watch resolution must use server-owned contact conversation ids',
);

console.log('sdkwork-im-pc incoming call watch contract passed');
