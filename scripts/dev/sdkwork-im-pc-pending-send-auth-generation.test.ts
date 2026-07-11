import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';

import { resetDesktopOfflineChatCacheForTests } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/desktopOfflineChatCache.ts';
import type { DesktopOfflinePendingSendRecord } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/desktopOfflineStore.ts';
import {
  claimDesktopPendingSends,
  enqueueDesktopPendingSend,
  listDesktopPendingSends,
  runDesktopPendingSendFlush,
  type DesktopPendingSendFlushResult,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/desktopOfflineSendQueue.ts';
import type { DesktopOfflinePrincipalScope } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/desktopOfflineScope.ts';
import {
  clearAppSdkSessionTokens,
  persistAppSdkSessionTokens,
  type SdkworkChatSession,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/session.ts';
import {
  createSdkworkChatService,
  type ChatService,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService.ts';

interface Deferred<T> {
  promise: Promise<T>;
  reject: (reason?: unknown) => void;
  resolve: (value: T) => void;
}

interface TestTauriBridge {
  __TAURI__?: {
    core: {
      invoke: (command: string, args?: Record<string, unknown>) => Promise<unknown>;
    };
  };
}

interface TestMessage {
  chatId: string;
  content: string;
  id: string;
  senderId: string;
  sendState?: 'failed' | 'pending';
  timestamp: number;
  type: 'text';
}

interface ChatServiceInternals {
  authSessionGeneration: number;
  flushDesktopPendingSendQueue: (generation?: number) => Promise<void>;
  handleAuthSessionChanged: () => void;
  handleConnectionOpen: (generation?: number) => Promise<void>;
  hydrateDesktopPendingSends: (generation?: number) => Promise<void>;
  latestReadSeq: Map<string, number>;
  localMessages: Map<string, TestMessage[]>;
  setLocalMessages: (chatId: string, messages: TestMessage[]) => void;
}

interface MutableSession {
  current: SdkworkChatSession;
}

const pendingPayload = {
  chatId: 'conversation-pending',
  clientMsgId: 'client-pending-1',
  content: 'pending text',
  type: 'text' as const,
};

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
      appId: 'sdkwork-im-pc',
      actorId: principalId,
      actorKind: 'user',
      organizationId: 'organization-1',
      tenantId: 'tenant-1',
      userId: principalId,
    },
  } as SdkworkChatSession;
}

function createScope(principalId: string): DesktopOfflinePrincipalScope {
  return {
    tenantId: 'tenant-1',
    organizationId: 'organization-1',
    principalKind: 'user',
    principalId,
  };
}

function createPendingRow(principalId: string): DesktopOfflinePendingSendRecord {
  return {
    scope: createScope(principalId),
    clientMsgId: `${principalId}-client-message`,
    conversationId: `${principalId}-conversation`,
    payloadJson: JSON.stringify({
      chatId: `${principalId}-conversation`,
      clientMsgId: `${principalId}-client-message`,
      content: `${principalId} pending`,
      type: 'text',
    }),
    createdAt: '2026-07-11T00:00:00.000Z',
    attemptCount: 0,
  };
}

function readChatInternals(service: ChatService): ChatServiceInternals {
  return service as unknown as ChatServiceInternals;
}

function switchSession(session: MutableSession, principalId: string): void {
  session.current = createSession(principalId);
  persistAppSdkSessionTokens(session.current);
}

