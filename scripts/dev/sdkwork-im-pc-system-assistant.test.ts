import assert from 'node:assert/strict';
import type { Chat } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-types/src/chat';
import {
  createSdkworkSystemAssistantService,
  SYSTEM_ASSISTANT_AGENT,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/SystemAssistantService';

type StartAgentChatInput = Pick<Chat, 'avatar' | 'name'> & { id: string };

function chat(overrides: Partial<Chat> & Pick<Chat, 'id' | 'name'>): Chat {
  return {
    avatar: '',
    type: 'single',
    unreadCount: 0,
    updatedAt: 100,
    ...overrides,
  };
}

async function main(): Promise<void> {
  assert.equal(
    SYSTEM_ASSISTANT_AGENT.name,
    'System Assistant',
    'assistant SDK profile fallback name must be neutral English and UI localization must provide the visible name',
  );

  const calls: StartAgentChatInput[] = [];
  const createdAssistant = chat({
    avatar: SYSTEM_ASSISTANT_AGENT.avatar,
    id: 'c_agent_0123456789abcdef01234567',
    name: SYSTEM_ASSISTANT_AGENT.name,
    updatedAt: 200,
  });
  const service = createSdkworkSystemAssistantService({
    startAgentChat: async (agent) => {
      calls.push(agent);
      return createdAssistant;
    },
  });

  const existingAssistant = chat({
    avatar: SYSTEM_ASSISTANT_AGENT.avatar,
    id: 'c_agent_89abcdef0123456789abcdef01',
    name: SYSTEM_ASSISTANT_AGENT.name,
  });
  const existingResult = await service.ensureSystemAssistantChat([existingAssistant]);
  assert.equal(existingResult.available, true, 'existing assistant conversation must be available');
  assert.equal(existingResult.created, false, 'existing assistant conversation must not be recreated');
  assert.equal(existingResult.chat?.id, existingAssistant.id, 'existing assistant conversation must be returned');
  assert.equal(calls.length, 0, 'existing assistant conversation must not call startAgentChat');

  const createdResult = await service.ensureSystemAssistantChat([]);
  assert.equal(createdResult.available, true, 'missing assistant conversation should be created when the IM SDK accepts it');
  assert.equal(createdResult.created, true, 'missing assistant conversation must report that it was created');
  assert.equal(createdResult.chat?.id, createdAssistant.id, 'created assistant conversation must be returned');
  assert.deepEqual(
    calls,
    [
      {
        avatar: SYSTEM_ASSISTANT_AGENT.avatar,
        id: SYSTEM_ASSISTANT_AGENT.id,
        name: SYSTEM_ASSISTANT_AGENT.name,
      },
    ],
    'assistant creation must delegate to ChatService.startAgentChat with the standard agent id and profile',
  );

  const failingService = createSdkworkSystemAssistantService({
    startAgentChat: async () => {
      throw new Error('agent dialog unavailable');
    },
  });
  const unavailableResult = await failingService.ensureSystemAssistantChat([]);
  assert.equal(unavailableResult.available, false, 'assistant startup failure must degrade instead of blocking login');
  assert.equal(unavailableResult.created, false, 'failed assistant startup must not claim creation');
  assert.equal(unavailableResult.chat, null, 'failed assistant startup must not synthesize a local conversation');
  assert.match(
    unavailableResult.error instanceof Error ? unavailableResult.error.message : '',
    /agent dialog unavailable/u,
    'assistant startup should preserve the underlying SDK error for diagnostics',
  );

  const unreadDirectChat = chat({
    id: 'pc-direct-alice-current-user',
    name: 'Alice',
    unreadCount: 2,
    updatedAt: 300,
  });
  const recentDirectChat = chat({
    id: 'pc-direct-bob-current-user',
    name: 'Bob',
    unreadCount: 0,
    updatedAt: 500,
  });
  assert.equal(
    service.selectInitialChat([existingAssistant, recentDirectChat, unreadDirectChat])?.id,
    unreadDirectChat.id,
    'startup should prefer a real unread conversation over the assistant workspace',
  );
  assert.equal(
    service.selectInitialChat([existingAssistant])?.id ?? null,
    existingAssistant.id,
    'startup should open the default assistant conversation when it is the only available conversation',
  );
  assert.equal(
    service.isSystemAssistantChat(existingAssistant),
    true,
    'assistant detection must recognize the stable SDK-backed assistant dialog id',
  );
  assert.equal(
    service.isSystemAssistantChat(recentDirectChat),
    false,
    'assistant detection must not classify normal conversations as the system assistant',
  );

  console.log('sdkwork-im-pc system assistant contract passed');
}

void main();
