import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const rpcBinMain = read('services/sdkwork-comms-conversation-rpc-bin/src/main.rs');
const rpcBinCargo = read('services/sdkwork-comms-conversation-rpc-bin/Cargo.toml');
const workspaceCargo = read('Cargo.toml');

assert.match(
  workspaceCargo,
  /sdkwork-comms-conversation-rpc-bin/u,
  'workspace must include sdkwork-comms-conversation-rpc-bin crate',
);

assert.match(
  rpcBinMain,
  /build_im_rpc_service_router_with_config_for_services/u,
  'conversation rpc host must build a service-scoped IM RPC router',
);
assert.match(
  rpcBinMain,
  /ConversationRpcDispatcher/u,
  'conversation rpc host must delegate to runtime dispatcher',
);
assert.match(
  rpcBinMain,
  /CONVERSATION_RPC_SERVICE_KEYS/u,
  'conversation rpc host must register conversation, message, and room RPC services',
);
assert.match(
  rpcBinMain,
  /SDKWORK_IM_COMMS_CONVERSATION_RPC_BIND_ADDR/u,
  'conversation rpc host must read bind addr from topology env key',
);

assert.match(
  rpcBinCargo,
  /sdkwork-comms-conversation-service/u,
  'conversation rpc host must depend on conversation runtime crate',
);
assert.match(
  rpcBinCargo,
  /sdkwork-im-rpc-service-rust/u,
  'conversation rpc host must depend on sdkwork-im-rpc-service-rust',
);

console.log('sdkwork im comms-conversation rpc host contract passed.');
