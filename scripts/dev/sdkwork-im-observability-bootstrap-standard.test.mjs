import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, ...relativePath.split('/')), 'utf8');
}

function readExists(relativePath) {
  const absolutePath = path.join(repoRoot, ...relativePath.split('/'));
  assert.ok(fs.existsSync(absolutePath), `expected file ${relativePath}`);
  return fs.readFileSync(absolutePath, 'utf8');
}

function listServiceProcessEntrypoints() {
  const servicesDir = path.join(repoRoot, 'services');
  const entrypoints = [];
  for (const entry of fs.readdirSync(servicesDir, { withFileTypes: true })) {
    if (!entry.isDirectory()) {
      continue;
    }
    const mainPath = path.join(servicesDir, entry.name, 'src', 'main.rs');
    if (fs.existsSync(mainPath)) {
      entrypoints.push(path.relative(repoRoot, mainPath).replace(/\\/g, '/'));
    }
  }
  return entrypoints.sort();
}

for (const entrypoint of listServiceProcessEntrypoints()) {
  const source = readExists(entrypoint);
  assert.doesNotMatch(
    source,
    /tracing_subscriber::fmt\(\)/,
    `${entrypoint} must use sdkwork-im-service-readiness tracing bootstrap instead of ad hoc tracing_subscriber::fmt`,
  );
  assert.match(
    source,
    /init_im_service_tracing_from_env\(\)/,
    `${entrypoint} must call init_im_service_tracing_from_env`,
  );
  assert.match(
    source,
    /ensure_im_service_process_identity\(/,
    `${entrypoint} must call ensure_im_service_process_identity for stable metrics and OTel service names`,
  );
}

const serviceReadiness = readExists('crates/sdkwork-im-service-readiness/src/lib.rs');
assert.match(
  serviceReadiness,
  /pub fn init_im_service_tracing_from_env\(\)/,
  'sdkwork-im-service-readiness must expose init_im_service_tracing_from_env',
);
assert.match(
  serviceReadiness,
  /pub fn ensure_im_service_process_identity\(/,
  'sdkwork-im-service-readiness must expose ensure_im_service_process_identity',
);

const webBootstrap = readExists('crates/sdkwork-im-web-bootstrap/Cargo.toml');
assert.match(
  webBootstrap,
  /features = \["otel"\]/,
  'sdkwork-im-web-bootstrap must enable sdkwork-web-bootstrap otel feature',
);

const envExample = read('.env.postgres.example');
assert.ok(
  envExample.includes('OTEL_EXPORTER_OTLP_ENDPOINT'),
  '.env.postgres.example must document optional OTEL_EXPORTER_OTLP_ENDPOINT',
);

process.stdout.write('sdkwork-im observability bootstrap standard passed\n');
