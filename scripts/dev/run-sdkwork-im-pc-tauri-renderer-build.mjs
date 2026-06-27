#!/usr/bin/env node

import path from 'node:path';
import process from 'node:process';
import { fileURLToPath, pathToFileURL } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const imPcAppRoot = path.resolve(scriptDir, '../../apps/sdkwork-im-pc');
const viteBuildScript = path.join(scriptDir, 'run-vite-cli.mjs');

process.chdir(imPcAppRoot);
process.argv = [
  process.argv[0],
  viteBuildScript,
  'build',
  ...process.argv.slice(2),
];

await import(pathToFileURL(viteBuildScript).href);
