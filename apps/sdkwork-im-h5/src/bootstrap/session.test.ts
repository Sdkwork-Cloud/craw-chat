import assert from 'node:assert/strict';
import test from 'node:test';

import {
  createImH5SessionBridge,
  readImH5PersistedSession,
  restoreAndValidateImH5Session,
  type ImH5SessionStorageLike,
} from './session';

function createStorage(): ImH5SessionStorageLike {
  const values = new Map<string, string>();
  return {
    getItem: (key) => values.get(key) ?? null,
    removeItem: (key) => {
      values.delete(key);
    },
    setItem: (key, value) => {
      values.set(key, value);
    },
  };
}

test('persists and restores one complete rotating IAM session', () => {
  const storage = createStorage();
  const bridge = createImH5SessionBridge({ storage });

  bridge.commitSession({
    accessToken: 'access-token',
    authToken: 'auth-token',
    refreshToken: 'refresh-token',
  });

  assert.deepEqual(readImH5PersistedSession(storage), {
    accessToken: 'access-token',
    authToken: 'auth-token',
    refreshToken: 'refresh-token',
  });
});

test('rejects partial token pairs without changing persisted state', () => {
  const storage = createStorage();
  const bridge = createImH5SessionBridge({ storage });

  assert.throws(
    () => bridge.commitSession({ accessToken: 'access-token' }),
    /complete IAM dual-token session/,
  );
  assert.equal(readImH5PersistedSession(storage), null);
});

test('clears persisted state and notifies session observers', () => {
  const storage = createStorage();
  const notifications: unknown[] = [];
  const bridge = createImH5SessionBridge({
    notifySessionChanged: (session) => notifications.push(session),
    storage,
  });
  bridge.commitSession({
    accessToken: 'access-token',
    authToken: 'auth-token',
  });

  bridge.clearSession();

  assert.equal(readImH5PersistedSession(storage), null);
  assert.deepEqual(notifications, [null]);
});

test('validates a hydrated dual-token session through appbase IAM', async () => {
  let retrieveCount = 0;
  let clearCount = 0;

  const authenticated = await restoreAndValidateImH5Session({
    clearSession: async () => {
      clearCount += 1;
    },
    hydrateTokenManager: async () => ({
      accessToken: 'access-token',
      authToken: 'auth-token',
    }),
    retrieveCurrentSession: async () => {
      retrieveCount += 1;
    },
  });

  assert.equal(authenticated, true);
  assert.equal(retrieveCount, 1);
  assert.equal(clearCount, 0);
});

test('clears an incomplete hydrated session before current-session retrieval', async () => {
  let retrieveCount = 0;
  let clearCount = 0;

  const authenticated = await restoreAndValidateImH5Session({
    clearSession: async () => {
      clearCount += 1;
    },
    hydrateTokenManager: async () => ({ accessToken: 'access-token' }),
    retrieveCurrentSession: async () => {
      retrieveCount += 1;
    },
  });

  assert.equal(authenticated, false);
  assert.equal(retrieveCount, 0);
  assert.equal(clearCount, 1);
});

test('keeps a hydrated session when current-session validation fails transiently', async () => {
  let clearCount = 0;

  const authenticated = await restoreAndValidateImH5Session({
    clearSession: async () => {
      clearCount += 1;
    },
    hydrateTokenManager: async () => ({
      accessToken: 'access-token',
      authToken: 'auth-token',
    }),
    retrieveCurrentSession: async () => {
      throw new Error('network error: fetch failed');
    },
  });

  assert.equal(authenticated, true);
  assert.equal(clearCount, 0);
});

test('keeps a hydrated session when current-session validation hits a server 5xx', async () => {
  let clearCount = 0;

  const authenticated = await restoreAndValidateImH5Session({
    clearSession: async () => {
      clearCount += 1;
    },
    hydrateTokenManager: async () => ({
      accessToken: 'access-token',
      authToken: 'auth-token',
    }),
    retrieveCurrentSession: async () => {
      throw Object.assign(new Error('HTTP 503: gateway unavailable'), { httpStatus: 503 });
    },
  });

  assert.equal(authenticated, true);
  assert.equal(clearCount, 0);
});

test('clears a hydrated session definitively rejected with HTTP 401', async () => {
  let clearCount = 0;

  const authenticated = await restoreAndValidateImH5Session({
    clearSession: async () => {
      clearCount += 1;
    },
    hydrateTokenManager: async () => ({
      accessToken: 'access-token',
      authToken: 'auth-token',
    }),
    retrieveCurrentSession: async () => {
      throw Object.assign(new Error('HTTP 401: unauthorized'), { httpStatus: 401 });
    },
  });

  assert.equal(authenticated, false);
  assert.equal(clearCount, 1);
});

test('clears a hydrated session rejected by appbase IAM', async () => {
  let clearCount = 0;

  const authenticated = await restoreAndValidateImH5Session({
    clearSession: async () => {
      clearCount += 1;
    },
    hydrateTokenManager: async () => ({
      accessToken: 'access-token',
      authToken: 'auth-token',
    }),
    retrieveCurrentSession: async () => {
      throw new Error('Unauthorized');
    },
  });

  assert.equal(authenticated, false);
  assert.equal(clearCount, 1);
});

test('clears a hydrated session when token manager hydration fails', async () => {
  let clearCount = 0;
  let retrieveCount = 0;

  const authenticated = await restoreAndValidateImH5Session({
    clearSession: async () => {
      clearCount += 1;
    },
    hydrateTokenManager: async () => {
      throw new Error('storage unavailable');
    },
    retrieveCurrentSession: async () => {
      retrieveCount += 1;
    },
  });

  assert.equal(authenticated, false);
  assert.equal(retrieveCount, 0);
  assert.equal(clearCount, 1);
});
