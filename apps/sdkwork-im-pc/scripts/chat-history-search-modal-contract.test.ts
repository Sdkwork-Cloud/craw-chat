import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import type { Message } from '../packages/sdkwork-im-pc-types/src/message';

const appRoot = path.resolve(import.meta.dirname, '..');

function readText(...segments: string[]): string {
  return fs.readFileSync(path.join(appRoot, ...segments), 'utf8');
}

function readJson(...segments: string[]): Record<string, unknown> {
  return JSON.parse(readText(...segments)) as Record<string, unknown>;
}

const searchModelPath = path.join(
  appRoot,
  'packages',
  'sdkwork-im-pc-chat',
  'src',
  'services',
  'ChatHistorySearchModel.ts',
);
const displayModelPath = path.join(
  appRoot,
  'packages',
  'sdkwork-im-pc-chat',
  'src',
  'services',
  'ChatHistoryMessageDisplayModel.ts',
);

assert.equal(
  fs.existsSync(searchModelPath),
  true,
  'Chat history search modal must define a reusable search model so tab filtering and UI rendering share one contract.',
);
assert.equal(
  fs.existsSync(displayModelPath),
  true,
  'Chat history search modal must define a reusable display model so sender nicknames never fall back to raw internal ids.',
);

const searchModel = await import('../packages/sdkwork-im-pc-chat/src/services/ChatHistorySearchModel') as typeof import('../packages/sdkwork-im-pc-chat/src/services/ChatHistorySearchModel');
const displayModel = await import('../packages/sdkwork-im-pc-chat/src/services/ChatHistoryMessageDisplayModel') as typeof import('../packages/sdkwork-im-pc-chat/src/services/ChatHistoryMessageDisplayModel');

assert.deepEqual(
  searchModel.CHAT_HISTORY_SEARCH_TABS.map((tab) => tab.id),
  ['all', 'messages', 'media', 'files', 'links', 'music', 'voice', 'apps'],
  'Chat history search tabs must cover unified searchable message, media, file, link, music, voice, and mini-app/card searches without notification-only call events.',
);

assert.equal(
  searchModel.getChatHistorySearchPlaceholderKey('files'),
  'chat.historySearch.placeholder.files',
  'Search input placeholder must switch to the active tab target.',
);

const messages: Message[] = [
  {
    chatId: 'chat-1',
    content: 'Roadmap review tomorrow',
    id: 'message-text',
    senderId: 'me',
    timestamp: 1,
    type: 'text',
  },
  {
    chatId: 'chat-1',
    content: 'https://sdkwork.com/docs/chat',
    desc: 'Sdkwork IM docs',
    fileName: 'Chat integration guide',
    id: 'message-link',
    senderId: 'user-1',
    timestamp: 2,
    type: 'link',
  },
  {
    chatId: 'chat-1',
    content: 'https://cdn.example.com/image.png',
    fileName: 'design-preview.png',
    id: 'message-image',
    senderId: 'user-1',
    timestamp: 3,
    type: 'image',
  },
  {
    chatId: 'chat-1',
    content: 'https://cdn.example.com/quarterly.pdf',
    fileName: 'Quarterly report.pdf',
    fileSize: '2.4 MB',
    id: 'message-file',
    senderId: 'user-2',
    timestamp: 4,
    type: 'file',
  },
  {
    chatId: 'chat-1',
    content: 'voice://local/1',
    duration: 18,
    id: 'message-voice',
    senderId: 'user-2',
    timestamp: 5,
    type: 'voice',
  },
  {
    chatId: 'chat-1',
    content: 'rtc-call',
    desc: 'accepted',
    duration: 48,
    id: 'message-call',
    senderId: 'user-3',
    timestamp: 6,
    type: 'video_call',
  },
  {
    chatId: 'chat-1',
    content: 'conversation.member_joined',
    id: 'message-system-event',
    senderId: 'system',
    timestamp: 7,
    type: 'system',
  },
  {
    chatId: 'chat-1',
    content: 'Owner updated the group notice',
    id: 'message-group-notice',
    senderId: 'system',
    timestamp: 8,
    type: 'text',
  },
  {
    chatId: 'chat-1',
    content: 'conversation.member_removed',
    id: 'message-member-notification',
    senderId: 'user-4',
    timestamp: 9,
    type: 'text',
  },
  {
    chatId: 'chat-1',
    content: '\u5f20\u4e09\u4fee\u6539\u7fa4\u516c\u544a',
    id: 'message-localized-group-notification',
    senderId: 'user-5',
    timestamp: 10,
    type: 'text',
  },
  {
    appIcon: 'https://cdn.example.com/app.png',
    chatId: 'chat-1',
    content: 'sdkwork://mini-app/approval',
    desc: 'Approval workflow',
    fileName: 'Approval mini app',
    id: 'message-applet',
    senderId: 'user-3',
    timestamp: 11,
    type: 'applet',
  },
];

