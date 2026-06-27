#!/usr/bin/env node

import process from 'node:process';
import { pathToFileURL } from 'node:url';

import {
  ensureLocalNodeModules,
  resolveReadablePackageEntry,
  resolveWorkspaceDonorRoots,
} from './vite-runtime-lib.mjs';
import { ensureSdkworkUiDist } from './sdkwork-ui-runtime-lib.mjs';

const REQUIRED_APP_PACKAGES = [
  '@sdkwork/ui-pc-react',
  '@tailwindcss/vite',
  '@types/react',
  '@types/react-dom',
  '@vitejs/plugin-react',
  'lucide-react',
  'react',
  'react-dom',
  'react-router-dom',
  'tailwindcss',
  'typescript',
  'vite',
];

export function resolveReadableTypeScriptCliPath({
  appRoot,
  donorRoots = resolveWorkspaceDonorRoots(appRoot),
} = {}) {
  if (!appRoot) {
    throw new Error('appRoot is required');
  }

  return resolveReadablePackageEntry({
    appRoot,
    donorRoots,
    packageName: 'typescript',
    relativeEntry: ['lib', 'tsc.js'],
  });
}

const appRoot = process.cwd();
const donorRoots = resolveWorkspaceDonorRoots(appRoot);
ensureSdkworkUiDist({ appRoot });
ensureLocalNodeModules({
  appRoot,
  donorRoots,
  requiredPackages: REQUIRED_APP_PACKAGES,
});
const tscCliPath = resolveReadableTypeScriptCliPath({ appRoot, donorRoots });

process.argv = [
  process.argv[0],
  tscCliPath,
  ...process.argv.slice(2),
];

await import(pathToFileURL(tscCliPath).href);
