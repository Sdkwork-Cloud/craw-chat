import assert from 'node:assert/strict';
import { pathToFileURL } from 'node:url';
import type { Chat, Message } from '@sdkwork/im-pc-types';
import type * as NotificationServiceModule from '../packages/sdkwork-im-pc-chat/src/services/NotificationService.ts';

type NotificationServiceExports = typeof NotificationServiceModule;

async function loadNotificationServiceModule(): Promise<NotificationServiceExports> {
  const moduleUrl = pathToFileURL(
    './packages/sdkwork-im-pc-chat/src/services/NotificationService.ts',
  ).href;
  return await import(moduleUrl) as NotificationServiceExports;
}

const {
  buildIncomingMessageNotification,
  createSdkworkNotificationService,
  playMessageNotificationSound,
  shouldNotifyIncomingMessage,
} = await loadNotificationServiceModule();

const baseChat: Chat = {
  id: 'conversation.contract',
  avatar: 'https://example.test/avatar.png',
  name: 'Product Group',
  type: 'group',
  unreadCount: 0,
  updatedAt: Date.now(),
};

const baseMessage: Message = {
  id: 'message.contract',
  chatId: baseChat.id,
  content: 'The rollout checklist is ready.',
  senderId: 'user.alice',
  timestamp: Date.now(),
  type: 'text',
};

assert.equal(
  shouldNotifyIncomingMessage({
    activeConversationId: 'another.conversation',
    chat: baseChat,
    currentUserId: 'user.bob',
    message: baseMessage,
    settings: {
      notifyDesktop: true,
      notificationPreview: 'sender-and-preview',
      notificationWhenFocused: false,
    },
    windowIsFocused: false,
  }),
  true,
  'Incoming messages from another user in another conversation should notify when desktop notifications are enabled.',
);

assert.equal(
  shouldNotifyIncomingMessage({
    activeConversationId: 'another.conversation',
    chat: baseChat,
    currentUserId: 'user.alice',
    message: baseMessage,
    settings: {
      notifyDesktop: true,
      notificationPreview: 'sender-and-preview',
      notificationWhenFocused: false,
    },
    windowIsFocused: false,
  }),
  false,
  'Messages sent by the current user must not create notifications.',
);

assert.equal(
  shouldNotifyIncomingMessage({
    activeConversationId: 'another.conversation',
    chat: { ...baseChat, isMuted: true },
    currentUserId: 'user.bob',
    message: baseMessage,
    settings: {
      notifyDesktop: true,
      notificationPreview: 'sender-and-preview',
      notificationWhenFocused: true,
    },
    windowIsFocused: false,
  }),
  false,
  'Muted conversations should keep unread state but suppress popup notifications.',
);

assert.equal(
  shouldNotifyIncomingMessage({
    activeConversationId: baseChat.id,
    chat: baseChat,
    currentUserId: 'user.bob',
    message: baseMessage,
    settings: {
      notifyDesktop: true,
      notificationPreview: 'sender-and-preview',
      notificationWhenFocused: false,
    },
    windowIsFocused: true,
  }),
  false,
  'The active conversation should not show popups while the window is focused unless explicitly enabled.',
);

assert.equal(
  shouldNotifyIncomingMessage({
    activeConversationId: baseChat.id,
    chat: baseChat,
    currentUserId: 'user.bob',
    message: baseMessage,
    settings: {
      notifyDesktop: true,
      notificationPreview: 'sender-and-preview',
      notificationWhenFocused: true,
    },
    windowIsFocused: true,
  }),
  true,
  'Users can opt into focused-window notifications for the active conversation.',
);

assert.deepEqual(
  buildIncomingMessageNotification({
    chat: baseChat,
    message: baseMessage,
    previewMode: 'sender-and-preview',
  }),
  {
    body: 'The rollout checklist is ready.',
    conversationId: 'conversation.contract',
    icon: 'https://example.test/avatar.png',
    kind: 'message',
    messageId: 'message.contract',
    title: 'Product Group',
  },
  'Default previews should include conversation title and text preview.',
);

assert.equal(
  buildIncomingMessageNotification({
    chat: baseChat,
    message: { ...baseMessage, content: 'Sensitive payroll numbers' },
    previewMode: 'sender-only',
  }).body,
  'New message',
  'Sender-only previews should hide message content.',
);

assert.equal(
  buildIncomingMessageNotification({
    chat: baseChat,
    message: { ...baseMessage, type: 'image' },
    previewMode: 'sender-and-preview',
    texts: {
      hiddenBody: '收到一条新消息',
      messageTypeLabels: {
        image: '[图片]',
      },
      titleFallback: 'Sdkwork IM',
    },
  }).body,
  '[图片]',
  'Notification message type labels should support injected locale text.',
);

assert.equal(
  buildIncomingMessageNotification({
    chat: baseChat,
    message: { ...baseMessage, content: 'Sensitive payroll numbers' },
    previewMode: 'hidden',
  }).title,
  'Sdkwork IM',
  'Hidden previews should hide the conversation title.',
);

const delivered: string[] = [];
const service = createSdkworkNotificationService({
  deliver(notification) {
    delivered.push(`${notification.title}:${notification.body}`);
  },
  getActiveConversationId: () => 'another.conversation',
  getCurrentUserId: () => 'user.bob',
  getSettings: () => ({
    notifyDesktop: true,
    notificationPreview: 'sender-and-preview',
    notificationWhenFocused: false,
  }),
  isWindowFocused: () => false,
});

service.handleIncomingMessage(baseChat, baseMessage);
service.handleIncomingMessage(baseChat, baseMessage);

assert.deepEqual(
  delivered,
  ['Product Group:The rollout checklist is ready.'],
  'The notification service should deduplicate the same message event.',
);

assert.equal(
  typeof playMessageNotificationSound,
  'function',
  'The notification service must expose a guarded message notification sound helper.',
);

console.log('sdkwork im pc notification service contract passed.');
