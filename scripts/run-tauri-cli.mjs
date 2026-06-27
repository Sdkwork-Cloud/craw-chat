#!/usr/bin/env node

import fs from 'node:fs';
import { spawn } from 'node:child_process';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import {
  ensureLocalNodeModules,
  resolveReadablePackageEntry,
  resolveWorkspaceDonorRoots,
} from './dev/vite-runtime-lib.mjs';
import {
  buildDesktopReleaseEnv,
  DESKTOP_TARGET_ENV_VAR,
} from './release/desktop-targets.mjs';

const __filename = fileURLToPath(import.meta.url);
const BACKGROUND_LAUNCH_ENV = 'SDKWORK_ROUTER_BACKGROUND';
const SDKWORK_IM_PC_DEV_HOST_ENV = 'SDKWORK_IM_PC_DEV_HOST';
const SDKWORK_IM_PC_DEV_PORT_ENV = 'SDKWORK_IM_PC_DEV_PORT';
const WIX_UI_EXTENSION = 'WixUIExtension';
const WIX_PDB_NAME = 'output.wixpdb';
const WINDOWS_WIX_ARTIFACT_CLOCK_SKEW_MS = 10_000;
const REQUIRED_TAURI_CLI_PACKAGES = [
  '@tauri-apps/cli',
];

function normalizeCliArgs(args = []) {
  return args.filter((arg) => arg !== '--');
}

function normalizeTcpPort(value) {
  const normalized = String(value ?? '').trim();
  if (!/^\d+$/u.test(normalized)) {
    return null;
  }
  const port = Number.parseInt(normalized, 10);
  return Number.isInteger(port) && port >= 1 && port <= 65535 ? port : null;
}

function createSdkworkChatPcDevConfigArgs({
  args = [],
  commandName,
  env = process.env,
} = {}) {
  if (commandName !== 'dev') {
    return [];
  }
  const normalizedArgs = normalizeCliArgs(args);
  if (
    normalizedArgs.includes('--config')
    || normalizedArgs.some((arg) => String(arg).startsWith('--config='))
  ) {
    return [];
  }
  const port = normalizeTcpPort(env[SDKWORK_IM_PC_DEV_PORT_ENV]);
  if (!port) {
    return [];
  }
  const host = String(env[SDKWORK_IM_PC_DEV_HOST_ENV] ?? '').trim() || '127.0.0.1';
  return [
    '--config',
    JSON.stringify({
      build: {
        devUrl: `http://${host}:${port}`,
      },
    }),
  ];
}

function shouldLaunchInBackground(commandName, args = [], env = process.env) {
  if (commandName !== 'dev') {
    return false;
  }

  if (String(env[BACKGROUND_LAUNCH_ENV] ?? '').trim() === '1') {
    return true;
  }

  return normalizeCliArgs(args).some((arg) => arg === '--service' || arg === '--start-hidden');
}

function stripAnsi(value) {
  return String(value).replaceAll(/\u001b\[[0-9;]*m/g, '');
}

export function withSupportedWindowsCmakeGenerator(
  baseEnv = process.env,
  platform = process.platform,
) {
  const env = { ...baseEnv };
  if (platform !== 'win32') {
    return env;
  }

  const requestedGenerator = String(env.CMAKE_GENERATOR ?? '').trim();
  if (requestedGenerator.length > 0 && !requestedGenerator.includes('2026')) {
    return env;
  }

  env.CMAKE_GENERATOR = 'Visual Studio 17 2022';
  env.HOST_CMAKE_GENERATOR = 'Visual Studio 17 2022';
  return env;
}

function resolveWindowsNativeBuildJobs(
  baseEnv = process.env,
  platform = process.platform,
) {
  const requestedJobs = String(baseEnv.CARGO_BUILD_JOBS ?? '').trim();
  if (requestedJobs.length > 0) {
    return requestedJobs;
  }

  return platform === 'win32' ? '1' : '';
}

export function withStableWindowsNativeBuildConcurrency(
  baseEnv = process.env,
  platform = process.platform,
) {
  const env = { ...baseEnv };
  if (platform !== 'win32') {
    return env;
  }

  const resolvedJobs = resolveWindowsNativeBuildJobs(baseEnv, platform);
  if (resolvedJobs.length === 0) {
    return env;
  }

  env.CARGO_BUILD_JOBS = resolvedJobs;

  if (String(env.NUM_JOBS ?? '').trim().length === 0) {
    env.NUM_JOBS = resolvedJobs;
  }

  if (String(env.CMAKE_BUILD_PARALLEL_LEVEL ?? '').trim().length === 0) {
    env.CMAKE_BUILD_PARALLEL_LEVEL = resolvedJobs;
  }

  return env;
}

export function extractWindowsWixBundleArtifacts(output = '') {
  const normalizedOutput = stripAnsi(output);
  const candleMatch = normalizedOutput.match(/Running candle for "([^"\r\n]+main\.wxs)"/);
  const lightMatch = normalizedOutput.match(/Running light to produce ([^\r\n]+\.msi)/);
  if (!candleMatch || !lightMatch) {
    return null;
  }

  const wixDir = path.dirname(candleMatch[1]);
  return {
    wixDir,
    wixSourcePath: candleMatch[1],
    wixObjPath: path.join(wixDir, 'main.wixobj'),
    wixPdbPath: path.join(wixDir, WIX_PDB_NAME),
    localePath: path.join(wixDir, 'locale.wxl'),
    msiPath: lightMatch[1].trim(),
  };
}

