#!/usr/bin/env node

import assert from 'node:assert/strict';

import fs from 'node:fs';

import path from 'node:path';

import { fileURLToPath } from 'node:url';



import {

  IM_OPENAPI_AUTHORITY_TARGETS,

  applyWebFrameworkOpenApiExtensions,

} from '../sdkwork-im-web-framework-openapi-extensions.mjs';



const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');



function read(relativePath) {

  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');

}



function loadYamlOpenApi(relativePath) {

  const text = read(relativePath);

  const document = { paths: {} };

  let currentPath = null;

  let currentMethod = null;

  for (const line of text.split(/\r?\n/u)) {

    const pathMatch = line.match(/^  (\/[^:]+):\s*$/u);

    if (pathMatch) {

      currentPath = pathMatch[1];

      document.paths[currentPath] = document.paths[currentPath] ?? {};

      currentMethod = null;

      continue;

    }

    const methodMatch = line.match(/^    (get|post|put|patch|delete|options|head):\s*$/u);

    if (methodMatch && currentPath) {

      currentMethod = methodMatch[1];

      document.paths[currentPath][currentMethod] = document.paths[currentPath][currentMethod] ?? {};

      continue;

    }

    if (currentPath && currentMethod) {

      const requestContextMatch = line.match(

        /^      x-sdkwork-request-context:\s+(.+?)\s*$/u,

      );

      if (requestContextMatch) {

        document.paths[currentPath][currentMethod]['x-sdkwork-request-context'] =

          requestContextMatch[1];

      }

      const apiSurfaceMatch = line.match(/^      x-sdkwork-api-surface:\s+(.+?)\s*$/u);

      if (apiSurfaceMatch) {

        document.paths[currentPath][currentMethod]['x-sdkwork-api-surface'] = apiSurfaceMatch[1];

      }

    }

  }

  return document;

}



function countOperationsMissingExtensions(document) {

  let operations = 0;

  let missing = 0;

  for (const pathItem of Object.values(document.paths ?? {})) {

    for (const operation of Object.values(pathItem ?? {})) {

      if (!operation || typeof operation !== 'object' || Array.isArray(operation)) {

        continue;

      }

      if (!operation.operationId) {

        continue;

      }

      operations += 1;

      if (

        operation['x-sdkwork-request-context'] !== 'WebRequestContext'

        || !operation['x-sdkwork-api-surface']

      ) {

        missing += 1;

      }

    }

  }

  return { operations, missing };

}



const rootCargo = read('Cargo.toml');

for (const dependencyKey of [

  'sdkwork_web_core',

  'sdkwork_web_axum',

  'sdkwork_web_bootstrap',

  'sdkwork_iam_web_adapter',

  'sdkwork_im_web_bootstrap',

]) {

  assert.match(

    rootCargo,

    new RegExp(`${dependencyKey}\\s*=\\s*\\{[^}]*sdkwork-web-framework|sdkwork-appbase|sdkwork-im-web-bootstrap`, 'u'),

    `Cargo.toml must declare workspace dependency ${dependencyKey} for sdkwork-web-framework integration`,

  );

}



const bootstrapCargo = read('crates/sdkwork-im-web-bootstrap/Cargo.toml');

assert.match(bootstrapCargo, /sdkwork_web_axum\.workspace\s*=\s*true/u);

assert.match(bootstrapCargo, /sdkwork_iam_web_adapter\.workspace\s*=\s*true/u);



const bootstrapLib = read('crates/sdkwork-im-web-bootstrap/src/lib.rs');

assert.match(bootstrapLib, /WebFrameworkLayer::new/u);

assert.match(bootstrapLib, /with_web_request_context/u);

assert.match(bootstrapLib, /ImAppContextInjector/u);