function createChatServiceHarness(
  session: MutableSession,
  client: ImSdkClient,
): { internals: ChatServiceInternals; service: ChatService } {
  const service = createSdkworkChatService({
    getClient: () => client,
    getSession: () => session.current,
  });
  return { internals: readChatInternals(service), service };
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

async function captureScopeAcrossInitialization(
  operation: () => Promise<unknown>,
  expectedCommand: string,
): Promise<DesktopOfflinePrincipalScope | undefined> {
  const initDeferred = createDeferred<void>();
  let initCalls = 0;
  let observedScope: DesktopOfflinePrincipalScope | undefined;
  (globalThis as TestTauriBridge).__TAURI__ = {
    core: {
      invoke: async (command, args) => {
        if (command === 'sdkwork_im_pc_offline_init') {
          initCalls += 1;
          await initDeferred.promise;
          return true;
        }
        if (command === expectedCommand) {
          const input = command === 'sdkwork_im_pc_offline_enqueue_pending_send'
            ? args?.record as { scope?: DesktopOfflinePrincipalScope } | undefined
            : args as { scope?: DesktopOfflinePrincipalScope } | undefined;
          observedScope = input?.scope;
          return command.includes('list') || command.includes('claim') ? [] : true;
        }
        return true;
      },
    },
  };
  resetDesktopOfflineChatCacheForTests();
  persistAppSdkSessionTokens(createSession('principal-a'));

  const pendingOperation = operation();
  await waitForCondition(() => initCalls === 1, 'pending-send offline initialization');
  persistAppSdkSessionTokens(createSession('principal-b'));
  initDeferred.resolve(undefined);
  await pendingOperation;
  return observedScope;
}

async function assertPendingSendOperationsCaptureScopeBeforeAwait(): Promise<void> {
  const enqueueScope = await captureScopeAcrossInitialization(
    () => enqueueDesktopPendingSend(pendingPayload),
    'sdkwork_im_pc_offline_enqueue_pending_send',
  );
  const listScope = await captureScopeAcrossInitialization(
    () => listDesktopPendingSends(),
    'sdkwork_im_pc_offline_list_pending_sends',
  );
  const claimScope = await captureScopeAcrossInitialization(
    () => claimDesktopPendingSends(),
    'sdkwork_im_pc_offline_claim_pending_sends',
  );

  assert.deepEqual(
    [enqueueScope?.principalId, listScope?.principalId, claimScope?.principalId],
    ['principal-a', 'principal-a', undefined],
    'pending-send writes/reads must retain scope A, while an auth-stale claim must cancel instead of touching B',
  );
}

async function assertNewScopeFlushDoesNotWaitForOldHungFlush(): Promise<void> {
  const claimCounts = new Map<string, number>();
  (globalThis as TestTauriBridge).__TAURI__ = {
    core: {
      invoke: async (command, args) => {
        if (command === 'sdkwork_im_pc_offline_init') {
          return true;
        }
        if (command === 'sdkwork_im_pc_offline_claim_pending_sends') {
          const scope = args?.scope as DesktopOfflinePrincipalScope;
          const count = (claimCounts.get(scope.principalId) ?? 0) + 1;
          claimCounts.set(scope.principalId, count);
          return count === 1 ? [createPendingRow(scope.principalId)] : [];
        }
        if (command === 'sdkwork_im_pc_offline_release_pending_send_claim') {
          return true;
        }
        return true;
      },
    },
  };
  resetDesktopOfflineChatCacheForTests();
  persistAppSdkSessionTokens(createSession('principal-a'));

  const oldFlushDeferred = createDeferred<DesktopPendingSendFlushResult>();
  const newFlushDeferred = createDeferred<DesktopPendingSendFlushResult>();
  let oldFlushCalls = 0;
  let newFlushCalls = 0;
  const oldFlush = runDesktopPendingSendFlush(async () => {
    oldFlushCalls += 1;
    return oldFlushDeferred.promise;
  });
  await waitForCondition(() => oldFlushCalls === 1, 'the old-account pending-send flush');

  persistAppSdkSessionTokens(createSession('principal-b'));
  const newFlush = runDesktopPendingSendFlush(async () => {
    newFlushCalls += 1;
    return newFlushDeferred.promise;
  });
  await waitForCondition(
    () => newFlushCalls === 1,
    'the new-account flush to start without waiting for the old hung promise',
  );

  oldFlushDeferred.resolve({ retryableFailure: true });
  await oldFlush;
  const coalescedNewFlush = runDesktopPendingSendFlush(async () => {
    newFlushCalls += 1;
    return { retryableFailure: true };
  });
  await new Promise<void>((resolve) => setImmediate(resolve));
  assert.equal(
    newFlushCalls,
    1,
    'the old flush finally block must not clear the new scope in-flight owner',
  );

  newFlushDeferred.resolve({ retryableFailure: true });
  await Promise.all([newFlush, coalescedNewFlush]);
  assert.deepEqual({ oldFlushCalls, newFlushCalls }, { oldFlushCalls: 1, newFlushCalls: 1 });
}

async function assertNewGenerationFlushDoesNotWaitForSameScopeHungFlush(): Promise<void> {
  let claimCalls = 0;
  (globalThis as TestTauriBridge).__TAURI__ = {
    core: {
      invoke: async (command) => {
        if (command === 'sdkwork_im_pc_offline_init') {
          return true;
        }
        if (command === 'sdkwork_im_pc_offline_claim_pending_sends') {
          claimCalls += 1;
          return claimCalls <= 2 ? [createPendingRow('principal-a')] : [];
        }
        if (command === 'sdkwork_im_pc_offline_release_pending_send_claim') {
          return true;
        }
        return true;
      },
    },
  };
  resetDesktopOfflineChatCacheForTests();
  persistAppSdkSessionTokens(createSession('principal-a'));

  const oldFlushDeferred = createDeferred<DesktopPendingSendFlushResult>();
  const newFlushDeferred = createDeferred<DesktopPendingSendFlushResult>();
  let oldFlushCalls = 0;
  let newFlushCalls = 0;
  const oldFlush = runDesktopPendingSendFlush(async () => {
    oldFlushCalls += 1;
    return oldFlushDeferred.promise;
  }, { generation: 1 } as { generation: number });
  await waitForCondition(() => oldFlushCalls === 1, 'the old-generation same-scope flush');

  const newFlush = runDesktopPendingSendFlush(async () => {
    newFlushCalls += 1;
    return newFlushDeferred.promise;
  }, { generation: 2 } as { generation: number });
  await waitForCondition(
    () => newFlushCalls === 1,
    'the new generation flush to start without waiting for the same-scope old promise',
  );

  oldFlushDeferred.resolve({ retryableFailure: true });
  await oldFlush;
  const coalescedNewFlush = runDesktopPendingSendFlush(async () => {
    newFlushCalls += 1;
    return { retryableFailure: true };
  }, { generation: 2 } as { generation: number });
  await new Promise<void>((resolve) => setImmediate(resolve));
  assert.equal(newFlushCalls, 1, 'old generation cleanup must not release the new generation slot');

  newFlushDeferred.resolve({ retryableFailure: true });
  await Promise.all([newFlush, coalescedNewFlush]);
}

async function assertOldConnectionHydrationDoesNotWriteNewSessionState(): Promise<void> {
  const listDeferred = createDeferred<DesktopOfflinePendingSendRecord[]>();
  let listCalls = 0;
  (globalThis as TestTauriBridge).__TAURI__ = {
    core: {
      invoke: async (command) => {
        if (command === 'sdkwork_im_pc_offline_init') {
          return true;
        }
        if (command === 'sdkwork_im_pc_offline_list_pending_sends') {
          listCalls += 1;
          return listDeferred.promise;
        }
        return true;
      },
    },
  };
  resetDesktopOfflineChatCacheForTests();
  const session = { current: createSession('principal-a') };
  persistAppSdkSessionTokens(session.current);
  const fakeClient = { conversations: {} } as unknown as ImSdkClient;
  const { internals } = createChatServiceHarness(session, fakeClient);
  const oldGeneration = internals.authSessionGeneration;

  const hydration = internals.hydrateDesktopPendingSends(oldGeneration);
  await waitForCondition(() => listCalls === 1, 'the old-account pending-send hydration read');
  switchSession(session, 'principal-b');
  internals.handleAuthSessionChanged();
  listDeferred.resolve([createPendingRow('principal-a')]);
  await hydration;

  assert.equal(
    internals.localMessages.size,
    0,
    'an old pending-send hydration completion must not populate the new account message cache',
  );
}

async function assertOldConnectionOpenStopsBeforeFlush(): Promise<void> {
  const listDeferred = createDeferred<DesktopOfflinePendingSendRecord[]>();
  let claimCalls = 0;
  let listCalls = 0;
  (globalThis as TestTauriBridge).__TAURI__ = {
    core: {
      invoke: async (command) => {
        if (command === 'sdkwork_im_pc_offline_init') {
          return true;
        }
        if (command === 'sdkwork_im_pc_offline_list_pending_sends') {
          listCalls += 1;
          return listDeferred.promise;
        }
        if (command === 'sdkwork_im_pc_offline_claim_pending_sends') {
          claimCalls += 1;
          return [];
        }
        return true;
      },
    },
  };
  resetDesktopOfflineChatCacheForTests();
  const session = { current: createSession('principal-a') };
  persistAppSdkSessionTokens(session.current);
  const fakeClient = { conversations: {} } as unknown as ImSdkClient;
  const { internals } = createChatServiceHarness(session, fakeClient);
  const oldGeneration = internals.authSessionGeneration;

  const connectionOpen = internals.handleConnectionOpen(oldGeneration);
  await waitForCondition(() => listCalls === 1, 'the old connection-open hydration');
  switchSession(session, 'principal-b');
  internals.handleAuthSessionChanged();
  listDeferred.resolve([]);
  await connectionOpen;

  assert.equal(claimCalls, 0, 'an old connection-open continuation must stop before queue flush');
}

async function assertOldPendingFlushCompletionDoesNotWriteNewSessionState(): Promise<void> {
  const postDeferred = createDeferred<{ messageId: string; messageSeq: number }>();
  let claimCalls = 0;
  let postCalls = 0;
  (globalThis as TestTauriBridge).__TAURI__ = {
    core: {
      invoke: async (command, args) => {
        if (command === 'sdkwork_im_pc_offline_init') {
          return true;
        }
        if (command === 'sdkwork_im_pc_offline_claim_pending_sends') {
          const scope = args?.scope as DesktopOfflinePrincipalScope;
          claimCalls += 1;
          return claimCalls === 1 ? [createPendingRow(scope.principalId)] : [];
        }
        if (command === 'sdkwork_im_pc_offline_delete_pending_send') {
          return true;
        }
        if (command === 'sdkwork_im_pc_offline_release_pending_send_claim') {
          return true;
        }
        return true;
      },
    },
  };
  resetDesktopOfflineChatCacheForTests();
  const session = { current: createSession('principal-a') };
  persistAppSdkSessionTokens(session.current);
  const fakeClient = {
    conversations: {
      async postText() {
        postCalls += 1;
        return postDeferred.promise;
      },
    },
  } as unknown as ImSdkClient;
  const { internals } = createChatServiceHarness(session, fakeClient);
  const oldGeneration = internals.authSessionGeneration;
  const oldConversationId = 'principal-a-conversation';
  internals.setLocalMessages(oldConversationId, [{
    chatId: oldConversationId,
    content: 'principal-a pending',
    id: 'principal-a-client-message',
    senderId: 'principal-a',
    sendState: 'pending',
    timestamp: 1,
    type: 'text',
  }]);

  const oldFlush = internals.flushDesktopPendingSendQueue(oldGeneration);
  await waitForCondition(() => postCalls === 1, 'the old-account pending post');
  switchSession(session, 'principal-b');
  internals.handleAuthSessionChanged();
  const newConversationId = 'principal-b-conversation';
  const newMessage: TestMessage = {
    chatId: newConversationId,
    content: 'principal-b current',
    id: 'principal-b-message',
    senderId: 'principal-b',
    timestamp: 2,
    type: 'text',
  };
  internals.setLocalMessages(newConversationId, [newMessage]);
  internals.latestReadSeq.set(oldConversationId, 5);
  postDeferred.resolve({ messageId: 'old-server-message', messageSeq: 99 });
  await oldFlush;

  assert.deepEqual(
    {
      newMessages: internals.localMessages.get(newConversationId),
      oldMessages: internals.localMessages.get(oldConversationId),
      oldReadSeqInNewGeneration: internals.latestReadSeq.get(oldConversationId),
    },
    {
      newMessages: [newMessage],
      oldMessages: undefined,
      oldReadSeqInNewGeneration: 5,
    },
    'an old queue post completion must not mutate any new-generation cache state',
  );
}

async function assertOldSendFailureDoesNotEnqueueForNewScope(): Promise<void> {
  const postDeferred = createDeferred<{ messageId: string; messageSeq: number }>();
  let enqueueCalls = 0;
  let enqueuedScope: DesktopOfflinePrincipalScope | undefined;
  let postCalls = 0;
  (globalThis as TestTauriBridge).__TAURI__ = {
    core: {
      invoke: async (command, args) => {
        if (command === 'sdkwork_im_pc_offline_init') {
          return true;
        }
        if (command === 'sdkwork_im_pc_offline_enqueue_pending_send') {
          enqueueCalls += 1;
          enqueuedScope = (args?.record as { scope?: DesktopOfflinePrincipalScope } | undefined)?.scope;
          return true;
        }
        return true;
      },
    },
  };
  resetDesktopOfflineChatCacheForTests();
  const session = { current: createSession('principal-a') };
  persistAppSdkSessionTokens(session.current);
  const fakeClient = {
    conversations: {
      async postText() {
        postCalls += 1;
        return postDeferred.promise;
      },
    },
  } as unknown as ImSdkClient;
  const { internals, service } = createChatServiceHarness(session, fakeClient);
  const sendPromise = service.sendMessage('principal-a-conversation', 'pending old send');
  const rejectedSend = assert.rejects(
    sendPromise,
    /Chat session changed while sending message\./u,
  );
  await waitForCondition(() => postCalls === 1, 'the old-account message request');

  switchSession(session, 'principal-b');
  internals.handleAuthSessionChanged();
  postDeferred.reject(new TypeError('failed to fetch'));
  await rejectedSend;

  assert.deepEqual(
    {
      enqueueCalls,
      enqueuedPrincipal: enqueuedScope?.principalId,
      localMessageCount: internals.localMessages.size,
    },
    { enqueueCalls: 0, enqueuedPrincipal: undefined, localMessageCount: 0 },
    'an old send failure must not enter the new principal queue or local pending cache',
  );
}

async function assertOldNonRetryableFlushCleanupDoesNotWriteNewSessionState(): Promise<void> {
  const deleteDeferred = createDeferred<boolean>();
  let claimCalls = 0;
  let deleteCalls = 0;
  (globalThis as TestTauriBridge).__TAURI__ = {
    core: {
      invoke: async (command, args) => {
        if (command === 'sdkwork_im_pc_offline_init') {
          return true;
        }
        if (command === 'sdkwork_im_pc_offline_claim_pending_sends') {
          const scope = args?.scope as DesktopOfflinePrincipalScope;
          claimCalls += 1;
          return claimCalls === 1 ? [createPendingRow(scope.principalId)] : [];
        }
        if (command === 'sdkwork_im_pc_offline_delete_pending_send') {
          deleteCalls += 1;
          return deleteDeferred.promise;
        }
        return true;
      },
    },
  };
  resetDesktopOfflineChatCacheForTests();
  const session = { current: createSession('principal-a') };
  persistAppSdkSessionTokens(session.current);
  const fakeClient = {
    conversations: {
      async postText() {
        throw new Error('non-retryable rejection');
      },
    },
  } as unknown as ImSdkClient;
  const { internals } = createChatServiceHarness(session, fakeClient);
  const oldGeneration = internals.authSessionGeneration;
  const sharedConversationId = 'principal-a-conversation';
  const sharedMessageId = 'principal-a-client-message';

  const oldFlush = internals.flushDesktopPendingSendQueue(oldGeneration);
  await waitForCondition(() => deleteCalls === 1, 'the old non-retryable pending cleanup');
  switchSession(session, 'principal-b');
  internals.handleAuthSessionChanged();
  const newGenerationMessage: TestMessage = {
    chatId: sharedConversationId,
    content: 'new generation message with a colliding client id',
    id: sharedMessageId,
    senderId: 'principal-b',
    sendState: 'pending',
    timestamp: 2,
    type: 'text',
  };
  internals.setLocalMessages(sharedConversationId, [newGenerationMessage]);
  deleteDeferred.resolve(true);
  await oldFlush;

  assert.deepEqual(
    internals.localMessages.get(sharedConversationId),
    [newGenerationMessage],
    'an old non-retryable cleanup completion must not mark a new-generation message as failed',
  );
}

async function main(): Promise<void> {
  const checks = new Map<string, () => Promise<void>>([
    ['scope-before-await', assertPendingSendOperationsCaptureScopeBeforeAwait],
    ['cross-scope-flush', assertNewScopeFlushDoesNotWaitForOldHungFlush],
    ['same-scope-generation-flush', assertNewGenerationFlushDoesNotWaitForSameScopeHungFlush],
    ['connection-hydration-fence', assertOldConnectionHydrationDoesNotWriteNewSessionState],
    ['connection-open-fence', assertOldConnectionOpenStopsBeforeFlush],
    ['connection-flush-fence', assertOldPendingFlushCompletionDoesNotWriteNewSessionState],
    ['send-failure-fence', assertOldSendFailureDoesNotEnqueueForNewScope],
    ['non-retryable-cleanup-fence', assertOldNonRetryableFlushCleanupDoesNotWriteNewSessionState],
  ]);
  const selectedCheck = process.argv[2];
  try {
    if (selectedCheck) {
      const check = checks.get(selectedCheck);
      assert.ok(check, `unknown pending-send auth generation check: ${selectedCheck}`);
      await check();
    } else {
      for (const check of checks.values()) {
        await check();
      }
    }
  } finally {
    clearAppSdkSessionTokens();
    delete (globalThis as TestTauriBridge).__TAURI__;
    resetDesktopOfflineChatCacheForTests();
  }
  console.log('sdkwork im pc pending-send auth generation contract passed.');
}

void main();
