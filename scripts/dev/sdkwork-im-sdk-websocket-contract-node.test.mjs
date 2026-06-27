import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import test from 'node:test';

import { ensureImSdkGeneratedTransportBuilt } from './ensure-im-sdk-generated-built.mjs';

const repoRoot = path.resolve(import.meta.dirname, '..', '..');

test('sdkwork im sdk websocket contract runs from the node entrypoint', () => {
  ensureImSdkGeneratedTransportBuilt({ quiet: true });
  const result = spawnSync(process.execPath, ['scripts/dev/sdkwork-im-sdk-websocket-contract.test.ts'], {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
    windowsHide: process.platform === 'win32',
  });

  assert.equal(
    result.status,
    0,
    [
      'sdkwork-im-sdk websocket contract node entrypoint failed',
      result.stdout.trim(),
      result.stderr.trim(),
    ].filter(Boolean).join('\n'),
  );
  assert.match(result.stdout, /sdkwork-im-sdk websocket contract passed/u);
  assert.equal(result.stderr.trim(), '');
});
