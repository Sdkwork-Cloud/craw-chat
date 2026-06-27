import assert from 'node:assert/strict';
import type { ConversationMember, ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkGroupService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/GroupService';

type GroupCreateCall =
  | {
      body: Record<string, unknown>;
      method: 'conversations.create';
    }
  | {
      body: Record<string, unknown>;
      conversationId: string;
      method: 'conversations.addMember' | 'conversations.updatePreferences' | 'conversations.updateProfile';
    };

const calls: GroupCreateCall[] = [];

function createMember(conversationId: string, principalId: string): ConversationMember {
  return {
    attributes: {},
    conversationId,
    joinedAt: '2026-06-04T00:00:00.000Z',
    memberId: `member-${principalId}`,
    principalId,
    principalKind: 'user',
    role: principalId === 'current-user' ? 'owner' : 'member',
    state: 'joined',
    tenantId: '100001',
  };
}

const fakeClient = {
  conversations: {
    async create(body: Record<string, unknown>) {
      calls.push({ method: 'conversations.create', body });
      return {
        conversationId: String(body.conversationId),
        eventId: 'evt-group-created',
      };
    },
    async addMember(conversationId: string, body: Record<string, unknown>) {
      calls.push({ method: 'conversations.addMember', conversationId, body });
      return createMember(conversationId, String(body.principalId));
    },
    async updateProfile(conversationId: string, body: Record<string, unknown>) {
      calls.push({ method: 'conversations.updateProfile', conversationId, body });
      return {
        avatarUrl: String(body.avatarUrl ?? ''),
        conversationId,
        displayName: String(body.displayName ?? ''),
        notice: String(body.notice ?? ''),
        tenantId: '100001',
        updatedAt: '2026-06-04T00:00:00.000Z',
      };
    },
    async updatePreferences(conversationId: string, body: Record<string, unknown>) {
      calls.push({ method: 'conversations.updatePreferences', conversationId, body });
      return {
        conversationId,
        isHidden: Boolean(body.isHidden),
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
  const service = createSdkworkGroupService(() => fakeClient);

  const group = await service.createGroup('  Project Room  ', [
    'u_bob',
    'u_carol',
    'u_bob',
    'current-user',
    ' ',
  ]);

  const createCall = calls[0];
  assert.equal(createCall?.method, 'conversations.create');
  assert.match(
    String(createCall.body.conversationId),
    /^pc-group-[0-9a-f-]{36}$/u,
    'group creation must use a standard client request id, not a mock Date.now/Math.random id',
  );
  assert.deepEqual(createCall.body, {
    conversationId: group.id,
    conversationType: 'group',
  });

  assert.deepEqual(
    calls.slice(1),
    [
      {
        method: 'conversations.updateProfile',
        conversationId: group.id,
        body: {
          avatarUrl: group.avatar,
          displayName: 'Project Room',
        },
      },
      {
        method: 'conversations.updatePreferences',
        conversationId: group.id,
        body: {
          isHidden: false,
        },
      },
      {
        method: 'conversations.addMember',
        conversationId: group.id,
        body: {
          principalId: 'u_bob',
          principalKind: 'user',
          role: 'member',
        },
      },
      {
        method: 'conversations.addMember',
        conversationId: group.id,
        body: {
          principalId: 'u_carol',
          principalKind: 'user',
          role: 'member',
        },
      },
    ],
    'group creation must create the conversation, persist profile, unhide, and then invite members through the IM SDK so invitees refresh into a named group',
  );
  assert.deepEqual(group.members, ['current-user', 'u_bob', 'u_carol']);
  assert.equal(group.memberCount, 3);
  assert.equal(group.activeCount, 3);

  calls.length = 0;
  const unnamedGroup = await service.createGroup('  ', ['u_bob']);
  assert.equal(
    unnamedGroup.name,
    '群聊(2�?',
    'empty group names from the create-group modal must persist a readable standard default name',
  );
  assert.deepEqual(
    calls.find((call) => call.method === 'conversations.updateProfile'),
    {
      method: 'conversations.updateProfile',
      conversationId: unnamedGroup.id,
      body: {
        avatarUrl: unnamedGroup.avatar,
        displayName: '群聊(2�?',
      },
    },
    'empty group names must update the real backend conversation profile with the same readable default name',
  );

  console.log('sdkwork-im-pc group create real logic contract passed');
}

void main();
