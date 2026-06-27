import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const messageListSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/MessageList.tsx');
const forwardModalSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ForwardModal.tsx');
const chatHistoryModalSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ChatHistoryModal.tsx');
const chatListSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ChatList.tsx');

assert.match(
  messageListSource,
  /contactService\.getContacts\(\)[\s\S]*?\.catch\s*\(/u,
  'MessageList contact hydration must handle backend failures instead of creating unhandled promises',
);
assert.match(
  messageListSource,
  /try\s*\{[\s\S]*?await\s+chatService\.getMessages\(chatId\)[\s\S]*?\}\s*catch\s*\{/u,
  'MessageList message loading must fail-close and show an error instead of leaving loading stuck',
);
assert.match(
  messageListSource,
  /finally\s*\{[\s\S]*?setLoading\(false\)/u,
  'MessageList message loading must always clear loading after success or failure',
);
assert.match(
  messageListSource,
  /try\s*\{[\s\S]*?Promise\.all\(Array\.from\(idsToDelete\)[\s\S]*?chatService\.deleteMessage/u,
  'MessageList delete must await the real SDK deletion before local state changes',
);
assert.match(
  messageListSource,
  /toast\(t\(['"]chat\.messageList\.toast\.deleteFailed['"]\),\s*['"]error['"]\)/u,
  'MessageList delete must surface SDK deletion failures',
);
assert.match(
  messageListSource,
  /await\s+favoriteService\.addFavorite/u,
  'MessageList favorite action must await the SDK-backed favorite creation',
);
assert.match(
  messageListSource,
  /toast\(t\(['"]chat\.messageList\.toast\.favoriteFailed['"]\),\s*['"]error['"]\)/u,
  'MessageList favorite action must surface SDK favorite failures',
);
assert.match(
  messageListSource,
  /try\s*\{[\s\S]*?chatService\.(?:removeReaction|addReaction)\(chatId,\s*messageId,\s*emoji\)[\s\S]*?\}\s*catch\s*\{/u,
  'MessageList reactions must handle SDK failures before local optimistic updates',
);
assert.match(
  messageListSource,
  /toast\(t\(['"]chat\.messageList\.toast\.reactionFailed['"]\),\s*['"]error['"]\)/u,
  'MessageList reactions must surface SDK reaction failures',
);

assert.match(
  forwardModalSource,
  /chatService\.getChats\(\)[\s\S]*?\.catch\s*\(/u,
  'ForwardModal chat loading must handle backend failures instead of leaving loading stuck',
);
assert.match(
  forwardModalSource,
  /chatService\.getChats\(\)[\s\S]*?\.finally\s*\(\s*\(\)\s*=>\s*setLoading\(false\)\s*\)/u,
  'ForwardModal chat loading must always clear loading after success or failure',
);

assert.match(
  chatHistoryModalSource,
  /import\s+\{\s*toast\s*\}\s+from\s+['"]\.\/Toast['"]/u,
  'ChatHistoryModal must use the existing toast surface for backend history load failures',
);
assert.match(
  chatHistoryModalSource,
  /chatService\.getMessages\(resolvedChatId\)[\s\S]*?\.catch\s*\(\s*\(\)\s*=>\s*\{[\s\S]*?toast\(t\(['"]chat\.historySearch\.toast\.loadFailed['"]\),\s*['"]error['"]\)/u,
  'ChatHistoryModal must surface history load failures instead of logging only to console',
);

assert.match(
  chatListSource,
  /try\s*\{[\s\S]*?await\s+chatService\.pinChat/u,
  'ChatList context menu preference actions must await SDK mutations inside handled async blocks',
);
assert.match(
  chatListSource,
  /catch\s*\{[\s\S]*?toast\(t\(['"]chat\.list\.toast\.operationFailed['"]\),\s*['"]error['"]\)/u,
  'ChatList context menu actions must surface localized SDK failures',
);
assert.match(
  chatListSource,
  /void\s+chatService\.markAsRead\(chat\.id\)\s*\.then/u,
  'ChatList click read cursor update must be explicitly fire-and-forget',
);
assert.match(
  chatListSource,
  /markAsRead\(chat\.id\)\s*\.then\([\s\S]*?\.catch\s*\(\s*\(\)\s*=>\s*toast\(t\(['"]chat\.list\.toast\.markReadFailed['"]\),\s*['"]error['"]\)/u,
  'ChatList click read cursor update must surface localized failures instead of leaving unhandled promises',
);

console.log('sdkwork-im-pc message UI failure handling contract passed');
