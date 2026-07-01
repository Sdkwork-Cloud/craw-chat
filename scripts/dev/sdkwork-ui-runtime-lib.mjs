import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';

import { pnpmProcessSpec } from './pnpm-launch-lib.mjs';

const REQUIRED_SDKWORK_UI_DIST_FILES = [
  'index.js',
  'theme.js',
  'components-ui.js',
  'ui-feedback.js',
  'patterns-app-shell.js',
  'patterns-desktop-shell.js',
  'sdkwork-ui.css',
];

const REQUIRED_SDKWORK_UI_BUILD_SOURCE_FILES = [
  'package-contract.ts',
  'docs-governance-contract.ts',
  'generated-reference-contract.json',
];

const REQUIRED_SDKWORK_UI_TYPE_PACKAGES = [
  '@types/react',
  '@types/react-dom',
];

function defaultFileExists(filePath) {
  return fs.existsSync(filePath);
}

function defaultRunProcess(command, args, options) {
  return spawnSync(command, args, options);
}

function readPackageJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function resolvePackageEntryPath(rootPath, packageName) {
  return path.join(rootPath, 'node_modules', ...packageName.split('/'));
}

function canResolvePackageEntry(startRoot, packageName, {
  fileExists = defaultFileExists,
  stopRoot,
} = {}) {
  let currentRoot = path.resolve(startRoot);
  const normalizedStopRoot = stopRoot ? path.resolve(stopRoot) : null;

  while (true) {
    if (fileExists(resolvePackageEntryPath(currentRoot, packageName))) {
      return true;
    }

    if (normalizedStopRoot && currentRoot === normalizedStopRoot) {
      return false;
    }

    const parentRoot = path.dirname(currentRoot);

    if (parentRoot === currentRoot) {
      return false;
    }

    currentRoot = parentRoot;
  }
}

function resolveSdkworkUiInstallRoot(appRoot, uiPackageRoot, fileExists = defaultFileExists) {
  let currentRoot = path.resolve(appRoot);

  while (true) {
    if (fileExists(path.join(currentRoot, 'pnpm-workspace.yaml'))) {
      return currentRoot;
    }

    const parentRoot = path.dirname(currentRoot);
    if (parentRoot === currentRoot) {
      break;
    }
    currentRoot = parentRoot;
  }

  return uiPackageRoot;
}

export function resolveSdkworkUiPackageRoot(appRoot, {
  fileExists = defaultFileExists,
} = {}) {
  if (!appRoot) {
    throw new Error('appRoot is required');
  }

  const normalizedAppRoot = path.resolve(appRoot);
  const candidateRoots = [
    path.resolve(normalizedAppRoot, 'packages', 'sdkwork-ui-pc-react'),
    path.resolve(
      normalizedAppRoot,
      '..',
      '..',
      '..',
      'sdkwork-ui',
      'sdkwork-ui-pc-react',
    ),
  ];

  return candidateRoots.find((candidateRoot) => (
    fileExists(path.join(candidateRoot, 'package.json'))
  )) ?? null;
}

function hasSdkworkUiBuildTooling(uiPackageRoot, fileExists = defaultFileExists) {
  return fileExists(path.join(uiPackageRoot, 'node_modules', 'vite', 'bin', 'vite.js'));
}

function hasSdkworkUiRuntimeDependencies(uiPackageRoot, {
  appRoot,
  fileExists = defaultFileExists,
} = {}) {
  const packageJsonPath = path.join(uiPackageRoot, 'package.json');

  if (!fileExists(packageJsonPath)) {
    return false;
  }

  const packageJson = readPackageJson(packageJsonPath);
  const requiredPackages = new Set([
    ...Object.keys(packageJson.dependencies ?? {}),
    ...Object.keys(packageJson.peerDependencies ?? {}),
    ...REQUIRED_SDKWORK_UI_TYPE_PACKAGES,
  ]);

  return [...requiredPackages].every((packageName) => (
    canResolvePackageEntry(uiPackageRoot, packageName, {
      fileExists,
      stopRoot: appRoot,
    })
  ));
}

