import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const realtimeSource = read(
  'sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/composed/im_sdk_composed/lib/src/im_realtime.dart',
);
const ccpWireSource = read(
  'sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/composed/im_sdk_composed/lib/src/ccp_wire.dart',
);

assert.doesNotMatch(
  realtimeSource,
  /\brequestId\b/u,
  'Flutter composed realtime wire frames must not expose legacy requestId fields.',
);
assert.doesNotMatch(
  realtimeSource,
  /sdkwork-im-(?:subscriptions-sync|auth-init)-\d+/u,
  'Flutter composed realtime must not create retired client request identifiers.',
);
assert.doesNotMatch(
  ccpWireSource,
  /\brequestId\b/u,
  'Flutter composed CCP trace variables must be named traceId, not requestId.',
);
assert.match(
  ccpWireSource,
  /\btrace_id\b/u,
  'Flutter composed CCP envelopes must keep trace_id as the CCP envelope trace field.',
);

console.log('sdkwork im Flutter realtime trace identity contract passed.');
