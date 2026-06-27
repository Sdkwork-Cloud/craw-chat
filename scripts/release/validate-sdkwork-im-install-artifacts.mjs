#!/usr/bin/env node

import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { gunzipSync } from 'node:zlib';

import { DEFAULT_RELEASE_VERSION } from './sdkwork-im-release-version.mjs';
import { createSdkworkImInstallPackagePlan } from './plan-sdkwork-im-install-packages.mjs';

function printHelp() {
  console.log(`Usage: node scripts/release/validate-sdkwork-im-install-artifacts.mjs --package-id <id> --artifact-path <path> [options]

Validate Sdkwork IM release package payload layout before GitHub Release upload.

Options:
  --package-id <id>       Package id from release package plan.
  --artifact-path <path>  Built package archive path.
  --version <value>       Package version used by the release build (default ${DEFAULT_RELEASE_VERSION}).
  --json                  Print machine-readable JSON.
  -h, --help              Show this help.
`);
}

function parseValidateArgs(argv = process.argv.slice(2)) {
  const settings = {
    artifactPath: null,
    help: false,
    json: false,
    packageId: null,
    version: DEFAULT_RELEASE_VERSION,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    switch (arg) {
      case '--artifact-path':
        settings.artifactPath = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--package-id':
        settings.packageId = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--version':
        settings.version = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--json':
        settings.json = true;
        break;
      case '--help':
      case '-h':
        settings.help = true;
        break;
      default:
        throw new Error(`Unsupported install artifact validation option: ${arg}`);
    }
  }

  return settings;
}

function requireValue(argv, index, flag) {
  const value = argv[index + 1];
  if (!value || value.startsWith('--')) {
    throw new Error(`${flag} requires a value`);
  }
  return value;
}

function validateSdkworkImInstallArtifact({
  artifactPath,
  packageId,
  root = process.cwd(),
  version = DEFAULT_RELEASE_VERSION,
} = {}) {
  const issues = [];
  const resolvedArtifactPath = artifactPath ? path.resolve(root, artifactPath) : null;
  if (!packageId) {
    issues.push('--package-id is required');
  }
  if (!resolvedArtifactPath) {
    issues.push('--artifact-path is required');
  } else if (!existsSync(resolvedArtifactPath)) {
    issues.push(`artifact does not exist: ${resolvedArtifactPath}`);
  }
  if (issues.length > 0) {
    return {
      ok: false,
      issues,
      packageId,
      artifactPath: resolvedArtifactPath,
    };
  }

  const packageItem = createSdkworkImInstallPackagePlan({ version }).packages.find((item) => item.id === packageId);
  if (!packageItem) {
    issues.push(`unknown package id: ${packageId}`);
    return {
      ok: false,
      issues,
      packageId,
      artifactPath: resolvedArtifactPath,
    };
  }

  const artifactBytes = readFileSync(resolvedArtifactPath);
  const artifactName = path.basename(resolvedArtifactPath);
  if (artifactBytes.length === 0) {
    issues.push(`artifact is empty: ${resolvedArtifactPath}`);
  }
  if (artifactName !== packageItem.archiveName) {
    issues.push(`artifact name ${artifactName} must be ${packageItem.archiveName}`);
  }

  const extension = artifactExtension(artifactName);
  const adjacentManifest = readAdjacentManifest(resolvedArtifactPath, extension, issues);
  if (adjacentManifest) {
    issues.push(...validateAdjacentManifest(packageItem, adjacentManifest));
  }

  if (extension === 'tar.gz') {
    issues.push(...validateTarGzArtifact(packageItem, artifactBytes));
  } else if (extension === 'zip') {
    issues.push(...validateZipArtifact(packageItem, artifactBytes));
  } else {
    issues.push(`unsupported artifact extension for ${packageItem.id}: ${extension}`);
  }

  return {
    ok: issues.length === 0,
    issues,
    packageId: packageItem.id,
    artifactPath: resolvedArtifactPath,
    artifactName,
    extension,
  };
}

