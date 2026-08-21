#!/usr/bin/env node

// Native install package matrix for Sdkwork IM standalone deployment.
//
// Archive/container packaging keeps the existing 18-package matrix in
// plan-sdkwork-im-install-packages.mjs. Native platform installers
// (.deb/.msi/.pkg server packages, Tauri desktop installers published
// directly) follow the GITHUB_WORKFLOW_SPEC §5 id grammar, which requires a
// distribution segment on Linux native items:
//
//   linux-ubuntu-x64-standalone-server-deb
//   windows-x64-standalone-server-msi
//   macos-x64-standalone-server-pkg
//   linux-ubuntu-x64-standalone-desktop-deb   (Tauri bundle, collected)
//
// The server native items are built by
// scripts/release/build-sdkwork-im-native-installer.mjs from the staged
// server package (the same dist/release-staging/<archive-package-id> layout
// the archive builder consumes); the desktop native items are the Tauri
// bundles collected by collect-sdkwork-im-desktop-bundles.mjs and published
// under the canonical installer names.

import process from 'node:process';
import { DEFAULT_RELEASE_VERSION, normalizeSdkworkImReleaseVersion } from './sdkwork-im-release-version.mjs';

const NATIVE_INSTALL_PACKAGE_SCHEMA_VERSION = '2026-08-08.sdkwork-im.native-install-packages.v1';
const NATIVE_SERVER_FORMATS = Object.freeze(['deb', 'msi', 'pkg']);
const LINUX_DISTRIBUTIONS = Object.freeze(['ubuntu']);
const SUPPORTED_PLATFORMS = Object.freeze(['linux', 'windows', 'macos']);
const SUPPORTED_ARCHITECTURES = Object.freeze(['x64', 'arm64']);
const APP_CODE = 'chat';
const PRODUCT_NAME = 'chat';
const PACKAGE_NAME = 'sdkwork-chat';
const RUNTIME_DISPLAY_NAME = 'Sdkwork IM';
const SERVER_BINARY_BASENAME = 'sdkwork-api-im-standalone-gateway';
const SERVER_FORMAT_FOR_PLATFORM = Object.freeze({ linux: 'deb', windows: 'msi', macos: 'pkg' });
// Tauri bundle.targets "all" produces these native formats per platform.
const DESKTOP_FORMAT_FOR_PLATFORM = Object.freeze({
  linux: ['deb', 'appimage'],
  windows: ['msi', 'exe'],
  macos: ['dmg'],
});
// GITHUB_WORKFLOW_SPEC §5: Linux native package items MUST carry the
// distribution segment (`linux-<distribution>-<architecture>-...`).
const LINUX_DISTRIBUTION = 'ubuntu';
// The staged archive package id whose dist/release-staging/<id> layout feeds
// the native server installer (same staged files, native mapping).
const STAGING_ARCHIVE_PACKAGE_ID = Object.freeze({
  linux: { x64: 'linux-x64-standalone-server-tar-gz', arm64: 'linux-arm64-standalone-server-tar-gz' },
  windows: { x64: 'windows-x64-standalone-server-zip', arm64: 'windows-arm64-standalone-server-zip' },
  macos: { x64: 'macos-x64-standalone-server-tar-gz', arm64: 'macos-arm64-standalone-server-tar-gz' },
});

function printHelp() {
  console.log(`Usage: node scripts/release/plan-sdkwork-im-native-install-packages.mjs [options]

Create and validate the Sdkwork IM native install package plan.

Options:
  --check             Validate the generated plan and exit nonzero on issues.
  --json              Print machine-readable JSON.
  --version <value>   Package version (default ${DEFAULT_RELEASE_VERSION}).
  --platform <value>  Platform subset: all, linux, windows, macos.
  --architecture <v>  Architecture subset: all, x64, arm64.
  --profile <value>   Package profile subset: all, server, desktop.
  -h, --help          Show this help.
`);
}

function requireValue(argv, index, flag) {
  const value = argv[index + 1];
  if (!value || value.startsWith('--')) {
    throw new Error(`${flag} requires a value`);
  }
  return value;
}

