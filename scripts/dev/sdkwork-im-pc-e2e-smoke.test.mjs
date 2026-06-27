#!/usr/bin/env node

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

const shell = process.platform === 'win32';
const server = spawn(process.execPath, [serverEntry], {
  cwd: pcRoot,
  env: {
    ...process.env,
    NODE_ENV: 'production',
    PORT: '3000',
  },
  stdio: ['ignore', 'pipe', 'pipe'],
  shell,
});

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
  if (!child || child.killed) {
    return Promise.resolve();
  }
  return new Promise((resolve) => {
    child.once('exit', () => resolve());
    child.kill('SIGTERM');
    setTimeout(() => {
      if (!child.killed) {
        child.kill('SIGKILL');
      }
    }, 5_000);
  });
}

try {
  await waitForHttpOk('http://127.0.0.1:3000/');
  const html = await fetch('http://127.0.0.1:3000/').then((response) => response.text());
  assert.match(html, /<div id="root"/u, 'PC production shell must expose #root mount point');
  assert.match(html, /<title>/u, 'PC production shell must include document title');
  console.log('sdkwork-im PC e2e smoke passed');
} finally {
  await stopServer(server);
}
