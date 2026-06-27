import assert from 'node:assert/strict';
import type { ConversationMember, ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkGroupService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/GroupService';

const calls: Array<{
  conversationId?: string;
  method: string;
  params?: Record<string, unknown>;
}> = [];
let scenario: 'default' | 'many-projected-groups' = 'default';
let activeMemberLookups = 0;
let maxActiveMemberLookups = 0;

async function delay(): Promise<void> {
  await new Promise((resolve) => {
    setTimeout(resolve, 5);
  });
}

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
  chat: {
    inbox: {
      async retrieve(params?: Record<string, unknown>) {
        calls.push({ method: 'chat.inbox.retrieve', params });
        if (scenario === 'many-projected-groups') {
          return {
            hasMore: false,
            items: Array.from({ length: 9 }, (_, index) => ({
              avatarUrl: `https://cdn.example.test/group-perf-${index}.png`,
              conversationId: `group-perf-${index}`,
              conversationType: 'group',
              displayName: `Projected Group ${index}`,
              lastActivityAt: `2026-06-04T10:00:0${index}.000Z`,
              lastMessageSeq: 2 + index,
              unreadCount: 0,
            })),
          };
        }
        return {
          hasMore: false,
          items: [
            {
              conversationId: 'group-1',
              conversationType: 'group',
              lastActivityAt: '2026-06-04T10:00:00.000Z',
              lastMessageSeq: 2,
              unreadCount: 0,
            },
            {
              conversationId: 'single-1',
              conversationType: 'single',
              lastActivityAt: '2026-06-04T09:00:00.000Z',
              lastMessageSeq: 1,
              unreadCount: 0,
            },
          ],
        };
      },
    },
  },
  conversations: {
    async getPreferences(conversationId: string) {
      calls.push({ method: 'conversations.getPreferences', conversationId });
      return {
        conversationId,
        isHidden: false,
        isMarkedUnread: false,
        isMuted: false,
        isPinned: false,
        principalId: 'current-user',
        principalKind: 'user',
        tenantId: '100001',
        updatedAt: '2026-06-04T10:00:00.000Z',
      };
    },
    async getProfile(conversationId: string) {
      calls.push({ method: 'conversations.getProfile', conversationId });
      return {
        avatarUrl: `https://cdn.example.test/${conversationId}.png`,
        conversationId,
        displayName: conversationId === 'group-1' ? 'Backend Group' : 'Backend Invited Group',
        notice: '',
        tenantId: '100001',
        updatedAt: '2026-06-04T10:00:00.000Z',
      };
    },
    async listMembers(conversationId: string, params?: Record<string, unknown>) {
      calls.push({ method: 'conversations.listMembers', conversationId, params });
      if (scenario === 'many-projected-groups') {
        activeMemberLookups += 1;
        maxActiveMemberLookups = Math.max(maxActiveMemberLookups, activeMemberLookups);
        try {
          await delay();
        } finally {
          activeMemberLookups -= 1;
        }
      }
      return {
        hasMore: false,
        items: [
          createMember(conversationId, 'current-user'),
          createMember(conversationId, conversationId === 'group-2' ? 'u_invited' : 'u_alice'),
        ],
      };
    },
    async list(params?: Record<string, unknown>) {
      calls.push({ method: 'conversations.list', params });
      if (scenario === 'many-projected-groups') {
        return {
          hasMore: false,
          items: [],
        };
      }
      return {
        hasMore: false,
        items: [
          {
            conversationId: 'group-1',
            conversationType: 'group',
            lastActivityAt: '2026-06-04T10:00:00.000Z',
            lastMessageSeq: 2,
            messageCount: 2,
            tenantId: '100001',
            unreadCount: 0,
          },
          {
            conversationId: 'group-2',
            conversationType: 'group',
            lastActivityAt: '2026-06-04T08:00:00.000Z',
            lastMessageSeq: 0,
            messageCount: 0,
            tenantId: '100001',
            unreadCount: 0,
          },
        ],
      };
    },
    async updateChat() {
      throw new Error('GroupService must not call non-standard updateChat on the IM SDK client');
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkGroupService(() => fakeClient);

  const groups = await service.getGroups();

  assert.deepEqual(
    groups.map((group) => [
      group.id,
      group.name,
      group.memberCount,
      group.activeCount,
      group.members,
      group.avatar,
    ]),
    [
      ['group-1', 'Backend Group', 2, 2, ['current-user', 'u_alice'], 'https://cdn.example.test/group-1.png'],
      ['group-2', 'Backend Invited Group', 2, 2, ['current-user', 'u_invited'], 'https://cdn.example.test/group-2.png'],
    ],
    'group service must hydrate profile and member state for inbox groups and newly joined conversation-list groups through the same injected IM SDK client',
  );
  assert.deepEqual(
    calls.map((call) => call.method),
    [
      'chat.inbox.retrieve',
      'conversations.getPreferences',
      'conversations.getProfile',
      'conversations.list',
      'conversations.getPreferences',
      'conversations.getProfile',
      'conversations.listMembers',
      'conversations.listMembers',
    ],
    'group service must read group inbox projections directly and must not trigger chatClient.getChats single-chat hydration while still profile-hydrating missing group names',
  );

  scenario = 'many-projected-groups';
  calls.length = 0;
  activeMemberLookups = 0;
  maxActiveMemberLookups = 0;
  const projectedGroups = await service.getGroups();
  assert.equal(projectedGroups.length, 9);
  assert.ok(
    maxActiveMemberLookups <= 4,
    `group member-state hydration must be bounded to 4 concurrent member lookups, saw ${maxActiveMemberLookups}`,
  );
  assert.equal(
    calls.filter((call) => call.method === 'conversations.getProfile').length,
    0,
    'complete group inbox projection must not perform per-group profile hydration',
  );

  console.log('sdkwork-im-pc group service client injection contract passed');
}

void main();
