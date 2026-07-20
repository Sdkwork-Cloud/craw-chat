#!/usr/bin/env node

import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import {
  copyFileSync,
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  rmSync,
  writeFileSync,
} from 'node:fs';
import { gzipSync } from 'node:zlib';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { createFlutterDefineConfig } from '../../apps/sdkwork-im-flutter-mobile/scripts/flutter-dev.mjs';
import { loadProfile } from '../lib/im-topology.mjs';
import { createTar, createZip } from './build-sdkwork-im-install-package.mjs';
import { normalizeSdkworkImReleaseVersion } from './sdkwork-im-release-version.mjs';

const MODULE_PATH = fileURLToPath(import.meta.url);
const REPO_ROOT = path.resolve(path.dirname(MODULE_PATH), '..', '..');
const PACKAGE_DEFINITIONS = Object.freeze({
  'h5-universal-cloud-mobile-zip': Object.freeze({
    architecture: 'universal',
    artifactExtension: 'zip',
    buildKind: 'h5',
    clientArchitecture: 'h5',
    deploymentProfile: 'cloud',
    platform: 'h5',
    profile: 'mobile',
    profileId: 'cloud.production',
    runtimeTarget: 'browser',
    targetPlatform: 'h5',
  }),
  'android-universal-cloud-mobile-apk': Object.freeze({
    architecture: 'universal',
    artifactExtension: 'apk',
    buildKind: 'flutter-android-apk',
    clientArchitecture: 'flutter',
    deploymentProfile: 'cloud',
    platform: 'android',
    profile: 'mobile',
    profileId: 'cloud.production',
    runtimeTarget: 'flutter-android',
    targetPlatform: 'android',
  }),
  'android-universal-cloud-mobile-aab': Object.freeze({
    architecture: 'universal',
    artifactExtension: 'aab',
    buildKind: 'flutter-android-aab',
    clientArchitecture: 'flutter',
    deploymentProfile: 'cloud',
    platform: 'android',
    profile: 'mobile',
    profileId: 'cloud.production',
    runtimeTarget: 'flutter-android',
    targetPlatform: 'android',
  }),
  'ios-universal-cloud-mobile-ipa': Object.freeze({
    architecture: 'universal',
    artifactExtension: 'ipa',
    buildKind: 'flutter-ios-ipa',
    clientArchitecture: 'flutter',
    deploymentProfile: 'cloud',
    platform: 'ios',
    profile: 'mobile',
    profileId: 'cloud.production',
    runtimeTarget: 'flutter-ios',
    targetPlatform: 'ios',
  }),
  'container-x64-cloud-container-kubernetes-tar-gz': Object.freeze({
    architecture: 'x64',
    artifactExtension: 'tar.gz',
    buildKind: 'kubernetes-bundle',
    deploymentProfile: 'cloud',
    platform: 'container',
    profile: 'container',
    profileId: 'cloud.production',
    runtimeTarget: 'container',
  }),
});

function requireText(value, label) {
  const normalized = String(value ?? '').trim();
  if (!normalized) throw new Error(`${label} is required`);
  return normalized;
}

function definitionFor(packageId) {
  const definition = PACKAGE_DEFINITIONS[packageId];
  if (!definition) throw new Error(`unsupported workflow release package: ${packageId}`);
  return definition;
}

function artifactPathFor(packageId, version, root = REPO_ROOT) {
  const definition = definitionFor(packageId);
  return path.join(
    root,
    'dist',
    'release-packages',
    `sdkwork-im-${packageId}-${normalizeSdkworkImReleaseVersion(version)}.${definition.artifactExtension}`,
  );
}

function packageManifestPathFor(packageId, version, root = REPO_ROOT) {
  return path.join(
    root,
    'dist',
    'release-packages',
    `sdkwork-im-${packageId}-${normalizeSdkworkImReleaseVersion(version)}.manifest.json`,
  );
}

function executable(name, platform = process.platform) {
  if (platform !== 'win32') return name;
  if (name === 'pnpm') return 'pnpm.cmd';
  if (name === 'flutter') return 'flutter.bat';
  return name;
}

