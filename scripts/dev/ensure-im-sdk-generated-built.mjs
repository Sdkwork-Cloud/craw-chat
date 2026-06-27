#!/usr/bin/env node

import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const scriptPath = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(scriptPath), '..', '..');
const generatedPackageDir = path.join(
  repoRoot,
  'sdks',
  'sdkwork-im-sdk',
  'sdkwork-im-sdk-typescript',
  'generated',
  'server-openapi',
);
const generatedDistEntry = path.join(generatedPackageDir, 'dist', 'index.js');

export function isImSdkGeneratedTransportBuilt() {
  return fs.existsSync(generatedDistEntry);
}

export function ensureImSdkGeneratedTransportBuilt({ quiet = false } = {}) {
  if (isImSdkGeneratedTransportBuilt()) {
    return { built: false, status: 0 };
  }

  const pnpmExecutable = process.platform === 'win32' ? 'pnpm.cmd' : 'pnpm';
  const result = spawnSync(
    pnpmExecutable,
    ['--dir', generatedPackageDir, 'run', 'build'],
    {
      cwd: repoRoot,
      encoding: 'utf8',
      shell: process.platform === 'win32',
      stdio: quiet ? 'pipe' : 'inherit',
    },
  );

  if (result.status !== 0) {
    const details = [result.stdout, result.stderr].filter(Boolean).join('\n').trim();
    throw new Error(
      details
        ? `failed to build @sdkwork/im-sdk-generated transport:\n${details}`
        : 'failed to build @sdkwork/im-sdk-generated transport',
    );
  }

  return { built: true, status: 0 };
}

if (process.argv[1] && path.resolve(process.argv[1]) === scriptPath) {
  ensureImSdkGeneratedTransportBuilt();
}
