import assert from 'node:assert/strict';
import { mkdtemp, mkdir, rm, writeFile } from 'node:fs/promises';
import { readFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { pathToFileURL } from 'node:url';

const repoRoot = path.resolve(import.meta.dirname, '..');
const scriptPath = path.join(repoRoot, 'scripts', 'build-sdkwork-im-desktop-assets.mjs');

async function loadModule() {
  return import(pathToFileURL(scriptPath).href);
}

test('desktop asset build script is aligned with the sdkwork-im desktop app roots', async () => {
  const source = readFileSync(scriptPath, 'utf8');
  assert.doesNotMatch(
    source,
    /sdkwork-im-(?:admin|portal)|control-plane|sdkwork-chat-pc/u,
    'desktop asset build script must not reference retired app roots',
  );

  const module = await loadModule();
  assert.equal(typeof module.createDesktopAssetBuildPlan, 'function');
  assert.equal(typeof module.assertDesktopSiteBuildReady, 'function');
  assert.equal(typeof module.assertDesktopPcRendererReady, 'function');

  const plan = module.createDesktopAssetBuildPlan({
    platform: 'linux',
    workspaceRoot: repoRoot,
  });
  assert.deepEqual(
    plan.map((step) => ({
      args: step.args,
      command: step.command,
      cwd: path.relative(repoRoot, step.cwd).replaceAll('\\', '/'),
    })),
    [
      {
        args: ['build'],
        command: 'pnpm',
        cwd: 'apps/sdkwork-im-pc',
      },
    ],
  );
});

test('desktop asset readiness checks the shared sdkwork-im-pc renderer output', async () => {
  const module = await loadModule();
  const tempRoot = await mkdtemp(path.join(os.tmpdir(), 'sdkwork-im-desktop-assets-'));
  try {
    const pcDistDir = path.join(tempRoot, 'apps', 'sdkwork-im-pc', 'dist');

    await mkdir(pcDistDir, { recursive: true });
    await writeFile(
      path.join(pcDistDir, 'index.html'),
      '<!doctype html><html><body>SDKWork IM PC</body></html>',
    );

    await module.assertDesktopPcRendererReady({
      workspaceRoot: tempRoot,
      pcDistRoot: pcDistDir,
    });
  } finally {
    await rm(tempRoot, { force: true, recursive: true });
  }
});
