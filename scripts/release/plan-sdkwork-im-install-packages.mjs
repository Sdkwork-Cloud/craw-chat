#!/usr/bin/env node

import process from 'node:process';
import { DEFAULT_RELEASE_VERSION, normalizeSdkworkImReleaseVersion } from './sdkwork-im-release-version.mjs';

const INSTALL_PACKAGE_SCHEMA_VERSION = '2026-06-04.sdkwork-im.install-packages.v1';
const SUPPORTED_PLATFORMS = Object.freeze(['web', 'linux', 'macos', 'windows']);
const SUPPORTED_ARCHITECTURES = Object.freeze(['universal', 'x64', 'arm64']);
const SUPPORTED_PACKAGE_PROFILES = Object.freeze(['browser', 'server', 'desktop']);
const SERVER_AND_DESKTOP_PLATFORMS = Object.freeze(['linux', 'macos', 'windows']);
const SERVER_AND_DESKTOP_ARCHITECTURES = Object.freeze(['x64', 'arm64']);
const APP_CODE = 'chat';
const PRODUCT_NAME = 'chat';
const PACKAGE_NAME = 'sdkwork-chat';
const RUNTIME_DISPLAY_NAME = 'Sdkwork IM';
const SERVER_BINARY_BASENAME = 'sdkwork-api-im-standalone-gateway';
const LINUX_INSTALL_ROOT = '/opt/sdkwork/chat';
const MACOS_INSTALL_ROOT = '/usr/lib/sdkwork/chat';
const POSIX_INSTALL_ROOT = LINUX_INSTALL_ROOT;
const WINDOWS_INSTALL_ROOT = '%ProgramFiles%/sdkwork/chat';
const HEALTH_CHECKS = Object.freeze(['/healthz', '/readyz']);
const SERVER_RUNTIME_PATHS = Object.freeze({
  linux: Object.freeze({
    installRoot: LINUX_INSTALL_ROOT,
    configDir: '/etc/sdkwork/chat',
    dataDir: '/var/lib/sdkwork/chat',
    logDir: '/var/log/sdkwork/chat',
    runDir: '/run/sdkwork/chat',
  }),
  macos: Object.freeze({
    installRoot: MACOS_INSTALL_ROOT,
    configDir: '/Library/Application Support/sdkwork/chat',
    dataDir: '/Library/Application Support/sdkwork/chat/Data',
    logDir: '/Library/Logs/sdkwork/chat',
    runDir: '/Library/Application Support/sdkwork/chat/Run',
  }),
  windows: Object.freeze({
    installRoot: WINDOWS_INSTALL_ROOT,
    configDir: '%ProgramData%/sdkwork/chat',
    dataDir: '%ProgramData%/sdkwork/chat/Data',
    logDir: '%ProgramData%/sdkwork/chat/Logs',
    runDir: '%ProgramData%/sdkwork/chat/Run',
  }),
});

