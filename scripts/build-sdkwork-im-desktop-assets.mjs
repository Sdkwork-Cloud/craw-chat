#!/usr/bin/env node

import { spawn } from 'node:child_process';
import { access } from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { pnpmProcessSpec } from './dev/pnpm-launch-lib.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');

export function createDesktopAssetBuildPlan({
  workspaceRoot: buildWorkspaceRoot = workspaceRoot,
  platform = process.platform,
} = {}) {
  return [
    {
      cwd: path.join(buildWorkspaceRoot, 'apps', 'sdkwork-im-pc'),
      ...pnpmProcessSpec(['build'], { platform }),
    },
  ];
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

export async function assertDesktopPcRendererReady({
  workspaceRoot: buildWorkspaceRoot = workspaceRoot,
  pcDistRoot = path.join(buildWorkspaceRoot, 'apps', 'sdkwork-im-pc', 'dist'),
} = {}) {
  await assertDesktopSiteBuildReady({
    siteLabel: 'shared PC renderer build',
    siteRoot: pcDistRoot,
    requiredFiles: ['index.html'],
  });
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

  await assertDesktopPcRendererReady();
}

if (fileURLToPath(import.meta.url) === process.argv[1]) {
  main().catch((error) => {
    console.error(`[build-sdkwork-im-desktop-assets] ${error.message}`);
    process.exit(1);
  });
}