function parseArgs(argv = process.argv.slice(2)) {
  const settings = {
    architectures: SUPPORTED_ARCHITECTURES,
    check: false,
    help: false,
    json: false,
    platforms: SUPPORTED_PLATFORMS,
    profiles: ['server', 'desktop'],
    version: DEFAULT_RELEASE_VERSION,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === '--') {
      continue;
    }
    switch (arg) {
      case '--check':
        settings.check = true;
        break;
      case '--json':
        settings.json = true;
        break;
      case '--version':
        settings.version = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--platform':
        settings.platforms = parseSelectionFlag(requireValue(argv, index, arg), SUPPORTED_PLATFORMS);
        index += 1;
        break;
      case '--architecture':
      case '--arch':
        settings.architectures = parseSelectionFlag(requireValue(argv, index, arg), SUPPORTED_ARCHITECTURES);
        index += 1;
        break;
      case '--profile':
        settings.profiles = parseSelectionFlag(requireValue(argv, index, arg), ['server', 'desktop']);
        index += 1;
        break;
      case '--help':
      case '-h':
        settings.help = true;
        break;
      default:
        throw new Error(`Unsupported native install package planner option: ${arg}`);
    }
  }

  return settings;
}

function createSdkworkImNativeInstallPackagePlan({
  version = DEFAULT_RELEASE_VERSION,
  platforms = SUPPORTED_PLATFORMS,
  architectures = SUPPORTED_ARCHITECTURES,
  profiles = ['server', 'desktop'],
} = {}) {
  const normalizedVersion = normalizeSdkworkImReleaseVersion(version);
  const selectedPlatforms = validateSelection('platforms', platforms, SUPPORTED_PLATFORMS);
  const selectedArchitectures = validateSelection('architectures', architectures, SUPPORTED_ARCHITECTURES);
  const selectedProfiles = validateSelection('profiles', profiles, ['server', 'desktop']);

  const packages = expectedNativePackageDefinitions({
    architectures: selectedArchitectures,
    platforms: selectedPlatforms,
    profiles: selectedProfiles,
  }).map((definition) => createNativeInstallPackageItem({
    ...definition,
    version: normalizedVersion,
  }));

  return {
    schemaVersion: NATIVE_INSTALL_PACKAGE_SCHEMA_VERSION,
    appCode: APP_CODE,
    product: PRODUCT_NAME,
    packageName: PACKAGE_NAME,
    runtimeName: APP_CODE,
    displayName: RUNTIME_DISPLAY_NAME,
    version: normalizedVersion,
    platforms: selectedPlatforms,
    architectures: selectedArchitectures,
    deploymentProfiles: ['standalone'],
    profiles: selectedProfiles,
    artifactPolicy: {
      noSecretsInPackage: true,
      envLocalGeneratedOnHost: true,
      envExampleReferenceOnly: true,
      releaseEnvLocalExcluded: true,
      generatedFromProductionBuild: true,
      excludesRuntimeState: true,
    },
    packages,
  };
}

function createNativeInstallPackageItem({
  architecture,
  distribution,
  format,
  platform,
  profile,
  version,
}) {
  const id = nativePackageId({ architecture, distribution, format, platform, profile });
  const artifactId = id.replace(/-deb$/u, '').replace(/-msi$/u, '').replace(/-pkg$/u, '')
    .replace(/-appimage$/u, '').replace(/-exe$/u, '').replace(/-dmg$/u, '');
  const extension = format === 'appimage' ? 'AppImage' : format;
  const isServer = profile === 'server';
  return {
    id,
    artifactId,
    version,
    platform,
    distribution: distribution ?? null,
    architecture,
    deploymentProfile: 'standalone',
    profile,
    runtimeTarget: profile,
    format,
    extension,
    installerName: `sdkwork-im-${id}-${version}.${extension}`,
    packageKind: isServer ? 'server-native-installer' : 'desktop-native-installer',
    binaryName: isServer ? `${SERVER_BINARY_BASENAME}${platform === 'windows' ? '.exe' : ''}` : null,
    startCommand: isServer
      ? (platform === 'windows' ? 'sc start sdkwork-im' : 'sudo systemctl start sdkwork-im')
      : null,
    healthChecks: isServer ? ['/healthz', '/readyz'] : [],
    stagingPackageId: isServer ? STAGING_ARCHIVE_PACKAGE_ID[platform]?.[architecture] : null,
    serviceName: isServer
      ? (platform === 'windows' ? 'sdkwork-im' : 'sdkwork-im.service')
      : null,
    buildHost: nativeBuildHostFor({ architecture, platform }),
    security: {
      noSecretsInPackage: true,
      envLocalGeneratedOnHost: true,
      envExampleReferenceOnly: true,
      excludesRuntimeState: true,
    },
  };
}