function printHelp() {
  console.log(`Usage: node scripts/release/plan-sdkwork-im-install-packages.mjs [options]

Create and validate the Sdkwork IM cross-platform release package plan.

Options:
  --check             Validate the generated plan and exit nonzero on issues.
  --json              Print machine-readable JSON.
  --version <value>   Package version (default ${DEFAULT_RELEASE_VERSION}).
  --platform <value>  Platform subset: all, web, linux, macos, windows.
  --architecture <v>  Architecture subset: all, universal, x64, arm64.
  --profile <value>   Package profile subset: all, browser, server, desktop.
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
    profiles: SUPPORTED_PACKAGE_PROFILES,
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
        settings.profiles = parseSelectionFlag(requireValue(argv, index, arg), SUPPORTED_PACKAGE_PROFILES);
        index += 1;
        break;
      case '--help':
      case '-h':
        settings.help = true;
        break;
      default:
        throw new Error(`Unsupported release package planner option: ${arg}`);
    }
  }

  return settings;
}

function createSdkworkImInstallPackagePlan({
  version = DEFAULT_RELEASE_VERSION,
  platforms = SUPPORTED_PLATFORMS,
  architectures = SUPPORTED_ARCHITECTURES,
  profiles = SUPPORTED_PACKAGE_PROFILES,
} = {}) {
  const normalizedVersion = normalizeSdkworkImReleaseVersion(version);
  const selectedPlatforms = validateSelection('platforms', platforms, SUPPORTED_PLATFORMS);
  const selectedArchitectures = validateSelection('architectures', architectures, SUPPORTED_ARCHITECTURES);
  const selectedProfiles = validateSelection('profiles', profiles, SUPPORTED_PACKAGE_PROFILES);

  const packages = expectedPackageDefinitions({
    architectures: selectedArchitectures,
    platforms: selectedPlatforms,
    profiles: selectedProfiles,
  }).map((packageDefinition) => createSdkworkImInstallPackageItem({
    ...packageDefinition,
    version: normalizedVersion,
  }));

  return {
    schemaVersion: INSTALL_PACKAGE_SCHEMA_VERSION,
    appCode: APP_CODE,
    product: PRODUCT_NAME,
    packageName: PACKAGE_NAME,
    runtimeName: APP_CODE,
    displayName: RUNTIME_DISPLAY_NAME,
    version: normalizedVersion,
    platforms: selectedPlatforms,
    architectures: selectedArchitectures,
    deploymentProfiles: uniqueInOrder(packages.map((packageItem) => packageItem.deploymentProfile)),
    profiles: selectedProfiles,
    fastInitializationContract: [
      'host-env-prepare',
      'runtime-config-copy',
      'database-configure',
      'server-readiness',
    ],
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

function createSdkworkImInstallPackageItem({
  architecture,
  deploymentProfile,
  platform,
  profile,
  runtimeTarget,
  version,
}) {
  const format = packageFormatFor({ platform, profile });
  const formatToken = format.replaceAll('.', '-');
  const id = `${platform}-${architecture}-${deploymentProfile}-${profile}-${formatToken}`;
  const artifactId = `${platform}-${architecture}-${deploymentProfile}-${profile}`;
  const exeSuffix = platform === 'windows' ? '.exe' : '';
  const binaryName = `${SERVER_BINARY_BASENAME}${exeSuffix}`;
  const runtimePaths = profile === 'server' ? serverRuntimePathsFor(platform) : null;
  return {
    id,
    artifactId,
    version,
    platform,
    architecture,
    deploymentProfile,
    profile,
    runtimeTarget,
    format,
    archiveName: `sdkwork-im-${id}-${version}.${format}`,
    packageKind: packageKindFor(profile),
    binaryName: profile === 'server' ? binaryName : null,
    startCommand: profile !== 'server'
      ? null
      : platform === 'windows'
        ? '.\\bin\\start-server.ps1 -Release'
        : './bin/start-server.sh --release',
    healthChecks: profile === 'browser' ? [] : [...HEALTH_CHECKS],
    artifacts: artifactsFor({ binaryName, profile }),
    databasePolicy: profile === 'browser' ? null : databasePolicyFor({ platform, runtimeTarget }),
    runtimePaths,
    serviceIntegration: profile === 'server' ? serviceIntegrationFor(platform) : null,
    security: {
      noSecretsInPackage: true,
      envLocalGeneratedOnHost: true,
      envExampleReferenceOnly: true,
      excludesRuntimeState: true,
      sameOriginBrowserApiDefaults: true,
    },
  };
}

function packageFormatFor({ platform, profile }) {
  if (profile === 'browser' || profile === 'desktop') {
    return 'zip';
  }
  return platform === 'windows' ? 'zip' : 'tar.gz';
}

function packageKindFor(profile) {
  if (profile === 'browser') {
    return 'browser-web-bundle';
  }
  return profile === 'desktop' ? 'desktop-installer-bundle' : 'server-runtime-archive';
}

function artifactsFor({ binaryName, profile }) {
  if (profile === 'browser') {
    return buildBrowserArtifacts();
  }
  return profile === 'desktop' ? buildDesktopArtifacts() : buildServerArchiveArtifacts(binaryName);
}

function buildBrowserArtifacts() {
  return [
    {
      kind: 'pc-web-dist',
      path: 'web/sdkwork-im-pc/dist',
      source: 'apps/sdkwork-im-pc/dist',
      required: true,
    },
    {
      kind: 'h5-web-dist',
      path: 'web/sdkwork-im-h5/dist',
      source: 'apps/sdkwork-im-h5/dist',
      required: false,
    },
    {
      kind: 'web-manifest',
      path: 'web-manifest.json',
      source: 'generated by release staging',
      required: true,
    },
  ];
}

function buildServerArchiveArtifacts(binaryName) {
  return [
    {
      kind: 'server-binary',
      path: `bin/${binaryName}`,
      source: 'target/release',
      required: true,
    },
    {
      kind: 'server-lifecycle-scripts',
      path: 'bin',
      source: 'bin/*server* plus shared runtime helpers',
      required: true,
    },
    {
      kind: 'server-config-template',
      path: 'config/chat.toml.example',
      source: 'deployments/templates/chat.toml.example',
      required: true,
    },
    {
      kind: 'server-env-template',
      path: 'config/server.env.example',
      source: 'deployments/templates/server.env.example',
      required: true,
    },
    {
      kind: 'postgresql-config-template',
      path: 'config/postgresql.yaml.example',
      source: 'deployments/templates/postgresql.yaml.example',
      required: true,
    },
    {
      kind: 'pc-web-dist',
      path: 'web/sdkwork-im-pc/dist',
      source: 'apps/sdkwork-im-pc/dist',
      required: true,
    },
    {
      kind: 'h5-web-dist',
      path: 'web/sdkwork-im-h5/dist',
      source: 'apps/sdkwork-im-h5/dist',
      required: false,
    },
    {
      kind: 'service-templates',
      path: 'service',
      source: 'deployments/systemd, deployments/launchd, deployments/windows-service',
      required: true,
    },
    {
      kind: 'install-guide',
      path: 'INSTALL.md',
      source: 'generated by release staging',
      required: true,
    },
    {
      kind: 'install-manifest',
      path: 'install-manifest.json',
      source: 'generated by release staging',
      required: true,
    },
  ];
}

function buildDesktopArtifacts() {
  return [
    {
      kind: 'desktop-installers',
      path: 'desktop',
      source: 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop/src-tauri/target/release/bundle',
      required: true,
    },
    {
      kind: 'desktop-manifest',
      path: 'desktop-manifest.json',
      source: 'generated by release staging',
      required: true,
    },
  ];
}

function databasePolicyFor({ platform, runtimeTarget }) {
  const locations = runtimeConfigLocationsFor(platform, runtimeTarget);
  return {
    defaultEngine: 'postgresql',
    defaultHost: 'db.example.com',
    defaultPort: 5432,
    defaultDatabase: 'sdkwork',
    defaultUsername: 'sdkwork',
    passwordFile: {
      path: postgresPasswordFileFor(platform, runtimeTarget),
      required: true,
    },
    requiresExternalDatabase: true,
    configFile: {
      path: locations.configFile,
    },
    dataDirectory: {
      path: locations.dataDirectory,
    },
    envOverrides: [
      'SDKWORK_IM_CONFIG_FILE',
        'SDKWORK_IM_DATA_DIR',
        'SDKWORK_IM_LOG_DIR',
        'SDKWORK_IM_CACHE_DIR',
        'SDKWORK_IM_RUN_DIR',
        'SDKWORK_IM_ID_NODE_ID',
      'SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND',
      'SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL',
      'SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL',
      'SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL',
      'SDKWORK_DATABASE_ENGINE',
      'SDKWORK_DATABASE_HOST',
      'SDKWORK_DATABASE_PORT',
      'SDKWORK_DATABASE_NAME',
      'SDKWORK_DATABASE_SCHEMA',
      'SDKWORK_DATABASE_USERNAME',
      'SDKWORK_DATABASE_PASSWORD_FILE',
      'SDKWORK_DATABASE_SSL_MODE',
      'SDKWORK_DATABASE_MAX_CONNECTIONS',
      'SDKWORK_IM_REDIS_ENABLED',
      'SDKWORK_IM_REDIS_HOST',
      'SDKWORK_IM_REDIS_PORT',
      'SDKWORK_IM_REDIS_DATABASE',
      'SDKWORK_IM_REDIS_PASSWORD_FILE',
      'SDKWORK_IM_REDIS_KEY_PREFIX',
      'SDKWORK_IM_REDIS_TLS',
      'SDKWORK_IM_REDIS_MAX_CONNECTIONS',
    ],
  };
}

function runtimeConfigLocationsFor(platform, runtimeTarget) {
  if (runtimeTarget === 'desktop') {
    if (platform === 'windows') {
      return {
        configFile: '%USERPROFILE%/.sdkwork/chat/config/chat.toml',
        dataDirectory: '%USERPROFILE%/.sdkwork/chat/data',
      };
    }
    if (platform === 'macos') {
      return {
        configFile: '~/.sdkwork/chat/config/chat.toml',
        dataDirectory: '~/.sdkwork/chat/data',
      };
    }
    return {
      configFile: '~/.sdkwork/chat/config/chat.toml',
      dataDirectory: '~/.sdkwork/chat/data',
    };
  }

  const paths = serverRuntimePathsFor(platform);
  return {
    configFile: `${paths.configDir}/chat.toml`,
    dataDirectory: paths.dataDir,
  };
}

function postgresPasswordFileFor(platform, runtimeTarget = 'server') {
  const locations = runtimeConfigLocationsFor(platform, runtimeTarget);
  return `${configDirectoryFromConfigFile(locations.configFile)}/database.secret`;
}

function configDirectoryFromConfigFile(configFile) {
  const normalized = String(configFile ?? '').replaceAll('\\', '/');
  const separatorIndex = normalized.lastIndexOf('/');
  if (separatorIndex <= 0) {
    throw new Error(`config file must include a directory: ${configFile}`);
  }
  return normalized.slice(0, separatorIndex);
}

function serverRuntimePathsFor(platform) {
  const paths = SERVER_RUNTIME_PATHS[platform];
  if (!paths) {
    throw new Error(`Unsupported server runtime platform: ${platform}`);
  }
  return { ...paths };
}

function serviceIntegrationFor(platform) {
  if (platform === 'windows') {
    return {
      kind: 'windows-service',
      manifest: 'service/windows/sdkwork-api-im-standalone-gateway-service.xml',
    };
  }
  if (platform === 'macos') {
    return {
      kind: 'launchd',
      manifest: 'service/macos/com.sdkwork.im.api-standalone-gateway.plist',
    };
  }
  return {
    kind: 'systemd',
    manifest: 'service/linux/sdkwork-api-im-standalone-gateway.service',
  };
}

function validateSdkworkImInstallPackagePlan(plan) {
  const issues = [];
  if (plan.schemaVersion !== INSTALL_PACKAGE_SCHEMA_VERSION) {
    issues.push(`schemaVersion must be ${INSTALL_PACKAGE_SCHEMA_VERSION}`);
  }
  if (plan.product !== PRODUCT_NAME) {
    issues.push(`product must be ${PRODUCT_NAME}`);
  }
  if (plan.packageName !== PACKAGE_NAME) {
    issues.push(`packageName must be ${PACKAGE_NAME}`);
  }
  if (plan.appCode !== APP_CODE || plan.runtimeName !== APP_CODE) {
    issues.push(`appCode and runtimeName must be ${APP_CODE}`);
  }
  if (!/^[0-9A-Za-z][0-9A-Za-z._-]*$/u.test(String(plan.version ?? ''))) {
    issues.push('version must be package-safe');
  }
  validateSubset('platforms', plan.platforms, SUPPORTED_PLATFORMS, issues);
  validateSubset('architectures', plan.architectures, SUPPORTED_ARCHITECTURES, issues);
  validateSubset('profiles', plan.profiles, SUPPORTED_PACKAGE_PROFILES, issues);
  const expectedDeploymentProfiles = uniqueInOrder((plan.packages ?? []).map((item) => item.deploymentProfile));
  validateArrayMatches('deployment profiles', plan.deploymentProfiles, expectedDeploymentProfiles, issues);
  for (const [field, expected] of Object.entries({
    noSecretsInPackage: true,
    envLocalGeneratedOnHost: true,
    envExampleReferenceOnly: true,
    releaseEnvLocalExcluded: true,
    generatedFromProductionBuild: true,
  })) {
    if (plan.artifactPolicy?.[field] !== expected) {
      issues.push(`artifactPolicy.${field} must be ${expected}`);
    }
  }

  const expectedIds = expectedPackageDefinitions({
    architectures: plan.architectures ?? [],
    platforms: plan.platforms ?? [],
    profiles: plan.profiles ?? [],
  }).map((packageDefinition) => {
    const formatToken = packageFormatFor(packageDefinition).replaceAll('.', '-');
    return `${packageDefinition.platform}-${packageDefinition.architecture}-${packageDefinition.deploymentProfile}-${packageDefinition.profile}-${formatToken}`;
  });
  if (!Array.isArray(plan.packages)) {
    issues.push('packages must be an array');
    return issues;
  }
  if (plan.packages.length === 0) {
    issues.push('packages must contain at least one package target');
  }
  validateArrayMatches('package ids', plan.packages.map((item) => item.id), expectedIds, issues);

  const seenIds = new Set();
  for (const packageItem of plan.packages) {
    validatePackageItem(packageItem, seenIds, issues);
  }
  return issues;
}

function validatePackageItem(packageItem, seenIds, issues) {
  const expectedFormat = packageFormatFor({
    platform: packageItem.platform,
    profile: packageItem.profile,
  });
  const expectedDeploymentProfile = deploymentProfileFor(packageItem.profile);
  const expectedRuntimeTarget = runtimeTargetFor(packageItem.profile);
  const expectedId = `${packageItem.platform}-${packageItem.architecture}-${expectedDeploymentProfile}-${packageItem.profile}-${expectedFormat.replaceAll('.', '-')}`;
  if (packageItem.id !== expectedId) {
    issues.push(`${packageItem.id ?? '(missing id)'} id must be ${expectedId}`);
  }
  if (seenIds.has(packageItem.id)) {
    issues.push(`${packageItem.id} is duplicated`);
  }
  seenIds.add(packageItem.id);
  if (!SUPPORTED_PLATFORMS.includes(packageItem.platform)) {
    issues.push(`${packageItem.id} has unsupported platform`);
  }
  if (!SUPPORTED_ARCHITECTURES.includes(packageItem.architecture)) {
    issues.push(`${packageItem.id} has unsupported architecture`);
  }
  if (packageItem.deploymentProfile !== expectedDeploymentProfile) {
    issues.push(`${packageItem.id} deploymentProfile must be ${expectedDeploymentProfile}`);
  }
  if (!SUPPORTED_PACKAGE_PROFILES.includes(packageItem.profile)) {
    issues.push(`${packageItem.id} has unsupported package profile`);
  }
  if (!isSupportedPackageCombination(packageItem)) {
    issues.push(`${packageItem.id} has an unsupported platform, architecture, and profile combination`);
  }
  if (packageItem.runtimeTarget !== expectedRuntimeTarget) {
    issues.push(`${packageItem.id} runtimeTarget must be ${expectedRuntimeTarget}`);
  }
  if (packageItem.format !== expectedFormat) {
    issues.push(`${packageItem.id} format must be ${expectedFormat}`);
  }
  if (!String(packageItem.archiveName ?? '').endsWith(`.${expectedFormat}`)) {
    issues.push(`${packageItem.id} archiveName must end with .${expectedFormat}`);
  }
  if (packageItem.security?.noSecretsInPackage !== true) {
    issues.push(`${packageItem.id} security.noSecretsInPackage must be true`);
  }
  const expectedHealthChecks = packageItem.profile === 'browser' ? [] : HEALTH_CHECKS;
  if (!Array.isArray(packageItem.healthChecks) || !arraysEqual(packageItem.healthChecks, expectedHealthChecks)) {
    issues.push(`${packageItem.id} healthChecks must be ${HEALTH_CHECKS.join(', ')}`);
  }
  if (packageItem.profile !== 'browser' && !packageItem.databasePolicy?.configFile?.path) {
    issues.push(`${packageItem.id} databasePolicy must declare configFile.path`);
  }
  if (packageItem.profile !== 'browser' && !packageItem.databasePolicy?.dataDirectory?.path) {
    issues.push(`${packageItem.id} databasePolicy must declare dataDirectory.path`);
  }

  if (packageItem.profile === 'browser') {
    if (packageItem.databasePolicy !== null) {
      issues.push(`${packageItem.id} browser packages must not declare a databasePolicy`);
    }
    for (const artifactKind of ['pc-web-dist', 'web-manifest']) {
      if (!packageItem.artifacts?.some((artifact) => artifact.kind === artifactKind && artifact.required === true)) {
        issues.push(`${packageItem.id} must include ${artifactKind}`);
      }
    }
  } else if (packageItem.profile === 'desktop') {
    if (packageItem.databasePolicy?.defaultEngine !== 'postgresql') {
      issues.push(`${packageItem.id} desktop packages must default to PostgreSQL until IM durable SQLite is implemented`);
    }
    if (packageItem.databasePolicy?.requiresExternalDatabase !== true) {
      issues.push(`${packageItem.id} desktop packages must require PostgreSQL until IM durable SQLite is implemented`);
    }
    if (packageItem.databasePolicy?.passwordFile?.required !== true) {
      issues.push(`${packageItem.id} desktop packages must declare required database password file`);
    }
    for (const artifactKind of ['desktop-installers', 'desktop-manifest']) {
      if (!packageItem.artifacts?.some((artifact) => artifact.kind === artifactKind && artifact.required === true)) {
        issues.push(`${packageItem.id} must include ${artifactKind}`);
      }
    }
  } else {
    if (packageItem.databasePolicy?.defaultEngine !== 'postgresql') {
      issues.push(`${packageItem.id} server packages must default to PostgreSQL`);
    }
    if (packageItem.databasePolicy?.requiresExternalDatabase !== true) {
      issues.push(`${packageItem.id} server packages must require an external database`);
    }
    if (!packageItem.binaryName) {
      issues.push(`${packageItem.id} server packages must include binaryName`);
    }
    if (!packageItem.serviceIntegration?.kind) {
      issues.push(`${packageItem.id} server packages must declare service integration metadata`);
    }
    for (const field of ['installRoot', 'configDir', 'dataDir', 'logDir', 'runDir']) {
      if (!packageItem.runtimePaths?.[field]) {
        issues.push(`${packageItem.id} server packages must declare runtimePaths.${field}`);
      }
    }
    for (const artifactKind of [
      'server-binary',
      'server-lifecycle-scripts',
      'server-config-template',
      'server-env-template',
      'postgresql-config-template',
      'pc-web-dist',
      'service-templates',
      'install-guide',
      'install-manifest',
    ]) {
      if (!packageItem.artifacts?.some((artifact) => artifact.kind === artifactKind && artifact.required === true)) {
        issues.push(`${packageItem.id} must include ${artifactKind}`);
      }
    }
  }
}

function renderSdkworkImInstallPackagePlan(plan) {
  return [
    `[sdkwork-im-install-packages] product: ${plan.product}`,
    `[sdkwork-im-install-packages] schema: ${plan.schemaVersion}`,
    `[sdkwork-im-install-packages] version: ${plan.version}`,
    `[sdkwork-im-install-packages] supported platforms: ${plan.platforms.join(', ')}`,
    `[sdkwork-im-install-packages] supported architectures: ${plan.architectures.join(', ')}`,
    `[sdkwork-im-install-packages] deployment profiles: ${plan.deploymentProfiles.join(', ')}`,
    `[sdkwork-im-install-packages] package profiles: ${plan.profiles.join(', ')}`,
    `[sdkwork-im-install-packages] packages: ${plan.packages.length}`,
    ...plan.packages.map((packageItem) => [
      `[sdkwork-im-install-packages]   ${packageItem.id}`,
      `archive=${packageItem.archiveName}`,
      `kind=${packageItem.packageKind}`,
      `profile=${packageItem.profile}`,
      `runtimeTarget=${packageItem.runtimeTarget}`,
      `database=${packageItem.databasePolicy?.defaultEngine ?? 'none'}`,
      renderRuntimePathsSummary(packageItem.runtimePaths),
    ].join(' ')),
  ];
}

function expectedPackageDefinitions({ architectures, platforms, profiles }) {
  const definitions = [];
  if (profiles.includes('browser') && platforms.includes('web') && architectures.includes('universal')) {
    definitions.push({
      architecture: 'universal',
      deploymentProfile: 'cloud',
      platform: 'web',
      profile: 'browser',
      runtimeTarget: 'browser',
    });
  }

  for (const profile of profiles.filter((value) => value === 'server' || value === 'desktop')) {
    for (const platform of SERVER_AND_DESKTOP_PLATFORMS.filter((value) => platforms.includes(value))) {
      for (const architecture of SERVER_AND_DESKTOP_ARCHITECTURES.filter((value) => architectures.includes(value))) {
        definitions.push({
          architecture,
          deploymentProfile: deploymentProfileFor(profile),
          platform,
          profile,
          runtimeTarget: runtimeTargetFor(profile),
        });
      }
    }
  }

  return definitions;
}

function deploymentProfileFor(profile) {
  return profile === 'browser' ? 'cloud' : 'standalone';
}

function runtimeTargetFor(profile) {
  if (!SUPPORTED_PACKAGE_PROFILES.includes(profile)) {
    return null;
  }
  return profile;
}

function isSupportedPackageCombination(packageItem) {
  if (packageItem.profile === 'browser') {
    return packageItem.platform === 'web' && packageItem.architecture === 'universal';
  }
  if (packageItem.profile === 'server' || packageItem.profile === 'desktop') {
    return SERVER_AND_DESKTOP_PLATFORMS.includes(packageItem.platform)
      && SERVER_AND_DESKTOP_ARCHITECTURES.includes(packageItem.architecture);
  }
  return false;
}

function uniqueInOrder(values) {
  return [...new Set(values.filter((value) => typeof value === 'string' && value.length > 0))];
}

function renderRuntimePathsSummary(runtimePaths) {
  if (!runtimePaths) {
    return '';
  }
  return [
    `paths=install=${runtimePaths.installRoot}`,
    `config=${runtimePaths.configDir}`,
    `data=${runtimePaths.dataDir}`,
    `log=${runtimePaths.logDir}`,
    `run=${runtimePaths.runDir}`,
  ].join(' ');
}

async function main(argv = process.argv.slice(2)) {
  const settings = parseArgs(argv);
  if (settings.help) {
    printHelp();
    return 0;
  }

  const plan = createSdkworkImInstallPackagePlan({
    architectures: settings.architectures,
    platforms: settings.platforms,
    profiles: settings.profiles,
    version: settings.version,
  });
  const issues = validateSdkworkImInstallPackagePlan(plan);
  if (settings.json) {
    console.log(JSON.stringify({
      ok: issues.length === 0,
      issues,
      plan,
    }, null, 2));
  } else {
    for (const line of renderSdkworkImInstallPackagePlan(plan)) {
      console.log(line);
    }
    if (issues.length > 0) {
      console.error('[sdkwork-im-install-packages] validation issues:');
      for (const issue of issues) {
        console.error(`[sdkwork-im-install-packages]   ${issue}`);
      }
    } else if (settings.check) {
      console.log('[sdkwork-im-install-packages] validation passed');
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
    console.error(`[sdkwork-im-install-packages] ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  });
}

export {
  DEFAULT_RELEASE_VERSION as DEFAULT_VERSION,
  HEALTH_CHECKS,
  INSTALL_PACKAGE_SCHEMA_VERSION,
  LINUX_INSTALL_ROOT,
  MACOS_INSTALL_ROOT,
  PACKAGE_NAME,
  POSIX_INSTALL_ROOT,
  PRODUCT_NAME,
  RUNTIME_DISPLAY_NAME,
  SERVER_BINARY_BASENAME,
  SERVER_RUNTIME_PATHS,
  SUPPORTED_ARCHITECTURES,
  SUPPORTED_PACKAGE_PROFILES,
  SUPPORTED_PLATFORMS,
  WINDOWS_INSTALL_ROOT,
  createSdkworkImInstallPackagePlan,
  databasePolicyFor,
  main,
  parseArgs,
  postgresPasswordFileFor,
  renderSdkworkImInstallPackagePlan,
  runtimeConfigLocationsFor,
  serverRuntimePathsFor,
  validateSdkworkImInstallPackagePlan,
};
