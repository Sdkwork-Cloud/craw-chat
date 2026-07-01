#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { IM_ROUTE_CRATES } from './sdkwork-im-web-backend-route-crates.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function assertFile(relativePath) {
  assert.ok(fs.existsSync(path.join(repoRoot, relativePath)), `${relativePath} must exist`);
}

const rootCargo = read('Cargo.toml');
for (const entry of IM_ROUTE_CRATES) {
  assertFile(`${entry.crateDir}/Cargo.toml`);
  assertFile(`${entry.crateDir}/README.md`);
  assertFile(`${entry.crateDir}/src/lib.rs`);
  assertFile(`${entry.crateDir}/src/paths.rs`);
  assertFile(`${entry.crateDir}/src/manifest.rs`);
  assertFile(`${entry.crateDir}/src/routes.rs`);
  assertFile(`${entry.crateDir}/src/web_bootstrap.rs`);

  assert.match(
    rootCargo,
    new RegExp(`"${entry.crateDir.replaceAll('\\', '/')}"`, 'u'),
    `Cargo.toml workspace members must include ${entry.crateDir}`,
  );

  const manifest = read(`${entry.crateDir}/src/manifest.rs`);
  assert.match(manifest, /HttpRoute/u, `${entry.crateDir} manifest must declare HttpRoute metadata`);
  assert.ok(
    manifest.includes(entry.apiSurface),
    `${entry.crateDir} manifest must reference api surface ${entry.apiSurface}`,
  );

  const paths = read(`${entry.crateDir}/src/paths.rs`);
  assert.match(
    paths,
    new RegExp(entry.pathPrefix.replace(/\//g, '\\/'), 'u'),
    `${entry.crateDir} paths must declare prefix ${entry.pathPrefix}`,
  );

  const routes = read(`${entry.crateDir}/src/routes.rs`);
  assert.match(
    routes,
    /\.route\(|build_[a-z0-9_]*domain_api_router|build_[a-z0-9_]*api_router/u,
    `${entry.crateDir}/src/routes.rs must mount HTTP routes directly or delegate to service domain routers`,
  );
  assert.doesNotMatch(
    routes,
    /inject_app_request_context_middleware/u,
    `${entry.crateDir} must not use legacy im-app-context middleware`,
  );

  const bootstrap = read(`${entry.crateDir}/src/web_bootstrap.rs`);
  assert.match(
    bootstrap,
    /wrap_im_(open_api_)?service_router/u,
    `${entry.crateDir} must wrap routers through sdkwork-im-web-bootstrap`,
  );

  const lib = read(`${entry.crateDir}/src/lib.rs`);
  assert.match(
    lib,
    new RegExp(`pub fn ${entry.buildFn}`, 'u'),
    `${entry.crateDir} must export ${entry.buildFn}`,
  );
}

for (const relativePath of [
  'services/social-service/src/postgres/http.rs',
  'services/social-service/src/http.rs',
  'services/space-service/src/http.rs',
  'services/sdkwork-comms-conversation-service/src/runtime/http.rs',
  'services/session-gateway/src/lib.rs',
  'services/media-service/src/lib.rs',
  'services/automation-service/src/lib.rs',
  'services/notification-service/src/lib.rs',
  'services/streaming-service/src/lib.rs',
  'services/audit-service/src/lib.rs',
  'services/ops-service/src/lib.rs',
  'services/governance-service/src/lib.rs',
  'services/projection-service/src/http.rs',
  'services/im-calls-service/src/app.rs',
]) {
  const source = read(relativePath);
  assert.doesNotMatch(
    source,
    /wrap_im_(open_api_)?service_router/u,
    `${relativePath} must not wrap routers locally; route crates own HTTP mounting and web-framework wrapping`,
  );
}

const specsReadme = read('specs/README.md');
assert.match(specsReadme, /WEB_BACKEND_SPEC\.md/u);

process.stdout.write('sdkwork-im web backend standard contract passed\n');
