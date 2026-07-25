import assert from 'node:assert/strict';
import fs from 'node:fs';

import {
  drainDesktopPendingSendBatches,
  partitionDesktopPendingSendRows,
  waitForDesktopPendingSendBackoff,
} from '../packages/sdkwork-im-pc-core/src/sdk/desktopOfflineSendQueue';
import {
  desktopOfflineScopeKey,
  desktopOfflineScopesEqual,
  type DesktopOfflinePrincipalScope,
} from '../packages/sdkwork-im-pc-core/src/sdk/desktopOfflineScope';

const principalScope: DesktopOfflinePrincipalScope = {
  environment: 'development',
  deploymentProfile: 'standalone',
  deploymentMode: 'local',
  apiOrigin: 'http://127.0.0.1:18079',
  tenantId: '100001',
  organizationId: 'org-a',
  accountId: 'account-1',
  principalKind: 'user',
  principalId: 'user-1',
};

assert.equal(desktopOfflineScopesEqual(principalScope, { ...principalScope }), true);
for (const [field, value] of [
  ['environment', 'test'],
  ['deploymentProfile', 'cloud'],
  ['deploymentMode', 'private'],
  ['apiOrigin', 'https://im.example.com'],
  ['tenantId', '100002'],
  ['organizationId', 'org-b'],
  ['accountId', 'account-2'],
  ['principalKind', 'agent'],
  ['principalId', 'agent-2'],
] as const) {
  const changed = { ...principalScope, [field]: value } as DesktopOfflinePrincipalScope;
  assert.equal(desktopOfflineScopesEqual(principalScope, changed), false, `${field} must isolate scope`);
  assert.notEqual(desktopOfflineScopeKey(principalScope), desktopOfflineScopeKey(changed));
}

const queued = Array.from({ length: 125 }, (_, index) => index + 1);
const flushed: number[] = [];
let claimCalls = 0;

await drainDesktopPendingSendBatches(
  async () => {
    claimCalls += 1;
    return queued.splice(0, 50);
  },
  async (batch) => {
    flushed.push(...batch);
    return { retryableFailure: false };
  },
  {
    backoff: async () => undefined,
  },
);

assert.deepEqual(flushed, Array.from({ length: 125 }, (_, index) => index + 1));
assert.equal(claimCalls, 4, 'drain must continue until a claim returns an empty batch');

let retryableClaimCalls = 0;
await drainDesktopPendingSendBatches(
  async () => {
    retryableClaimCalls += 1;
    return [1];
  },
  async () => ({ retryableFailure: true }),
  { backoff: async () => undefined },
);
assert.equal(retryableClaimCalls, 1, 'retryable failure must stop the current drain');

const abortController = new AbortController();
let cancelledClaimCalls = 0;
await drainDesktopPendingSendBatches(
  async () => {
    cancelledClaimCalls += 1;
    return [1];
  },
  async () => {
    abortController.abort();
    return { retryableFailure: false };
  },
  { signal: abortController.signal, backoff: async () => undefined },
);
assert.equal(cancelledClaimCalls, 1, 'cancellation must prevent another claim');

const claimStageAbortController = new AbortController();
const abandonedAfterAbort: number[] = [];
let flushedAfterClaimStageAbort = false;
await drainDesktopPendingSendBatches(
  async () => {
    claimStageAbortController.abort();
    return [1, 2];
  },
  async () => {
    flushedAfterClaimStageAbort = true;
    return { retryableFailure: false };
  },
  {
    signal: claimStageAbortController.signal,
    abandon: async (batch) => {
      abandonedAfterAbort.push(...batch);
    },
    backoff: async () => undefined,
  },
);
assert.equal(flushedAfterClaimStageAbort, false);
assert.deepEqual(abandonedAfterAbort, [1, 2]);

let principalCurrent = true;
const abandonedAfterPrincipalChange: number[] = [];
await drainDesktopPendingSendBatches(
  async () => {
    principalCurrent = false;
    return [3, 4];
  },
  async () => {
    throw new Error('principal-changed batch must not flush');
  },
  {
    isCurrent: () => principalCurrent,
    abandon: async (batch) => {
      abandonedAfterPrincipalChange.push(...batch);
    },
    backoff: async () => undefined,
  },
);
assert.deepEqual(abandonedAfterPrincipalChange, [3, 4]);

