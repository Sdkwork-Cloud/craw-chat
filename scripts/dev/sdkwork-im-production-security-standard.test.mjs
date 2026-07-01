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
const productionTopology = read('configs/topology/standalone.split-services.production.env');

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

process.stdout.write('sdkwork-im production security standard passed\n');
