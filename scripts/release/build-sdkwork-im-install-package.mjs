#!/usr/bin/env node

import { createHash } from 'node:crypto';
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';
import { gzipSync } from 'node:zlib';

import { DEFAULT_RELEASE_VERSION } from './sdkwork-im-release-version.mjs';
import { createSdkworkImInstallPackagePlan, validateSdkworkImInstallPackagePlan } from './plan-sdkwork-im-install-packages.mjs';
import { currentHostServerPackageId } from './stage-sdkwork-im-release-package.mjs';

const __filename = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(__filename), '..', '..');
const BUILD_SCHEMA_VERSION = '2026-06-04.sdkwork-im.install-package-build.v1';
const PACKAGE_MANIFEST_SCHEMA_VERSION = '2026-06-04.sdkwork-im.release-package-manifest.v1';
const AGGREGATE_MANIFEST_SCHEMA_VERSION = '2026-06-04.sdkwork-im.release-packages-manifest.v1';
const AGGREGATE_MANIFEST_FILE = 'release-packages-manifest.json';
const SHA256SUMS_FILE = 'SHA256SUMS';
const ZIP_DATE = new Date('2026-01-01T00:00:00Z');

const CRC32_TABLE = new Uint32Array(256);
for (let index = 0; index < 256; index += 1) {
  let value = index;
  for (let bit = 0; bit < 8; bit += 1) {
    value = (value & 1) ? (0xedb88320 ^ (value >>> 1)) : (value >>> 1);
  }
  CRC32_TABLE[index] = value >>> 0;
}

