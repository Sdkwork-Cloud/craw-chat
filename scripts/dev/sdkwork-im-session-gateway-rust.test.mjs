#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

const result = spawnSync(
  'cargo',
  ['test', '-p', 'session-gateway', 'auth_context::tests', '--lib', '--', '--test-threads=1'],
  {
    cwd: repoRoot,
    stdio: 'inherit',
    shell: process.platform === 'win32',
  },
);

if (result.status !== 0) {
  process.exit(result.status ?? 1);
}

process.stdout.write('session-gateway rust auth contract passed\n');
