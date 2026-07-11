import assert from 'node:assert/strict';
import type { Chat } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-types/src/chat';
import {
  createSdkworkSystemAssistantService,
  SYSTEM_ASSISTANT_AGENT,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/SystemAssistantService';

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

  const service = createSdkworkSystemAssistantService();

  const existingAssistant = chat({
    avatar: SYSTEM_ASSISTANT_AGENT.avatar,
    id: 'c_agent_89abcdef0123456789abcdef01',
    name: SYSTEM_ASSISTANT_AGENT.name,
  });
  const existingResult = await service.ensureSystemAssistantChat([existingAssistant]);
  assert.equal(existingResult.available, true, 'existing assistant conversation must be available');
  assert.equal(existingResult.created, false, 'existing assistant conversation must not be recreated');
  assert.equal(existingResult.chat?.id, existingAssistant.id, 'existing assistant conversation must be returned');

  const unavailableResult = await service.ensureSystemAssistantChat([]);
  assert.equal(unavailableResult.available, false, 'missing assistant conversation must not trigger automatic creation during startup');
  assert.equal(unavailableResult.created, false, 'missing assistant startup must not claim creation');
  assert.equal(unavailableResult.chat, null, 'missing assistant startup must not synthesize a local conversation');
  assert.equal(unavailableResult.error, undefined, 'missing assistant startup should not report a backend error when no create was attempted');

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
