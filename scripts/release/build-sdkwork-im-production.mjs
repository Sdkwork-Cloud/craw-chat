#!/usr/bin/env node

import { spawn } from 'node:child_process';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { resolveDesktopReleaseTarget } from './desktop-targets.mjs';

const __filename = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(__filename), '..', '..');

function pnpmCommand(platform = process.platform) {
  return platform === 'win32' ? 'pnpm.cmd' : 'pnpm';
}

function cargoCommand(platform = process.platform) {
  return platform === 'win32' ? 'cargo.exe' : 'cargo';
}

function printHelp() {
  console.log(`Usage: node scripts/release/build-sdkwork-im-production.mjs [options]

Build Sdkwork IM production artifacts for server archives and desktop installers.

Options:
  --target <value>          Build target: server, desktop, all (default all).
  --target-triple <value>   Tauri/Rust target triple for desktop packaging.
  --platform <value>        Desktop target platform override: windows, linux, macos.
  --arch <value>            Desktop target architecture override: x64, arm64.
  --dry-run                 Print the build plan without executing commands.
  --json                    Print machine-readable JSON.
  -h, --help                Show this help.
`);
}

function requireValue(argv, index, flag) {
  const value = argv[index + 1];
  if (!value || value.startsWith('--')) {
    throw new Error(`${flag} requires a value`);
  }
  return value;
}

function parseProductionBuildArgs(argv = process.argv.slice(2)) {
  const settings = {
    arch: null,
    dryRun: false,
    help: false,
    json: false,
    platform: null,
    target: 'all',
    targetTriple: null,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === '--') {
      continue;
    }
    switch (arg) {
      case '--target':
        settings.target = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--target-triple':
        settings.targetTriple = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--platform':
        settings.platform = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--arch':
        settings.arch = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--dry-run':
        settings.dryRun = true;
        break;
      case '--json':
        settings.json = true;
        break;
      case '--help':
      case '-h':
        settings.help = true;
        break;
      default:
        throw new Error(`Unsupported production build option: ${arg}`);
    }
  }

  if (!['server', 'desktop', 'all'].includes(settings.target)) {
    throw new Error('--target must be server, desktop, or all');
  }

  return settings;
}

function createSdkworkImProductionBuildPlan({
  arch = null,
  env = process.env,
  platform = null,
  root = repoRoot,
  target = 'all',
  targetTriple = null,
} = {}) {
  const steps = [];
  const includeServer = target === 'server' || target === 'all';
  const includeDesktop = target === 'desktop' || target === 'all';
  const includeWebAssets = includeServer || includeDesktop;

  if (includeWebAssets) {
    steps.push({
      label: 'build sdkwork-im-pc web assets',
      command: pnpmCommand(),
      args: ['release:build'],
      cwd: root,
      env,
    });
  }

  if (includeServer) {
    steps.push({
      label: 'build sdkwork-im-server release binary',
      command: cargoCommand(),
      args: ['build', '-p', 'sdkwork-im-cloud-gateway', '--bin', 'sdkwork-im-server', '--release'],
      cwd: root,
      env,
    });
  }

  if (includeDesktop) {
    const desktopTarget = resolveDesktopReleaseTarget({
      arch,
      env,
      platform,
      targetTriple,
    });
    steps.push({
      label: `build desktop installer ${desktopTarget.targetTriple}`,
      command: pnpmCommand(),
      args: [
        '--dir',
        'apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop',
        'build:desktop:local',
        '--',
        '--target',
        desktopTarget.targetTriple,
      ],
      cwd: root,
      env: {
        ...env,
        SDKWORK_DESKTOP_TARGET: desktopTarget.targetTriple,
        SDKWORK_DESKTOP_TARGET_PLATFORM: desktopTarget.platform,
        SDKWORK_DESKTOP_TARGET_ARCH: desktopTarget.arch,
      },
      target: desktopTarget,
    });
  }

  return {
    schemaVersion: '2026-06-04.sdkwork-im.production-build.v1',
    target,
    root,
    steps,
    expectedArtifacts: {
      serverBinary: path.join(root, 'target', 'release', process.platform === 'win32' ? 'sdkwork-im-server.exe' : 'sdkwork-im-server'),
      pcWebDist: path.join(root, 'apps', 'sdkwork-im-pc', 'dist'),
      desktopBundleRoot: path.join(
        root,
        'apps',
        'sdkwork-im-pc',
        'packages',
        'sdkwork-im-pc-desktop',
        'src-tauri',
        'target',
        'release',
        'bundle',
      ),
    },
  };
}

