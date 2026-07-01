#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const failures = [];

const rootStandardDirectories = [
  'apis',
  'apps',
  'crates',
  'database',
  'sdks',
  'jobs',
  'tools',
  'plugins',
  'examples',
  'configs',
  'deployments',
  'scripts',
  'docs',
  'tests',
];

const rootDictionaryFiles = [
  'AGENTS.md',
  'CODEX.md',
  'CLAUDE.md',
  'GEMINI.md',
  '.sdkwork/README.md',
  '.sdkwork/.gitignore',
  '.sdkwork/skills/README.md',
  '.sdkwork/plugins/README.md',
  'sdkwork.app.config.json',
  'specs/README.md',
  'specs/component.spec.json',
];

const pcAppRoot = 'apps/sdkwork-im-pc';
const pcAppDictionaryFiles = [
  'AGENTS.md',
  'CODEX.md',
  'CLAUDE.md',
  'GEMINI.md',
  '.sdkwork/README.md',
  '.sdkwork/.gitignore',
  '.sdkwork/skills/README.md',
  '.sdkwork/plugins/README.md',
  'sdkwork.app.config.json',
  'specs/README.md',
  'specs/component.spec.json',
  'package.json',
];

function absolutePath(relativePath) {
  return path.join(repoRoot, relativePath);
}

function slashPath(value) {
  return String(value).replaceAll('\\', '/');
}

function readText(relativePath) {
  const filePath = absolutePath(relativePath);
  if (!fs.existsSync(filePath)) {
    failures.push(`${relativePath} must exist`);
    return '';
  }
  return fs.readFileSync(filePath, 'utf8');
}

function readJson(relativePath) {
  const text = readText(relativePath);
  if (!text) {
    return {};
  }
  try {
    return JSON.parse(text);
  } catch (error) {
    failures.push(`${relativePath} must contain valid JSON: ${error.message}`);
    return {};
  }
}

function assertManifestPathExists({ manifestPath, jsonPath, relativePath, expectedPath }) {
  assert(relativePath === expectedPath, `${manifestPath} ${jsonPath} must be ${expectedPath}`);
  assert(
    fs.existsSync(absolutePath(relativePath)),
    `${manifestPath} ${jsonPath} must resolve to an existing workspace path: ${relativePath}`,
  );
}

function gitLsFiles(relativePath) {
  const result = spawnSync('git', ['ls-files', '--', relativePath], {
    cwd: repoRoot,
    encoding: 'utf8',
  });
  if (result.status !== 0) {
    failures.push(`git ls-files ${relativePath} failed: ${(result.stderr || result.stdout).trim()}`);
    return [];
  }
  return result.stdout.split(/\r?\n/u).filter(Boolean);
}

function assert(condition, message) {
  if (!condition) {
    failures.push(message);
  }
}

function assertFile(relativePath) {
  const filePath = absolutePath(relativePath);
  assert(fs.existsSync(filePath) && fs.statSync(filePath).isFile(), `${relativePath} must exist`);
}

function assertDirectory(relativePath) {
  const directoryPath = absolutePath(relativePath);
  assert(
    fs.existsSync(directoryPath) && fs.statSync(directoryPath).isDirectory(),
    `${relativePath}/ must exist`,
  );
}

function assertReadmeDirectory(relativePath) {
  assertDirectory(relativePath);
  assertFile(`${relativePath}/README.md`);
}

function assertStandardDirectoryReadme(relativePath) {
  const readmePath = `${relativePath}/README.md`;
  const source = readText(readmePath);
  for (const requiredHeading of [
    '## Purpose',
    '## Owner',
    '## Allowed Content',
    '## Forbidden Content',
    '## Related Specs',
    '## Verification',
  ]) {
    assert(source.includes(requiredHeading), `${readmePath} must include ${requiredHeading}`);
  }
  assert(
    source.includes('../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md'),
    `${readmePath} must cite ../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`,
  );
}

function assertRelativeSpecPath(fromDirectory, relativeSpecPath) {
  const resolvedPath = path.resolve(absolutePath(fromDirectory), relativeSpecPath);
  assert(fs.existsSync(resolvedPath), `${fromDirectory}/${relativeSpecPath} must resolve`);
}

