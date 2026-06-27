import assert from 'node:assert/strict';
import { readdirSync, readFileSync, statSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const appRoot = path.resolve(__dirname, '..');
const packagesRoot = path.join(appRoot, 'packages');
const pcCoreIndexSource = readFileSync(
  path.join(packagesRoot, 'sdkwork-im-pc-core', 'src', 'index.ts'),
  'utf8',
);
const adminCoreSdkIndexSource = readFileSync(
  path.join(packagesRoot, 'sdkwork-im-admin-core', 'src', 'sdk', 'index.ts'),
  'utf8',
);

const backendSdkMarkers = [
  /@sdkwork-internal\/im-backend-api-generated/u,
    /@sdkwork\/iam-backend-sdk/u,
  /\bgetBackendSdkClient(?:WithSession)?\b/u,
  /\bgetAppbaseBackendSdkClient(?:WithSession)?\b/u,
  /\bresolveBackendSdkBaseUrl\b/u,
  /\bappbaseBackendApiBaseUrl\b/u,
  /\bresetBackendSdkClient\b/u,
  /\bresetAppbaseBackendSdkClient\b/u,
];

function listSourceFiles(root) {
  const files = [];
  for (const entry of readdirSync(root, { withFileTypes: true })) {
    const fullPath = path.join(root, entry.name);
    if (entry.isDirectory()) {
      if (entry.name === 'node_modules' || entry.name === 'dist' || entry.name === 'target') {
        continue;
      }
      files.push(...listSourceFiles(fullPath));
      continue;
    }
    if (entry.isFile() && /\.(?:ts|tsx)$/u.test(entry.name)) {
      files.push(fullPath);
    }
  }
  return files;
}

function isAdminSourceFile(filePath) {
  const normalized = filePath.split(path.sep).join('/');
  return normalized.includes('/packages/sdkwork-im-admin-')
    && normalized.includes('/src/');
}

assert.ok(statSync(packagesRoot).isDirectory(), 'Expected sdkwork-im-pc packages directory to exist.');

assert.match(
  pcCoreIndexSource,
  /export \* from ['"]\.\/sdk\/appSdkClient['"][\s\S]*export \* from ['"]\.\/sdk\/appbaseAppSdkClient['"]/u,
  'PC core must keep exporting product app SDK and appbase app SDK wrappers for frontend app integration.',
);
assert.doesNotMatch(
  pcCoreIndexSource,
  /backendSdkClient|appbaseBackendSdkClient/u,
  'PC core is an app/runtime export boundary and must not export backend SDK wrappers.',
);
assert.match(
  adminCoreSdkIndexSource,
  /export \* from ['"]\.\/backendSdkClient['"][\s\S]*export \* from ['"]\.\/appbaseBackendSdkClient['"]/u,
  'Admin core sdk subpath must export product backend SDK and appbase backend SDK wrappers for admin modules.',
);

const violations = [];

for (const filePath of listSourceFiles(packagesRoot)) {
  if (isAdminSourceFile(filePath)) {
    continue;
  }

  const source = readFileSync(filePath, 'utf8');
  for (const marker of backendSdkMarkers) {
    if (marker.test(source)) {
      violations.push(`${path.relative(appRoot, filePath)} :: ${marker.source}`);
    }
  }
}

assert.deepEqual(
  violations,
  [],
  [
    'Generated backend SDK clients are admin/operator-only.',
    'App, auth runtime, pc-core, and user-facing console packages must use app SDK or approved app wrappers instead.',
    ...violations,
  ].join('\n'),
);

console.log('sdkwork im pc backend SDK surface boundary contract passed.');
