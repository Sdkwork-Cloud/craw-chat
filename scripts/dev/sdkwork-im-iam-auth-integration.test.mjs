import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const gatewayMain = read('crates/sdkwork-api-im-standalone-gateway/src/main.rs');
const interceptors = read('../sdkwork-web-framework/crates/sdkwork-web-core/src/interceptors.rs');
const iamAdapterLib = read('../sdkwork-iam/crates/sdkwork-iam-web-adapter/src/lib.rs');
const iamDatabaseEnv = read('../sdkwork-iam/crates/sdkwork-iam-web-adapter/src/iam_database_env.rs');
const imWebBootstrap = read('crates/sdkwork-im-web-bootstrap/src/lib.rs');
const embeddedDependencyRoutes = read('crates/sdkwork-api-im-standalone-gateway/src/embedded_dependency_routes.rs');
const realtimeBootstrap = read('crates/sdkwork-routes-im-realtime-open-api/src/web_bootstrap.rs');

assert.match(
  gatewayMain,
  /assemble_api_router_with_realtime_bootstrap/u,
  'standalone gateway must mount realtime through the application API assembly',
);
assert.match(
  read('crates/sdkwork-api-im-assembly/src/bootstrap.rs'),
  /sdkwork_routes_im_realtime_open_api::build_public_app_with_realtime_bootstrap_from_env/u,
  'application assembly must mount the realtime open-api route crate',
);
assert.doesNotMatch(
  gatewayMain,
  /proxy|upstream registry/iu,
  'standalone gateway must not proxy application routes through a local registry',
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
  gatewayMain,
  /shared_iam_web_request_context_resolver_from_env/,
  'embedded realtime bootstrap must initialize shared IAM resolver',
);
assert.match(
  gatewayMain,
  /assemble_api_router_with_realtime_bootstrap/,
  'standalone gateway must pass the live realtime bootstrap into the API assembly',
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

const gatewayLauncher = read('scripts/gateway-standalone-run.mjs');
const imPcDev = read('scripts/lib/im-pc-dev.mjs');
assert.match(
  gatewayLauncher,
  /run-standalone-gateway-dev\.mjs/,
  'standalone launcher must use the canonical gateway build-and-run helper',
);
assert.doesNotMatch(
  gatewayLauncher,
  /createUnifiedImApiSidecarProcesses|for\s*\(\s*const\s+\w*sidecar\w*\s+of/u,
  'im-server-dev must not spawn unified HTTP sidecar processes',
);
assert.match(
  read('crates/sdkwork-api-im-standalone-gateway/src/main.rs'),
  /sdkwork_api_im_assembly::assemble_api_router_with_realtime_bootstrap/,
  'standalone gateway must consume the canonical application API assembly',
);
assert.match(
  read('crates/sdkwork-api-im-standalone-gateway/src/main.rs'),
  /embedded_dependency_routes::bootstrap_embedded_dependency_routes/,
  'standalone gateway must embed sibling dependency route crates in-process',
);
assert.match(
  read('crates/sdkwork-api-im-standalone-gateway/src/main.rs'),
  /SDKWORK_IAM_APP_API_HOST_MOUNTED/,
  'standalone gateway must declare host-mounted IAM before embedding knowledgebase sibling assemblies',
);
assert.match(
  read('crates/sdkwork-api-im-standalone-gateway/src/embedded_dependency_routes.rs'),
  /sdkwork_api_drive_assembly::assemble_api_router/,
  'standalone dependency bootstrap must mount drive through its canonical API assembly',
);
assert.match(
  read('crates/sdkwork-api-im-standalone-gateway/src/embedded_dependency_routes.rs'),
  /sdkwork_api_knowledgebase_assembly::assemble_api_router/,
  'standalone dependency bootstrap must mount knowledgebase through its canonical API assembly',
);
assert.match(
  read('crates/sdkwork-api-im-standalone-gateway/src/embedded_dependency_routes.rs'),
  /sdkwork_api_catalog_assembly::assemble_api_router[\s\S]*sdkwork_api_order_assembly::assemble_api_router[\s\S]*sdkwork_api_shop_assembly::assemble_api_router/,
  'standalone dependency bootstrap must mount commerce capabilities through canonical API assemblies',
);
assert.match(
  read('crates/sdkwork-api-im-standalone-gateway/src/embedded_dependency_routes.rs'),
  /sdkwork_api_mail_assembly::assemble_api_router/,
  'standalone dependency bootstrap must mount mail through its canonical API assembly',
);
assert.match(
  read('crates/sdkwork-api-im-standalone-gateway/src/embedded_dependency_routes.rs'),
  /sdkwork_api_agents_assembly::assemble_app_business_runtime/,
  'standalone dependency bootstrap must mount agents through its canonical API assembly',
);
assert.match(
  read('crates/sdkwork-api-im-standalone-gateway/src/embedded_dependency_routes.rs'),
  /sdkwork_api_notary_assembly::assemble_api_router/,
  'standalone dependency bootstrap must mount notary through its canonical API assembly',
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
  embeddedDependencyRoutes,
  /sdkwork_api_course_assembly::assemble_api_router/,
  'standalone dependency bootstrap must mount course through its canonical API assembly',
);

const serverDevRuntime = read('scripts/dev/sdkwork-im-server-dev-runtime.mjs');
assert.doesNotMatch(
  serverDevRuntime,
  /28082[\s\S]*28093/u,
  'server bind resolver must not reserve IM API internal runtime port matrices',
);

console.log('sdkwork-im IAM auth integration contract passed');