function renderSdkworkImProductionBuildPlan(plan) {
  return [
    `[sdkwork-im-production-build] target: ${plan.target}`,
    `[sdkwork-im-production-build] root: ${plan.root}`,
    ...plan.steps.map((step) => `[sdkwork-im-production-build]   ${step.label}: ${step.command} ${step.args.join(' ')}`),
    `[sdkwork-im-production-build] expected server binary: ${plan.expectedArtifacts.serverBinary}`,
    `[sdkwork-im-production-build] expected web dist: ${plan.expectedArtifacts.pcWebDist}`,
    `[sdkwork-im-production-build] expected desktop bundle root: ${plan.expectedArtifacts.desktopBundleRoot}`,
  ];
}

function runStep(step) {
  return new Promise((resolve, reject) => {
    console.error(`[sdkwork-im-production-build] ${step.label}: ${step.command} ${step.args.join(' ')}`);
    const child = spawn(step.command, step.args, {
      cwd: step.cwd,
      env: step.env,
      shell: false,
      stdio: 'inherit',
      windowsHide: process.platform === 'win32',
    });
    child.on('error', reject);
    child.on('exit', (code, signal) => {
      if (signal) {
        reject(new Error(`${step.label} exited with signal ${signal}`));
        return;
      }
      if ((code ?? 1) !== 0) {
        reject(new Error(`${step.label} exited with code ${code}`));
        return;
      }
      resolve();
    });
  });
}

async function main(argv = process.argv.slice(2)) {
  const settings = parseProductionBuildArgs(argv);
  if (settings.help) {
    printHelp();
    return 0;
  }

  const plan = createSdkworkImProductionBuildPlan(settings);
  if (settings.json) {
    console.log(JSON.stringify({
      ok: true,
      dryRun: settings.dryRun,
      plan: serializableProductionBuildPlan(plan),
    }, null, 2));
  } else {
    for (const line of renderSdkworkImProductionBuildPlan(plan)) {
      console.log(line);
    }
  }

  if (settings.dryRun) {
    return 0;
  }

  for (const step of plan.steps) {
    await runStep(step);
  }
  return 0;
}

function serializableProductionBuildPlan(plan) {
  return {
    ...plan,
    steps: plan.steps.map((step) => {
      const { env, ...publicStep } = step;
      return {
        ...publicStep,
        envKeys: selectedReleaseEnvKeys(env),
      };
    }),
  };
}

function selectedReleaseEnvKeys(env = {}) {
  return [
    'SDKWORK_DESKTOP_TARGET',
    'SDKWORK_DESKTOP_TARGET_PLATFORM',
    'SDKWORK_DESKTOP_TARGET_ARCH',
  ].filter((key) => Object.hasOwn(env, key));
}

if (process.argv[1] && import.meta.url.endsWith(process.argv[1].replaceAll('\\', '/'))) {
  main().then((code) => {
    process.exitCode = code;
  }).catch((error) => {
    console.error(`[sdkwork-im-production-build] ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  });
}

export {
  cargoCommand,
  createSdkworkImProductionBuildPlan,
  main,
  parseProductionBuildArgs,
  pnpmCommand,
  renderSdkworkImProductionBuildPlan,
  serializableProductionBuildPlan,
};
