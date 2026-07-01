import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function readText(...segments) {
  return fs.readFileSync(path.join(repoRoot, ...segments), 'utf8').replace(/\r\n/gu, '\n');
}

const interactionHttp = readText('services', 'interaction-service', 'src', 'http.rs');
assert.doesNotMatch(
  interactionHttp,
  /\/im\/v3\/api\/interactions/u,
  'interaction-service must not mount retired /im/v3/api/interactions/* routes; chat OpenAPI owns those paths.',
);
assert.match(
  interactionHttp,
  /mount_im_infra_routes/u,
  'interaction-service must keep infra health routes for workspace membership smoke checks.',
);

const interactionSpec = JSON.parse(
  readText('services', 'interaction-service', 'specs', 'component.spec.json'),
);
assert.equal(
  interactionSpec.status,
  'deprecated',
  'interaction-service component spec must remain deprecated.',
);

const contactLib = readText('services', 'contact-service', 'src', 'lib.rs');
assert.match(
  contactLib,
  /pub use social_service::/u,
  'contact-service must remain a compatibility shim that re-exports social-service Postgres handlers.',
);

const gatewayConfig = readText('crates', 'sdkwork-im-cloud-gateway-config', 'src', 'lib.rs');
assert.match(
  gatewayConfig,
  /upstream_base_url\("interaction-service"\),\s*None/u,
  'gateway config must not route to interaction-service upstream.',
);

const interactionCargo = readText('services', 'interaction-service', 'Cargo.toml');
assert.doesNotMatch(
  interactionCargo,
  /im-domain-core/u,
  'interaction-service must not depend on im-domain-core after retiring interaction HTTP handlers.',
);
assert.match(
  interactionCargo,
  /im-app-context/u,
  'interaction-service must depend on im-app-context for request-context middleware.',
);

console.log('sdkwork-im deprecated service boundary contract passed');
