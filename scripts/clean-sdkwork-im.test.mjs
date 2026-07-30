import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import { cleanSdkworkIm } from './clean-sdkwork-im.mjs';
import { resolveImRuntimeStateDirectory } from './lib/im-temporary-state.mjs';

test('cleans only enumerated generated directories and the OS-owned dev site fallback', () => {
  const repoRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-clean-repo-'));
  const temporaryDirectory = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-clean-state-'));
  const runtimeStateOptions = { env: {}, temporaryDirectory };
  const preservedSource = path.join(repoRoot, 'apps', 'sdkwork-im-pc', 'build', 'package-contract.ts');
  const generatedDirectories = [
    path.join(repoRoot, 'apps', 'sdkwork-im-pc', 'dist'),
    path.join(repoRoot, 'apps', 'sdkwork-im-h5', 'dist'),
    path.join(repoRoot, 'target', 'sdkwork', 'sdkwork-api-im-standalone-gateway-dev'),
  ];
  const devSitesDirectory = resolveImRuntimeStateDirectory({
    ...runtimeStateOptions,
    purpose: 'dev-sites',
    repoRoot,
  });
  for (const directory of [...generatedDirectories, devSitesDirectory]) {
    fs.mkdirSync(directory, { recursive: true });
    fs.writeFileSync(path.join(directory, 'generated.txt'), 'generated');
  }
  fs.mkdirSync(path.dirname(preservedSource), { recursive: true });
  fs.writeFileSync(preservedSource, 'export const contract = true;\n');

  const result = cleanSdkworkIm({ repoRoot, runtimeStateOptions });

  assert.deepEqual(result.generatedDirectories, generatedDirectories);
  assert.equal(result.devSitesDirectory, devSitesDirectory);
  assert.ok(generatedDirectories.every((directory) => !fs.existsSync(directory)));
  assert.equal(fs.existsSync(devSitesDirectory), false);
  assert.equal(fs.existsSync(preservedSource), true);
  fs.rmSync(repoRoot, { force: true, recursive: true });
  fs.rmSync(temporaryDirectory, { force: true, recursive: true });
});
