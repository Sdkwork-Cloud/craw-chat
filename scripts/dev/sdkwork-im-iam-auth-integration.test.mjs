import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const gatewayRuntime = read('services/sdkwork-im-cloud-gateway/src/runtime.rs');
const interceptors = read('../sdkwork-web-framework/crates/sdkwork-web-core/src/interceptors.rs');
const iamAdapterLib = read('../sdkwork-iam/crates/sdkwork-iam-web-adapter/src/lib.rs');
const iamDatabaseEnv = read('../sdkwork-iam/crates/sdkwork-iam-web-adapter/src/iam_database_env.rs');
const imWebBootstrap = read('crates/sdkwork-im-web-bootstrap/src/lib.rs');
const embeddedGateway = read('services/sdkwork-im-cloud-gateway/src/embedded_session_gateway.rs');
const embeddedDependencyRoutes = read('services/sdkwork-im-standalone-gateway/src/embedded_dependency_routes.rs');
const realtimeBootstrap = read('crates/sdkwork-routes-im-realtime-open-api/src/web_bootstrap.rs');

const embeddedDispatch = gatewayRuntime.match(
  /fn should_dispatch_embedded_session_gateway\(path: &str\) -> bool \{[\s\S]*?\n\}/u,
)?.[0] ?? '';
assert.match(
  embeddedDispatch,
  /\/im\/v3\/api\/realtime/,
  'embedded gateway dispatch must include realtime paths',
);
assert.match(
  embeddedDispatch,
  /\/im\/v3\/api\/presence/,
  'embedded gateway dispatch must include presence paths',
);
assert.doesNotMatch(
  embeddedDispatch,
  /path\.starts_with\("\/im\/v3\/api\/"\)/u,
  'embedded gateway must not capture all /im/v3/api traffic',
);
assert.match(
  embeddedDispatch,
  /REALTIME_WS/,
  'embedded gateway must bypass oneshot dispatch for websocket upgrade path',
);

assert.match(
  interceptors,
  /WebApiSurface::OpenApi[\s\S]*resolve_dual_token/,
  'open-api surface must accept dual-token app credentials before api-key/oauth detection',
);

assert.match(
  iamAdapterLib,
  /resolve_iam_postgres_pool_from_env/,
  'IAM adapter must expose shared postgres pool resolver',
);
assert.match(
  iamDatabaseEnv,
  /bridge_iam_database_env_from_im/,
  'IAM adapter must bridge IM postgres URL into IAM database env',
);

assert.match(
  imWebBootstrap,
  /shared_iam_web_request_context_resolver_from_env/,
  'IM web bootstrap must cache IAM resolver for route crates in one process',
);

assert.match(
  embeddedGateway,
  /shared_iam_web_request_context_resolver_from_env/,
  'embedded realtime bootstrap must initialize shared IAM resolver',
);
assert.match(
  embeddedGateway,
  /build_public_app_with_realtime_bootstrap_from_env/,
  'embedded realtime router must use IAM resolver from environment',
);

assert.match(
  realtimeBootstrap,
  /wrap_im_open_api_service_router_from_env/,
  'realtime open-api bootstrap must wire IAM resolver from environment',
);
assert.match(
  realtimeBootstrap,
  /wrap_http_router_from_env/,
  'realtime websocket route must stay outside the HTTP interceptor wrapper',
);
assert.match(
  read('crates/sdkwork-routes-im-realtime-open-api/src/lib.rs'),
  /build_realtime_websocket_router/,
  'realtime open-api router must mount websocket upgrade outside HTTP framework layer',
);

