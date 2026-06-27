#!/usr/bin/env node

import { createHash } from 'node:crypto';
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import { copyFile, mkdir, rm, writeFile } from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { collectSdkworkImDesktopBundles } from './collect-sdkwork-im-desktop-bundles.mjs';
import { DEFAULT_RELEASE_VERSION } from './sdkwork-im-release-version.mjs';
import {
  createSdkworkImInstallPackagePlan,
  serverRuntimePathsFor,
  validateSdkworkImInstallPackagePlan,
} from './plan-sdkwork-im-install-packages.mjs';

const __filename = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(__filename), '..', '..');
const STAGING_SCHEMA_VERSION = '2026-06-04.sdkwork-im.release-staging.v1';
const SERVER_SCRIPT_PATTERNS = Object.freeze([
  /^_cmd-forward-powershell\.cmd$/u,
  /^install-server\.(ps1|sh|cmd)$/u,
  /^init-config-server\.(ps1|sh|cmd)$/u,
  /^init-storage-server\.(ps1|sh|cmd)$/u,
  /^verify-server\.(ps1|sh|cmd)$/u,
  /^install-service-server\.(ps1|sh|cmd)$/u,
  /^uninstall-service-server\.(ps1|sh|cmd)$/u,
  /^start-server\.(ps1|sh|cmd)$/u,
  /^stop-server\.(ps1|sh|cmd)$/u,
  /^restart-server\.(ps1|sh|cmd)$/u,
  /^status-server\.(ps1|sh|cmd)$/u,
]);

function printHelp() {
  console.log(`Usage: node scripts/release/stage-sdkwork-im-release-package.mjs [options]

Stage one Sdkwork IM server archive or desktop bundle package from production build outputs.

Options:
  --package-id <id>       Package id from the release package plan.
  --all                   Stage all package ids.
  --staging-root <dir>    Staging output root (default dist/release-staging/<package-id>).
  --version <value>       Package version (default ${DEFAULT_RELEASE_VERSION}).
  --check                 Validate the staging plan and staged output.
  --dry-run               Print the staging plan without writing files.
  --json                  Print machine-readable JSON.
  -h, --help              Show this help.
`);
}

function requireValue(argv, index, flag) {
  const value = argv[index + 1];
  if (!value || value.startsWith('--')) {
    throw new Error(`${flag} requires a value`);
  }
  return value;
}

function parseStagingArgs(argv = process.argv.slice(2)) {
  const settings = {
    all: false,
    check: false,
    dryRun: false,
    help: false,
    json: false,
    packageId: currentHostServerPackageId(),
    stagingRoot: null,
    version: DEFAULT_RELEASE_VERSION,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === '--') {
      continue;
    }
    switch (arg) {
      case '--all':
        settings.all = true;
        break;
      case '--package-id':
        settings.packageId = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--staging-root':
        settings.stagingRoot = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--version':
        settings.version = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--check':
        settings.check = true;
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
        throw new Error(`Unsupported release staging option: ${arg}`);
    }
  }

  return settings;
}

function createSdkworkImReleaseStagingPlan({
  packageId = currentHostServerPackageId(),
  root = repoRoot,
  stagingRoot = null,
  version = DEFAULT_RELEASE_VERSION,
} = {}) {
  const installPlan = createSdkworkImInstallPackagePlan({ version });
  const planIssues = validateSdkworkImInstallPackagePlan(installPlan);
  if (planIssues.length > 0) {
    throw new Error(`release package plan is invalid: ${planIssues.join('; ')}`);
  }
  const packageItem = installPlan.packages.find((item) => item.id === packageId);
  if (!packageItem) {
    throw new Error(`Unknown release package id: ${packageId}`);
  }
  const absoluteStagingRoot = path.resolve(root, stagingRoot ?? path.join('dist', 'release-staging', packageId));
  const actions = packageItem.deploymentMode === 'desktop'
    ? createDesktopStagingActions({ packageItem, root, stagingRoot: absoluteStagingRoot })
    : createServerStagingActions({ packageItem, root, stagingRoot: absoluteStagingRoot });

  return {
    schemaVersion: STAGING_SCHEMA_VERSION,
    package: packageItem,
    root,
    stagingRoot: absoluteStagingRoot,
    actions,
  };
}