let abortListenersAdded = 0;
let abortListenersRemoved = 0;
const observedSignal = {
  aborted: false,
  addEventListener: () => {
    abortListenersAdded += 1;
  },
  removeEventListener: () => {
    abortListenersRemoved += 1;
  },
} as unknown as AbortSignal;
await waitForDesktopPendingSendBackoff(1, observedSignal);
assert.equal(abortListenersAdded, 1);
assert.equal(abortListenersRemoved, 1, 'completed backoff must release its abort listener');

const partitioned = partitionDesktopPendingSendRows([
  {
    scope: {
      ...principalScope,
    },
    clientMsgId: 'valid',
    conversationId: 'conversation',
    payloadJson: JSON.stringify({
      chatId: 'conversation',
      content: 'hello',
      type: 'text',
      clientMsgId: 'valid',
    }),
    createdAt: '2026-07-10T00:00:00Z',
    attemptCount: 1,
  },
  {
    scope: {
      ...principalScope,
    },
    clientMsgId: 'corrupt',
    conversationId: 'conversation',
    payloadJson: '{not-json',
    createdAt: '2026-07-10T00:00:00Z',
    attemptCount: 1,
  },
]);
assert.deepEqual(partitioned.payloads.map((item) => item.clientMsgId), ['valid']);
assert.deepEqual(partitioned.quarantined, [{
  clientMsgId: 'corrupt',
  reason: 'invalid pending send payload',
}]);

const cacheSource = fs.readFileSync(
  new URL('../packages/sdkwork-im-pc-core/src/sdk/desktopOfflineChatCache.ts', import.meta.url),
  'utf8',
);
assert.match(cacheSource, /SDKWORK_IM_SESSION_CHANGED_EVENT/u);
assert.match(cacheSource, /purgeDesktopOfflinePrincipalCache\(previousScope\)/u);

const authSource = fs.readFileSync(
  new URL('../packages/sdkwork-im-pc-core/src/sdk/appAuthService.ts', import.meta.url),
  'utf8',
);
assert.match(authSource, /resolveDesktopOfflinePrincipalScope\(readAppSdkSessionTokens\(\)\)/u);
assert.match(authSource, /await purgeDesktopOfflinePrincipalCache\(offlineScope\)/u);
const logoutStart = authSource.indexOf('async logout()');
const logoutPurge = authSource.indexOf('await purgeDesktopOfflinePrincipalCache(offlineScope)', logoutStart);
const logoutClear = authSource.indexOf('clearAppSdkSessionTokens()', logoutStart);
assert.ok(logoutStart >= 0 && logoutPurge > logoutStart && logoutClear > logoutPurge,
  'logout must await principal cache purge before clearing session tokens');

const offlineStoreRustSource = fs.readFileSync(
  new URL('../packages/sdkwork-im-pc-desktop/src-tauri/src/offline_store.rs', import.meta.url),
  'utf8',
);
assert.doesNotMatch(
  offlineStoreRustSource,
  /#\[tauri::command\]\s+pub fn sdkwork_im_pc_offline_/u,
  'SQLite-backed Tauri commands must not block the command dispatch thread',
);
assert.match(offlineStoreRustSource, /tauri::async_runtime::spawn_blocking/u);

const offlineStoreSource = fs.readFileSync(
  new URL('../packages/sdkwork-im-pc-core/src/sdk/desktopOfflineStore.ts', import.meta.url),
  'utf8',
);
assert.match(offlineStoreSource, /initDesktopOfflineStore\(\s*scope: DesktopOfflinePrincipalScope/u);
assert.match(
  offlineStoreSource,
  /invoke\('sdkwork_im_pc_offline_init', \{ scope \}\)/u,
  'desktop initialization must bind the complete scope in native code',
);

const offlineScopeSource = fs.readFileSync(
  new URL('../packages/sdkwork-im-pc-core/src/sdk/desktopOfflineScope.ts', import.meta.url),
  'utf8',
);
for (const field of [
  'environment',
  'deploymentProfile',
  'deploymentMode',
  'apiOrigin',
  'tenantId',
  'organizationId',
  'accountId',
  'principalKind',
  'principalId',
]) {
  assert.match(offlineScopeSource, new RegExp(`scope\\.${field}`, 'u'));
}
assert.match(offlineScopeSource, /VITE_SDKWORK_IM_DEPLOYMENT_PROFILE/u);
assert.match(offlineScopeSource, /resolveImApiBaseUrl\(\)/u);
assert.doesNotMatch(offlineScopeSource, /organizationId\s*\?\?\s*['"]0['"]/u);

console.log('desktop offline send queue contract passed');
