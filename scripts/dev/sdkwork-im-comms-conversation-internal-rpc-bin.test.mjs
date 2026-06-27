import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const rpcBinMain = read('services/sdkwork-comms-conversation-internal-rpc-bin/src/main.rs');
const rpcBinCargo = read('services/sdkwork-comms-conversation-internal-rpc-bin/Cargo.toml');
const workspaceCargo = read('Cargo.toml');

assert.match(
  workspaceCargo,
  /sdkwork-comms-conversation-internal-rpc-bin/u,
  'workspace must include sdkwork-comms-conversation-internal-rpc-bin crate',
);

assert.match(
  rpcBinMain,
  /build_im_rpc_service_router_with_config_for_services/u,
  'conversation internal rpc host must build a service-scoped IM RPC router',
);
assert.match(
  rpcBinMain,
  /ConversationInternalRpcDispatcher/u,
  'conversation internal rpc host must delegate to internal runtime dispatcher',
);
assert.match(
  rpcBinMain,
  /CONVERSATION_INTERNAL_RPC_SERVICE_KEYS/u,
  'conversation internal rpc host must register room orchestration and message dispatch services',
);
assert.match(
  rpcBinMain,
  /SDKWORK_IM_COMMS_CONVERSATION_INTERNAL_RPC_BIND_ADDR/u,
  'conversation internal rpc host must read bind addr from topology env key',
);
assert.match(
  rpcBinMain,
  /sdkwork-communication-internal-rpc/u,
  'conversation internal rpc host must document internal discovery service name',
);

assert.match(
  rpcBinCargo,
  /sdkwork-comms-conversation-service/u,
  'conversation internal rpc host must depend on conversation runtime crate',
);
assert.match(
  rpcBinCargo,
  /sdkwork-im-rpc-service-rust/u,
  'conversation internal rpc host must depend on im rpc service scaffold',
);

console.log('sdkwork im comms-conversation internal rpc host contract passed.');
