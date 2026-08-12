import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function readText(...segments) {
  return readFileSync(path.join(repoRoot, ...segments), 'utf8');
}

// Track A — weak network ARQ (client + server)
const realtimeSdkText = readText(
  'sdks',
  'sdkwork-im-sdk',
  'sdkwork-im-sdk-typescript',
  'src',
  'realtime.ts',
);
const runtimeLinkText = readText('crates', 'sdkwork-im-runtime-link', 'src', 'lib.rs');
const gatewayWebsocketText = readText('services', 'session-gateway', 'src', 'websocket.rs');

assert.match(
  realtimeSdkText,
  /sendEventsNack/u,
  'IM SDK realtime client must send events.nack for ARQ gap recovery',
);
assert.match(
  realtimeSdkText,
  /createRealtimeSeqTracker/u,
  'IM SDK realtime client must track contiguous realtime seq before nack',
);
assert.match(
  runtimeLinkText,
  /plan_nack_replay/u,
  'runtime-link must plan ARQ replay windows after client nack',
);
assert.match(
  gatewayWebsocketText,
  /"events\.nack"/u,
  'session-gateway must handle events.nack business frames',
);

// Track B — 10K large group
const spaceDomainText = readText('crates', 'im-domain-core', 'src', 'space.rs');
const memberDirectoryText = readText(
  'services',
  'sdkwork-comms-conversation-service',
  'src',
  'conversation_state',
  'member_directory.rs',
);
const memberStoreText = readText(
  'services',
  'sdkwork-comms-conversation-service',
  'src',
  'conversation_state',
  'member_store.rs',
);

assert.match(
  spaceDomainText,
  /DEFAULT_CHAT_GROUP_MAX_MEMBERS:\s*i32\s*=\s*10_000/u,
  'default chat group cap must be 10_000 members (WeChat-class)',
);
assert.match(
  memberDirectoryText,
  /collect_member_directory_window/u,
  'member directory list must page from maintained index, not full roster collect',
);
assert.match(
  memberStoreText,
  /member_directory_by_scope/u,
  'conversation state must maintain a per-scope member directory index',
);

// Track C — desktop offline persistence
const offlineStoreText = readText(
  'apps',
  'sdkwork-im-pc',
  'packages',
  'sdkwork-im-pc-desktop',
  'src-tauri',
  'src',
  'offline_store.rs',
);
const offlineCacheText = readText(
  'apps',
  'sdkwork-im-pc',
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'desktopOfflineChatCache.ts',
);
const chatServiceText = readText(
  'apps',
  'sdkwork-im-pc',
  'packages',
  'sdkwork-im-pc-chat',
  'src',
  'services',
  'ChatService.ts',
);

assert.match(
  offlineStoreText,
  /im_local_message_cache/u,
  'Tauri offline store must persist messages in SQLite',
);
assert.match(
  offlineCacheText,
  /persistDesktopOfflineMessages/u,
  'pc-core must expose desktop offline chat cache helpers',
);
assert.match(
  chatServiceText,
  /loadDesktopOfflineMessages/u,
  'ChatService must read desktop offline cache when network fetch fails',
);
assert.match(
  chatServiceText,
  /persistDesktopOfflineMessages/u,
  'ChatService must write fetched messages into desktop offline cache',
);

// Production alignment: RPC normalized state + favorites index pagination
const rpcStateDispatchText = readText(
  'services',
  'sdkwork-comms-conversation-service',
  'src',
  'runtime',
  'rpc_state_dispatch.rs',
);
const rpcDispatchText = readText(
  'services',
  'sdkwork-comms-conversation-service',
  'src',
  'runtime',
  'rpc_dispatch.rs',
);
const messageFavoritesText = readText(
  'services',
  'sdkwork-comms-conversation-service',
  'src',
  'conversation_state',
  'message_favorites.rs',
);
const offlineSendQueueText = readText(
  'apps',
  'sdkwork-im-pc',
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'desktopOfflineSendQueue.ts',
);

assert.match(
  rpcStateDispatchText,
  /dispatch_retrieve_conversation_preferences/u,
  'conversation RPC host must serve preferences.retrieve through normalized state',
);
assert.match(
  rpcStateDispatchText,
  /dispatch_create_message_favorite/u,
  'conversation RPC host must serve messages.favorites.create through normalized state',
);
assert.doesNotMatch(
  rpcDispatchText,
  /dispatch_read_model_boundary/u,
  'conversation RPC dispatch must not retain retired read-model boundary stubs',
);
assert.match(
  messageFavoritesText,
  /message_favorites_index/u,
  'message favorites list must page from maintained per-principal index',
);
assert.match(
  messageFavoritesText,
  /collect_message_favorites_index_window/u,
  'message favorites window must collect from index without full principal scan',
);

