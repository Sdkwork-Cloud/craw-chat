#!/usr/bin/env node

import { createHash } from 'node:crypto';
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { DEFAULT_RELEASE_VERSION } from './sdkwork-im-release-version.mjs';

const __filename = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(__filename), '..', '..');

const DESKTOP_BUNDLE_SCHEMA_VERSION = '2026-06-04.sdkwork-im.desktop-bundles.v1';
const ALLOWED_DESKTOP_BUNDLE_EXTENSIONS = Object.freeze([
  '.AppImage',
  '.app.tar.gz',
  '.deb',
  '.dmg',
  '.exe',
  '.msi',
  '.pkg',
  '.rpm',
  '.zip',
]);
const PLATFORM_DESKTOP_BUNDLE_EXTENSIONS = Object.freeze({
  linux: ['.AppImage', '.deb', '.rpm'],
  macos: ['.app.tar.gz', '.dmg', '.pkg'],
  windows: ['.exe', '.msi', '.zip'],
});
const ARCH_ALIASES = Object.freeze({
  x64: ['x64', 'x86_64', 'amd64'],
  arm64: ['arm64', 'aarch64'],
});

function printHelp() {
  console.log(`Usage: node scripts/release/collect-sdkwork-im-desktop-bundles.mjs [options]

Collect Tauri desktop installer outputs for a Sdkwork IM release package.

Options:
  --bundle-root <dir>  Explicit Tauri bundle root.
  --platform <value>   Target platform metadata.
  --arch <value>       Target architecture metadata.
  --version <value>    Release version (default ${DEFAULT_RELEASE_VERSION}).
  --check              Require at least one installer artifact.
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

function parseDesktopBundleCollectorArgs(argv = process.argv.slice(2)) {
  const settings = {
    arch: null,
    bundleRoot: null,
    check: false,
    help: false,
    json: false,
    platform: null,
    version: DEFAULT_RELEASE_VERSION,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === '--') {
      continue;
    }
    switch (arg) {
      case '--bundle-root':
        settings.bundleRoot = requireValue(argv, index, arg);
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
      case '--version':
        settings.version = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--check':
        settings.check = true;
        break;
      case '--json':
        settings.json = true;
        break;
      case '--help':
      case '-h':
        settings.help = true;
        break;
      default:
        throw new Error(`Unsupported desktop bundle collector option: ${arg}`);
    }
  }

  return settings;
}

function defaultDesktopBundleRoots({
  env = process.env,
  root = repoRoot,
} = {}) {
  const roots = [];
  const cargoTargetDir = String(env.CARGO_TARGET_DIR ?? '').trim();
  if (cargoTargetDir) {
    roots.push(path.resolve(root, cargoTargetDir, 'release', 'bundle'));
  }
  roots.push(path.join(
    root,
    'apps',
    'sdkwork-im-pc',
    'packages',
    'sdkwork-im-pc-desktop',
    'src-tauri',
    'target',
    'release',
    'bundle',
  ));
  return roots;
}

function resolveDesktopBundleRoot({
  bundleRoot = null,
  env = process.env,
  root = repoRoot,
} = {}) {
  if (bundleRoot) {
    return path.resolve(root, bundleRoot);
  }
  return defaultDesktopBundleRoots({ env, root }).find((candidate) => existsSync(candidate)) ?? defaultDesktopBundleRoots({ env, root })[0];
}

function collectSdkworkImDesktopBundles({
  arch = null,
  bundleRoot = null,
  env = process.env,
  platform = null,
  root = repoRoot,
  version = DEFAULT_RELEASE_VERSION,
} = {}) {
  const resolvedBundleRoot = resolveDesktopBundleRoot({ bundleRoot, env, root });
  const files = existsSync(resolvedBundleRoot)
    ? collectBundleFiles(resolvedBundleRoot, resolvedBundleRoot, { arch, platform })
    : [];

  return {
    schemaVersion: DESKTOP_BUNDLE_SCHEMA_VERSION,
    product: 'chat',
    version,
    platform,
    architecture: arch,
    bundleRoot: resolvedBundleRoot,
    files,
  };
}

function collectBundleFiles(currentDir, rootDir, { arch = null, platform = null } = {}) {
  const entries = [];
  for (const entry of readdirSync(currentDir, { withFileTypes: true }).sort((left, right) => left.name.localeCompare(right.name))) {
    const absolutePath = path.join(currentDir, entry.name);
    const relativePath = normalizeArchivePath(path.relative(rootDir, absolutePath));
    if (shouldExcludeBundlePath(relativePath)) {
      continue;
    }
    if (entry.isDirectory()) {
      entries.push(...collectBundleFiles(absolutePath, rootDir, { arch, platform }));
      continue;
    }
    if (!entry.isFile() || !isAllowedDesktopBundleFile(relativePath, { arch, platform })) {
      continue;
    }
    const stat = statSync(absolutePath);
    entries.push({
      path: relativePath,
      sourcePath: absolutePath,
      size: stat.size,
      sha256: sha256File(absolutePath),
    });
  }
  return entries;
}

function isAllowedDesktopBundleFile(relativePath, { arch = null, platform = null } = {}) {
  const extensions = PLATFORM_DESKTOP_BUNDLE_EXTENSIONS[platform] ?? ALLOWED_DESKTOP_BUNDLE_EXTENSIONS;
  return extensions.some((extension) => relativePath.endsWith(extension))
    && isAllowedDesktopBundleArchitecture(relativePath, arch);
}

function isAllowedDesktopBundleArchitecture(relativePath, arch = null) {
  if (!arch || !ARCH_ALIASES[arch]) {
    return true;
  }
  const normalizedPath = String(relativePath).replaceAll('\\', '/').toLowerCase();
  const requestedAliases = ARCH_ALIASES[arch];
  if (requestedAliases.some((alias) => pathHasArchToken(normalizedPath, alias))) {
    return true;
  }
  const otherAliases = Object.entries(ARCH_ALIASES)
    .filter(([candidateArch]) => candidateArch !== arch)
    .flatMap(([, aliases]) => aliases);
  return !otherAliases.some((alias) => pathHasArchToken(normalizedPath, alias));
}

function pathHasArchToken(normalizedPath, alias) {
  const escapedAlias = escapeRegExp(alias.toLowerCase());
  return new RegExp(`(^|[^a-z0-9])${escapedAlias}([^a-z0-9]|$)`, 'u').test(normalizedPath);
}

function escapeRegExp(value) {
  return String(value).replace(/[.*+?^${}()|[\]\\]/gu, '\\$&');
}

function shouldExcludeBundlePath(relativePath) {
  return /(^|\/)(\.env|node_modules|\.git|\.runtime|target\/debug|secrets?)(\/|$)/u.test(relativePath)
    || /secret/u.test(relativePath);
}

function validateSdkworkImDesktopBundleManifest(manifest, { requireFiles = true } = {}) {
  const issues = [];
  if (manifest.schemaVersion !== DESKTOP_BUNDLE_SCHEMA_VERSION) {
    issues.push(`schemaVersion must be ${DESKTOP_BUNDLE_SCHEMA_VERSION}`);
  }
  if (manifest.product !== 'chat') {
    issues.push('product must be chat');
  }
  if (!manifest.version) {
    issues.push('version is required');
  }
  if (!manifest.bundleRoot) {
    issues.push('bundleRoot is required');
  }
  if (!Array.isArray(manifest.files)) {
    issues.push('files must be an array');
    return issues;
  }
  if (requireFiles && manifest.files.length === 0) {
    issues.push(`no desktop installer files found under ${manifest.bundleRoot}`);
  }
  for (const file of manifest.files) {
    try {
      normalizeArchivePath(file.path);
    } catch (error) {
      issues.push(error instanceof Error ? error.message : String(error));
    }
    if (shouldExcludeBundlePath(file.path)) {
      issues.push(`desktop bundle file must not include sensitive path: ${file.path}`);
    }
    if (!isAllowedDesktopBundleFile(file.path, { arch: manifest.architecture, platform: manifest.platform })) {
      issues.push(`unsupported desktop bundle file extension: ${file.path}`);
    }
  }
  return issues;
}

function normalizeArchivePath(value) {
  const normalized = String(value ?? '').replaceAll('\\', '/').replace(/^\/+/u, '');
  if (!normalized || normalized === '.' || normalized.includes('..') || path.isAbsolute(normalized)) {
    throw new Error(`Unsafe archive path: ${value}`);
  }
  return normalized;
}

function sha256File(filePath) {
  return createHash('sha256').update(readFileSync(filePath)).digest('hex');
}

async function main(argv = process.argv.slice(2)) {
  const settings = parseDesktopBundleCollectorArgs(argv);
  if (settings.help) {
    printHelp();
    return 0;
  }

  const manifest = collectSdkworkImDesktopBundles(settings);
  const issues = validateSdkworkImDesktopBundleManifest(manifest, { requireFiles: settings.check });
  if (settings.json) {
    console.log(JSON.stringify({
      ok: issues.length === 0,
      issues,
      manifest,
    }, null, 2));
  } else {
    console.log(`[sdkwork-im-desktop-bundles] root: ${manifest.bundleRoot}`);
    console.log(`[sdkwork-im-desktop-bundles] files: ${manifest.files.length}`);
    for (const file of manifest.files) {
      console.log(`[sdkwork-im-desktop-bundles]   ${file.path}`);
    }
    if (issues.length > 0) {
      console.error('[sdkwork-im-desktop-bundles] validation issues:');
      for (const issue of issues) {
        console.error(`[sdkwork-im-desktop-bundles]   ${issue}`);
      }
    }
  }

  if (settings.check && issues.length > 0) {
    return 1;
  }
  return 0;
}

if (process.argv[1] && import.meta.url.endsWith(process.argv[1].replaceAll('\\', '/'))) {
  main().then((code) => {
    process.exitCode = code;
  }).catch((error) => {
    console.error(`[sdkwork-im-desktop-bundles] ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  });
}

export {
  ALLOWED_DESKTOP_BUNDLE_EXTENSIONS,
  DESKTOP_BUNDLE_SCHEMA_VERSION,
  PLATFORM_DESKTOP_BUNDLE_EXTENSIONS,
  collectSdkworkImDesktopBundles,
  defaultDesktopBundleRoots,
  isAllowedDesktopBundleFile,
  main,
  parseDesktopBundleCollectorArgs,
  resolveDesktopBundleRoot,
  validateSdkworkImDesktopBundleManifest,
};
