#!/usr/bin/env node

import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function repoPath(...segments) {
  return path.join(repoRoot, ...segments);
}

function readText(...segments) {
  return readFileSync(repoPath(...segments), 'utf8');
}

function assertFile(relativePath) {
  assert.equal(
    existsSync(repoPath(...relativePath.split('/'))),
    true,
    `${relativePath} should exist`,
  );
}

async function importRepoModule(relativePath) {
  return await import(pathToFileURL(repoPath(...relativePath.split('/'))).href);
}

const rootPackageJson = JSON.parse(readText('package.json'));

for (const [scriptName, expectedCommand] of Object.entries({
  'release:plan': 'pnpm exec sdkwork-app release:plan',
  'release:build:prod': 'node scripts/release/build-sdkwork-im-production.mjs',
  'release:stage': 'node scripts/release/stage-sdkwork-im-release-package.mjs',
  'release:package': 'pnpm exec sdkwork-app release:package',
  'release:package:check': 'node scripts/release/build-sdkwork-im-install-package.mjs --check --dry-run --all',
  'release:validate': 'pnpm exec sdkwork-app release:validate',
  'release:validate:evidence': 'node scripts/release/sync-sdkwork-im-release-evidence.mjs --check',
  'release:stage:evidence': 'node scripts/release/sync-sdkwork-im-release-evidence.mjs --write',
  'release:build:desktop': 'node scripts/release/build-sdkwork-im-production.mjs --target desktop',
})) {
  assert.equal(rootPackageJson.scripts?.[scriptName], expectedCommand, `package.json script ${scriptName}`);
}

for (const relativePath of [
  'scripts/release/sdkwork-im-release-version.mjs',
  'scripts/release/plan-sdkwork-im-install-packages.mjs',
  'scripts/release/build-sdkwork-im-production.mjs',
  'scripts/release/stage-sdkwork-im-release-package.mjs',
  'scripts/release/build-sdkwork-im-install-package.mjs',
  'scripts/release/collect-sdkwork-im-desktop-bundles.mjs',
  'scripts/release/desktop-targets.mjs',
  'scripts/release/validate-sdkwork-im-install-artifacts.mjs',
  'scripts/release/sync-sdkwork-im-release-evidence.mjs',
]) {
  assertFile(relativePath);
}

const planModule = await importRepoModule('scripts/release/plan-sdkwork-im-install-packages.mjs');
for (const exportName of [
  'createSdkworkImInstallPackagePlan',
  'validateSdkworkImInstallPackagePlan',
  'renderSdkworkImInstallPackagePlan',
  'SUPPORTED_PLATFORMS',
  'SUPPORTED_ARCHITECTURES',
  'SUPPORTED_PACKAGE_PROFILES',
]) {
  assert.equal(typeof planModule[exportName], exportName.startsWith('SUPPORTED_') ? 'object' : 'function', `${exportName} export`);
}

