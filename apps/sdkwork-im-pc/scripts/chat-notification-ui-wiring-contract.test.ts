import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const zhLocale = JSON.parse(readFileSync(
  './packages/sdkwork-im-pc-chat/src/i18n/locales/zh-CN.json',
  'utf8',
)) as { chat?: { notification?: unknown } };

const enLocale = JSON.parse(readFileSync(
  './packages/sdkwork-im-pc-chat/src/i18n/locales/en-US.json',
  'utf8',
)) as { chat?: { notification?: unknown } };

const chatLayoutSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/pages/ChatLayout.tsx',
  'utf8',
);

const settingsModalSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/components/SettingsModal.tsx',
  'utf8',
);

const notificationCenterSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/components/NotificationCenter.tsx',
  'utf8',
);

assert.match(
  chatLayoutSource,
  /handlePotentialIncomingNotifications\(nextChats\)/,
  'ChatLayout must evaluate subscribed chat list updates for incoming message notifications.',
);

assert.match(
  chatLayoutSource,
  /sdkwork-im-pc:settings-changed/,
  'ChatLayout must listen for settings changes so notification policy changes take effect without reload.',
);

assert.match(
  chatLayoutSource,
  /sdkwork-im-pc:open-conversation/,
  'ChatLayout must handle notification clicks by opening the target conversation.',
);

assert.match(
  chatLayoutSource,
  /<NotificationCenter[\s\S]*onOpenConversation=/,
  'ChatLayout must render the bottom-right message notification center with a conversation click handler.',
);

assert.match(
  chatLayoutSource,
  /<NotificationCenter[\s\S]*onOpenCall=/,
  'ChatLayout must render the bottom-right notification center with a call click handler.',
);

assert.match(
  settingsModalSource,
  /notifySystem/,
  'Settings center must expose an operating-system/browser notification control.',
);

assert.match(
  settingsModalSource,
  /notificationPreview/,
  'Settings center must expose notification content preview controls.',
);

assert.match(
  settingsModalSource,
  /notificationWhenFocused/,
  'Settings center must expose focused-window notification behavior controls.',
);

assert.match(
  settingsModalSource,
  /chat\.notification\.settings\./,
  'Settings notification copy must use chat notification i18n keys.',
);

assert.match(
  notificationCenterSource,
  /useTranslation/,
  'NotificationCenter must localize message notification relative time and accessibility text.',
);

assert.match(
  notificationCenterSource,
  /chat\.notification\.center\./,
  'NotificationCenter must use chat notification center i18n keys.',
);

assert.match(
  notificationCenterSource,
  /sdkwork-im-pc:notify-app/,
  'NotificationCenter must consume the unified app notification event so messages and calls share the same right-bottom UI shell.',
);

assert.match(
  notificationCenterSource,
  /onOpenCall/,
  'NotificationCenter must expose a call click callback instead of routing call notifications as ordinary conversation clicks.',
);

assert.ok(
  zhLocale.chat?.notification && enLocale.chat?.notification,
  'zh-CN and en-US locales must define chat.notification resources.',
);

console.log('sdkwork im pc notification ui wiring contract passed.');
