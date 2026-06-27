#!/usr/bin/env node

import { spawn } from 'node:child_process';
import { access, copyFile, mkdir, readdir, rm } from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { pnpmProcessSpec } from './dev/pnpm-launch-lib.mjs';
import {
  assertPortalDistReleaseSafe,
  rebuildDist,
} from '../apps/sdkwork-im-portal/scripts/lib/build-dist.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const REQUIRED_PORTAL_VENDOR_FILES = Object.freeze([
  '__vendor__/sdkwork-im-sdk/index.js',
  '__vendor__/sdkwork-sdk-common/index.js',
]);

export function createDesktopAssetBuildPlan({
  workspaceRoot: buildWorkspaceRoot = workspaceRoot,
  platform = process.platform,
} = {}) {
  return [
    {
      cwd: path.join(buildWorkspaceRoot, 'apps', 'control-plane'),
      ...pnpmProcessSpec(['build'], { platform }),
    },
  ];
}

async function copyDirectory(sourceRoot, targetRoot) {
  await mkdir(targetRoot, { recursive: true });

  const entries = await readdir(sourceRoot, { withFileTypes: true });
  for (const entry of entries) {
    const sourcePath = path.join(sourceRoot, entry.name);
    const targetPath = path.join(targetRoot, entry.name);

    if (entry.isDirectory()) {
      // Recursively mirror the portal static source tree into dist.
      // The portal build is a deterministic file copy, not a Vite pipeline.
      // This keeps desktop bundling independent from portal node_modules state.
      // It intentionally mirrors the existing portal build contract.
      // The admin app remains on the router-style pnpm build path above.
      // The portal stays lightweight and static.
      // This also avoids Windows child-process lock issues observed with portal build.mjs.
      await copyDirectory(sourcePath, targetPath);
      continue;
    }

    await copyFile(sourcePath, targetPath);
  }
}

export async function assertDesktopSiteBuildReady({
  siteLabel,
  siteRoot,
  requiredFiles = ['index.html'],
} = {}) {
  if (typeof siteLabel !== 'string' || siteLabel.trim() === '') {
    throw new Error('siteLabel is required when validating desktop site assets.');
  }
  if (typeof siteRoot !== 'string' || siteRoot.trim() === '') {
    throw new Error(`${siteLabel} root path is required.`);
  }

  await access(siteRoot).catch(() => {
    throw new Error(`${siteLabel} directory is missing: ${siteRoot}`);
  });

  for (const relativePath of requiredFiles) {
    const requiredPath = path.join(siteRoot, relativePath);
    await access(requiredPath).catch(() => {
      throw new Error(`${siteLabel} required asset is missing: ${requiredPath}`);
    });
  }
}

export async function assertDesktopEmbeddedSitesReady({
  workspaceRoot: buildWorkspaceRoot = workspaceRoot,
  adminDistRoot = path.join(buildWorkspaceRoot, 'apps', 'control-plane', 'dist'),
  portalDistRoot = path.join(buildWorkspaceRoot, 'apps', 'sdkwork-im-portal', 'dist'),
  portalDesktopDistRoot = path.join(buildWorkspaceRoot, 'apps', 'control-plane', 'dist-portal'),
} = {}) {
  await assertDesktopSiteBuildReady({
    siteLabel: 'admin desktop site build',
    siteRoot: adminDistRoot,
    requiredFiles: ['index.html'],
  });
  await assertDesktopSiteBuildReady({
    siteLabel: 'portal site build',
    siteRoot: portalDistRoot,
    requiredFiles: ['index.html', ...REQUIRED_PORTAL_VENDOR_FILES],
  });
  await assertPortalDistReleaseSafe(portalDistRoot);
  await assertDesktopSiteBuildReady({
    siteLabel: 'embedded portal desktop site build',
    siteRoot: portalDesktopDistRoot,
    requiredFiles: ['index.html', ...REQUIRED_PORTAL_VENDOR_FILES],
  });
  await assertPortalDistReleaseSafe(portalDesktopDistRoot);
}

export async function syncPortalDesktopAssets({
  workspaceRoot: buildWorkspaceRoot = workspaceRoot,
  portalDistRoot = path.join(buildWorkspaceRoot, 'apps', 'sdkwork-im-portal', 'dist'),
  portalDesktopDistRoot = path.join(buildWorkspaceRoot, 'apps', 'control-plane', 'dist-portal'),
} = {}) {
  await access(portalDistRoot);
  await rm(portalDesktopDistRoot, { force: true, recursive: true });
  await copyDirectory(portalDistRoot, portalDesktopDistRoot);
}

async function buildPortalAssets({
  workspaceRoot: buildWorkspaceRoot = workspaceRoot,
} = {}) {
  const portalDistRoot = path.join(buildWorkspaceRoot, 'apps', 'sdkwork-im-portal', 'dist');

  if (path.resolve(buildWorkspaceRoot) === workspaceRoot) {
    await rebuildDist();
  } else {
    await access(portalDistRoot);
  }

  await assertDesktopSiteBuildReady({
    siteLabel: 'portal site build',
    siteRoot: portalDistRoot,
    requiredFiles: ['index.html', ...REQUIRED_PORTAL_VENDOR_FILES],
  });
  await syncPortalDesktopAssets({ workspaceRoot: buildWorkspaceRoot });
}

async function runBuildStep(step) {
  await new Promise((resolve, reject) => {
    const child = spawn(step.command, step.args, {
      cwd: step.cwd,
      stdio: 'inherit',
      windowsHide: process.platform === 'win32',
    });

    child.on('error', reject);
    child.on('exit', (code, signal) => {
      if (signal) {
        reject(new Error(`build in ${step.cwd} exited with signal ${signal}`));
        return;
      }

      if ((code ?? 1) !== 0) {
        reject(new Error(`build in ${step.cwd} exited with code ${code ?? 1}`));
        return;
      }

      resolve();
    });
  });
}

async function main() {
  const plan = createDesktopAssetBuildPlan();
  for (const step of plan) {
    // eslint-disable-next-line no-await-in-loop
    await runBuildStep(step);
  }

  await buildPortalAssets();
  await assertDesktopEmbeddedSitesReady();
}

if (fileURLToPath(import.meta.url) === process.argv[1]) {
  main().catch((error) => {
    console.error(`[build-sdkwork-im-desktop-assets] ${error.message}`);
    process.exit(1);
  });
}