function createBuildPlan({ env = process.env, packageId, platform = process.platform, root = REPO_ROOT, version }) {
  const definition = definitionFor(packageId);
  const releaseVersion = normalizeSdkworkImReleaseVersion(version);
  const profileEnv = loadProfile(definition.profileId);
  const buildEnv = { ...env, ...profileEnv };
  const steps = [];
  let flutterConfig = null;
  let flutterConfigPath = null;

  if (definition.buildKind === 'h5') {
    steps.push({
      command: executable('pnpm', platform),
      args: ['run', '_sdkwork:build'],
      cwd: path.join(root, 'apps', 'sdkwork-im-h5'),
      env: buildEnv,
      label: 'build H5 production bundle from cloud.production source config',
    });
  } else if (definition.buildKind.startsWith('flutter-')) {
    flutterConfig = createFlutterDefineConfig(buildEnv);
    flutterConfigPath = path.join(root, '.runtime', 'release', `${packageId}.dart-define.json`);
    const flutterTarget = definition.buildKind === 'flutter-android-apk'
      ? 'apk'
      : definition.buildKind === 'flutter-android-aab'
        ? 'appbundle'
        : 'ipa';
    const args = [
      'build',
      flutterTarget,
      '--release',
      '--build-name',
      releaseVersion,
      '--dart-define-from-file',
      flutterConfigPath,
    ];
    if (flutterTarget === 'ipa') {
      if (platform !== 'darwin') throw new Error('Flutter IPA packaging requires a macOS/Xcode runner');
      args.push('--export-options-plist', requireText(env.SDKWORK_IOS_EXPORT_OPTIONS_PLIST, 'SDKWORK_IOS_EXPORT_OPTIONS_PLIST'));
    }
    steps.push({
      command: executable('flutter', platform),
      args,
      cwd: path.join(root, 'apps', 'sdkwork-im-flutter-mobile'),
      env: buildEnv,
      label: `build signed Flutter ${flutterTarget} from cloud.production source config`,
    });
  } else if (definition.buildKind === 'kubernetes-bundle') {
    const imageLock = requireText(env.SDKWORK_IM_CLOUD_IMAGE_LOCK_FILE, 'SDKWORK_IM_CLOUD_IMAGE_LOCK_FILE');
    steps.push({
      command: process.execPath,
      args: [
        'scripts/release/materialize-sdkwork-im-kubernetes.mjs',
        '--image-lock',
        imageLock,
        '--output',
        path.join(root, 'dist', 'cloud-release', packageId),
      ],
      cwd: root,
      env: buildEnv,
      label: 'materialize cloud Kubernetes bundle from immutable image digests',
    });
  }

  return { definition, flutterConfig, flutterConfigPath, packageId, releaseVersion, steps };
}

function runBuildPlan(plan) {
  let androidKeystorePath = null;
  if (plan.flutterConfigPath) {
    mkdirSync(path.dirname(plan.flutterConfigPath), { recursive: true });
    writeFileSync(plan.flutterConfigPath, `${JSON.stringify(plan.flutterConfig, null, 2)}\n`, { mode: 0o600 });
  }
  if (plan.definition.buildKind.startsWith('flutter-android-')) {
    androidKeystorePath = prepareAndroidSigning(plan);
  }
  try {
    for (const step of plan.steps) runStep(step);
  } finally {
    if (androidKeystorePath) rmSync(androidKeystorePath, { force: true });
  }
}

function prepareAndroidSigning(plan) {
  const step = plan.steps[0];
  const encodedKeystore = requireText(step.env.SDKWORK_ANDROID_KEYSTORE_BASE64, 'SDKWORK_ANDROID_KEYSTORE_BASE64');
  const keystore = Buffer.from(encodedKeystore, 'base64');
  if (keystore.length === 0) throw new Error('SDKWORK_ANDROID_KEYSTORE_BASE64 did not decode to key material');
  const keystorePath = path.join(REPO_ROOT, '.runtime', 'release', 'android-release.keystore');
  mkdirSync(path.dirname(keystorePath), { recursive: true });
  writeFileSync(keystorePath, keystore, { mode: 0o600 });
  step.env = {
    ...step.env,
    ORG_GRADLE_PROJECT_SDKWORK_RELEASE_KEYSTORE_FILE: keystorePath,
    ORG_GRADLE_PROJECT_SDKWORK_RELEASE_KEYSTORE_PASSWORD: requireText(
      step.env.SDKWORK_ANDROID_KEYSTORE_PASSWORD,
      'SDKWORK_ANDROID_KEYSTORE_PASSWORD',
    ),
    ORG_GRADLE_PROJECT_SDKWORK_RELEASE_KEY_ALIAS: requireText(
      step.env.SDKWORK_ANDROID_KEY_ALIAS,
      'SDKWORK_ANDROID_KEY_ALIAS',
    ),
    ORG_GRADLE_PROJECT_SDKWORK_RELEASE_KEY_PASSWORD: requireText(
      step.env.SDKWORK_ANDROID_KEY_PASSWORD,
      'SDKWORK_ANDROID_KEY_PASSWORD',
    ),
  };
  return keystorePath;
}

