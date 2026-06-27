#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const appRoot = path.join(repoRoot, 'apps/sdkwork-im-pc');
const testScript = path.join(repoRoot, 'scripts/dev/sdkwork-im-pc-request-context.test.ts');

const result = spawnSync('pnpm', ['exec', 'tsx', testScript], {
  cwd: appRoot,
  stdio: 'inherit',
  shell: process.platform === 'win32',
});

if (result.status !== 0) {
  process.exit(result.status ?? 1);
}

process.stdout.write('sdkwork-im-pc request context contract runner passed\n');
