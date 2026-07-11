import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const read = (relativePath) => fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');

const chatServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService.ts');
const imSyncSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ImSyncCoordinatorService.ts');

assert.match(
  chatServiceSource,
  /async startDirectChat\s*\([\s\S]*?\(?await\s+this\.client\(\)\)?\.conversations\.bindDirectChat/u,
  'startDirectChat must bind direct chats through the generated IM SDK',
);
assert.match(
  chatServiceSource,
  /bindDirectChat/u,
  'startDirectChat must bind direct chats through the generated IM SDK without client-local ids',
);
assert.doesNotMatch(
  chatServiceSource,
  /buildDirectChatStableIds/u,
  'startDirectChat must not derive pc-direct client-local conversation ids',
);
assert.match(
  chatServiceSource,
  /conversations\.updateProfile/u,
  'startDirectChat must sync direct chat display profile through the IM SDK',
);
assert.match(
  chatServiceSource,
  /conversations\.updatePreferences[\s\S]*?isHidden:\s*false/u,
  'startDirectChat must unhide the real direct chat conversation through the IM SDK',
);

assert.match(
  imSyncSource,
  /syncStartup[\s\S]*?this\.chatService\.syncOfflineMessages/u,
  'startup sync must refresh chat inbox metadata through ChatService without preloading every message window',
);
assert.doesNotMatch(
  imSyncSource,
  /this\.contactService|result\.contacts\s*=/u,
  'startup sync must not refresh contacts before the contacts surface is opened',
);
assert.doesNotMatch(
  imSyncSource,
  /syncStartup[\s\S]*?this\.groupService\.syncGroupMembers/u,
  'startup sync must not refresh every group member list before a group conversation is active',
);

console.log('sdkwork-im-pc startup and direct chat contract passed');
