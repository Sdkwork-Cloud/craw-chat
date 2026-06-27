import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

const rpcBinMain = read('services/session-gateway-rpc-bin/src/main.rs');
const rpcBinCargo = read('services/session-gateway-rpc-bin/Cargo.toml');
const workspaceCargo = read('Cargo.toml');
const topologySpec = readJson('specs/topology.spec.json');

assert.match(
  workspaceCargo,
  /session-gateway-rpc-bin/u,
  'workspace must include session-gateway-rpc-bin crate',
);

assert.match(
  rpcBinMain,
  /build_im_rpc_service_router_with_config_for_services/u,
  'session-gateway rpc host must build a service-scoped IM RPC router',
);
assert.match(
  rpcBinMain,
  /SessionGatewayRpcDispatcher/u,
  'session-gateway rpc host must delegate to runtime dispatcher',
);
assert.match(
  rpcBinMain,
  /SESSION_GATEWAY_RPC_SERVICE_KEYS/u,
  'session-gateway rpc host must register only realtime RPC services',
);
assert.match(
  rpcBinMain,
  /ImRpcHealthService|enable_health: true/u,
  'session-gateway rpc host must enable gRPC health checks',
);
assert.match(
  rpcBinMain,
  /SDKWORK_IM_SESSION_GATEWAY_RPC_BIND_ADDR/u,
  'session-gateway rpc host must read bind addr from topology env key',
);
assert.match(
  rpcBinMain,
  /SDKWORK_IM_SESSION_GATEWAY_RPC_PUBLIC_ENDPOINT/u,
  'session-gateway rpc host must expose public endpoint env key',
);

assert.match(
  rpcBinCargo,
  /session-gateway/u,
  'session-gateway rpc host must depend on session-gateway runtime',
);
assert.match(
  rpcBinCargo,
  /sdkwork-im-rpc-service-rust/u,
  'session-gateway rpc host must depend on sdkwork-im-rpc-service-rust',
);

assert.ok(
  topologySpec.internalUpstreams?.['split-services']?.['session-gateway-rpc'],
  'topology spec must declare session-gateway-rpc internal upstream',
);

console.log('sdkwork im session-gateway rpc host contract passed.');
