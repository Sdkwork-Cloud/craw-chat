#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const fcmSource = read('adapters/push-providers/push-fcm/src/lib.rs');
const apnsSource = read('adapters/push-providers/push-apns/src/lib.rs');

assert.match(
  fcmSource,
  /accepted:\s*false/u,
  'FCM adapter must fail closed instead of returning fake accepted=true',
);
assert.match(
  fcmSource,
  /send_oauth_v1/u,
  'FCM adapter must implement HTTP v1 OAuth transport when credentials path is configured',
);
assert.match(
  fcmSource,
  /FCM_LEGACY_URL/u,
  'FCM adapter must implement legacy HTTP transport when server key is configured',
);
assert.match(
  fcmSource,
  /status:\s*"degraded"/u,
  'FCM adapter must report degraded health when transport is unavailable',
);
assert.match(
  apnsSource,
  /accepted:\s*false/u,
  'APNs adapter must fail closed instead of returning fake accepted=true',
);
assert.match(
  apnsSource,
  /status:\s*"degraded"/u,
  'APNs adapter must report degraded health until HTTP/2 transport ships',
);

console.log('sdkwork-im push provider standard contract passed');
