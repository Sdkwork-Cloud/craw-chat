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
  'release:plan': 'node scripts/release/plan-sdkwork-im-install-packages.mjs',
  'release:build:prod': 'node scripts/release/build-sdkwork-im-production.mjs',
  'release:stage': 'node scripts/release/stage-sdkwork-im-release-package.mjs',
  'release:package': 'node scripts/release/build-sdkwork-im-install-package.mjs',
  'release:package:check': 'node scripts/release/build-sdkwork-im-install-package.mjs --check --dry-run --all',
  'release:validate': 'node scripts/release/validate-sdkwork-im-install-artifacts.mjs',
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
  'SUPPORTED_DEPLOYMENT_MODES',
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
assert.deepEqual(releasePlan.deploymentModes, ['server-archive', 'desktop']);

const serverPackagesByPlatform = new Map(
  releasePlan.packages
    .filter((item) => item.deploymentMode === 'server-archive' && item.architecture === 'x64')
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
  '/etc/sdkwork/chat/database.secret',
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
  'linux-x64-server-archive',
  'linux-arm64-server-archive',
  'macos-x64-server-archive',
  'macos-arm64-server-archive',
  'windows-x64-server-archive',
  'windows-arm64-server-archive',
  'linux-x64-desktop',
  'linux-arm64-desktop',
  'macos-x64-desktop',
  'macos-arm64-desktop',
  'windows-x64-desktop',
  'windows-arm64-desktop',
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
    '--deployment-mode',
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
  ['windows-x64-desktop'],
  'filtered release plan should include only the requested package id',
);