function nativePackageId({ architecture, distribution, format, platform, profile }) {
  const platformSegment = platform === 'linux' ? `linux-${distribution ?? LINUX_DISTRIBUTION}` : platform;
  return `${platformSegment}-${architecture}-standalone-${profile}-${format}`;
}

function nativeBuildHostFor({ architecture, platform }) {
  if (platform === 'linux') {
    return architecture === 'arm64' ? 'ubuntu-24.04-arm' : 'ubuntu-latest';
  }
  if (platform === 'windows') {
    return architecture === 'arm64' ? 'windows-11-arm' : 'windows-latest';
  }
  return architecture === 'arm64' ? 'macos-latest' : 'macos-15-intel';
}

function expectedNativePackageDefinitions({ architectures, platforms, profiles }) {
  const definitions = [];
  for (const profile of profiles) {
    for (const platform of platforms) {
      const formats = profile === 'server'
        ? [SERVER_FORMAT_FOR_PLATFORM[platform]]
        : DESKTOP_FORMAT_FOR_PLATFORM[platform] ?? [];
      for (const format of formats) {
        for (const architecture of architectures) {
          definitions.push({
            architecture,
            distribution: platform === 'linux' ? LINUX_DISTRIBUTION : null,
            format,
            platform,
            profile,
          });
        }
      }
    }
  }
  return definitions;
}

function validateSdkworkImNativeInstallPackagePlan(plan) {
  const issues = [];
  if (plan.schemaVersion !== NATIVE_INSTALL_PACKAGE_SCHEMA_VERSION) {
    issues.push(`schemaVersion must be ${NATIVE_INSTALL_PACKAGE_SCHEMA_VERSION}`);
  }
  if (plan.product !== PRODUCT_NAME || plan.packageName !== PACKAGE_NAME) {
    issues.push(`product/packageName must be ${PRODUCT_NAME}/${PACKAGE_NAME}`);
  }
  if (plan.appCode !== APP_CODE || plan.runtimeName !== APP_CODE) {
    issues.push(`appCode and runtimeName must be ${APP_CODE}`);
  }
  if (!/^[0-9A-Za-z][0-9A-Za-z._-]*$/u.test(String(plan.version ?? ''))) {
    issues.push('version must be package-safe');
  }
  validateSubset('platforms', plan.platforms, SUPPORTED_PLATFORMS, issues);
  validateSubset('architectures', plan.architectures, SUPPORTED_ARCHITECTURES, issues);
  validateSubset('profiles', plan.profiles, ['server', 'desktop'], issues);

  const expectedIds = expectedNativePackageDefinitions({
    architectures: plan.architectures ?? [],
    platforms: plan.platforms ?? [],
    profiles: plan.profiles ?? [],
  }).map((definition) => nativePackageId(definition));
  validateArrayMatches(
    'package ids',
    (plan.packages ?? []).map((item) => item.id),
    expectedIds,
    issues,
  );

  const seenIds = new Set();
  for (const packageItem of plan.packages ?? []) {
    validateNativePackageItem(packageItem, seenIds, issues);
  }
  return issues;
}

