import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkChatService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService';

const sdkCalls: string[] = [];
let inboxScenario: 'projected-direct' | 'legacy-direct' | 'many-legacy-direct' | 'group-avatar-only' | 'projected-group' | 'missing-group' | 'agent-handoff' | 'missing-agent-dialog' = 'legacy-direct';
let activeMemberLookups = 0;
let maxActiveMemberLookups = 0;

async function delay(): Promise<void> {
  await new Promise((resolve) => {
    setTimeout(resolve, 5);
  });
}

const fakeClient = {
  chat: {
    inbox: {
      async retrieve() {
        sdkCalls.push('chat.inbox.retrieve');
        if (inboxScenario === 'projected-direct') {
          return {
            items: [
              {
                tenantId: '100001',
                conversationId: 'c_direct_projected',
                conversationType: 'single',
                displayName: 'Alice Project Lead',
                avatarUrl: 'https://cdn.example.test/alice.png',
                displaySource: 'contact_remark',
                peer: {
                  principalKind: 'user',
                  principalId: 'u_alice',
                  userId: 'u_alice',
                  chatId: 'alice-chat-id',
                  displayName: 'Alice Chen',
                  avatarUrl: 'https://cdn.example.test/alice.png',
                  relationshipState: 'active',
                },
                preferences: {
                  isPinned: true,
                  isMuted: true,
                  isMarkedUnread: true,
                  isHidden: false,
                },
                lastActivityAt: '2026-06-10T08:00:00.000Z',
                lastMessageId: 'msg-1',
                lastSenderId: 'u_alice',
                messageCount: 1,
                lastMessageSeq: 12,
                lastSummary: 'See you soon',
                unreadCount: 0,
              },
            ],
            hasMore: false,
          };
        }
        if (inboxScenario === 'many-legacy-direct') {
          return {
            items: Array.from({ length: 9 }, (_, index) => ({
              tenantId: '100001',
              conversationId: `c_direct_perf_${index}`,
              conversationType: 'single',
              lastActivityAt: `2026-06-10T08:00:0${index}.000Z`,
              lastMessageId: `msg-perf-${index}`,
              lastSenderId: `u_peer_${index}`,
              messageCount: 1,
              lastMessageSeq: 12 + index,
              lastSummary: 'See you soon',
              unreadCount: 0,
            })),
            hasMore: false,
          };
        }
        if (inboxScenario === 'group-avatar-only') {
          return {
            items: [
              {
                tenantId: '100001',
                conversationId: 'c_group_avatar_only',
                conversationType: 'group',
                avatarUrl: 'https://cdn.example.test/group.png',
                preferences: {
                  isPinned: false,
                  isMuted: false,
                  isMarkedUnread: false,
                  isHidden: false,
                },
                lastActivityAt: '2026-06-10T08:00:00.000Z',
                lastMessageId: 'msg-group-1',
                lastSenderId: 'u_alice',
                messageCount: 1,
                lastMessageSeq: 12,
                lastSummary: 'Group update',
                unreadCount: 0,
              },
            ],
            hasMore: false,
          };
        }
        if (inboxScenario === 'projected-group') {
          return {
            items: [
              {
                tenantId: '100001',
                conversationId: 'c_group_projected',
                conversationType: 'group',
                displayName: 'Design Review Room',
                avatarUrl: 'https://cdn.example.test/design-room.png',
                preferences: {
                  isPinned: false,
                  isMuted: true,
                  isMarkedUnread: false,
                  isHidden: false,
                },
                lastActivityAt: '2026-06-10T08:00:00.000Z',
                lastMessageId: 'msg-group-projected-1',
                lastSenderId: 'u_alice',
                messageCount: 1,
                lastMessageSeq: 12,
                lastSummary: 'Group update',
                unreadCount: 0,
              },
            ],
            hasMore: false,
          };
        }
        if (inboxScenario === 'missing-group') {
          return {
            items: [
              {
                tenantId: '100001',
                conversationId: 'c_group_missing_profile',
                conversationType: 'group',
                preferences: {
                  isPinned: false,
                  isMuted: false,
                  isMarkedUnread: false,
                  isHidden: false,
                },
                lastActivityAt: '2026-06-10T08:00:00.000Z',
                lastMessageId: 'msg-group-missing-1',
                lastSenderId: 'u_alice',
                messageCount: 1,
                lastMessageSeq: 12,
                lastSummary: 'Group update',
                unreadCount: 0,
              },
            ],
            hasMore: false,
          };
        }
        if (inboxScenario === 'agent-handoff') {
          return {
            items: [
              {
                tenantId: '100001',
                conversationId: 'c_agent_handoff_123',
                conversationType: 'single',
                agentHandoff: true,
                preferences: {
                  isPinned: false,
                  isMuted: false,
                  isMarkedUnread: false,
                  isHidden: false,
                },
                lastActivityAt: '2026-06-10T08:00:00.000Z',
                lastMessageId: 'msg-agent-1',
                lastSenderId: 'agent',
                messageCount: 1,
                lastMessageSeq: 12,
                lastSummary: 'Support update',
                unreadCount: 0,
              },
            ],
            hasMore: false,
          };
        }
        if (inboxScenario === 'missing-agent-dialog') {
          return {
            items: [
              {
                tenantId: '100001',
                conversationId: 'pc-agent-current-user-agent.code',
                conversationType: 'single',
                preferences: {
                  isPinned: false,
                  isMuted: false,
                  isMarkedUnread: false,
                  isHidden: false,
                },
                lastActivityAt: '2026-06-10T08:00:00.000Z',
                lastMessageId: 'msg-agent-dialog-1',
                lastSenderId: 'agent.code',
                messageCount: 1,
                lastMessageSeq: 12,
                lastSummary: 'Agent update',
                unreadCount: 0,
              },
            ],
            hasMore: false,
          };
        }
        return {
          items: [
            {
              tenantId: '100001',
              conversationId: 'c_direct_e46da962d83a0fc8c4069f96',
              conversationType: 'single',
              lastActivityAt: '2026-06-10T08:00:00.000Z',
              lastMessageId: 'msg-1',
              lastSenderId: 'u_alice',
              messageCount: 1,
              lastMessageSeq: 12,
              lastSummary: 'See you soon',
              unreadCount: 0,
            },
          ],
          hasMore: false,
        };
      },
    },
  },
  conversations: {
    async getPreferences(conversationId: string) {
      sdkCalls.push(`conversations.getPreferences:${conversationId}`);
      return {
        tenantId: '100001',
        conversationId,
        ownerUserId: 'current-user',
        targetUserId: conversationId,
        isPinned: false,
        isMuted: false,
        isMarkedUnread: false,
        isHidden: false,
        updatedAt: '2026-06-10T08:00:01.000Z',
      };
    },
    async getProfile(conversationId: string) {
      sdkCalls.push(`conversations.getProfile:${conversationId}`);
      if (conversationId === 'c_group_avatar_only') {
        return {
          tenantId: '100001',
          conversationId,
          displayName: 'Project Room',
          avatarUrl: 'https://cdn.example.test/group.png',
          notice: '',
          updatedAt: '2026-06-10T08:00:01.000Z',
        };
      }
      if (conversationId === 'c_group_missing_profile') {
        throw new Error('profile unavailable');
      }
      if (conversationId === 'pc-agent-current-user-agent.code') {
        throw new Error('agent dialog profile unavailable');
      }
      return {
        tenantId: '100001',
        conversationId,
        displayName: `Chat ${conversationId}`,
        avatarUrl: '',
        notice: '',
        updatedAt: '2026-06-10T08:00:01.000Z',
      };
    },
    async listMembers(conversationId: string) {
      sdkCalls.push(`conversations.listMembers:${conversationId}`);
      if (inboxScenario === 'many-legacy-direct') {
        activeMemberLookups += 1;
        maxActiveMemberLookups = Math.max(maxActiveMemberLookups, activeMemberLookups);
        try {
          await delay();
        } finally {
          activeMemberLookups -= 1;
        }
      }
      const peerUserId = conversationId.startsWith('c_direct_perf_')
        ? `u_peer_${conversationId.slice('c_direct_perf_'.length)}`
        : 'u_alice';
      return {
        items: [
          {
            tenantId: '100001',
            conversationId,
            memberId: 'm-current',
            principalId: 'current-user',
            principalKind: 'user',
            role: 'owner',
            state: 'active',
            joinedAt: '2026-06-10T08:00:00.000Z',
          },
          {
            tenantId: '100001',
            conversationId,
            memberId: 'm-alice',
            principalId: peerUserId,
            principalKind: 'user',
            role: 'member',
            state: 'active',
            joinedAt: '2026-06-10T08:00:00.000Z',
          },
        ],
        hasMore: false,
      };
    },
  },
  social: {
    contacts: {
      preferences: {
        async retrieve(targetUserId: string) {
          sdkCalls.push(`social.contacts.preferences.retrieve:${targetUserId}`);
          return {
            tenantId: '100001',
            ownerUserId: 'current-user',
            targetUserId,
            isStarred: false,
            remark: 'Alice Project Lead',
            isBlocked: false,
            updatedAt: '2026-06-10T08:00:01.000Z',
          };
        },
      },
    },
    users: {
      async list(params: { q?: string }) {
        sdkCalls.push(`social.users.list:${params.q ?? ''}`);
        if (params.q?.startsWith('u_peer_')) {
          return {
            items: [
              {
                tenantId: '100001',
                userId: params.q,
                chatId: `${params.q}-chat-id`,
                displayName: `Peer ${params.q}`,
                relationshipState: 'active',
                avatarUrl: `https://cdn.example.test/${params.q}.png`,
              },
            ],
            hasMore: false,
          };
        }
        return {
          items: [
            {
              tenantId: '100001',
              userId: 'u_alice',
              chatId: 'alice-chat-id',
              displayName: 'Alice Chen',
              relationshipState: 'active',
              avatarUrl: 'https://cdn.example.test/alice.png',
            },
          ],
          hasMore: false,
        };
      },
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkChatService({
    getClient: () => fakeClient,
    getSession: () => ({
      authToken: 'auth-token',
      accessToken: 'access-token',
      context: {
        tenantId: '100001',
        userId: 'current-user',
      },
      user: {
        id: 'current-user',
        userId: 'current-user',
        displayName: 'Current User',
      },
    }),
  });

  inboxScenario = 'projected-direct';
  sdkCalls.length = 0;
  const projectedChats = await service.getChats();
  assert.equal(projectedChats.length, 1);
  assert.equal(
    projectedChats[0]?.name,
    'Alice Project Lead',
    'single-chat title must prefer the inbox display projection returned for the current viewer',
  );
  assert.equal(projectedChats[0]?.avatar, 'https://cdn.example.test/alice.png');
  assert.equal(projectedChats[0]?.type, 'single');
  assert.equal(projectedChats[0]?.isPinned, true);
  assert.equal(projectedChats[0]?.isMuted, true);
  assert.equal(projectedChats[0]?.isMarkedUnread, true);
  assert.deepEqual(
    sdkCalls,
    ['chat.inbox.retrieve'],
    'complete inbox display projection must not trigger per-conversation profile, preference, member, social user, or contact preference hydration',
  );

  inboxScenario = 'group-avatar-only';
  sdkCalls.length = 0;
  const groupChats = await service.getChats();
  assert.equal(groupChats.length, 1);
  assert.equal(
    groupChats[0]?.name,
    'Project Room',
    'group-chat title must not treat an avatar-only inbox projection as a complete group display projection',
  );
  assert.deepEqual(
    sdkCalls,
    [
      'chat.inbox.retrieve',
      'conversations.getProfile:c_group_avatar_only',
    ],
    'avatar-only group inbox projection should still hydrate the group profile without per-conversation preference reads',
  );

  inboxScenario = 'projected-group';
  sdkCalls.length = 0;
  const projectedGroupChats = await service.getChats();
  assert.equal(projectedGroupChats.length, 1);
  assert.equal(
    projectedGroupChats[0]?.name,
    'Design Review Room',
    'complete group inbox display projection must provide the group title without extra profile hydration',
  );
  assert.equal(projectedGroupChats[0]?.avatar, 'https://cdn.example.test/design-room.png');
  assert.equal(projectedGroupChats[0]?.isMuted, true);
  assert.deepEqual(
    sdkCalls,
    ['chat.inbox.retrieve'],
    'complete group inbox display and preference projection must not trigger per-conversation profile or preference hydration',
  );

  inboxScenario = 'missing-group';
  sdkCalls.length = 0;
  const missingGroupChats = await service.getChats();
  assert.equal(missingGroupChats.length, 1);
  assert.equal(
    missingGroupChats[0]?.name,
    'Group chat',
    'group fallback title must stay user-safe when projection and profile are unavailable',
  );
  assert.doesNotMatch(
    missingGroupChats[0]?.name ?? '',
    /c_group_missing_profile|Group c_/u,
    'group fallback title must not expose internal conversation ids',
  );

  inboxScenario = 'agent-handoff';
  sdkCalls.length = 0;
  const agentChats = await service.getChats();
  assert.equal(agentChats.length, 1);
  assert.equal(
    agentChats[0]?.name,
    'Support conversation',
    'agent handoff fallback title must stay product-facing without exposing internal conversation ids',
  );
  assert.doesNotMatch(
    agentChats[0]?.name ?? '',
    /c_agent_handoff|Agent Handoff c_/u,
    'agent handoff fallback title must not expose internal conversation ids',
  );

  inboxScenario = 'missing-agent-dialog';
  sdkCalls.length = 0;
  const missingAgentDialogChats = await service.getChats();
  assert.equal(missingAgentDialogChats.length, 1);
  assert.equal(
    missingAgentDialogChats[0]?.name,
    'AI assistant chat',
    'agent dialog fallback title must stay agent-specific when projection and profile are unavailable',
  );
  assert.doesNotMatch(
    missingAgentDialogChats[0]?.name ?? '',
    /pc-agent|Direct chat|Chat pc-agent/u,
    'agent dialog fallback title must not expose internal ids or degrade to ordinary direct-chat text',
  );
  assert.deepEqual(
    sdkCalls,
    ['chat.inbox.retrieve'],
    'missing agent dialog projection must not trigger ordinary direct-chat member or social-user hydration',
  );

  inboxScenario = 'legacy-direct';
  sdkCalls.length = 0;
  const chats = await service.getChats();
  assert.equal(chats.length, 1);
  assert.equal(
    chats[0]?.name,
    'Alice Project Lead',
    'single-chat title must use the peer contact remark/profile instead of exposing Chat c_direct_*',
  );
  assert.equal(chats[0]?.avatar, 'https://cdn.example.test/alice.png');
  assert.equal(chats[0]?.type, 'single');
  assert.ok(
    sdkCalls.includes('conversations.listMembers:c_direct_e46da962d83a0fc8c4069f96'),
    'direct-chat hydration must resolve the peer through the standard IM members SDK surface',
  );
  assert.ok(
    sdkCalls.includes('social.users.list:u_alice'),
    'direct-chat hydration must resolve the peer profile through the standard IM social user SDK surface',
  );

  inboxScenario = 'many-legacy-direct';
  sdkCalls.length = 0;
  activeMemberLookups = 0;
  maxActiveMemberLookups = 0;
  const manyLegacyChats = await service.getChats();
  assert.equal(manyLegacyChats.length, 9);
  assert.ok(
    maxActiveMemberLookups <= 4,
    `legacy direct-chat title hydration must be bounded to 4 concurrent member lookups, saw ${maxActiveMemberLookups}`,
  );

  console.log('sdkwork-im-pc direct chat title contract passed');
}

void main();