function artifactExtension(fileName) {
  if (fileName.endsWith('.tar.gz')) {
    return 'tar.gz';
  }
  return path.extname(fileName).replace(/^\./u, '');
}

function readAdjacentManifest(artifactPath, extension, issues) {
  const manifestPath = extension === 'tar.gz'
    ? artifactPath.replace(/\.tar\.gz$/u, '.manifest.json')
    : artifactPath.replace(new RegExp(`\\.${escapeRegExp(extension)}$`, 'u'), '.manifest.json');
  if (!existsSync(manifestPath)) {
    issues.push(`missing adjacent manifest: ${manifestPath}`);
    return null;
  }
  try {
    return JSON.parse(readFileSync(manifestPath, 'utf8'));
  } catch (error) {
    issues.push(`adjacent manifest is invalid JSON: ${error instanceof Error ? error.message : String(error)}`);
    return null;
  }
}

function escapeRegExp(value) {
  return String(value).replace(/[.*+?^${}()|[\]\\]/gu, '\\$&');
}

function validateAdjacentManifest(packageItem, manifest) {
  const issues = [];
  if (manifest.product !== 'chat') {
    issues.push('adjacent manifest product must be chat');
  }
  if (manifest.package?.id !== packageItem.id) {
    issues.push(`adjacent manifest package id must be ${packageItem.id}`);
  }
  if (manifest.package?.version !== packageItem.version) {
    issues.push(`adjacent manifest version must be ${packageItem.version}`);
  }
  if (manifest.archive?.file !== packageItem.archiveName) {
    issues.push(`adjacent manifest archive.file must be ${packageItem.archiveName}`);
  }
  if (!Array.isArray(manifest.files) || manifest.files.length === 0) {
    issues.push('adjacent manifest files must be a non-empty array');
  }
  for (const file of manifest.files ?? []) {
    if (!file.path) {
      issues.push('adjacent manifest file path is required');
      continue;
    }
    if (isSensitiveArchivePath(file.path)) {
      issues.push(`adjacent manifest must not include sensitive path ${file.path}`);
    }
  }
  return issues;
}

function validateTarGzArtifact(packageItem, artifactBytes) {
  const entries = readTarEntries(gunzipSync(artifactBytes));
  return validateArchiveEntries(packageItem, new Set(entries.keys()));
}

function validateZipArtifact(packageItem, artifactBytes) {
  return validateArchiveEntries(packageItem, readZipEntries(artifactBytes));
}

function validateArchiveEntries(packageItem, entries) {
  const issues = [];
  if (entries.size === 0) {
    issues.push(`${packageItem.id} archive must include files`);
    return issues;
  }
  for (const entry of entries) {
    if (isSensitiveArchivePath(entry)) {
      issues.push(`${packageItem.id} archive must not include sensitive path ${entry}`);
    }
  }

  if (packageItem.deploymentMode === 'server-archive') {
    for (const requiredEntry of [
      `bin/${packageItem.binaryName}`,
      'config/chat.toml.example',
      'config/server.env.example',
      'config/postgresql.yaml.example',
      'INSTALL.md',
      'install-manifest.json',
    ]) {
      requireArchiveEntry(entries, requiredEntry, packageItem.id, issues);
    }
    requireEntryPrefix(entries, 'web/sdkwork-im-pc/dist/', packageItem.id, issues);
    requireEntryPrefix(entries, `service/${packageItem.platform}/`, packageItem.id, issues);
  } else if (packageItem.deploymentMode === 'desktop') {
    requireArchiveEntry(entries, 'desktop-manifest.json', packageItem.id, issues);
    const installerEntries = [...entries].filter((entry) =>
      entry.startsWith('desktop/') && isPlatformDesktopInstaller(entry, packageItem.platform)
    );
    if (installerEntries.length === 0) {
      issues.push(`${packageItem.id} desktop bundle must include at least one ${packageItem.platform} installer under desktop/`);
    }
  }

  return issues;
}

function requireArchiveEntry(entries, requiredEntry, packageId, issues) {
  if (!entries.has(requiredEntry)) {
    issues.push(`${packageId} archive missing ${requiredEntry}`);
  }
}

