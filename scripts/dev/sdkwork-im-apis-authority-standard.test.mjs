#!/usr/bin/env node
import assert from 'node:assert/strict';
import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function sha256(relativePath) {
  return crypto.createHash('sha256').update(read(relativePath)).digest('hex');
}

const authorityPairs = [
  {
    apisPath: 'apis/open-api/im/sdkwork-im-im.openapi.yaml',
    sdkMirrorPath: 'sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml',
    assemblyPath: 'sdks/sdkwork-im-sdk/.sdkwork-assembly.json',
    componentSpecPath: 'sdks/sdkwork-im-sdk/specs/component.spec.json',
    expectedAuthoritySpec: '../../apis/open-api/im/sdkwork-im-im.openapi.yaml',
  },
  {
    apisPath: 'apis/app-api/communication/sdkwork-im-app-api.openapi.yaml',
    sdkMirrorPath: 'sdks/sdkwork-im-app-sdk/openapi/sdkwork-im-app-api.openapi.yaml',
    assemblyPath: 'sdks/sdkwork-im-app-sdk/.sdkwork-assembly.json',
    componentSpecPath: 'sdks/sdkwork-im-app-sdk/specs/component.spec.json',
    expectedAuthoritySpec: '../../apis/app-api/communication/sdkwork-im-app-api.openapi.yaml',
  },
  {
    apisPath: 'apis/backend-api/communication/sdkwork-im-backend-api.openapi.yaml',
    sdkMirrorPath: 'sdks/sdkwork-im-backend-sdk/openapi/sdkwork-im-backend-api.openapi.yaml',
    assemblyPath: 'sdks/sdkwork-im-backend-sdk/.sdkwork-assembly.json',
    componentSpecPath: 'sdks/sdkwork-im-backend-sdk/specs/component.spec.json',
    expectedAuthoritySpec: '../../apis/backend-api/communication/sdkwork-im-backend-api.openapi.yaml',
  },
];

for (const entry of authorityPairs) {
  assert.ok(fs.existsSync(path.join(repoRoot, entry.apisPath)), `${entry.apisPath} must exist`);
  assert.ok(
    fs.existsSync(path.join(repoRoot, entry.sdkMirrorPath)),
    `${entry.sdkMirrorPath} must remain as SDK materialization mirror`,
  );
  assert.equal(
    sha256(entry.apisPath),
    sha256(entry.sdkMirrorPath),
    `${entry.apisPath} must match ${entry.sdkMirrorPath} until SDK mirrors are retired`,
  );

  const assembly = JSON.parse(read(entry.assemblyPath));
  assert.equal(assembly.authoritySpec, entry.expectedAuthoritySpec);

  const componentSpec = JSON.parse(read(entry.componentSpecPath));
  const authorityOpenApi = componentSpec.contracts?.apiAuthority?.authorityOpenApi
    ?? componentSpec.sdkFamilies?.[0]?.authorityOpenApi
    ?? componentSpec.httpSdkFamily?.authorityOpenApi;
  assert.equal(authorityOpenApi, entry.expectedAuthoritySpec);
}

const apisReadme = read('apis/README.md');
assert.match(apisReadme, /apis\/open-api\//u);
assert.match(apisReadme, /apis\/app-api\//u);
assert.match(apisReadme, /apis\/backend-api\//u);
assert.match(apisReadme, /apis\/rpc\//u);

process.stdout.write('sdkwork-im APIs authority standard contract passed\n');