function validateNativePackageItem(packageItem, seenIds, issues) {
  const expectedId = nativePackageId(packageItem);
  if (packageItem.id !== expectedId) {
    issues.push(`${packageItem.id ?? '(missing id)'} id must be ${expectedId}`);
  }
  if (seenIds.has(packageItem.id)) {
    issues.push(`${packageItem.id} is duplicated`);
  }
  seenIds.add(packageItem.id);
  if (packageItem.deploymentProfile !== 'standalone') {
    issues.push(`${packageItem.id} deploymentProfile must be standalone`);
  }
  if (packageItem.platform === 'linux' && packageItem.distribution !== LINUX_DISTRIBUTION) {
    issues.push(`${packageItem.id} linux distribution must be ${LINUX_DISTRIBUTION}`);
  }
  if (packageItem.platform !== 'linux' && packageItem.distribution !== null) {
    issues.push(`${packageItem.id} only linux packages carry a distribution segment`);
  }
  if (packageItem.profile === 'server') {
    const expectedFormat = SERVER_FORMAT_FOR_PLATFORM[packageItem.platform];
    if (packageItem.format !== expectedFormat) {
      issues.push(`${packageItem.id} format must be ${expectedFormat} for ${packageItem.platform}`);
    }
    if (!NATIVE_SERVER_FORMATS.includes(packageItem.format)) {
      issues.push(`${packageItem.id} has unsupported native server format`);
    }
    if (!packageItem.stagingPackageId) {
      issues.push(`${packageItem.id} must declare stagingPackageId`);
    }
    if (!packageItem.binaryName || !packageItem.serviceName) {
      issues.push(`${packageItem.id} server packages must declare binaryName and serviceName`);
    }
    if (!packageItem.startCommand || packageItem.healthChecks.length !== 2) {
      issues.push(`${packageItem.id} server packages must declare startCommand and health checks`);
    }
  } else {
    const expectedFormats = DESKTOP_FORMAT_FOR_PLATFORM[packageItem.platform] ?? [];
    if (!expectedFormats.includes(packageItem.format)) {
      issues.push(`${packageItem.id} format ${packageItem.format} is not produced by Tauri on ${packageItem.platform}`);
    }
    if (packageItem.binaryName !== null || packageItem.stagingPackageId !== null) {
      issues.push(`${packageItem.id} desktop packages must not declare server staging inputs`);
    }
  }
  if (!String(packageItem.installerName ?? '').endsWith(`.${packageItem.extension}`)) {
    issues.push(`${packageItem.id} installerName must end with .${packageItem.extension}`);
  }
  if (!/^sdkwork-im-[a-z0-9-]+-[0-9][0-9A-Za-z._-]*\.[A-Za-z0-9.]+$/u.test(String(packageItem.installerName ?? ''))) {
    issues.push(`${packageItem.id} installerName must match the canonical artifact grammar`);
  }
  if (packageItem.security?.noSecretsInPackage !== true) {
    issues.push(`${packageItem.id} security.noSecretsInPackage must be true`);
  }
}

function renderSdkworkImNativeInstallPackagePlan(plan) {
  return [
    `[sdkwork-im-native-install-packages] product: ${plan.product}`,
    `[sdkwork-im-native-install-packages] schema: ${plan.schemaVersion}`,
    `[sdkwork-im-native-install-packages] version: ${plan.version}`,
    `[sdkwork-im-native-install-packages] platforms: ${plan.platforms.join(', ')}`,
    `[sdkwork-im-native-install-packages] architectures: ${plan.architectures.join(', ')}`,
    `[sdkwork-im-native-install-packages] packages: ${plan.packages.length}`,
    ...plan.packages.map((packageItem) => [
      `[sdkwork-im-native-install-packages]   ${packageItem.id}`,
      `installer=${packageItem.installerName}`,
      `kind=${packageItem.packageKind}`,
      `format=${packageItem.format}`,
      `staging=${packageItem.stagingPackageId ?? 'tauri-bundle'}`,
      `buildHost=${packageItem.buildHost}`,
    ].join(' ')),
  ];
}