export function shouldRetryWindowsMsiBundleWithSval({
  commandName,
  output = '',
  platform = process.platform,
} = {}) {
  if (platform !== 'win32' || commandName !== 'build') {
    return false;
  }

  const normalizedOutput = stripAnsi(output);
  return /failed to run .*light\.exe/i.test(normalizedOutput)
    && extractWindowsWixBundleArtifacts(normalizedOutput) !== null;
}

function normalizeWindowsMsiCultureSegment(culture = 'en-US') {
  const normalized = String(culture).trim();
  if (normalized.length === 0) {
    return 'en-US';
  }

  const parts = normalized.split('-').filter(Boolean);
  if (parts.length === 0) {
    return 'en-US';
  }

  return parts.map((part, index) => {
    if (index === 0) {
      return part.toLowerCase();
    }

    if (part.length <= 3 || /^\d+$/.test(part)) {
      return part.toUpperCase();
    }

    return `${part.slice(0, 1).toUpperCase()}${part.slice(1).toLowerCase()}`;
  }).join('-');
}

function resolveWindowsWixArtifactFreshnessCutoff(buildStartedAt = 0) {
  if (!Number.isFinite(buildStartedAt) || buildStartedAt <= 0) {
    return Number.NEGATIVE_INFINITY;
  }

  return buildStartedAt - WINDOWS_WIX_ARTIFACT_CLOCK_SKEW_MS;
}

function readWindowsWixProductMetadata(wixSourcePath) {
  if (!wixSourcePath || !fs.existsSync(wixSourcePath)) {
    return null;
  }

  const wixSource = fs.readFileSync(wixSourcePath, 'utf8');
  const productTagMatch = wixSource.match(/<Product\b([\s\S]*?)>/i);
  if (!productTagMatch) {
    return null;
  }

  const productTag = productTagMatch[1];
  const productNameMatch = productTag.match(/\bName="([^"]+)"/);
  const productVersionMatch = productTag.match(/\bVersion="([^"]+)"/);
  if (!productNameMatch || !productVersionMatch) {
    return null;
  }

  return {
    productName: productNameMatch[1],
    productVersion: productVersionMatch[1],
  };
}

function readWindowsWixCulture(localePath) {
  if (!localePath || !fs.existsSync(localePath)) {
    return 'en-US';
  }

  const localeSource = fs.readFileSync(localePath, 'utf8');
  const cultureMatch = localeSource.match(/\bCulture="([^"]+)"/i);
  return normalizeWindowsMsiCultureSegment(cultureMatch?.[1] ?? 'en-US');
}

function resolveWindowsWixArtifactMtimeMs(paths = []) {
  const mtimes = paths
    .filter(Boolean)
    .filter((candidatePath) => fs.existsSync(candidatePath))
    .map((candidatePath) => fs.statSync(candidatePath).mtimeMs);
  return mtimes.length > 0 ? Math.max(...mtimes) : 0;
}

