import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  ensurePostgresDevEnvFile,
  mergePostgresDevRuntimeEnv,
  resolvePostgresDevEnvExamplePath,
  resolvePostgresDevEnvFilePath,
} from './sdkwork-im-postgres-dev-profile.mjs';

const __filename = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(__filename), '..', '..');

function withTempRepo(testFn) {
  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-postgres-dev-'));
  try {
    testFn(tempRoot);
  } finally {
    fs.rmSync(tempRoot, { recursive: true, force: true });
  }
}

withTempRepo((tempRoot) => {
  const examplePath = path.join(tempRoot, '.env.postgres.example');
  fs.writeFileSync(examplePath, [
    'SDKWORK_IM_DATABASE_ENGINE=postgresql',
    'SDKWORK_IM_DATABASE_HOST=127.0.0.1',
    'SDKWORK_CLAW_DATABASE_NAME=sdkwork_ai_dev',
    'SDKWORK_CLAW_DATABASE_USERNAME=sdkwork_ai_dev',
    'SDKWORK_CLAW_DATABASE_PASSWORD=example',
    'SDKWORK_CLAW_DATABASE_ADMIN_PASSWORD=admin-example',
    '',
  ].join('\n'));
  const createdPath = ensurePostgresDevEnvFile(tempRoot);
  assert.equal(createdPath, resolvePostgresDevEnvFilePath(tempRoot));
  assert.ok(fs.existsSync(createdPath));
});

assert.equal(
  resolvePostgresDevEnvExamplePath(repoRoot),
  path.join(repoRoot, '.env.postgres.example'),
);

const merged = mergePostgresDevRuntimeEnv({
  env: {
    SDKWORK_CLAW_DATABASE_PASSWORD: 'shell-password',
    SDKWORK_IM_PC_DEV_PORT: '4176',
  },
  fileEnv: {
    SDKWORK_CLAW_DATABASE_PASSWORD: 'file-password',
    SDKWORK_CLAW_DATABASE_USERNAME: 'sdkwork_ai_dev',
  },
});
assert.equal(merged.SDKWORK_CLAW_DATABASE_PASSWORD, 'file-password');
assert.equal(merged.SDKWORK_IM_PC_DEV_PORT, '4176');

console.log('ensure-postgres-dev-database.test.mjs passed');