assert.match(
  chatServiceText,
  /flushDesktopPendingSendQueue/u,
  'ChatService must flush desktop pending send queue on realtime reconnect',
);
assert.match(
  chatServiceText,
  /runDesktopPendingSendFlush/u,
  'ChatService pending send flush must claim queued sends before posting to avoid duplicate sends across reconnects or windows',
);
assert.match(
  chatServiceText,
  /releaseDesktopPendingSendClaim/u,
  'ChatService pending send flush must release claimed sends when retryable failures keep them queued',
);
assert.match(
  offlineStoreText,
  /im_local_pending_send/u,
  'Tauri offline store must persist pending outbound sends',
);

assert.match(
  messageFavoritesText,
  /favorite_matches_filters/u,
  'message favorites filtered list must apply filters during index scan',
);
assert.match(
  chatServiceText,
  /hydrateDesktopPendingSends/u,
  'ChatService must hydrate pending sends from SQLite on reconnect',
);
assert.match(
  offlineSendQueueText,
  /enqueueDesktopPendingSend/u,
  'desktop offline send queue must support text and uploaded media payloads',
);
assert.match(
  readText(
    'apps',
    'sdkwork-im-pc',
    'packages',
    'sdkwork-im-pc-chat',
    'src',
    'components',
    'MessageList.tsx',
  ),
  /msg\.sendState/u,
  'MessageList must render local sendState for desktop offline pending/failed messages',
);
assert.match(
  readText(
    'apps',
    'sdkwork-im-pc',
    'packages',
    'sdkwork-im-pc-chat',
    'src',
    'services',
    'ChatService.ts',
  ),
  /retryFailedMessage/u,
  'ChatService must expose retry for failed desktop outbound messages',
);

// Flutter mobile inbox cursor pagination
const flutterInboxText = readText(
  'apps',
  'sdkwork-im-flutter-mobile',
  'packages',
  'sdkwork_im_flutter_mobile_chat',
  'lib',
  'src',
  'services',
  'chat_inbox_service.dart',
);
assert.match(
  flutterInboxText,
  /fetchInboxPage/u,
  'Flutter inbox service must expose cursor page fetch',
);
assert.match(
  flutterInboxText,
  /inboxPageSize = 20/u,
  'Flutter inbox page size must align with SdkWork default page size',
);
assert.match(
  flutterInboxText,
  /maxInboxSyncPages = 10/u,
  'Flutter inbox sync must be bounded to prevent unbounded multi-page download',
);

// RPC stream contract — ConversationService is unary-only; realtime streams live in session-gateway
const conversationProtoText = readText(
  'apis',
  'rpc',
  'sdkwork',
  'communication',
  'app',
  'v3',
  'conversation_service.proto',
);
const sessionGatewayRpcText = readText('services', 'session-gateway', 'src', 'rpc_dispatch.rs');
assert.doesNotMatch(
  conversationProtoText,
  /returns \(stream/u,
  'ConversationService RPC contract must remain unary-only for pre-release scope',
);
assert.match(
  sessionGatewayRpcText,
  /"presence\.watch"/u,
  'session-gateway must implement presence.watch server stream',
);
assert.match(
  sessionGatewayRpcText,
  /spawn_blocking/u,
  'session-gateway gRPC list_events must isolate blocking realtime I/O via spawn_blocking',
);
assert.match(
  sessionGatewayRpcText,
  /"realtime\.events\.watch"/u,
  'session-gateway must implement realtime.events.watch server stream',
);

const h5OfflineQueueText = readText(
  'apps',
  'sdkwork-im-h5',
  'packages',
  'sdkwork-im-h5-chat',
  'src',
  'services',
  'offlineSendQueue.ts',
);
assert.match(
  h5OfflineQueueText,
  /indexedDB\.open/u,
  'H5 offline send queue must persist via IndexedDB',
);
assert.match(
  h5OfflineQueueText,
  /claimPendingTextSends/u,
  'H5 offline send queue must claim pending sends before flush',
);
assert.match(
  h5OfflineQueueText,
  /runPendingTextSendFlush/u,
  'H5 offline send queue must serialize flush with in-flight guard',
);

const flutterOfflineQueueText = readText(
  'apps',
  'sdkwork-im-flutter-mobile',
  'packages',
  'sdkwork_im_flutter_mobile_chat',
  'lib',
  'src',
  'services',
  'offline_send_queue.dart',
);
assert.match(
  flutterOfflineQueueText,
  /claimPendingTextSends/u,
  'Flutter offline send queue must claim pending sends before flush',
);
assert.match(
  flutterOfflineQueueText,
  /runPendingTextSendFlushForTenant/u,
  'Flutter offline send queue must serialize flush with in-flight guard',
);
assert.match(
  flutterOfflineQueueText,
  /flushClaimId/u,
  'Flutter offline send records must track flush claim lease',
);

console.log('sdkwork-im three-capabilities alignment standard passed');