function resolveWindowsWixRetryArtifactMtimeMs(artifacts) {
  if (!artifacts) {
    return 0;
  }

  return resolveWindowsWixArtifactMtimeMs([
    artifacts.wixSourcePath,
    artifacts.wixObjPath,
  ]);
}

export function resolveWindowsWixBundleArtifactsFromTargetDir({
  targetDir,
  platform = process.platform,
  buildStartedAt = 0,
} = {}) {
  if (platform !== 'win32') {
    return null;
  }

  const normalizedTargetDir = String(targetDir ?? '').trim();
  if (normalizedTargetDir.length === 0 || !fs.existsSync(normalizedTargetDir)) {
    return null;
  }

  const wixRoot = path.join(normalizedTargetDir, 'release', 'wix');
  if (!fs.existsSync(wixRoot)) {
    return null;
  }

  const freshnessCutoff = resolveWindowsWixArtifactFreshnessCutoff(buildStartedAt);
  const artifactCandidates = fs.readdirSync(wixRoot, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => {
      const wixDir = path.join(wixRoot, entry.name);
      const wixSourcePath = path.join(wixDir, 'main.wxs');
      const wixObjPath = path.join(wixDir, 'main.wixobj');
      const wixPdbPath = path.join(wixDir, WIX_PDB_NAME);
      const localePath = path.join(wixDir, 'locale.wxl');
      if (!fs.existsSync(wixSourcePath) || !fs.existsSync(wixObjPath)) {
        return null;
      }

      const artifactMtimeMs = resolveWindowsWixArtifactMtimeMs([
        wixSourcePath,
        wixObjPath,
      ]);
      if (artifactMtimeMs < freshnessCutoff) {
        return null;
      }

      const productMetadata = readWindowsWixProductMetadata(wixSourcePath);
      if (!productMetadata) {
        return null;
      }

      return {
        targetDir: normalizedTargetDir,
        wixDir,
        wixSourcePath,
        wixObjPath,
        wixPdbPath,
        localePath,
        msiPath: path.join(
          normalizedTargetDir,
          'release',
          'bundle',
          'msi',
          `${productMetadata.productName}_${productMetadata.productVersion}_${entry.name}_${readWindowsWixCulture(localePath)}.msi`,
        ),
        architecture: entry.name,
      };
    })
    .filter(Boolean)
    .sort(
      (left, right) => resolveWindowsWixRetryArtifactMtimeMs(right) - resolveWindowsWixRetryArtifactMtimeMs(left),
    );

  return artifactCandidates[0] ?? null;
}

function resolveCargoBinDir(baseEnv = process.env, platform = process.platform) {
  if (platform === 'win32') {
    const cargoHome = String(baseEnv.CARGO_HOME ?? '').trim()
      || (baseEnv.USERPROFILE ? path.join(baseEnv.USERPROFILE, '.cargo') : '');
    return cargoHome ? path.join(cargoHome, 'bin') : null;
  }

  const home = String(baseEnv.HOME ?? '').trim();
  return home ? path.join(home, '.cargo', 'bin') : null;
}

function withCargoToolchainOnPath(baseEnv = process.env, platform = process.platform) {
  const env = { ...baseEnv };
  const cargoBinDir = resolveCargoBinDir(baseEnv, platform);
  if (!cargoBinDir || !fs.existsSync(cargoBinDir)) {
    return env;
  }

  const currentPath = String(env.PATH ?? env.Path ?? '').trim();
  const pathEntries = currentPath ? currentPath.split(path.delimiter) : [];
  if (!pathEntries.some((entry) => entry.toLowerCase() === cargoBinDir.toLowerCase())) {
    const joinedPath = [cargoBinDir, ...pathEntries].filter(Boolean).join(path.delimiter);
    env.PATH = joinedPath;
    env.Path = joinedPath;
  }

  return env;
}

function resolveWindowsCargoTargetDir(baseEnv = process.env, cwd = process.cwd()) {
  const existingTargetDir = String(baseEnv.CARGO_TARGET_DIR ?? '').trim();
  if (existingTargetDir) {
    return null;
  }

  const tempRoot = String(baseEnv.TEMP ?? baseEnv.TMP ?? '').trim()
    || (baseEnv.USERPROFILE ? path.join(baseEnv.USERPROFILE, 'AppData', 'Local', 'Temp') : '');
  if (!tempRoot || !fs.existsSync(tempRoot)) {
    return null;
  }

  const appName = path.basename(cwd).trim().toLowerCase();
  return path.join(tempRoot, 'sdkwork-tauri-target', appName || 'app');
}

