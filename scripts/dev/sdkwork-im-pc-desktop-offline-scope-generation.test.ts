import assert from 'node:assert/strict';

import {
  ensureDesktopOfflineChatCache,
  loadDesktopOfflineChats,
  persistDesktopOfflineChats,
  resetDesktopOfflineChatCacheForTests,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/desktopOfflineChatCache.ts';
import {
  resolveDesktopOfflinePrincipalScope,
  type DesktopOfflinePrincipalScope,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/desktopOfflineScope.ts';
import {
  clearAppSdkSessionTokens,
  persistAppSdkSessionTokens,
  type SdkworkChatSession,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/session.ts';

interface Deferred<T> {
  promise: Promise<T>;
  resolve: (value: T) => void;
}

interface TestTauriBridge {
  __TAURI__?: {
    core: {
      invoke: (command: string, args?: Record<string, unknown>) => Promise<unknown>;
    };
  };
}

function createDeferred<T>(): Deferred<T> {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((resolvePromise) => {
    resolve = resolvePromise;
  });
  return { promise, resolve };
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

async function waitForCondition(predicate: () => boolean, description: string): Promise<void> {
  for (let attempt = 0; attempt < 100; attempt += 1) {
    if (predicate()) {
      return;
    }
    await new Promise<void>((resolve) => setImmediate(resolve));
  }
  assert.fail(`timed out waiting for ${description}`);
}

async function assertPersistCapturesScopeBeforeInitializationAwait(): Promise<void> {
  const initDeferred = createDeferred<void>();
  let initCalls = 0;
  let persistedScope: DesktopOfflinePrincipalScope | undefined;
  const bridge = globalThis as TestTauriBridge;
  bridge.__TAURI__ = {
    core: {
      invoke: async (command, args) => {
        if (command === 'sdkwork_im_pc_offline_init') {
          initCalls += 1;
          await initDeferred.promise;
        }
        if (command === 'sdkwork_im_pc_offline_upsert_conversations') {
          const records = args?.records as Array<{ scope: DesktopOfflinePrincipalScope }>;
          persistedScope = records[0]?.scope;
          return records.length;
        }
        return true;
      },
    },
  };
  resetDesktopOfflineChatCacheForTests();
  persistAppSdkSessionTokens(createSession('principal-a'));
  assert.equal(resolveDesktopOfflinePrincipalScope()?.principalId, 'principal-a');

  const persistPromise = persistDesktopOfflineChats([{
    id: 'conversation-a',
    name: 'Conversation A',
    type: 'single',
    unreadCount: 0,
    updatedAt: 1,
  }]);
  await waitForCondition(() => initCalls === 1, 'desktop offline initialization');
  persistAppSdkSessionTokens(createSession('principal-b'));
  initDeferred.resolve(undefined);
  await persistPromise;

  assert.equal(
    persistedScope?.principalId,
    'principal-a',
    'a persist operation must retain the principal scope captured when it started',
  );
}

async function assertConcurrentInitializationIsShared(): Promise<void> {
  const initDeferred = createDeferred<void>();
  let initCalls = 0;
  const bridge = globalThis as TestTauriBridge;
  bridge.__TAURI__ = {
    core: {
      invoke: async (command) => {
        if (command === 'sdkwork_im_pc_offline_init') {
          initCalls += 1;
          await initDeferred.promise;
        }
        return true;
      },
    },
  };
  resetDesktopOfflineChatCacheForTests();

  const firstInitialization = ensureDesktopOfflineChatCache();
  const secondInitialization = ensureDesktopOfflineChatCache();
  await waitForCondition(() => initCalls > 0, 'the shared desktop cache initialization');
  await new Promise<void>((resolve) => setImmediate(resolve));
  assert.equal(initCalls, 1, 'concurrent cache callers must share one SQLite initialization');
  initDeferred.resolve(undefined);
  assert.deepEqual(await Promise.all([firstInitialization, secondInitialization]), [true, true]);
}

async function assertFailedSharedInitializationCanRetry(): Promise<void> {
  const failedInitDeferred = createDeferred<void>();
  let initCalls = 0;
  let shouldFail = true;
  const bridge = globalThis as TestTauriBridge;
  bridge.__TAURI__ = {
    core: {
      invoke: async (command) => {
        if (command === 'sdkwork_im_pc_offline_init') {
          initCalls += 1;
          if (shouldFail) {
            await failedInitDeferred.promise;
            throw new Error('offline init failed');
          }
        }
        return true;
      },
    },
  };
  resetDesktopOfflineChatCacheForTests();

  const firstInitialization = ensureDesktopOfflineChatCache();
  const secondInitialization = ensureDesktopOfflineChatCache();
  await waitForCondition(() => initCalls > 0, 'the failing shared desktop cache initialization');
  failedInitDeferred.resolve(undefined);
  const failedResults = await Promise.allSettled([firstInitialization, secondInitialization]);
  assert.deepEqual(failedResults.map((result) => result.status), ['rejected', 'rejected']);
  assert.equal(initCalls, 1, 'concurrent callers must observe the same initialization failure');

  shouldFail = false;
  assert.equal(await ensureDesktopOfflineChatCache(), true);
  assert.equal(initCalls, 2, 'a failed shared initialization must release its slot for retry');
}

async function assertLoadCapturesScopeBeforeEnsureAwait(): Promise<void> {
  let loadedScope: DesktopOfflinePrincipalScope | undefined;
  const bridge = globalThis as TestTauriBridge;
  bridge.__TAURI__ = {
    core: {
      invoke: async (command, args) => {
        if (command === 'sdkwork_im_pc_offline_list_conversations') {
          loadedScope = args?.scope as DesktopOfflinePrincipalScope;
          return [];
        }
        return true;
      },
    },
  };
  resetDesktopOfflineChatCacheForTests();
  await ensureDesktopOfflineChatCache();
  persistAppSdkSessionTokens(createSession('principal-a'));
  assert.equal(resolveDesktopOfflinePrincipalScope()?.principalId, 'principal-a');

  const loadPromise = loadDesktopOfflineChats(20);
  persistAppSdkSessionTokens(createSession('principal-b'));
  await loadPromise;

  assert.equal(
    loadedScope?.principalId,
    'principal-a',
    'a load operation must retain the principal scope captured when it started',
  );
}

async function main(): Promise<void> {
  const checks = new Map<string, () => Promise<void>>([
    ['init-dedup', assertConcurrentInitializationIsShared],
    ['init-failure-retry', assertFailedSharedInitializationCanRetry],
    ['persist-scope', assertPersistCapturesScopeBeforeInitializationAwait],
    ['load-scope', assertLoadCapturesScopeBeforeEnsureAwait],
  ]);
  const selectedCheck = process.argv[2];
  try {
    if (selectedCheck) {
      const check = checks.get(selectedCheck);
      assert.ok(check, `unknown desktop offline scope check: ${selectedCheck}`);
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
  console.log('sdkwork im pc desktop offline scope generation contract passed.');
}

void main();
