#!/usr/bin/env node

import { createRequire } from 'node:module';
import process from 'node:process';

import {
  ensureLocalNodeModules,
  resolveReadablePackageEntry,
  resolveWorkspaceDonorRoots,
} from './vite-runtime-lib.mjs';

const REQUIRED_APP_PACKAGES = [
  'esbuild',
];

export function resolveReadableEsbuildCliPath({
  appRoot,
  donorRoots = resolveWorkspaceDonorRoots(appRoot),
} = {}) {
  if (!appRoot) {
    throw new Error('appRoot is required');
  }

  return resolveReadablePackageEntry({
    appRoot,
    donorRoots,
    packageName: 'esbuild',
    relativeEntry: ['bin', 'esbuild'],
  });
}

const appRoot = process.cwd();
const donorRoots = resolveWorkspaceDonorRoots(appRoot);
ensureLocalNodeModules({
  appRoot,
  donorRoots,
  requiredPackages: REQUIRED_APP_PACKAGES,
});
const esbuildCliPath = resolveReadableEsbuildCliPath({ appRoot, donorRoots });

process.argv = [
  process.argv[0],
  esbuildCliPath,
  ...process.argv.slice(2),
];

createRequire(import.meta.url)(esbuildCliPath);
