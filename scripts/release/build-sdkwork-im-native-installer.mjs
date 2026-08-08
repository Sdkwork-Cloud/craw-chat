#!/usr/bin/env node

// Build Sdkwork IM platform-native install packages from staged production
// server files (the same dist/release-staging/<archive-package-id> layout the
// archive builder consumes).
//
// Native package mapping (server profile only; desktop native installers are
// the Tauri bundles collected by collect-sdkwork-im-desktop-bundles.mjs and
// published by workflow-release-target.mjs):
//   linux   .deb   ar(debian-binary, control.tar.gz, data.tar.gz)
//   windows .msi   WiX CLI (service registered via pinned WinSW wrapper)
//   macos   .pkg   pkgbuild (macOS host / CI only)
//
// Directory mapping follows PACKAGING_SPEC §5.5 and RUNTIME_DIRECTORY_SPEC
// §4: private runtime assets under /usr/lib/sdkwork/chat (Linux) and
// %ProgramFiles%\sdkwork\chat (Windows), config under /etc/sdkwork/chat and
// %ProgramData%\sdkwork\chat, workspace database secret under
// /etc/sdkwork/database/database.secret.
//
// The .deb and .msi are built with deterministic output (gzip mtime 0, sorted
// entries, SOURCE_DATE_EPOCH manifest timestamps) and cached by an input
// snapshot, so repeat builds reuse unchanged installers (PACKAGING_SPEC §4).
//
// Supply chain: the WinSW wrapper is downloaded at build time from the pinned
// release with a recorded SHA-256 (never committed as a blob); the package
// records it as a third-party component in the adjacent manifest.

