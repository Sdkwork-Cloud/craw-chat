#!/usr/bin/env node
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function fail(message) {
  console.error(`[sdk-release-catalog] ${message}`);
  process.exit(1);
}

function readJson(targetPath) {
  return JSON.parse(readFileSync(targetPath, 'utf8'));
}

function parseArgs(argv) {
  const parsed = {
    bundleId: 'wave-d-2026-04-08',
    check: false,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--bundle') {
      const value = (argv[index + 1] || '').trim();
      if (!value) {
        fail('Missing value for --bundle');
      }
      parsed.bundleId = value;
      index += 1;
      continue;
    }
    if (current === '--check') {
      parsed.check = true;
      continue;
    }

    fail(`Unknown argument: ${current}`);
  }

  return parsed;
}

function normalizeGenerationState(languageEntry) {
  if (languageEntry?.generationState === 'materialized' || languageEntry?.generationState === 'generated') {
    return 'generated';
  }
  if (Array.isArray(languageEntry?.packages) && languageEntry.packages.length > 0) {
    return 'generated';
  }
  return 'template_only_pending_generation';
}

function normalizeReleaseState(languageEntry) {
  return languageEntry?.releaseState === 'published' ? 'published' : 'not_published';
}

function buildArtifactEntry(workspaceConfig, languageEntry) {
  const packageName = workspaceConfig.packageField === 'publicPackage'
    ? languageEntry.publicPackage || languageEntry.workspace
    : languageEntry.workspace;
  const readmePath = workspaceConfig.readmePathFor
    ? workspaceConfig.readmePathFor(workspaceConfig, languageEntry)
    : `sdks/${workspaceConfig.workspace}/${languageEntry.workspace}/README.md`;
  return {
    id: `${workspaceConfig.audience}-${languageEntry.language}`,
    audience: workspaceConfig.audience,
    language: languageEntry.language,
    package: packageName,
    readmePath,
    plannedVersion: null,
    versionStatus: 'version_unassigned_pending_freeze',
    versionDecisionSourcePath: null,
    generationStatus: normalizeGenerationState(languageEntry),
    releaseStatus: normalizeReleaseState(languageEntry),
  };
}

function buildCatalog(repoRoot, bundleId) {
  const workspaceConfigs = [
    {
      audience: 'im',
      workspace: 'sdkwork-im-sdk',
      assemblyPath: path.join(repoRoot, 'sdks', 'sdkwork-im-sdk', '.sdkwork-assembly.json'),
    },
    {
      audience: 'app',
      workspace: 'sdkwork-im-app-sdk',
      assemblyPath: path.join(repoRoot, 'sdks', 'sdkwork-im-app-sdk', '.sdkwork-assembly.json'),
    },
    {
      audience: 'backend',
      workspace: 'sdkwork-im-backend-sdk',
      assemblyPath: path.join(repoRoot, 'sdks', 'sdkwork-im-backend-sdk', '.sdkwork-assembly.json'),
    },
    {
      audience: 'rtc',
      workspace: 'sdkwork-rtc-sdk',
      assemblyPath: path.join(repoRoot, '.sdkwork', 'dependencies', 'sdkwork-rtc', 'sdks', 'sdkwork-rtc-sdk', '.sdkwork-assembly.json'),
      packageField: 'publicPackage',
      readmePathFor: (workspaceConfig, languageEntry) =>
        `../../../sdkwork-rtc/sdks/${workspaceConfig.workspace}/${languageEntry.workspace}/README.md`,
    },
  ];

  const sdkArtifacts = workspaceConfigs.flatMap((workspaceConfig) => {
    if (!existsSync(workspaceConfig.assemblyPath)) {
      fail(`Missing SDK assembly: ${workspaceConfig.assemblyPath}`);
    }
    const assembly = readJson(workspaceConfig.assemblyPath);
    const languages = Array.isArray(assembly.languages) ? assembly.languages : [];
    return languages.map((languageEntry) => buildArtifactEntry(workspaceConfig, languageEntry));
  });

  const state = sdkArtifacts.every((entry) => entry.releaseStatus === 'published')
    ? 'published'
    : sdkArtifacts.every((entry) => entry.generationStatus === 'generated')
      ? 'generated_pending_publication'
      : 'template_only_pending_generation';

  return {
    $schema: '../schemas/sdk-release-catalog.schema.json',
    version: 1,
    bundleId,
    wave: bundleId.split('-').slice(0, 2).join('-'),
    artifact: 'sdk-release-catalog',
    state,
    updatedAt: new Date().toISOString().slice(0, 10),
    sdkArtifacts,
  };
}

const args = parseArgs(process.argv.slice(2));
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '..', '..');
const targetPath = path.join(scriptDir, args.bundleId, 'sdk-release-catalog.json');

if (!existsSync(path.dirname(targetPath))) {
  fail(`Missing release bundle directory: ${path.dirname(targetPath)}`);
}

const nextCatalogSource = `${JSON.stringify(buildCatalog(repoRoot, args.bundleId), null, 2)}\n`;
const currentCatalogSource = existsSync(targetPath) ? readFileSync(targetPath, 'utf8') : null;

if (args.check) {
  if (currentCatalogSource !== nextCatalogSource) {
    fail(`SDK release catalog drift detected: ${targetPath}`);
  }
  console.log(`[sdk-release-catalog] Up to date: ${targetPath}`);
  process.exit(0);
}

if (currentCatalogSource !== nextCatalogSource) {
  writeFileSync(targetPath, nextCatalogSource, 'utf8');
  console.log(`[sdk-release-catalog] Updated ${targetPath}`);
} else {
  console.log(`[sdk-release-catalog] No changes: ${targetPath}`);
}
