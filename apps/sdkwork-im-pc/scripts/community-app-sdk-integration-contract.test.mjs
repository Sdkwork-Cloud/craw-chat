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

const packageJson = readJson('package.json');
const releaseSources = readRepoJson('config', 'shared-sdk-release-sources.json');
const workflow = readRepoJson('sdkwork.workflow.json');
const gatewayConfigSource = readRepoText('crates', 'sdkwork-im-cloud-gateway-config', 'src', 'lib.rs');
const gatewaySource = readRepoText('services', 'sdkwork-im-cloud-gateway', 'src', 'lib.rs');
const sharedSdkGitSource = readRepoText('scripts', 'dev', 'prepare-shared-sdk-git-sources.mjs');
const releaseBuildSource = readRepoText('scripts', 'release', 'run-sdkwork-im-pc-release-build.mjs');
const devRunnerSource = readRepoText('scripts', 'lib', 'im-pc-dev.mjs');
const componentSpec = readRepoJson('specs', 'component.spec.json');
const moduleRegistrySource = readText('packages', 'sdkwork-im-pc-shell', 'src', 'moduleRegistry.ts');
const communityServiceSource = readText('packages', 'sdkwork-im-pc-community', 'src', 'services', 'CommunityService.ts');
const communityClientSource = readText('packages', 'sdkwork-im-pc-core', 'src', 'sdk', 'communityAppSdkClient.ts');
const viteConfigSource = readText('vite.config.ts');
const tsconfig = readJson('tsconfig.json');

assert.equal(
  packageJson.scripts?.['test:community-app-sdk-integration'],
  'node scripts/community-app-sdk-integration-contract.test.mjs',
  'Chat PC must expose a dedicated community app SDK integration contract script.',
);

assert.equal(
  readJson('packages', 'sdkwork-im-pc-core', 'package.json').dependencies?.['@sdkwork/community-app-sdk'],
  'workspace:*',
  'Chat PC must consume sdkwork-community through the workspace app SDK package.',
);

assert.match(
  releaseSources.sources?.['sdkwork-community']?.repoUrl ?? '',
  /^(?:https:\/\/github\.com\/|git@github\.com:)Sdkwork-Cloud\/sdkwork-community\.git$/u,
  'Shared SDK release config must materialize sdkwork-community from the canonical git repository.',
);

assert.ok(
  typeof releaseSources.sources?.['sdkwork-community']?.ref === 'string'
    && releaseSources.sources['sdkwork-community'].ref.trim().length > 0,
  'Shared SDK release config must pin a non-empty sdkwork-community git ref.',
);

assert.equal(
  releaseSources.sources?.['sdkwork-community']?.ref,
  workflow.dependencies?.find((dependency) => dependency.id === 'sdkwork-community')?.ref,
  'Shared SDK release config must use the same pinned sdkwork-community ref as sdkwork.workflow.json.',
);