function createServerStagingActions({ packageItem, root, stagingRoot }) {
  const binarySource = path.join(root, 'target', 'release', packageItem.binaryName);
  const actions = [
    copyAction(binarySource, path.join(stagingRoot, 'bin', packageItem.binaryName), 'server release binary', true, stagingRoot),
  ];

  for (const fileName of serverScriptFileNames(path.join(root, 'bin'))) {
    actions.push(copyAction(
      path.join(root, 'bin', fileName),
      path.join(stagingRoot, 'bin', fileName),
      'server lifecycle script',
      true,
      stagingRoot,
    ));
  }

  actions.push(
    copyAction(
      path.join(root, 'deployments', 'templates', 'chat.toml.example'),
      path.join(stagingRoot, 'config', 'chat.toml.example'),
      'server config template',
      true,
      stagingRoot,
    ),
    generatedAction(
      path.join(stagingRoot, 'config', 'server.env.example'),
      'server env template',
      () => createServerEnvExample(packageItem),
      true,
      stagingRoot,
    ),
    copyAction(
      path.join(root, 'deployments', 'templates', 'postgresql.yaml.example'),
      path.join(stagingRoot, 'config', 'postgresql.yaml.example'),
      'postgresql config template',
      true,
      stagingRoot,
    ),
    copyAction(
      path.join(root, 'deployments', 'systemd', 'sdkwork-im-server.service'),
      path.join(stagingRoot, 'service', 'linux', 'sdkwork-im-server.service'),
      'linux service template',
      true,
      stagingRoot,
    ),
    copyAction(
      path.join(root, 'deployments', 'launchd', 'com.sdkwork.im.server.plist'),
      path.join(stagingRoot, 'service', 'macos', 'com.sdkwork.im.server.plist'),
      'macos service template',
      true,
      stagingRoot,
    ),
    copyAction(
      path.join(root, 'deployments', 'windows-service', 'SdkworkImServer.xml'),
      path.join(stagingRoot, 'service', 'windows', 'SdkworkImServer.xml'),
      'windows service template',
      true,
      stagingRoot,
    ),
    copyAction(
      path.join(root, 'apps', 'sdkwork-im-pc', 'dist'),
      path.join(stagingRoot, 'web', 'sdkwork-im-pc', 'dist'),
      'sdkwork-im-pc web dist',
      true,
      stagingRoot,
    ),
    generatedAction(
      path.join(stagingRoot, 'INSTALL.md'),
      'install guide',
      () => createInstallGuide(packageItem),
      true,
      stagingRoot,
    ),
    generatedAction(
      path.join(stagingRoot, 'install-manifest.json'),
      'install manifest',
      () => JSON.stringify(createInstallManifest(packageItem), null, 2) + '\n',
      true,
      stagingRoot,
    ),
  );

  return actions;
}

function createDesktopStagingActions({ packageItem, root, stagingRoot }) {
  const desktopManifest = collectSdkworkImDesktopBundles({
    arch: packageItem.architecture,
    platform: packageItem.platform,
    root,
    version: packageItem.version,
  });
  const actions = desktopManifest.files.map((file) => copyAction(
    file.sourcePath,
    path.join(stagingRoot, 'desktop', file.path),
    'desktop installer artifact',
    true,
    stagingRoot,
  ));
  actions.push(generatedAction(
    path.join(stagingRoot, 'desktop-manifest.json'),
    'desktop manifest',
    () => JSON.stringify({
      schemaVersion: desktopManifest.schemaVersion,
      product: desktopManifest.product,
      version: desktopManifest.version,
      platform: desktopManifest.platform,
      architecture: desktopManifest.architecture,
      files: desktopManifest.files.map((file) => ({
        path: `desktop/${file.path}`,
        size: file.size,
        sha256: file.sha256,
      })),
    }, null, 2) + '\n',
    true,
    stagingRoot,
  ));
  return actions;
}

