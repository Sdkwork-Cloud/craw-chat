import assert from 'node:assert/strict';
import { spawn, spawnSync } from 'node:child_process';
import path from 'node:path';
import test from 'node:test';

const repoRoot = path.resolve(import.meta.dirname, '..', '..');
const appRoot = path.join(repoRoot, 'apps', 'sdkwork-im-pc');
const runnerArgs = [
  path.join(repoRoot, 'scripts', 'dev', 'run-tsx-cli.mjs'),
  path.join(repoRoot, 'scripts', 'dev', 'sdkwork-im-pc-group-service-client-injection.test.ts'),
];

function runPcGroupServiceScript() {
  const child = spawn(process.execPath, runnerArgs, {
    cwd: appRoot,
    shell: false,
    windowsHide: process.platform === 'win32',
  });
  let stdout = '';
  let stderr = '';
  child.stdout.setEncoding('utf8');
  child.stderr.setEncoding('utf8');
  child.stdout.on('data', (chunk) => {
    stdout += chunk;
  });
  child.stderr.on('data', (chunk) => {
    stderr += chunk;
  });
  return new Promise((resolve, reject) => {
    child.on('error', reject);
    child.on('close', (status) => {
      resolve({ status, stdout, stderr });
    });
  });
}

test('run-tsx-cli executes PC service contract scripts without runtime type-definition path aliases', () => {
  const result = spawnSync(
    process.execPath,
    runnerArgs,
    {
      cwd: appRoot,
      encoding: 'utf8',
      shell: false,
      windowsHide: process.platform === 'win32',
    },
  );

  assert.equal(
    result.status,
    0,
    [
      'run-tsx-cli failed to execute a PC service contract script from the app root',
      result.stdout.trim(),
      result.stderr.trim(),
    ].filter(Boolean).join('\n'),
  );
  assert.match(result.stdout, /sdkwork-im-pc group service client injection contract passed/u);
  assert.equal(result.stderr.trim(), '');
});

test('run-tsx-cli materializes PC runtime tsconfig safely for concurrent script tests', async () => {
  const results = await Promise.all([
    runPcGroupServiceScript(),
    runPcGroupServiceScript(),
  ]);

  for (const [index, result] of results.entries()) {
    assert.equal(
      result.status,
      0,
      [
        `run-tsx-cli concurrent PC script ${index + 1} failed`,
        result.stdout.trim(),
        result.stderr.trim(),
      ].filter(Boolean).join('\n'),
    );
    assert.match(result.stdout, /sdkwork-im-pc group service client injection contract passed/u);
    assert.equal(result.stderr.trim(), '');
  }
});
