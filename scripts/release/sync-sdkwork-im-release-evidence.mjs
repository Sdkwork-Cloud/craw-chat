#!/usr/bin/env node

import { createHash } from 'node:crypto';
import { existsSync, readFileSync, statSync } from 'node:fs';
import { readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { AGGREGATE_MANIFEST_FILE } from './build-sdkwork-im-install-package.mjs';

const __filename = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(__filename), '..', '..');
const DEFAULT_PACKAGES_DIR = 'dist/release-packages';
const DEFAULT_EVIDENCE_ROOT = 'dist/release-evidence';
const DEFAULT_MANIFEST_PATH = 'sdkwork.app.config.json';

const STORE_CONTROLLED_SOURCE_TYPES = new Set(['APP_STORE', 'MARKETPLACE', 'STORE']);
const SIGNATURE_CANDIDATES = Object.freeze([
  { suffix: '.sig', kind: 'detached-signature' },
  { suffix: '.asc', kind: 'gpg-detached-signature' },
  { suffix: '.sigstore.json', kind: 'sigstore-bundle' },
  { suffix: '.minisig', kind: 'minisign-signature' },
  { suffix: '.p7s', kind: 'pkcs7-signature' },
]);
const SBOM_CANDIDATES = Object.freeze([
  { suffix: '.cdx.json', format: 'CycloneDX' },
  { suffix: '.spdx.json', format: 'SPDX' },
]);
const PROVENANCE_CANDIDATES = Object.freeze([
  { suffix: '.intoto.jsonl', format: 'in-toto' },
  { suffix: '.attestation.jsonl', format: 'in-toto' },
  { suffix: '.provenance.json', format: 'SLSA' },
]);

function printHelp() {
  console.log(`Usage: node scripts/release/sync-sdkwork-im-release-evidence.mjs [options]

Synchronize sdkwork.app.config.json package evidence from real release artifacts.

Options:
  --package-id <id>      Restrict sync to one package id. May be repeated.
  --packages-dir <dir>   Release package directory (default ${DEFAULT_PACKAGES_DIR}).
  --evidence-root <dir>  Release evidence root (default ${DEFAULT_EVIDENCE_ROOT}).
  --manifest-path <file> App manifest path (default ${DEFAULT_MANIFEST_PATH}).
  --check                Validate evidence without writing the manifest.
  --write                Write sdkwork.app.config.json when all evidence is present.
  --json                 Print machine-readable JSON.
  -h, --help             Show this help.
`);
}

