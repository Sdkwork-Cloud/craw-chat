#!/usr/bin/env node
import { rmSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { sdkFamilyConfig } from './sdk-family-config.mjs';

const familyRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const sdkName = 'sdkwork-im-sdk';
const configuredClientFiles = [
  sdkFamilyConfig.primaryClient,
  sdkFamilyConfig.legacyClient,
]
  .filter((clientName, index, clientNames) => clientName && clientNames.indexOf(clientName) === index)
  .map((clientName) => `${clientName}.cs`);

const generatedTargetsByLanguage = {
  typescript: [
    '.sdkwork/manual-backups',
    '.sdkwork/sdkwork-generator-changes.json',
    '.sdkwork/sdkwork-generator-manifest.json',
    '.sdkwork/sdkwork-generator-report.json',
    'CHANGELOG.md',
    'LICENSE',
    'README.md',
    'dist',
    'package.json',
    'sdkwork-sdk.json',
    'src',
    'tsconfig.json',
  ],
  flutter: [
    '.sdkwork/manual-backups',
    '.sdkwork/sdkwork-generator-changes.json',
    '.sdkwork/sdkwork-generator-manifest.json',
    '.sdkwork/sdkwork-generator-report.json',
    'CHANGELOG.md',
    'LICENSE',
    'README.md',
    'lib',
    'pubspec.yaml',
    'pubspec_overrides.yaml',
    'sdkwork-sdk.json',
  ],
  rust: [
    '.sdkwork/manual-backups',
    '.sdkwork/sdkwork-generator-changes.json',
    '.sdkwork/sdkwork-generator-manifest.json',
    '.sdkwork/sdkwork-generator-report.json',
    'CHANGELOG.md',
    'Cargo.toml',
    'LICENSE',
    'README.md',
    'sdkwork-sdk.json',
    'src',
  ],
  java: [
    '.sdkwork/manual-backups',
    '.sdkwork/sdkwork-generator-changes.json',
    '.sdkwork/sdkwork-generator-manifest.json',
    '.sdkwork/sdkwork-generator-report.json',
    'CHANGELOG.md',
    'LICENSE',
    'README.md',
    'pom.xml',
    'sdkwork-sdk.json',
    'src',
  ],
  csharp: [
    '.sdkwork/manual-backups',
    '.sdkwork/sdkwork-generator-changes.json',
    '.sdkwork/sdkwork-generator-manifest.json',
    '.sdkwork/sdkwork-generator-report.json',
    'Api',
    'CHANGELOG.md',
    'Http',
    'LICENSE',
    'Models',
    'README.md',
    'Sdkwork.Im.Sdk.Generated.csproj',
    ...configuredClientFiles,
    'sdkwork-sdk.json',
  ],
  swift: [
    '.sdkwork/manual-backups',
    '.sdkwork/sdkwork-generator-changes.json',
    '.sdkwork/sdkwork-generator-manifest.json',
    '.sdkwork/sdkwork-generator-report.json',
    'CHANGELOG.md',
    'LICENSE',
    'Package.swift',
    'README.md',
    'Sources',
    'sdkwork-sdk.json',
  ],
  kotlin: [
    '.sdkwork/manual-backups',
    '.sdkwork/sdkwork-generator-changes.json',
    '.sdkwork/sdkwork-generator-manifest.json',
    '.sdkwork/sdkwork-generator-report.json',
    'CHANGELOG.md',
    'LICENSE',
    'README.md',
    'build.gradle.kts',
    'sdkwork-sdk.json',
    'settings.gradle.kts',
    'src',
  ],
  go: [
    '.sdkwork/manual-backups',
    '.sdkwork/sdkwork-generator-changes.json',
    '.sdkwork/sdkwork-generator-manifest.json',
    '.sdkwork/sdkwork-generator-report.json',
    'CHANGELOG.md',
    'LICENSE',
    'README.md',
    'api',
    'doc.go',
    'go.mod',
    'http',
    'sdk.go',
    'sdkwork-sdk.json',
    'types',
  ],
  python: [
    '.sdkwork/manual-backups',
    '.sdkwork/sdkwork-generator-changes.json',
    '.sdkwork/sdkwork-generator-manifest.json',
    '.sdkwork/sdkwork-generator-report.json',
    'CHANGELOG.md',
    'LICENSE',
    'MANIFEST.in',
    'README.md',
    'pyproject.toml',
    'requirements.txt',
    'sdkwork-sdk.json',
    'sdkwork_im_sdk_generated',
    'setup.py',
  ],
};

function fail(message) {
  console.error(`[${sdkName}] ${message}`);
  process.exit(1);
}

function parseLanguage(argv) {
  for (let index = 0; index < argv.length; index += 1) {
    if (argv[index] === '--language') {
      return argv[index + 1] || '';
    }
  }
  fail('prepare-generated-output requires --language');
}

function assertInside(parent, candidate) {
  const relative = path.relative(parent, candidate);
  if (relative.startsWith('..') || path.isAbsolute(relative)) {
    fail(`refusing to remove path outside SDK family: ${candidate}`);
  }
}

const language = parseLanguage(process.argv.slice(2));
const targets = generatedTargetsByLanguage[language];
if (!targets) {
  fail(`unsupported generated-output language: ${language}`);
}

const outputRoot = path.join(
  familyRoot,
  `${sdkName}-${language}`,
  'generated',
  'server-openapi',
);
assertInside(familyRoot, outputRoot);

for (const relativeTarget of targets) {
  const target = path.join(outputRoot, relativeTarget);
  assertInside(outputRoot, target);
  rmSync(target, {
    force: true,
    maxRetries: 5,
    recursive: true,
    retryDelay: 100,
  });
}

console.log(`[${sdkName}] prepared generated output for ${language}`);