assert.match(bootstrapLib, /\/im\/v3\/api/u);
assert.match(bootstrapLib, /IamWebRequestContextResolver/u);
assert.match(bootstrapLib, /iam_web_request_context_resolver_from_env/u);
assert.doesNotMatch(
  bootstrapLib,
  /IamDatabaseWebRequestContextResolver/u,
  'im web bootstrap must use the canonical IAM web request context resolver type',
);
assert.doesNotMatch(
  bootstrapLib,
  /iam_database_resolver_from_env/u,
  'im web bootstrap must use the canonical IAM web request context resolver factory',
);



const gatewayCargo = read('services/sdkwork-im-cloud-gateway/Cargo.toml');

assert.match(gatewayCargo, /sdkwork_web_axum\.workspace\s*=\s*true/u);

assert.match(gatewayCargo, /sdkwork_web_bootstrap\.workspace\s*=\s*true/u);

assert.match(gatewayCargo, /sdkwork_iam_web_adapter\.workspace\s*=\s*true/u);



const gatewayLib = read('services/sdkwork-im-cloud-gateway/src/lib.rs');

const gatewayApp = read('services/sdkwork-im-cloud-gateway/src/app.rs');

const gatewayWebFramework = read('services/sdkwork-im-cloud-gateway/src/web_framework.rs');

assert.match(gatewayLib, /mod web_framework;/u);

assert.match(gatewayApp, /web_framework::wrap_gateway_router/u);

assert.match(gatewayApp, /RealtimeAuthContextResolver/u);

assert.match(
  gatewayApp,
  /resolve_iam_auth_pool_from_env/u,
  'gateway must bootstrap realtime websocket auth from IAM database pool when configured',
);

assert.match(gatewayWebFramework, /WebFramework::builder/u);

assert.match(gatewayWebFramework, /with_web_request_context/u);

assert.match(gatewayWebFramework, /service_router/u);

assert.match(gatewayWebFramework, /IamWebRequestContextResolver/u);

assert.match(
  gatewayWebFramework,
  /iam_web_request_context_resolver_from_env/u,
  'gateway web framework must wire the canonical IAM web request context resolver from environment',
);
assert.doesNotMatch(
  gatewayWebFramework,
  /IamDatabaseWebRequestContextResolver/u,
  'gateway web framework must not reference the concrete IAM database resolver type in application integration',
);
assert.doesNotMatch(
  gatewayWebFramework,
  /iam_database_resolver_from_env/u,
  'gateway web framework must not keep the legacy IAM database resolver factory name',
);

assert.match(gatewayWebFramework, /IM_APP_API_PREFIX/u);

assert.match(gatewayWebFramework, /REALTIME_WS/u);

