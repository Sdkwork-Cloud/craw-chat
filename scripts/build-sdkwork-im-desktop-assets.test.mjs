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
    /apps[\\/](?:sdkwork-im-admin|sdkwork-im-portal|sdkwork-chat-pc)/u,
    'desktop asset build script must not reference retired app roots',
  );

  const module = await loadModule();
  assert.equal(typeof module.createDesktopAssetBuildPlan, 'function');
  assert.equal(typeof module.assertDesktopSiteBuildReady, 'function');
  assert.equal(typeof module.assertDesktopEmbeddedSitesReady, 'function');

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
        cwd: 'apps/control-plane',
      },
    ],
  );
});

test('desktop asset readiness checks the control-plane and sdkwork-im-portal dist output', async () => {
  const module = await loadModule();
  const tempRoot = await mkdtemp(path.join(os.tmpdir(), 'sdkwork-im-desktop-assets-'));
  try {
    const adminDistDir = path.join(tempRoot, 'apps', 'control-plane', 'dist');
    const portalDistDir = path.join(tempRoot, 'apps', 'sdkwork-im-portal', 'dist');
    const portalDesktopDistDir = path.join(tempRoot, 'apps', 'control-plane', 'dist-portal');

    await mkdir(path.join(adminDistDir), { recursive: true });
    await writeFile(
      path.join(adminDistDir, 'index.html'),
      '<!doctype html><html><body>SDKWork IM admin</body></html>',
    );

    await mkdir(path.join(portalDistDir, '__vendor__', 'sdkwork-im-sdk'), { recursive: true });
    await mkdir(path.join(portalDistDir, '__vendor__', 'sdkwork-sdk-common'), { recursive: true });
    await writeFile(
      path.join(portalDistDir, 'index.html'),
      '<!doctype html><html><body>SDKWork IM portal</body></html>',
    );
    await writeFile(
      path.join(portalDistDir, '__vendor__', 'sdkwork-im-sdk', 'index.js'),
      'export {};',
    );
    await writeFile(
      path.join(portalDistDir, '__vendor__', 'sdkwork-sdk-common', 'index.js'),
      'export {};',
    );

    await module.syncPortalDesktopAssets({
      workspaceRoot: tempRoot,
      portalDistRoot: portalDistDir,
      portalDesktopDistRoot: portalDesktopDistDir,
    });
    await module.assertDesktopEmbeddedSitesReady({
      workspaceRoot: tempRoot,
      adminDistRoot: adminDistDir,
      portalDistRoot: portalDistDir,
      portalDesktopDistRoot: portalDesktopDistDir,
    });
  } finally {
    await rm(tempRoot, { force: true, recursive: true });
  }
});
