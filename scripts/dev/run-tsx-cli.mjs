#!/usr/bin/env node

import process from 'node:process';
import { pathToFileURL } from 'node:url';

import {
  ensureLocalNodeModules,
  resolveReadablePackageEntry,
  resolveWorkspaceDonorRoots,
} from './vite-runtime-lib.mjs';

const REQUIRED_APP_PACKAGES = [
  'tsx',
];

export function resolveReadableTsxCliPath({
  appRoot,
  donorRoots = resolveWorkspaceDonorRoots(appRoot),
} = {}) {
  if (!appRoot) {
    throw new Error('appRoot is required');
  }

  return resolveReadablePackageEntry({
    appRoot,
    donorRoots,
    packageName: 'tsx',
    relativeEntry: ['dist', 'cli.mjs'],
  });
}

const appRoot = process.cwd();
const donorRoots = resolveWorkspaceDonorRoots(appRoot);
ensureLocalNodeModules({
  appRoot,
  donorRoots,
  requiredPackages: REQUIRED_APP_PACKAGES,
});
const tsxCliPath = resolveReadableTsxCliPath({ appRoot, donorRoots });

process.argv = [
  process.argv[0],
  tsxCliPath,
  ...process.argv.slice(2),
];

await import(pathToFileURL(tsxCliPath).href);