function serverScriptFileNames(binRoot) {
  if (!existsSync(binRoot)) {
    return [];
  }
  return readdirSync(binRoot)
    .filter((fileName) => SERVER_SCRIPT_PATTERNS.some((pattern) => pattern.test(fileName)))
    .sort();
}

function copyAction(sourcePath, targetPath, label, required, stagingRoot) {
  return {
    kind: 'copy',
    label,
    required,
    sourcePath,
    targetPath,
    archivePath: normalizeArchivePath(path.relative(stagingRoot, targetPath)),
  };
}

function generatedAction(targetPath, label, contentFactory, required, stagingRoot) {
  return {
    kind: 'generate',
    label,
    required,
    targetPath,
    archivePath: normalizeArchivePath(path.relative(stagingRoot, targetPath)),
    contentFactory,
  };
}

function validateSdkworkImReleaseStagingPlan(plan, { requireSources = true } = {}) {
  const issues = [];
  if (plan.schemaVersion !== STAGING_SCHEMA_VERSION) {
    issues.push(`schemaVersion must be ${STAGING_SCHEMA_VERSION}`);
  }
  if (!plan.package?.id) {
    issues.push('package id is required');
  }
  if (!Array.isArray(plan.actions) || plan.actions.length === 0) {
    issues.push(`${plan.package?.id ?? '(unknown)'} must include staging actions`);
    return issues;
  }
  if (!isPathInside(plan.stagingRoot, plan.root) || samePath(plan.stagingRoot, plan.root)) {
    issues.push(`${plan.package.id} staging root must stay inside the repository root and must not be the repository root itself`);
  }
  for (const action of plan.actions) {
    const relativeTarget = normalizeArchivePath(path.relative(plan.stagingRoot, action.targetPath));
    if (isSensitiveArchivePath(relativeTarget)) {
      issues.push(`${plan.package.id} must not stage sensitive path ${relativeTarget}`);
    }
    if (!isPathInside(action.targetPath, plan.stagingRoot) && !samePath(action.targetPath, plan.stagingRoot)) {
      issues.push(`${plan.package.id} target path escapes staging root: ${action.targetPath}`);
    }
    if (action.kind === 'copy' && requireSources && action.required && !existsSync(action.sourcePath)) {
      issues.push(`${plan.package.id} requires source ${action.sourcePath}`);
    }
  }
  if (plan.package?.deploymentMode === 'desktop' && requireSources) {
    const desktopCopies = plan.actions.filter((action) => action.kind === 'copy' && action.label === 'desktop installer artifact');
    if (desktopCopies.length === 0) {
      issues.push(`${plan.package.id} requires at least one desktop installer artifact. Run release:build:desktop first.`);
    }
  }
  return issues;
}

async function stageSdkworkImReleasePackage(plan) {
  const issues = validateSdkworkImReleaseStagingPlan(plan, { requireSources: true });
  if (issues.length > 0) {
    throw new Error(`release staging plan is invalid: ${issues.join('; ')}`);
  }

  await rm(plan.stagingRoot, { recursive: true, force: true });
  await mkdir(plan.stagingRoot, { recursive: true });
  for (const action of plan.actions) {
    if (action.kind === 'copy') {
      await copyPath(action.sourcePath, action.targetPath);
    } else if (action.kind === 'generate') {
      await mkdir(path.dirname(action.targetPath), { recursive: true });
      await writeFile(action.targetPath, action.contentFactory(), 'utf8');
    }
  }

  return createStagingResult(plan);
}

function createStagingResult(plan) {
  const files = existsSync(plan.stagingRoot) ? collectStagedFiles(plan.stagingRoot, plan.stagingRoot) : [];
  return {
    packageId: plan.package.id,
    stagingRoot: plan.stagingRoot,
    files,
  };
}

