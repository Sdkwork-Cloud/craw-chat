#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const appContextSource = read('crates/im-app-context/src/lib.rs');
const chatOpenApiSource = read('crates/sdkwork-routes-im-chat-open-api/src/lib.rs');
const conversationHttpSource = read('services/sdkwork-comms-conversation-service/src/runtime/http.rs');
const sessionGatewayBootstrapSource = read('services/session-gateway/src/runtime_bootstrap.rs');
const imCallsHandlersSource = read('services/im-calls-service/src/handlers.rs');
const productionTopology = read('etc/topology/cloud.production.env');

assert.match(
  appContextSource,
  /Production environment must not use the built-in dev\/test JWT signing secret/u,
  'im-app-context must reject the public dev JWT signing secret in production-like environments.',
);

assert.match(
  chatOpenApiSource,
  /bootstrap_conversation_app_state_from_env\(\)/u,
  'IM chat open-api gateway_mount must bootstrap conversation app state from environment.',
);

assert.doesNotMatch(
  chatOpenApiSource,
  /pub async fn gateway_mount\(\)[\s\S]*default_app_state\(\)/u,
  'gateway_mount must not mount allow-all principal directory via default_app_state.',
);

assert.ok(
  conversationHttpSource.includes(
    'ALLOW_ALL_PRINCIPALS_ENV}=true is forbidden in production',
  ),
  'Conversation runtime must forbid SDKWORK_IM_ALLOW_ALL_PRINCIPALS in production.',
);

assert.match(
  conversationHttpSource,
  /principal directory is required in production/u,
  'Conversation runtime must require a principal directory catalog in production.',
);

assert.match(
  productionTopology,
  /SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE=true/u,
  'Production topology profile must enable AppContext signature verification.',
);

assert.match(
  productionTopology,
  /SDKWORK_IM_JWT_REQUIRE_JTI=true/u,
  'Production topology profile must require JWT jti replay protection.',
);

assert.match(
  productionTopology,
  /SDKWORK_IM_JWT_REPLAY_REDIS_URL=/u,
  'Production topology profile must configure JWT replay Redis URL.',
);

assert.match(
  sessionGatewayBootstrapSource,
  /session-gateway fail-closed.*REALTIME_DATABASE_URL.*production/u,
  'session-gateway must fail-closed in production without PostgreSQL realtime stores.',
);

assert.match(
  imCallsHandlersSource,
  /spawn_blocking/u,
  'im-calls-service handlers must isolate blocking RTC runtime I/O via spawn_blocking.',
);

const sessionGatewayRealtimeSource = read('services/session-gateway/src/realtime.rs');
assert.match(
  sessionGatewayRealtimeSource,
  /lock_scope_sequence_maps/u,
  'session-gateway realtime restore must use canonical lock_scope_sequence_maps ordering.',
);
assert.match(
  sessionGatewayRealtimeSource,
  /Apply restored sequence\/window maps under the canonical lock order/u,
  'session-gateway realtime restore path must document canonical mutex lock order.',
);

process.stdout.write('sdkwork-im production security standard passed\n');
