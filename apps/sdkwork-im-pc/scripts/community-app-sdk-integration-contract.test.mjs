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
const sharedSdkGitSource = readRepoText('scripts', 'dev', 'prepare-shared-sdk-git-sources.mjs');
const releaseBuildSource = readRepoText('scripts', 'release', 'run-sdkwork-im-pc-release-build.mjs');
const devRunnerSource = readRepoText('scripts', 'lib', 'im-pc-dev.mjs');
const componentSpec = readRepoJson('specs', 'component.spec.json');
const moduleRegistrySource = readText('packages', 'sdkwork-im-pc-shell', 'src', 'moduleRegistry.ts');
const imCommunityAdapterSource = readText('packages', 'sdkwork-im-pc-community', 'src', 'createImCommunityPcHostAdapter.tsx');
const communityServiceSource = readRepoText(
  '..',
  'sdkwork-community',
  'apps',
  'sdkwork-community-pc',
  'packages',
  'sdkwork-community-pc-community',
  'src',
  'services',
  'CommunityService.ts',
);
const communityClientSource = readText('packages', 'sdkwork-im-pc-core', 'src', 'sdk', 'communityPcIntegration.ts');
const communityClientReexportSource = readText('packages', 'sdkwork-im-pc-core', 'src', 'sdk', 'communityAppSdkClient.ts');
const communityBootstrapSource = readText('src', 'bootstrap', 'communityPc.ts');
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
  'Chat PC core must consume sdkwork-community through the composed @sdkwork/community-app-sdk facade.',
);

assert.match(
  releaseSources.sources?.['sdkwork-community']?.repoUrl ?? '',
  /^(?:https:\/\/github\.com\/|git@github\.com:)sdkwork-ai\/sdkwork-community\.git$/u,
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

assert.doesNotMatch(
  devRunnerSource,
  /explicitCommunityAppApiUpstream|SDKWORK_IM_COMMUNITY_APP_API_UPSTREAM|SDKWORK_COMMUNITY_APP_API_UPSTREAM|SDKWORK_COMMUNITY_APP_API_BASE_URL/u,
  'Community foundation traffic must use the platform assembly gateway without per-module upstream overrides.',
);

assert.ok(
  workflow.dependencies?.some((dependency) => (
    dependency.id === 'sdkwork-community'
      && dependency.repository === 'sdkwork-ai/sdkwork-community'
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
  dependencySurface.targetRuntimeIntegration?.connectivitySurface,
  'platform.api-gateway',
  'sdkwork-community app API must route through the shared platform.api-gateway root.',
);

assert.equal(
  componentSpec.integration?.platformApiGateway?.explicitExternalUpstreamEnvKeys,
  undefined,
  'component.spec.json must not publish per-module foundation upstream keys.',
);

assert.match(
  moduleRegistrySource,
  /COMMERCIAL_RUNTIME_MODULES[\s\S]*"community"/u,
  'Community must be enabled in commercial runtime modules after SDK wiring.',
);

assert.match(
  imCommunityAdapterSource,
  /getCommunityAppSdkClientWithSession/u,
  'IM community host adapter must consume the generated community app SDK client instead of fail-closed stubs.',
);
assert.match(
  imCommunityAdapterSource,
  /createGeneratedCommunityAppSdkPort/u,
  'IM community host adapter must bridge the generated community app SDK through community-runtime ports.',
);
assert.match(
  communityServiceSource,
  /getCommunityPcHost\(\)\.createAppSdkPort\(\)/u,
  'Canonical community service must consume the host-injected community app SDK port.',
);
assert.doesNotMatch(
  communityServiceSource,
  /pc community contract is not available/u,
  'Canonical community service must not keep the legacy contract-unavailable fail-closed stub.',
);
assert.match(
  communityBootstrapSource,
  /bootstrapCommunityPcForIm/u,
  'IM PC bootstrap must sync IM session into community PC runtime before rendering community UI.',
);
assert.match(
  communityBootstrapSource,
  /bootstrapImCommunityPcHost/u,
  'IM PC bootstrap must wire the thin community host adapter before rendering community UI.',
);

assert.match(
  communityClientSource,
  /bootstrapCommunityPcForIm|syncImSessionToCommunityPc/u,
  'Community PC integration must expose IM session bridge helpers.',
);
assert.match(
  communityClientSource,
  /@sdkwork\/community-app-sdk/u,
  'Community PC integration must import the composed community app SDK facade.',
);
assert.match(
  communityClientReexportSource,
  /from '\.\/communityPcIntegration'/u,
  'Legacy communityAppSdkClient export must re-export from communityPcIntegration only.',
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
