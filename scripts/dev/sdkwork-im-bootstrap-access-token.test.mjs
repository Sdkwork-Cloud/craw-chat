import assert from 'node:assert/strict';
import test from 'node:test';

import {
  buildSdkworkImBootstrapAccessTokenEnv,
  mergeSdkworkImBootstrapAccessTokenEnv,
  resolveSdkworkImBootstrapAccessTokenEnv,
} from './sdkwork-im-bootstrap-access-token.mjs';

test('buildSdkworkImBootstrapAccessTokenEnv generates JWT bootstrap access token when missing', () => {
  const env = buildSdkworkImBootstrapAccessTokenEnv();
  const token = env.SDKWORK_ACCESS_TOKEN;
  assert.match(token, /^eyJ/u);
  assert.match(token, /\.signature$/u);
});

test('buildSdkworkImBootstrapAccessTokenEnv preserves configured bootstrap access token', () => {
  const env = buildSdkworkImBootstrapAccessTokenEnv({
    existingAccessToken: 'existing-bootstrap-token',
  });
  assert.equal(env.SDKWORK_ACCESS_TOKEN, 'existing-bootstrap-token');
});

test('mergeSdkworkImBootstrapAccessTokenEnv injects bootstrap token into renderer env', () => {
  const merged = mergeSdkworkImBootstrapAccessTokenEnv({
    VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: 'http://127.0.0.1:18079',
  });
  assert.match(merged.SDKWORK_ACCESS_TOKEN, /^eyJ/u);
  assert.equal(
    merged.VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL,
    'http://127.0.0.1:18079',
  );
});

test('resolveSdkworkImBootstrapAccessTokenEnv reads existing process env token', () => {
  const resolved = resolveSdkworkImBootstrapAccessTokenEnv({
    SDKWORK_ACCESS_TOKEN: 'configured-token',
  });
  assert.equal(resolved.SDKWORK_ACCESS_TOKEN, 'configured-token');
});

console.log('sdkwork-im bootstrap access token contract passed.');
