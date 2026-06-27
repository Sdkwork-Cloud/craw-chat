import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import type { User } from '@sdkwork/im-pc-types';
import { createSdkworkChatService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService';

type StartDirectChatCall =
  | {
      body: Record<string, unknown>;
      method: 'conversations.bindDirectChat';
    }
  | {
      body: Record<string, unknown>;
      conversationId: string;
      method: 'conversations.updateProfile' | 'conversations.updatePreferences';
    };

const calls: StartDirectChatCall[] = [];

const fakeClient = {
  conversations: {
    async bindDirectChat(body: Record<string, unknown>) {
      calls.push({ method: 'conversations.bindDirectChat', body });
      return {
        conversationId: body.conversationId,
        directChatId: body.directChatId,
      };
    },
    async updateProfile(conversationId: string, body: Record<string, unknown>) {
      calls.push({ method: 'conversations.updateProfile', conversationId, body });
      return {
        avatarUrl: body.avatarUrl,
        conversationId,
        displayName: body.displayName,
        notice: '',
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
  const service = createSdkworkChatService(() => fakeClient);
  const user: User = {
    id: 'u_alice',
    name: 'Alice Chen',
    avatar: 'https://example.com/alice.png',
  };

  const chat = await service.startDirectChat(user);

  assert.deepEqual(
    calls[0],
    {
      method: 'conversations.bindDirectChat',
      body: {
        conversationId: 'pc-direct-current-user-u_alice',
        directChatId: 'pc-dc-current-user-u_alice',
        leftActorId: 'current-user',
        leftActorKind: 'user',
        rightActorId: 'u_alice',
        rightActorKind: 'user',
      },
    },
    'starting a direct chat from contacts must bind a real IM direct-chat conversation',
  );
  assert.deepEqual(
    calls.slice(1),
    [
      {
        method: 'conversations.updateProfile',
        conversationId: 'pc-direct-current-user-u_alice',
        body: {
          avatarUrl: 'https://example.com/alice.png',
          displayName: 'Alice Chen',
        },
      },
      {
        method: 'conversations.updatePreferences',
        conversationId: 'pc-direct-current-user-u_alice',
        body: {
          isHidden: false,
        },
      },
    ],
    'starting a direct chat must sync display profile and unhide the real conversation',
  );
  assert.deepEqual(
    [chat.id, chat.name, chat.avatar, chat.type, chat.unreadCount],
    ['pc-direct-current-user-u_alice', 'Alice Chen', 'https://example.com/alice.png', 'single', 0],
  );

  calls.length = 0;
  const existingContactChat = await service.startDirectChat({
    id: 'u_bob',
    name: 'Bob Li',
    avatar: 'https://example.com/bob.png',
    conversationId: 'c_direct_friend_bob',
    directChatId: 'dc_friend_bob',
  });

  assert.equal(
    calls.some((call) => call.method === 'conversations.bindDirectChat'),
    false,
    'starting a chat from an existing contact projection must not call the privileged direct-chat binding route',
  );
  assert.deepEqual(
    calls,
    [
      {
        method: 'conversations.updatePreferences',
        conversationId: 'c_direct_friend_bob',
        body: {
          isHidden: false,
        },
      },
    ],
    'starting a chat from an existing contact projection only needs to unhide the server-owned conversation',
  );
  assert.deepEqual(
    [
      existingContactChat.id,
      existingContactChat.name,
      existingContactChat.avatar,
      existingContactChat.type,
      existingContactChat.unreadCount,
    ],
    ['c_direct_friend_bob', 'Bob Li', 'https://example.com/bob.png', 'single', 0],
  );

  console.log('sdkwork-im-pc start direct chat contract passed');
}

void main();