function runStep(step) {
  const result = spawnSync(step.command, step.args, {
    cwd: step.cwd,
    env: step.env,
    shell: process.platform === 'win32' && /\.(?:cmd|bat)$/iu.test(step.command),
    stdio: 'inherit',
    windowsHide: true,
  });
  if (result.error) throw result.error;
  if (result.status !== 0) throw new Error(`${step.label} failed with exit code ${result.status ?? 1}`);
}

function sourceArtifactFor(packageId, root = REPO_ROOT) {
  const definition = definitionFor(packageId);
  if (definition.buildKind === 'h5') return path.join(root, 'apps', 'sdkwork-im-h5', 'dist');
  if (definition.buildKind === 'flutter-android-apk') {
    return path.join(root, 'apps', 'sdkwork-im-flutter-mobile', 'build', 'app', 'outputs', 'flutter-apk', 'app-release.apk');
  }
  if (definition.buildKind === 'flutter-android-aab') {
    return path.join(root, 'apps', 'sdkwork-im-flutter-mobile', 'build', 'app', 'outputs', 'bundle', 'release', 'app-release.aab');
  }
  if (definition.buildKind === 'flutter-ios-ipa') {
    const ipaRoot = path.join(root, 'apps', 'sdkwork-im-flutter-mobile', 'build', 'ios', 'ipa');
    return singleFileWithExtension(ipaRoot, '.ipa');
  }
  return path.join(root, 'dist', 'cloud-release', packageId);
}

function packageReleaseTarget({ packageId, root = REPO_ROOT, version }) {
  const definition = definitionFor(packageId);
  const releaseVersion = normalizeSdkworkImReleaseVersion(version);
  const source = sourceArtifactFor(packageId, root);
  if (!existsSync(source)) throw new Error(`required build output does not exist: ${source}`);
  const artifactPath = artifactPathFor(packageId, releaseVersion, root);
  mkdirSync(path.dirname(artifactPath), { recursive: true });

  if (definition.buildKind === 'h5') {
    writeFileSync(artifactPath, createZip(collectArchiveFiles(source)));
  } else if (definition.buildKind === 'kubernetes-bundle') {
    writeFileSync(artifactPath, gzipSync(createTar(collectArchiveFiles(source)), { mtime: 0 }));
  } else {
    copyFileSync(source, artifactPath);
  }

  const artifactBytes = readFileSync(artifactPath);
  if (artifactBytes.length === 0) throw new Error(`${packageId} produced an empty artifact`);
  const manifest = {
    schemaVersion: 1,
    appId: 'sdkwork-im',
    packageId,
    version: releaseVersion,
    artifactPath: portable(path.relative(root, artifactPath)),
    sizeBytes: artifactBytes.length,
    sha256: sha256(artifactBytes),
    deploymentProfile: definition.deploymentProfile,
    runtimeTarget: definition.runtimeTarget,
    targetPlatform: definition.targetPlatform ?? null,
    clientArchitecture: definition.clientArchitecture ?? null,
  };
  const manifestPath = packageManifestPathFor(packageId, releaseVersion, root);
  writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, 'utf8');
  writeFileSync(
    path.join(path.dirname(artifactPath), `sdkwork-im-${packageId}-${releaseVersion}.SHA256SUMS`),
    `${manifest.sha256}  ${path.basename(artifactPath)}\n`,
    'utf8',
  );
  return { artifactPath, manifest, manifestPath };
}