assert.match(gatewayWebFramework, /\/ws\//u);

const gatewayHttpProxyTest = read('services/sdkwork-im-cloud-gateway/tests/http_proxy_test.rs');
assert.match(
  gatewayHttpProxyTest,
  /SDKWORK_IM_ENVIRONMENT[\s\S]*SDKWORK_ENV/,
  'gateway http proxy tests must bootstrap both IM and IAM test environments for dual-token fallback',
);

for (const relativePath of [
  'crates/sdkwork-routes-im-social-open-api/src/web_bootstrap.rs',
  'crates/sdkwork-routes-im-social-backend-api/src/web_bootstrap.rs',
  'crates/sdkwork-routes-im-space-open-api/src/web_bootstrap.rs',
  'crates/sdkwork-routes-im-chat-open-api/src/web_bootstrap.rs',
  'crates/sdkwork-routes-im-realtime-open-api/src/web_bootstrap.rs',
  'crates/sdkwork-routes-im-media-app-api/src/web_bootstrap.rs',
  'crates/sdkwork-routes-im-automation-app-api/src/web_bootstrap.rs',
  'crates/sdkwork-routes-im-notification-app-api/src/web_bootstrap.rs',
  'crates/sdkwork-routes-im-stream-app-api/src/web_bootstrap.rs',
  'crates/sdkwork-routes-im-audit-backend-api/src/web_bootstrap.rs',
  'crates/sdkwork-routes-im-ops-backend-api/src/web_bootstrap.rs',
  'crates/sdkwork-routes-im-governance-backend-api/src/web_bootstrap.rs',
  'crates/sdkwork-routes-im-projection-open-api/src/web_bootstrap.rs',
  'crates/sdkwork-routes-im-calls-open-api/src/web_bootstrap.rs',
]) {
  const source = read(relativePath);
  assert.match(
    source,
    /wrap_im_(open_api_)?service_router/u,
    `${relativePath} must wrap HTTP routers through sdkwork-im-web-bootstrap`,
  );
}

for (const relativePath of [
  'services/social-service/src/http.rs',
  'services/social-service/src/postgres/http.rs',
  'services/space-service/src/http.rs',
  'services/interaction-service/src/http.rs',
  'services/sdkwork-comms-conversation-service/src/runtime/http.rs',
  'services/session-gateway/src/lib.rs',
  'services/audit-service/src/lib.rs',
  'services/ops-service/src/lib.rs',
  'services/notification-service/src/lib.rs',
  'services/media-service/src/lib.rs',
  'services/streaming-service/src/lib.rs',
  'services/governance-service/src/lib.rs',
  'services/projection-service/src/http.rs',
  'services/im-calls-service/src/app.rs',
  'services/automation-service/src/lib.rs',
]) {
  const source = read(relativePath);
  assert.doesNotMatch(
    source,
    /wrap_im_(open_api_)?service_router/u,
    `${relativePath} must not wrap routers locally; route crates own web-framework wrapping`,
  );

  assert.doesNotMatch(
    source,
    /layer\(middleware::from_fn_with_state\([^)]*,\s*require_app_context\s*\)/u,
    `${relativePath} must not layer legacy im-app-context middleware after web-framework migration`,
  );

  assert.doesNotMatch(
    source,
    /apply_public_http_guardrails[\s\S]*?layer\(middleware::from_fn_with_state\([^)]*,\s*require_app_context\s*\)/u,
    `${relativePath} must not wrap public apps with legacy im-app-context middleware`,
  );

}



const appContextLib = read('crates/im-app-context/src/lib.rs');

assert.match(

  appContextLib,

  /TenantBoundJwtVerifier/u,

  'im-app-context must verify production JWT signatures through tenant-bound verifier',

);

assert.match(

  appContextLib,

  /EnvBootstrapTenantSigningKeyLookup/u,

  'im-app-context must resolve tenant signing keys from env bootstrap lookup',

);

assert.match(

  appContextLib,

  /SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET/u,

  'im-app-context must document JWT signing secret env for production verification',

);

assert.match(

  appContextLib,

  /pub fn app_context_from_web_request/u,

  'im-app-context must map WebRequestContext into AppContext for domain injectors',

);



const materializeScript = read('sdks/materialize-im-v3-openapi-boundaries.mjs');

assert.match(

  materializeScript,

  /applyWebFrameworkOpenApiExtensions/u,

  'OpenAPI materializer must emit web-framework request-context and api-surface extensions',

);



for (const target of IM_OPENAPI_AUTHORITY_TARGETS) {

  const yaml = read(target.relativePath);

  assert.match(

    yaml,

    /x-sdkwork-request-context:\s+WebRequestContext/u,

    `${target.relativePath} must declare x-sdkwork-request-context on operations`,

  );

  assert.match(

    yaml,

    new RegExp(`x-sdkwork-api-surface:\\s+${target.apiSurface}`, 'u'),

    `${target.relativePath} must declare x-sdkwork-api-surface=${target.apiSurface}`,

  );

  const apisYaml = read(target.apisAuthorityPath);

  assert.equal(

    apisYaml,

    yaml,

    `${target.apisAuthorityPath} must stay byte-identical to ${target.relativePath}`,

  );

}



const specsReadme = read('specs/README.md');

assert.match(specsReadme, /WEB_FRAMEWORK_SPEC\.md/u);

assert.match(specsReadme, /WEB_BACKEND_SPEC\.md/u);



process.stdout.write('sdkwork-im web framework standard contract passed\n');

