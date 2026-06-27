import assert from 'node:assert/strict';
import type { ConversationMember, ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkGroupService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/GroupService';
import type { ChatService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService';

const calls: Array<{
  body?: Record<string, unknown>;
  conversationId?: string;
  method: string;
  params?: Record<string, unknown>;
}> = [];

function createMember(conversationId: string, principalId: string): ConversationMember {
  return {
    attributes: {},
    conversationId,
    joinedAt: '2026-06-04T00:00:00.000Z',
    memberId: `member-${principalId}`,
    principalId,
    principalKind: 'user',
    role: 'member',
    state: 'joined',
    tenantId: '100001',
  };
}

const fakeClient = {
  conversations: {
    async listMembers(conversationId: string, params?: Record<string, unknown>) {
      calls.push({ method: 'conversations.listMembers', conversationId, params });
      return {
        hasMore: false,
        items: [
          createMember(conversationId, 'current-user'),
          createMember(conversationId, 'u_existing'),
        ],
      };
    },
    async addMember(conversationId: string, body: Record<string, unknown>) {
      calls.push({ method: 'conversations.addMember', conversationId, body });
      return createMember(conversationId, String(body.principalId));
    },
    async removeMember(conversationId: string, body: Record<string, unknown>) {
      calls.push({ method: 'conversations.removeMember', conversationId, body });
      return createMember(conversationId, 'u_bob');
    },
    async leave(conversationId: string) {
      calls.push({ method: 'conversations.leave', conversationId });
      return {};
    },
  },
  social: {
    users: {
      async list(params: Record<string, unknown>) {
        calls.push({ method: 'social.users.list', params });
        throw new Error('group membership must not use arbitrary social user search');
      },
    },
  },
} as unknown as ImSdkClient;

function createGroupProjectionClient(options: {
  getProfileName: () => string;
  updateProfile?: (conversationId: string, body: Record<string, unknown>) => Promise<unknown>;
}): ImSdkClient {
  return {
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
          updatedAt: '2026-06-04T00:00:00.000Z',
        };
      },
      async getProfile(conversationId: string) {
        calls.push({ method: 'conversations.getProfile', conversationId });
        return {
          avatarUrl: '',
          conversationId,
          displayName: options.getProfileName(),
          notice: '',
          tenantId: '100001',
          updatedAt: '2026-06-04T00:00:00.000Z',
        };
      },
      async list() {
        calls.push({ method: 'conversations.list' });
        return {
          hasMore: false,
          items: [
            {
              conversationId: 'group-1',
              conversationType: 'group',
              lastActivityAt: '2026-06-04T00:00:00.000Z',
              lastMessageSeq: 1,
              messageCount: 1,
              tenantId: '100001',
              unreadCount: 0,
            },
          ],
        };
      },
      async listMembers(conversationId: string, params?: Record<string, unknown>) {
        calls.push({ method: 'conversations.listMembers', conversationId, params });
        return {
          hasMore: false,
          items: [createMember(conversationId, 'current-user')],
        };
      },
      async updateProfile(conversationId: string, body: Record<string, unknown>) {
        if (options.updateProfile) {
          return options.updateProfile(conversationId, body);
        }
        calls.push({ method: 'conversations.updateProfile', conversationId, body });
        return {
          conversationId,
          displayName: String(body.displayName ?? ''),
          tenantId: '100001',
          updatedAt: '2026-06-04T00:00:00.000Z',
        };
      },
    },
  } as unknown as ImSdkClient;
}