function collectStagedFiles(currentDir, rootDir) {
  const files = [];
  for (const entry of readdirSync(currentDir, { withFileTypes: true }).sort((left, right) => left.name.localeCompare(right.name))) {
    const absolutePath = path.join(currentDir, entry.name);
    const relativePath = normalizeArchivePath(path.relative(rootDir, absolutePath));
    if (isSensitiveArchivePath(relativePath)) {
      continue;
    }
    if (entry.isDirectory()) {
      files.push(...collectStagedFiles(absolutePath, rootDir));
      continue;
    }
    if (!entry.isFile()) {
      continue;
    }
    const stat = statSync(absolutePath);
    files.push({
      path: relativePath,
      size: stat.size,
      sha256: sha256File(absolutePath),
    });
  }
  return files;
}

async function copyPath(sourcePath, targetPath) {
  const info = statSync(sourcePath);
  if (info.isDirectory()) {
    await mkdir(targetPath, { recursive: true });
    for (const child of readdirSync(sourcePath).sort()) {
      if (isSensitiveArchivePath(child)) {
        continue;
      }
      await copyPath(path.join(sourcePath, child), path.join(targetPath, child));
    }
    return;
  }
  if (!info.isFile()) {
    return;
  }
  await mkdir(path.dirname(targetPath), { recursive: true });
  await copyFile(sourcePath, targetPath);
}

function createServerEnvExample(packageItem) {
  const paths = serverRuntimePathsFor(packageItem.platform);
  return [
    'SDKWORK_IM_DEPLOYMENT_MODE=server',
    `SDKWORK_IM_CONFIG_FILE=${paths.configDir}/chat.toml`,
    `SDKWORK_IM_DATA_DIR=${paths.dataDir}`,
    `SDKWORK_IM_LOG_DIR=${paths.logDir}`,
    `SDKWORK_IM_RUN_DIR=${paths.runDir}`,
    'SDKWORK_IM_ID_NODE_ID=1',
    'SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND=0.0.0.0:18080',
    'SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=https://im.sdkwork.com',
    'SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=wss://im.sdkwork.com',
    'SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=https://api.sdkwork.com',
    'SDKWORK_IM_DATABASE_ENGINE=postgresql',
    'SDKWORK_IM_DATABASE_HOST=db.example.com',
    'SDKWORK_IM_DATABASE_PORT=5432',
    'SDKWORK_CLAW_DATABASE_NAME=sdkwork_ai_prod',
    'SDKWORK_CLAW_DATABASE_SCHEMA=sdkwork_ai_prod_chat_prod',
    'SDKWORK_CLAW_DATABASE_USERNAME=sdkwork_ai_prod',
    `SDKWORK_IM_DATABASE_PASSWORD_FILE=${paths.configDir}/database.secret`,
    'SDKWORK_IM_DATABASE_SSL_MODE=require',
    'SDKWORK_IM_DATABASE_MAX_CONNECTIONS=20',
    'SDKWORK_IM_REDIS_ENABLED=true',
    'SDKWORK_IM_REDIS_HOST=redis.example.com',
    'SDKWORK_IM_REDIS_PORT=6379',
    'SDKWORK_IM_REDIS_DATABASE=0',
    `SDKWORK_IM_REDIS_PASSWORD_FILE=${paths.configDir}/redis.secret`,
    'SDKWORK_IM_REDIS_KEY_PREFIX=chat',
    'SDKWORK_IM_REDIS_TLS=false',
    'SDKWORK_IM_REDIS_MAX_CONNECTIONS=16',
    'SDKWORK_IM_BROWSER_ORIGINS=https://im.sdkwork.com',
    `SDKWORK_IM_ADMIN_SITE_DIR=${paths.installRoot}/web/sdkwork-im-pc/dist`,
    `SDKWORK_IM_PORTAL_SITE_DIR=${paths.installRoot}/web/sdkwork-im-pc/dist`,
    '',
  ].join('\n');
}

