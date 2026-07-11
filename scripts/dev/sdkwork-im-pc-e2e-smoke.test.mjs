#!/usr/bin/env node

import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

import {
  assertPortAvailable,
  createOwnedProcessLifecycle,
  parseTcpPort,
  waitForOwnedHttpOk,
} from './sdkwork-im-pc-playwright-runner.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const pcRoot = path.join(repoRoot, 'apps', 'sdkwork-im-pc');
const distIndex = path.join(pcRoot, 'dist', 'index.html');
const serverEntry = path.join(pcRoot, 'dist', 'server.cjs');
const serverPort = parseTcpPort(
  process.env.PLAYWRIGHT_PC_SMOKE_PORT ?? '3000',
  'PLAYWRIGHT_PC_SMOKE_PORT',
);
const serverBaseUrl = `http://127.0.0.1:${serverPort}`;
const lifecycle = createOwnedProcessLifecycle();
const useProcessGroup = process.platform !== 'win32';

assert.equal(
  fs.existsSync(distIndex),
  true,
  'apps/sdkwork-im-pc/dist/index.html must exist; run pnpm build in apps/sdkwork-im-pc first',
);
assert.equal(
  fs.existsSync(serverEntry),
  true,
  'apps/sdkwork-im-pc/dist/server.cjs must exist; run pnpm build in apps/sdkwork-im-pc first',
);

async function main() {
  await lifecycle.run(async ({ signal }) => {
    await assertPortAvailable({ host: '0.0.0.0', port: serverPort });
    if (signal.aborted) {
      return;
    }
    const server = lifecycle.track(spawn(process.execPath, [serverEntry], {
      cwd: pcRoot,
      detached: useProcessGroup,
      env: {
        ...process.env,
        NODE_ENV: 'production',
        PORT: String(serverPort),
      },
      shell: false,
      stdio: 'inherit',
      windowsHide: true,
    }), { processGroup: useProcessGroup });
    const { body } = await waitForOwnedHttpOk({
      child: server,
      url: `${serverBaseUrl}/`,
      verifyResponse: ({ body: html, headers }) => (
        headers['x-content-type-options'] === 'nosniff'
        && /<div\s+id=["']root["']/u.test(html)
        && html.includes('<title>')
      ),
    });
    assert.match(body, /<div\s+id=["']root["']/u, 'PC production shell must expose #root mount point');
    if (!signal.aborted) {
      console.log('sdkwork-im PC e2e smoke passed');
    }
  });
}

const isMain = process.argv[1] && pathToFileURL(process.argv[1]).href === import.meta.url;
if (isMain) {
  await main();
}