assert.deepEqual(
  searchModel.filterChatHistoryMessages(messages, { activeTab: 'messages', query: 'roadmap' }).map((message) => message.id),
  ['message-text'],
  'Messages tab must search user-authored text message content without mixing file, media, link, system, or notification rows.',
);

assert.deepEqual(
  searchModel.filterChatHistoryMessages(messages, { activeTab: 'links', query: 'sdkwork' }).map((message) => message.id),
  ['message-link'],
  'Links tab must search link title, description, and URL fields.',
);

assert.deepEqual(
  searchModel.filterChatHistoryMessages(messages, { activeTab: 'media', query: 'design' }).map((message) => message.id),
  ['message-image'],
  'Media tab must search image/video attachment names and content.',
);

assert.deepEqual(
  searchModel.filterChatHistoryMessages(messages, { activeTab: 'files', query: 'quarterly' }).map((message) => message.id),
  ['message-file'],
  'Files tab must search file names and file metadata.',
);

assert.deepEqual(
  searchModel.filterChatHistoryMessages(messages, { activeTab: 'voice', query: '' }).map((message) => message.id),
  ['message-voice'],
  'Voice tab must list voice-message rows even before a keyword is entered.',
);

assert.deepEqual(
  searchModel.filterChatHistoryMessages(messages, { activeTab: 'apps', query: 'approval' }).map((message) => message.id),
  ['message-applet'],
  'Mini-app/card tab must search applet and contact-card metadata.',
);

assert.equal(
  typeof searchModel.isChatHistoryNotificationMessage,
  'function',
  'Chat history search model must expose notification classification for unsupported event rows.',
);

assert.equal(
  searchModel.isChatHistoryNotificationMessage(messages.find((message) => message.id === 'message-call') as Message),
  true,
  'Call event messages must be classified as notification/event rows and excluded from search.',
);

assert.equal(
  searchModel.isChatHistoryNotificationMessage(messages.find((message) => message.id === 'message-system-event') as Message),
  true,
  'System messages must be classified as notification rows and excluded from search.',
);

assert.equal(
  searchModel.isChatHistoryNotificationMessage(messages.find((message) => message.id === 'message-group-notice') as Message),
  true,
  'Group notice messages from the system sender must be classified as notification rows and excluded from search.',
);

assert.equal(
  searchModel.isChatHistoryNotificationMessage(messages.find((message) => message.id === 'message-member-notification') as Message),
  true,
  'Conversation member lifecycle event rows must be classified as notification rows even when projected as text.',
);

assert.equal(
  searchModel.isChatHistoryNotificationMessage(messages.find((message) => message.id === 'message-localized-group-notification') as Message),
  true,
  'Localized group notice lifecycle rows must be classified as notification rows even when projected as text from a member sender.',
);

