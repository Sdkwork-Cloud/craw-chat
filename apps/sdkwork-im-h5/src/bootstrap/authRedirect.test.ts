import assert from 'node:assert/strict';
import test from 'node:test';

import {
  clearImH5AuthRedirectTarget,
  IM_H5_AUTH_REDIRECT_STORAGE_KEY,
  isSafeInternalTarget,
  persistImH5AuthRedirectTarget,
  resolveImH5AuthRedirectTarget,
  type ImH5AuthRedirectStorageLike,
} from './authRedirect';

function createStorage(): ImH5AuthRedirectStorageLike {
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

test('resolves the redirect query parameter over the persisted target', () => {
  const storage = createStorage();
  persistImH5AuthRedirectTarget('/chat/from-storage', storage);
  assert.equal(
    resolveImH5AuthRedirectTarget('?redirect=%2Fchat%2Ffrom-query', storage),
    '/chat/from-query',
  );
});

test('falls back to the persisted target for the WeChat authorization round trip', () => {
  const storage = createStorage();
  persistImH5AuthRedirectTarget('/chat/from-storage', storage);
  // The OAuth callback URL carries code/state, not the redirect parameter.
  assert.equal(resolveImH5AuthRedirectTarget('?code=wx-1&state=s1', storage), '/chat/from-storage');
});

test('lands on the app home when neither source provides a safe target', () => {
  const storage = createStorage();
  assert.equal(resolveImH5AuthRedirectTarget('', storage), '/');
  assert.equal(resolveImH5AuthRedirectTarget('?redirect=%2Fchat%2Flist', storage), '/chat/list');
  clearImH5AuthRedirectTarget(storage);
  assert.equal(resolveImH5AuthRedirectTarget('', storage), '/');
});

test('rejects open-redirect and auth-loop targets', () => {
  assert.equal(isSafeInternalTarget('/chat/list'), true);
  assert.equal(isSafeInternalTarget('/chat/list?tab=2'), true);
  assert.equal(isSafeInternalTarget('https://evil.example.com'), false);
  assert.equal(isSafeInternalTarget('//evil.example.com'), false);
  assert.equal(isSafeInternalTarget('javascript:alert(1)'), false);
  assert.equal(isSafeInternalTarget('/auth/login?redirect=%2F'), false);
  assert.equal(isSafeInternalTarget('/auth'), false);
  assert.equal(isSafeInternalTarget(''), false);
  assert.equal(isSafeInternalTarget('chat/list'), false);
});