async function main(): Promise<void> {
  const service = createSdkworkGroupService(() => fakeClient);

  await service.addMembers('group-1', [' u_alice ', 'u_existing', 'u_bob', 'u_alice', '']);

  assert.deepEqual(
    calls.filter((call) => call.method === 'social.users.list'),
    [],
    'group member operations must use selected address-book user ids without arbitrary user search',
  );
  assert.deepEqual(
    calls.filter((call) => call.method === 'conversations.addMember'),
    [
      {
        body: {
          principalId: 'u_alice',
          principalKind: 'user',
          role: 'member',
        },
        conversationId: 'group-1',
        method: 'conversations.addMember',
      },
      {
        body: {
          principalId: 'u_bob',
          principalKind: 'user',
          role: 'member',
        },
        conversationId: 'group-1',
        method: 'conversations.addMember',
      },
    ],
    'group add-member flow must invite only selected non-member contact user ids through the IM SDK',
  );

  calls.length = 0;
  await service.removeMember('group-1', 'u_existing');

  assert.deepEqual(
    calls.filter((call) => call.method === 'conversations.removeMember'),
    [
      {
        body: {
          memberId: 'member-u_existing',
        },
        conversationId: 'group-1',
        method: 'conversations.removeMember',
      },
    ],
    'group remove-member flow must resolve the backend conversation member id before removing the selected member',
  );

  let backendGroupName = 'Original Group';
  const failingProfileClient = createGroupProjectionClient({
    getProfileName: () => backendGroupName,
    async updateProfile() {
      throw new Error('profile update failed');
    },
  });
  const updateService = createSdkworkGroupService(() => failingProfileClient);
  await assert.rejects(
    () => updateService.updateGroupInfo('group-1', { name: 'Failed Name' }),
    /profile update failed/u,
    'group profile updates must surface SDK failures',
  );
  const groupsAfterFailedUpdate = await updateService.getGroups();
  assert.equal(
    groupsAfterFailedUpdate[0]?.name,
    'Original Group',
    'failed group profile updates must not pollute the GroupService group projection cache',
  );

  backendGroupName = 'Cached Group Name';
  const profileRefreshClient = createGroupProjectionClient({
    getProfileName: () => backendGroupName,
    async updateProfile(conversationId: string, body: Record<string, unknown>) {
      calls.push({ method: 'conversations.updateProfile', conversationId, body });
      backendGroupName = String(body.displayName ?? backendGroupName);
      return {
        conversationId,
        displayName: backendGroupName,
        tenantId: '100001',
        updatedAt: '2026-06-04T00:00:00.000Z',
      };
    },
  });
  const profileRefreshService = createSdkworkGroupService(() => profileRefreshClient);
  await profileRefreshService.updateGroupInfo('group-1', { name: 'Cached Group Name' });
  backendGroupName = 'Backend Renamed Group';
  const groupsAfterBackendProfileRefresh = await profileRefreshService.getGroups();
  assert.equal(
    groupsAfterBackendProfileRefresh[0]?.name,
    'Backend Renamed Group',
    'group member projection refresh must not let stale cached group profile fields override the latest SDK conversation profile',
  );

  backendGroupName = 'Group group-1';
  const groupsAfterFallbackProfileRefresh = await profileRefreshService.getGroups();
  assert.equal(
    groupsAfterFallbackProfileRefresh[0]?.name,
    'Cached Group Name',
    'group member projection refresh should keep the last successful group profile when the SDK chat profile falls back to a generated name',
  );

  calls.length = 0;
  const groupDeleteChatCalls: string[] = [];
  const groupDeleteChatService = {
    async deleteChat(chatId: string) {
      groupDeleteChatCalls.push(chatId);
    },
  } as unknown as ChatService;
  const deleteService = createSdkworkGroupService(() => fakeClient, groupDeleteChatService);
  await deleteService.deleteGroup('group-1');
  assert.deepEqual(
    calls.filter((call) => call.method === 'conversations.leave'),
    [
      {
        conversationId: 'group-1',
        method: 'conversations.leave',
      },
    ],
    'group leave flow must leave the SDK-backed conversation',
  );
  assert.deepEqual(
    groupDeleteChatCalls,
    ['group-1'],
    'group leave flow must also clear ChatService local view and message caches so stale group messages cannot resurrect the left group',
  );

  console.log('sdkwork-im-pc group member contacts contract passed');
}

void main();
