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
const chatLayoutSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/pages/ChatLayout.tsx');

assert.match(
  messageListSource,
  /contactService\.getUserById\(participantId\)[\s\S]*?\.catch\s*\(\s*\(\)\s*=>\s*undefined\s*\)/u,
  'MessageList participant hydration must handle backend failures instead of creating unhandled promises',
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
  /Promise\.allSettled\(\s*Array\.from\(idsToDelete\)\.map\(\(messageId\)\s*=>\s*chatService\.deleteMessage\(chatId,\s*messageId\)\)/u,
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
  /chatService\.listChatsPage\([^)]*\)[\s\S]*?\.catch\s*\(/u,
  'ForwardModal chat loading must handle backend failures instead of leaving loading stuck',
);
assert.match(
  forwardModalSource,
  /chatService\.listChatsPage\([^)]*\)[\s\S]*?\.finally\s*\(\s*\(\)\s*=>\s*setLoading\(false\)\s*\)/u,
  'ForwardModal chat loading must always clear loading after success or failure',
);
assert.doesNotMatch(
  forwardModalSource,
  /chatService\.getChats\(\)/u,
  'ForwardModal must not aggregate every inbox page for interactive forwarding; use listChatsPage and explicit pagination.',
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
const chatListItemClickImplementation = chatListSource.match(
  /onClick=\{\(\)\s*=>\s*\{([\s\S]*?)\n\s+\}\}/u,
)?.[1] ?? '';
assert.match(
  chatListItemClickImplementation,
  /onChatSelect\(chat\)/u,
  'ChatList item click must delegate selection to ChatLayout.',
);
assert.doesNotMatch(
  chatListItemClickImplementation,
  /markAsRead/u,
  'ChatList item click must not duplicate read cursor mutations.',
);
assert.match(
  chatLayoutSource,
  /const\s+markSelectedChatAsRead\s*=\s*\(chat:\s*Chat\):\s*void\s*=>\s*\{[\s\S]*?pendingReadCursorChatIdsRef\.current\.has\(chat\.id\)[\s\S]*?chatService\.markAsRead\(chat\.id\)[\s\S]*?toast\(t\(["']chat\.list\.toast\.markReadFailed["']\),\s*["']error["']\)/u,
  'ChatLayout must own click/focus read cursor updates, deduplicate in-flight updates, and surface localized failures.',
);

console.log('sdkwork-im-pc message UI failure handling contract passed');