function printHelp() {
  console.log(`Usage: node scripts/release/build-sdkwork-im-install-package.mjs [options]

Build Sdkwork IM release archives from staged server or desktop package files.

Options:
  --package-id <id>       Package id from the release package plan.
  --all                   Validate or build all package ids.
  --staging-root <dir>    Staging root (default dist/release-staging/<package-id>).
  --output-dir <dir>      Output directory (default dist/release-packages).
  --version <value>       Package version (default ${DEFAULT_RELEASE_VERSION}).
  --check                 Validate the package build plan.
  --dry-run               Print the package build plan without writing archives.
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

function parseInstallPackageBuildArgs(argv = process.argv.slice(2)) {
  const settings = {
    all: false,
    check: false,
    dryRun: false,
    help: false,
    json: false,
    outputDir: null,
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
      case '--output-dir':
        settings.outputDir = requireValue(argv, index, arg);
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
        throw new Error(`Unsupported install package build option: ${arg}`);
    }
  }

  return settings;
}

function createSdkworkImInstallPackageBuildPlan({
  packageId = currentHostServerPackageId(),
  outputDir = null,
  requireStagedFiles = true,
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
  const absoluteOutputDir = path.resolve(root, outputDir ?? path.join('dist', 'release-packages'));
  const entries = existsSync(absoluteStagingRoot)
    ? collectArchiveEntries(absoluteStagingRoot, absoluteStagingRoot)
    : createPlannedArchiveEntries(packageItem, absoluteStagingRoot, { requireStagedFiles });
  const archivePath = path.join(absoluteOutputDir, packageItem.archiveName);
  const manifestPath = path.join(
    absoluteOutputDir,
    packageItem.archiveName.replace(/\.(zip|tar\.gz)$/u, '.manifest.json'),
  );

  return {
    schemaVersion: BUILD_SCHEMA_VERSION,
    package: packageItem,
    root,
    stagingRoot: absoluteStagingRoot,
    outputDir: absoluteOutputDir,
    archivePath,
    manifestPath,
    aggregateManifestPath: path.join(absoluteOutputDir, AGGREGATE_MANIFEST_FILE),
    sha256SumsPath: path.join(absoluteOutputDir, SHA256SUMS_FILE),
    entries,
  };
}

function createPlannedArchiveEntries(packageItem, stagingRoot, { requireStagedFiles }) {
  const plannedPaths = new Set();
  if (packageItem.deploymentMode === 'desktop') {
    plannedPaths.add('desktop');
    plannedPaths.add('desktop-manifest.json');
  } else {
    for (const artifact of packageItem.artifacts) {
      if (artifact.path === 'bin') {
        plannedPaths.add(`bin/${packageItem.binaryName}`);
        plannedPaths.add('bin/start-server.ps1');
        plannedPaths.add('bin/start-server.sh');
        plannedPaths.add('bin/install-server.ps1');
        plannedPaths.add('bin/install-server.sh');
      } else if (artifact.path === 'service') {
        plannedPaths.add('service/linux/sdkwork-im-server.service');
        plannedPaths.add('service/macos/com.sdkwork.im.server.plist');
        plannedPaths.add('service/windows/SdkworkImServer.xml');
      } else {
        plannedPaths.add(artifact.path);
      }
    }
  }
  return [...plannedPaths].sort().map((archivePath) => ({
    archivePath: normalizeArchivePath(archivePath),
    sourcePath: path.join(stagingRoot, archivePath),
    mode: modeForArchivePath(archivePath),
    missing: requireStagedFiles,
    planned: true,
    required: true,
  }));
}

function collectArchiveEntries(currentDir, rootDir) {
  const entries = [];
  for (const entry of readdirSync(currentDir, { withFileTypes: true }).sort((left, right) => left.name.localeCompare(right.name))) {
    const absolutePath = path.join(currentDir, entry.name);
    const archivePath = normalizeArchivePath(path.relative(rootDir, absolutePath));
    if (isSensitiveArchivePath(archivePath)) {
      continue;
    }
    if (entry.isDirectory()) {
      entries.push(...collectArchiveEntries(absolutePath, rootDir));
      continue;
    }
    if (!entry.isFile()) {
      continue;
    }
    entries.push({
      archivePath,
      sourcePath: absolutePath,
      mode: modeForArchivePath(archivePath),
      required: true,
    });
  }
  return entries.sort((left, right) => left.archivePath.localeCompare(right.archivePath));
}

function validateSdkworkImInstallPackageBuildPlan(buildPlan) {
  const issues = [];
  if (buildPlan.schemaVersion !== BUILD_SCHEMA_VERSION) {
    issues.push(`schemaVersion must be ${BUILD_SCHEMA_VERSION}`);
  }
  if (!buildPlan.package?.id) {
    issues.push('package id is required');
  }
  if (!buildPlan.archivePath || !buildPlan.archivePath.endsWith(buildPlan.package.archiveName)) {
    issues.push('archivePath must end with package archiveName');
  }
  if (!Array.isArray(buildPlan.entries) || buildPlan.entries.length === 0) {
    issues.push(`${buildPlan.package?.id ?? '(unknown)'} must include archive entries`);
    return issues;
  }
  for (const entry of buildPlan.entries) {
    try {
      normalizeArchivePath(entry.archivePath);
    } catch (error) {
      issues.push(error instanceof Error ? error.message : String(error));
      continue;
    }
    if (isSensitiveArchivePath(entry.archivePath)) {
      issues.push(`${buildPlan.package.id} must not package sensitive path ${entry.archivePath}`);
    }
    if (entry.required && entry.missing) {
      issues.push(`${buildPlan.package.id} requires staged artifact ${entry.archivePath}`);
    }
    if (entry.sourcePath && !isPathInside(entry.sourcePath, buildPlan.stagingRoot) && !samePath(entry.sourcePath, buildPlan.stagingRoot)) {
      issues.push(`${buildPlan.package.id} source path escapes staging root: ${entry.sourcePath}`);
    }
  }
  if (buildPlan.package.deploymentMode === 'server-archive') {
    for (const requiredPath of [
      `bin/${buildPlan.package.binaryName}`,
      'config/chat.toml.example',
      'config/server.env.example',
      'config/postgresql.yaml.example',
      'INSTALL.md',
      'install-manifest.json',
    ]) {
      if (!buildPlan.entries.some((entry) => entry.archivePath === requiredPath || entry.archivePath.startsWith(`${requiredPath}/`))) {
        issues.push(`${buildPlan.package.id} must include ${requiredPath}`);
      }
    }
  }
  if (buildPlan.package.deploymentMode === 'desktop') {
    if (!buildPlan.entries.some((entry) => entry.archivePath === 'desktop-manifest.json')) {
      issues.push(`${buildPlan.package.id} must include desktop-manifest.json`);
    }
    if (!buildPlan.entries.some((entry) => entry.archivePath === 'desktop' || entry.archivePath.startsWith('desktop/'))) {
      issues.push(`${buildPlan.package.id} must include desktop installer files`);
    }
  }
  return issues;
}

async function buildSdkworkImInstallPackageArchive(buildPlan) {
  const issues = validateSdkworkImInstallPackageBuildPlan(buildPlan);
  if (issues.length > 0) {
    throw new Error(`install package build plan is invalid: ${issues.join('; ')}`);
  }
  await mkdir(buildPlan.outputDir, { recursive: true });
  const fileEntries = [];
  const files = [];
  for (const entry of buildPlan.entries) {
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

  const archiveBytes = createInstallArchiveBytes(buildPlan, fileEntries);
  await writeFile(buildPlan.archivePath, archiveBytes);

  const archive = {
    file: path.basename(buildPlan.archivePath),
    packageId: buildPlan.package.id,
    version: buildPlan.package.version,
    platform: buildPlan.package.platform,
    architecture: buildPlan.package.architecture,
    deploymentMode: buildPlan.package.deploymentMode,
    size: archiveBytes.length,
    sha256: sha256(archiveBytes),
  };
  const manifest = createPackageBuildManifest(buildPlan, archive, files);
  await writeFile(buildPlan.manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, 'utf8');
  const aggregateManifest = createAggregateManifest(buildPlan, archive);
  await writeFile(buildPlan.aggregateManifestPath, `${JSON.stringify(aggregateManifest, null, 2)}\n`, 'utf8');
  await writeSha256Sums(buildPlan.outputDir, buildPlan.sha256SumsPath);

  return {
    archive,
    archivePath: buildPlan.archivePath,
    manifest,
    manifestPath: buildPlan.manifestPath,
    aggregateManifest,
    aggregateManifestPath: buildPlan.aggregateManifestPath,
    sha256SumsPath: buildPlan.sha256SumsPath,
  };
}

function createPackageBuildManifest(buildPlan, archive, files) {
  return {
    schemaVersion: PACKAGE_MANIFEST_SCHEMA_VERSION,
    generatedAt: manifestTimestamp(),
    product: 'chat',
    package: {
      id: buildPlan.package.id,
      archiveName: buildPlan.package.archiveName,
      version: buildPlan.package.version,
      platform: buildPlan.package.platform,
      architecture: buildPlan.package.architecture,
      deploymentMode: buildPlan.package.deploymentMode,
      runtimeProfile: buildPlan.package.runtimeProfile,
    },
    archive,
    files,
  };
}

function createAggregateManifest(buildPlan, archive) {
  const existingArchives = readExistingAggregateArchives(buildPlan.aggregateManifestPath);
  const archivesByPackageId = new Map();
  for (const existingArchive of existingArchives) {
    archivesByPackageId.set(existingArchive.packageId, existingArchive);
  }
  archivesByPackageId.set(archive.packageId, archive);
  return {
    schemaVersion: AGGREGATE_MANIFEST_SCHEMA_VERSION,
    generatedAt: manifestTimestamp(),
    product: 'chat',
    packageName: 'sdkwork-chat',
    archives: [...archivesByPackageId.values()].sort((left, right) => left.packageId.localeCompare(right.packageId)),
  };
}

function readExistingAggregateArchives(aggregateManifestPath) {
  if (!existsSync(aggregateManifestPath)) {
    return [];
  }
  try {
    const payload = JSON.parse(readFileSync(aggregateManifestPath, 'utf8'));
    if (payload?.schemaVersion !== AGGREGATE_MANIFEST_SCHEMA_VERSION || !Array.isArray(payload.archives)) {
      return [];
    }
    return payload.archives.filter((archive) =>
      archive && typeof archive.packageId === 'string' && typeof archive.file === 'string'
    );
  } catch {
    return [];
  }
}

async function writeSha256Sums(outputDir, sha256SumsPath) {
  const lines = [];
  if (!existsSync(outputDir)) {
    return;
  }
  for (const entry of readdirSync(outputDir, { withFileTypes: true }).sort((left, right) => left.name.localeCompare(right.name))) {
    if (!entry.isFile() || entry.name === SHA256SUMS_FILE) {
      continue;
    }
    const filePath = path.join(outputDir, entry.name);
    lines.push(`${sha256(readFileSync(filePath))}  ${entry.name}`);
  }
  await writeFile(sha256SumsPath, `${lines.join('\n')}${lines.length > 0 ? '\n' : ''}`, 'utf8');
}

function createInstallArchiveBytes(buildPlan, fileEntries) {
  if (buildPlan.package.archiveName.endsWith('.zip')) {
    return createZip(fileEntries);
  }
  if (buildPlan.package.archiveName.endsWith('.tar.gz')) {
    return gzipSync(createTar(fileEntries), { mtime: 0 });
  }
  throw new Error(`Unsupported archive extension: ${buildPlan.package.archiveName}`);
}

function createZip(entries) {
  const fileRecords = [];
  const chunks = [];
  let offset = 0;
  for (const entry of entries) {
    const relativePath = normalizeArchivePath(entry.relativePath);
    const name = Buffer.from(relativePath, 'utf8');
    const data = Buffer.from(entry.data);
    const crc = crc32(data);
    const localHeader = Buffer.alloc(30);
    localHeader.writeUInt32LE(0x04034b50, 0);
    localHeader.writeUInt16LE(20, 4);
    localHeader.writeUInt16LE(0x0800, 6);
    localHeader.writeUInt16LE(0, 8);
    writeDosDateTime(localHeader, 10, ZIP_DATE);
    localHeader.writeUInt32LE(crc, 14);
    localHeader.writeUInt32LE(data.length, 18);
    localHeader.writeUInt32LE(data.length, 22);
    localHeader.writeUInt16LE(name.length, 26);
    localHeader.writeUInt16LE(0, 28);
    chunks.push(localHeader, name, data);
    fileRecords.push({
      name,
      crc,
      mode: entry.mode ?? modeForArchivePath(relativePath),
      offset,
      size: data.length,
    });
    offset += localHeader.length + name.length + data.length;
  }

  const centralDirectoryOffset = offset;
  for (const record of fileRecords) {
    const header = Buffer.alloc(46);
    header.writeUInt32LE(0x02014b50, 0);
    header.writeUInt16LE(20, 4);
    header.writeUInt16LE(20, 6);
    header.writeUInt16LE(0x0800, 8);
    header.writeUInt16LE(0, 10);
    writeDosDateTime(header, 12, ZIP_DATE);
    header.writeUInt32LE(record.crc, 16);
    header.writeUInt32LE(record.size, 20);
    header.writeUInt32LE(record.size, 24);
    header.writeUInt16LE(record.name.length, 28);
    header.writeUInt16LE(0, 30);
    header.writeUInt16LE(0, 32);
    header.writeUInt16LE(0, 34);
    header.writeUInt16LE(0, 36);
    header.writeUInt32LE((record.mode & 0xffff) << 16, 38);
    header.writeUInt32LE(record.offset, 42);
    chunks.push(header, record.name);
    offset += header.length + record.name.length;
  }

  const centralDirectorySize = offset - centralDirectoryOffset;
  const end = Buffer.alloc(22);
  end.writeUInt32LE(0x06054b50, 0);
  end.writeUInt16LE(0, 4);
  end.writeUInt16LE(0, 6);
  end.writeUInt16LE(fileRecords.length, 8);
  end.writeUInt16LE(fileRecords.length, 10);
  end.writeUInt32LE(centralDirectorySize, 12);
  end.writeUInt32LE(centralDirectoryOffset, 16);
  end.writeUInt16LE(0, 20);
  chunks.push(end);
  return Buffer.concat(chunks);
}

function createTar(fileEntries) {
  const chunks = [];
  for (const entry of fileEntries) {
    const data = Buffer.from(entry.data);
    const name = normalizeArchivePath(entry.relativePath);
    const header = createTarHeader(name, data.length, entry.mode ?? modeForArchivePath(name));
    chunks.push(header, data, Buffer.alloc(paddingForTar(data.length)));
  }
  chunks.push(Buffer.alloc(1024));
  return Buffer.concat(chunks);
}

function createTarHeader(name, size, mode = 0o644) {
  const tarPath = splitTarPath(name);
  const header = Buffer.alloc(512, 0);
  Buffer.from(tarPath.name, 'utf8').copy(header, 0);
  Buffer.from(tarPath.prefix, 'utf8').copy(header, 345);
  writeTarOctal(header, 100, 8, mode);
  writeTarOctal(header, 108, 8, 0);
  writeTarOctal(header, 116, 8, 0);
  writeTarOctal(header, 124, 12, size);
  writeTarOctal(header, 136, 12, 0);
  header.fill(0x20, 148, 156);
  header[156] = 0x30;
  Buffer.from('ustar\0', 'ascii').copy(header, 257);
  Buffer.from('00', 'ascii').copy(header, 263);
  const checksum = header.reduce((sum, byte) => sum + byte, 0);
  writeTarChecksum(header, checksum);
  return header;
}

function splitTarPath(name) {
  const normalized = normalizeArchivePath(name);
  if (Buffer.byteLength(normalized, 'utf8') <= 100) {
    return {
      name: normalized,
      prefix: '',
    };
  }
  const segments = normalized.split('/');
  for (let index = segments.length - 1; index > 0; index -= 1) {
    const prefix = segments.slice(0, index).join('/');
    const basename = segments.slice(index).join('/');
    if (Buffer.byteLength(prefix, 'utf8') <= 155 && Buffer.byteLength(basename, 'utf8') <= 100) {
      return {
        name: basename,
        prefix,
      };
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

function writeDosDateTime(buffer, offset, date) {
  const dosTime = (date.getUTCHours() << 11) | (date.getUTCMinutes() << 5) | Math.floor(date.getUTCSeconds() / 2);
  const dosDate = ((date.getUTCFullYear() - 1980) << 9) | ((date.getUTCMonth() + 1) << 5) | date.getUTCDate();
  buffer.writeUInt16LE(dosTime, offset);
  buffer.writeUInt16LE(dosDate, offset + 2);
}

function crc32(buffer) {
  let crc = 0xffffffff;
  for (const byte of buffer) {
    crc = CRC32_TABLE[(crc ^ byte) & 0xff] ^ (crc >>> 8);
  }
  return (crc ^ 0xffffffff) >>> 0;
}

function modeForArchivePath(archivePath) {
  const normalized = normalizeArchivePath(archivePath);
  if (normalized.startsWith('bin/') && !normalized.endsWith('.cmd') && !normalized.endsWith('.ps1')) {
    return 0o755;
  }
  if (normalized.endsWith('.sh')) {
    return 0o755;
  }
  return 0o644;
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

function sha256(data) {
  return createHash('sha256').update(data).digest('hex');
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

async function main(argv = process.argv.slice(2)) {
  const settings = parseInstallPackageBuildArgs(argv);
  if (settings.help) {
    printHelp();
    return 0;
  }
  if (settings.all) {
    return await runAllInstallPackageBuilds(settings);
  }

  const buildPlan = createSdkworkImInstallPackageBuildPlan({
    packageId: settings.packageId,
    outputDir: settings.outputDir,
    requireStagedFiles: !settings.dryRun,
    stagingRoot: settings.stagingRoot,
    version: settings.version,
  });
  const issues = validateSdkworkImInstallPackageBuildPlan(buildPlan);
  if (settings.json && (settings.check || settings.dryRun)) {
    console.log(JSON.stringify({
      ok: issues.length === 0,
      issues,
      plan: buildPlan,
    }, null, 2));
  } else if (!settings.json) {
    for (const line of renderInstallPackageBuildPlan(buildPlan)) {
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

  const result = await buildSdkworkImInstallPackageArchive(buildPlan);
  if (settings.json) {
    console.log(JSON.stringify({
      ok: true,
      archive: result.archive,
      manifestPath: result.manifestPath,
      aggregateManifestPath: result.aggregateManifestPath,
      sha256SumsPath: result.sha256SumsPath,
    }, null, 2));
  } else {
    console.log(`[sdkwork-im-install-package] written: ${result.archivePath}`);
    console.log(`[sdkwork-im-install-package] sha256: ${result.archive.sha256}`);
  }
  return 0;
}

async function runAllInstallPackageBuilds(settings) {
  const packageIds = createSdkworkImInstallPackagePlan({ version: settings.version }).packages.map((item) => item.id);
  const plans = packageIds.map((packageId) => createSdkworkImInstallPackageBuildPlan({
    packageId,
    outputDir: settings.outputDir,
    requireStagedFiles: !settings.dryRun,
    root: repoRoot,
    stagingRoot: settings.stagingRoot ? path.join(settings.stagingRoot, packageId) : null,
    version: settings.version,
  }));
  const issues = plans.flatMap((plan) =>
    validateSdkworkImInstallPackageBuildPlan(plan).map((issue) => `${plan.package.id}: ${issue}`)
  );
  if (settings.json && (settings.check || settings.dryRun)) {
    console.log(JSON.stringify({
      ok: issues.length === 0,
      issues,
      plans,
    }, null, 2));
  } else if (!settings.json) {
    for (const plan of plans) {
      for (const line of renderInstallPackageBuildPlan(plan)) {
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
    results.push(await buildSdkworkImInstallPackageArchive(plan));
  }
  if (settings.json) {
    console.log(JSON.stringify({
      ok: true,
      archives: results.map((result) => result.archive),
      aggregateManifestPath: results.at(-1)?.aggregateManifestPath ?? null,
      sha256SumsPath: results.at(-1)?.sha256SumsPath ?? null,
    }, null, 2));
  }
  return 0;
}

function renderInstallPackageBuildPlan(buildPlan) {
  return [
    `[sdkwork-im-install-package] package: ${buildPlan.package.id}`,
    `[sdkwork-im-install-package] archive: ${buildPlan.archivePath}`,
    `[sdkwork-im-install-package] manifest: ${buildPlan.manifestPath}`,
    `[sdkwork-im-install-package] entries: ${buildPlan.entries.length}`,
    ...buildPlan.entries.map((entry) => `[sdkwork-im-install-package]   ${entry.archivePath}${entry.planned ? ' (planned)' : ''}`),
  ];
}

function printIssues(issues) {
  if (issues.length === 0) {
    return;
  }
  console.error('[sdkwork-im-install-package] validation issues:');
  for (const issue of issues) {
    console.error(`[sdkwork-im-install-package]   ${issue}`);
  }
}

if (process.argv[1] && import.meta.url.endsWith(process.argv[1].replaceAll('\\', '/'))) {
  main().then((code) => {
    process.exitCode = code;
  }).catch((error) => {
    console.error(`[sdkwork-im-install-package] ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  });
}

export {
  AGGREGATE_MANIFEST_FILE,
  BUILD_SCHEMA_VERSION,
  PACKAGE_MANIFEST_SCHEMA_VERSION,
  SHA256SUMS_FILE,
  buildSdkworkImInstallPackageArchive,
  createAggregateManifest,
  createSdkworkImInstallPackageBuildPlan,
  createInstallArchiveBytes,
  createPackageBuildManifest,
  createTar,
  createZip,
  currentHostServerPackageId,
  main,
  modeForArchivePath,
  normalizeArchivePath,
  parseInstallPackageBuildArgs,
  renderInstallPackageBuildPlan,
  sha256,
  validateSdkworkImInstallPackageBuildPlan,
};
