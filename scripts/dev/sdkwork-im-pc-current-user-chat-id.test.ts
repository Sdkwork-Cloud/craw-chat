import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkContactService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ContactService';
import {
  applyAppSdkSessionTokens,
  clearAppSdkSessionTokens,
  normalizeSdkworkChatSessionUser,
  readAppSdkSessionTokens,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/session';

const storage = new Map<string, string>();
const sessionStorageMap = new Map<string, string>();

(globalThis as unknown as {
  window: Pick<Window, 'dispatchEvent' | 'localStorage' | 'sessionStorage'>;
}).window = {
  dispatchEvent() {
    return true;
  },
  localStorage: {
    clear() {
      storage.clear();
    },
    getItem(key: string) {
      return storage.get(key) ?? null;
    },
    key(index: number) {
      return Array.from(storage.keys())[index] ?? null;
    },
    get length() {
      return storage.size;
    },
    removeItem(key: string) {
      storage.delete(key);
    },
    setItem(key: string, value: string) {
      storage.set(key, value);
    },
  } as Storage,
  sessionStorage: {
    clear() {
      sessionStorageMap.clear();
    },
    getItem(key: string) {
      return sessionStorageMap.get(key) ?? null;
    },
    key(index: number) {
      return Array.from(sessionStorageMap.keys())[index] ?? null;
    },
    get length() {
      return sessionStorageMap.size;
    },
    removeItem(key: string) {
      sessionStorageMap.delete(key);
    },
    setItem(key: string, value: string) {
      sessionStorageMap.set(key, value);
    },
  } as Storage,
};

const calls: Array<{ method: string; params?: Record<string, unknown> }> = [];

const fakeClient = {
  chat: {
    contacts: {
      async list() {
        calls.push({ method: 'chat.contacts.list' });
        return { items: [], hasMore: false };
      },
    },
  },
  social: {
    contacts: {
      async list() {
        calls.push({ method: 'social.contacts.list' });
        return { items: [], hasMore: false };
      },
    },
    users: {
      async list(params: Record<string, unknown>) {
        calls.push({ method: 'social.users.list', params });
        if (params.q === 'current-user' || params.q === 'current-chat-id') {
          return {
            items: [
              {
                userId: 'current-user',
                chatId: 'current-chat-id',
                displayName: 'Current User',
                avatarUrl: 'https://example.com/current.png',
                email: 'current@example.com',
                relationshipState: 'self',
              },
            ],
            hasMore: false,
          };
        }
        return { items: [], hasMore: false };
      },
    },
  },
} as unknown as ImSdkClient;

const profileOnlyFakeClient = {
  chat: {
    contacts: {
      async list() {
        calls.push({ method: 'chat.contacts.list' });
        throw new Error('contacts unavailable');
      },
    },
  },
  social: fakeClient.social,
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  clearAppSdkSessionTokens();

  assert.deepEqual(
    normalizeSdkworkChatSessionUser({
      userId: 'current-user',
      chat_id: 'current-chat-id',
      displayName: 'Current User',
    })?.chatId,
    'current-chat-id',
    'session normalization must preserve chat_id from login payloads as local chatId',
  );

  const storedSession = applyAppSdkSessionTokens({
    accessToken: 'access-token',
    authToken: 'auth-token',
    context: {
      appId: 'sdkwork-im-pc',
      tenantId: '100001',
      userId: 'current-user',
      sessionId: 'session-1',
      environment: 'dev',
      deploymentMode: 'saas',
      authLevel: 'password',
      dataScope: [],
      permissionScope: [],
    },
    sessionId: 'session-1',
    user: {
      userId: 'current-user',
      displayName: 'Current User',
      chat_id: 'current-chat-id',
    } as never,
  });
  assert.equal(storedSession.user?.chatId, 'current-chat-id', 'login commit must store chatId locally');
  assert.equal(readAppSdkSessionTokens()?.user?.chatId, 'current-chat-id', 'persisted session must retain chatId');

  clearAppSdkSessionTokens();
  applyAppSdkSessionTokens({
    accessToken: 'snake-access-token',
    authToken: 'snake-auth-token',
    context: {
      app_id: 'sdkwork-im-pc',
      tenant_id: 'tenant-snake',
      user_id: 'snake-user',
      session_id: 'snake-session',
      environment: 'dev',
      deployment_mode: 'saas',
      auth_level: 'password',
      data_scope: ['tenant:tenant-snake'],
      permission_scope: ['chat:read'],
    } as never,
    user: {
      userId: 'snake-user',
      displayName: 'Snake User',
    },
  });
  const snakeCaseSession = readAppSdkSessionTokens();
  assert.equal(
    snakeCaseSession?.context?.userId,
    'snake-user',
    'login commit must normalize snake_case IAM context into the local app session',
  );
  assert.equal(
    snakeCaseSession?.sessionId,
    'snake-session',
    'login commit must normalize snake_case session_id into the local app session id',
  );

  clearAppSdkSessionTokens();
  applyAppSdkSessionTokens({
    accessToken: 'access-token',
    authToken: 'auth-token',
    context: {
      appId: 'sdkwork-im-pc',
      tenantId: '100001',
      userId: 'current-user',
      sessionId: 'session-1',
      environment: 'dev',
      deploymentMode: 'saas',
      authLevel: 'password',
      dataScope: [],
      permissionScope: [],
    },
    sessionId: 'session-1',
    user: {
      userId: 'current-user',
      displayName: 'Current User',
    },
  });

  const service = createSdkworkContactService(() => fakeClient);
  assert.equal(service.getCurrentUser().chatId, undefined, 'test setup starts with no local chatId');

  const syncedUser = await service.getUserById('current-user');
  assert.equal(syncedUser?.chatId, 'current-chat-id', 'current profile sync must include self search chatId');
  assert.equal(service.getCurrentUser().chatId, 'current-chat-id', 'current profile sync must update service current user cache');
  assert.equal(readAppSdkSessionTokens()?.user?.chatId, 'current-chat-id', 'current profile sync must write chatId back to local session');
  assert.deepEqual(
    calls.filter((call) => call.method === 'social.users.list').at(-1),
    { method: 'social.users.list', params: { q: 'current-user', limit: 20 } },
    'current profile sync must use the generated IM SDK social user search endpoint',
  );

  const serviceSearchResults = await service.searchContacts('current-user');
  assert.deepEqual(
    serviceSearchResults,
    [],
    'add-friend search must still hide the current user after current profile sync is fixed',
  );

  clearAppSdkSessionTokens();
  applyAppSdkSessionTokens({
    accessToken: 'access-token',
    authToken: 'auth-token',
    context: {
      appId: 'sdkwork-im-pc',
      tenantId: '100001',
      userId: 'current-user',
      sessionId: 'session-1',
      environment: 'dev',
      deploymentMode: 'saas',
      authLevel: 'password',
      dataScope: [],
      permissionScope: [],
    },
    sessionId: 'session-1',
    user: {
      userId: 'current-user',
      displayName: 'Current User',
    },
  });
  calls.length = 0;
  const profileOnlyService = createSdkworkContactService(() => profileOnlyFakeClient);
  const profileOnlyUser = await profileOnlyService.getUserById('current-user');
  assert.equal(
    profileOnlyUser?.chatId,
    'current-chat-id',
    'current profile sync must not depend on the contacts list endpoint',
  );
  assert.deepEqual(
    calls,
    [
      { method: 'social.users.list', params: { q: 'current-user', limit: 20 } },
    ],
    'current profile sync should use the direct self profile lookup before contact-list hydration',
  );

  console.log('sdkwork-im-pc current user chat id contract passed');
}

void main();