function requireEntryPrefix(entries, prefix, packageId, issues) {
  if (![...entries].some((entry) => entry.startsWith(prefix))) {
    issues.push(`${packageId} archive missing entries under ${prefix}`);
  }
}

function isPlatformDesktopInstaller(entry, platform) {
  const extensionsByPlatform = {
    linux: ['.AppImage', '.deb', '.rpm'],
    macos: ['.app.tar.gz', '.dmg', '.pkg'],
    windows: ['.exe', '.msi', '.zip'],
  };
  return (extensionsByPlatform[platform] ?? []).some((extension) => entry.endsWith(extension));
}

function isSensitiveArchivePath(value) {
  const normalized = String(value ?? '').replaceAll('\\', '/');
  return /(^|\/)\.env($|\.|\/)|(^|\/)node_modules(\/|$)|(^|\/)\.runtime(\/|$)|(^|\/)secrets?(\/|$)|secret/u.test(normalized);
}

function readTarEntries(buffer) {
  const entries = new Map();
  for (let offset = 0; offset + 512 <= buffer.length;) {
    const header = buffer.subarray(offset, offset + 512);
    if (header.every((byte) => byte === 0)) {
      break;
    }
    const namePart = readTarString(header, 0, 100);
    const prefixPart = readTarString(header, 345, 155);
    const name = prefixPart ? `${prefixPart}/${namePart}` : namePart;
    const mode = Number.parseInt(readTarString(header, 100, 8) || '0', 8);
    const size = Number.parseInt(readTarString(header, 124, 12) || '0', 8);
    const typeflag = header.subarray(156, 157).toString('ascii');
    const dataOffset = offset + 512;
    entries.set(name, {
      data: buffer.subarray(dataOffset, dataOffset + size),
      mode,
      size,
      type: typeflag === '5' ? 'directory' : 'file',
      typeflag,
    });
    offset += 512 + Math.ceil(size / 512) * 512;
  }
  return entries;
}

function readTarString(buffer, offset, length) {
  return buffer
    .subarray(offset, offset + length)
    .toString('utf8')
    .replace(/\0.*$/u, '')
    .trim();
}

function readZipEntries(buffer) {
  const entries = new Set();
  for (let offset = 0; offset + 30 <= buffer.length;) {
    const signature = buffer.readUInt32LE(offset);
    if (signature !== 0x04034b50) {
      break;
    }
    const compressedSize = buffer.readUInt32LE(offset + 18);
    const nameLength = buffer.readUInt16LE(offset + 26);
    const extraLength = buffer.readUInt16LE(offset + 28);
    const name = buffer.subarray(offset + 30, offset + 30 + nameLength).toString('utf8');
    entries.add(name);
    offset += 30 + nameLength + extraLength + compressedSize;
  }
  return entries;
}

async function main(argv = process.argv.slice(2)) {
  const settings = parseValidateArgs(argv);
  if (settings.help) {
    printHelp();
    return 0;
  }

  const result = validateSdkworkImInstallArtifact(settings);
  if (settings.json) {
    console.log(JSON.stringify(result, null, 2));
  } else if (result.ok) {
    console.log(`[sdkwork-im-install-artifact-validate] ok: ${result.packageId} ${result.artifactName}`);
  } else {
    console.error(`[sdkwork-im-install-artifact-validate] failed: ${result.packageId ?? '(missing package id)'}`);
    for (const issue of result.issues) {
      console.error(`[sdkwork-im-install-artifact-validate]   ${issue}`);
    }
  }
  return result.ok ? 0 : 1;
}

if (process.argv[1] && import.meta.url.endsWith(process.argv[1].replaceAll('\\', '/'))) {
  main().then((code) => {
    process.exitCode = code;
  }).catch((error) => {
    console.error(`[sdkwork-im-install-artifact-validate] ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  });
}

export {
  main,
  parseValidateArgs,
  readTarEntries,
  readZipEntries,
  validateSdkworkImInstallArtifact,
  validateTarGzArtifact,
  validateZipArtifact,
};
