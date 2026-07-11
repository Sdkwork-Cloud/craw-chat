import assert from 'node:assert/strict';
import type { ConversationMember, ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkGroupService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/GroupService';

const calls: Array<{
  conversationId?: string;
  method: string;
  params?: Record<string, unknown>;
}> = [];
let scenario: 'default' | 'empty-leading-inbox-page' | 'many-projected-groups' = 'default';

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
      async list(params?: Record<string, unknown>) {
        calls.push({ method: 'chat.inbox.list', params });
        if (scenario === 'empty-leading-inbox-page') {
          if (params?.cursor === 'cursor-1') {
            return {
              items: [
                {
                  avatarUrl: 'https://cdn.example.test/group-2.png',
                  conversationId: 'group-2',
                  conversationType: 'group',
                  displayName: 'Paged Group',
                  lastActivityAt: '2026-06-04T10:30:00.000Z',
                  lastMessageSeq: 3,
                  preferences: {
                    isHidden: false,
                    isMarkedUnread: false,
                    isMuted: false,
                    isPinned: false,
                  },
                  unreadCount: 0,
                },
              ],
              pageInfo: {
                hasMore: false,
                mode: 'cursor',
              },
            };
          }
          return {
            items: [
              {
                conversationId: 'single-leading-page',
                conversationType: 'single',
                lastActivityAt: '2026-06-04T10:00:00.000Z',
                lastMessageSeq: 2,
                unreadCount: 0,
              },
            ],
            pageInfo: {
              hasMore: true,
              mode: 'cursor',
              nextCursor: 'cursor-1',
            },
          };
        }
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
              preferences: {
                isHidden: false,
                isMarkedUnread: false,
                isMuted: false,
                isPinned: false,
              },
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
              preferences: {
                isHidden: false,
                isMarkedUnread: false,
                isMuted: false,
                isPinned: false,
              },
              unreadCount: 0,
            },
            {
              avatarUrl: 'https://cdn.example.test/group-hidden.png',
              conversationId: 'group-hidden',
              conversationType: 'group',
              displayName: 'Hidden Group',
              lastActivityAt: '2026-06-04T10:15:00.000Z',
              lastMessageSeq: 4,
              preferences: {
                isHidden: true,
                isMarkedUnread: false,
                isMuted: false,
                isPinned: false,
              },
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
      if (scenario === 'empty-leading-inbox-page' || scenario === 'many-projected-groups') {
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
      group.avatar?.startsWith('https://cdn.example.test/'),
    ]),
    [
      ['group-1', 'Group chat', undefined, undefined, undefined, false],
    ],
    'group service list aggregation must keep inbox groups lightweight when display and member projections are missing',
  );
  assert.deepEqual(
    calls.map((call) => call.method),
    [
      'chat.inbox.list',
    ],
    'group service must not issue per-group profile, preference, member, or conversation-list fallback calls while loading group lists',
  );

  scenario = 'many-projected-groups';
  calls.length = 0;
  const projectedGroups = await service.getGroups();
  assert.equal(projectedGroups.length, 9);
  assert.equal(
    calls.filter((call) => call.method === 'conversations.getProfile').length,
    0,
    'complete group inbox projection must not perform per-group profile hydration',
  );
  assert.equal(
    calls.filter((call) => call.method === 'conversations.getPreferences').length,
    0,
    'complete group inbox projection must not perform per-group preference hydration',
  );
  assert.equal(
    calls.filter((call) => call.method === 'conversations.listMembers').length,
    0,
    'complete group inbox projection must not perform per-group member hydration',
  );

  scenario = 'empty-leading-inbox-page';
  calls.length = 0;
  const groupsAfterEmptyLeadingInboxPage = await service.getGroups();
  assert.deepEqual(
    groupsAfterEmptyLeadingInboxPage.map((group) => group.id),
    [],
    'getGroups compatibility entry must not chase additional cursor pages when the first inbox page contains no group entries',
  );
  assert.deepEqual(
    calls.filter((call) => call.method === 'chat.inbox.list').map((call) => call.params),
    [
      { pageSize: 20, conversationType: 'group' },
    ],
    'getGroups compatibility entry must request only one bounded server-filtered SDK inbox page',
  );

  scenario = 'default';
  calls.length = 0;
  const activeGroup = await service.getGroupById('group-1');
  assert.deepEqual(
    activeGroup
      ? [activeGroup.id, activeGroup.name, activeGroup.memberCount, activeGroup.activeCount, activeGroup.members, activeGroup.avatar]
      : null,
    ['group-1', 'Backend Group', 2, 2, ['current-user', 'u_alice'], 'https://cdn.example.test/group-1.png'],
    'explicit active-group detail loading must hydrate one group profile and member state through the injected IM SDK client',
  );
  assert.deepEqual(
    calls.map((call) => call.method),
    [
      'conversations.getPreferences',
      'conversations.getProfile',
      'conversations.listMembers',
    ],
    'explicit active-group detail loading may call profile/preferences/members once for that selected group only',
  );

  console.log('sdkwork-im-pc group service client injection contract passed');
}

void main();
