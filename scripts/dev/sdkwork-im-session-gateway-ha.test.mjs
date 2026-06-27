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

const runtimeBootstrapSource = read('services/session-gateway/src/runtime_bootstrap.rs');
const gatewayEmbedSource = read('services/session-gateway/src/gateway_embed.rs');
const embeddedGatewayModule = read('services/sdkwork-im-cloud-gateway/src/embedded_session_gateway.rs');
const sessionGatewayLib = read('services/session-gateway/src/lib.rs');
const sessionGatewayHttpLimits = read('services/session-gateway/src/http_limits.rs');
const sessionGatewayBin = read('services/session-gateway-bin/src/main.rs');
const standaloneGatewayMain = read('services/sdkwork-im-standalone-gateway/src/main.rs');
const gatewayConfigLib = read('crates/sdkwork-im-cloud-gateway-config/src/lib.rs');
const gatewayLib = read('services/sdkwork-im-cloud-gateway/src/lib.rs');
const openApiRouter = read('crates/sdkwork-routes-im-realtime-open-api/src/lib.rs');
const topologySpec = readJson('specs/topology.spec.json');

const haEnvKeys = [
  'SDKWORK_IM_REALTIME_CLUSTER_BUS_URL',
  'SDKWORK_IM_REALTIME_CLUSTER_BUS_SECRET',
  'SDKWORK_IM_REALTIME_ROUTE_STORE_URL',
  'SDKWORK_IM_REALTIME_NODE_ID',
  'SDKWORK_IM_REALTIME_MAX_WEBSOCKET_CONNECTIONS',
  'SDKWORK_IM_SESSION_GATEWAY_MAX_IN_FLIGHT_REQUESTS',
  'SDKWORK_IM_SESSION_GATEWAY_MAX_REQUEST_BODY_BYTES',
];

