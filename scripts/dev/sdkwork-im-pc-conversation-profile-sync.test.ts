import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkChatService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService';

type ProfileCall = {
  conversationId: string;
  body?: {
    displayName?: string;
    avatarUrl?: string;
    notice?: string;
  };
};

const retrieveCalls: ProfileCall[] = [];
const updateCalls: ProfileCall[] = [];
const storedProfile = {
  tenantId: '100001',
  conversationId: 'chat-1',
  displayName: 'Backend Group Name',
  avatarUrl: 'https://cdn.example.test/group.png',
  notice: 'Backend group notice',
  updatedAt: '2026-06-04T11:02:00.000Z',
};

const fakeClient = {
  chat: {
    inbox: {
      async list() {
        return {
          items: [
            {
              conversationId: 'chat-1',
              conversationType: 'group',
              unreadCount: 3,
              lastMessageSeq: 7,
              lastActivityAt: '2026-06-04T11:00:00.000Z',
            },
          ],
          hasMore: false,
        };
      },
    },
  },
  conversations: {
    async getPreferences(conversationId: string) {
      return {
        tenantId: '100001',
        conversationId,
        principalKind: 'user',
        principalId: 'u_owner',
        isPinned: false,
        isMuted: false,
        updatedAt: '2026-06-04T11:01:00.000Z',
      };
    },
    async getProfile(conversationId: string) {
      retrieveCalls.push({ conversationId });
      return {
        ...storedProfile,
        conversationId,
      };
    },
    async updateProfile(
      conversationId: string,
      body: {
        displayName?: string;
        avatarUrl?: string;
        notice?: string;
      },
    ) {
      updateCalls.push({ conversationId, body });
      storedProfile.displayName = body.displayName?.trim() ?? storedProfile.displayName;
      storedProfile.avatarUrl = body.avatarUrl?.trim() ?? storedProfile.avatarUrl;
      storedProfile.notice = body.notice?.trim() ?? storedProfile.notice;
      storedProfile.updatedAt = '2026-06-04T11:03:00.000Z';
      return {
        ...storedProfile,
        conversationId,
      };
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkChatService(() => fakeClient);

  const chats = await service.getChats();
  assert.deepEqual(
    retrieveCalls,
    [],
    'chat list sync must not restore missing conversation profile fields through per-conversation IM SDK calls',
  );
  assert.equal(chats[0]?.name, 'Group chat');
  assert.equal(chats[0]?.notice, undefined);

  const updated = await service.updateChat('chat-1', {
    avatar: 'https://cdn.example.test/new.png',
    memberCount: 5,
    name: ' Renamed Group ',
    notice: ' New group notice ',
  });

  assert.deepEqual(
    updateCalls,
    [
      {
        conversationId: 'chat-1',
        body: {
          avatarUrl: 'https://cdn.example.test/new.png',
          displayName: ' Renamed Group ',
          notice: ' New group notice ',
        },
      },
    ],
    'chat profile updates must persist only name/avatar/notice through the standard IM SDK profile API',
  );
  assert.equal(updated.name, 'Renamed Group');
  assert.equal(updated.avatar, 'https://cdn.example.test/new.png');
  assert.equal(updated.notice, 'New group notice');
  assert.equal(updated.memberCount, 5);

  const updatedMembers = await service.updateChat('chat-1', {
    activeCount: 4,
    memberCount: 4,
    members: ['u_owner', 'u_alice', 'u_bob', 'u_carol'],
    type: 'group',
  });
  assert.equal(
    updatedMembers.activeCount,
    4,
    'chat view-state updates must preserve group active member counts for the PC group panel',
  );
  assert.deepEqual(
    updatedMembers.members,
    ['u_owner', 'u_alice', 'u_bob', 'u_carol'],
    'chat view-state updates must preserve group member ids for follow-up local group rendering',
  );

  const chatsAfterMemberUpdate = await service.getChats();
  assert.equal(chatsAfterMemberUpdate[0]?.activeCount, 4);
  assert.deepEqual(chatsAfterMemberUpdate[0]?.members, ['u_owner', 'u_alice', 'u_bob', 'u_carol']);

  console.log('sdkwork-im-pc conversation profile sync contract passed');
}

void main();
