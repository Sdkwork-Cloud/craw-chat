import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkChatService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService';
import type { SdkworkChatSession } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/session';

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
const authenticatedSession: SdkworkChatSession = {
  accessToken: 'test-access-token',
  authToken: 'test-auth-token',
  context: {
    appId: 'sdkwork-im-pc',
    authLevel: 'password',
    dataScope: [],
    deploymentMode: 'saas',
    environment: 'dev',
    permissionScope: [],
    sessionId: 'session-1',
    tenantId: '100001',
    userId: 'session-user-1',
  },
  sessionId: 'session-1',
  user: {
    id: 'cached-local-user',
    userId: 'session-user-1',
  },
};

const fakeClient = {
  conversations: {
    async bindDirectChat(body: Record<string, unknown>) {
      calls.push({ method: 'conversations.bindDirectChat', body });
      const conversationId = `pc-enterprise-${body.leftActorId}-${body.rightActorId}`;
      return {
        conversationId,
        createdAt: '2026-06-04T00:00:00.000Z',
        kind: 'direct',
        tenantId: '100001',
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
  const service = createSdkworkChatService({
    getClient: () => fakeClient,
    getSession: () => authenticatedSession,
  });

  const chat = await service.startEnterpriseChat({
    id: 'enterprise-a',
    name: 'Enterprise A',
  });

  assert.deepEqual(
    calls[0],
    {
      method: 'conversations.bindDirectChat',
      body: {
        leftActorId: 'session-user-1',
        leftActorKind: 'user',
        rightActorId: 'enterprise-a',
        rightActorKind: 'enterprise',
      },
    },
    'starting an enterprise chat must bind a real IM direct-chat conversation with the enterprise principal and use the server conversation id',
  );
  assert.deepEqual(
    calls.slice(1),
    [
      {
        body: {
          displayName: 'Enterprise A (Official)',
        },
        conversationId: 'pc-enterprise-session-user-1-enterprise-a',
        method: 'conversations.updateProfile',
      },
      {
        body: {
          isHidden: false,
        },
        conversationId: 'pc-enterprise-session-user-1-enterprise-a',
        method: 'conversations.updatePreferences',
      },
    ],
    'starting an enterprise chat must sync the display profile and unhide the real enterprise conversation',
  );
  assert.deepEqual(
    [chat.id, chat.name, chat.avatar, chat.type, chat.unreadCount],
    ['pc-enterprise-session-user-1-enterprise-a', 'Enterprise A (Official)', undefined, 'single', 0],
  );

  console.log('sdkwork-im-pc start enterprise chat contract passed');
}

void main();