function validateReleaseTarget({ packageId, root = REPO_ROOT, version }) {
  const artifactPath = artifactPathFor(packageId, version, root);
  const manifestPath = packageManifestPathFor(packageId, version, root);
  if (!existsSync(artifactPath)) throw new Error(`release artifact does not exist: ${artifactPath}`);
  if (!existsSync(manifestPath)) throw new Error(`release manifest does not exist: ${manifestPath}`);
  const bytes = readFileSync(artifactPath);
  const manifest = JSON.parse(readFileSync(manifestPath, 'utf8'));
  if (bytes.length === 0) throw new Error(`${packageId} release artifact is empty`);
  if (manifest.packageId !== packageId) throw new Error(`${packageId} release manifest package id is invalid`);
  if (manifest.sha256 !== sha256(bytes)) throw new Error(`${packageId} release manifest digest does not match artifact bytes`);
  return { artifactPath, manifestPath, sha256: manifest.sha256, sizeBytes: bytes.length };
}

function collectArchiveFiles(root) {
  const entries = [];
  for (const filePath of listFiles(root)) {
    entries.push({
      relativePath: portable(path.relative(root, filePath)),
      data: readFileSync(filePath),
      mode: 0o644,
    });
  }
  if (entries.length === 0) throw new Error(`package source directory is empty: ${root}`);
  return entries;
}

function listFiles(root) {
  const files = [];
  for (const entry of readdirSync(root, { withFileTypes: true })) {
    const entryPath = path.join(root, entry.name);
    if (entry.isDirectory()) files.push(...listFiles(entryPath));
    else if (entry.isFile()) files.push(entryPath);
  }
  return files.sort();
}

function singleFileWithExtension(root, extension) {
  if (!existsSync(root)) throw new Error(`build output directory does not exist: ${root}`);
  const candidates = listFiles(root).filter((filePath) => filePath.toLowerCase().endsWith(extension));
  if (candidates.length !== 1) throw new Error(`${root} must contain exactly one ${extension} artifact, found ${candidates.length}`);
  return candidates[0];
}

function sha256(value) {
  return createHash('sha256').update(value).digest('hex');
}

function portable(value) {
  return value.split(path.sep).join('/');
}

function parseArgs(argv) {
  const [command, ...rest] = argv;
  const settings = { command, dryRun: false, packageId: null, version: null };
  for (let index = 0; index < rest.length; index += 1) {
    const arg = rest[index];
    if (arg === '--dry-run') settings.dryRun = true;
    else if (arg === '--package-id') settings.packageId = rest[++index];
    else if (arg === '--version') settings.version = rest[++index];
    else throw new Error(`unsupported workflow target option: ${arg}`);
  }
  if (!['build', 'package', 'validate'].includes(command)) throw new Error('command must be build, package, or validate');
  requireText(settings.packageId, '--package-id');
  requireText(settings.version, '--version');
  return settings;
}

async function main(argv = process.argv.slice(2)) {
  const settings = parseArgs(argv);
  if (settings.command === 'build') {
    const plan = createBuildPlan(settings);
    console.log(JSON.stringify({
      ok: true,
      dryRun: settings.dryRun,
      packageId: plan.packageId,
      version: plan.releaseVersion,
      steps: plan.steps.map(({ env, ...step }) => ({ ...step, envKeys: Object.keys(env).filter((key) => key.startsWith('SDKWORK_')).sort() })),
    }, null, 2));
    if (!settings.dryRun) runBuildPlan(plan);
    return 0;
  }
  const result = settings.command === 'package'
    ? packageReleaseTarget(settings)
    : validateReleaseTarget(settings);
  console.log(JSON.stringify({ ok: true, ...result }, null, 2));
  return 0;
}

if (process.argv[1] && path.resolve(process.argv[1]) === MODULE_PATH) {
  main().then((code) => { process.exitCode = code; }).catch((error) => {
    console.error(`[sdkwork-im-workflow-target] ${error instanceof Error ? error.message : String(error)}`);
    process.exitCode = 1;
  });
}

export {
  PACKAGE_DEFINITIONS,
  artifactPathFor,
  createBuildPlan,
  definitionFor,
  packageReleaseTarget,
  packageManifestPathFor,
  runBuildPlan,
  validateReleaseTarget,
};
