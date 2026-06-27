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
  /async startDirectChat\s*\([\s\S]*?this\.client\(\)\.conversations\.bindDirectChat/u,
  'startDirectChat must bind direct chats through the generated IM SDK',
);
assert.match(
  chatServiceSource,
  /buildDirectChatStableIds/u,
  'startDirectChat must derive stable direct-chat ids before binding conversations',
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
  'startup sync must refresh chat message windows through ChatService',
);
assert.match(
  imSyncSource,
  /syncStartup[\s\S]*?this\.contactService\.syncContacts/u,
  'startup sync must refresh contacts through ContactService',
);
assert.match(
  imSyncSource,
  /syncStartup[\s\S]*?this\.groupService\.syncGroupMembers/u,
  'startup sync must refresh group members through GroupService',
);

console.log('sdkwork-im-pc startup and direct chat contract passed');
