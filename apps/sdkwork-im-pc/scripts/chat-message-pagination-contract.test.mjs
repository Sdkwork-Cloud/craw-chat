import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const chatServiceSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/services/ChatService.ts',
  'utf8',
);
const messageListSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/components/MessageList.tsx',
  'utf8',
);
const chatListSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/components/ChatList.tsx',
  'utf8',
);
const chatLayoutSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/pages/ChatLayout.tsx',
  'utf8',
);
const conversationsModuleSource = readFileSync(
  '../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/conversations-module.ts',
  'utf8',
);
const transportClientLikeSource = readFileSync(
  '../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/transport-client-like.ts',
  'utf8',
);

assert.match(
  transportClientLikeSource,
  /export\s+interface\s+MessageHistoryListParams\s*\{[\s\S]*?cursor\?:\s*string;[\s\S]*?pageSize\?:\s*number;[\s\S]*?\}/u,
  'the composed IM SDK must expose a message-history-specific cursor/page-size parameter contract.',
);
assert.match(
  conversationsModuleSource,
  /listMessages\(\s*conversationId:\s*string,\s*params\?:\s*MessageHistoryListParams,?\s*\)/u,
  'the composed IM SDK must not expose the broad transport QueryParams type for message history.',
);

assert.doesNotMatch(
  chatServiceSource,
  /listMessages\(\s*chatId,\s*\{\s*pageSize:\s*1\s*\}/u,
  'markAsRead must not issue an extra messages list request with pageSize=1 when a conversation item is clicked.',
);

assert.match(
  chatServiceSource,
  /private\s+resolveReadSeqForMarkAsRead\(chatId:\s*string\):\s*number/u,
  'markAsRead should resolve the read cursor from cached inbox/message websocket sequence state synchronously.',
);

assert.match(
  chatServiceSource,
  /return\s+this\.latestReadSeq\.get\(chatId\)\s*\?\?\s*0/u,
  'markAsRead must use cached inbox/message websocket sequence state instead of issuing an extra message-history read.',
);

const resolveReadSeqForMarkAsReadImplementation = chatServiceSource.match(
  /private\s+resolveReadSeqForMarkAsRead\(chatId:\s*string\):\s*number\s*\{([\s\S]*?)\n\s+\}/u,
)?.[1] ?? '';
assert.doesNotMatch(
  resolveReadSeqForMarkAsReadImplementation,
  /listMessages|pageSize:\s*1|await/u,
  'read cursor resolution must not list messages, peek pageSize=1, or await network IO.',
);

assert.match(
  chatServiceSource,
  /interface\s+MessageHistoryPaginationState\s*\{[\s\S]*?hasMore:\s*boolean;[\s\S]*?nextCursor\?:\s*string;[\s\S]*?\}/u,
  'message history state must retain the server-issued opaque cursor as a string.',
);

assert.doesNotMatch(
  chatServiceSource,
  /readSeqCursorPageInfo|nextAfterSeq|resolveInitialMessageHistoryAfterSeq|Number\.parseInt\(pageInfo\.nextCursor/u,
  'PC message history must not parse, compare, or construct cursor sequence values.',
);

assert.match(
  chatServiceSource,
  /listMessages\(chatId,\s*\{\s*pageSize,\s*\}\)/u,
  'initial message history load must request the latest server page without constructing a cursor.',
);

assert.match(
  chatServiceSource,
  /listMessages\(chatId,\s*\{[\s\S]{0,120}?cursor:\s*state\.nextCursor,[\s\S]{0,120}?pageSize:/u,
  'loadMoreMessages must pass the server-issued opaque cursor through unchanged.',
);

assert.doesNotMatch(
  chatServiceSource,
  /doCatchUpConversationMessages|MAX_CATCH_UP_MESSAGE_PAGES/u,
  'PC reconnect must use durable realtime resume/replay instead of repurposing backward history pagination as a delta API.',
);

assert.match(
  chatServiceSource,
  /function\s+buildConversationName\([\s\S]*?return\s+normalizeConversationType\(entry\.conversationType\)\s*===\s*'group'\s*\?\s*'Group chat'\s*:\s*'Direct chat'/u,
  'conversation title fallback must be product text, not the technical conversation id.',
);

assert.doesNotMatch(
  chatServiceSource,
  /name:\s*viewState\.name\s*\?\?\s*chatId|name:\s*viewState\.name\s*\?\?\s*entry\.conversationId/u,
  'cached conversation projections must not fall back to displaying a technical id as the conversation name.',
);

assert.match(
  messageListSource,
  /const\s+MESSAGE_HISTORY_LOAD_COOLDOWN_MS\s*=\s*800/u,
  'MessageList must throttle manual history pagination requests to avoid bursty or accidental repeated loading.',
);

assert.doesNotMatch(
  messageListSource,
  /scrollTop\s*<\s*\d+[\s\S]{0,240}load(?:OlderMessages|NextMessagePage)|onScroll=\{[^}]*loadNextMessagePage/u,
  'MessageList must not bind scroll movement directly to history pagination; older history should load by explicit user intent.',
);

assert.match(
  messageListSource,
  /<button[\s\S]*?loadNextMessagePage\(\)[\s\S]*?\{t\('chat\.messageList\.loadMore'\)\}/u,
  'MessageList must expose explicit user-triggered history pagination when more pages exist.',
);

assert.match(
  messageListSource,
  /return\s+\[\s*\.\.\.nextMessages\.filter\([\s\S]{0,180}?\.\.\.previous/u,
  'older message history pages must be prepended before the currently displayed chronological page.',
);

assert.doesNotMatch(
  messageListSource,
  /return\s+\[\.\.\.previous,\s*\.\.\.nextMessages\.filter/u,
  'older message history pages must never be appended after newer messages.',
);

assert.match(
  messageListSource,
  /previousScrollHeight[\s\S]{0,1000}?scrollTop\s*\+=\s*element\.scrollHeight\s*-\s*previousScrollHeight/u,
  'prepending older messages must preserve the current viewport anchor using the scroll-height delta.',
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
  'ChatList item click must not also call markAsRead and duplicate read cursor mutations.',
);

assert.match(
  chatLayoutSource,
  /const\s+markSelectedChatAsRead\s*=\s*\(chat:\s*Chat\):\s*void\s*=>\s*\{[\s\S]*?chatService\.markAsRead\(chat\.id\)[\s\S]*?toast\(t\(["']chat\.list\.toast\.markReadFailed["']\),\s*["']error["']\)/u,
  'ChatLayout must own click/focus read cursor updates and surface localized failures.',
);

console.log('sdkwork im pc chat message pagination contract passed.');
