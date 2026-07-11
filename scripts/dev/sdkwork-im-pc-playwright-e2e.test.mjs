#!/usr/bin/env node
// Commercial gate wrapper: builds on apps/sdkwork-im-pc/dist, serves production shell on
// PLAYWRIGHT_PC_PORT (default 4173), then runs Playwright specs under apps/sdkwork-im-pc/e2e/.

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
const pnpmExecutable = process.platform === 'win32' ? 'pnpm.cmd' : 'pnpm';
const commandShell = process.platform === 'win32';
const e2ePort = parseTcpPort(process.env.PLAYWRIGHT_PC_PORT ?? '4173', 'PLAYWRIGHT_PC_PORT');
const e2eBaseUrl = `http://127.0.0.1:${e2ePort}`;
const componentPort = parseTcpPort(
  process.env.PLAYWRIGHT_PC_COMPONENT_PORT ?? '4174',
  'PLAYWRIGHT_PC_COMPONENT_PORT',
);
const componentBaseUrl = `http://127.0.0.1:${componentPort}`;
const componentServerEntry = path.join(repoRoot, 'scripts', 'dev', 'run-sdkwork-im-pc-vite-dev.mjs');
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

function runCommand(command, args, options = {}) {
  return new Promise((resolve, reject) => {
    const child = lifecycle.track(spawn(command, args, {
      detached: useProcessGroup,
      stdio: 'inherit',
      shell: commandShell,
      ...options,
    }), { processGroup: useProcessGroup });
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`${command} ${args.join(' ')} exited with code ${code ?? 'unknown'}`));
    });
  });
}

async function main() {
  await lifecycle.run(async ({ signal }) => {
    assert.notEqual(
      e2ePort,
      componentPort,
      'PLAYWRIGHT_PC_PORT and PLAYWRIGHT_PC_COMPONENT_PORT must be different',
    );
    await Promise.all([
      assertPortAvailable({
        host: '0.0.0.0',
        port: e2ePort,
        readinessHosts: ['127.0.0.1'],
      }),
      assertPortAvailable({ host: '127.0.0.1', port: componentPort }),
    ]);
    if (signal.aborted) {
      return;
    }

    const server = lifecycle.track(spawn(process.execPath, [serverEntry], {
      cwd: pcRoot,
      detached: useProcessGroup,
      env: {
        ...process.env,
        NODE_ENV: 'production',
        PORT: String(e2ePort),
      },
      shell: false,
      stdio: 'inherit',
      windowsHide: true,
    }), { processGroup: useProcessGroup });
    const componentServer = lifecycle.track(spawn(
      process.execPath,
      [componentServerEntry, '--strictPort'],
      {
        cwd: pcRoot,
        detached: useProcessGroup,
        env: {
          ...process.env,
          SDKWORK_IM_PC_DEV_HOST: '127.0.0.1',
          SDKWORK_IM_PC_DEV_PORT: String(componentPort),
        },
        shell: false,
        stdio: 'ignore',
        windowsHide: true,
      },
    ), { processGroup: useProcessGroup });
    await Promise.all([
      waitForOwnedHttpOk({
        child: server,
        url: `${e2eBaseUrl}/`,
        verifyResponse: ({ body, headers }) => (
          headers['x-content-type-options'] === 'nosniff'
          && /<div\s+id=["']root["']/u.test(body)
        ),
      }),
      waitForOwnedHttpOk({
        child: componentServer,
        url: `${componentBaseUrl}/e2e/fixtures/conversation-list-harness.html?count=1`,
        verifyResponse: ({ body }) => body.includes('Conversation list virtualization harness'),
      }),
    ]);
    await runCommand(pnpmExecutable, ['exec', 'playwright', 'test'], {
      cwd: pcRoot,
      env: {
        ...process.env,
        PLAYWRIGHT_BASE_URL: e2eBaseUrl,
        PLAYWRIGHT_COMPONENT_BASE_URL: componentBaseUrl,
      },
    });
    if (!signal.aborted) {
      console.log('sdkwork-im PC Playwright e2e passed');
    }
  });
}

const isMain = process.argv[1] && pathToFileURL(process.argv[1]).href === import.meta.url;
if (isMain) {
  await main();
}
