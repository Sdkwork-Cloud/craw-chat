import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const chatServiceText = readFileSync(
  './packages/sdkwork-im-pc-chat/src/services/ChatService.ts',
  'utf8',
);
const contactServiceText = readFileSync(
  './packages/sdkwork-im-pc-chat/src/services/ContactService.ts',
  'utf8',
);
const managerText = readFileSync(
  './packages/sdkwork-im-pc-core/src/sdk/pcRealtimeConnectionManager.ts',
  'utf8',
);

assert.match(
  chatServiceText,
  /\bhandleRealtimeAuthenticationFailure\b[\s\S]*\bcloseAllLiveSubscriptions\b/u,
  'ChatService must stop realtime subscriptions on authentication failure without clearing persisted auth state',
);
assert.match(
  managerText,
  /scheduleReconnect\([\s\S]*!isAppSdkSessionAuthenticated\(resolveSession\(\)\)/u,
  'Shared PC realtime manager must not schedule websocket reconnects without an authenticated session',
);

assert.match(
  contactServiceText,
  /\bSDKWORK_IM_SESSION_CHANGED_EVENT\b/u,
  'ContactService must react to auth session changes',
);
assert.match(
  contactServiceText,
  /\bhandleAuthenticationFailure\b/u,
  'ContactService must centralize authentication failure handling',
);
assert.match(
  contactServiceText,
  /\bsubscribePendingFriendRequestCount\b[\s\S]*this\.hasAuthenticatedSession\(\)/u,
  'ContactService pending friend request subscriptions must guard against unauthenticated startup',
);
assert.match(
  contactServiceText,
  /\bstartPendingFriendRequestRealtime\b[\s\S]*!this\.hasAuthenticatedSession\(\)/u,
  'ContactService realtime startup must short-circuit when no authenticated session exists',
);

console.log('sdkwork im pc chat auth realtime guard contract passed.');
