import type { Message } from '@sdkwork/im-pc-types';

export type ChatHistorySearchTabId =
  | 'all'
  | 'messages'
  | 'media'
  | 'files'
  | 'links'
  | 'music'
  | 'voice'
  | 'apps';

export type ChatHistorySearchResultKind =
  | 'app'
  | 'file'
  | 'link'
  | 'media'
  | 'message'
  | 'music'
  | 'voice';

export interface ChatHistorySearchTab {
  id: ChatHistorySearchTabId;
  labelKey: string;
  messageTypes?: readonly Message['type'][];
  placeholderKey: string;
}

export interface ChatHistorySearchOptions {
  activeTab: ChatHistorySearchTabId;
  date?: string;
  query: string;
}

const MESSAGE_TYPES: readonly Message['type'][] = ['text'];
const MEDIA_TYPES: readonly Message['type'][] = ['image', 'video'];
const FILE_TYPES: readonly Message['type'][] = ['file'];
const LINK_TYPES: readonly Message['type'][] = ['link'];
const MUSIC_TYPES: readonly Message['type'][] = ['music'];
const VOICE_TYPES: readonly Message['type'][] = ['voice'];
const APP_TYPES: readonly Message['type'][] = ['applet', 'card'];
const NOTIFICATION_CONTENT_PATTERNS: readonly RegExp[] = [
  /^conversation\.(?:created|updated|member_joined|member_role_changed|member_removed|member_left|owner_transferred)\b/iu,
  /^group\.(?:notice|name|member|owner)\b/iu,
  /(?:group notice|group announcement|updated the group notice|changed the group name|joined the group|left the group|removed from the group|invited .* to the group)/iu,
  /(?:\u4fee\u6539|\u66f4\u65b0|\u53d1\u5e03|\u8bbe\u7f6e).{0,12}(?:\u7fa4\u516c\u544a|\u7fa4\u901a\u77e5)/u,
  /(?:\u4fee\u6539|\u66f4\u65b0|\u8bbe\u7f6e).{0,12}\u7fa4(?:\u540d\u79f0|\u540d)/u,
  /(?:\u52a0\u5165|\u9000\u51fa|\u79bb\u5f00)\u7fa4\u804a/u,
  /(?:\u79fb\u51fa|\u8e22\u51fa).{0,12}\u7fa4\u804a/u,
  /\u9080\u8bf7.{0,24}(?:\u52a0\u5165)?\u7fa4\u804a/u,
  /(?:\u8f6c\u8ba9|\u6210\u4e3a).{0,12}\u7fa4\u4e3b/u,
];

export const CHAT_HISTORY_SEARCH_TABS: readonly ChatHistorySearchTab[] = [
  {
    id: 'all',
    labelKey: 'chat.historySearch.tabs.all',
    placeholderKey: 'chat.historySearch.placeholder.all',
  },
  {
    id: 'messages',
    labelKey: 'chat.historySearch.tabs.messages',
    messageTypes: MESSAGE_TYPES,
    placeholderKey: 'chat.historySearch.placeholder.messages',
  },
  {
    id: 'media',
    labelKey: 'chat.historySearch.tabs.media',
    messageTypes: MEDIA_TYPES,
    placeholderKey: 'chat.historySearch.placeholder.media',
  },
  {
    id: 'files',
    labelKey: 'chat.historySearch.tabs.files',
    messageTypes: FILE_TYPES,
    placeholderKey: 'chat.historySearch.placeholder.files',
  },
  {
    id: 'links',
    labelKey: 'chat.historySearch.tabs.links',
    messageTypes: LINK_TYPES,
    placeholderKey: 'chat.historySearch.placeholder.links',
  },
  {
    id: 'music',
    labelKey: 'chat.historySearch.tabs.music',
    messageTypes: MUSIC_TYPES,
    placeholderKey: 'chat.historySearch.placeholder.music',
  },
  {
    id: 'voice',
    labelKey: 'chat.historySearch.tabs.voice',
    messageTypes: VOICE_TYPES,
    placeholderKey: 'chat.historySearch.placeholder.voice',
  },
  {
    id: 'apps',
    labelKey: 'chat.historySearch.tabs.apps',
    messageTypes: APP_TYPES,
    placeholderKey: 'chat.historySearch.placeholder.apps',
  },
] as const;

const TAB_BY_ID = new Map<ChatHistorySearchTabId, ChatHistorySearchTab>(
  CHAT_HISTORY_SEARCH_TABS.map((tab) => [tab.id, tab]),
);

export function getChatHistorySearchPlaceholderKey(tabId: ChatHistorySearchTabId): string {
  return TAB_BY_ID.get(tabId)?.placeholderKey ?? 'chat.historySearch.placeholder.all';
}

export function getChatHistoryMessageResultKind(message: Message): ChatHistorySearchResultKind {
  switch (message.type) {
    case 'image':
    case 'video':
      return 'media';
    case 'file':
      return 'file';
    case 'link':
      return 'link';
    case 'music':
      return 'music';
    case 'voice':
      return 'voice';
    case 'applet':
    case 'card':
      return 'app';
    case 'text':
    default:
      return 'message';
  }
}

export function filterChatHistoryMessages(
  messages: readonly Message[],
  options: ChatHistorySearchOptions,
): Message[] {
  const normalizedQuery = normalizeSearchText(options.query);

  return messages.filter((message) => (
    !isChatHistoryNotificationMessage(message)
    && matchesTab(message, options.activeTab)
    && matchesDate(message, options.date)
    && matchesQuery(message, normalizedQuery)
  ));
}

export function isChatHistoryNotificationMessage(message: Message): boolean {
  if (message.type === 'system' || message.type === 'video_call') {
    return true;
  }
  if (message.senderId.trim().toLowerCase() === 'system') {
    return true;
  }

  const normalizedContent = message.content.trim();
  return NOTIFICATION_CONTENT_PATTERNS.some((pattern) => pattern.test(normalizedContent));
}

function matchesTab(message: Message, activeTab: ChatHistorySearchTabId): boolean {
  const tab = TAB_BY_ID.get(activeTab);
  if (!tab?.messageTypes) {
    return true;
  }
  return tab.messageTypes.includes(message.type);
}

function matchesDate(message: Message, date: string | undefined): boolean {
  if (!date) {
    return true;
  }
  return formatMessageDate(message.timestamp) === date;
}

function matchesQuery(message: Message, normalizedQuery: string): boolean {
  if (!normalizedQuery) {
    return true;
  }
  return collectSearchableText(message).some((value) => normalizeSearchText(value).includes(normalizedQuery));
}

function collectSearchableText(message: Message): string[] {
  return [
    message.content,
    message.fileName,
    message.fileSize,
    message.desc,
    String(message.duration ?? ''),
  ].filter((value): value is string => Boolean(value));
}

function normalizeSearchText(value: string): string {
  return value.trim().toLowerCase();
}

function formatMessageDate(timestamp: number): string {
  const date = new Date(timestamp);
  if (!Number.isFinite(date.getTime())) {
    return '';
  }
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
}