function extractTargetTriple(args, env = process.env) {
  for (let index = 0; index < args.length; index += 1) {
    if (args[index] === '--target') {
      return String(args[index + 1] ?? '').trim();
    }
  }

  return String(env?.[DESKTOP_TARGET_ENV_VAR] ?? '').trim();
}

function resolveReadableTauriCliPath({
  cwd = process.cwd(),
  donorRoots = resolveWorkspaceDonorRoots(cwd),
} = {}) {
  ensureLocalNodeModules({
    appRoot: cwd,
    donorRoots,
    requiredPackages: REQUIRED_TAURI_CLI_PACKAGES,
  });
  return resolveReadablePackageEntry({
    appRoot: cwd,
    donorRoots,
    packageName: '@tauri-apps/cli',
    relativeEntry: 'tauri.js',
  });
}

function resolveWindowsTauriToolsRoot(baseEnv = process.env, platform = process.platform) {
  if (platform !== 'win32') {
    return null;
  }

  const localAppData = String(baseEnv.LOCALAPPDATA ?? '').trim()
    || (baseEnv.USERPROFILE ? path.join(baseEnv.USERPROFILE, 'AppData', 'Local') : '');
  if (!localAppData || !fs.existsSync(localAppData)) {
    return null;
  }

  const tauriToolsRoot = path.join(localAppData, 'tauri');
  return fs.existsSync(tauriToolsRoot) ? tauriToolsRoot : null;
}

function resolveWindowsWixLightExecutable(baseEnv = process.env, platform = process.platform) {
  const tauriToolsRoot = resolveWindowsTauriToolsRoot(baseEnv, platform);
  if (!tauriToolsRoot) {
    return null;
  }

  const wixToolDirs = fs.readdirSync(tauriToolsRoot, { withFileTypes: true })
    .filter((entry) => entry.isDirectory() && /^WixTools/i.test(entry.name))
    .map((entry) => path.join(tauriToolsRoot, entry.name, 'light.exe'))
    .filter((candidatePath) => fs.existsSync(candidatePath))
    .sort((left, right) => right.localeCompare(left));

  return wixToolDirs[0] ?? null;
}

export function createTauriCliPlan({
  commandName,
  args = [],
  cwd = process.cwd(),
  env = process.env,
  platform = process.platform,
} = {}) {
  if (typeof commandName !== 'string' || commandName.trim().length === 0) {
    throw new Error('commandName is required.');
  }

  const background = shouldLaunchInBackground(commandName, args, env);
  const requestedTargetTriple = extractTargetTriple(args, env);
  const resolvedEnv = requestedTargetTriple
    ? buildDesktopReleaseEnv({
        env,
        targetTriple: requestedTargetTriple,
      })
    : { ...env };
  if (platform === 'win32') {
    const shortTargetDir = resolveWindowsCargoTargetDir(resolvedEnv, cwd);
    if (shortTargetDir) {
      resolvedEnv.CARGO_TARGET_DIR = shortTargetDir;
    }
  }

  const tauriCliPath = resolveReadableTauriCliPath({ cwd });

  return {
    command: process.execPath,
    args: [
      tauriCliPath,
      commandName,
      ...createSdkworkChatPcDevConfigArgs({ args, commandName, env: resolvedEnv }),
      ...args,
    ],
    cwd,
    env: withSupportedWindowsCmakeGenerator(
      withStableWindowsNativeBuildConcurrency(
        withCargoToolchainOnPath(resolvedEnv, platform),
        platform,
      ),
      platform,
    ),
    detached: background,
    windowsHide: platform === 'win32',
  };
}

function runForegroundPlan(plan) {
  return new Promise((resolve, reject) => {
    const child = spawn(plan.command, plan.args, {
      cwd: plan.cwd,
      env: plan.env,
      stdio: 'inherit',
      detached: false,
      windowsHide: plan.windowsHide ?? process.platform === 'win32',
    });

    child.on('error', reject);
    child.on('exit', (code, signal) => {
      resolve({
        code,
        signal,
      });
    });
  });
}

