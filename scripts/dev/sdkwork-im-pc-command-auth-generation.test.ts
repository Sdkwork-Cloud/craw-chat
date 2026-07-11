import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';

import type { SdkworkChatSession } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/session.ts';
import {
  createSdkworkChatService,
  type ChatService,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService.ts';

interface Deferred<T> {
  promise: Promise<T>;
  reject: (reason?: unknown) => void;
  resolve: (value: T) => void;
}

interface TestMessage {
  chatId: string;
  content: string;
  id: string;
  reactions?: Array<{ count: number; emoji: string; hasReacted: boolean }>;
  senderId: string;
  timestamp: number;
  type: 'text';
}

interface ChatServiceInternals {
  conversationViewState: Map<string, Record<string, unknown>>;
  handleAuthSessionChanged: () => void;
  localMessages: Map<string, TestMessage[]>;
  setLocalMessages: (chatId: string, messages: TestMessage[]) => void;
}

interface Harness {
  internals: ChatServiceInternals;
  service: ChatService;
  switchSession: (principalId: string) => void;
}

function createDeferred<T>(): Deferred<T> {
  let reject!: (reason?: unknown) => void;
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((resolvePromise, rejectPromise) => {
    reject = rejectPromise;
    resolve = resolvePromise;
  });
  return { promise, reject, resolve };
}

function createSession(principalId: string): SdkworkChatSession {
  return {
    accessToken: `access-${principalId}`,
    authToken: `auth-${principalId}`,
    context: {
      actorId: principalId,
      actorKind: 'user',
      appId: 'sdkwork-im-pc',
      organizationId: 'organization-1',
      tenantId: 'tenant-1',
      userId: principalId,
    },
  } as SdkworkChatSession;
}

function createHarness(client: ImSdkClient): Harness {
  const session = { current: createSession('principal-a') };
  const service = createSdkworkChatService({
    getClient: () => client,
    getSession: () => session.current,
  });
  const internals = service as unknown as ChatServiceInternals;
  return {
    internals,
    service,
    switchSession(principalId: string) {
      session.current = createSession(principalId);
      internals.handleAuthSessionChanged();
    },
  };
}

async function waitForCondition(predicate: () => boolean, description: string): Promise<void> {
  for (let attempt = 0; attempt < 100; attempt += 1) {
    if (predicate()) {
      return;
    }
    await new Promise<void>((resolve) => setImmediate(resolve));
  }
  assert.fail(`timed out waiting for ${description}`);
}

function createCurrentMessage(overrides: Partial<TestMessage> = {}): TestMessage {
  return {
    chatId: 'shared-conversation',
    content: 'new account message',
    id: 'shared-message',
    senderId: 'principal-b',
    timestamp: 2,
    type: 'text',
    ...overrides,
  };
}

async function assertPreferenceCommandsFenceOldCompletions(): Promise<void> {
  const cases: Array<{
    invoke: (service: ChatService) => Promise<void>;
    label: string;
  }> = [
    { label: 'mark unread', invoke: (service) => service.markAsUnread('shared-conversation') },
    { label: 'pin chat', invoke: (service) => service.pinChat('shared-conversation', true) },
    { label: 'mute chat', invoke: (service) => service.muteChat('shared-conversation', true) },
    { label: 'delete chat', invoke: (service) => service.deleteChat('shared-conversation') },
  ];

  for (const testCase of cases) {
    const request = createDeferred<void>();
    let calls = 0;
    const client = {
      conversations: {
        async updatePreferences() {
          calls += 1;
          return request.promise;
        },
      },
    } as unknown as ImSdkClient;
    const harness = createHarness(client);
    const operation = testCase.invoke(harness.service);
    await waitForCondition(() => calls === 1, `${testCase.label} request`);
    harness.switchSession('principal-b');
    const currentState = {
      isHidden: false,
      isMarkedUnread: false,
      isMuted: false,
      isPinned: false,
      name: 'new account state',
    };
    const currentMessage = createCurrentMessage();
    harness.internals.conversationViewState.set('shared-conversation', currentState);
    harness.internals.setLocalMessages('shared-conversation', [currentMessage]);
    request.resolve(undefined);

    await assert.rejects(operation, /Chat session changed while/u);
    assert.deepEqual(
      {
        messages: harness.internals.localMessages.get('shared-conversation'),
        state: harness.internals.conversationViewState.get('shared-conversation'),
      },
      { messages: [currentMessage], state: currentState },
      `${testCase.label} completion must not mutate the new account`,
    );
  }
}

async function assertMessageCommandsFenceOldCompletions(): Promise<void> {
  const cases: Array<{
    invoke: (service: ChatService) => Promise<void>;
    label: string;
    message: TestMessage;
  }> = [
    {
      label: 'delete message',
      invoke: (service) => service.deleteMessage('shared-conversation', 'shared-message'),
      message: createCurrentMessage(),
    },
    {
      label: 'recall message',
      invoke: (service) => service.recallMessage('shared-conversation', 'shared-message'),
      message: createCurrentMessage(),
    },
    {
      label: 'edit message',
      invoke: (service) => service.editMessage('shared-conversation', 'shared-message', 'old edit'),
      message: createCurrentMessage(),
    },
    {
      label: 'add reaction',
      invoke: (service) => service.addReaction('shared-conversation', 'shared-message', 'ok'),
      message: createCurrentMessage({ reactions: [{ count: 1, emoji: 'ok', hasReacted: false }] }),
    },
    {
      label: 'remove reaction',
      invoke: (service) => service.removeReaction('shared-conversation', 'shared-message', 'ok'),
      message: createCurrentMessage({ reactions: [{ count: 2, emoji: 'ok', hasReacted: true }] }),
    },
  ];

  for (const testCase of cases) {
    const request = createDeferred<void>();
    let calls = 0;
    const invoke = async () => {
      calls += 1;
      return request.promise;
    };
    const client = {
      addReaction: invoke,
      editMessage: invoke,
      messages: { deleteForMe: invoke },
      recallMessage: invoke,
      removeReaction: invoke,
    } as unknown as ImSdkClient;
    const harness = createHarness(client);
    const operation = testCase.invoke(harness.service);
    await waitForCondition(() => calls === 1, `${testCase.label} request`);
    harness.switchSession('principal-b');
    harness.internals.setLocalMessages('shared-conversation', [testCase.message]);
    request.resolve(undefined);

    await assert.rejects(operation, /Chat session changed while/u);
    assert.deepEqual(
      harness.internals.localMessages.get('shared-conversation'),
      [testCase.message],
      `${testCase.label} completion must not mutate the new account message`,
    );
  }
}

async function assertUpdateAndCreateCommandsFenceOldCompletions(): Promise<void> {
  const profileRequest = createDeferred<{ displayName: string }>();
  let profileCalls = 0;
  const updateClient = {
    conversations: {
      async updateProfile() {
        profileCalls += 1;
        return profileRequest.promise;
      },
    },
  } as unknown as ImSdkClient;
  const updateHarness = createHarness(updateClient);
  const updateOperation = updateHarness.service.updateChat('shared-conversation', {
    name: 'old account name',
  });
  await waitForCondition(() => profileCalls === 1, 'update chat profile request');
  updateHarness.switchSession('principal-b');
  const currentState = { name: 'new account name' };
  updateHarness.internals.conversationViewState.set('shared-conversation', currentState);
  profileRequest.resolve({ displayName: 'old server profile' });
  await assert.rejects(updateOperation, /Chat session changed while/u);
  assert.deepEqual(updateHarness.internals.conversationViewState.get('shared-conversation'), currentState);

  const createRequest = createDeferred<{ conversationId: string }>();
  let createCalls = 0;
  let followUpCalls = 0;
  const createClient = {
    conversations: {
      async create() {
        createCalls += 1;
        return createRequest.promise;
      },
      async updatePreferences() {
        followUpCalls += 1;
      },
      async updateProfile() {
        followUpCalls += 1;
        return {};
      },
    },
  } as unknown as ImSdkClient;
  const createHarnessInstance = createHarness(createClient);
  const createOperation = createHarnessInstance.service.createChat({
    avatar: '',
    id: 'client-placeholder',
    name: 'old account group',
    type: 'group',
    unreadCount: 0,
    updatedAt: 1,
  });
  await waitForCondition(() => createCalls === 1, 'create chat request');
  createHarnessInstance.switchSession('principal-b');
  const currentCreatedState = { name: 'new account shared conversation' };
  createHarnessInstance.internals.conversationViewState.set(
    'shared-created-conversation',
    currentCreatedState,
  );
  createRequest.resolve({ conversationId: 'shared-created-conversation' });
  await assert.rejects(createOperation, /Chat session changed while/u);
  assert.equal(followUpCalls, 0, 'an old create completion must not issue follow-up commands');
  assert.deepEqual(
    createHarnessInstance.internals.conversationViewState.get('shared-created-conversation'),
    currentCreatedState,
  );
}

async function assertStartChatCommandsFenceOldCompletions(): Promise<void> {
  const preferenceRequest = createDeferred<void>();
  let preferenceCalls = 0;
  const directClient = {
    conversations: {
      async updatePreferences() {
        preferenceCalls += 1;
        return preferenceRequest.promise;
      },
    },
  } as unknown as ImSdkClient;
  const directHarness = createHarness(directClient);
  const directOperation = directHarness.service.startDirectChat({
    avatar: '',
    conversationId: 'shared-direct-conversation',
    id: 'target-user',
    name: 'old account contact',
  });
  await waitForCondition(() => preferenceCalls === 1, 'restore direct chat preference request');
  directHarness.switchSession('principal-b');
  const currentDirectState = { name: 'new account direct chat' };
  directHarness.internals.conversationViewState.set('shared-direct-conversation', currentDirectState);
  preferenceRequest.resolve(undefined);
  await assert.rejects(directOperation, /Chat session changed while/u);
  assert.deepEqual(
    directHarness.internals.conversationViewState.get('shared-direct-conversation'),
    currentDirectState,
  );

  const bindRequest = createDeferred<{ conversationId: string }>();
  let bindCalls = 0;
  let enterpriseFollowUps = 0;
  const enterpriseClient = {
    conversations: {
      async bindDirectChat() {
        bindCalls += 1;
        return bindRequest.promise;
      },
      async updatePreferences() {
        enterpriseFollowUps += 1;
      },
      async updateProfile() {
        enterpriseFollowUps += 1;
        return {};
      },
    },
  } as unknown as ImSdkClient;
  const enterpriseHarness = createHarness(enterpriseClient);
  const enterpriseOperation = enterpriseHarness.service.startEnterpriseChat({
    avatar: '',
    id: 'enterprise-1',
    name: 'Enterprise',
  });
  await waitForCondition(() => bindCalls === 1, 'enterprise bind request');
  enterpriseHarness.switchSession('principal-b');
  const currentEnterpriseState = { name: 'new account enterprise chat' };
  enterpriseHarness.internals.conversationViewState.set(
    'shared-enterprise-conversation',
    currentEnterpriseState,
  );
  bindRequest.resolve({ conversationId: 'shared-enterprise-conversation' });
  await assert.rejects(enterpriseOperation, /Chat session changed while/u);
  assert.equal(enterpriseFollowUps, 0, 'an old enterprise bind must not issue follow-up commands');
  assert.deepEqual(
    enterpriseHarness.internals.conversationViewState.get('shared-enterprise-conversation'),
    currentEnterpriseState,
  );
}

async function assertAgentInboxScanFencesOldCompletion(): Promise<void> {
  const inboxRequest = createDeferred<{
    items: never[];
    pageInfo: { hasMore: false; mode: 'cursor' };
  }>();
  let createCalls = 0;
  let inboxCalls = 0;
  const client = {
    chat: {
      inbox: {
        async list() {
          inboxCalls += 1;
          return inboxRequest.promise;
        },
      },
    },
    conversations: {
      async createAgentDialog() {
        createCalls += 1;
        return { conversationId: 'shared-agent-conversation' };
      },
      async updatePreferences() {},
      async updateProfile() {
        return {};
      },
    },
  } as unknown as ImSdkClient;
  const harness = createHarness(client);
  const operation = harness.service.startAgentChat({
    avatar: '',
    id: 'agent.market.code',
    name: 'Code Agent',
    welcomeMessage: 'Hello',
  });
  await waitForCondition(() => inboxCalls === 1, 'agent inbox scan');
  harness.switchSession('principal-b');
  inboxRequest.resolve({ items: [], pageInfo: { hasMore: false, mode: 'cursor' } });

  await assert.rejects(operation, /Chat session changed while/u);
  assert.equal(createCalls, 0, 'an old inbox scan must not create an agent dialog for a new account');
}

async function main(): Promise<void> {
  const checks = new Map<string, () => Promise<void>>([
    ['preferences', assertPreferenceCommandsFenceOldCompletions],
    ['messages', assertMessageCommandsFenceOldCompletions],
    ['update-create', assertUpdateAndCreateCommandsFenceOldCompletions],
    ['start-chat', assertStartChatCommandsFenceOldCompletions],
    ['agent-scan', assertAgentInboxScanFencesOldCompletion],
  ]);
  const selectedCheck = process.argv[2];
  if (selectedCheck) {
    const check = checks.get(selectedCheck);
    assert.ok(check, `unknown PC command auth generation check: ${selectedCheck}`);
    await check();
  } else {
    for (const check of checks.values()) {
      await check();
    }
  }
  console.log('sdkwork im pc command auth generation contract passed.');
}

void main();
