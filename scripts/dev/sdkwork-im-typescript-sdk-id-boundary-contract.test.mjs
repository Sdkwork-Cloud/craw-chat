import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const conversationsModuleSource = read(
  'sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/conversations-module.ts',
);
const realtimeSource = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/realtime.ts');
const readmeSource = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md');
const transportClientLikeSource = read(
  'sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/transport-client-like.ts',
);

for (const [label, source] of [
  ['conversations-module.ts', conversationsModuleSource],
  ['transport-client-like.ts', transportClientLikeSource],
]) {
  assert.doesNotMatch(
    source,
    /\b(?:conversationId|messageId|memberId|targetUserId): string \| number\b/u,
    `${label} must expose snowflake and conversation identifiers as string-only values`,
  );
}

assert.match(
  conversationsModuleSource,
  /\brequireStringIdentifier\b/u,
  'conversations-module.ts must reject non-string identifier values before delegating to generated transport',
);

assert.doesNotMatch(
  realtimeSource,
  /sdkwork-im-(?:auth-init|subscriptions-sync)-\d+/u,
  'TypeScript realtime SDK must not create retired client request identifiers.',
);
assert.doesNotMatch(
  realtimeSource.match(/function sendAuthInit[\s\S]*?\n\}\n/u)?.[0] ?? '',
  /\brequestId\b/u,
  'TypeScript realtime auth.init frames must not send legacy requestId fields.',
);
assert.doesNotMatch(
  readmeSource,
  /"requestId"\s*:\s*"sdkwork-im-auth-init-\d+"/u,
  'TypeScript IM SDK README must not document legacy auth.init requestId fields.',
);
assert.match(
  readmeSource,
  /\bauth\.ok\b[\s\S]*\btraceId\b/u,
  'TypeScript IM SDK README must document server-owned auth.ok traceId semantics.',
);

console.log('sdkwork im TypeScript SDK id boundary contract passed.');