const imServerDev = read('scripts/im-server-dev.mjs');
const imPcDev = read('scripts/lib/im-pc-dev.mjs');
assert.match(
  imServerDev,
  /createStandaloneGatewayProcess/,
  'im-server-dev must use standalone gateway for unified IAM ingress',
);
assert.doesNotMatch(
  imServerDev,
  /createUnifiedImApiSidecarProcesses|for\s*\(\s*const\s+\w*sidecar\w*\s+of/u,
  'im-server-dev must not spawn unified HTTP sidecar processes',
);
assert.match(
  read('services/sdkwork-im-standalone-gateway/src/main.rs'),
  /embedded_application_routes::bootstrap_embedded_application_routes/,
  'standalone gateway must embed application-plane route crates in-process',
);
assert.match(
  read('services/sdkwork-im-standalone-gateway/src/main.rs'),
  /embedded_dependency_routes::bootstrap_embedded_dependency_routes/,
  'standalone gateway must embed sibling dependency route crates in-process',
);
assert.match(
  read('services/sdkwork-im-standalone-gateway/src/main.rs'),
  /SDKWORK_IAM_APP_API_HOST_MOUNTED/,
  'standalone gateway must declare host-mounted IAM before embedding knowledgebase sibling assemblies',
);
assert.match(
  read('services/sdkwork-im-standalone-gateway/src/embedded_dependency_routes.rs'),
  /sdkwork_drive_gateway_assembly::assemble_api_router/,
  'standalone dependency bootstrap must mount drive business routes through sibling gateway assembly library',
);
assert.match(
  read('services/sdkwork-im-standalone-gateway/src/embedded_dependency_routes.rs'),
  /sdkwork_knowledgebase_gateway_assembly::assemble_api_router/,
  'standalone dependency bootstrap must mount knowledgebase business routes through sibling gateway assembly library',
);
assert.match(
  read('services/sdkwork-im-standalone-gateway/src/embedded_dependency_routes.rs'),
  /sdkwork_catalog_gateway_assembly::assemble_api_router[\s\S]*sdkwork_order_gateway_assembly::assemble_api_router[\s\S]*sdkwork_shop_gateway_assembly::assemble_api_router/,
  'standalone dependency bootstrap must mount commerce T1 capabilities through sibling gateway assemblies',
);
assert.match(
  read('services/sdkwork-im-standalone-gateway/src/embedded_dependency_routes.rs'),
  /sdkwork_mail_gateway_assembly::assemble_api_router/,
  'standalone dependency bootstrap must mount mail through sibling gateway assembly library',
);
assert.match(
  read('services/sdkwork-im-standalone-gateway/src/embedded_dependency_routes.rs'),
  /sdkwork_routes_agents_app_api::build_served_router/,
  'standalone dependency bootstrap must mount agents through sibling gateway assembly library',
);
assert.match(
  read('services/sdkwork-im-standalone-gateway/src/embedded_dependency_routes.rs'),
  /sdkwork_notary_gateway_assembly::assemble_api_router/,
  'standalone dependency bootstrap must mount notary business routes through sibling gateway assembly library',
);
assert.match(
  embeddedDependencyRoutes,
  /bootstrap_embedded_dependency_routes\(\)\s*->\s*Result<EmbeddedDependencyRoutes, String>/u,
  'standalone dependency bootstrap must fail readiness instead of returning a partially mounted router',
);
assert.match(
  embeddedDependencyRoutes,
  /merge_embedded_dependency\([\s\S]*?\.await\?/u,
  'declared dependency route assembly failures must propagate to standalone gateway startup',
);
assert.doesNotMatch(
  embeddedDependencyRoutes,
  /dependency_bootstrap_skipped|embedded dependency bootstrap skipped/u,
  'standalone gateway must not hide dependency bootstrap failures and degrade missing SDK routes into 404 responses',
);
assert.match(
  read('crates/sdkwork-im-cloud-gateway-config/src/lib.rs'),
  /COMMERCE_T1_APP_API_SERVICES[\s\S]*sdkwork-mail-app-api[\s\S]*sdkwork-notary-app-api/,
  'cloud gateway config must treat T1 commerce, mail, and notary as standalone-embedded dependency services',
);

const serverDevRuntime = read('scripts/dev/sdkwork-im-server-dev-runtime.mjs');
assert.doesNotMatch(
  serverDevRuntime,
  /28082[\s\S]*28093/u,
  'server bind resolver must not reserve IM API internal runtime port matrices',
);

console.log('sdkwork-im IAM auth integration contract passed');
