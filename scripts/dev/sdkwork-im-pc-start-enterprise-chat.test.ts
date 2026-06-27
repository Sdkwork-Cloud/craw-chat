import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkChatService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService';

type StartEnterpriseChatCall =
  | {
      body: Record<string, unknown>;
      method: 'conversations.bindDirectChat';
    }
  | {
      body: Record<string, unknown>;
      conversationId: string;
      method: 'conversations.updateProfile' | 'conversations.updatePreferences';
    };

const calls: StartEnterpriseChatCall[] = [];

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
        principalId: 'current-user',
        principalKind: 'user',
        tenantId: '100001',
        updatedAt: '2026-06-04T00:00:00.000Z',
      };
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkChatService(() => fakeClient);

  const chat = await service.startEnterpriseChat({
    id: 'enterprise-a',
    name: 'Enterprise A',
  });

  assert.deepEqual(
    calls[0],
    {
      method: 'conversations.bindDirectChat',
      body: {
        conversationId: 'pc-enterprise-current-user-enterprise-a',
        directChatId: 'pc-enterprise-dc-current-user-enterprise-a',
        leftActorId: 'current-user',
        leftActorKind: 'user',
        rightActorId: 'enterprise-a',
        rightActorKind: 'enterprise',
      },
    },
    'starting an enterprise chat must bind a real IM direct-chat conversation with the enterprise principal',
  );
  assert.deepEqual(
    calls.slice(1),
    [
      {
        body: {
          displayName: 'Enterprise A (Official)',
        },
        conversationId: 'pc-enterprise-current-user-enterprise-a',
        method: 'conversations.updateProfile',
      },
      {
        body: {
          isHidden: false,
        },
        conversationId: 'pc-enterprise-current-user-enterprise-a',
        method: 'conversations.updatePreferences',
      },
    ],
    'starting an enterprise chat must sync the display profile and unhide the real enterprise conversation',
  );
  assert.deepEqual(
    [chat.id, chat.name, chat.avatar, chat.type, chat.unreadCount],
    ['pc-enterprise-current-user-enterprise-a', 'Enterprise A (Official)', undefined, 'single', 0],
  );

  console.log('sdkwork-im-pc start enterprise chat contract passed');
}

void main();
