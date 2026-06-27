import assert from 'node:assert/strict';
import type { ConversationMember, ImSdkClient } from '@sdkwork/im-sdk';
import type { Chat, Message, User } from '@sdkwork/im-pc-types';
import {
  GROUP_INVITE_DESCRIPTOR_PREFIX,
  createSdkworkGroupService,
  parseGroupInviteDescriptor,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/GroupService';
import type { ChatService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService';

const calls: Array<{
  body?: Record<string, unknown>;
  chatId?: string;
  conversationId?: string;
  content?: string;
  method: string;
  targetUserId?: string;
  type?: Message['type'];
}> = [];

function createMember(conversationId: string, principalId: string): ConversationMember {
  return {
    attributes: {},
    conversationId,
    joinedAt: '2026-06-09T00:00:00.000Z',
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
    async listMembers(conversationId: string) {
      calls.push({ method: 'conversations.listMembers', conversationId });
      return {
        hasMore: false,
        items: [
          createMember(conversationId, 'current-user'),
          createMember(conversationId, 'existing-user'),
        ],
      };
    },
    async addMember(conversationId: string, body: Record<string, unknown>) {
      calls.push({ method: 'conversations.addMember', conversationId, body });
      return createMember(conversationId, String(body.principalId));
    },
  },
} as unknown as ImSdkClient;

const directChats: Chat[] = [];
const sentMessages: Message[] = [];
const fakeChatService = {
  async startDirectChat(user: Pick<Chat, 'avatar' | 'name'> & { conversationId?: string; directChatId?: string; id: string }) {
    calls.push({ method: 'chatService.startDirectChat', targetUserId: user.id });
    const chat: Chat = {
      avatar: user.avatar,
      id: `direct-${user.id}`,
      name: user.name,
      type: 'single',
      unreadCount: 0,
      updatedAt: Date.parse('2026-06-09T10:00:00.000Z'),
    };
    directChats.push(chat);
    return chat;
  },
  async sendMessage(
    chatId: string,
    content: string,
    type: Message['type'] = 'text',
    _replyTo?: Message['replyTo'],
    extraInfo?: Partial<Message>,
  ) {
    calls.push({ method: 'chatService.sendMessage', chatId, content, type, body: extraInfo });
    const message: Message = {
      chatId,
      content,
      id: `message-${sentMessages.length + 1}`,
      senderId: 'current-user',
      timestamp: Date.parse('2026-06-09T10:00:01.000Z'),
      type,
      ...extraInfo,
    };
    sentMessages.push(message);
    return message;
  },
  async updateChat() {
    return {
      id: 'group-1',
      name: 'Backend Group',
      type: 'group',
      unreadCount: 0,
      updatedAt: Date.parse('2026-06-09T10:00:00.000Z'),
    };
  },
} as unknown as ChatService;

async function main(): Promise<void> {
  const service = createSdkworkGroupService(() => fakeClient, fakeChatService);
  const targetUser: User = {
    avatar: 'https://cdn.example.test/users/invited.png',
    id: ' non-contact-user ',
    name: 'Non Contact User',
    status: 'offline',
  };
  const group: Chat = {
    avatar: 'https://cdn.example.test/groups/group-1.png',
    id: 'group-1',
    name: 'Backend Group',
    type: 'group',
    unreadCount: 0,
    updatedAt: Date.parse('2026-06-09T09:59:00.000Z'),
  };

  const invitation = await service.inviteUserToGroup(group, targetUser);

  assert.deepEqual(
    calls.filter((call) => call.method === 'conversations.addMember'),
    [
      {
        body: {
          principalId: 'non-contact-user',
          principalKind: 'user',
          role: 'member',
        },
        conversationId: 'group-1',
        method: 'conversations.addMember',
      },
    ],
    'owner non-contact group invites must grant group membership through the IM SDK before sending the clickable invitation card',
  );
  assert.deepEqual(
    directChats.map((chat) => [chat.id, chat.name]),
    [['direct-non-contact-user', 'Non Contact User']],
    'owner non-contact group invites must open or bind a direct chat to the target user through ChatService',
  );
  assert.equal(sentMessages.length, 1);
  assert.equal(sentMessages[0]?.type, 'card');
  assert.equal(sentMessages[0]?.chatId, 'direct-non-contact-user');
  assert.equal(sentMessages[0]?.content, 'sdkwork-chat://groups/group-1');
  assert.equal(sentMessages[0]?.fileName, '邀请你加入群聊');
  assert.equal(sentMessages[0]?.appIcon, 'https://cdn.example.test/groups/group-1.png');
  assert.ok(
    sentMessages[0]?.desc?.startsWith(GROUP_INVITE_DESCRIPTOR_PREFIX),
    'group invite card messages must store a machine-readable descriptor in desc',
  );
  assert.deepEqual(
    parseGroupInviteDescriptor(sentMessages[0] as Message),
    {
      groupAvatar: 'https://cdn.example.test/groups/group-1.png',
      groupId: 'group-1',
      groupName: 'Backend Group',
      inviterId: 'current-user',
      kind: 'group_invite',
    },
    'group invite card descriptor must be readable by the target client when the card is clicked',
  );
  assert.deepEqual(
    parseGroupInviteDescriptor({
      chatId: 'direct-non-contact-user',
      content: 'sdkwork-chat://groups/group-1',
      desc: '邀请你加入 Backend Group',
      id: 'legacy-card',
      senderId: 'current-user',
      timestamp: Date.now(),
      type: 'card',
    }),
    {
      groupId: 'group-1',
      kind: 'group_invite',
    },
    'target clients must still open group chats from legacy sdkwork-chat group card URLs even without descriptor metadata',
  );
  assert.deepEqual(
    invitation,
    sentMessages[0],
    'inviteUserToGroup must return the posted invitation card message for UI feedback and tests',
  );

  console.log('sdkwork-im-pc group invite message contract passed');
}

void main();