function parseReleaseEvidenceSyncArgs(argv = process.argv.slice(2)) {
  const settings = {
    check: false,
    evidenceRoot: null,
    help: false,
    json: false,
    manifestPath: null,
    packageIds: [],
    packagesDir: null,
    write: false,
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
      case '--write':
        settings.write = true;
        break;
      case '--json':
        settings.json = true;
        break;
      case '--package-id':
        settings.packageIds.push(requireValue(argv, index, arg));
        index += 1;
        break;
      case '--packages-dir':
        settings.packagesDir = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--evidence-root':
        settings.evidenceRoot = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--manifest-path':
        settings.manifestPath = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--help':
      case '-h':
        settings.help = true;
        break;
      default:
        throw new Error(`Unsupported release evidence sync option: ${arg}`);
    }
  }

  settings.packageIds = uniqueStrings(settings.packageIds);
  if (settings.check && settings.write) {
    throw new Error('--check and --write cannot be used together');
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

async function createSdkworkImReleaseEvidenceSyncPlan({
  evidenceRoot = DEFAULT_EVIDENCE_ROOT,
  manifestPath = DEFAULT_MANIFEST_PATH,
  packageIds = [],
  packagesDir = DEFAULT_PACKAGES_DIR,
  root = repoRoot,
} = {}) {
  const resolvedRoot = path.resolve(root);
  const resolvedManifestPath = resolveInsideRoot(resolvedRoot, manifestPath);
  const resolvedPackagesDir = resolveInsideRoot(resolvedRoot, packagesDir);
  const resolvedEvidenceRoot = resolveInsideRoot(resolvedRoot, evidenceRoot);
  const issues = [];

  const manifest = JSON.parse(await readFile(resolvedManifestPath, 'utf8'));
  const manifestPackages = Array.isArray(manifest?.artifacts?.installConfig?.packages)
    ? manifest.artifacts.installConfig.packages
    : [];
  const selectedPackageIds = packageIds.length > 0
    ? uniqueStrings(packageIds)
    : manifestPackages
      .filter((releasePackage) => releasePackage?.enabled !== false && !isStoreControlledPackage(releasePackage))
      .map((releasePackage) => releasePackage.id)
      .filter((value) => typeof value === 'string' && value.length > 0);
  const manifestPackagesById = new Map(manifestPackages.map((releasePackage) => [releasePackage.id, releasePackage]));
  for (const packageId of selectedPackageIds) {
    if (!manifestPackagesById.has(packageId)) {
      issues.push(`${packageId} is not present in ${toRepoRelative(resolvedRoot, resolvedManifestPath)}.`);
    }
  }

  const aggregateManifestPath = path.join(resolvedPackagesDir, AGGREGATE_MANIFEST_FILE);
  const aggregateManifest = existsSync(aggregateManifestPath)
    ? JSON.parse(await readFile(aggregateManifestPath, 'utf8'))
    : null;
  if (!aggregateManifest) {
    issues.push(`${toRepoRelative(resolvedRoot, aggregateManifestPath)} is required before release evidence can be synchronized.`);
  }

  const aggregateArchives = Array.isArray(aggregateManifest?.archives) ? aggregateManifest.archives : [];
  const aggregateArchivesByPackageId = new Map(
    aggregateArchives
      .filter((archive) => typeof archive?.packageId === 'string')
      .map((archive) => [archive.packageId, archive]),
  );
  const packages = [];

  for (const packageId of selectedPackageIds) {
    const releasePackage = manifestPackagesById.get(packageId);
    if (!releasePackage || isStoreControlledPackage(releasePackage)) {
      continue;
    }

    const packageEvidence = collectPackageEvidence({
      aggregateArchive: aggregateArchivesByPackageId.get(packageId),
      evidenceRoot: resolvedEvidenceRoot,
      packageId,
      packagesDir: resolvedPackagesDir,
      root: resolvedRoot,
    });
    issues.push(...packageEvidence.issues);
    if (packageEvidence.ok) {
      packages.push(packageEvidence.package);
    }
  }

  if (issues.length > 0) {
    return {
      ok: false,
      issues,
      root: resolvedRoot,
      manifestPath: resolvedManifestPath,
      packagesDir: resolvedPackagesDir,
      evidenceRoot: resolvedEvidenceRoot,
      packages,
      updatedManifest: null,
    };
  }

  const updatedManifest = cloneJson(manifest);
  const updatedPackagesById = new Map(packages.map((releasePackage) => [releasePackage.packageId, releasePackage]));
  updatedManifest.artifacts.installConfig.packages = updatedManifest.artifacts.installConfig.packages.map((releasePackage) => {
    const packageEvidence = updatedPackagesById.get(releasePackage.id);
    if (!packageEvidence) {
      return releasePackage;
    }
    return applyPackageEvidence(releasePackage, packageEvidence);
  });

  return {
    ok: true,
    issues: [],
    root: resolvedRoot,
    manifestPath: resolvedManifestPath,
    packagesDir: resolvedPackagesDir,
    evidenceRoot: resolvedEvidenceRoot,
    packages,
    updatedManifest,
  };
}

function collectPackageEvidence({
  aggregateArchive,
  evidenceRoot,
  packageId,
  packagesDir,
  root,
}) {
  const issues = [];
  if (!aggregateArchive) {
    return {
      ok: false,
      issues: [`${packageId} release archive is missing from ${toRepoRelative(root, path.join(packagesDir, AGGREGATE_MANIFEST_FILE))}.`],
      package: null,
    };
  }

  const archiveFile = String(aggregateArchive.file ?? '').trim();
  if (!archiveFile) {
    return {
      ok: false,
      issues: [`${packageId} release archive entry must declare archive.file.`],
      package: null,
    };
  }

  let archivePath;
  try {
    archivePath = resolveInsideDirectory(packagesDir, archiveFile);
  } catch (error) {
    return {
      ok: false,
      issues: [`${packageId} archive file is unsafe: ${error instanceof Error ? error.message : String(error)}`],
      package: null,
    };
  }

  if (!existsSync(archivePath)) {
    issues.push(`${packageId} release archive does not exist: ${toRepoRelative(root, archivePath)}.`);
  } else if (!statSync(archivePath).isFile()) {
    issues.push(`${packageId} release archive is not a file: ${toRepoRelative(root, archivePath)}.`);
  }

  const manifestPath = manifestPathForArchive(archivePath);
  if (!existsSync(manifestPath)) {
    issues.push(`${packageId} adjacent package manifest is missing: ${toRepoRelative(root, manifestPath)}.`);
  }

  const packageEvidenceRoot = path.join(evidenceRoot, packageId);
  const signature = findEvidenceFile({
    archiveFile,
    candidates: SIGNATURE_CANDIDATES,
    evidenceRoot: packageEvidenceRoot,
  });
  const sbom = findEvidenceFile({
    archiveFile,
    candidates: SBOM_CANDIDATES,
    evidenceRoot: packageEvidenceRoot,
  });
  const provenance = findEvidenceFile({
    archiveFile,
    candidates: PROVENANCE_CANDIDATES,
    evidenceRoot: packageEvidenceRoot,
  });

  if (!signature) {
    issues.push(`${packageId} signature evidence is missing under ${toRepoRelative(root, packageEvidenceRoot)}.`);
  }
  if (!sbom) {
    issues.push(`${packageId} SBOM evidence is missing under ${toRepoRelative(root, packageEvidenceRoot)}.`);
  }
  if (!provenance) {
    issues.push(`${packageId} provenance evidence is missing under ${toRepoRelative(root, packageEvidenceRoot)}.`);
  }

  if (issues.length > 0) {
    return {
      ok: false,
      issues,
      package: null,
    };
  }

  const archiveBytes = readFileSync(archivePath);
  const computedSha256 = sha256(archiveBytes);
  const declaredSha256 = String(aggregateArchive.sha256 ?? '').trim().replace(/^sha256:/iu, '');
  if (declaredSha256 && declaredSha256.toLowerCase() !== computedSha256) {
    issues.push(`${packageId} aggregate archive sha256 does not match ${toRepoRelative(root, archivePath)}.`);
  }

  if (issues.length > 0) {
    return {
      ok: false,
      issues,
      package: null,
    };
  }

  return {
    ok: true,
    issues: [],
    package: {
      packageId,
      archive: {
        path: archivePath,
        ref: toRepoRelative(root, archivePath),
        manifestRef: toRepoRelative(root, manifestPath),
        sha256: computedSha256,
        sizeBytes: archiveBytes.length,
      },
      signature: {
        kind: signature.kind,
        ref: toRepoRelative(root, signature.path),
      },
      sbom: {
        format: sbom.format,
        ref: toRepoRelative(root, sbom.path),
      },
      provenance: {
        format: provenance.format,
        ref: toRepoRelative(root, provenance.path),
      },
    },
  };
}

function applyPackageEvidence(releasePackage, packageEvidence) {
  return {
    ...releasePackage,
    checksumAlgorithm: 'SHA-256',
    checksum: packageEvidence.archive.sha256,
    sizeBytes: packageEvidence.archive.sizeBytes,
    metadata: {
      ...(isObject(releasePackage.metadata) ? releasePackage.metadata : {}),
      artifact: {
        ref: packageEvidence.archive.ref,
        manifestRef: packageEvidence.archive.manifestRef,
      },
      signing: {
        kind: packageEvidence.signature.kind,
        ref: packageEvidence.signature.ref,
      },
      sbom: {
        format: packageEvidence.sbom.format,
        ref: packageEvidence.sbom.ref,
      },
      provenance: {
        format: packageEvidence.provenance.format,
        ref: packageEvidence.provenance.ref,
      },
    },
  };
}

async function applySdkworkImReleaseEvidenceSync(plan, { write = false } = {}) {
  if (!plan?.ok || !plan.updatedManifest) {
    return {
      written: false,
      manifestPath: plan?.manifestPath ?? null,
      updatedManifest: null,
      issues: Array.isArray(plan?.issues) ? plan.issues : ['release evidence sync plan is not complete'],
    };
  }

  if (write) {
    await writeFile(plan.manifestPath, `${JSON.stringify(plan.updatedManifest, null, 2)}\n`, 'utf8');
  }

  return {
    written: write,
    manifestPath: plan.manifestPath,
    updatedManifest: plan.updatedManifest,
    issues: [],
  };
}

function findEvidenceFile({ archiveFile, candidates, evidenceRoot }) {
  for (const candidate of candidates) {
    const evidencePath = path.join(evidenceRoot, `${archiveFile}${candidate.suffix}`);
    if (!existsSync(evidencePath) || !statSync(evidencePath).isFile()) {
      continue;
    }
    return {
      ...candidate,
      path: evidencePath,
    };
  }
  return null;
}

function manifestPathForArchive(archivePath) {
  if (archivePath.endsWith('.tar.gz')) {
    return archivePath.replace(/\.tar\.gz$/u, '.manifest.json');
  }
  return archivePath.replace(/\.[^.]+$/u, '.manifest.json');
}

function resolveInsideRoot(root, relativePath) {
  return resolveInsideDirectory(root, relativePath);
}

function resolveInsideDirectory(parent, relativePath) {
  const value = String(relativePath ?? '').trim();
  if (!value) {
    throw new Error('path must not be empty');
  }
  if (value.includes('\\')) {
    throw new Error(`${value} must use forward-slash paths`);
  }
  if (path.isAbsolute(value)) {
    throw new Error(`${value} must be relative`);
  }
  const resolvedParent = path.resolve(parent);
  const resolvedPath = path.resolve(resolvedParent, value);
  if (!isPathInsideOrSame(resolvedPath, resolvedParent)) {
    throw new Error(`${value} must stay inside ${resolvedParent}`);
  }
  return resolvedPath;
}

function isPathInsideOrSame(candidatePath, parentPath) {
  const relative = path.relative(path.resolve(parentPath), path.resolve(candidatePath));
  return relative === '' || (Boolean(relative) && !relative.startsWith('..') && !path.isAbsolute(relative));
}

function toRepoRelative(root, absolutePath) {
  return path.relative(root, absolutePath).replaceAll('\\', '/');
}

function sha256(data) {
  return createHash('sha256').update(data).digest('hex');
}

function isStoreControlledPackage(releasePackage) {
  const sourceType = typeof releasePackage?.sourceType === 'string'
    ? releasePackage.sourceType.toUpperCase()
    : '';
  return STORE_CONTROLLED_SOURCE_TYPES.has(sourceType);
}

function uniqueStrings(values) {
  return [...new Set(
    values
      .map((value) => String(value ?? '').trim())
      .filter(Boolean),
  )];
}

function isObject(value) {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function cloneJson(value) {
  return JSON.parse(JSON.stringify(value));
}

async function main(argv = process.argv.slice(2)) {
  const settings = parseReleaseEvidenceSyncArgs(argv);
  if (settings.help) {
    printHelp();
    return 0;
  }

  const plan = await createSdkworkImReleaseEvidenceSyncPlan({
    evidenceRoot: settings.evidenceRoot ?? DEFAULT_EVIDENCE_ROOT,
    manifestPath: settings.manifestPath ?? DEFAULT_MANIFEST_PATH,
    packageIds: settings.packageIds,
    packagesDir: settings.packagesDir ?? DEFAULT_PACKAGES_DIR,
  });
  const result = await applySdkworkImReleaseEvidenceSync(plan, {
    write: settings.write,
  });
  const payload = {
    ok: plan.ok,
    issues: plan.issues,
    written: result.written,
    manifestPath: plan.manifestPath,
    packages: plan.packages.map((releasePackage) => ({
      packageId: releasePackage.packageId,
      archive: {
        ref: releasePackage.archive.ref,
        sha256: releasePackage.archive.sha256,
        sizeBytes: releasePackage.archive.sizeBytes,
      },
      signature: releasePackage.signature,
      sbom: releasePackage.sbom,
      provenance: releasePackage.provenance,
    })),
  };

  if (settings.json) {
    console.log(JSON.stringify(payload, null, 2));
  } else {
    renderReleaseEvidenceSyncResult(payload);
  }

  return plan.ok ? 0 : 1;
}

function renderReleaseEvidenceSyncResult(payload) {
  console.log(`[sdkwork-im-release-evidence] manifest: ${payload.manifestPath}`);
  console.log(`[sdkwork-im-release-evidence] packages: ${payload.packages.length}`);
  console.log(`[sdkwork-im-release-evidence] written: ${payload.written}`);
  for (const releasePackage of payload.packages) {
    console.log(`[sdkwork-im-release-evidence]   ${releasePackage.packageId} sha256=${releasePackage.archive.sha256}`);
  }
  if (payload.issues.length > 0) {
    console.error('[sdkwork-im-release-evidence] validation issues:');
    for (const issue of payload.issues) {
      console.error(`[sdkwork-im-release-evidence]   ${issue}`);
    }
  }
}

if (process.argv[1] && path.resolve(process.argv[1]) === __filename) {
  main().then((code) => {
    process.exitCode = code;
  }).catch((error) => {
    console.error(`[sdkwork-im-release-evidence] ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  });
}

export {
  DEFAULT_EVIDENCE_ROOT,
  DEFAULT_MANIFEST_PATH,
  DEFAULT_PACKAGES_DIR,
  applySdkworkImReleaseEvidenceSync,
  createSdkworkImReleaseEvidenceSyncPlan,
  main,
  parseReleaseEvidenceSyncArgs,
};