function shouldRetryWindowsMsiBundleFromArtifacts({
  artifacts,
  commandName,
  platform = process.platform,
  buildStartedAt = 0,
} = {}) {
  if (platform !== 'win32' || commandName !== 'build' || !artifacts) {
    return false;
  }

  if (!fs.existsSync(artifacts.wixObjPath)) {
    return false;
  }

  const freshnessCutoff = resolveWindowsWixArtifactFreshnessCutoff(buildStartedAt);
  const artifactMtimeMs = resolveWindowsWixRetryArtifactMtimeMs(artifacts);
  if (artifactMtimeMs < freshnessCutoff) {
    return false;
  }

  if (!fs.existsSync(artifacts.msiPath)) {
    return true;
  }

  const msiMtimeMs = fs.statSync(artifacts.msiPath).mtimeMs;
  return msiMtimeMs < Math.max(artifactMtimeMs, freshnessCutoff);
}

async function retryWindowsMsiBundleWithSval({
  artifacts = null,
  output,
  env = process.env,
  platform = process.platform,
} = {}) {
  const resolvedArtifacts = artifacts ?? extractWindowsWixBundleArtifacts(output);
  const lightExecutable = resolveWindowsWixLightExecutable(env, platform);
  if (!resolvedArtifacts || !lightExecutable || !fs.existsSync(resolvedArtifacts.wixObjPath)) {
    return false;
  }

  fs.mkdirSync(path.dirname(resolvedArtifacts.msiPath), { recursive: true });

  const lightArgs = [
    '-v',
    '-sval',
    '-out',
    resolvedArtifacts.msiPath,
    '-pdbout',
    resolvedArtifacts.wixPdbPath,
  ];
  if (fs.existsSync(resolvedArtifacts.localePath)) {
    lightArgs.push('-loc', resolvedArtifacts.localePath);
  }
  lightArgs.push('-ext', WIX_UI_EXTENSION, resolvedArtifacts.wixObjPath);

  console.error('[run-tauri-cli] retrying WiX MSI link with -sval after ICE validation failure');
  const retryResult = await runForegroundPlan({
    command: lightExecutable,
    args: lightArgs,
    cwd: resolvedArtifacts.wixDir,
    env,
    windowsHide: true,
  });

  return !retryResult.signal
    && retryResult.code === 0
    && fs.existsSync(resolvedArtifacts.msiPath);
}

async function runCli() {
  const [commandName, ...args] = process.argv.slice(2);
  const plan = createTauriCliPlan({
    commandName,
    args,
  });

  if (plan.detached) {
    const child = spawn(plan.command, plan.args, {
      cwd: plan.cwd,
      env: plan.env,
      stdio: 'ignore',
      detached: true,
      windowsHide: plan.windowsHide ?? process.platform === 'win32',
    });
    child.on('error', (error) => {
      console.error(`[run-tauri-cli] ${error.message}`);
      process.exit(1);
    });
    child.unref();
    process.exit(0);
    return;
  }

  const buildStartedAt = Date.now();
  const result = await runForegroundPlan(plan);

  if (result.signal) {
    console.error(`[run-tauri-cli] command exited with signal ${result.signal}`);
    process.exit(1);
    return;
  }

  const retryArtifacts = resolveWindowsWixBundleArtifactsFromTargetDir({
    targetDir: plan.env.CARGO_TARGET_DIR,
    platform: process.platform,
    buildStartedAt,
  });

  if ((result.code ?? 1) !== 0 && shouldRetryWindowsMsiBundleFromArtifacts({
    commandName,
    artifacts: retryArtifacts,
    platform: process.platform,
    buildStartedAt,
  })) {
    const recovered = await retryWindowsMsiBundleWithSval({
      artifacts: retryArtifacts,
      env: plan.env,
      platform: process.platform,
    });
    if (recovered) {
      process.exit(0);
      return;
    }
  }

  process.exit(result.code ?? 0);
}

if (__filename === process.argv[1]) {
  runCli().catch((error) => {
    console.error(`[run-tauri-cli] ${error.message}`);
    process.exit(1);
  });
}
