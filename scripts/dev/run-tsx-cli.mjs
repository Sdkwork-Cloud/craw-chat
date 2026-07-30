#!/usr/bin/env node

import process from 'node:process';
import { pathToFileURL } from 'node:url';
import fs from 'node:fs';
import path from 'node:path';

import {
  ensureLocalNodeModules,
  resolveReadablePackageEntry,
  resolveWorkspaceDonorRoots,
} from './vite-runtime-lib.mjs';
import {
  createImTemporaryDirectory,
  removeImTemporaryDirectory,
} from '../lib/im-temporary-state.mjs';

const REQUIRED_APP_PACKAGES = [
  'tsx',
];
const RUNTIME_UNSAFE_PATH_ALIASES = [
  'react',
  'react/jsx-runtime',
  'react-dom',
  'react-router-dom',
];

function hasExplicitTsconfig(args) {
  return args.some((arg) => arg === '--tsconfig' || arg.startsWith('--tsconfig='));
}

function materializeRuntimeTsconfig({ appRoot, tsconfigPath = path.join(appRoot, 'tsconfig.json') }) {
  if (!fs.existsSync(tsconfigPath)) {
    return undefined;
  }

  const tsconfig = JSON.parse(fs.readFileSync(tsconfigPath, 'utf8'));
  const paths = tsconfig.compilerOptions?.paths;
  if (!paths || typeof paths !== 'object') {
    return undefined;
  }

  const runtimePaths = { ...paths };
  let changed = false;
  for (const alias of RUNTIME_UNSAFE_PATH_ALIASES) {
    if (Object.hasOwn(runtimePaths, alias)) {
      delete runtimePaths[alias];
      changed = true;
    }
  }

  if (!changed) {
    return undefined;
  }

  const runtimeConfigDir = createImTemporaryDirectory({ repoRoot: appRoot, purpose: 'tsx' });
  const runtimeConfigPath = path.join(runtimeConfigDir, 'tsconfig.runtime.json');
  const extendsPath = path.relative(runtimeConfigDir, tsconfigPath).replaceAll(path.sep, '/');
  const baseUrl = path.relative(runtimeConfigDir, appRoot).replaceAll(path.sep, '/') || '.';
  const runtimeConfig = {
    extends: extendsPath,
    compilerOptions: {
      baseUrl,
      paths: runtimePaths,
    },
  };
  try {
    fs.writeFileSync(runtimeConfigPath, `${JSON.stringify(runtimeConfig, null, 2)}\n`, {
      encoding: 'utf8',
      mode: 0o600,
    });
    return { directory: runtimeConfigDir, path: runtimeConfigPath };
  } catch (error) {
    removeImTemporaryDirectory(runtimeConfigDir, { repoRoot: appRoot });
    throw error;
  }
}

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
const tsxArgs = process.argv.slice(2);
const runtimeTsconfig = hasExplicitTsconfig(tsxArgs)
  ? undefined
  : materializeRuntimeTsconfig({ appRoot });

process.argv = [
  process.argv[0],
  tsxCliPath,
  ...(runtimeTsconfig ? ['--tsconfig', runtimeTsconfig.path] : []),
  ...tsxArgs,
];

try {
  await import(pathToFileURL(tsxCliPath).href);
} finally {
  if (runtimeTsconfig) {
    removeImTemporaryDirectory(runtimeTsconfig.directory, { repoRoot: appRoot });
  }
}
