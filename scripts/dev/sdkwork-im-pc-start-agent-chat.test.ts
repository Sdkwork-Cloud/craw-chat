import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkChatService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService';

type StartAgentChatCall =
  | {
      body: Record<string, unknown>;
      method: 'chat.inbox.list';
    }
  | {
      conversationId: string;
      method: 'conversations.listMembers';
    }
  | {
      body: Record<string, unknown>;
      method: 'conversations.createAgentDialog';
    }
  | {
      body: Record<string, unknown>;
      conversationId: string;
      method: 'conversations.updateProfile' | 'conversations.updatePreferences';
    };

const calls: StartAgentChatCall[] = [];

const CANONICAL_AGENT_DIALOG_ID = 'c_agent_0123456789abcdef01234567';
const EXISTING_AGENT_DIALOG_ID = 'c_agent_89abcdef0123456789abcdef01';
let inboxScenario: 'existing-agent-dialog' | 'missing-agent-dialog' = 'existing-agent-dialog';

const fakeClient = {
  chat: {
    inbox: {
      async list(params?: Record<string, unknown>) {
        calls.push({ method: 'chat.inbox.list', body: params ?? {} });
        return {
          items: inboxScenario === 'existing-agent-dialog'
            ? [
                {
                  tenantId: '100001',
                  conversationId: EXISTING_AGENT_DIALOG_ID,
                  conversationType: 'agent_dialog',
                  preferences: {
                    isPinned: false,
                    isMuted: false,
                    isMarkedUnread: false,
                    isHidden: false,
                  },
                  lastActivityAt: '2026-06-10T08:00:00.000Z',
                  lastMessageId: 'msg-agent-existing-1',
                  lastSenderId: 'agent.code',
                  messageCount: 1,
                  lastMessageSeq: 12,
                  lastSummary: 'Existing agent response',
                  unreadCount: 0,
                },
              ]
            : [],
          pageInfo: {
            mode: 'cursor',
            hasMore: false,
            nextCursor: null,
          },
        };
      },
    },
  },
  conversations: {
    async listMembers(conversationId: string) {
      calls.push({ method: 'conversations.listMembers', conversationId });
      return {
        items: [
          {
            tenantId: '100001',
            conversationId,
            memberId: 'member-current-user',
            principalId: 'current-user',
            principalKind: 'user',
            role: 'owner',
            state: 'joined',
            joinedAt: '2026-06-04T00:00:00.000Z',
          },
          {
            tenantId: '100001',
            conversationId,
            memberId: 'member-agent-code',
            principalId: 'agent.code',
            principalKind: 'agent',
            role: 'member',
            state: 'joined',
            joinedAt: '2026-06-04T00:00:00.000Z',
          },
        ],
        pageInfo: {
          mode: 'cursor',
          hasMore: false,
          nextCursor: null,
        },
      };
    },
    async createAgentDialog(body: Record<string, unknown>) {
      calls.push({ method: 'conversations.createAgentDialog', body });
      return {
        conversationId: CANONICAL_AGENT_DIALOG_ID,
        eventId: 'evt-agent-dialog-created',
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

  const existingChat = await service.startAgentChat({
    avatar: 'https://cdn.example.test/agent.png',
    id: 'agent.code',
    name: 'Code Assistant',
  });

  assert.deepEqual(
    calls.map((call) => call.method),
    [
      'chat.inbox.list',
      'conversations.listMembers',
      'conversations.updateProfile',
      'conversations.updatePreferences',
    ],
    'starting an agent chat must first reuse the unified conversation inbox instead of posting a duplicate agent dialog',
  );
  assert.equal(
    existingChat.id,
    EXISTING_AGENT_DIALOG_ID,
    'starting an existing agent chat must return the unified inbox conversation id',
  );
  assert.equal(existingChat.name, 'Code Assistant');
  assert.equal(existingChat.avatar, 'https://cdn.example.test/agent.png');
  assert.equal(existingChat.unreadCount, 0);

  inboxScenario = 'missing-agent-dialog';
  calls.length = 0;
  const chat = await service.startAgentChat({
    avatar: 'https://cdn.example.test/agent.png',
    id: 'agent.code',
    name: 'Code Assistant',
  });

  assert.deepEqual(
    calls.slice(0, 2).map((call) => call.method),
    ['chat.inbox.list', 'conversations.createAgentDialog'],
    'starting a new agent chat may create a backend agent dialog only after the unified inbox has no matching conversation',
  );
  assert.deepEqual(
    calls[1],
    {
      method: 'conversations.createAgentDialog',
      body: {
        agentId: 'agent.code',
      },
    },
    'new agent dialog creation must still go through the generated IM SDK',
  );
  assert.equal(
    chat.id,
    CANONICAL_AGENT_DIALOG_ID,
    'starting an agent chat must return the server-assigned canonical conversation id',
  );
  assert.match(
    chat.id,
    /^c_agent_[a-f0-9]{24}$/u,
    'agent dialog conversation ids must use the canonical server format',
  );
  assert.deepEqual(
    calls.slice(2),
    [
      {
        body: {
          avatarUrl: 'https://cdn.example.test/agent.png',
          displayName: 'Code Assistant',
        },
        conversationId: CANONICAL_AGENT_DIALOG_ID,
        method: 'conversations.updateProfile',
      },
      {
        body: {
          isHidden: false,
        },
        conversationId: CANONICAL_AGENT_DIALOG_ID,
        method: 'conversations.updatePreferences',
      },
    ],
    'starting an agent chat must sync display profile and unhide the real agent dialog',
  );
  assert.deepEqual(
    [chat.name, chat.avatar, chat.type, chat.unreadCount],
    ['Code Assistant', 'https://cdn.example.test/agent.png', 'single', 0],
  );

  const callCountAfterStandardAgent = calls.length;
  await assert.rejects(
    () =>
      service.startAgentChat({
        avatar: 'https://cdn.example.test/legacy-agent.png',
        id: 'agent-code',
        name: 'Legacy Code Assistant',
      }),
    /Agent chat target id must use the standard agent\./,
    'starting an agent chat must reject legacy agent-* ids before calling the backend',
  );
  assert.equal(
    calls.length,
    callCountAfterStandardAgent,
    'invalid legacy agent ids must not reach conversations.createAgentDialog',
  );

  console.log('sdkwork-im-pc start agent chat contract passed');
}

void main();
