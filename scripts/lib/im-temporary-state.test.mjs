import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import {
  createImTemporaryDirectory,
  removeImRuntimeStateFile,
  removeImTemporaryDirectory,
  resolveImRuntimeStateDirectory,
  resolveImTemporaryFilePath,
  writeImPrivateFile,
  writeImPrivateJsonFile,
} from './im-temporary-state.mjs';

test('creates isolated temporary state outside the repository and cleans it safely', () => {
  const temporaryDirectory = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-temp-root-'));
  const repoRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-temp-repo-'));
  const options = { env: {}, repoRoot, temporaryDirectory };
  const first = createImTemporaryDirectory({ ...options, purpose: 'release' });
  const second = createImTemporaryDirectory({ ...options, purpose: 'release' });
  assert.notEqual(first, second);
  assert.equal(path.relative(repoRoot, first).startsWith('..'), true);
  fs.writeFileSync(path.join(first, 'private.bin'), 'secret', { mode: 0o600 });
  removeImTemporaryDirectory(first, options);
  assert.equal(fs.existsSync(first), false);
  assert.equal(fs.existsSync(second), true);
  removeImTemporaryDirectory(second, options);
});

test('resolves deterministic process state outside the repository', () => {
  const temporaryDirectory = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-state-root-'));
  const repoRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-state-repo-'));
  const state = resolveImRuntimeStateDirectory({
    env: {},
    purpose: 'dev-sites',
    repoRoot,
    temporaryDirectory,
  });
  assert.equal(state.startsWith(path.join(temporaryDirectory, 'sdkwork', 'sdkwork-im')), true);
  assert.equal(state.endsWith('dev-sites'), true);
});

test('rejects traversal and cleanup outside the owned state root', () => {
  const repoRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-temp-guard-'));
  assert.throws(
    () => createImTemporaryDirectory({ repoRoot, purpose: '../escape' }),
    /lowercase kebab-case/u,
  );
  assert.throws(
    () => removeImTemporaryDirectory(os.tmpdir(), { repoRoot }),
    /must target one child/u,
  );
  assert.throws(
    () => writeImPrivateFile(path.join(repoRoot, 'escape.bin'), 'no', { repoRoot }),
    /must be inside/u,
  );
});

test('writes private temporary files and removes only the selected file', () => {
  const temporaryDirectory = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-private-root-'));
  const repoRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-private-repo-'));
  const options = { env: {}, repoRoot, temporaryDirectory };
  const jsonPath = resolveImTemporaryFilePath({
    ...options,
    extension: '.json',
    fileName: 'config',
    purpose: 'release',
  });
  const binaryPath = resolveImTemporaryFilePath({
    ...options,
    extension: '.bin',
    fileName: 'signing',
    purpose: 'release',
  });
  writeImPrivateJsonFile(jsonPath, { ok: true }, options);
  writeImPrivateFile(binaryPath, Buffer.from('private'), options);
  assert.deepEqual(JSON.parse(fs.readFileSync(jsonPath, 'utf8')), { ok: true });
  assert.equal(fs.readFileSync(binaryPath, 'utf8'), 'private');
  removeImRuntimeStateFile(jsonPath, options);
  assert.equal(fs.existsSync(jsonPath), false);
  assert.equal(fs.existsSync(binaryPath), true);
  removeImRuntimeStateFile(binaryPath, options);
});