for (const packageItem of releasePlan.packages) {
  assert.equal(packageItem.security?.noSecretsInPackage, true, `${packageItem.id} no secrets policy`);
  assert.equal(packageItem.version, '1.2.3', `${packageItem.id} version`);
  if (packageItem.deploymentMode === 'server-archive') {
    assert.equal(packageItem.runtimeProfile, 'server', `${packageItem.id} runtime profile`);
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
    assert.match(packageItem.archiveName, /^sdkwork-im-server-.+\.(zip|tar\.gz)$/u);
  } else {
    assert.equal(packageItem.runtimeProfile, 'desktop', `${packageItem.id} runtime profile`);
    assert.equal(packageItem.databasePolicy?.defaultEngine, 'sqlite', `${packageItem.id} database engine`);
    assert.equal(packageItem.databasePolicy?.requiresExternalDatabase, false, `${packageItem.id} external database`);
    for (const expectedArtifact of ['desktop-installers', 'desktop-manifest']) {
      assert.equal(
        packageItem.artifacts.some((artifact) => artifact.kind === expectedArtifact && artifact.required === true),
        true,
        `${packageItem.id} should include ${expectedArtifact}`,
      );
    }
    assert.match(packageItem.archiveName, /^sdkwork-im-desktop-.+\.zip$/u);
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

const dryRunBuildPlan = packageBuilder.createSdkworkImInstallPackageBuildPlan({
  packageId: 'windows-x64-server-archive',
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
  const serverStage = path.join(validatorTempRoot, 'stage', 'windows-x64-server-archive');
  const desktopStage = path.join(validatorTempRoot, 'stage', 'windows-x64-desktop');
  const outputDir = path.join(validatorTempRoot, 'out');
  writeFixture(serverStage, 'bin/sdkwork-im-server.exe', 'server');
  writeFixture(serverStage, 'config/chat.toml.example', '[server]\nbind_address = "127.0.0.1:18080"\n');
  writeFixture(serverStage, 'config/server.env.example', 'SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND=127.0.0.1:18080\n');
  writeFixture(serverStage, 'config/postgresql.yaml.example', 'engine: postgresql');
  writeFixture(serverStage, 'INSTALL.md', '# install');
  writeFixture(serverStage, 'install-manifest.json', '{"product":"chat"}');
  writeFixture(serverStage, 'web/sdkwork-im-pc/dist/index.html', '<!doctype html>');
  writeFixture(serverStage, 'service/windows/SdkworkImServer.xml', '<service />');
  const serverBuildPlan = packageBuilder.createSdkworkImInstallPackageBuildPlan({
    outputDir,
    packageId: 'windows-x64-server-archive',
    root: repoRoot,
    stagingRoot: serverStage,
    version: '1.2.3',
  });
  const serverArchive = await packageBuilder.buildSdkworkImInstallPackageArchive(serverBuildPlan);
  const serverValidation = validatorModule.validateSdkworkImInstallArtifact({
    artifactPath: serverArchive.archivePath,
    packageId: 'windows-x64-server-archive',
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
    packageId: 'windows-x64-desktop',
    root: repoRoot,
    stagingRoot: desktopStage,
    version: '1.2.3',
  });
  const desktopArchive = await packageBuilder.buildSdkworkImInstallPackageArchive(desktopBuildPlan);
  const desktopValidation = validatorModule.validateSdkworkImInstallArtifact({
    artifactPath: desktopArchive.archivePath,
    packageId: 'windows-x64-desktop',
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
const dryRunStagingPlan = stagingModule.createSdkworkImReleaseStagingPlan({
  packageId: 'windows-x64-server-archive',
  version: '1.2.3',
});
assert.equal(
  stagingModule.validateSdkworkImReleaseStagingPlan(dryRunStagingPlan, { requireSources: false }).length,
  0,
  'dry-run staging plan should be valid without source artifacts',
);
const stagingArchivePaths = new Set(dryRunStagingPlan.actions.map((action) => action.archivePath).filter(Boolean));
for (const expectedPath of [
  'config/chat.toml.example',
  'config/postgresql.yaml.example',
  'service/linux/sdkwork-im-server.service',
  'service/macos/com.sdkwork.im.server.plist',
  'service/windows/SdkworkImServer.xml',
  'web/sdkwork-im-pc/dist',
]) {
  assert.equal(
    stagingArchivePaths.has(expectedPath),
    true,
    `staging plan should expose archive path ${expectedPath}`,
  );
}

const linuxStagingPlan = stagingModule.createSdkworkImReleaseStagingPlan({
  packageId: 'linux-x64-server-archive',
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
  'password_file = "/etc/sdkwork/chat/database.secret"',
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
assert.match(postgresqlTemplate, /passwordFile: \/etc\/sdkwork\/chat\/database\.secret/u);
assert.doesNotMatch(postgresqlTemplate, /\/etc\/sdkwork-im\/default/u);

const systemdTemplate = readText('deployments', 'systemd', 'sdkwork-im-server.service');
for (const expectedText of [
  'WorkingDirectory=/opt/sdkwork/chat',
  'EnvironmentFile=/etc/sdkwork/chat/server.env',
  'ExecStart=/opt/sdkwork/chat/bin/sdkwork-im-server --config /etc/sdkwork/chat/chat.toml',
]) {
  assert.match(systemdTemplate, new RegExp(expectedText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'));
}
assert.doesNotMatch(systemdTemplate, /\/etc\/sdkwork-im|\/opt\/sdkwork-im/u);

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
assert.equal(workflowConfig.publish?.githubRelease, true);
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
  /SBOM generation is configured by release environment/u,
  'sdkwork.workflow.json lifecycle.sbom must delegate SBOM generation to the release environment when sbomRequired is true',
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
assert.deepEqual(
  workflowConfig.targets?.map((target) => target.id),
  expectedWorkflowTargetIds,
  'sdkwork.workflow.json should expose canonical package ids for the supported server archives and desktop bundles',
);
for (const target of workflowConfig.targets ?? []) {
  assert.equal(
    target.id,
    `${target.platform}-${target.architecture}-${target.deploymentProfile}-${target.profile}-${String(target.formats?.[0] ?? '').replaceAll('.', '-')}`,
  );
  assert.equal(target.outputGlobs?.includes('dist/release-packages/*'), true, `${target.id} should upload release packages`);
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
  'publish_release: true',
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
