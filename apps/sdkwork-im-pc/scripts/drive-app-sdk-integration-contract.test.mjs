#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';

const appRoot = path.resolve(import.meta.dirname, '..');
const repoRoot = path.resolve(appRoot, '..', '..');

function readText(...segments) {
  return fs.readFileSync(path.join(appRoot, ...segments), 'utf8');
}

function readJson(...segments) {
  return JSON.parse(readText(...segments));
}

function readRepoText(...segments) {
  return fs.readFileSync(path.join(repoRoot, ...segments), 'utf8');
}

function readRepoJson(...segments) {
  return JSON.parse(readRepoText(...segments));
}

function functionBody(source, functionName) {
  const match = new RegExp(`function\\s+${functionName}\\s*\\(`, 'u').exec(source);
  assert.ok(match, `Expected ${functionName} in source.`);

  const openBraceIndex = source.indexOf('{', match.index);
  assert.notEqual(openBraceIndex, -1, `Expected ${functionName} body.`);

  let depth = 0;
  for (let index = openBraceIndex; index < source.length; index += 1) {
    const character = source[index];
    if (character === '{') {
      depth += 1;
    } else if (character === '}') {
      depth -= 1;
      if (depth === 0) {
        return source.slice(openBraceIndex, index + 1);
      }
    }
  }

  throw new Error(`Could not find closing brace for ${functionName}.`);
}

const packageJson = readJson('package.json');
const corePackageJson = readJson('packages', 'sdkwork-im-pc-core', 'package.json');
const shellPackageJson = readJson('packages', 'sdkwork-im-pc-shell', 'package.json');
const tsconfig = readJson('tsconfig.json');
const tsconfigApp = readJson('tsconfig.app.json');
const pnpmWorkspaceSource = readRepoText('pnpm-workspace.yaml');
const viteConfigSource = readText('vite.config.ts');
const releaseSources = readRepoJson('config', 'shared-sdk-release-sources.json');
const sharedSdkGitSource = readRepoText('scripts', 'dev', 'prepare-shared-sdk-git-sources.mjs');
const devRunnerSource = readRepoText('scripts', 'lib', 'im-pc-dev.mjs');
const gatewayConfigSource = readRepoText('crates', 'sdkwork-im-cloud-gateway-config', 'src', 'lib.rs');
const gatewaySource = readRepoText('services', 'sdkwork-im-cloud-gateway', 'src', 'lib.rs');
const workflow = readRepoJson('sdkwork.workflow.json');
const componentSpec = readRepoJson('specs', 'component.spec.json');
const appAuthRuntimeSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'appAuthRuntime.ts',
);
const driveClientSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'driveAppSdkClient.ts',
);
const drivePcIntegrationSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'drivePcIntegration.ts',
);
const driveBootstrapSource = readText('src', 'bootstrap', 'drivePc.ts');
const shellLoadersSource = readText(
  'packages',
  'sdkwork-im-pc-shell',
  'src',
  'capabilityModuleLoaders.ts',
);
const drivePcRoot = path.resolve(repoRoot, '..', 'sdkwork-drive', 'apps', 'sdkwork-drive-pc');
const driveViewSource = fs.readFileSync(
  path.join(drivePcRoot, 'packages', 'sdkwork-drive-pc-drive', 'src', 'DriveView.tsx'),
  'utf8',
);

assert.equal(
  packageJson.scripts?.['test:drive-app-sdk-integration'],
  'node scripts/drive-app-sdk-integration-contract.test.mjs',
  'Chat PC must expose a dedicated drive app SDK integration contract script.',
);

assert.equal(
  corePackageJson.dependencies?.['@sdkwork/drive-app-sdk'],
  'workspace:*',
  '@sdkwork/im-pc-core must consume sdkwork-drive through the workspace app SDK package.',
);

assert.equal(
  shellPackageJson.dependencies?.['@sdkwork/drive-pc-drive'],
  'workspace:*',
  '@sdkwork/im-pc-shell must consume the sdkwork-drive-pc-drive embed package through workspace:*.',
);

assert.equal(
  readRepoJson('package.json').pnpm?.overrides?.['@sdkwork/drive-app-sdk'],
  'workspace:*',
  'Repository root pnpm overrides must keep sdkwork-drive app SDK on workspace:*.',
);

assert.match(
  pnpmWorkspaceSource,
  /sdkwork-drive-pc-drive/u,
  'pnpm-workspace.yaml must include the sdkwork-drive-pc-drive package.',
);

assert.equal(
  fs.existsSync(path.join(appRoot, 'packages', 'sdkwork-im-pc-drive')),
  false,
  'apps/sdkwork-im-pc must not keep a local sdkwork-im-pc-drive package.',
);

assert.match(
  releaseSources.sources?.['sdkwork-drive']?.repoUrl ?? '',
  /^(?:https:\/\/github\.com\/|git@github\.com:)Sdkwork-Cloud\/sdkwork-drive\.git$/u,
  'Shared SDK release config must materialize sdkwork-drive from the canonical git repository.',
);

assert.equal(
  releaseSources.sources?.['sdkwork-drive']?.ref,
  workflow.dependencies?.find((dependency) => dependency.id === 'sdkwork-drive')?.ref,
  'Shared SDK release config must use the same pinned sdkwork-drive ref as sdkwork.workflow.json.',
);

