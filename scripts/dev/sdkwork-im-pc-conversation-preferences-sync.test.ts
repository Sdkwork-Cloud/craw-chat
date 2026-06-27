import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkChatService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService';

type PreferenceCall = {
  conversationId: string;
  body?: {
    isPinned?: boolean;
    isMuted?: boolean;
    isMarkedUnread?: boolean;
    isHidden?: boolean;
  };
};

const retrieveCalls: PreferenceCall[] = [];
const updateCalls: PreferenceCall[] = [];
const leaveCalls: string[] = [];
const readCursorCalls: Array<{ conversationId: string; body: { readSeq: number; lastReadMessageId?: string } }> = [];

const fakeClient = {
  chat: {
    inbox: {
      async retrieve() {
        return {
          items: [
            {
              conversationId: 'chat-1',
              conversationType: 'group',
              unreadCount: 3,
              lastMessageSeq: 7,
              lastActivityAt: '2026-06-04T11:00:00.000Z',
            },
            {
              conversationId: 'chat-hidden',
              conversationType: 'group',
              unreadCount: 1,
              lastMessageSeq: 3,
              lastActivityAt: '2026-06-04T10:00:00.000Z',
            },
          ],
          hasMore: false,
        };
      },
    },
  },
  conversations: {
    async getPreferences(conversationId: string) {
      retrieveCalls.push({ conversationId });
      return {
        tenantId: '100001',
        conversationId,
        principalKind: 'user',
        principalId: 'u_owner',
        isPinned: true,
        isMuted: true,
        isMarkedUnread: true,
        isHidden: conversationId === 'chat-hidden',
        updatedAt: '2026-06-04T11:01:00.000Z',
      };
    },
    async updatePreferences(
      conversationId: string,
      body: {
        isPinned?: boolean;
        isMuted?: boolean;
        isMarkedUnread?: boolean;
        isHidden?: boolean;
      },
    ) {
      updateCalls.push({ conversationId, body });
      return {
        tenantId: '100001',
        conversationId,
        principalKind: 'user',
        principalId: 'u_owner',
        isPinned: body.isPinned ?? false,
        isMuted: body.isMuted ?? false,
        isMarkedUnread: body.isMarkedUnread ?? false,
        isHidden: body.isHidden ?? false,
        updatedAt: '2026-06-04T11:02:00.000Z',
      };
    },
    async leave(conversationId: string) {
      leaveCalls.push(conversationId);
      return {
        conversationId,
        memberId: 'member-1',
        principalId: 'u_owner',
        principalKind: 'user',
        role: 'member',
        state: 'left',
        updatedAt: '2026-06-04T11:02:30.000Z',
      };
    },
    async updateReadCursor(
      conversationId: string,
      body: { readSeq: number; lastReadMessageId?: string },
    ) {
      readCursorCalls.push({ conversationId, body });
      return {
        tenantId: '100001',
        conversationId,
        memberId: 'member-1',
        principalId: 'u_owner',
        principalKind: 'user',
        readSeq: body.readSeq,
        lastReadMessageId: body.lastReadMessageId,
        unreadCount: 0,
        updatedAt: '2026-06-04T11:03:00.000Z',
      };
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkChatService(() => fakeClient);

  const chats = await service.getChats();
  assert.deepEqual(
    retrieveCalls,
    [{ conversationId: 'chat-1' }, { conversationId: 'chat-hidden' }],
    'chat list sync must restore per-user conversation preferences through the IM SDK',
  );
  assert.equal(chats[0]?.isPinned, true);
  assert.equal(chats[0]?.isMuted, true);
  assert.equal(chats[0]?.isMarkedUnread, true);
  assert.deepEqual(
    chats.map((chat) => chat.id),
    ['chat-1'],
    'chat list sync must hide conversations using SDK-backed ConversationPreferencesView.isHidden',
  );

  await service.pinChat('chat-1', false);
  await service.muteChat('chat-1', false);
  await service.markAsUnread('chat-1');
  await service.markAsRead('chat-1');
  await service.deleteChat('chat-1');

  assert.deepEqual(
    updateCalls,
    [
      { conversationId: 'chat-1', body: { isPinned: false } },
      { conversationId: 'chat-1', body: { isMuted: false } },
      { conversationId: 'chat-1', body: { isMarkedUnread: true } },
      { conversationId: 'chat-1', body: { isMarkedUnread: false } },
      { conversationId: 'chat-1', body: { isHidden: true } },
    ],
    'pinChat, muteChat, mark-unread, and delete-chat hidden state must persist through the standard IM SDK conversation preferences API',
  );
  assert.deepEqual(
    leaveCalls,
    [],
    'deleteChat must hide the conversation list item through preferences instead of leaving the conversation',
  );
  assert.deepEqual(
    readCursorCalls,
    [{ conversationId: 'chat-1', body: { readSeq: 7 } }],
    'markAsRead must continue to advance the read cursor through the standard IM SDK read protocol',
  );

  console.log('sdkwork-im-pc conversation preferences sync contract passed');
}

void main();