const releasePlan = planModule.createSdkworkImInstallPackagePlan({ version: '1.2.3' });
const planIssues = planModule.validateSdkworkImInstallPackagePlan(releasePlan);
assert.deepEqual(planIssues, [], `release package plan issues: ${planIssues.join('; ')}`);
const renderedReleasePlan = planModule.renderSdkworkImInstallPackagePlan(releasePlan).join('\n');
for (const expectedText of [
  'paths=install=/opt/sdkwork/chat config=/etc/sdkwork/chat data=/var/lib/sdkwork/chat log=/var/log/sdkwork/chat run=/run/sdkwork/chat',
  'paths=install=/usr/lib/sdkwork/chat config=/Library/Application Support/sdkwork/chat data=/Library/Application Support/sdkwork/chat/Data log=/Library/Logs/sdkwork/chat run=/Library/Application Support/sdkwork/chat/Run',
  'paths=install=%ProgramFiles%/sdkwork/chat config=%ProgramData%/sdkwork/chat data=%ProgramData%/sdkwork/chat/Data log=%ProgramData%/sdkwork/chat/Logs run=%ProgramData%/sdkwork/chat/Run',
]) {
  assert.match(renderedReleasePlan, new RegExp(expectedText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'));
}
assert.equal(releasePlan.appCode, 'chat');
assert.equal(releasePlan.runtimeName, 'chat');
assert.equal(releasePlan.product, 'chat');
assert.equal(releasePlan.packageName, 'sdkwork-chat');
assert.equal(releasePlan.artifactPolicy?.noSecretsInPackage, true);
assert.equal(releasePlan.artifactPolicy?.envLocalGeneratedOnHost, true);
assert.equal(releasePlan.artifactPolicy?.generatedFromProductionBuild, true);
assert.deepEqual(releasePlan.deploymentProfiles, ['cloud', 'standalone']);
assert.deepEqual(releasePlan.profiles, ['browser', 'server', 'desktop']);

const serverPackagesByPlatform = new Map(
  releasePlan.packages
    .filter((item) => item.profile === 'server' && item.architecture === 'x64')
    .map((item) => [item.platform, item]),
);
assert.equal(
  serverPackagesByPlatform.get('linux')?.databasePolicy?.configFile?.path,
  '/etc/sdkwork/chat/chat.toml',
  'linux server config file should use the Ubuntu production config root',
);
assert.deepEqual(
  serverPackagesByPlatform.get('linux')?.runtimePaths,
  {
    installRoot: '/opt/sdkwork/chat',
    configDir: '/etc/sdkwork/chat',
    dataDir: '/var/lib/sdkwork/chat',
    logDir: '/var/log/sdkwork/chat',
    runDir: '/run/sdkwork/chat',
  },
  'linux server package should expose the complete Ubuntu production path matrix',
);
assert.equal(
  serverPackagesByPlatform.get('linux')?.databasePolicy?.dataDirectory?.path,
  '/var/lib/sdkwork/chat',
  'linux server data directory should use the SDKWork production data root',
);
assert.equal(
  serverPackagesByPlatform.get('linux')?.databasePolicy?.passwordFile?.path,
  '/etc/sdkwork/database/database.secret',
  'linux PostgreSQL password file should stay under the production config root',
);
assert.equal(
  serverPackagesByPlatform.get('windows')?.databasePolicy?.configFile?.path,
  '%ProgramData%/sdkwork/chat/chat.toml',
  'windows server config file should use ProgramData Sdkwork IM config root',
);
assert.deepEqual(
  serverPackagesByPlatform.get('windows')?.runtimePaths,
  {
    installRoot: '%ProgramFiles%/sdkwork/chat',
    configDir: '%ProgramData%/sdkwork/chat',
    dataDir: '%ProgramData%/sdkwork/chat/Data',
    logDir: '%ProgramData%/sdkwork/chat/Logs',
    runDir: '%ProgramData%/sdkwork/chat/Run',
  },
  'windows server package should expose the complete ProgramData path matrix',
);
assert.equal(
  serverPackagesByPlatform.get('windows')?.databasePolicy?.dataDirectory?.path,
  '%ProgramData%/sdkwork/chat/Data',
  'windows server data directory should use ProgramData Sdkwork IM data root',
);
assert.equal(
  serverPackagesByPlatform.get('windows')?.databasePolicy?.passwordFile?.path,
  '%ProgramData%/sdkwork/chat/database.secret',
  'windows PostgreSQL password file should stay under ProgramData Sdkwork IM config root',
);
assert.equal(
  serverPackagesByPlatform.get('macos')?.databasePolicy?.configFile?.path,
  '/Library/Application Support/sdkwork/chat/chat.toml',
  'macOS server config file should use the Sdkwork IM application support config root',
);
assert.deepEqual(
  serverPackagesByPlatform.get('macos')?.runtimePaths,
  {
    installRoot: '/usr/lib/sdkwork/chat',
    configDir: '/Library/Application Support/sdkwork/chat',
    dataDir: '/Library/Application Support/sdkwork/chat/Data',
    logDir: '/Library/Logs/sdkwork/chat',
    runDir: '/Library/Application Support/sdkwork/chat/Run',
  },
  'macOS server package should expose the complete application support path matrix',
);
assert.equal(
  serverPackagesByPlatform.get('macos')?.databasePolicy?.dataDirectory?.path,
  '/Library/Application Support/sdkwork/chat/Data',
  'macOS server data directory should use the Sdkwork IM application support data root',
);
assert.equal(
  serverPackagesByPlatform.get('macos')?.databasePolicy?.passwordFile?.path,
  '/Library/Application Support/sdkwork/chat/database.secret',
  'macOS PostgreSQL password file should stay under the Sdkwork IM config root',
);

const expectedPackageIds = [
  'web-universal-cloud-browser-zip',
  'linux-x64-standalone-server-tar-gz',
  'linux-arm64-standalone-server-tar-gz',
  'macos-x64-standalone-server-tar-gz',
  'macos-arm64-standalone-server-tar-gz',
  'windows-x64-standalone-server-zip',
  'windows-arm64-standalone-server-zip',
  'linux-x64-standalone-desktop-zip',
  'linux-arm64-standalone-desktop-zip',
  'macos-x64-standalone-desktop-zip',
  'macos-arm64-standalone-desktop-zip',
  'windows-x64-standalone-desktop-zip',
  'windows-arm64-standalone-desktop-zip',
];
assert.deepEqual(releasePlan.packages.map((item) => item.id), expectedPackageIds);

const subsetPlanJson = execFileSync(
  process.execPath,
  [
    'scripts/release/plan-sdkwork-im-install-packages.mjs',
    '--platform',
    'windows',
    '--architecture',
    'x64',
    '--profile',
    'desktop',
    '--version',
    '1.2.3',
    '--json',
  ],
  { cwd: repoRoot, encoding: 'utf8' },
);
const subsetPlanPayload = JSON.parse(subsetPlanJson);
assert.equal(subsetPlanPayload.ok, true, 'filtered release plan should validate');
assert.deepEqual(
  subsetPlanPayload.plan.packages.map((item) => item.id),
  ['windows-x64-standalone-desktop-zip'],
  'filtered release plan should include only the requested package id',
);

for (const packageItem of releasePlan.packages) {
  assert.equal(packageItem.security?.noSecretsInPackage, true, `${packageItem.id} no secrets policy`);
  assert.equal(packageItem.version, '1.2.3', `${packageItem.id} version`);
  if (packageItem.profile === 'browser') {
    assert.equal(packageItem.deploymentProfile, 'cloud', `${packageItem.id} deployment profile`);
    assert.equal(packageItem.runtimeTarget, 'browser', `${packageItem.id} runtime target`);
    assert.equal(packageItem.platform, 'web', `${packageItem.id} platform`);
    assert.equal(packageItem.architecture, 'universal', `${packageItem.id} architecture`);
    assert.equal(packageItem.format, 'zip', `${packageItem.id} format`);
    assert.equal(packageItem.databasePolicy, null, `${packageItem.id} database policy`);
    for (const expectedArtifact of ['pc-web-dist', 'web-manifest']) {
      assert.equal(
        packageItem.artifacts.some((artifact) => artifact.kind === expectedArtifact && artifact.required === true),
        true,
        `${packageItem.id} should include ${expectedArtifact}`,
      );
    }
    assert.equal(
      packageItem.artifacts.some((artifact) => artifact.kind === 'h5-web-dist' && artifact.required === false),
      true,
      `${packageItem.id} should include optional h5-web-dist`,
    );
    assert.match(packageItem.archiveName, /^sdkwork-im-web-universal-cloud-browser-zip-.+\.zip$/u);
  } else if (packageItem.profile === 'server') {
    assert.equal(packageItem.deploymentProfile, 'standalone', `${packageItem.id} deployment profile`);
    assert.equal(packageItem.runtimeTarget, 'server', `${packageItem.id} runtime target`);
    assert.equal(packageItem.databasePolicy?.defaultEngine, 'postgresql', `${packageItem.id} database engine`);
    assert.equal(packageItem.databasePolicy?.requiresExternalDatabase, true, `${packageItem.id} external database`);
    assert.equal(packageItem.databasePolicy?.defaultDatabase, 'sdkwork', `${packageItem.id} production database`);
    assert.equal(packageItem.databasePolicy?.defaultUsername, 'sdkwork', `${packageItem.id} production database user`);
    for (const expectedArtifact of [
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
      assert.equal(
        packageItem.artifacts.some((artifact) => artifact.kind === expectedArtifact && artifact.required === true),
        true,
        `${packageItem.id} should include ${expectedArtifact}`,
      );
    }
    assert.equal(
      packageItem.artifacts.some((artifact) => artifact.kind === 'h5-web-dist' && artifact.required === false),
      true,
      `${packageItem.id} should include optional h5-web-dist`,
    );
    assert.match(packageItem.archiveName, /^sdkwork-im-.+standalone-server-.+\.(zip|tar\.gz)$/u);
  } else {
    assert.equal(packageItem.deploymentProfile, 'standalone', `${packageItem.id} deployment profile`);
    assert.equal(packageItem.runtimeTarget, 'desktop', `${packageItem.id} runtime target`);
    assert.equal(packageItem.databasePolicy?.defaultEngine, 'postgresql', `${packageItem.id} database engine`);
    assert.equal(packageItem.databasePolicy?.requiresExternalDatabase, true, `${packageItem.id} external database`);
    assert.equal(packageItem.databasePolicy?.passwordFile?.required, true, `${packageItem.id} password file required`);
    assert.ok(
      packageItem.databasePolicy?.envOverrides?.includes('SDKWORK_DATABASE_PASSWORD_FILE'),
      `${packageItem.id} database password file env override`,
    );
    for (const expectedArtifact of ['desktop-installers', 'desktop-manifest']) {
      assert.equal(
        packageItem.artifacts.some((artifact) => artifact.kind === expectedArtifact && artifact.required === true),
        true,
        `${packageItem.id} should include ${expectedArtifact}`,
      );
    }
    assert.match(packageItem.archiveName, /^sdkwork-im-.+standalone-desktop-zip-.+\.zip$/u);
  }
}

const packageBuilder = await importRepoModule('scripts/release/build-sdkwork-im-install-package.mjs');
const desktopBundleCollector = await importRepoModule('scripts/release/collect-sdkwork-im-desktop-bundles.mjs');
const productionDryRunJson = execFileSync(
  process.execPath,
  ['scripts/release/build-sdkwork-im-production.mjs', '--target', 'desktop', '--dry-run', '--json'],
  {
    cwd: repoRoot,
    encoding: 'utf8',
    env: {
      ...process.env,
      SDKWORK_IM_TEST_SECRET: 'do-not-leak-release-secret',
    },
  },
);
assert.doesNotMatch(productionDryRunJson, /do-not-leak-release-secret/u, 'production dry-run JSON must not leak env values');
assert.doesNotMatch(productionDryRunJson, /"env"\s*:/u, 'production dry-run JSON must not expose raw env objects');
const productionDryRunPayload = JSON.parse(productionDryRunJson);
assert.deepEqual(
  productionDryRunPayload.plan.steps.map((step) => step.label),
  [
    'build sdkwork-im-pc web assets',
    'build desktop installer x86_64-pc-windows-msvc',
  ],
  'desktop-only production build should prepare web assets before Tauri packaging',
);
const browserProductionDryRunPayload = JSON.parse(execFileSync(
  process.execPath,
  ['scripts/release/build-sdkwork-im-production.mjs', '--target', 'browser', '--dry-run', '--json'],
  {
    cwd: repoRoot,
    encoding: 'utf8',
    env: {
      ...process.env,
      SDKWORK_IM_TEST_SECRET: 'do-not-leak-release-secret',
    },
  },
));
assert.deepEqual(
  browserProductionDryRunPayload.plan.steps.map((step) => step.label),
  [
    'build sdkwork-im-pc web assets',
    'build sdkwork-im-h5 web assets',
  ],
  'browser production build should build both adaptive browser renderers',
);

const dryRunBuildPlan = packageBuilder.createSdkworkImInstallPackageBuildPlan({
  packageId: 'windows-x64-standalone-server-zip',
  version: '1.2.3',
  requireStagedFiles: false,
});
assert.equal(
  packageBuilder.validateSdkworkImInstallPackageBuildPlan(dryRunBuildPlan).length,
  0,
  'dry-run package build plan should be valid without staged files',
);
for (const entry of dryRunBuildPlan.entries) {
  assert.doesNotMatch(entry.archivePath, /(^|\/)\.env($|\.|\/)|secret|secrets\/|node_modules|\.runtime/u);
  assert.doesNotMatch(entry.archivePath, /\.\.|^[A-Za-z]:|^\/|\\/u);
}

const validatorModule = await importRepoModule('scripts/release/validate-sdkwork-im-install-artifacts.mjs');
for (const exportName of [
  'parseValidateArgs',
  'readTarEntries',
  'readZipEntries',
  'validateSdkworkImInstallArtifact',
  'validateTarGzArtifact',
  'validateZipArtifact',
]) {
  assert.equal(typeof validatorModule[exportName], 'function', `${exportName} export`);
}

const validatorTempRoot = mkTempDir('sdkwork-im-release-validator-');
try {
  const browserStage = path.join(validatorTempRoot, 'stage', 'web-universal-cloud-browser-zip');
  const serverStage = path.join(validatorTempRoot, 'stage', 'windows-x64-standalone-server-zip');
  const desktopStage = path.join(validatorTempRoot, 'stage', 'windows-x64-standalone-desktop-zip');
  const outputDir = path.join(validatorTempRoot, 'out');
  writeFixture(browserStage, 'web/sdkwork-im-pc/dist/index.html', '<!doctype html>');
  writeFixture(browserStage, 'web-manifest.json', '{"product":"chat"}');
  const browserBuildPlan = packageBuilder.createSdkworkImInstallPackageBuildPlan({
    outputDir,
    packageId: 'web-universal-cloud-browser-zip',
    root: repoRoot,
    stagingRoot: browserStage,
    version: '1.2.3',
  });
  const browserArchive = await packageBuilder.buildSdkworkImInstallPackageArchive(browserBuildPlan);
  assert.equal(browserArchive.manifest.package.deploymentProfile, 'cloud');
  assert.equal(browserArchive.manifest.package.profile, 'browser');
  assert.equal(browserArchive.manifest.package.runtimeTarget, 'browser');
  assert.equal(browserArchive.manifest.package.format, 'zip');
  const browserValidation = validatorModule.validateSdkworkImInstallArtifact({
    artifactPath: browserArchive.archivePath,
    packageId: 'web-universal-cloud-browser-zip',
    root: repoRoot,
    version: '1.2.3',
  });
  assert.equal(browserValidation.ok, true, `browser archive validation issues: ${browserValidation.issues.join('; ')}`);

  writeFixture(serverStage, 'bin/sdkwork-api-im-standalone-gateway.exe', 'server');
  writeFixture(serverStage, 'config/chat.toml.example', '[server]\nbind_address = "127.0.0.1:18079"\n');
  writeFixture(serverStage, 'config/server.env.example', 'SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND=127.0.0.1:18079\n');
  writeFixture(serverStage, 'config/postgresql.yaml.example', 'engine: postgresql');
  writeFixture(serverStage, 'INSTALL.md', '# install');
  writeFixture(serverStage, 'install-manifest.json', '{"product":"chat"}');
  writeFixture(serverStage, 'web/sdkwork-im-pc/dist/index.html', '<!doctype html>');
  writeFixture(serverStage, 'service/windows/sdkwork-api-im-standalone-gateway-service.xml', '<service />');
  const serverBuildPlan = packageBuilder.createSdkworkImInstallPackageBuildPlan({
    outputDir,
    packageId: 'windows-x64-standalone-server-zip',
    root: repoRoot,
    stagingRoot: serverStage,
    version: '1.2.3',
  });
  const serverArchive = await packageBuilder.buildSdkworkImInstallPackageArchive(serverBuildPlan);
  assert.equal(serverArchive.manifest.package.deploymentProfile, 'standalone');
  assert.equal(serverArchive.manifest.package.profile, 'server');
  assert.equal(serverArchive.manifest.package.runtimeTarget, 'server');
  assert.equal(serverArchive.manifest.package.format, 'zip');
  assert.equal(
    Object.hasOwn(serverArchive.manifest.package, 'deploymentMode'),
    false,
    'release package manifest must not keep retired deploymentMode',
  );
  assert.equal(
    Object.hasOwn(serverArchive.manifest.package, 'runtimeProfile'),
    false,
    'release package manifest must not keep retired runtimeProfile',
  );
  const serverValidation = validatorModule.validateSdkworkImInstallArtifact({
    artifactPath: serverArchive.archivePath,
    packageId: 'windows-x64-standalone-server-zip',
    root: repoRoot,
    version: '1.2.3',
  });
  assert.equal(serverValidation.ok, true, `server archive validation issues: ${serverValidation.issues.join('; ')}`);

  writeFixture(desktopStage, 'desktop/Sdkwork IM_1.2.3_x64-setup.exe', 'desktop');
  writeFixture(desktopStage, 'desktop-manifest.json', JSON.stringify({
    product: 'chat',
    version: '1.2.3',
    files: [{ path: 'desktop/Sdkwork IM_1.2.3_x64-setup.exe' }],
  }));
  const desktopBuildPlan = packageBuilder.createSdkworkImInstallPackageBuildPlan({
    outputDir,
    packageId: 'windows-x64-standalone-desktop-zip',
    root: repoRoot,
    stagingRoot: desktopStage,
    version: '1.2.3',
  });
  const desktopArchive = await packageBuilder.buildSdkworkImInstallPackageArchive(desktopBuildPlan);
  assert.equal(desktopArchive.manifest.package.deploymentProfile, 'standalone');
  assert.equal(desktopArchive.manifest.package.profile, 'desktop');
  assert.equal(desktopArchive.manifest.package.runtimeTarget, 'desktop');
  assert.equal(desktopArchive.manifest.package.format, 'zip');
  assert.equal(
    Object.hasOwn(desktopArchive.manifest.package, 'deploymentMode'),
    false,
    'desktop release package manifest must not keep retired deploymentMode',
  );
  assert.equal(
    Object.hasOwn(desktopArchive.manifest.package, 'runtimeProfile'),
    false,
    'desktop release package manifest must not keep retired runtimeProfile',
  );
  const desktopValidation = validatorModule.validateSdkworkImInstallArtifact({
    artifactPath: desktopArchive.archivePath,
    packageId: 'windows-x64-standalone-desktop-zip',
    root: repoRoot,
    version: '1.2.3',
  });
  assert.equal(desktopValidation.ok, true, `desktop archive validation issues: ${desktopValidation.issues.join('; ')}`);
} finally {
  rmSync(validatorTempRoot, { recursive: true, force: true });
}

const desktopCollectorTempRoot = mkTempDir('sdkwork-im-release-desktop-bundles-');
try {
  writeFixture(desktopCollectorTempRoot, 'nsis/Sdkwork IM_1.2.3_x64-setup.exe', 'x64 exe');
  writeFixture(desktopCollectorTempRoot, 'nsis/Sdkwork IM_1.2.3_arm64-setup.exe', 'arm64 exe');
  writeFixture(desktopCollectorTempRoot, 'msi/Sdkwork IM_1.2.3_x64_en-US.msi', 'x64 msi');
  writeFixture(desktopCollectorTempRoot, 'msi/Sdkwork IM_1.2.3_arm64_en-US.msi', 'arm64 msi');
  const x64DesktopBundles = desktopBundleCollector.collectSdkworkImDesktopBundles({
    arch: 'x64',
    bundleRoot: desktopCollectorTempRoot,
    platform: 'windows',
    root: repoRoot,
    version: '1.2.3',
  });
  assert.deepEqual(
    x64DesktopBundles.files.map((file) => file.path).sort(),
    [
      'msi/Sdkwork IM_1.2.3_x64_en-US.msi',
      'nsis/Sdkwork IM_1.2.3_x64-setup.exe',
    ],
    'desktop collector should exclude installer artifacts for the opposite architecture',
  );
  assert.deepEqual(
    desktopBundleCollector.validateSdkworkImDesktopBundleManifest(x64DesktopBundles),
    [],
    'desktop collector manifest with matching architecture files should validate',
  );
} finally {
  rmSync(desktopCollectorTempRoot, { recursive: true, force: true });
}

const stagingModule = await importRepoModule('scripts/release/stage-sdkwork-im-release-package.mjs');
const browserDryRunStagingPlan = stagingModule.createSdkworkImReleaseStagingPlan({
  packageId: 'web-universal-cloud-browser-zip',
  version: '1.2.3',
});
assert.equal(
  stagingModule.validateSdkworkImReleaseStagingPlan(browserDryRunStagingPlan, { requireSources: false }).length,
  0,
  'browser dry-run staging plan should be valid without source artifacts',
);
const browserInstallManifest = stagingModule.createInstallManifest(browserDryRunStagingPlan.package);
assert.equal(browserInstallManifest.package.deploymentProfile, 'cloud');
assert.equal(browserInstallManifest.package.profile, 'browser');
assert.equal(browserInstallManifest.package.runtimeTarget, 'browser');
assert.equal(browserInstallManifest.package.format, 'zip');
const browserStagingArchivePaths = new Set(browserDryRunStagingPlan.actions.map((action) => action.archivePath).filter(Boolean));
for (const expectedPath of [
  'web/sdkwork-im-pc/dist',
  'web/sdkwork-im-h5/dist',
  'web-manifest.json',
]) {
  assert.equal(
    browserStagingArchivePaths.has(expectedPath),
    true,
    `browser staging plan should expose archive path ${expectedPath}`,
  );
}

const dryRunStagingPlan = stagingModule.createSdkworkImReleaseStagingPlan({
  packageId: 'windows-x64-standalone-server-zip',
  version: '1.2.3',
});
assert.equal(
  stagingModule.validateSdkworkImReleaseStagingPlan(dryRunStagingPlan, { requireSources: false }).length,
  0,
  'dry-run staging plan should be valid without source artifacts',
);
const installManifest = stagingModule.createInstallManifest(dryRunStagingPlan.package);
assert.equal(installManifest.package.deploymentProfile, 'standalone');
assert.equal(installManifest.package.profile, 'server');
assert.equal(installManifest.package.runtimeTarget, 'server');
assert.equal(installManifest.package.format, 'zip');
assert.equal(
  Object.hasOwn(installManifest.package, 'deploymentMode'),
  false,
  'install manifest must not keep retired deploymentMode',
);
assert.equal(
  Object.hasOwn(installManifest.package, 'runtimeProfile'),
  false,
  'install manifest must not keep retired runtimeProfile',
);
const stagingArchivePaths = new Set(dryRunStagingPlan.actions.map((action) => action.archivePath).filter(Boolean));
for (const expectedPath of [
  'config/chat.toml.example',
  'config/postgresql.yaml.example',
  'service/linux/sdkwork-api-im-standalone-gateway.service',
  'service/macos/com.sdkwork.im.api-standalone-gateway.plist',
  'service/windows/sdkwork-api-im-standalone-gateway-service.xml',
  'web/sdkwork-im-pc/dist',
  'web/sdkwork-im-h5/dist',
]) {
  assert.equal(
    stagingArchivePaths.has(expectedPath),
    true,
    `staging plan should expose archive path ${expectedPath}`,
  );
}

const linuxStagingPlan = stagingModule.createSdkworkImReleaseStagingPlan({
  packageId: 'linux-x64-standalone-server-tar-gz',
  version: '1.2.3',
});
const linuxGeneratedEnvAction = linuxStagingPlan.actions.find((action) => action.label === 'server env template');
assert.equal(typeof linuxGeneratedEnvAction?.contentFactory, 'function', 'linux staging plan should generate server env template');
const linuxGeneratedEnv = linuxGeneratedEnvAction.contentFactory();
for (const expectedText of [
  'SDKWORK_IM_CONFIG_FILE=/etc/sdkwork/chat/chat.toml',
  'SDKWORK_IM_DATA_DIR=/var/lib/sdkwork/chat',
  'SDKWORK_IM_LOG_DIR=/var/log/sdkwork/chat',
  'SDKWORK_IM_RUN_DIR=/run/sdkwork/chat',
  'SDKWORK_IM_ID_NODE_ID=1',
  'SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=https://im.sdkwork.com',
  'SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=wss://im.sdkwork.com',
  'SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=https://api.sdkwork.com',
  'SDKWORK_IM_ADMIN_SITE_DIR=/opt/sdkwork/chat/web/sdkwork-im-pc/dist',
  'SDKWORK_IM_H5_SITE_DIR=/opt/sdkwork/chat/web/sdkwork-im-h5/dist',
]) {
  assert.match(linuxGeneratedEnv, new RegExp(expectedText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'));
}
assert.doesNotMatch(linuxGeneratedEnv, /\/etc\/sdkwork-im\/default|\/opt\/sdkwork-im/u);

const serverYamlTemplate = readText('deployments', 'templates', 'chat.toml.example');
for (const expectedText of [
  'config_file = "/etc/sdkwork/chat/chat.toml"',
  'base_url = "https://im.sdkwork.com"',
  'api_base_url = "https://im.sdkwork.com"',
  'websocket_base_url = "wss://im.sdkwork.com"',
  'docs_base_url = "https://im.sdkwork.com/docs"',
  'data_directory = "/var/lib/sdkwork/chat"',
  'log_directory = "/var/log/sdkwork/chat"',
  'runtime_directory = "/run/sdkwork/chat"',
  'password_file = "/etc/sdkwork/database/database.secret"',
]) {
  assert.match(serverYamlTemplate, new RegExp(expectedText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'));
}
assert.doesNotMatch(serverYamlTemplate, /\/etc\/sdkwork-im\/default|\/var\/run\/sdkwork-im\/default/u);

const serverEnvTemplate = readText('deployments', 'templates', 'server.env.example');
for (const expectedText of [
  'SDKWORK_IM_CONFIG_FILE=/etc/sdkwork/chat/chat.toml',
  'SDKWORK_IM_DATA_DIR=/var/lib/sdkwork/chat',
  'SDKWORK_IM_LOG_DIR=/var/log/sdkwork/chat',
  'SDKWORK_IM_RUN_DIR=/run/sdkwork/chat',
  'SDKWORK_IM_ID_NODE_ID=1',
]) {
  assert.match(serverEnvTemplate, new RegExp(expectedText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'));
}
assert.doesNotMatch(serverEnvTemplate, /\/etc\/sdkwork-im\/default|\/var\/run\/sdkwork-im\/default/u);

const postgresqlTemplate = readText('deployments', 'templates', 'postgresql.yaml.example');
assert.match(postgresqlTemplate, /passwordFile: \/etc\/sdkwork\/database\/database\.secret/u);
assert.doesNotMatch(postgresqlTemplate, /\/etc\/sdkwork-im\/default/u);

const systemdTemplate = readText('deployments', 'systemd', 'sdkwork-api-im-standalone-gateway.service');
for (const expectedText of [
  'WorkingDirectory=/opt/sdkwork/chat',
  'EnvironmentFile=/etc/sdkwork/chat/server.env',
  'ExecStart=/opt/sdkwork/chat/bin/sdkwork-api-im-standalone-gateway',
]) {
  assert.match(systemdTemplate, new RegExp(expectedText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'));
}
assert.doesNotMatch(systemdTemplate, /\/etc\/sdkwork-im|\/opt\/sdkwork-im/u);
assert.doesNotMatch(systemdTemplate, /--config/u);

for (const relativePath of [
  'bin/dev.ps1',
  'bin/dev.sh',
  'bin/build.ps1',
  'bin/build.sh',
  'bin/package.ps1',
  'bin/package.sh',
  'bin/start-prod.ps1',
  'bin/start-prod.sh',
]) {
  assertFile(relativePath);
  const scriptText = readText(...relativePath.split('/'));
  assert.doesNotMatch(scriptText, /Invoke-Expression|\biex\b|\beval\b/u, `${relativePath} should not use dynamic shell execution`);
}

const installServerPs1 = readText('bin', 'install-server.ps1');
assert.match(
  installServerPs1,
  /Resolve-ServerTemplatePath/u,
  'bin/install-server.ps1 should resolve templates from packaged config paths',
);
assert.match(
  installServerPs1,
  /config[\\/]chat\.toml\.example/u,
  'bin/install-server.ps1 should support server archive config/chat.toml.example',
);
assert.match(
  installServerPs1,
  /config[\\/]postgresql\.yaml\.example/u,
  'bin/install-server.ps1 should support server archive config/postgresql.yaml.example',
);
const installServerSh = readText('bin', 'install-server.sh');
assert.match(
  installServerSh,
  /resolve_template_path/u,
  'bin/install-server.sh should resolve templates from packaged config paths',
);
assert.match(
  installServerSh,
  /config\/chat\.toml\.example/u,
  'bin/install-server.sh should support server archive config/chat.toml.example',
);
assert.match(
  installServerSh,
  /config\/postgresql\.yaml\.example/u,
  'bin/install-server.sh should support server archive config/postgresql.yaml.example',
);

assert.equal(
  existsSync(repoPath('.github', 'workflows', 'release-package.yml')),
  false,
  'legacy copied release-package.yml must be removed after sdkwork-github-workflow integration',
);
assertFile('sdkwork.workflow.json');
assertFile('.github/workflows/package.yml');

const workflowConfig = JSON.parse(readText('sdkwork.workflow.json'));
assert.equal(workflowConfig.schemaVersion, '2026-06-06.sdkwork.workflow.v1');
assert.equal(workflowConfig.app?.id, 'sdkwork-im');
assert.equal(workflowConfig.app?.repository, 'Sdkwork-Cloud/sdkwork-im');
assert.equal(workflowConfig.release?.artifactPrefix, 'sdkwork-im');
assert.equal(workflowConfig.release?.defaultVersion, '0.1.0');
assert.equal(workflowConfig.release?.changelog?.source, 'auto');
assert.equal(
  workflowConfig.publish?.githubRelease,
  false,
  'DRAFT IM packages must not be published to GitHub Releases before real signing and store evidence is approved',
);
assert.equal(workflowConfig.publish?.workflowArtifact, true);
assert.equal(workflowConfig.security?.artifactAttestations, true);
assert.equal(workflowConfig.security?.sbomRequired, true, 'sdkwork.workflow.json must require SBOM when sdkwork.app.config.json does');
assert.equal(workflowConfig.security?.signingRequired, true, 'sdkwork.workflow.json must require signing when sdkwork.app.config.json does');
assert.ok(
  Array.isArray(workflowConfig.lifecycle?.sbom) && workflowConfig.lifecycle.sbom.length > 0,
  'sdkwork.workflow.json lifecycle.sbom must declare at least one step',
);
assert.ok(
  Array.isArray(workflowConfig.lifecycle?.sign) && workflowConfig.lifecycle.sign.length > 0,
  'sdkwork.workflow.json lifecycle.sign must declare at least one step',
);
assert.match(
  workflowConfig.lifecycle?.sbom?.map((step) => step.run).join('\n') ?? '',
  /workflow-supply-chain-evidence\.mjs attest/u,
  'sdkwork.workflow.json lifecycle.sbom must create byte-bound SBOM, provenance, and framework evidence',
);
assert.match(
  workflowConfig.lifecycle?.sign?.map((step) => step.run).join('\n') ?? '',
  /workflow-supply-chain-evidence\.mjs sign/u,
  'sdkwork.workflow.json lifecycle.sign must invoke the real detached-signature producer',
);
assert.doesNotMatch(
  workflowConfig.lifecycle?.sbom?.map((step) => step.run).join('\n') ?? '',
  /SBOM generation is not required/u,
  'sdkwork.workflow.json lifecycle.sbom must not contradict security.sbomRequired=true',
);

const appManifest = JSON.parse(readText('sdkwork.app.config.json'));
assert.equal(appManifest.security?.sbomRequired, true, 'sdkwork.app.config.json must require SBOM evidence');
assert.equal(appManifest.security?.signatureRequired, true, 'sdkwork.app.config.json must require release signatures');
assert.equal(appManifest.security?.checksumRequired, true, 'sdkwork.app.config.json must require checksum evidence');
assert.match(
  workflowConfig.lifecycle?.install?.map((step) => step.run).join('\n') ?? '',
  /SDKWORK_SHARED_SDK_GITHUB_TOKEN/u,
  'install lifecycle should preserve the legacy shared SDK GitHub token environment',
);
assert.match(
  workflowConfig.lifecycle?.install?.map((step) => step.run).join('\n') ?? '',
  /pnpm install --no-frozen-lockfile --config\.auto-install-peers=false/u,
  'install lifecycle must use the executable cross-workspace pnpm install mode for sibling SDKWork packages',
);
assert.doesNotMatch(
  workflowConfig.lifecycle?.install?.map((step) => step.run).join('\n') ?? '',
  /--frozen-lockfile/u,
  'install lifecycle must not use frozen lockfile while sibling workspace package manifests are the dependency authority',
);
assert.match(
  workflowConfig.lifecycle?.build?.map((step) => step.run).join('\n') ?? '',
  /SDKWORK_SHARED_SDK_GITHUB_TOKEN/u,
  'build lifecycle should preserve the legacy shared SDK GitHub token environment',
);
assert.doesNotMatch(
  workflowConfig.lifecycle?.preflight?.map((step) => step.run).join('\n') ?? '',
  /dotnet tool install --global wix/u,
  'preflight lifecycle should not duplicate framework WiX toolchain setup',
);

const expectedWorkflowTargetIds = [
  'web-universal-cloud-browser-zip',
  'linux-x64-standalone-server-tar-gz',
  'linux-arm64-standalone-server-tar-gz',
  'macos-x64-standalone-server-tar-gz',
  'macos-arm64-standalone-server-tar-gz',
  'windows-x64-standalone-server-zip',
  'windows-arm64-standalone-server-zip',
  'linux-x64-standalone-desktop-zip',
  'linux-arm64-standalone-desktop-zip',
  'macos-x64-standalone-desktop-zip',
  'macos-arm64-standalone-desktop-zip',
  'windows-x64-standalone-desktop-zip',
  'windows-arm64-standalone-desktop-zip',
  'h5-universal-cloud-mobile-zip',
  'android-universal-cloud-mobile-apk',
  'android-universal-cloud-mobile-aab',
  'ios-universal-cloud-mobile-ipa',
  'container-x64-cloud-container-kubernetes-tar-gz',
];
const sortedExpectedWorkflowTargetIds = [...expectedWorkflowTargetIds].sort();
assert.deepEqual(
  workflowConfig.targets?.map((target) => target.id).sort(),
  sortedExpectedWorkflowTargetIds,
  'sdkwork.workflow.json should expose canonical package ids for every implemented root release target',
);
assert.deepEqual(
  appManifest.artifacts?.installConfig?.packages?.map((releasePackage) => releasePackage.id).sort(),
  sortedExpectedWorkflowTargetIds,
  'sdkwork.app.config.json install packages should match sdkwork.workflow.json targets exactly',
);
assert.deepEqual(
  appManifest.release?.notes?.find((note) => note.current === true)?.packageIds.sort(),
  sortedExpectedWorkflowTargetIds,
  'sdkwork.app.config.json current release note packageIds should match sdkwork.workflow.json targets exactly',
);
const workflowTargetsById = new Map((workflowConfig.targets ?? []).map((target) => [target.id, target]));
for (const releasePackage of appManifest.artifacts?.installConfig?.packages ?? []) {
  const workflowTarget = workflowTargetsById.get(releasePackage.id);
  assert.ok(workflowTarget, `${releasePackage.id} should have a workflow target`);
  assert.equal(releasePackage.deploymentProfile, workflowTarget.deploymentProfile, `${releasePackage.id} deploymentProfile`);
  assert.equal(releasePackage.runtimeTarget, workflowTarget.runtimeTarget, `${releasePackage.id} runtimeTarget`);
  assert.equal(releasePackage.architecture, workflowTarget.architecture, `${releasePackage.id} architecture`);
  if (releasePackage.clientArchitecture) {
    assert.equal(releasePackage.targetPlatform, workflowTarget.targetPlatform, `${releasePackage.id} targetPlatform`);
    assert.equal(releasePackage.clientArchitecture, workflowTarget.clientArchitecture, `${releasePackage.id} clientArchitecture`);
  }
  assert.equal(
    releasePackage.packageFormat,
    packageFormatForManifest(workflowTarget.formats?.[0]),
    `${releasePackage.id} packageFormat`,
  );
}
const workflowConfigText = JSON.stringify(workflowConfig);
for (const forbiddenText of [
  'Resolve-SdkworkImLegacyPackageId',
  'legacyPackageId',
  'server-archive',
  'windows-x64-desktop',
  'linux-x64-server-archive',
  'deploymentMode',
  'runtimeProfile',
]) {
  assert.doesNotMatch(
    workflowConfigText,
    new RegExp(forbiddenText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'),
    `sdkwork.workflow.json must not retain release package legacy marker: ${forbiddenText}`,
  );
}
for (const [phase, expectedCommand] of Object.entries({
  stage: 'pnpm release:stage -- --package-id $env:SDKWORK_PACKAGE_ID',
  package: 'node scripts/release/build-sdkwork-im-install-package.mjs --package-id $env:SDKWORK_PACKAGE_ID',
  validate: 'node scripts/release/validate-sdkwork-im-install-artifacts.mjs --package-id $env:SDKWORK_PACKAGE_ID',
})) {
  const phaseScript = workflowConfig.lifecycle?.[phase]?.map((step) => step.run).join('\n') ?? '';
  assert.match(
    phaseScript,
    new RegExp(expectedCommand.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'),
    `sdkwork.workflow.json lifecycle.${phase} must consume canonical SDKWORK_PACKAGE_ID directly`,
  );
}
for (const target of workflowConfig.targets ?? []) {
  const variantToken = target.variant ? `-${target.variant}` : '';
  assert.equal(
    target.id,
    `${target.platform}-${target.architecture}-${target.deploymentProfile}-${target.profile}${variantToken}-${String(target.formats?.[0] ?? '').replaceAll('.', '-')}`,
  );
  assert.equal(target.outputGlobs?.includes(target.artifactPath), true, `${target.id} should upload its primary artifact`);
  assert.equal(
    target.outputGlobs?.includes(`dist/release-evidence/${target.id}/*`),
    true,
    `${target.id} should upload its target-scoped release evidence`,
  );
}

const packageWorkflowText = readText('.github', 'workflows', 'package.yml');
assert.match(
  packageWorkflowText,
  /uses: Sdkwork-Cloud\/sdkwork-github-workflow\/\.github\/workflows\/sdkwork-package\.yml@b0829529b9277a3da32b90c2d36ff34ff09fa832/u,
  'package workflow should call the pinned sdkwork-github-workflow reusable workflow',
);
for (const expectedText of [
  'workflow_dispatch:',
  'push:',
  'release:',
  'config_path: sdkwork.workflow.json',
  "package_version: ${{ github.event.inputs.package_version || '' }}",
  'publish_release: false',
  'upload_artifact: true',
  'framework_ref: b0829529b9277a3da32b90c2d36ff34ff09fa832',
]) {
  assert.match(
    packageWorkflowText,
    new RegExp(expectedText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'),
    `package workflow should include ${expectedText}`,
  );
}
for (const forbiddenText of [
  'Plan package matrix',
  'fromJson(needs.plan.outputs.matrix)',
  'pnpm release:build:prod',
  'actions/upload-artifact',
  'actions/download-artifact',
  'gh release',
  'sha256sum "$file"',
]) {
  assert.doesNotMatch(
    packageWorkflowText,
    new RegExp(forbiddenText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'),
    `package workflow must not copy framework logic: ${forbiddenText}`,
  );
}

console.log('[sdkwork-im-release-package] contract passed');

function mkTempDir(prefix) {
  return mkdtempSafe(path.join(os.tmpdir(), prefix));
}

function mkdtempSafe(prefix) {
  const tempRoot = `${prefix}${process.pid}-${Date.now()}`;
  mkdirSync(tempRoot, { recursive: true });
  return tempRoot;
}

function writeFixture(root, relativePath, content) {
  const absolutePath = path.join(root, ...relativePath.split('/'));
  mkdirSync(path.dirname(absolutePath), { recursive: true });
  writeFileSync(absolutePath, content);
}

function packageFormatForManifest(format) {
  if (format === 'tar.gz') {
    return 'TAR_GZ';
  }
  return String(format ?? '').replaceAll('.', '_').replaceAll('-', '_').toUpperCase();
}
