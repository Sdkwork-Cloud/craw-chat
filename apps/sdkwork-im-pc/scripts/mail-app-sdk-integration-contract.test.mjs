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
const mailServiceSource = readText('packages', 'sdkwork-im-pc-mail', 'src', 'services', 'MailService.ts');
const mailClientSource = readText('packages', 'sdkwork-im-pc-core', 'src', 'sdk', 'mailAppSdkClient.ts');
const viteConfigSource = readText('vite.config.ts');
const tsconfig = readJson('tsconfig.json');

assert.equal(
  packageJson.scripts?.['test:mail-app-sdk-integration'],
  'node scripts/mail-app-sdk-integration-contract.test.mjs',
  'Chat PC must expose a dedicated mail app SDK integration contract script.',
);

assert.equal(
  packageJson.dependencies?.['@sdkwork/mail-app-sdk'] ?? readJson('packages', 'sdkwork-im-pc-core', 'package.json').dependencies?.['@sdkwork/mail-app-sdk'],
  'workspace:*',
  'Chat PC must consume sdkwork-mail through the workspace app SDK package.',
);

assert.match(
  releaseSources.sources?.['sdkwork-mail']?.repoUrl ?? '',
  /^(?:https:\/\/github\.com\/|git@github\.com:)Sdkwork-Cloud\/sdkwork-mail\.git$/u,
  'Shared SDK release config must materialize sdkwork-mail from the canonical git repository.',
);

assert.ok(
  typeof releaseSources.sources?.['sdkwork-mail']?.ref === 'string'
    && releaseSources.sources['sdkwork-mail'].ref.trim().length > 0,
  'Shared SDK release config must pin a non-empty sdkwork-mail git ref.',
);

assert.equal(
  releaseSources.sources?.['sdkwork-mail']?.ref,
  workflow.dependencies?.find((dependency) => dependency.id === 'sdkwork-mail')?.ref,
  'Shared SDK release config must use the same pinned sdkwork-mail ref as sdkwork.workflow.json.',
);

assert.match(
  sharedSdkGitSource,
  /id:\s*['"]sdkwork-mail['"][\s\S]*sdkwork-mail-app-sdk[\\/]sdkwork-mail-app-sdk-typescript[\\/]generated[\\/]server-openapi[\\/]package\.json/u,
  'Shared SDK git materializer must know how to prepare the sdkwork-mail app SDK source.',
);

assert.match(
  sharedSdkGitSource,
  /SDKWORK_SHARED_MAIL_REPO_URL[\s\S]*SDKWORK_SHARED_MAIL_GIT_REF/u,
  'Shared SDK git materializer must expose sdkwork-mail repo/ref override environment variables.',
);

assert.match(
  releaseBuildSource,
  /SDKWORK_SHARED_MAIL_GIT_REF[\s\S]*SDKWORK_MAIL_REF/u,
  'Release build plan must bridge SDKWORK_MAIL_REF into the shared SDK materializer ref for the mail app SDK.',
);

assert.match(
  devRunnerSource,
  /SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL[\s\S]*explicitMailAppApiUpstream[\s\S]*SDKWORK_IM_MAIL_APP_API_UPSTREAM/u,
  'PC dev runner must default sdkwork-mail traffic through the shared gateway root while preserving explicit Mail split upstream overrides.',
);

assert.match(
  gatewayConfigSource,
  /sdkwork-mail-app-api[\s\S]*SDKWORK_IM_MAIL_APP_API_UPSTREAM[\s\S]*SDKWORK_MAIL_APP_API_UPSTREAM[\s\S]*SDKWORK_MAIL_APP_API_BASE_URL/u,
  'Gateway config must expose deterministic sdkwork-mail app-api upstream environment keys.',
);

assert.match(
  gatewaySource,
  /"sdkwork-mail-app-api"[\s\S]*\/app\/v3\/api\/mail\/\{\*path\}[\s\S]*SdkworkMailAppSdk/u,
  'Web gateway must route sdkwork-mail app-api paths to the Mail app SDK upstream.',
);

assert.ok(
  workflow.dependencies?.some((dependency) => (
    dependency.id === 'sdkwork-mail'
      && dependency.repository === 'Sdkwork-Cloud/sdkwork-mail'
      && dependency.refInput === 'SDKWORK_MAIL_REF'
      && dependency.tokenSecret === 'SDKWORK_RELEASE_TOKEN'
  )),
  'sdkwork.workflow.json must declare sdkwork-mail as a release dependency.',
);

const dependencySurface = componentSpec.contracts?.dependencyApiSurfaces?.find(
  (surface) => surface.apiAuthority === 'sdkwork-mail-app-api',
);
assert.ok(dependencySurface, 'component.spec.json must declare sdkwork-mail-app-api dependency surface.');
assert.equal(
  dependencySurface.sdkFamily,
  'sdkwork-mail-app-sdk',
  'component.spec.json must bind sdkwork-mail-app-api to sdkwork-mail-app-sdk.',
);
assert.equal(
  dependencySurface.targetRuntimeIntegration?.gatewayApplication,
  'sdkwork-api-cloud-gateway',
  'sdkwork-mail app API must route through the shared sdkwork-api-cloud-gateway root.',
);

const splitOverrideEnvKeys =
  componentSpec.integration?.foundationApiGateway?.splitOverrideEnvKeys?.['sdkwork-mail-app-api'];
assert.deepEqual(
  splitOverrideEnvKeys,
  [
    'SDKWORK_IM_MAIL_APP_API_UPSTREAM',
    'SDKWORK_MAIL_APP_API_UPSTREAM',
    'SDKWORK_MAIL_APP_API_BASE_URL',
  ],
  'component.spec.json must document mail split upstream override env keys.',
);

assert.match(
  moduleRegistrySource,
  /COMMERCIAL_RUNTIME_MODULES[\s\S]*"mail"/u,
  'Mail must be enabled in commercial runtime modules after SDK wiring.',
);

assert.match(
  mailServiceSource,
  /getMailAppSdkClientWithSession/u,
  'MailService must consume the generated mail app SDK client instead of fail-closed stubs.',
);
assert.doesNotMatch(
  mailServiceSource,
  /PC_MAIL_CONTRACT_UNAVAILABLE/u,
  'MailService must not keep contract-unavailable fail-closed stubs.',
);

assert.match(
  mailClientSource,
  /@sdkwork\/mail-app-sdk/u,
  'Mail app SDK client wrapper must import the composed mail app SDK package.',
);

assert.match(
  viteConfigSource,
  /@sdkwork\/mail-app-sdk/u,
  'Vite config must alias @sdkwork/mail-app-sdk for PC mail integration.',
);

assert.ok(
  tsconfig.compilerOptions?.paths?.['@sdkwork/mail-app-sdk'],
  'tsconfig must map @sdkwork/mail-app-sdk for PC mail integration.',
);

console.log('mail app SDK integration contract checks passed');