import { execFile } from 'node:child_process';
import { createHash } from 'node:crypto';
import {
  access,
  chmod,
  mkdir,
  readFile,
  rm,
  stat as statFile,
  writeFile,
} from 'node:fs/promises';
import { existsSync, readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { promisify } from 'node:util';
import { fileURLToPath } from 'node:url';
import { gzipSync } from 'node:zlib';

import { DEFAULT_RELEASE_VERSION } from './sdkwork-im-release-version.mjs';
import {
  createSdkworkImInstallPackageBuildPlan,
  createAggregateManifest,
  modeForArchivePath,
  sha256,
} from './build-sdkwork-im-install-package.mjs';
import {
  createSdkworkImNativeInstallPackagePlan,
  currentHostNativePackageId,
  validateSdkworkImNativeInstallPackagePlan,
} from './plan-sdkwork-im-native-install-packages.mjs';

const execFileAsync = promisify(execFile);
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const repoRoot = path.resolve(__dirname, '..', '..');

const NATIVE_INSTALLER_SCHEMA_VERSION = '2026-08-08.sdkwork-im.native-installer-build.v1';
const NATIVE_INSTALL_LAYOUT_SCHEMA_VERSION = '2026-08-08.sdkwork-im.native-install-layout.v1';
const NATIVE_MANIFEST_SCHEMA_VERSION = '2026-08-08.sdkwork-im.native-installer-manifest.v1';
const SNAPSHOT_SCHEMA_VERSION = '2026-08-08.sdkwork-im.native-installer-snapshot.v1';
const SHA256SUMS_FILE = 'SHA256SUMS';

// RUNTIME_DIRECTORY_SPEC §4.1/§4.4 canonical host paths (application code
// `chat`; PACKAGING_SPEC §5.5 installer projection).
const LINUX_NATIVE_INSTALL_ROOT = '/usr/lib/sdkwork/chat';
const LINUX_NATIVE_SHARED_ROOT = '/usr/share/sdkwork/chat';
const LINUX_NATIVE_SHARED_DOC_ROOT = '/usr/share/doc/sdkwork/chat';
const LINUX_SERVICE_CONFIG_ROOT = '/etc/sdkwork/chat';
const LINUX_SERVICE_DATABASE_SECRET_ROOT = '/etc/sdkwork/database';
const LINUX_SERVICE_DATABASE_SECRET_FILE = '/etc/sdkwork/database/database.secret';
const LINUX_SERVICE_DATA_ROOT = '/var/lib/sdkwork/chat';
const LINUX_SERVICE_LOG_ROOT = '/var/log/sdkwork/chat';
const LINUX_SERVICE_CACHE_ROOT = '/var/cache/sdkwork/chat';
const LINUX_SERVICE_RUN_ROOT = '/run/sdkwork/chat';
const LINUX_SYSTEMD_UNIT_PATH = '/usr/lib/systemd/system/sdkwork-chat.service';
const LINUX_SERVICE_ENV_FILE = `${LINUX_SERVICE_CONFIG_ROOT}/server.env`;

// Windows: service name per PACKAGING_SPEC §5.5 (SCM name sdkwork-<code>).
const WINDOWS_SERVICE_NAME = 'sdkwork-chat';
const WINDOWS_SERVICE_DISPLAY_NAME = 'SDKWork IM Standalone API Gateway';
// Fixed upgrade code: changing it would orphan previous installations.
const WINDOWS_UPGRADE_CODE = '8F2C4E1A-6B9D-4A3F-9C5E-2D1B7A0F3E8C';
// Pinned WinSW wrapper (MIT, https://github.com/winsw/winsw). WinSW v2.12.0
// publishes only x64/x86 assets; Windows on ARM runs x64 emulation, so the
// x64 wrapper is used for both MSI architectures.
const WINSW_VERSION = '2.12.0';
const WINSW_X64_SHA256 = '05b82d46ad331cc16bdc00de5c6332c1ef818df8ceefcd49c726553209b3a0da';
const WINSW_X64_SIZE = 18243033;
const WINSW_DOWNLOAD_URL = `https://github.com/winsw/winsw/releases/download/v${WINSW_VERSION}/WinSW-x64.exe`;

const MACOS_SERVICE_ROOT = '/Library/Application Support/sdkwork/chat';
const MACOS_INSTALL_ROOT = '/usr/lib/sdkwork/chat';
const MACOS_LAUNCH_DAEMON_PATH = '/Library/LaunchDaemons/com.sdkwork.chat.plist';

// Embedded dependency workspaces whose packaged database modules must exist in
// the installed package (the standalone gateway boots each module's lifecycle
// and materializes the IAM catalog into its module root). Mirrors
// scripts/build-im-standalone-container.mjs. The module roots are installed
// under the durable-data directory so catalog materialization stays writable
// while /usr/lib stays immutable (PACKAGING_SPEC §5.5 / RUNTIME_DIRECTORY_SPEC
// §4.1); SDKWORK_<MODULE>_APP_ROOT points at them.
const EMBEDDED_MODULE_WORKSPACES = [
  'sdkwork-drive',
  'sdkwork-knowledgebase',
  'sdkwork-inventory',
  'sdkwork-invoice',
  'sdkwork-membership',
  'sdkwork-order',
  'sdkwork-payment',
  'sdkwork-shop',
  'sdkwork-notary',
  'sdkwork-agents',
  'sdkwork-iam',
];
const EXTRA_APP_ROOT_DIRS = {
  'sdkwork-iam': ['iam'],
};
const LINUX_MODULE_ROOT = `${LINUX_SERVICE_DATA_ROOT}/modules`;
const WINDOWS_MODULE_ROOT = 'sdkwork/chat/modules';
const MACOS_MODULE_ROOT = `${MACOS_SERVICE_ROOT}/modules`;

const SERVER_BINARY_BASENAME = 'sdkwork-api-im-standalone-gateway';
const APP_CONFIG_ARCHIVE_PATH = 'sdkwork.app.config.json';
const IM_DATABASE_MODULE_ARCHIVE_PATH = 'database';
const INSTALL_MANIFEST_ARCHIVE_PATH = 'install-manifest.json';
const DEB_INSTALL_MANIFEST_PATH = `${LINUX_NATIVE_SHARED_ROOT}/${INSTALL_MANIFEST_ARCHIVE_PATH}`;
const MSI_INSTALL_MANIFEST_PATH = 'sdkwork/chat/install-manifest.json';

function printHelp() {
  console.log(`Usage: node scripts/release/build-sdkwork-im-native-installer.mjs [options]

Build Sdkwork IM platform-native server installers from staged production files.

Native package mapping (server profile):
  linux   .deb   ar archive (control + data, generated systemd unit)
  windows .msi   WiX CLI (WinSW-registered sdkwork-chat service)
  macos   .pkg   pkgbuild (macOS host / CI only)

Options:
  --package-id <id>    Native package id from the native package plan.
  --all                Build all native package ids for the current host platform.
  --staging-root <dir> Staging root (default dist/release-staging/<archive-package-id>).
  --output-dir <dir>   Output directory (default dist/release-packages).
  --version <value>    Package version (default ${DEFAULT_RELEASE_VERSION}).
  --winsw-path <file>  Pre-staged WinSW.exe (skips the pinned download).
  --check              Validate the native installer build plan.
  --dry-run            Print the build plan without writing packages.
  --json               Print machine-readable JSON.
  -h, --help           Show this help.
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
    all: false,
    check: false,
    dryRun: false,
    help: false,
    json: false,
    outputDir: null,
    packageId: currentHostNativePackageId(),
    stagingRoot: null,
    version: DEFAULT_RELEASE_VERSION,
    winswPath: null,
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
      case '--output-dir':
        settings.outputDir = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--version':
        settings.version = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--winsw-path':
        settings.winswPath = requireValue(argv, index, arg);
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
        throw new Error(`Unsupported native installer build option: ${arg}`);
    }
  }

  return settings;
}

function createNativeInstallerBuildPlan({
  packageId = currentHostNativePackageId(),
  outputDir = null,
  requireStagedFiles = true,
  root = repoRoot,
  stagingRoot = null,
  version = DEFAULT_RELEASE_VERSION,
  winswPath = null,
} = {}) {
  const nativePlan = createSdkworkImNativeInstallPackagePlan({ version });
  const planIssues = validateSdkworkImNativeInstallPackagePlan(nativePlan);
  if (planIssues.length > 0) {
    throw new Error(`native install package plan is invalid: ${planIssues.join('; ')}`);
  }
  const packageItem = nativePlan.packages.find((item) => item.id === packageId);
  if (!packageItem) {
    throw new Error(`Unknown native install package id: ${packageId}`);
  }
  if (packageItem.profile !== 'server') {
    throw new Error(`${packageId} is a desktop native package; Tauri builds and publishes it (workflow-release-target.mjs)`);
  }

  const archiveBuildPlan = createSdkworkImInstallPackageBuildPlan({
    packageId: packageItem.stagingPackageId,
    outputDir,
    requireStagedFiles,
    root,
    stagingRoot,
    version,
  });
  const absoluteOutputDir = path.resolve(root, outputDir ?? path.join('dist', 'release-packages'));
  const installerName = packageItem.installerName;

  return {
    schemaVersion: NATIVE_INSTALLER_SCHEMA_VERSION,
    package: packageItem,
    nativeFormat: packageItem.format,
    nativeInstallLayout: createNativeInstallLayout(packageItem),
    installerName,
    installerPath: path.join(absoluteOutputDir, installerName),
    manifestPath: path.join(absoluteOutputDir, installerName.replace(/\.(deb|pkg|msi)$/u, '.manifest.json')),
    snapshotPath: path.join(absoluteOutputDir, `${installerName}.snapshot.json`),
    outputDir: absoluteOutputDir,
    stagingRoot: archiveBuildPlan.stagingRoot,
    archiveBuildPlan,
    winswPath,
  };
}

function validateNativeInstallerBuildPlan(plan) {
  const issues = [];
  if (plan.schemaVersion !== NATIVE_INSTALLER_SCHEMA_VERSION) {
    issues.push(`schemaVersion must be ${NATIVE_INSTALLER_SCHEMA_VERSION}`);
  }
  if (!plan.package?.id) {
    issues.push('package id is required');
    return issues;
  }
  const expectedFormat = plan.package.format;
  if (plan.nativeFormat !== expectedFormat) {
    issues.push(`${plan.package.id} nativeFormat must be ${expectedFormat}`);
  }
  if (!plan.installerPath || !plan.installerPath.endsWith(plan.installerName)) {
    issues.push('installerPath must end with installerName');
  }
  if (plan.package.platform === 'windows' && process.platform !== 'win32' && !plan.dryRun) {
    issues.push(`${plan.package.id} .msi build requires Windows WiX tooling; use --dry-run on non-Windows hosts`);
  }
  if (plan.package.platform === 'macos' && process.platform !== 'darwin' && !plan.dryRun) {
    issues.push(`${plan.package.id} .pkg build requires macOS pkgbuild; use --dry-run on non-macOS hosts`);
  }
  const archiveIssues = validateArchiveBuildPlanEntries(plan.archiveBuildPlan ?? {});
  issues.push(...archiveIssues.map((issue) => `${plan.package.id}: ${issue}`));
  return issues;
}

function validateArchiveBuildPlanEntries(buildPlan) {
  const issues = [];
  if (!Array.isArray(buildPlan.entries) || buildPlan.entries.length === 0) {
    issues.push('staged package must include archive entries');
    return issues;
  }
  for (const requiredPath of [
    `bin/${buildPlan.package?.binaryName ?? SERVER_BINARY_BASENAME}`,
    'config/chat.toml.example',
    'config/server.env.example',
    'config/postgresql.yaml.example',
    'INSTALL.md',
    INSTALL_MANIFEST_ARCHIVE_PATH,
  ]) {
    if (!buildPlan.entries.some((entry) => entry.archivePath === requiredPath)) {
      issues.push(`staged package must include ${requiredPath}`);
    }
  }
  return issues;
}

async function buildNativeInstaller(plan) {
  const issues = validateNativeInstallerBuildPlan(plan);
  if (issues.length > 0) {
    throw new Error(`native installer build plan is invalid: ${issues.join('; ')}`);
  }
  await mkdir(plan.outputDir, { recursive: true });

  const generatedAt = manifestTimestamp();
  const packageFiles = await collectPackageFileEntries(plan, { generatedAt });

  const snapshot = await collectNativeInstallerSnapshot(plan);
  let cached = false;
  if (await nativeInstallerCacheHits(plan, snapshot)) {
    console.log(`[sdkwork-im-native-installer] inputs unchanged; reusing cached installer ${path.basename(plan.installerPath)}`);
    cached = true;
  } else {
    if (plan.package.platform === 'linux') {
      await writeFile(plan.installerPath, createDebianPackage(plan, packageFiles.fileEntries));
    } else if (plan.package.platform === 'macos') {
      await buildMacosPkg(plan, packageFiles.fileEntries);
    } else if (plan.package.platform === 'windows') {
      await buildWindowsMsi(plan, packageFiles.fileEntries, { winswPath: plan.winswPath });
    } else {
      throw new Error(`Unsupported native installer platform: ${plan.package.platform}`);
    }
    await writeFile(plan.snapshotPath, `${JSON.stringify(snapshot, null, 2)}\n`, 'utf8');
  }

  const installerBytes = await readFile(plan.installerPath);
  const installer = {
    file: path.basename(plan.installerPath),
    packageId: plan.package.id,
    version: plan.package.version,
    kind: 'native-installer',
    format: plan.nativeFormat,
    platform: plan.package.platform,
    architecture: plan.package.architecture,
    deploymentProfile: plan.package.deploymentProfile,
    profile: plan.package.profile,
    runtimeTarget: plan.package.runtimeTarget,
    size: installerBytes.length,
    sha256: sha256(installerBytes),
    reused: cached,
  };
  const manifest = createNativeInstallerManifest(plan, installer, packageFiles, { generatedAt });
  await writeFile(plan.manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, 'utf8');
  const aggregateManifest = createAggregateManifest(
    { aggregateManifestPath: path.join(plan.outputDir, 'release-packages-manifest.json') },
    installer,
  );
  await writeFile(
    path.join(plan.outputDir, 'release-packages-manifest.json'),
    `${JSON.stringify(aggregateManifest, null, 2)}\n`,
    'utf8',
  );
  await writeSha256Sums(plan.outputDir);

  return {
    installer,
    installerPath: plan.installerPath,
    manifest,
    manifestPath: plan.manifestPath,
    aggregateManifest,
    aggregateManifestPath: path.join(plan.outputDir, 'release-packages-manifest.json'),
  };
}

function createNativeInstallerManifest(plan, installer, packageFiles, { generatedAt }) {
  return {
    schemaVersion: NATIVE_MANIFEST_SCHEMA_VERSION,
    generatedAt,
    product: 'chat',
    package: {
      id: plan.package.id,
      installerName: plan.installerName,
      version: plan.package.version,
      platform: plan.package.platform,
      distribution: plan.package.distribution,
      architecture: plan.package.architecture,
      deploymentProfile: plan.package.deploymentProfile,
      profile: plan.package.profile,
      runtimeTarget: plan.package.runtimeTarget,
      format: plan.nativeFormat,
      serviceName: plan.package.serviceName,
    },
    installer,
    nativeInstallLayout: plan.nativeInstallLayout,
    files: packageFiles.files,
    thirdPartyComponents: packageFiles.thirdPartyComponents,
  };
}

async function collectPackageFileEntries(plan, { generatedAt }) {
  const files = [];
  const thirdPartyComponents = [];
  const fileEntries = [];
  const stagedManifestBytes = await readStagedInstallManifest(plan);

  for (const entry of plan.archiveBuildPlan.entries) {
    if (entry.archivePath === INSTALL_MANIFEST_ARCHIVE_PATH) {
      continue;
    }
    if (isSkippedNativeStagingEntry(plan, entry.archivePath)) {
      continue;
    }
    const data = await readFile(entry.sourcePath);
    files.push({
      path: entry.archivePath,
      size: data.length,
      sha256: sha256(data),
    });
    fileEntries.push({
      relativePath: entry.archivePath,
      data,
      mode: entry.mode ?? modeForArchivePath(entry.archivePath),
    });
  }

  // Application identity manifest: IAM tenant provisioning resolves it via
  // SDKWORK_APP_ROOT (mirrors the container image payload).
  const appConfigBytes = await readFile(path.join(repoRoot, 'sdkwork.app.config.json'));
  files.push({
    path: APP_CONFIG_ARCHIVE_PATH,
    size: appConfigBytes.length,
    sha256: sha256(appConfigBytes),
  });
  fileEntries.push({
    relativePath: APP_CONFIG_ARCHIVE_PATH,
    data: appConfigBytes,
    mode: 0o644,
  });

  // The IM database module (manifest + migrations) resolves from
  // SDKWORK_IM_APP_ROOT/database; the staged archive package does not carry
  // it (only the container payload does), so the native installer packages it
  // from the repository like the container builder.
  for (const filePath of listFiles(path.join(repoRoot, IM_DATABASE_MODULE_ARCHIVE_PATH))) {
    const relative = path.relative(
      path.join(repoRoot, IM_DATABASE_MODULE_ARCHIVE_PATH),
      filePath,
    ).replaceAll('\\', '/');
    if (isSensitiveModulePath(relative)) {
      continue;
    }
    const data = await readFile(filePath);
    files.push({
      path: `${IM_DATABASE_MODULE_ARCHIVE_PATH}/${relative}`,
      size: data.length,
      sha256: sha256(data),
      module: 'sdkwork-im',
    });
    fileEntries.push({
      relativePath: `${IM_DATABASE_MODULE_ARCHIVE_PATH}/${relative}`,
      data,
      mode: 0o644,
    });
  }

  // Embedded dependency database modules (see EMBEDDED_MODULE_WORKSPACES).
  for (const moduleEntry of await collectEmbeddedModuleEntries(plan)) {
    files.push({
      path: moduleEntry.relativePath,
      size: moduleEntry.data.length,
      sha256: sha256(moduleEntry.data),
      module: moduleEntry.module,
    });
    fileEntries.push(moduleEntry);
  }

  // The packaged install-manifest.json carries the native layout injected
  // into the staged manifest so installed systems self-describe the layout.
  const nativeManifest = {
    ...JSON.parse(stagedManifestBytes),
    generatedAt,
    package: {
      ...JSON.parse(stagedManifestBytes).package,
      id: plan.package.id,
      format: plan.nativeFormat,
      archiveName: plan.installerName,
    },
    nativeInstall: plan.nativeInstallLayout,
  };
  const nativeManifestBytes = Buffer.from(`${JSON.stringify(nativeManifest, null, 2)}\n`, 'utf8');
  files.push({
    path: INSTALL_MANIFEST_ARCHIVE_PATH,
    size: nativeManifestBytes.length,
    sha256: sha256(nativeManifestBytes),
  });
  fileEntries.push({
    relativePath: INSTALL_MANIFEST_ARCHIVE_PATH,
    data: nativeManifestBytes,
    mode: 0o644,
  });

  if (plan.package.platform === 'linux') {
    const unitBytes = Buffer.from(createSystemdUnit(plan), 'utf8');
    files.push({
      path: 'service/linux/sdkwork-chat.service',
      size: unitBytes.length,
      sha256: sha256(unitBytes),
      generated: true,
    });
    fileEntries.push({
      relativePath: 'service/linux/sdkwork-chat.service',
      data: unitBytes,
      mode: 0o644,
    });
  }

  if (plan.package.platform === 'windows') {
    const winswBytes = await resolveWinSwBytes(plan);
    const winswXmlBytes = Buffer.from(createWinSwXml(plan), 'utf8');
    thirdPartyComponents.push({
      name: 'WinSW',
      version: WINSW_VERSION,
      license: 'MIT',
      url: 'https://github.com/winsw/winsw',
      file: 'service/windows/sdkwork-chat-service.exe',
      sha256: sha256(winswBytes),
      note: 'Windows service wrapper around the standalone gateway; downloaded at build time and verified against the pinned digest.',
    });
    files.push({
      path: 'service/windows/sdkwork-chat-service.exe',
      size: winswBytes.length,
      sha256: sha256(winswBytes),
      thirdParty: 'WinSW',
    });
    files.push({
      path: 'service/windows/sdkwork-chat-service.xml',
      size: winswXmlBytes.length,
      sha256: sha256(winswXmlBytes),
      generated: true,
    });
    fileEntries.push(
      {
        relativePath: 'service/windows/sdkwork-chat-service.exe',
        data: winswBytes,
        mode: 0o755,
      },
      {
        relativePath: 'service/windows/sdkwork-chat-service.xml',
        data: winswXmlBytes,
        mode: 0o644,
      },
    );
  }

  return {
    fileEntries: fileEntries.sort((left, right) => left.relativePath.localeCompare(right.relativePath)),
    files: files.sort((left, right) => left.path.localeCompare(right.path)),
    manifest: nativeManifest,
    thirdPartyComponents,
    generatedAt,
  };
}

// Walks the sibling workspace database modules (and IAM extra app-root
// directories) exactly like the container image builder.
async function collectEmbeddedModuleEntries() {
  const entries = [];
  for (const workspace of EMBEDDED_MODULE_WORKSPACES) {
    const workspaceRoot = path.resolve(repoRoot, '..', workspace);
    const databaseRoot = path.join(workspaceRoot, 'database');
    if (!existsSync(databaseRoot)) {
      throw new Error(
        `embedded database module missing for ${workspace}: ${databaseRoot} `
        + '(the native installer packages sibling workspace modules; clone sdkwork-<workspace> next to sdkwork-im)',
      );
    }
    for (const filePath of listFiles(databaseRoot)) {
      const relative = path.relative(databaseRoot, filePath).replaceAll('\\', '/');
      if (isSensitiveModulePath(relative)) {
        continue;
      }
      const data = await readFile(filePath);
      entries.push({
        module: workspace,
        relativePath: `modules/${workspace}/database/${relative}`,
        data,
        mode: 0o644,
      });
    }
    for (const extraDir of EXTRA_APP_ROOT_DIRS[workspace] ?? []) {
      const extraRoot = path.join(workspaceRoot, extraDir);
      if (!existsSync(extraRoot)) {
        continue;
      }
      for (const filePath of listFiles(extraRoot)) {
        const relative = path.relative(extraRoot, filePath).replaceAll('\\', '/');
        if (isSensitiveModulePath(relative)) {
          continue;
        }
        const data = await readFile(filePath);
        entries.push({
          module: workspace,
          relativePath: `modules/${workspace}/${extraDir}/${relative}`,
          data,
          mode: 0o644,
        });
      }
    }
  }
  return entries.sort((left, right) => left.relativePath.localeCompare(right.relativePath));
}

function isSensitiveModulePath(relativePath) {
  const normalized = String(relativePath).replaceAll('\\', '/');
  return /(^|\/)\.env($|\.|\/)|(^|\/)node_modules(\/|$)|(^|\/)\.runtime(\/|$)|secret/u.test(normalized);
}

function listFiles(root) {
  const files = [];
  for (const entry of readdirSync(root, { withFileTypes: true })) {
    const entryPath = path.join(root, entry.name);
    if (entry.isDirectory()) {
      files.push(...listFiles(entryPath));
    } else if (entry.isFile()) {
      files.push(entryPath);
    }
  }
  return files.sort();
}

function isSkippedNativeStagingEntry(plan, archivePath) {
  const normalized = String(archivePath).replaceAll('\\', '/');
  if (plan.package.platform === 'linux') {
    return normalized.startsWith('service/windows/') || normalized.startsWith('service/macos/');
  }
  if (plan.package.platform === 'windows') {
    return normalized.startsWith('service/linux/') || normalized.startsWith('service/macos/');
  }
  return normalized.startsWith('service/linux/') || normalized.startsWith('service/windows/');
}

async function readStagedInstallManifest(plan) {
  const stagedManifestEntry = plan.archiveBuildPlan.entries.find(
    (entry) => entry.archivePath === INSTALL_MANIFEST_ARCHIVE_PATH,
  );
  if (!stagedManifestEntry) {
    throw new Error('staged package is missing install-manifest.json');
  }
  return readFile(stagedManifestEntry.sourcePath, 'utf8');
}

// --- Debian (.deb) ---------------------------------------------------------

function createDebianPackage(plan, fileEntries) {
  const controlTar = createDebTar([
    { relativePath: './control', data: Buffer.from(createDebianControl(plan), 'utf8'), mode: 0o644 },
    { relativePath: './postinst', data: Buffer.from(createDebianPostinst(plan), 'utf8'), mode: 0o755 },
    { relativePath: './prerm', data: Buffer.from(createDebianPrerm(plan), 'utf8'), mode: 0o755 },
  ]);
  const dataTar = createDebTar(withDebianDirectoryEntries(
    fileEntries.flatMap((entry) => debianDataEntriesForPackageFile(plan, entry)),
  ));
  return createArArchive([
    { name: 'debian-binary', data: Buffer.from('2.0\n', 'utf8'), mode: 0o644 },
    { name: 'control.tar.gz', data: gzipSync(controlTar, { mtime: 0 }), mode: 0o644 },
    { name: 'data.tar.gz', data: gzipSync(dataTar, { mtime: 0 }), mode: 0o644 },
  ]);
}

function createDebianControl(plan) {
  return [
    'Package: sdkwork-chat',
    `Version: ${debianVersion(plan.package.version)}`,
    'Section: net',
    'Priority: optional',
    `Architecture: ${debianArchitecture(plan.package.architecture)}`,
    'Maintainer: SDKWork <release@sdkwork.com>',
    'Homepage: https://sdkwork.com',
    // PACKAGING_SPEC §5.2: .deb MUST declare real runtime dependencies and
    // MUST NOT bundle distro-provided libraries. The gateway links OpenSSL 3
    // (libssl3) and the glibc/gcc runtime (libc6, libgcc-s1) and needs
    // ca-certificates for outbound HTTPS.
    'Depends: libssl3, libc6, libgcc-s1, ca-certificates',
    'Description: SDKWork IM standalone API gateway (server)',
    ' Standalone IM server native installer: gateway binary, renderer web assets,',
    ' runtime configuration templates, and systemd service registration.',
    ' Installs under /usr/lib/sdkwork/chat with config in /etc/sdkwork/chat and',
    ' durable data in /var/lib/sdkwork/chat per RUNTIME_DIRECTORY_SPEC.',
    '',
  ].join('\n');
}

function createDebianPostinst(plan) {
  const summary = debianInstallSummaryLines(plan);
  return [
    '#!/bin/sh',
    'set -e',
    'if ! getent group sdkwork >/dev/null; then',
    '  groupadd --system sdkwork',
    'fi',
    'if ! id -u sdkwork >/dev/null 2>&1; then',
    `  useradd --system --gid sdkwork --home-dir ${LINUX_SERVICE_DATA_ROOT} --shell /usr/sbin/nologin sdkwork`,
    'fi',
    `mkdir -p ${LINUX_SERVICE_CONFIG_ROOT} ${LINUX_SERVICE_DATABASE_SECRET_ROOT} ${LINUX_SERVICE_DATA_ROOT} ${LINUX_SERVICE_LOG_ROOT} ${LINUX_SERVICE_CACHE_ROOT} ${LINUX_SERVICE_RUN_ROOT} ${LINUX_SERVICE_DATA_ROOT}/secrets`,
    // RUNTIME_DIRECTORY_SPEC §6 permission table.
    `chown root:sdkwork ${LINUX_SERVICE_CONFIG_ROOT} ${LINUX_SERVICE_DATABASE_SECRET_ROOT}`,
    `chmod 0750 ${LINUX_SERVICE_CONFIG_ROOT} ${LINUX_SERVICE_DATABASE_SECRET_ROOT}`,
    `chown -R sdkwork:sdkwork ${LINUX_SERVICE_DATA_ROOT} ${LINUX_SERVICE_LOG_ROOT} ${LINUX_SERVICE_CACHE_ROOT} ${LINUX_SERVICE_RUN_ROOT}`,
    `chmod 0750 ${LINUX_SERVICE_DATA_ROOT} ${LINUX_SERVICE_LOG_ROOT} ${LINUX_SERVICE_CACHE_ROOT} ${LINUX_SERVICE_RUN_ROOT} ${LINUX_SERVICE_DATA_ROOT}/secrets`,
    // Config templates installed by dpkg land in /etc/sdkwork/chat with 0640
    // root:sdkwork (dpkg installs 0644; tighten here).
    `chown root:sdkwork ${LINUX_SERVICE_CONFIG_ROOT}/*.example 2>/dev/null || true`,
    `chmod 0640 ${LINUX_SERVICE_CONFIG_ROOT}/*.example 2>/dev/null || true`,
    // Service environment: created once, operator-editable, secret-free.
    `if [ ! -f ${LINUX_SERVICE_ENV_FILE} ]; then`,
    `  cat > ${LINUX_SERVICE_ENV_FILE} <<'EOF'`,
    '# SDKWork IM standalone gateway service environment.',
    '# Created by the sdkwork-chat Debian package; use this file for explicit process overrides.',
    '# Secrets belong in /etc/sdkwork/database/database.secret or protected files, never here.',
    'SDKWORK_IM_DEPLOYMENT_PROFILE=standalone',
    'SDKWORK_IM_RUNTIME_TARGET=server',
    'SDKWORK_IM_ENVIRONMENT=production',
    'SDKWORK_IM_PROFILE_ID=standalone.production',
    'SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND=0.0.0.0:18079',
    'SDKWORK_IM_ID_NODE_ID=1',
    'SDKWORK_CORS_ALLOWED_ORIGINS=http://localhost,http://127.0.0.1:18079',
    'SDKWORK_DATABASE_ENGINE=postgresql',
    'SDKWORK_DATABASE_HOST=127.0.0.1',
    'SDKWORK_DATABASE_PORT=5432',
    'SDKWORK_DATABASE_NAME=sdkwork_ai_prod',
    'SDKWORK_DATABASE_SCHEMA=sdkwork_ai_prod',
    'SDKWORK_DATABASE_USERNAME=sdkwork_ai_prod',
    'SDKWORK_DATABASE_PASSWORD_FILE=/etc/sdkwork/database/database.secret',
    'SDKWORK_DATABASE_SSL_MODE=require',
    'SDKWORK_DATABASE_AUTO_MIGRATE=true',
    // Shared process pool plumbing required by the IM pool bootstrap.
    'SDKWORK_DATABASE_TEMPORARY_DRIVER_POOL_COUNT=2',
    'SDKWORK_DATABASE_TEMPORARY_ANY_POOL_EXCEPTION=true',
    // Application roots: installed layout, never a build-machine checkout
    // (DEPLOYMENT_SPEC §1.1). Embedded modules live under durable data so the
    // IAM catalog materialization stays writable.
    `SDKWORK_APP_ROOT=${LINUX_NATIVE_INSTALL_ROOT}`,
    `SDKWORK_IM_APP_ROOT=${LINUX_NATIVE_INSTALL_ROOT}`,
    ...EMBEDDED_MODULE_WORKSPACES.map((workspace) =>
      `SDKWORK_${workspace.replace('sdkwork-', '').toUpperCase()}_APP_ROOT=${LINUX_MODULE_ROOT}/${workspace}`
    ),
    // Payment credential master key: required in production unless a payment
    // credential cipher host is installed; auto-created in development.
    'SDKWORK_PAYMENT_CREDENTIAL_MASTER_KEY_FILE=/var/lib/sdkwork/chat/secrets/payment-credential-master.key',
    'RUST_LOG=info',
    'EOF',
    'fi',
    `chown root:sdkwork ${LINUX_SERVICE_ENV_FILE}`,
    `chmod 0640 ${LINUX_SERVICE_ENV_FILE}`,
    // Workspace database secret placeholder: ENVIRONMENT_SPEC §7.3. Startup
    // fails closed until the operator replaces it with the real password.
    `if [ ! -f ${LINUX_SERVICE_DATABASE_SECRET_FILE} ]; then`,
    `  printf "%s\\n" "change-me" > ${LINUX_SERVICE_DATABASE_SECRET_FILE}`,
    'fi',
    `chown root:sdkwork ${LINUX_SERVICE_DATABASE_SECRET_FILE}`,
    `chmod 0640 ${LINUX_SERVICE_DATABASE_SECRET_FILE}`,
    'if command -v systemctl >/dev/null 2>&1; then',
    '  systemctl daemon-reload || true',
    '  systemctl enable sdkwork-chat.service >/dev/null 2>&1 || true',
    'fi',
    'cat <<\'EOF\'',
    ...summary,
    'EOF',
    'exit 0',
    '',
  ].join('\n');
}

function createDebianPrerm() {
  return [
    '#!/bin/sh',
    'set -e',
    'if [ "$1" = "remove" ] && command -v systemctl >/dev/null 2>&1; then',
    '  systemctl stop sdkwork-chat.service >/dev/null 2>&1 || true',
    '  systemctl disable sdkwork-chat.service >/dev/null 2>&1 || true',
    'fi',
    'exit 0',
    '',
  ].join('\n');
}

function debianInstallSummaryLines(plan) {
  return [
    '',
    'SDKWork IM installation summary',
    '-------------------------------',
    `Package: ${plan.package.id}`,
    `Service environment: ${LINUX_SERVICE_ENV_FILE}`,
    `PostgreSQL password file: ${LINUX_SERVICE_DATABASE_SECRET_FILE}`,
    'Systemd service: sdkwork-chat.service',
    '',
    'Before first start:',
    `  sudo editor ${LINUX_SERVICE_ENV_FILE}`,
    `  sudo editor ${LINUX_SERVICE_DATABASE_SECRET_FILE}`,
    '  sudo systemctl start sdkwork-chat',
    '  sudo systemctl status sdkwork-chat --no-pager',
    '  sudo journalctl -u sdkwork-chat -f',
    '',
    'Placeholder database values (change-me / sdkwork_ai_prod defaults) are rejected at startup until configured.',
    '',
  ];
}

function createSystemdUnit() {
  // Generated at build time with the /usr/lib install paths; the staged
  // archive unit (opt paths) is not reused for the native package.
  return [
    '[Unit]',
    'Description=SDKWork IM Standalone API Gateway',
    'After=network-online.target',
    'Wants=network-online.target',
    '',
    '[Service]',
    'Type=simple',
    'User=sdkwork',
    'Group=sdkwork',
    `WorkingDirectory=${LINUX_SERVICE_DATA_ROOT}`,
    `EnvironmentFile=${LINUX_SERVICE_ENV_FILE}`,
    `ExecStart=${LINUX_NATIVE_INSTALL_ROOT}/bin/${SERVER_BINARY_BASENAME}`,
    'Restart=on-failure',
    'RestartSec=5',
    // systemd-managed runtime directories (RUNTIME_DIRECTORY_SPEC §4.1).
    'StateDirectory=sdkwork/chat',
    'RuntimeDirectory=sdkwork/chat',
    'LogsDirectory=sdkwork/chat',
    'CacheDirectory=sdkwork/chat',
    'NoNewPrivileges=true',
    'ProtectSystem=strict',
    'ProtectHome=true',
    'PrivateTmp=true',
    '',
    '[Install]',
    'WantedBy=multi-user.target',
    '',
  ].join('\n');
}

function debianDataEntriesForPackageFile(plan, entry) {
  const targetPaths = debianInstallPathsForArchivePath(plan, entry.relativePath);
  return targetPaths.map((targetPath) => ({
    relativePath: `.${targetPath}`,
    data: entry.data,
    mode: targetPath.startsWith(`${LINUX_SERVICE_CONFIG_ROOT}/`) ? 0o640 : (entry.mode ?? modeForArchivePath(entry.relativePath)),
  }));
}

function debianInstallPathsForArchivePath(plan, archivePath) {
  const normalized = String(archivePath).replaceAll('\\', '/');
  if (normalized.startsWith('bin/')) {
    return [`${LINUX_NATIVE_INSTALL_ROOT}/${normalized}`];
  }
  if (normalized.startsWith('config/')) {
    return [`${LINUX_SERVICE_CONFIG_ROOT}/${normalized.slice('config/'.length)}`];
  }
  if (normalized.startsWith('web/')) {
    return [`${LINUX_NATIVE_SHARED_ROOT}/${normalized}`];
  }
  if (normalized.startsWith('modules/')) {
    // Embedded modules live under durable data: the IAM registry
    // materialization writes into the module root (must stay writable while
    // /usr/lib stays immutable).
    return [`${LINUX_MODULE_ROOT}/${normalized.slice('modules/'.length)}`];
  }
  if (normalized.startsWith('database/')) {
    // IM database module resolves from SDKWORK_IM_APP_ROOT/database; kept
    // under immutable assets because the IM lifecycle only reads it.
    return [`${LINUX_NATIVE_INSTALL_ROOT}/${normalized}`];
  }
  if (normalized === APP_CONFIG_ARCHIVE_PATH) {
    return [`${LINUX_NATIVE_INSTALL_ROOT}/${APP_CONFIG_ARCHIVE_PATH}`];
  }
  if (normalized === 'INSTALL.md') {
    return [`${LINUX_NATIVE_SHARED_DOC_ROOT}/INSTALL.md`];
  }
  if (normalized === INSTALL_MANIFEST_ARCHIVE_PATH) {
    return [DEB_INSTALL_MANIFEST_PATH];
  }
  if (normalized === 'service/linux/sdkwork-chat.service') {
    return [LINUX_SYSTEMD_UNIT_PATH];
  }
  return [];
}

function withDebianDirectoryEntries(fileEntries) {
  const directories = new Map();
  for (const entry of fileEntries) {
    for (const directory of parentDirectoriesForTarPath(entry.relativePath)) {
      if (!directories.has(directory)) {
        directories.set(directory, {
          relativePath: directory,
          data: Buffer.alloc(0),
          mode: debianDirectoryMode(directory),
          type: 'directory',
        });
      }
    }
  }
  return [
    ...directories.values(),
    ...fileEntries,
  ].sort((left, right) => {
    if (left.relativePath === right.relativePath) {
      return 0;
    }
    const leftIsParent = right.relativePath.startsWith(`${left.relativePath}/`);
    const rightIsParent = left.relativePath.startsWith(`${right.relativePath}/`);
    if (leftIsParent) {
      return -1;
    }
    if (rightIsParent) {
      return 1;
    }
    return left.relativePath.localeCompare(right.relativePath);
  });
}

function parentDirectoriesForTarPath(relativePath) {
  const normalized = String(relativePath).replaceAll('\\', '/').replace(/\/+$/u, '');
  const parts = normalized.split('/');
  parts.pop();
  const directories = [];
  for (let index = 1; index <= parts.length; index += 1) {
    directories.push(parts.slice(0, index).join('/'));
  }
  return directories.filter((directory) => directory && directory !== '.');
}

function debianDirectoryMode(directory) {
  if (directory === `.${LINUX_SERVICE_CONFIG_ROOT}` || directory === `.${LINUX_SERVICE_DATABASE_SECRET_ROOT}`) {
    return 0o750;
  }
  if (directory === `.${LINUX_SERVICE_DATA_ROOT}`
    || directory === `.${LINUX_SERVICE_LOG_ROOT}`
    || directory === `.${LINUX_SERVICE_CACHE_ROOT}`
    || directory === `.${LINUX_SERVICE_RUN_ROOT}`
    || directory === `.${LINUX_MODULE_ROOT}`) {
    return 0o750;
  }
  return 0o755;
}

// Deterministic ustar writer with uid/gid/mtime (the shared archive writer
// omits them; dpkg requires sane numeric fields).
function createDebTar(fileEntries) {
  const chunks = [];
  for (const entry of fileEntries) {
    const data = Buffer.from(entry.data);
    const header = createDebTarHeader(entry.relativePath, data.length, entry.mode ?? 0o644, entry.type ?? 'file');
    chunks.push(header, data, Buffer.alloc(paddingForTar(data.length)));
  }
  chunks.push(Buffer.alloc(1024));
  return Buffer.concat(chunks);
}

function createDebTarHeader(name, size, mode = 0o644, type = 'file') {
  const tarPath = splitDebTarPath(name);
  const header = Buffer.alloc(512, 0);
  Buffer.from(tarPath.name, 'utf8').copy(header, 0);
  Buffer.from(tarPath.prefix, 'utf8').copy(header, 345);
  writeTarOctal(header, 100, 8, mode);
  writeTarOctal(header, 108, 8, 0); // uid
  writeTarOctal(header, 116, 8, 0); // gid
  writeTarOctal(header, 124, 12, type === 'directory' ? 0 : size);
  writeTarOctal(header, 136, 12, 0); // mtime: reproducible
  header.fill(0x20, 148, 156);
  header[156] = type === 'directory' ? 0x35 : 0x30;
  Buffer.from('ustar\0', 'ascii').copy(header, 257);
  Buffer.from('00', 'ascii').copy(header, 263);
  const checksum = header.reduce((sum, byte) => sum + byte, 0);
  writeTarChecksum(header, checksum);
  return header;
}

function splitDebTarPath(name) {
  const normalized = String(name).replace(/^\.\/+/u, '').replace(/\/+$/u, '');
  const withDotSlash = normalized ? `./${normalized}` : './';
  if (Buffer.byteLength(withDotSlash, 'utf8') <= 100) {
    return { name: withDotSlash, prefix: '' };
  }
  const segments = normalized.split('/');
  for (let index = segments.length - 1; index > 0; index -= 1) {
    const prefix = segments.slice(0, index).join('/');
    const basename = segments.slice(index).join('/');
    if (Buffer.byteLength(prefix, 'utf8') <= 155 && Buffer.byteLength(`./${basename}`, 'utf8') <= 100) {
      return { name: `./${basename}`, prefix };
    }
  }
  throw new Error(`tar entry path is too long: ${name}`);
}

function writeTarOctal(buffer, offset, length, value) {
  const text = value.toString(8).padStart(length - 1, '0').slice(-(length - 1));
  buffer.write(text, offset, length - 1, 'ascii');
  buffer[offset + length - 1] = 0;
}

function writeTarChecksum(buffer, checksum) {
  const text = checksum.toString(8).padStart(6, '0').slice(-6);
  buffer.write(text, 148, 6, 'ascii');
  buffer[154] = 0;
  buffer[155] = 0x20;
}

function paddingForTar(size) {
  return (512 - (size % 512)) % 512;
}

function createArArchive(entries) {
  const chunks = [Buffer.from('!<arch>\n', 'ascii')];
  for (const entry of entries) {
    const data = Buffer.from(entry.data);
    const header = Buffer.alloc(60, 0x20);
    const name = `${entry.name}/`;
    if (Buffer.byteLength(name, 'ascii') > 16) {
      throw new Error(`ar entry name is too long: ${entry.name}`);
    }
    header.write(name.padEnd(16, ' '), 0, 16, 'ascii');
    header.write('0'.padEnd(12, ' '), 16, 12, 'ascii');
    header.write('0'.padEnd(6, ' '), 28, 6, 'ascii');
    header.write('0'.padEnd(6, ' '), 34, 6, 'ascii');
    header.write((entry.mode ?? 0o644).toString(8).padEnd(8, ' '), 40, 8, 'ascii');
    header.write(String(data.length).padEnd(10, ' '), 48, 10, 'ascii');
    header.write('`\n', 58, 2, 'ascii');
    chunks.push(header, data);
    if (data.length % 2 === 1) {
      chunks.push(Buffer.from('\n', 'ascii'));
    }
  }
  return Buffer.concat(chunks);
}

function debianVersion(version) {
  const normalized = String(version).replaceAll('_', '-').replace(/\+.*$/u, '');
  return /^[0-9]/u.test(normalized) ? normalized : `0${normalized}`;
}

function debianArchitecture(architecture) {
  return architecture === 'arm64' ? 'arm64' : 'amd64';
}

// --- Windows (.msi) --------------------------------------------------------

async function buildWindowsMsi(plan, fileEntries, { winswPath }) {
  if (process.platform !== 'win32') {
    throw new Error('Windows .msi builds require WiX on a Windows host');
  }
  const buildRoot = path.join(plan.outputDir, '.native-build', `${plan.package.id}-msi`);
  const payloadRoot = path.join(buildRoot, 'payload');
  await rm(buildRoot, { recursive: true, force: true });
  await mkdir(payloadRoot, { recursive: true });
  await writeMappedPackageFiles(payloadRoot, fileEntries, (entry) =>
    windowsPayloadPathForArchivePath(plan, entry.relativePath)
  );
  const wixSourcePath = path.join(buildRoot, 'sdkwork-chat.wxs');
  await writeFile(wixSourcePath, createWixSource(plan, payloadRoot, fileEntries), 'utf8');
  await execFileAsync('wix', [
    'build',
    wixSourcePath,
    '-arch',
    plan.package.architecture === 'arm64' ? 'arm64' : 'x64',
    '-pdbtype',
    'none',
    '-out',
    plan.installerPath,
  ], {
    cwd: repoRoot,
    maxBuffer: 1024 * 1024 * 16,
  });
  await rm(buildRoot, { recursive: true, force: true });
}

function windowsPayloadPathForArchivePath(plan, archivePath) {
  const normalized = String(archivePath).replaceAll('\\', '/');
  if (normalized.startsWith('config/')) {
    return `ProgramData/sdkwork/chat/${normalized.slice('config/'.length)}`;
  }
  if (normalized.startsWith('modules/')) {
    // Embedded modules under ProgramData: catalog materialization writes into
    // the module root (ProgramData is the mutable data tree).
    return `ProgramData/${WINDOWS_MODULE_ROOT}/${normalized.slice('modules/'.length)}`;
  }
  if (normalized === 'INSTALL.md') {
    return 'ProgramFiles/sdkwork/chat/doc/INSTALL.md';
  }
  if (normalized === INSTALL_MANIFEST_ARCHIVE_PATH) {
    return MSI_INSTALL_MANIFEST_PATH;
  }
  return `ProgramFiles/sdkwork/chat/${normalized}`;
}

function createWixSource(plan, payloadRoot, fileEntries) {
  const componentRefs = [];
  // Root nodes carry their own id as the path prefix so directory ids stay
  // unique across the ProgramFiles and ProgramData trees (both contain a
  // top-level `chat` directory).
  const programFilesTree = new DirectoryNode('PROGRAMFILESSDKWORK', 'sdkwork', 'PROGRAMFILESSDKWORK');
  const programDataTree = new DirectoryNode('COMMONAPPDATASDKWORK', 'sdkwork', 'COMMONAPPDATASDKWORK');
  const serviceComponentId = stableWixId('cmp', 'sdkwork/chat/service/windows/sdkwork-chat-service.exe');
  for (const entry of fileEntries) {
    const payloadPath = windowsPayloadPathForArchivePath(plan, entry.relativePath);
    if (!payloadPath) {
      continue;
    }
    const destination = windowsWixDestinationForPayloadPath(payloadPath, {
      programDataTree,
      programFilesTree,
    });
    const fileId = stableWixId('fil', payloadPath);
    const componentId = stableWixId('cmp', payloadPath);
    componentRefs.push(componentId);
    const source = path.join(payloadRoot, ...payloadPath.split('/'));
    destination.tree.addFile(destination.parts, {
      componentId,
      fileId,
      source,
      serviceInstall: componentId === serviceComponentId,
    });
  }

  return [
    '<?xml version="1.0" encoding="UTF-8"?>',
    '<Wix xmlns="http://wixtoolset.org/schemas/v4/wxs">',
    `  <Package Name="Sdkwork IM" Manufacturer="SDKWork" Version="${xmlEscape(windowsPackageVersion(plan.package.version))}" UpgradeCode="{${WINDOWS_UPGRADE_CODE}}" Scope="perMachine">`,
    '    <MajorUpgrade DowngradeErrorMessage="A newer version of Sdkwork IM is already installed." />',
    '    <MediaTemplate EmbedCab="yes" />',
    '    <StandardDirectory Id="ProgramFiles64Folder">',
    ...renderWixDirectory(programFilesTree, 3),
    '    </StandardDirectory>',
    ...(programDataTree.hasContent()
      ? [
        '    <StandardDirectory Id="CommonAppDataFolder">',
        ...renderWixDirectory(programDataTree, 3),
        '    </StandardDirectory>',
      ]
      : []),
    `    <Feature Id="MainFeature" Title="Sdkwork IM" Level="1">`,
    ...componentRefs.map((componentId) => `      <ComponentRef Id="${componentId}" />`),
    '    </Feature>',
    '  </Package>',
    '</Wix>',
    '',
  ].join('\n');
}

function windowsWixDestinationForPayloadPath(payloadPath, trees) {
  const parts = String(payloadPath).split('/');
  if (parts[0] === 'ProgramData') {
    return {
      tree: trees.programDataTree,
      parts: stripWindowsRootDirectoryName(parts.slice(1), 'sdkwork'),
    };
  }
  return {
    tree: trees.programFilesTree,
    parts: parts.slice(1),
  };
}

function stripWindowsRootDirectoryName(parts, expectedRootName) {
  return parts[0] === expectedRootName ? parts.slice(1) : parts;
}

class DirectoryNode {
  constructor(id, name, path = '') {
    this.id = id;
    this.name = name;
    this.path = path;
    this.directories = new Map();
    this.files = [];
  }

  addFile(parts, file) {
    if (parts.length === 0) {
      throw new Error(`Cannot add Wix file without destination path: ${file.source}`);
    }
    if (parts.length === 1) {
      this.files.push({ ...file, name: parts[0] });
      return;
    }
    const directoryName = parts[0];
    const childPath = this.path ? `${this.path}/${directoryName}` : directoryName;
    const directoryId = stableWixId('dir', childPath);
    if (!this.directories.has(directoryName)) {
      this.directories.set(directoryName, new DirectoryNode(directoryId, directoryName, childPath));
    }
    this.directories.get(directoryName).addFile(parts.slice(1), file);
  }

  hasContent() {
    return this.files.length > 0 || this.directories.size > 0;
  }
}

function renderWixDirectory(node, indentLevel) {
  const indent = '  '.repeat(indentLevel);
  const lines = [`${indent}<Directory Id="${node.id}" Name="${xmlEscape(node.name)}">`];
  for (const child of [...node.directories.values()].sort((left, right) => left.name.localeCompare(right.name))) {
    lines.push(...renderWixDirectory(child, indentLevel + 1));
  }
  for (const file of node.files.sort((left, right) => left.name.localeCompare(right.name))) {
    lines.push(`${indent}  <Component Id="${file.componentId}" Guid="*">`);
    lines.push(`${indent}    <File Id="${file.fileId}" Source="${xmlEscape(file.source)}" KeyPath="yes" />`);
    if (file.serviceInstall) {
      // WinSW.exe implements the SCM protocol and reads the adjacent
      // sdkwork-chat-service.xml; register the service on install and stop/
      // remove it on uninstall (PACKAGING_SPEC §5.5 Windows SCM row).
      lines.push(`${indent}    <ServiceInstall Name="${WINDOWS_SERVICE_NAME}" DisplayName="${WINDOWS_SERVICE_DISPLAY_NAME}" Type="ownProcess" Start="auto" ErrorControl="normal" />`);
      lines.push(`${indent}    <ServiceControl Id="${stableWixId('svc', WINDOWS_SERVICE_NAME)}" Name="${WINDOWS_SERVICE_NAME}" Stop="both" Remove="uninstall" Wait="yes" />`);
    }
    lines.push(`${indent}  </Component>`);
  }
  lines.push(`${indent}</Directory>`);
  return lines;
}

async function writeMappedPackageFiles(root, fileEntries, mapPath) {
  for (const entry of fileEntries) {
    const target = mapPath(entry);
    if (!target) {
      continue;
    }
    const safeTarget = String(target).replace(/^\/+/u, '');
    const targetPath = path.join(root, ...safeTarget.split('/'));
    await mkdir(path.dirname(targetPath), { recursive: true });
    await writeFile(targetPath, entry.data);
    if ((entry.mode ?? 0o644) & 0o111) {
      await chmod(targetPath, 0o755);
    }
  }
}

function createWinSwXml() {
  // WinSW reads the xml adjacent to the wrapper exe. Logs go to the
  // ProgramData Logs directory (RUNTIME_DIRECTORY_SPEC §4.4). The canonical
  // runtime environment is carried inline (WinSW has no env-file support);
  // secrets belong in the referenced secret files, never here.
  const moduleEnv = EMBEDDED_MODULE_WORKSPACES.map((workspace) =>
    `  <env name="SDKWORK_${workspace.replace('sdkwork-', '').toUpperCase()}_APP_ROOT" value="%ProgramData%\\sdkwork\\chat\\modules\\${workspace}" />`
  ).join('\r\n');
  return [
    '<service>',
    `  <id>${WINDOWS_SERVICE_NAME}</id>`,
    `  <name>${WINDOWS_SERVICE_DISPLAY_NAME}</name>`,
    '  <description>SDKWork IM standalone API gateway Windows service wrapper</description>',
    `  <executable>%ProgramFiles%\\sdkwork\\chat\\bin\\${SERVER_BINARY_BASENAME}.exe</executable>`,
    '  <logpath>%ProgramData%\\sdkwork\\chat\\Logs</logpath>',
    '  <log mode="roll" />',
    '',
    '  <env name="SDKWORK_IM_DEPLOYMENT_PROFILE" value="standalone" />',
    '  <env name="SDKWORK_IM_RUNTIME_TARGET" value="server" />',
    '  <env name="SDKWORK_IM_ENVIRONMENT" value="production" />',
    '  <env name="SDKWORK_IM_PROFILE_ID" value="standalone.production" />',
    '  <env name="SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND" value="0.0.0.0:18079" />',
    '  <env name="SDKWORK_IM_ID_NODE_ID" value="1" />',
    '  <env name="SDKWORK_DATABASE_ENGINE" value="postgresql" />',
    '  <env name="SDKWORK_DATABASE_SSL_MODE" value="require" />',
    '  <env name="SDKWORK_DATABASE_AUTO_MIGRATE" value="true" />',
    '  <env name="SDKWORK_DATABASE_TEMPORARY_DRIVER_POOL_COUNT" value="2" />',
    '  <env name="SDKWORK_DATABASE_TEMPORARY_ANY_POOL_EXCEPTION" value="true" />',
    '  <env name="SDKWORK_APP_ROOT" value="%ProgramFiles%\\sdkwork\\chat" />',
    '  <env name="SDKWORK_IM_APP_ROOT" value="%ProgramFiles%\\sdkwork\\chat" />',
    ...moduleEnv.split('\r\n'),
    '  <env name="SDKWORK_PAYMENT_CREDENTIAL_MASTER_KEY_FILE" value="%ProgramData%\\sdkwork\\chat\\Data\\secrets\\payment-credential-master.key" />',
    '  <env name="RUST_LOG" value="info" />',
    '',
    '  <onfailure action="restart" delay="10 sec" />',
    '</service>',
    '',
  ].join('\r\n');
}

async function resolveWinSwBytes(plan) {
  const cacheDir = path.join(repoRoot, 'dist', '.native-cache');
  const cachedPath = path.join(cacheDir, `WinSW-x64-${WINSW_VERSION}.exe`);
  const candidates = [
    plan.winswPath ? path.resolve(repoRoot, plan.winswPath) : null,
    process.env.SDKWORK_IM_WINSW_X64_FILE ? path.resolve(repoRoot, process.env.SDKWORK_IM_WINSW_X64_FILE) : null,
    cachedPath,
  ].filter(Boolean);

  for (const candidate of candidates) {
    if (!existsSync(candidate)) {
      continue;
    }
    const bytes = await readFile(candidate);
    if (verifyWinSwBytes(bytes, candidate)) {
      if (candidate !== cachedPath) {
        await mkdir(cacheDir, { recursive: true });
        await writeFile(cachedPath, bytes);
      }
      return bytes;
    }
  }

  // Pinned download with digest verification (SUPPLY_CHAIN_SECURITY_SPEC §5).
  await mkdir(cacheDir, { recursive: true });
  const temporaryPath = `${cachedPath}.download`;
  const proxy = String(process.env.HTTPS_PROXY ?? process.env.https_proxy ?? '').trim();
  const args = ['-fsSL'];
  if (proxy) {
    args.push('--proxy', proxy);
  }
  args.push('-o', temporaryPath, WINSW_DOWNLOAD_URL);
  try {
    await execFileAsync('curl', args, { maxBuffer: 1024 * 1024 });
  } catch (error) {
    throw new Error(
      `failed to download pinned WinSW ${WINSW_VERSION} (${WINSW_DOWNLOAD_URL}): `
      + `${error instanceof Error ? error.message : String(error)}. `
      + 'Pre-stage it via --winsw-path or SDKWORK_IM_WINSW_X64_FILE.',
    );
  }
  const bytes = await readFile(temporaryPath);
  if (!verifyWinSwBytes(bytes, temporaryPath)) {
    await rm(temporaryPath, { force: true });
    throw new Error(`pinned WinSW download failed digest verification: ${WINSW_DOWNLOAD_URL}`);
  }
  await writeFile(cachedPath, bytes);
  await rm(temporaryPath, { force: true });
  return bytes;
}

function verifyWinSwBytes(bytes, label) {
  if (bytes.length !== WINSW_X64_SIZE) {
    console.warn(`[sdkwork-im-native-installer] WinSW size mismatch at ${label}: ${bytes.length} != ${WINSW_X64_SIZE}`);
    return false;
  }
  return sha256(bytes) === WINSW_X64_SHA256;
}

// --- macOS (.pkg) ----------------------------------------------------------

async function buildMacosPkg(plan, fileEntries) {
  if (process.platform !== 'darwin') {
    throw new Error('macOS .pkg builds require pkgbuild on a macOS host');
  }
  const buildRoot = path.join(plan.outputDir, '.native-build', `${plan.package.id}-pkg`);
  const payloadRoot = path.join(buildRoot, 'payload');
  const scriptsRoot = path.join(buildRoot, 'scripts');
  await rm(buildRoot, { recursive: true, force: true });
  await mkdir(payloadRoot, { recursive: true });
  await mkdir(scriptsRoot, { recursive: true });
  await writeMappedPackageFiles(payloadRoot, fileEntries, (entry) =>
    macosPayloadPathForArchivePath(plan, entry.relativePath)
  );
  await writeFile(
    path.join(scriptsRoot, 'postinstall'),
    createMacosPostinstall(plan),
    { mode: 0o755 },
  );
  await execFileAsync('pkgbuild', [
    '--root',
    payloadRoot,
    '--scripts',
    scriptsRoot,
    '--identifier',
    'com.sdkwork.chat.server',
    '--version',
    windowsPackageVersion(plan.package.version),
    '--install-location',
    '/',
    plan.installerPath,
  ], {
    cwd: repoRoot,
    maxBuffer: 1024 * 1024 * 16,
  });
  await rm(buildRoot, { recursive: true, force: true });
}

function macosPayloadPathForArchivePath(plan, archivePath) {
  const normalized = String(archivePath).replaceAll('\\', '/');
  if (normalized.startsWith('config/')) {
    return `Library/Application Support/sdkwork/chat/${normalized.slice('config/'.length)}`;
  }
  if (normalized.startsWith('modules/')) {
    return `${MACOS_MODULE_ROOT}/${normalized.slice('modules/'.length)}`;
  }
  if (normalized === 'INSTALL.md') {
    return 'usr/share/doc/sdkwork/chat/INSTALL.md';
  }
  if (normalized === INSTALL_MANIFEST_ARCHIVE_PATH) {
    return 'usr/share/sdkwork/chat/install-manifest.json';
  }
  if (normalized.startsWith('bin/') || normalized.startsWith('web/') || normalized === APP_CONFIG_ARCHIVE_PATH) {
    return `usr/lib/sdkwork/chat/${normalized}`;
  }
  return null;
}

function createMacosPostinstall(plan) {
  return [
    '#!/bin/sh',
    'set -e',
    `mkdir -p "${MACOS_SERVICE_ROOT}"`,
    `chmod 0750 "${MACOS_SERVICE_ROOT}"`,
    `chown root:wheel "${MACOS_SERVICE_ROOT}"`,
    'cat <<\'EOF\'',
    '',
    'SDKWork IM installation summary',
    '-------------------------------',
    `Package: ${plan.package.id}`,
    `Config directory: ${MACOS_SERVICE_ROOT}`,
    `LaunchDaemon: ${MACOS_LAUNCH_DAEMON_PATH}`,
    '',
    'Before first start:',
    `  sudo editor "${MACOS_SERVICE_ROOT}/server.env"`,
    '  sudo launchctl load /Library/LaunchDaemons/com.sdkwork.chat.plist',
    '',
    'EOF',
    'exit 0',
    '',
  ].join('\n');
}

// --- Native install layout manifests --------------------------------------

function createNativeInstallLayout(packageItem) {
  if (packageItem.platform === 'linux') {
    return createLinuxNativeInstallLayout(packageItem);
  }
  if (packageItem.platform === 'windows') {
    return createWindowsNativeInstallLayout(packageItem);
  }
  return createMacosNativeInstallLayout(packageItem);
}

function createBaseNativeInstallLayout(packageItem, { format, installRoot, files, service = null, permissions = [], commands = {} }) {
  return {
    schemaVersion: NATIVE_INSTALL_LAYOUT_SCHEMA_VERSION,
    packageId: packageItem.id,
    platform: packageItem.platform,
    distribution: packageItem.distribution,
    architecture: packageItem.architecture,
    profile: packageItem.profile,
    runtimeTarget: packageItem.runtimeTarget,
    format,
    installRoot,
    files,
    service,
    permissions,
    commands,
  };
}

function createLinuxNativeInstallLayout(packageItem) {
  const binaryName = packageItem.binaryName;
  return createBaseNativeInstallLayout(packageItem, {
    format: 'deb',
    installRoot: LINUX_NATIVE_INSTALL_ROOT,
    files: {
      binary: `${LINUX_NATIVE_INSTALL_ROOT}/bin/${binaryName}`,
      web: `${LINUX_NATIVE_SHARED_ROOT}/web`,
      documentation: `${LINUX_NATIVE_SHARED_DOC_ROOT}/INSTALL.md`,
      installManifest: DEB_INSTALL_MANIFEST_PATH,
      appManifest: `${LINUX_NATIVE_INSTALL_ROOT}/${APP_CONFIG_ARCHIVE_PATH}`,
      configDir: LINUX_SERVICE_CONFIG_ROOT,
      serviceEnvironment: LINUX_SERVICE_ENV_FILE,
      configTemplates: `${LINUX_SERVICE_CONFIG_ROOT}/*.example`,
      passwordFile: LINUX_SERVICE_DATABASE_SECRET_FILE,
      systemdUnit: LINUX_SYSTEMD_UNIT_PATH,
      modules: LINUX_MODULE_ROOT,
    },
    service: {
      manager: 'systemd',
      name: 'sdkwork-chat.service',
      unitPath: LINUX_SYSTEMD_UNIT_PATH,
      enabledOnInstall: true,
      startedOnInstall: false,
    },
    permissions: [
      { path: LINUX_NATIVE_INSTALL_ROOT, owner: 'root', group: 'root', mode: '0755' },
      { path: `${LINUX_NATIVE_INSTALL_ROOT}/bin`, owner: 'root', group: 'root', mode: '0755' },
      { path: `${LINUX_NATIVE_INSTALL_ROOT}/bin/${binaryName}`, owner: 'root', group: 'root', mode: '0755' },
      { path: LINUX_NATIVE_SHARED_ROOT, owner: 'root', group: 'root', mode: '0755' },
      { path: LINUX_NATIVE_SHARED_DOC_ROOT, owner: 'root', group: 'root', mode: '0755' },
      { path: LINUX_SERVICE_CONFIG_ROOT, owner: 'root', group: 'sdkwork', mode: '0750' },
      { path: LINUX_SERVICE_ENV_FILE, owner: 'root', group: 'sdkwork', mode: '0640' },
      { path: `${LINUX_SERVICE_CONFIG_ROOT}/*.example`, owner: 'root', group: 'sdkwork', mode: '0640' },
      { path: LINUX_SERVICE_DATABASE_SECRET_ROOT, owner: 'root', group: 'sdkwork', mode: '0750' },
      { path: LINUX_SERVICE_DATABASE_SECRET_FILE, owner: 'root', group: 'sdkwork', mode: '0640' },
      { path: LINUX_SERVICE_DATA_ROOT, owner: 'sdkwork', group: 'sdkwork', mode: '0750' },
      { path: LINUX_SERVICE_LOG_ROOT, owner: 'sdkwork', group: 'sdkwork', mode: '0750' },
      { path: LINUX_SERVICE_CACHE_ROOT, owner: 'sdkwork', group: 'sdkwork', mode: '0750' },
      { path: LINUX_SERVICE_RUN_ROOT, owner: 'sdkwork', group: 'sdkwork', mode: '0750' },
    ],
    commands: {
      configure: [
        `sudo editor ${LINUX_SERVICE_ENV_FILE}`,
        `sudo editor ${LINUX_SERVICE_DATABASE_SECRET_FILE}`,
      ],
      start: 'sudo systemctl start sdkwork-chat',
      status: 'sudo systemctl status sdkwork-chat --no-pager',
      logs: 'sudo journalctl -u sdkwork-chat -f',
    },
  });
}

function createWindowsNativeInstallLayout(packageItem) {
  const binaryName = packageItem.binaryName;
  return createBaseNativeInstallLayout(packageItem, {
    format: 'msi',
    installRoot: '%ProgramFiles%/sdkwork/chat',
    files: {
      binary: `%ProgramFiles%\\sdkwork\\chat\\bin\\${binaryName}`,
      web: '%ProgramFiles%\\sdkwork\\chat\\web',
      documentation: '%ProgramFiles%\\sdkwork\\chat\\doc\\INSTALL.md',
      installManifest: '%ProgramFiles%\\sdkwork\\chat\\install-manifest.json',
      appManifest: '%ProgramFiles%\\sdkwork\\chat\\sdkwork.app.config.json',
      configDir: '%ProgramData%\\sdkwork\\chat',
      serviceEnvironment: '%ProgramData%\\sdkwork\\chat\\server.env',
      configTemplates: '%ProgramData%\\sdkwork\\chat\\*.example',
      serviceWrapper: `%ProgramFiles%\\sdkwork\\chat\\service\\windows\\${WINDOWS_SERVICE_NAME}-service.exe`,
      serviceConfig: `%ProgramFiles%\\sdkwork\\chat\\service\\windows\\${WINDOWS_SERVICE_NAME}-service.xml`,
      dataDirectory: '%ProgramData%\\sdkwork\\chat\\Data',
      logDirectory: '%ProgramData%\\sdkwork\\chat\\Logs',
      modules: '%ProgramData%\\sdkwork\\chat\\modules',
      paymentMasterKey: '%ProgramData%\\sdkwork\\chat\\Data\\secrets\\payment-credential-master.key',
    },
    service: {
      manager: 'windows-service',
      name: WINDOWS_SERVICE_NAME,
      displayName: WINDOWS_SERVICE_DISPLAY_NAME,
      registeredOnInstall: true,
      startedOnInstall: true,
    },
    permissions: [],
    commands: {
      start: 'sc start sdkwork-chat',
      status: 'sc query sdkwork-chat',
      logs: 'Get-Content -Tail 100 "$env:ProgramData\\sdkwork\\chat\\Logs\\*.log" -Wait',
    },
  });
}

function createMacosNativeInstallLayout(packageItem) {
  const binaryName = packageItem.binaryName;
  return createBaseNativeInstallLayout(packageItem, {
    format: 'pkg',
    installRoot: MACOS_INSTALL_ROOT,
    files: {
      binary: `${MACOS_INSTALL_ROOT}/bin/${binaryName}`,
      web: `${MACOS_INSTALL_ROOT}/web`,
      documentation: '/usr/share/doc/sdkwork/chat/INSTALL.md',
      installManifest: '/usr/share/sdkwork/chat/install-manifest.json',
      appManifest: `${MACOS_INSTALL_ROOT}/${APP_CONFIG_ARCHIVE_PATH}`,
      configDir: MACOS_SERVICE_ROOT,
      launchDaemon: MACOS_LAUNCH_DAEMON_PATH,
      dataDirectory: `${MACOS_SERVICE_ROOT}/Data`,
      modules: MACOS_MODULE_ROOT,
    },
    service: {
      manager: 'launchd',
      name: 'com.sdkwork.chat',
      unitPath: MACOS_LAUNCH_DAEMON_PATH,
      enabledOnInstall: false,
      startedOnInstall: false,
    },
    permissions: [
      { path: MACOS_SERVICE_ROOT, owner: 'root', group: 'wheel', mode: '0750' },
      { path: MACOS_INSTALL_ROOT, owner: 'root', group: 'wheel', mode: '0755' },
    ],
    commands: {
      start: 'sudo launchctl load /Library/LaunchDaemons/com.sdkwork.chat.plist',
      logs: 'sudo log stream --predicate \'process == "sdkwork-api-im-standalone-gateway"\'',
    },
  });
}

// --- Snapshot cache (PACKAGING_SPEC §4) ------------------------------------

async function collectNativeInstallerSnapshot(plan) {
  const entries = [];
  for (const entry of plan.archiveBuildPlan.entries) {
    if (entry.archivePath === INSTALL_MANIFEST_ARCHIVE_PATH) {
      // The packaged manifest is regenerated per build; capture the staged
      // digest so staging changes invalidate the cache.
      try {
        const staged = await readStagedInstallManifest(plan);
        entries.push({
          path: entry.archivePath,
          size: Buffer.byteLength(staged, 'utf8'),
          mtimeMs: 0,
          generatedDigest: sha256(Buffer.from(staged, 'utf8')),
        });
      } catch {
        entries.push({ path: entry.archivePath, size: -1, mtimeMs: -1 });
      }
      continue;
    }
    if (isSkippedNativeStagingEntry(plan, entry.archivePath)) {
      continue;
    }
    let stat;
    try {
      stat = await statFile(entry.sourcePath);
    } catch {
      entries.push({ path: entry.archivePath, size: -1, mtimeMs: -1 });
      continue;
    }
    entries.push({
      path: entry.archivePath,
      size: stat.size,
      mtimeMs: stat.mtimeMs,
    });
  }
  // App manifest + IM database module + embedded modules are packaged from
  // source trees (not the staged dir); snapshot their file stats so changes
  // invalidate the cache.
  const appConfigStat = await statFile(path.join(repoRoot, 'sdkwork.app.config.json'));
  entries.push({
    path: APP_CONFIG_ARCHIVE_PATH,
    size: appConfigStat.size,
    mtimeMs: appConfigStat.mtimeMs,
  });
  for (const filePath of listFiles(path.join(repoRoot, IM_DATABASE_MODULE_ARCHIVE_PATH))) {
    const stat = await statFile(filePath);
    const relative = path.relative(
      path.join(repoRoot, IM_DATABASE_MODULE_ARCHIVE_PATH),
      filePath,
    ).replaceAll('\\', '/');
    entries.push({
      path: `${IM_DATABASE_MODULE_ARCHIVE_PATH}/${relative}`,
      size: stat.size,
      mtimeMs: stat.mtimeMs,
    });
  }
  for (const moduleEntry of await collectEmbeddedModuleEntries()) {
    const sourcePath = moduleSourcePathFor(moduleEntry.relativePath);
    try {
      const stat = await statFile(sourcePath);
      entries.push({
        path: moduleEntry.relativePath,
        size: stat.size,
        mtimeMs: stat.mtimeMs,
      });
    } catch {
      entries.push({ path: moduleEntry.relativePath, size: -1, mtimeMs: -1 });
    }
  }
  return {
    schemaVersion: SNAPSHOT_SCHEMA_VERSION,
    packageId: plan.package.id,
    version: plan.package.version,
    nativeFormat: plan.nativeFormat,
    winswVersion: WINSW_VERSION,
    // Generated content (postinst env template, systemd unit, WinSW xml)
    // lives in this module; any builder change must invalidate the cache.
    builderDigest: moduleSourceDigest(),
    entries: entries.sort((left, right) => left.path.localeCompare(right.path)),
  };
}

function moduleSourceDigest() {
  try {
    return sha256(readFileSync(fileURLToPath(import.meta.url)));
  } catch {
    return 'unavailable';
  }
}

// Reconstructs the source path of a packaged module entry
// (modules/<workspace>/database/... or modules/<workspace>/<extraDir>/...
// -> ../<workspace>/<same relative tree>).
function moduleSourcePathFor(relativePath) {
  const parts = String(relativePath).split('/');
  if (parts[0] !== 'modules' || parts.length < 3) {
    throw new Error(`invalid module archive path: ${relativePath}`);
  }
  const workspace = parts[1];
  const rest = parts.slice(2).join('/');
  return path.join(repoRoot, '..', workspace, rest);
}

async function nativeInstallerCacheHits(plan, snapshot) {
  try {
    await access(plan.installerPath);
    await access(plan.manifestPath);
  } catch {
    return false;
  }
  let previous;
  try {
    previous = JSON.parse(await readFile(plan.snapshotPath, 'utf8'));
  } catch {
    return false;
  }
  if (previous.schemaVersion !== snapshot.schemaVersion
    || previous.packageId !== snapshot.packageId
    || previous.version !== snapshot.version
    || previous.nativeFormat !== snapshot.nativeFormat
    || previous.winswVersion !== snapshot.winswVersion
    || previous.builderDigest !== snapshot.builderDigest) {
    return false;
  }
  const previousEntries = new Map(previous.entries.map((entry) => [entry.path, entry]));
  for (const entry of snapshot.entries) {
    const prior = previousEntries.get(entry.path);
    if (!prior) {
      return false;
    }
    if (prior.size !== entry.size || prior.mtimeMs !== entry.mtimeMs) {
      return false;
    }
    if (prior.generatedDigest !== entry.generatedDigest) {
      return false;
    }
  }
  const installerStat = await statFile(plan.installerPath);
  return installerStat.size > 0;
}

// --- Shared helpers --------------------------------------------------------

async function writeSha256Sums(outputDir) {
  const { readdirSync } = await import('node:fs');
  const lines = [];
  if (!existsSync(outputDir)) {
    return;
  }
  for (const entry of readdirSync(outputDir, { withFileTypes: true }).sort((left, right) => left.name.localeCompare(right.name))) {
    if (!entry.isFile() || entry.name === SHA256SUMS_FILE || entry.name.startsWith('.native-build')) {
      continue;
    }
    const filePath = path.join(outputDir, entry.name);
    lines.push(`${sha256(readFileSync(filePath))}  ${entry.name}`);
  }
  await writeFile(path.join(outputDir, SHA256SUMS_FILE), `${lines.join('\n')}${lines.length > 0 ? '\n' : ''}`, 'utf8');
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

function stableWixId(prefix, value) {
  const digest = createHash('sha1').update(value).digest('hex').slice(0, 20);
  return `${prefix}${digest}`;
}

function xmlEscape(value) {
  return String(value)
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;');
}

function windowsPackageVersion(version) {
  const parts = String(version).split('-')[0].split('.');
  return [parts[0] ?? '0', parts[1] ?? '0', parts[2] ?? '0'].map((part) => parseInt(part, 10) || 0).join('.');
}

function renderNativeInstallerBuildPlan(buildPlan) {
  return [
    `[sdkwork-im-native-installer] package: ${buildPlan.package.id}`,
    `[sdkwork-im-native-installer] installer: ${buildPlan.installerPath}`,
    `[sdkwork-im-native-installer] manifest: ${buildPlan.manifestPath}`,
    `[sdkwork-im-native-installer] format: ${buildPlan.nativeFormat}`,
    `[sdkwork-im-native-installer] staging: ${buildPlan.stagingRoot}`,
    `[sdkwork-im-native-installer] entries: ${buildPlan.archiveBuildPlan.entries.length}`,
    ...buildPlan.archiveBuildPlan.entries.map((entry) => `[sdkwork-im-native-installer]   ${entry.archivePath}`),
  ];
}

function printIssues(issues) {
  if (issues.length === 0) {
    return;
  }
  console.error('[sdkwork-im-native-installer] validation issues:');
  for (const issue of issues) {
    console.error(`[sdkwork-im-native-installer]   ${issue}`);
  }
}

async function main(argv = process.argv.slice(2)) {
  const settings = parseArgs(argv);
  if (settings.help) {
    printHelp();
    return 0;
  }
  if (settings.all) {
    return await runAllNativeBuilds(settings);
  }

  const buildPlan = createNativeInstallerBuildPlan({
    packageId: settings.packageId,
    outputDir: settings.outputDir,
    requireStagedFiles: !settings.dryRun,
    stagingRoot: settings.stagingRoot,
    version: settings.version,
    winswPath: settings.winswPath,
  });
  const issues = validateNativeInstallerBuildPlan(buildPlan);
  if (settings.json && (settings.check || settings.dryRun)) {
    console.log(JSON.stringify({
      ok: issues.length === 0,
      issues,
      plan: buildPlan,
    }, null, 2));
  } else if (!settings.json) {
    for (const line of renderNativeInstallerBuildPlan(buildPlan)) {
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

  const result = await buildNativeInstaller(buildPlan);
  if (settings.json) {
    console.log(JSON.stringify({
      ok: true,
      installer: result.installer,
      manifestPath: result.manifestPath,
      aggregateManifestPath: result.aggregateManifestPath,
    }, null, 2));
  } else {
    console.log(`[sdkwork-im-native-installer] written: ${result.installerPath}`);
    console.log(`[sdkwork-im-native-installer] sha256: ${result.installer.sha256}`);
  }
  return 0;
}

async function runAllNativeBuilds(settings) {
  const nativePlan = createSdkworkImNativeInstallPackagePlan({ version: settings.version });
  const hostPlatform = process.platform === 'win32' ? 'windows' : process.platform === 'darwin' ? 'macos' : 'linux';
  const packageIds = nativePlan.packages
    .filter((item) => item.profile === 'server' && item.platform === hostPlatform)
    .map((item) => item.id);
  const plans = packageIds.map((packageId) => createNativeInstallerBuildPlan({
    packageId,
    outputDir: settings.outputDir,
    requireStagedFiles: !settings.dryRun,
    root: repoRoot,
    stagingRoot: settings.stagingRoot,
    version: settings.version,
    winswPath: settings.winswPath,
  }));
  const issues = plans.flatMap((plan) =>
    validateNativeInstallerBuildPlan(plan).map((issue) => `${plan.package.id}: ${issue}`)
  );
  if (settings.json && (settings.check || settings.dryRun)) {
    console.log(JSON.stringify({
      ok: issues.length === 0,
      issues,
      plans,
    }, null, 2));
  } else if (!settings.json) {
    for (const plan of plans) {
      for (const line of renderNativeInstallerBuildPlan(plan)) {
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
  const results = [];
  for (const plan of plans) {
    results.push(await buildNativeInstaller(plan));
  }
  if (settings.json) {
    console.log(JSON.stringify({
      ok: true,
      installers: results.map((result) => result.installer),
    }, null, 2));
  }
  return 0;
}

if (process.argv[1] && import.meta.url.endsWith(process.argv[1].replaceAll('\\', '/'))) {
  main().then((code) => {
    process.exitCode = code;
  }).catch((error) => {
    console.error(`[sdkwork-im-native-installer] ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  });
}

export {
  LINUX_SERVICE_CONFIG_ROOT,
  LINUX_SERVICE_DATABASE_SECRET_FILE,
  LINUX_SYSTEMD_UNIT_PATH,
  NATIVE_INSTALL_LAYOUT_SCHEMA_VERSION,
  NATIVE_INSTALLER_SCHEMA_VERSION,
  WINDOWS_SERVICE_NAME,
  WINDOWS_UPGRADE_CODE,
  WINSW_VERSION,
  buildNativeInstaller,
  createDebianPackage,
  createNativeInstallerBuildPlan,
  createNativeInstallLayout,
  createSystemdUnit,
  createWixSource,
  main,
  parseArgs,
  renderNativeInstallerBuildPlan,
  validateNativeInstallerBuildPlan,
};
