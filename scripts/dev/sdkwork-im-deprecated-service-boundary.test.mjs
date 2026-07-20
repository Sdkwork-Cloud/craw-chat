import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function readText(...segments) {
  return fs.readFileSync(path.join(repoRoot, ...segments), 'utf8').replace(/\r\n/gu, '\n');
}

for (const retiredService of ['contact-service', 'interaction-service']) {
  assert.equal(
    fs.existsSync(path.join(repoRoot, 'services', retiredService)),
    false,
    `${retiredService} must not remain as a compatibility crate`,
  );
  assert.doesNotMatch(readText('Cargo.toml'), new RegExp(`services/${retiredService}`, 'u'));
  assert.doesNotMatch(
    readText('deployments', 'docker', 'sdkwork-im-cloud-service.Dockerfile'),
    new RegExp(retiredService, 'u'),
  );
}

const gatewayConfig = readText('crates', 'sdkwork-api-im-standalone-gateway', 'src', 'lib.rs');
assert.doesNotMatch(gatewayConfig, /"(?:contact-service|interaction-service)"/u);

console.log('sdkwork-im removed service boundary contract passed');