function parsePnpmWorkspacePackages(relativePath) {
  const text = readText(relativePath);
  const packages = [];
  let inPackages = false;
  for (const line of text.split(/\r?\n/u)) {
    if (/^packages:\s*$/u.test(line)) {
      inPackages = true;
      continue;
    }
    if (/^[A-Za-z0-9_'"][^:]*:\s*/u.test(line) && !/^packages:\s*$/u.test(line)) {
      inPackages = false;
    }
    if (!inPackages) {
      continue;
    }
    const match = line.match(/^\s*-\s+(.+?)\s*$/u);
    if (match) {
      packages.push(match[1].replace(/^['"]|['"]$/gu, ''));
    }
  }
  return packages;
}

function listPackageDirectories() {
  const packagesDir = absolutePath(`${pcAppRoot}/packages`);
  if (!fs.existsSync(packagesDir)) {
    failures.push(`${pcAppRoot}/packages must exist`);
    return [];
  }
  return fs.readdirSync(packagesDir, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => entry.name)
    .sort();
}

function assertNoGeneratedSdkOutputInRepositorySdkwork() {
  const sdkworkRoot = absolutePath('.sdkwork');
  const forbidden = [];
  function visit(directory) {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
      const entryPath = path.join(directory, entry.name);
      const relativePath = slashPath(path.relative(repoRoot, entryPath));
      if (entry.isDirectory()) {
        visit(entryPath);
        continue;
      }
      if (/\.sdkwork\/sdkwork-generator-(manifest|changes|report)\.json$/u.test(relativePath)) {
        forbidden.push(relativePath);
      }
    }
  }
  visit(sdkworkRoot);
  assert(
    forbidden.length === 0,
    `repository .sdkwork must not contain generated SDK control-plane files: ${forbidden.join(', ')}`,
  );
}

function assertRepositoryRootDictionary() {
  for (const relativePath of rootDictionaryFiles) {
    assertFile(relativePath);
  }
  for (const directory of rootStandardDirectories) {
    assertReadmeDirectory(directory);
    assertStandardDirectoryReadme(directory);
  }

  const agents = readText('AGENTS.md');
  assert(agents.includes('../sdkwork-specs/SOUL.md'), 'root AGENTS.md must cite ../sdkwork-specs/SOUL.md');
  assert(agents.includes('../sdkwork-specs/AGENTS_SPEC.md'), 'root AGENTS.md must cite ../sdkwork-specs/AGENTS_SPEC.md');
  assertRelativeSpecPath('.', '../sdkwork-specs/README.md');
  assertRelativeSpecPath('.', '../sdkwork-specs/SOUL.md');
  assertRelativeSpecPath('.', '../sdkwork-specs/AGENTS_SPEC.md');

  const sdkworkGitignore = readText('.sdkwork/.gitignore');
  assert(
    sdkworkGitignore.includes('dart/pub-cache/'),
    '.sdkwork/.gitignore must ignore dart/pub-cache/ local cache state',
  );
  const rootGitignore = readText('.gitignore');
  assert(
    rootGitignore.includes('/.sdkwork/dart/pub-cache/'),
    '.gitignore must ignore root .sdkwork/dart/pub-cache/',
  );
  assertNoGeneratedSdkOutputInRepositorySdkwork();
}

function assertPcAppRootDictionary() {
  for (const relativePath of pcAppDictionaryFiles) {
    assertFile(`${pcAppRoot}/${relativePath}`);
  }

  const agents = readText(`${pcAppRoot}/AGENTS.md`);
  assert(
    agents.includes('../../../sdkwork-specs/SOUL.md'),
    'PC app AGENTS.md must cite ../../../sdkwork-specs/SOUL.md',
  );
  assert(
    agents.includes('../../../sdkwork-specs/APP_PC_ARCHITECTURE_SPEC.md'),
    'PC app AGENTS.md must cite APP_PC_ARCHITECTURE_SPEC.md',
  );
  for (const shim of ['CODEX.md', 'CLAUDE.md', 'GEMINI.md']) {
    const shimText = readText(`${pcAppRoot}/${shim}`);
    assert(shimText.includes('AGENTS.md'), `${pcAppRoot}/${shim} must point to AGENTS.md`);
    assert(
      shimText.includes('../../../sdkwork-specs/README.md'),
      `${pcAppRoot}/${shim} must cite ../../../sdkwork-specs/README.md`,
    );
  }
  assertRelativeSpecPath(pcAppRoot, '../../../sdkwork-specs/README.md');
  assertRelativeSpecPath(pcAppRoot, '../../../sdkwork-specs/SOUL.md');
  assertRelativeSpecPath(pcAppRoot, '../../../sdkwork-specs/APP_PC_ARCHITECTURE_SPEC.md');

  const appSdkworkGitignore = readText(`${pcAppRoot}/.sdkwork/.gitignore`);
  assert(
    appSdkworkGitignore.includes('dart/pub-cache/'),
    `${pcAppRoot}/.sdkwork/.gitignore must ignore dart/pub-cache/ local cache state`,
  );
}

function assertAppManifestWorkspaceRoots() {
  const rootManifestPath = 'sdkwork.app.config.json';
  const rootManifest = readJson(rootManifestPath);
  assertManifestPathExists({
    manifestPath: rootManifestPath,
    jsonPath: 'publish.config.workspaceRoot',
    relativePath: rootManifest.publish?.config?.workspaceRoot,
    expectedPath: pcAppRoot,
  });
  assertManifestPathExists({
    manifestPath: rootManifestPath,
    jsonPath: 'artifacts.installConfig.metadata.workspaceRoot',
    relativePath: rootManifest.artifacts?.installConfig?.metadata?.workspaceRoot,
    expectedPath: pcAppRoot,
  });
  assertManifestPathExists({
    manifestPath: rootManifestPath,
    jsonPath: 'devApp.sourceRoot',
    relativePath: rootManifest.devApp?.sourceRoot,
    expectedPath: pcAppRoot,
  });

  const pcManifestPath = `${pcAppRoot}/sdkwork.app.config.json`;
  const pcManifest = readJson(pcManifestPath);
  assertManifestPathExists({
    manifestPath: pcManifestPath,
    jsonPath: 'publish.config.workspaceRoot',
    relativePath: pcManifest.publish?.config?.workspaceRoot,
    expectedPath: pcAppRoot,
  });
  assertManifestPathExists({
    manifestPath: pcManifestPath,
    jsonPath: 'artifacts.installConfig.metadata.workspaceRoot',
    relativePath: pcManifest.artifacts?.installConfig?.metadata?.workspaceRoot,
    expectedPath: pcAppRoot,
  });
  assertManifestPathExists({
    manifestPath: pcManifestPath,
    jsonPath: 'devApp.sourceRoot',
    relativePath: pcManifest.devApp?.sourceRoot,
    expectedPath: pcAppRoot,
  });
}

function assertPnpmWorkspaceAuthority() {
  assertFile('pnpm-workspace.yaml');
  const rootPackages = parsePnpmWorkspacePackages('pnpm-workspace.yaml');
  const nestedWorkspacePath = `${pcAppRoot}/pnpm-workspace.yaml`;

  assert(
    !fs.existsSync(absolutePath(nestedWorkspacePath)),
    `${nestedWorkspacePath} must not exist; repository root pnpm-workspace.yaml is the only workspace authority`,
  );
  assert(rootPackages.includes(pcAppRoot), 'root pnpm-workspace.yaml must include apps/sdkwork-im-pc');
  assert(
    rootPackages.includes(`${pcAppRoot}/packages/*`),
    'root pnpm-workspace.yaml must include apps/sdkwork-im-pc/packages/*',
  );

  const packageJson = readJson('package.json');
  assert(
    packageJson.scripts?.['test:sdkwork-workspace-structure-standard']
      === 'node scripts/sdkwork-workspace-structure-standard.test.mjs',
    'package.json must expose test:sdkwork-workspace-structure-standard',
  );
}

function assertPcPackageNamingCompatibilityIsDocumented() {
  const packageDirectories = listPackageDirectories();
  const canonicalConsolePackages = packageDirectories.filter((name) => /^sdkwork-im-console-/u.test(name));
  const canonicalAdminPackages = packageDirectories.filter((name) => /^sdkwork-im-admin-/u.test(name));
  const invalidNewPackages = packageDirectories.filter((name) =>
    /^sdkwork-im-(console|admin)-/u.test(name)
    && !canonicalConsolePackages.includes(name)
    && !canonicalAdminPackages.includes(name)
  );

  assert(invalidNewPackages.length === 0, `unexpected PC package names: ${invalidNewPackages.join(', ')}`);

  const rootSpecs = readText('specs/README.md');
  const appSpecs = readText(`${pcAppRoot}/specs/README.md`);
  for (const [relativePath, source] of [
    ['specs/README.md', rootSpecs],
    [`${pcAppRoot}/specs/README.md`, appSpecs],
    [`${pcAppRoot}/AGENTS.md`, readText(`${pcAppRoot}/AGENTS.md`)],
  ]) {
    assert(
      source.includes('sdkwork-im-console-*') && source.includes('sdkwork-im-admin-*'),
      `${relativePath} must document canonical console/admin package naming`,
    );
    assert(
      source.includes('sdkwork-im-pc-console-*') && source.includes('sdkwork-im-pc-admin-*'),
      `${relativePath} must document normalized PC console/admin package targets`,
    );
  }
}

function assertPubCacheIsNotTracked() {
  const trackedPubCacheFiles = gitLsFiles('.sdkwork/dart/pub-cache');
  assert(
    trackedPubCacheFiles.length === 0,
    `.sdkwork/dart/pub-cache must not be tracked source state; found ${trackedPubCacheFiles.length} tracked files`,
  );
}

function assertSpecsDocumentNewVerification() {
  const specsReadme = readText('specs/README.md');
  assert(
    specsReadme.includes('node scripts/sdkwork-workspace-structure-standard.test.mjs'),
    'specs/README.md must list the workspace structure verification command',
  );
  const appSpecsReadme = readText(`${pcAppRoot}/specs/README.md`);
  assert(
    appSpecsReadme.includes('repository root `pnpm-workspace.yaml`'),
    `${pcAppRoot}/specs/README.md must document root pnpm workspace authority`,
  );
}

function extractCargoManifestPaths(text) {
  return Array.from(text.matchAll(/cargo\s+test\s+--manifest-path\s+([^`\s]+)/gu))
    .map((match) => match[1]);
}

function assertDocumentedCargoManifestPathsExist() {
  const documentationSources = [
    ['README.md', readText('README.md')],
    ['specs/README.md', readText('specs/README.md')],
  ];
  const componentSpec = readJson('specs/component.spec.json');
  const verificationCommands = componentSpec.verification?.commands ?? [];
  documentationSources.push(['specs/component.spec.json verification.commands', verificationCommands.join('\n')]);

  for (const [sourceName, sourceText] of documentationSources) {
    for (const manifestPath of extractCargoManifestPaths(sourceText)) {
      assert(
        fs.existsSync(absolutePath(manifestPath)),
        `${sourceName} documents a cargo manifest path that must exist: ${manifestPath}`,
      );
    }
  }
}

function assertRepositoryRootRtcSdkVerifierCommand() {
  const commandDocumentationSources = [
    ['sdks/README.md', readText('sdks/README.md')],
    ['docs/架构/sdkwork-im-rtc-complete-integration-guide.md', readText('docs/架构/sdkwork-im-rtc-complete-integration-guide.md')],
  ];
  const expectedCommand = 'node ..\\sdkwork-rtc\\sdks\\sdkwork-rtc-sdk\\bin\\verify-sdk.mjs';
  const staleCommand = 'node ../../sdkwork-rtc\\sdks\\sdkwork-rtc-sdk\\bin\\verify-sdk.mjs';
  const staleDeepCommand = 'node ../../../sdkwork-rtc\\sdks\\sdkwork-rtc-sdk\\bin\\verify-sdk.mjs';
  for (const [relativePath, source] of commandDocumentationSources) {
    assert(
      source.includes(expectedCommand),
      `${relativePath} must document the repository-root RTC SDK verifier command: ${expectedCommand}`,
    );
    for (const forbiddenCommand of [staleCommand, staleDeepCommand]) {
      assert(
        !source.includes(forbiddenCommand),
        `${relativePath} must not document the over-deep repository-root RTC SDK verifier command: ${forbiddenCommand}`,
      );
    }
  }
}

assertRepositoryRootDictionary();
assertPcAppRootDictionary();
assertAppManifestWorkspaceRoots();
assertPnpmWorkspaceAuthority();
assertPcPackageNamingCompatibilityIsDocumented();
assertPubCacheIsNotTracked();
assertSpecsDocumentNewVerification();
assertDocumentedCargoManifestPathsExist();
assertRepositoryRootRtcSdkVerifierCommand();

if (failures.length > 0) {
  process.stderr.write(`SDKWork workspace structure standard failed:\n${failures.map((failure) => `- ${failure}`).join('\n')}\n`);
  process.exit(1);
}

process.stdout.write('SDKWork workspace structure standard passed\n');