assert.deepEqual(
  searchModel.filterChatHistoryMessages(messages, { activeTab: 'all', query: '' }).map((message) => message.id),
  ['message-text', 'message-link', 'message-image', 'message-file', 'message-voice', 'message-applet'],
  'All tab must exclude system notifications, call events, and group lifecycle notices before keyword filtering.',
);

assert.deepEqual(
  searchModel.filterChatHistoryMessages(messages, { activeTab: 'all', query: 'user-3' }).map((message) => message.id),
  [],
  'Search must not match raw senderId values because the modal should not expose internal user ids as searchable content.',
);

const senderProfileIndex = displayModel.createChatHistorySenderProfileIndex(
  [
    {
      avatar: 'alice.png',
      chatId: 'chat-alice',
      id: 'user-alice',
      name: 'Alice Chen',
    },
    {
      avatar: 'bob.png',
      chatId: 'chat-bob',
      id: 'user-bob',
      name: 'Bob Lee',
    },
  ],
  {
    'agent-chat': {
      avatar: 'agent.png',
      id: 'agent-chat',
      name: 'Assistant',
    },
  },
);

assert.deepEqual(
  displayModel.resolveChatHistoryMessageSender({
    currentUser: {
      avatar: 'alice.png',
      chatId: 'chat-alice',
      id: 'user-alice',
      name: 'Alice Chen',
    },
    fallbackMemberName: 'Member',
    message: { senderId: 'chat-alice' },
    senderProfiles: senderProfileIndex,
  }),
  {
    avatar: 'alice.png',
    isCurrentUser: true,
    name: 'Alice Chen',
  },
  'History results must show the current user nickname instead of a generic Me label or raw sender id.',
);

assert.deepEqual(
  displayModel.resolveChatHistoryMessageSender({
    currentUser: {
      avatar: 'alice.png',
      chatId: 'chat-alice',
      id: 'user-alice',
      name: 'Alice Chen',
    },
    fallbackMemberName: 'Member',
    message: { senderId: 'chat-bob' },
    senderProfiles: senderProfileIndex,
  }),
  {
    avatar: 'bob.png',
    isCurrentUser: false,
    name: 'Bob Lee',
  },
  'History results must resolve member nicknames from chatId/profile indexes.',
);

assert.deepEqual(
  displayModel.resolveChatHistoryMessageSender({
    chat: {
      avatar: 'direct.png',
      id: 'direct-chat',
      name: 'Design Partner',
      type: 'single',
    },
    currentUser: {
      id: 'user-alice',
      name: 'Alice Chen',
    },
    fallbackMemberName: 'Member',
    message: { senderId: 'unknown-peer' },
    senderProfiles: senderProfileIndex,
  }),
  {
    avatar: 'direct.png',
    isCurrentUser: false,
    name: 'Design Partner',
  },
  'Direct-chat history results must fall back to the conversation nickname for the peer instead of exposing sender ids.',
);

assert.deepEqual(
  displayModel.resolveChatHistoryMessageSender({
    chat: {
      id: 'group-chat',
      name: 'Product Group',
      type: 'group',
    },
    currentUser: {
      id: 'user-alice',
      name: 'Alice Chen',
    },
    fallbackMemberName: 'Member',
    message: { senderId: 'user-raw-99' },
    senderProfiles: senderProfileIndex,
  }),
  {
    avatar: undefined,
    isCurrentUser: false,
    name: 'Member',
  },
  'Group history results must use a localized member fallback rather than exposing unresolved raw sender ids.',
);

const chatHistoryModalSource = readText(
  'packages',
  'sdkwork-im-pc-chat',
  'src',
  'components',
  'ChatHistoryModal.tsx',
);
const chatLayoutSource = readText(
  'packages',
  'sdkwork-im-pc-chat',
  'src',
  'pages',
  'ChatLayout.tsx',
);

