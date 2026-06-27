import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const managerText = readFileSync(
  './packages/sdkwork-im-pc-core/src/sdk/pcRealtimeConnectionManager.ts',
  'utf8',
);
const chatServiceText = readFileSync(
  './packages/sdkwork-im-pc-chat/src/services/ChatService.ts',
  'utf8',
);
const contactServiceText = readFileSync(
  './packages/sdkwork-im-pc-chat/src/services/ContactService.ts',
  'utf8',
);
const callServiceText = readFileSync(
  './packages/sdkwork-im-pc-chat/src/services/CallService.ts',
  'utf8',
);

assert.match(
  managerText,
  /sharedConnectionPromise/u,
  'PC realtime manager must dedupe in-flight connect attempts',
);
assert.match(
  managerText,
  /recoverPcLiveConnection[\s\S]*connectionStatus === 'open'[\s\S]*connectionStatus === 'connecting'/u,
  'PC realtime recovery must skip healthy connections',
);
assert.match(
  managerText,
  /CIRCUIT_BREAKER_FAILURE_THRESHOLD/u,
  'PC realtime manager must include circuit breaker protection',
);
assert.match(
  managerText,
  /connectionStatus = 'connecting'/u,
  'PC realtime manager must not mark the connection open before lifecycle open',
);
assert.match(
  managerText,
  /state\.status === 'open'[\s\S]*syncWireSubscriptions\(connection\)/u,
  'PC realtime wire subscription sync must run on lifecycle open',
);
assert.match(
  managerText,
  /syncWireSubscriptionsWhenReady[\s\S]*connectionStatus !== 'open'/u,
  'PC realtime wire subscription sync must defer until lifecycle open',
);
assert.doesNotMatch(
  managerText,
  /\.then\(\(connection\) => \{[\s\S]*syncWireSubscriptions\(connection\)/u,
  'PC realtime manager must not sync wire subscriptions immediately after connect resolves',
);
assert.doesNotMatch(
  managerText,
  /connectionStatus = 'open'[\s\S]*syncWireSubscriptions\(connection\)[\s\S]*lifecycleUnsub = connection\.lifecycle\.onStateChange/u,
  'PC realtime manager must not sync wire subscriptions before lifecycle subscription',
);
assert.doesNotMatch(
  chatServiceText,
  /this\.client\(\)\.connect\(/u,
  'ChatService must not open dedicated websocket connections',
);
assert.match(
  chatServiceText,
  /subscribePcConversationMessages/u,
  'ChatService must subscribe through the shared PC realtime manager',
);
assert.match(
  chatServiceText,
  /recoverPcLiveConnection/u,
  'ChatService must delegate realtime recovery to the shared manager',
);
assert.doesNotMatch(
  contactServiceText,
  /this\.client\(\)\.connect\(/u,
  'ContactService must not open dedicated websocket connections',
);
assert.match(
  contactServiceText,
  /subscribePcRealtimeScope/u,
  'ContactService must subscribe friend-request scopes through the shared manager',
);
assert.match(
  callServiceText,
  /watchIncoming\(\{[\s\S]*connection,/u,
  'CallService must reuse the shared live connection for incoming call watch',
);
assert.match(
  callServiceText,
  /acquirePcLiveConnectionLease/u,
  'CallService must hold a shared-connection lease while watching incoming calls',
);

console.log('sdkwork im pc realtime connection contract passed.');