function createInstallGuide(packageItem) {
  const startCommand = packageItem.platform === 'windows'
    ? '.\\bin\\start-server.ps1 -Release -Foreground'
    : './bin/start-server.sh --release --foreground';
  return [
    '# Sdkwork IM Server Package',
    '',
    `Package: ${packageItem.id}`,
    `Version: ${packageItem.version}`,
    '',
    '## Included Files',
    '',
    '- `bin/sdkwork-im-server` or `bin/sdkwork-im-server.exe`: release server binary.',
    '- `bin/*server.*`: install, config, storage, service, start, stop, restart, and status helpers.',
    '- `config/chat.toml.example`: server runtime config template.',
    '- `config/server.env.example`: environment template with packaged web site directories.',
    '- `config/postgresql.yaml.example`: PostgreSQL template using file-based password storage.',
    '- `service/`: systemd, launchd, and Windows service templates.',
    '- `web/sdkwork-im-pc/dist`: production PC web assets.',
    '',
    '## Quick Start',
    '',
    '1. Copy `config/chat.toml.example` to your runtime `chat.toml`.',
    '2. Copy `config/server.env.example` to your runtime `server.env`.',
    '3. Update PostgreSQL host, database, username, and password file path.',
    '4. Start the server:',
    '',
    `\`\`\`sh\n${startCommand}\n\`\`\``,
    '',
    'Secrets and host-local `.env` files are intentionally not included in this package.',
    '',
  ].join('\n');
}

function createInstallManifest(packageItem) {
  return {
    schemaVersion: '2026-06-04.sdkwork-im.install-manifest.v1',
    generatedAt: manifestTimestamp(),
    product: 'chat',
    package: {
      id: packageItem.id,
      version: packageItem.version,
      platform: packageItem.platform,
      architecture: packageItem.architecture,
      deploymentMode: packageItem.deploymentMode,
      runtimeProfile: packageItem.runtimeProfile,
      archiveName: packageItem.archiveName,
      binaryName: packageItem.binaryName,
      startCommand: packageItem.startCommand,
      healthChecks: packageItem.healthChecks,
    },
    databasePolicy: packageItem.databasePolicy,
    runtimePaths: packageItem.runtimePaths,
    serviceIntegration: packageItem.serviceIntegration,
    security: packageItem.security,
  };
}

function manifestTimestamp({ env = process.env, now = new Date() } = {}) {
  const sourceDateEpoch = String(env.SOURCE_DATE_EPOCH ?? '').trim();
  if (sourceDateEpoch) {
    if (!/^\d+$/u.test(sourceDateEpoch)) {
      throw new Error('SOURCE_DATE_EPOCH must be an integer Unix timestamp in seconds');
    }
    return new Date(Number(sourceDateEpoch) * 1000).toISOString();
  }
  return now.toISOString();
}

function currentHostServerPackageId(platform = process.platform, arch = process.arch) {
  const normalizedPlatform = platform === 'win32' ? 'windows' : platform === 'darwin' ? 'macos' : 'linux';
  const normalizedArch = arch === 'arm64' ? 'arm64' : 'x64';
  return `${normalizedPlatform}-${normalizedArch}-server-archive`;
}

function normalizeArchivePath(value) {
  const normalized = String(value ?? '').replaceAll('\\', '/').replace(/^\/+/u, '');
  if (!normalized || normalized === '.' || normalized.includes('..') || path.isAbsolute(normalized)) {
    throw new Error(`Unsafe archive path: ${value}`);
  }
  return normalized;
}

function isSensitiveArchivePath(value) {
  const normalized = String(value ?? '').replaceAll('\\', '/');
  return /(^|\/)\.env($|\.|\/)|(^|\/)node_modules(\/|$)|(^|\/)\.runtime(\/|$)|(^|\/)secrets?(\/|$)|secret/u.test(normalized);
}

function isPathInside(candidatePath, parentPath) {
  const relative = path.relative(path.resolve(parentPath), path.resolve(candidatePath));
  return Boolean(relative) && !relative.startsWith('..') && !path.isAbsolute(relative);
}

function samePath(left, right) {
  return path.resolve(left) === path.resolve(right);
}

function sha256File(filePath) {
  return createHash('sha256').update(readFileSync(filePath)).digest('hex');
}

