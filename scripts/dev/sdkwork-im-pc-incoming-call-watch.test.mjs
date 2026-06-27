import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const read = (relativePath) => fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');

const chatServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService.ts');

assert.match(
  chatServiceSource,
  /export function projectDirectChatConversationId/u,
  'chat service must export stable direct-chat conversation id projection for RTC watch coverage',
);
assert.match(
  chatServiceSource,
  /export function resolveIncomingCallWatchConversationIds/u,
  'chat service must export incoming call watch conversation id resolution',
);
assert.match(
  chatServiceSource,
  /projectDirectChatConversationId\s*\([\s\S]*?buildDirectChatStableIds/u,
  'direct chat conversation projection must reuse the same stable id builder as startDirectChat',
);
assert.match(
  chatServiceSource,
  /resolveIncomingCallWatchConversationIds\s*\([\s\S]*?projectDirectChatConversationId/u,
  'incoming call watch resolution must project contact-only direct chats even before they appear in chat list hydration',
);

console.log('sdkwork-im-pc incoming call watch contract passed');
