import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

const source = readFileSync(
  path.join(
    repoRoot,
    'apps',
    'sdkwork-im-flutter-mobile',
    'packages',
    'sdkwork_im_flutter_mobile_chat',
    'lib',
    'src',
    'services',
    'chat_realtime_service.dart',
  ),
  'utf8',
);

assert.match(
  source,
  /void _clearWireSubscriptions\(\)/u,
  'Flutter mobile realtime hub must clear wire subscription handles when a live connection is lost.',
);
assert.match(
  source,
  /void _bindWireSubscriptions\(ImLiveConnection connection\)/u,
  'Flutter mobile realtime hub must rebind live handlers to a replacement connection before syncing leases.',
);
assert.match(
  source,
  /Timer\? _reconnectTimer/u,
  'Flutter mobile realtime hub must keep a reconnect timer while live subscription demand exists.',
);
assert.match(
  source,
  /int _connectionGeneration = 0/u,
  'Flutter mobile realtime hub must guard against stale lifecycle events from older connections.',
);
assert.match(
  source,
  /void _scheduleReconnect\(\)/u,
  'Flutter mobile realtime hub must actively reconnect when a live connection is lost with active handlers.',
);
assert.match(
  source,
  /if\s*\(\s*state\.status == 'closed' \|\| state\.status == 'error'\s*\)\s*\{(?=[\s\S]*?_clearWireSubscriptions\(\);)/u,
  'Flutter mobile realtime hub must clear old wire listeners on closed/error lifecycle transitions.',
);
assert.match(
  source,
  /if\s*\(\s*generation != _connectionGeneration \|\| !identical\(_connection, connection\)\s*\)\s*\{/u,
  'Flutter mobile realtime lifecycle handlers must ignore stale events from superseded connections.',
);
assert.match(
  source,
  /_scheduleReconnect\(\);/u,
  'Flutter mobile realtime lifecycle loss handling must schedule reconnect while handlers remain active.',
);
assert.match(
  source,
  /void _syncSubscriptions\(ImLiveConnection connection\)\s*\{(?=[\s\S]*?_bindWireSubscriptions\(connection\);)/u,
  'Flutter mobile realtime subscription sync must bind local handlers to the active connection before syncing server leases.',
);
assert.match(
  source,
  /if\s*\(\s*_conversationUnsubs\.containsKey\(conversationId\)\s*\)\s*\{\s*continue;\s*\}/u,
  'Flutter mobile realtime rebinding must not duplicate existing conversation wire listeners.',
);
assert.match(
  source,
  /if\s*\(\s*_inboxUnsubs\.containsKey\(scopeKey\)\s*\)\s*\{\s*continue;\s*\}/u,
  'Flutter mobile realtime rebinding must not duplicate existing inbox wire listeners.',
);

console.log('sdkwork im Flutter realtime session keep contract passed.');