async function main(argv = process.argv.slice(2)) {
  const settings = parseStagingArgs(argv);
  if (settings.help) {
    printHelp();
    return 0;
  }
  if (settings.all) {
    return await runAllStaging(settings);
  }

  const plan = createSdkworkImReleaseStagingPlan(settings);
  const issues = validateSdkworkImReleaseStagingPlan(plan, { requireSources: !settings.dryRun });
  const payload = {
    ok: issues.length === 0,
    issues,
    dryRun: settings.dryRun,
    plan: serializablePlan(plan),
  };
  if (settings.json) {
    console.log(JSON.stringify(payload, null, 2));
  } else {
    for (const line of renderStagingPlan(plan)) {
      console.log(line);
    }
    printIssues(issues);
  }
  if (settings.check && issues.length > 0) {
    return 1;
  }
  if (settings.dryRun) {
    return 0;
  }

  const result = await stageSdkworkImReleasePackage(plan);
  if (settings.json) {
    console.log(JSON.stringify({
      ok: true,
      result,
    }, null, 2));
  } else {
    console.log(`[sdkwork-im-release-stage] staged: ${result.stagingRoot}`);
    console.log(`[sdkwork-im-release-stage] files: ${result.files.length}`);
  }
  return 0;
}

async function runAllStaging(settings) {
  const plans = createSdkworkImInstallPackagePlan({ version: settings.version }).packages.map((packageItem) =>
    createSdkworkImReleaseStagingPlan({
      packageId: packageItem.id,
      root: repoRoot,
      stagingRoot: settings.stagingRoot ? path.join(settings.stagingRoot, packageItem.id) : null,
      version: settings.version,
    })
  );
  const issues = plans.flatMap((plan) =>
    validateSdkworkImReleaseStagingPlan(plan, { requireSources: !settings.dryRun })
      .map((issue) => `${plan.package.id}: ${issue}`)
  );
  if (settings.json) {
    console.log(JSON.stringify({
      ok: issues.length === 0,
      issues,
      dryRun: settings.dryRun,
      plans: plans.map(serializablePlan),
    }, null, 2));
  } else {
    for (const plan of plans) {
      for (const line of renderStagingPlan(plan)) {
        console.log(line);
      }
    }
    printIssues(issues);
  }
  if (settings.check && issues.length > 0) {
    return 1;
  }
  if (settings.dryRun) {
    return 0;
  }
  for (const plan of plans) {
    await stageSdkworkImReleasePackage(plan);
  }
  return 0;
}

function renderStagingPlan(plan) {
  return [
    `[sdkwork-im-release-stage] package: ${plan.package.id}`,
    `[sdkwork-im-release-stage] staging root: ${plan.stagingRoot}`,
    ...plan.actions.map((action) => `[sdkwork-im-release-stage]   ${action.kind}: ${action.label} -> ${path.relative(plan.stagingRoot, action.targetPath)}`),
  ];
}

function printIssues(issues) {
  if (issues.length === 0) {
    return;
  }
  console.error('[sdkwork-im-release-stage] validation issues:');
  for (const issue of issues) {
    console.error(`[sdkwork-im-release-stage]   ${issue}`);
  }
}

function serializablePlan(plan) {
  return {
    ...plan,
    actions: plan.actions.map((action) => {
      const { contentFactory, ...rest } = action;
      return rest;
    }),
  };
}

if (process.argv[1] && import.meta.url.endsWith(process.argv[1].replaceAll('\\', '/'))) {
  main().then((code) => {
    process.exitCode = code;
  }).catch((error) => {
    console.error(`[sdkwork-im-release-stage] ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  });
}

export {
  SERVER_SCRIPT_PATTERNS,
  STAGING_SCHEMA_VERSION,
  createSdkworkImReleaseStagingPlan,
  createInstallGuide,
  createInstallManifest,
  currentHostServerPackageId,
  main,
  parseStagingArgs,
  stageSdkworkImReleasePackage,
  validateSdkworkImReleaseStagingPlan,
};