export function hasSdkworkUiDist(uiPackageRoot, {
  fileExists = defaultFileExists,
} = {}) {
  return REQUIRED_SDKWORK_UI_DIST_FILES.every((entryPath) => (
    fileExists(path.join(uiPackageRoot, 'dist', entryPath))
  ));
}

function hasSdkworkUiBuildSources(uiPackageRoot, {
  fileExists = defaultFileExists,
} = {}) {
  return REQUIRED_SDKWORK_UI_BUILD_SOURCE_FILES.every((entryPath) => (
    fileExists(path.join(uiPackageRoot, 'build', entryPath))
  ));
}

function restoreSdkworkUiBuildSources(uiPackageRoot, {
  runProcess = defaultRunProcess,
} = {}) {
  const result = runProcess('git', ['checkout', 'HEAD', '--', 'build/'], {
    cwd: uiPackageRoot,
    env: process.env,
    stdio: 'pipe',
  });

  return result.status === 0;
}

function ensureSdkworkUiBuildSources(uiPackageRoot, {
  fileExists = defaultFileExists,
  runProcess = defaultRunProcess,
} = {}) {
  if (hasSdkworkUiBuildSources(uiPackageRoot, { fileExists })) {
    return;
  }

  const restored = restoreSdkworkUiBuildSources(uiPackageRoot, { runProcess });

  if (!restored || !hasSdkworkUiBuildSources(uiPackageRoot, { fileExists })) {
    throw new Error(
      `sdkwork-ui build source files are missing in ${uiPackageRoot}/build/ and could not be restored from git. `
      + `Required files: ${REQUIRED_SDKWORK_UI_BUILD_SOURCE_FILES.join(', ')}. `
      + `Please run "git checkout HEAD -- build/" manually in ${uiPackageRoot}.`,
    );
  }
}

function runPnpmStep(stepArgs, {
  cwd,
  runProcess = defaultRunProcess,
} = {}) {
  const { command, args } = pnpmProcessSpec(stepArgs);
  const result = runProcess(command, args, {
    cwd,
    env: process.env,
    stdio: 'inherit',
  });

  if (result.status !== 0) {
    throw new Error(
      `pnpm ${stepArgs.join(' ')} failed in ${cwd} with exit code ${result.status ?? 'unknown'}`,
    );
  }
}

function installSdkworkUiWorkspaceDependencies({
  appRoot,
  uiPackageRoot,
  fileExists = defaultFileExists,
  runProcess = defaultRunProcess,
}) {
  runPnpmStep(
    ['install', '--ignore-scripts', '--force', '--config.confirmModulesPurge=false'],
    {
      cwd: resolveSdkworkUiInstallRoot(appRoot, uiPackageRoot, fileExists),
      runProcess,
    },
  );
}

export function ensureSdkworkUiDist({
  appRoot,
  fileExists = defaultFileExists,
  runProcess = defaultRunProcess,
} = {}) {
  const uiPackageRoot = resolveSdkworkUiPackageRoot(appRoot, { fileExists });

  if (!uiPackageRoot) {
    return null;
  }

  if (!hasSdkworkUiRuntimeDependencies(uiPackageRoot, { appRoot, fileExists })) {
    installSdkworkUiWorkspaceDependencies({
      appRoot,
      uiPackageRoot,
      fileExists,
      runProcess,
    });
  }

  if (hasSdkworkUiDist(uiPackageRoot, { fileExists })) {
    return uiPackageRoot;
  }

  ensureSdkworkUiBuildSources(uiPackageRoot, { fileExists, runProcess });

  if (!hasSdkworkUiBuildTooling(uiPackageRoot, fileExists)) {
    installSdkworkUiWorkspaceDependencies({
      appRoot,
      uiPackageRoot,
      fileExists,
      runProcess,
    });
  }

  if (!hasSdkworkUiDist(uiPackageRoot, { fileExists })) {
    runPnpmStep(['build'], {
      cwd: uiPackageRoot,
      runProcess,
    });
  }

  if (!hasSdkworkUiDist(uiPackageRoot, { fileExists })) {
    throw new Error(`sdkwork ui dist is still missing after build: ${uiPackageRoot}`);
  }

  return uiPackageRoot;
}