assert.match(
  sharedSdkGitSource,
  /id:\s*['"]sdkwork-community['"][\s\S]*sdkwork-community-app-sdk[\\/]sdkwork-community-app-sdk-typescript[\\/]generated[\\/]server-openapi[\\/]package\.json/u,
  'Shared SDK git materializer must know how to prepare the sdkwork-community app SDK source.',
);

assert.match(
  sharedSdkGitSource,
  /SDKWORK_SHARED_COMMUNITY_REPO_URL[\s\S]*SDKWORK_SHARED_COMMUNITY_GIT_REF/u,
  'Shared SDK git materializer must expose sdkwork-community repo/ref override environment variables.',
);

assert.match(
  releaseBuildSource,
  /SDKWORK_SHARED_COMMUNITY_GIT_REF[\s\S]*SDKWORK_COMMUNITY_REF/u,
  'Release build plan must bridge SDKWORK_COMMUNITY_REF into the shared SDK materializer ref for the community app SDK.',
);

assert.match(
  devRunnerSource,
  /SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL[\s\S]*explicitCommunityAppApiUpstream[\s\S]*SDKWORK_IM_COMMUNITY_APP_API_UPSTREAM/u,
  'PC dev runner must default sdkwork-community traffic through the shared gateway root while preserving explicit Community split upstream overrides.',
);

assert.match(
  gatewayConfigSource,
  /sdkwork-community-app-api[\s\S]*SDKWORK_IM_COMMUNITY_APP_API_UPSTREAM[\s\S]*SDKWORK_COMMUNITY_APP_API_UPSTREAM[\s\S]*SDKWORK_COMMUNITY_APP_API_BASE_URL/u,
  'Gateway config must expose deterministic sdkwork-community app-api upstream environment keys.',
);

assert.match(
  gatewaySource,
  /"sdkwork-community-app-api"[\s\S]*\/app\/v3\/api\/community\/\{\*path\}[\s\S]*SdkworkCommunityAppSdk/u,
  'Web gateway must route sdkwork-community app-api paths to the Community app SDK upstream.',
);

assert.ok(
  workflow.dependencies?.some((dependency) => (
    dependency.id === 'sdkwork-community'
      && dependency.repository === 'Sdkwork-Cloud/sdkwork-community'
      && dependency.refInput === 'SDKWORK_COMMUNITY_REF'
      && dependency.tokenSecret === 'SDKWORK_RELEASE_TOKEN'
  )),
  'sdkwork.workflow.json must declare sdkwork-community as a release dependency.',
);

const dependencySurface = componentSpec.contracts?.dependencyApiSurfaces?.find(
  (surface) => surface.apiAuthority === 'sdkwork-community-app-api',
);
assert.ok(dependencySurface, 'component.spec.json must declare sdkwork-community-app-api dependency surface.');
assert.equal(
  dependencySurface.sdkFamily,
  'sdkwork-community-app-sdk',
  'component.spec.json must bind sdkwork-community-app-api to sdkwork-community-app-sdk.',
);
assert.equal(
  dependencySurface.targetRuntimeIntegration?.gatewayApplication,
  'sdkwork-api-cloud-gateway',
  'sdkwork-community app API must route through the shared sdkwork-api-cloud-gateway root.',
);

const splitOverrideEnvKeys =
  componentSpec.integration?.foundationApiGateway?.splitOverrideEnvKeys?.['sdkwork-community-app-api'];
assert.deepEqual(
  splitOverrideEnvKeys,
  [
    'SDKWORK_IM_COMMUNITY_APP_API_UPSTREAM',
    'SDKWORK_COMMUNITY_APP_API_UPSTREAM',
    'SDKWORK_COMMUNITY_APP_API_BASE_URL',
  ],
  'component.spec.json must document community split upstream override env keys.',
);

assert.match(
  moduleRegistrySource,
  /COMMERCIAL_RUNTIME_MODULES[\s\S]*"community"/u,
  'Community must be enabled in commercial runtime modules after SDK wiring.',
);

assert.match(
  communityServiceSource,
  /getCommunityAppSdkClientWithSession/u,
  'CommunityService must consume the generated community app SDK client instead of fail-closed stubs.',
);
assert.doesNotMatch(
  communityServiceSource,
  /pc community contract is not available/u,
  'CommunityService must not keep the legacy contract-unavailable fail-closed stub.',
);

assert.match(
  communityClientSource,
  /@sdkwork\/community-app-sdk/u,
  'Community app SDK client wrapper must import the composed community app SDK package.',
);

assert.match(
  viteConfigSource,
  /@sdkwork\/community-app-sdk/u,
  'Vite config must alias @sdkwork/community-app-sdk for PC community integration.',
);

assert.ok(
  tsconfig.compilerOptions?.paths?.['@sdkwork/community-app-sdk'],
  'tsconfig must map @sdkwork/community-app-sdk for PC community integration.',
);

console.log('community app SDK integration contract checks passed');
