#!/usr/bin/env node

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  removeImTemporaryDirectory,
  resolveImRuntimeStateDirectory,
} from './lib/im-temporary-state.mjs';

const MODULE_PATH = fileURLToPath(import.meta.url);
const REPO_ROOT = path.resolve(path.dirname(MODULE_PATH), '..');

export function cleanSdkworkIm({
  removeDirectory = (directory) => fs.rmSync(directory, { force: true, recursive: true }),
  repoRoot = REPO_ROOT,
  runtimeStateOptions = {},
} = {}) {
  const generatedDirectories = [
    path.join(repoRoot, 'apps', 'sdkwork-im-pc', 'dist'),
    path.join(repoRoot, 'apps', 'sdkwork-im-h5', 'dist'),
    path.join(repoRoot, 'target', 'sdkwork', 'sdkwork-api-im-standalone-gateway-dev'),
  ];
  for (const directory of generatedDirectories) removeDirectory(directory);

  const devSitesDirectory = resolveImRuntimeStateDirectory({
    ...runtimeStateOptions,
    purpose: 'dev-sites',
    repoRoot,
  });
  removeImTemporaryDirectory(devSitesDirectory, { repoRoot, ...runtimeStateOptions });
  return { devSitesDirectory, generatedDirectories };
}

if (process.argv[1] && path.resolve(process.argv[1]) === MODULE_PATH) {
  cleanSdkworkIm();
}