for (const envKey of haEnvKeys) {
  assert.match(
    `${runtimeBootstrapSource}\n${sessionGatewayLib}\n${sessionGatewayHttpLimits}`,
    new RegExp(envKey.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
    `session-gateway HA must declare env key ${envKey}`,
  );
}

assert.match(
  runtimeBootstrapSource,
  /bootstrap_realtime_plane_from_env/u,
  'session-gateway must expose env-driven realtime plane bootstrap',
);
assert.match(
  runtimeBootstrapSource,
  /spawn_cluster_route_event_subscriber/u,
  'session-gateway must expose cluster route subscriber bootstrap',
);
assert.match(
  runtimeBootstrapSource,
  /PostgresRealtimeDisconnectFenceStore/u,
  'session-gateway HA bootstrap must support postgres realtime stores',
);
assert.match(
  runtimeBootstrapSource,
  /validate_realtime_node_id_for_cluster/u,
  'session-gateway HA bootstrap must reject default node id when cluster is enabled',
);
assert.match(
  runtimeBootstrapSource,
  /resolve_cluster_bus_secret_from_env/u,
  'session-gateway HA bootstrap must require cluster bus secret when redis cluster is enabled',
);

assert.match(
  sessionGatewayBin,
  /bootstrap_realtime_plane_from_env/u,
  'session-gateway binary must bootstrap realtime plane from env',
);
assert.match(
  sessionGatewayBin,
  /spawn_cluster_route_event_subscriber/u,
  'session-gateway binary must start cluster route subscriber when configured',
);
assert.match(
  sessionGatewayBin,
  /build_public_app_with_realtime_bootstrap/u,
  'session-gateway binary must wire env bootstrap into public app builder',
);

assert.match(
  openApiRouter,
  /build_public_app_with_realtime_bootstrap/u,
  'realtime open-api router must expose env bootstrap builder',
);

const authContextSource = read('services/session-gateway/src/auth_context.rs');
const routeStoreTierSource = read('services/session-gateway/src/route_store_tier.rs');

assert.match(
  authContextSource,
  /resolve_iam_app_context_from_dual_tokens/u,
  'session-gateway auth context must verify dual tokens through IAM when database lookup is configured',
);
assert.match(
  authContextSource,
  /resolver_without_iam_pool_uses_im_app_context/u,
  'session-gateway auth context must test im-app-context fallback when IAM pool is absent',
);
assert.match(
  authContextSource,
  /resolver_without_iam_pool_rejects_production_environment/u,
  'session-gateway auth context must fail closed in production when IAM pool is absent',
);
assert.match(
  authContextSource,
  /resolve_app_context/u,
  'session-gateway auth context must fall back to im-app-context env bootstrap in dev/test when IAM pool is unavailable',
);
assert.match(
  routeStoreTierSource,
  /mirror_persist_with_retry/u,
  'session-gateway postgres route mirror must retry durable writes',
);
assert.match(
  runtimeBootstrapSource,
  /resolve_iam_auth_pool_from_env/u,
  'session-gateway HA bootstrap must load IAM signing pool for realtime auth',
);

assert.match(
  runtimeBootstrapSource,
  /RedisBackedRouteStore/u,
  'session-gateway HA bootstrap must support redis-backed route store',
);
assert.match(
  runtimeBootstrapSource,
  /RedisPostgresTieredRouteStore/u,
  'session-gateway HA bootstrap must support redis+postgres tiered route store',
);

const linkTransportSource = read('services/session-gateway/src/link_transport.rs');
const linkQuicSource = read('services/session-gateway/src/link_quic.rs');

assert.match(
  `${linkTransportSource}\n${linkQuicSource}`,
  /SDKWORK_IM_REALTIME_QUIC_BIND_ADDR/u,
  'session-gateway link transport must declare quic bind env',
);
assert.match(
  linkQuicSource,
  /SDKWORK_IM_REALTIME_QUIC_TLS_CERT_PATH/u,
  'session-gateway quic listener must require tls cert env',
);
assert.match(
  linkQuicSource,
  /SDKWORK_IM_REALTIME_QUIC_TLS_KEY_PATH/u,
  'session-gateway quic listener must require tls key env',
);

assert.match(
  runtimeBootstrapSource,
  /PostgresBackedRouteStore/u,
  'session-gateway HA bootstrap must support postgres-backed route store',
);
assert.match(
  runtimeBootstrapSource,
  /resolve_route_store_from_env/u,
  'session-gateway must resolve route store from env',
);

assert.ok(
  topologySpec.internalUpstreams?.['split-services']?.['session-gateway'],
  'topology spec must declare session-gateway internal upstream',
);

assert.match(
  gatewayConfigLib,
  /Unified/u,
  'gateway config must expose unified runtime mode for unified-process layouts',
);
assert.match(
  gatewayConfigLib,
  /SDKWORK_IM_SERVICE_LAYOUT/u,
  'gateway config must resolve runtime mode from service layout env',
);
assert.match(
  gatewayConfigLib,
  /SDKWORK_IM_GATEWAY_EMBED_REALTIME_PLANE/u,
  'gateway config must declare optional split-layout embed env',
);
assert.match(
  gatewayConfigLib,
  /should_embed_session_gateway/u,
  'gateway config must resolve embed policy from layout and env',
);
assert.match(
  gatewayEmbedSource,
  /bootstrap_gateway_embedded_realtime_plane/u,
  'session-gateway must expose gateway embed bootstrap helper',
);
assert.match(
  gatewayEmbedSource,
  /spawn_link_transport_listeners/u,
  'gateway embed bootstrap must start optional tcp/udp/quic link listeners',
);
assert.match(
  embeddedGatewayModule,
  /bootstrap_embedded_session_gateway_runtime/u,
  'gateway must expose shared embedded session-gateway runtime bootstrap',
);
assert.match(
  embeddedGatewayModule,
  /spawn_blocking\(move \|\| drop\(router\)\)/u,
  'embedded session-gateway shutdown must drop postgres-backed router off async runtime workers',
);
assert.match(
  gatewayLib,
  /bootstrap_embedded_session_gateway_runtime/u,
  'gateway lib must re-export embedded session-gateway bootstrap',
);
assert.match(
  gatewayLib,
  /build_app_with_registry_product_runtime_and_embedded_services/u,
  'gateway must support embedded session-gateway router injection',
);
assert.match(
  gatewayLib,
  /dispatch_embedded_session_gateway_if_configured/u,
  'gateway must dispatch embedded session-gateway when router is configured',
);
assert.doesNotMatch(
  gatewayLib,
  /runtime_mode != GatewayRuntimeMode::Unified/u,
  'embedded session-gateway dispatch must not be gated on unified-only runtime mode',
);
assert.match(
  standaloneGatewayMain,
  /bootstrap_embedded_session_gateway_runtime/u,
  'standalone gateway must use shared embedded session-gateway bootstrap',
);
assert.doesNotMatch(
  standaloneGatewayMain,
  /GatewayRuntimeMode::Unified/u,
  'standalone gateway must not hard-code unified-only embed gate',
);

const realtimeApiPathsLib = read('crates/sdkwork-im-realtime-api-paths/src/lib.rs');
const sessionGatewayLibSource = read('services/session-gateway/src/lib.rs');

assert.match(
  realtimeApiPathsLib,
  /REALTIME_WS/u,
  'realtime api paths crate must declare canonical websocket path',
);
assert.match(
  sessionGatewayLibSource,
  /sdkwork_im_realtime_api_paths/u,
  'session-gateway must consume canonical realtime api paths',
);
assert.match(
  sessionGatewayLibSource,
  /build_domain_api_router_literals_match_canonical_paths/u,
  'session-gateway must verify router literals against canonical paths',
);

const ccpRegistrySource = read('crates/sdkwork-im-ccp-registry/src/lib.rs');

assert.match(
  ccpRegistrySource,
  /session\.resume/u,
  'control-plane CCP registry must publish session.resume capability',
);

const chatCliRealtime = read('tools/chat-cli/src/realtime.rs');
const chatCliCargo = read('tools/chat-cli/Cargo.toml');

assert.match(
  chatCliCargo,
  /sdkwork-im-realtime-api-paths/u,
  'chat-cli must depend on canonical realtime api paths crate',
);
assert.match(
  chatCliRealtime,
  /sdkwork_im_realtime_api_paths::REALTIME_WS/u,
  'chat-cli must connect using canonical REALTIME_WS path',
);
assert.match(
  chatCliRealtime,
  /server did not negotiate CCP websocket subprotocol/u,
  'chat-cli must fail closed when CCP subprotocol is not negotiated',
);
assert.doesNotMatch(
  chatCliRealtime,
  /LegacyJson/u,
  'chat-cli must not retain legacy.json websocket mode branches',
);

assert.match(
  sessionGatewayLib,
  /REALTIME_ACCEPT_LEGACY_WEBSOCKET_JSON/u,
  'session-gateway must declare legacy websocket json compat env',
);
assert.match(
  sessionGatewayLib,
  /realtime_accepts_legacy_websocket_json/u,
  'session-gateway must resolve legacy websocket json compat policy from env',
);

console.log('sdkwork im session-gateway HA contract passed.');
