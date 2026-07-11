import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import type { Chat } from '@sdkwork/im-pc-types';
import { createSdkworkChatService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService';

type CreateChatCall =
  | {
      body: Record<string, unknown>;
      method: 'conversations.create';
    }
  | {
      body: Record<string, unknown>;
      conversationId: string;
      method: 'conversations.updatePreferences' | 'conversations.updateProfile';
    };

const calls: CreateChatCall[] = [];

const fakeClient = {
  conversations: {
    async create(body: Record<string, unknown>) {
      calls.push({ method: 'conversations.create', body });
      return {
        conversationId: 'g_server_created_chat_1',
        eventId: 'evt-create-chat-1',
      };
    },
    async updateProfile(conversationId: string, body: Record<string, unknown>) {
      calls.push({ method: 'conversations.updateProfile', conversationId, body });
      return {
        avatarUrl: body.avatarUrl,
        conversationId,
        displayName: body.displayName,
        notice: body.notice,
        tenantId: '100001',
        updatedAt: '2026-06-04T00:00:00.000Z',
      };
    },
    async updatePreferences(conversationId: string, body: Record<string, unknown>) {
      calls.push({ method: 'conversations.updatePreferences', conversationId, body });
      return {
        conversationId,
        isHidden: body.isHidden === true,
        isMarkedUnread: false,
        isMuted: false,
        isPinned: false,
        tenantId: '100001',
        updatedAt: '2026-06-04T00:00:00.000Z',
      };
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkChatService({
    getClient: () => fakeClient,
  });

  const group = await service.createChat({
    id: 'pc-group-local-id',
    name: 'Legacy Create Group',
    avatar: 'https://example.com/group.png',
    type: 'group',
    unreadCount: 0,
    updatedAt: Date.now(),
    memberCount: 2,
  });

  const createCall = calls[0];
  assert.equal(createCall?.method, 'conversations.create');
  assert.equal(
    'conversationId' in createCall.body,
    false,
    'ChatService.createChat must not send client-local chat.id as backend conversationId',
  );
  assert.match(
    String(createCall.body.clientRequestKey),
    /^pc-create-chat-[0-9a-f-]{36}$/u,
    'ChatService.createChat must use a client request key only as an idempotency seed',
  );
  assert.deepEqual({
    conversationType: createCall.body.conversationType,
    groupName: createCall.body.groupName,
  }, {
    conversationType: 'group',
    groupName: 'Legacy Create Group',
  });
  assert.deepEqual(
    calls.slice(1),
    [
      {
        method: 'conversations.updateProfile',
        conversationId: 'g_server_created_chat_1',
        body: {
          avatarUrl: 'https://example.com/group.png',
          displayName: 'Legacy Create Group',
        },
      },
      {
        method: 'conversations.updatePreferences',
        conversationId: 'g_server_created_chat_1',
        body: {
          isHidden: false,
        },
      },
    ],
    'ChatService.createChat must apply profile and visibility to the server-created canonical conversation',
  );
  assert.equal(group.id, 'g_server_created_chat_1');

  await assert.rejects(
    () => service.createChat({
      id: 'pc-direct-local-id',
      name: 'Direct Legacy',
      type: 'single',
      unreadCount: 0,
      updatedAt: Date.now(),
    } as Chat),
    /startDirectChat/u,
    'single conversations must be created through direct-chat binding, not generic createChat',
  );

  console.log('sdkwork-im-pc createChat boundary contract passed');
}

void main();
