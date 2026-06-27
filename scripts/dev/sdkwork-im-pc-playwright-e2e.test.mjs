#!/usr/bin/env node
// Commercial gate wrapper: builds on apps/sdkwork-im-pc/dist, serves production shell on
// PLAYWRIGHT_PC_PORT (default 4173), then runs Playwright specs under apps/sdkwork-im-pc/e2e/.

import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import fs from 'node:fs';
import http from 'node:http';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const pcRoot = path.join(repoRoot, 'apps', 'sdkwork-im-pc');
const distIndex = path.join(pcRoot, 'dist', 'index.html');
const serverEntry = path.join(pcRoot, 'dist', 'server.cjs');
const pnpmExecutable = process.platform === 'win32' ? 'pnpm.cmd' : 'pnpm';
const shell = process.platform === 'win32';
const e2ePort = process.env.PLAYWRIGHT_PC_PORT ?? '4173';
const e2eBaseUrl = `http://127.0.0.1:${e2ePort}`;

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
    const child = spawn(command, args, {
      stdio: 'inherit',
      shell,
      ...options,
    });
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

function waitForHttpOk(url, timeoutMs = 30_000) {
  const started = Date.now();
  return new Promise((resolve, reject) => {
    const attempt = () => {
      const request = http.get(url, (response) => {
        response.resume();
        if (response.statusCode === 200) {
          resolve();
          return;
        }
        if (Date.now() - started > timeoutMs) {
          reject(new Error(`expected HTTP 200 from ${url}, received ${response.statusCode}`));
          return;
        }
        setTimeout(attempt, 500);
      });
      request.on('error', () => {
        if (Date.now() - started > timeoutMs) {
          reject(new Error(`timed out waiting for ${url}`));
          return;
        }
        setTimeout(attempt, 500);
      });
    };
    attempt();
  });
}

function stopServer(child) {
  if (!child || child.killed || child.exitCode !== null) {
    return Promise.resolve();
  }
  return new Promise((resolve) => {
    const finish = () => resolve();
    child.once('exit', finish);
    if (process.platform === 'win32') {
      child.kill();
    } else {
      child.kill('SIGTERM');
    }
    setTimeout(() => {
      if (child.exitCode === null && !child.killed) {
        child.kill('SIGKILL');
      }
      finish();
    }, 5_000);
  });
}

const server = spawn(process.execPath, [serverEntry], {
  cwd: pcRoot,
  env: {
    ...process.env,
    NODE_ENV: 'production',
    PORT: e2ePort,
  },
  stdio: ['ignore', 'pipe', 'pipe'],
  shell,
});

try {
  await waitForHttpOk(`${e2eBaseUrl}/`);
  await runCommand(pnpmExecutable, ['exec', 'playwright', 'test'], {
    cwd: pcRoot,
    env: {
      ...process.env,
      PLAYWRIGHT_BASE_URL: e2eBaseUrl,
    },
  });
  console.log('sdkwork-im PC Playwright e2e passed');
} finally {
  await stopServer(server);
}

process.exit(0);