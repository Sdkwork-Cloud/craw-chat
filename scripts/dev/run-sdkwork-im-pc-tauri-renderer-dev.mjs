#!/usr/bin/env node

import path from 'node:path';
import process from 'node:process';
import { fileURLToPath, pathToFileURL } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const imPcAppRoot = path.resolve(scriptDir, '../../apps/sdkwork-im-pc');
const viteDevScript = path.join(scriptDir, 'run-sdkwork-im-pc-vite-dev.mjs');

process.chdir(imPcAppRoot);

await import(pathToFileURL(viteDevScript).href);