function currentHostNativePackageId(platform = process.platform, arch = process.arch) {
  const normalizedPlatform = platform === 'win32' ? 'windows' : platform === 'darwin' ? 'macos' : 'linux';
  const normalizedArch = arch === 'arm64' ? 'arm64' : 'x64';
  return nativePackageId({
    architecture: normalizedArch,
    distribution: LINUX_DISTRIBUTION,
    format: SERVER_FORMAT_FOR_PLATFORM[normalizedPlatform],
    platform: normalizedPlatform,
    profile: 'server',
  });
}

async function main(argv = process.argv.slice(2)) {
  const settings = parseArgs(argv);
  if (settings.help) {
    printHelp();
    return 0;
  }

  const plan = createSdkworkImNativeInstallPackagePlan({
    architectures: settings.architectures,
    platforms: settings.platforms,
    profiles: settings.profiles,
    version: settings.version,
  });
  const issues = validateSdkworkImNativeInstallPackagePlan(plan);
  if (settings.json) {
    console.log(JSON.stringify({
      ok: issues.length === 0,
      issues,
      plan,
    }, null, 2));
  } else {
    for (const line of renderSdkworkImNativeInstallPackagePlan(plan)) {
      console.log(line);
    }
    if (issues.length > 0) {
      console.error('[sdkwork-im-native-install-packages] validation issues:');
      for (const issue of issues) {
        console.error(`[sdkwork-im-native-install-packages]   ${issue}`);
      }
    } else if (settings.check) {
      console.log('[sdkwork-im-native-install-packages] validation passed');
    }
  }

  if (settings.check && issues.length > 0) {
    return 1;
  }
  return 0;
}

function validateSelection(label, selected, supported) {
  if (!Array.isArray(selected) || selected.length === 0) {
    throw new Error(`${label} must contain at least one value`);
  }
  const unique = [...new Set(selected.map((value) => String(value).trim()))];
  for (const value of unique) {
    if (!supported.includes(value)) {
      throw new Error(`${label} contains unsupported value: ${value}`);
    }
  }
  return unique;
}

function parseSelectionFlag(value, supported) {
  const selected = String(value ?? '')
    .split(',')
    .map((item) => item.trim())
    .filter(Boolean);
  if (selected.length === 0 || selected.includes('all')) {
    return supported;
  }
  return selected;
}

function validateSubset(label, actual, supported, issues) {
  if (!Array.isArray(actual) || actual.length === 0) {
    issues.push(`${label} must contain at least one value`);
    return;
  }
  for (const value of actual) {
    if (!supported.includes(value)) {
      issues.push(`${label} contains unsupported value: ${value}`);
    }
  }
}

function validateArrayMatches(label, actual, expected, issues) {
  if (!arraysEqual(actual, expected)) {
    issues.push(`${label} must be ${expected.join(', ')}`);
  }
}

function arraysEqual(actual, expected) {
  return Array.isArray(actual)
    && actual.length === expected.length
    && actual.every((value, index) => value === expected[index]);
}

if (process.argv[1] && import.meta.url.endsWith(process.argv[1].replaceAll('\\', '/'))) {
  main().then((code) => {
    process.exitCode = code;
  }).catch((error) => {
    console.error(`[sdkwork-im-native-install-packages] ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  });
}

export {
  LINUX_DISTRIBUTION,
  DESKTOP_FORMAT_FOR_PLATFORM,
  NATIVE_INSTALL_PACKAGE_SCHEMA_VERSION,
  NATIVE_SERVER_FORMATS,
  PACKAGE_NAME,
  RUNTIME_DISPLAY_NAME,
  SERVER_BINARY_BASENAME,
  SERVER_FORMAT_FOR_PLATFORM,
  STAGING_ARCHIVE_PACKAGE_ID,
  SUPPORTED_ARCHITECTURES,
  SUPPORTED_PLATFORMS,
  createSdkworkImNativeInstallPackagePlan,
  currentHostNativePackageId,
  main,
  nativePackageId,
  parseArgs,
  renderSdkworkImNativeInstallPackagePlan,
  validateSdkworkImNativeInstallPackagePlan,
};