assert.match(
  sharedSdkGitSource,
  /id:\s*['"]sdkwork-drive['"][\s\S]*sdkwork-drive-pc-drive[\\/]package\.json/u,
  'Shared SDK git materializer must know how to prepare the sdkwork-drive-pc-drive embed package.',
);

assert.match(
  devRunnerSource,
  /SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL[\s\S]*explicitDriveAppApiUpstream[\s\S]*SDKWORK_IM_DRIVE_APP_API_UPSTREAM/u,
  'PC dev runner must default sdkwork-drive traffic through the shared gateway root while preserving explicit Drive split upstream overrides.',
);

assert.match(
  gatewayConfigSource,
  /sdkwork-drive-app-api[\s\S]*SDKWORK_IM_DRIVE_APP_API_UPSTREAM[\s\S]*SDKWORK_DRIVE_APP_API_UPSTREAM[\s\S]*SDKWORK_DRIVE_APP_API_BASE_URL/u,
  'Gateway config must expose deterministic sdkwork-drive app-api upstream environment keys.',
);

assert.match(
  gatewaySource,
  /sdkwork-drive-app-api[\s\S]*\/app\/v3\/api\/drive\/\{\*path\}[\s\S]*SdkworkDriveAppSdk/u,
  'Web gateway must route sdkwork-drive app-api paths to the Drive app SDK upstream.',
);

const dependencySurface = componentSpec.contracts?.dependencyApiSurfaces?.find(
  (surface) => surface.apiAuthority === 'sdkwork-drive-app-api',
);
assert.ok(dependencySurface, 'component.spec.json must declare sdkwork-drive-app-api dependency surface.');
assert.equal(
  dependencySurface.sdkFamily,
  'sdkwork-drive-app-sdk',
  'component.spec.json must bind sdkwork-drive-app-api to sdkwork-drive-app-sdk.',
);

assert.match(
  driveClientSource,
  /createDriveAppClient/u,
  'Core drive client must use the sdkwork-drive generated app SDK factory.',
);

assert.match(
  driveClientSource,
  /tokenManager:\s*getSdkworkChatGlobalTokenManager\(\)/u,
  'Core drive client must share the Sdkwork IM global token manager.',
);

assert.doesNotMatch(
  driveClientSource,
  /fetch\(|axios|Authorization|Access-Token/u,
  'Core drive client must not assemble raw HTTP or auth headers.',
);

assert.match(
  drivePcIntegrationSource,
  /bootstrapDrivePcForIm/u,
  'IM core must expose drive PC integration bootstrap.',
);

assert.match(
  drivePcIntegrationSource,
  /configureDrivePcRuntime/u,
  'IM core must configure sdkwork-drive-pc runtime through the integration module.',
);

assert.match(
  drivePcIntegrationSource,
  /rebootstrapDrivePcRuntimeForIm/u,
  'IM core must re-bootstrap drive PC runtime after session changes.',
);

assert.match(
  drivePcIntegrationSource,
  /subscribeHostLanguage/u,
  'IM core must bridge host language changes into sdkwork-drive-pc runtime.',
);

assert.match(
  functionBody(appAuthRuntimeSource, 'createSdkworkChatIamRuntime'),
  /rebootstrapDrivePcRuntimeForIm\(\)/u,
  'IM auth runtime must re-bootstrap drive PC runtime after session changes.',
);

assert.match(
  driveBootstrapSource,
  /bootstrapDrivePcForIm/u,
  'IM app bootstrap must wire sdkwork-drive-pc host adapters.',
);

assert.match(
  shellLoadersSource,
  /import\('@sdkwork\/drive-pc-drive'\)/u,
  'IM shell must lazy-load the sdkwork-drive-pc-drive capability package.',
);

assert.match(
  driveViewSource,
  /createHostManagedDriveRuntime/u,
  'Drive embed package must use host-managed runtime without standalone auth.',
);

assert.match(
  driveViewSource,
  /driveSurface\.css/u,
  'Drive embed package must import the drive application stylesheet for host-managed UI fidelity.',
);

const driveSurfaceCssSource = fs.readFileSync(
  path.join(drivePcRoot, 'packages', 'sdkwork-drive-pc-drive', 'src', 'driveSurface.css'),
  'utf8',
);
assert.match(
  driveSurfaceCssSource,
  /driveWorkspaceChrome\.css/u,
  'Drive embed stylesheet must include workspace chrome layout rules for host embedding.',
);
assert.doesNotMatch(
  driveSurfaceCssSource,
  /@import\s+["']tailwindcss["']\s*;/u,
  'Drive embed stylesheet must not bootstrap Tailwind; the host shell index.css owns the single bootstrap.',
);
assert.doesNotMatch(
  viteConfigSource,
  /find:\s*['"]tailwindcss['"]/u,
  'Host Vite config must not alias bare specifier tailwindcss once shell bootstrap owns Tailwind resolution.',
);

assert.match(
  driveViewSource,
  /<DrivePcPreferencesProvider/u,
  'Drive embed package must provide DrivePcPreferencesProvider for host-managed embedding.',
);

assert.match(
  driveViewSource,
  /subscribeHostLanguage/u,
  'Drive embed package must subscribe to host-managed language changes.',
);

assert.match(
  viteConfigSource,
  /@sdkwork\/drive-pc-drive/u,
  'Vite must alias @sdkwork/drive-pc-drive for host-managed drive embedding.',
);

assert.ok(
  tsconfig.compilerOptions?.paths?.['@sdkwork/drive-pc-drive'],
  'tsconfig must map @sdkwork/drive-pc-drive for host-managed drive embedding.',
);

assert.ok(
  tsconfigApp.compilerOptions?.paths?.['@sdkwork/utils'],
  'tsconfig.app must map @sdkwork/utils so drive SDK composed clients typecheck.',
);

assert.match(
  viteConfigSource,
  /@sdkwork\/utils/u,
  'Vite must alias @sdkwork/utils for shared utility standardization.',
);

console.log('sdkwork im drive app SDK integration contract passed.');
