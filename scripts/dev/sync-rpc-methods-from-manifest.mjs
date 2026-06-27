import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const manifestPath = path.join(repoRoot, 'sdks/sdkwork-im-rpc-sdk/rpc/sdkwork-im-rpc.manifest.json');
const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));

assert.equal(manifest.kind, 'sdkwork.rpc.manifest');

for (const language of ['typescript', 'go', 'java', 'python', 'rust']) {
  const catalogPath = path.join(
    repoRoot,
    `sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-${language}/rpc-methods.json`,
  );
  const catalog = JSON.parse(fs.readFileSync(catalogPath, 'utf8'));
  assert.equal(catalog.kind, 'sdkwork.rpc.methodCatalog');

  catalog.language = language;
  catalog.services = manifest.services.map((service) => ({
    package: service.package,
    service: service.service,
    surface: service.surface,
    methodCount: service.methods.length,
  }));
  catalog.methods = manifest.services.flatMap((service) =>
    service.methods.map((method) => ({
      methodKey: `${service.package}.${service.service}/${method.method}`,
      package: service.package,
      service: service.service,
      method: method.method,
      surface: service.surface,
      operationId: method.operationId,
      auth: method.auth,
      idempotency: method.idempotency,
      streaming: method.streaming,
      owner: method.owner,
      compatibility: method.compatibility,
    })),
  );

  fs.writeFileSync(catalogPath, `${JSON.stringify(catalog, null, 2)}\n`);
  console.log(`synced ${language}: ${catalog.services.length} services, ${catalog.methods.length} methods`);
}
