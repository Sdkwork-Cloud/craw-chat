import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkChatService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService';

type StartAgentChatCall =
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

const fakeClient = {
  conversations: {
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

  const chat = await service.startAgentChat({
    avatar: 'https://cdn.example.test/agent.png',
    id: 'agent.code',
    name: 'Code Assistant',
  });

  assert.deepEqual(
    calls[0],
    {
      method: 'conversations.createAgentDialog',
      body: {
        agentId: 'agent.code',
      },
    },
    'starting an agent chat must create a real backend agent dialog conversation through the IM SDK',
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
    calls.slice(1),
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
