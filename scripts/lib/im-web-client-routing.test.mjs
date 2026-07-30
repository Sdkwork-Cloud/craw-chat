import assert from 'node:assert/strict';
import test from 'node:test';

import {
  IM_WEB_CLIENTS,
  imWebClientFallbackOrder,
  isCanonicalImApiPath,
  preferredImWebClient,
  resolveAvailableImWebClient,
} from './im-web-client-routing.mjs';

const MOBILE_USER_AGENTS = [
  'Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X) AppleWebKit/605.1.15 Mobile/15E148',
  'Mozilla/5.0 (Linux; Android 15; Pixel 9 Pro) AppleWebKit/537.36 Chrome/136.0 Mobile Safari/537.36',
  'Mozilla/5.0 (iPad; CPU OS 18_0 like Mac OS X) AppleWebKit/605.1.15 Mobile/15E148',
];

test('selects H5 for mobile user agents and PC for desktop or missing user agents', () => {
  for (const userAgent of MOBILE_USER_AGENTS) {
    assert.equal(preferredImWebClient(userAgent), IM_WEB_CLIENTS.H5);
  }
  assert.equal(
    preferredImWebClient('Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/136.0'),
    IM_WEB_CLIENTS.PC,
  );
  assert.equal(preferredImWebClient(undefined), IM_WEB_CLIENTS.PC);
});

test('falls back in both directions when the preferred renderer is unavailable', () => {
  assert.equal(
    resolveAvailableImWebClient({
      availableClients: [IM_WEB_CLIENTS.PC],
      userAgent: MOBILE_USER_AGENTS[0],
    }),
    IM_WEB_CLIENTS.PC,
  );
  assert.equal(
    resolveAvailableImWebClient({
      availableClients: [IM_WEB_CLIENTS.H5],
      userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 14_5)',
    }),
    IM_WEB_CLIENTS.H5,
  );
  assert.equal(resolveAvailableImWebClient({ availableClients: [] }), undefined);
  assert.deepEqual(imWebClientFallbackOrder(MOBILE_USER_AGENTS[0]), ['h5', 'pc']);
});

test('recognizes canonical API paths before renderer routing', () => {
  for (const path of [
    '/app/v3/api/auth/sessions',
    '/backend/v3/api/admin/users',
    '/im/v3/api/realtime/ws?token=redacted',
    '/api/config/modules',
    '/healthz',
    '/openapi.json',
  ]) {
    assert.equal(isCanonicalImApiPath(path), true, path);
  }
  for (const path of ['/', '/assets/app.js', '/workspace/inbox', '/@vite/client']) {
    assert.equal(isCanonicalImApiPath(path), false, path);
  }
});
