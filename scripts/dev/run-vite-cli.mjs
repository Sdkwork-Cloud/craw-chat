#!/usr/bin/env node

import process from 'node:process';
import { pathToFileURL } from 'node:url';

import {
  applyWindowsVitePreload,
  ensureLocalNodeModules,
  resolveReadablePackageEntry,
  resolveWorkspaceDonorRoots,
} from './vite-runtime-lib.mjs';
import { ensureSdkworkUiDist } from './sdkwork-ui-runtime-lib.mjs';
import { mergeSdkworkImBootstrapAccessTokenEnv } from './sdkwork-im-bootstrap-access-token.mjs';

const REQUIRED_APP_PACKAGES = [
  '@sdkwork/rtc-sdk-provider-volcengine',
  '@sdkwork/ui-pc-react',
  '@tailwindcss/vite',
  '@tanstack/react-virtual',
  '@tiptap/react',
  '@vitejs/plugin-react',
  '@zxing/browser',
  '@zxing/library',
  'lucide-react',
  'motion',
  'react',
  'react-dom',
  'react-router',
  'react-router-dom',
  'tailwindcss',
  'vite',
];

const appRoot = process.cwd();
const donorRoots = resolveWorkspaceDonorRoots(appRoot);
ensureSdkworkUiDist({ appRoot });
ensureLocalNodeModules({
  appRoot,
  donorRoots,
  requiredPackages: REQUIRED_APP_PACKAGES,
});
const viteCliPath = resolveReadablePackageEntry({
  appRoot,
  donorRoots,
  packageName: 'vite',
  relativeEntry: ['bin', 'vite.js'],
});

await applyWindowsVitePreload();

if (!process.argv.slice(2).includes('build')) {
  // Dev server only: seed the private bootstrap SDKWORK_ACCESS_TOKEN fixture
  // into the renderer environment. IAM app-api/backend-api requests require an
  // Access-Token before dispatch; vite.config.ts surfaces it to the browser
  // through the credential-entry bootstrap plugin. Builds never inject it.
  Object.assign(process.env, mergeSdkworkImBootstrapAccessTokenEnv(process.env));
}

process.argv = [
  process.argv[0],
  viteCliPath,
  ...process.argv.slice(2),
];

await import(pathToFileURL(viteCliPath).href);