for (const marker of [
  'Avatar',
  'CHAT_HISTORY_SEARCH_TABS',
  'resolveChatHistoryMessageSender',
  'createChatHistorySenderProfileIndex',
  'contactService.getContacts',
  'filterChatHistoryMessages',
  'getChatHistorySearchPlaceholderKey',
  'renderHistoryMessagePlainText',
  'text-[14px] font-medium leading-6 text-gray-100',
  'role="list"',
  'role="listitem"',
  'role="tablist"',
  'aria-selected',
  'chat.historySearch.',
  'shadow-[0_24px_80px_rgba(0,0,0,0.46)]',
  'bg-[#242529]',
  'h-7 shrink-0',
]) {
  assert.match(
    chatHistoryModalSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
    `ChatHistoryModal must include unified search marker ${marker}.`,
  );
}

assert.doesNotMatch(
  chatHistoryModalSource,
  /\bborder(?:-[A-Za-z0-9:[\]./%]+)?\b/u,
  'ChatHistoryModal must use a borderless visual design; hierarchy should come from background layers, shadow, spacing, and focus rings instead of border utilities.',
);

assert.doesNotMatch(
  chatHistoryModalSource,
  /video_call|chat\.historySearch\.type\.call|chat\.historySearch\.preview\.call/u,
  'ChatHistoryModal must not render call event rows because notification/event messages are not searchable.',
);

assert.doesNotMatch(
  chatHistoryModalSource,
  /message\.senderId === 'me' \? t\('chat\.historySearch\.sender\.me'\) : t\('chat\.historySearch\.sender\.other'\)/u,
  'ChatHistoryModal must resolve real sender nicknames instead of showing generic sender labels for every result row.',
);

assert.doesNotMatch(
  chatHistoryModalSource,
  /justify-end|flex-row-reverse|items-end/u,
  'ChatHistoryModal search results must remain a single list layout; search rows must not split messages into left/right conversation alignment.',
);

assert.doesNotMatch(
  chatHistoryModalSource,
  /TextMessageItem|ImageMessageItem|VideoMessageItem|VoiceMessageItem|LinkMessageItem|AppletMessageItem|CardMessageItem|FileMessageItem|MusicMessageItem/u,
  'ChatHistoryModal search result rows must render plain text previews instead of chat bubble message components.',
);

assert.match(
  chatLayoutSource,
  /import\s+\{\s*ChatHistoryModal\s*\}\s+from\s+['"]\.\.\/components\/ChatHistoryModal['"]/u,
  'ChatLayout header and right-panel search paths must use the unified ChatHistoryModal.',
);
assert.match(
  chatLayoutSource,
  /<ChatHistoryModal[\s\S]*isOpen=\{activeModal === "search" && Boolean\(localizedActiveChat\)\}/u,
  'ChatLayout must render the unified history search modal when the header or right-panel search action opens activeModal=search.',
);
assert.doesNotMatch(
  chatLayoutSource,
  /activeModal === "search" && \(\s*<div>\s*<input/u,
  'ChatLayout must not keep the legacy one-input search modal for activeModal=search.',
);

for (const localeName of ['zh-CN.json', 'en-US.json']) {
  const locale = readJson(
    'packages',
    'sdkwork-im-pc-chat',
    'src',
    'i18n',
    'locales',
    localeName,
  ) as {
    chat?: {
      historySearch?: {
        placeholder?: Record<string, string>;
        tabs?: Record<string, string>;
      };
    };
  };

  for (const tabId of searchModel.CHAT_HISTORY_SEARCH_TABS.map((tab) => tab.id)) {
    assert.equal(
      typeof locale.chat?.historySearch?.tabs?.[tabId],
      'string',
      `${localeName} must define chat.historySearch.tabs.${tabId}.`,
    );
    assert.equal(
      typeof locale.chat?.historySearch?.placeholder?.[tabId],
      'string',
      `${localeName} must define chat.historySearch.placeholder.${tabId}.`,
    );
  }
}

console.log('sdkwork im history search modal contract passed.');
