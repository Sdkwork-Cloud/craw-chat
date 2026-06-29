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
const tsconfig = readJson('tsconfig.json');
const pnpmWorkspaceSource = readText('pnpm-workspace.yaml');
const viteConfigSource = readText('vite.config.ts');
const releaseSources = readRepoJson('config', 'shared-sdk-release-sources.json');
const sharedSdkGitSource = readRepoText('scripts', 'dev', 'prepare-shared-sdk-git-sources.mjs');
const releaseBuildSource = readRepoText('scripts', 'release', 'run-sdkwork-im-pc-release-build.mjs');
const devRunnerSource = readRepoText('scripts', 'lib', 'im-pc-dev.mjs');
const gatewayConfigSource = readRepoText('crates', 'sdkwork-im-cloud-gateway-config', 'src', 'lib.rs');
const gatewaySource = readRepoText('services', 'sdkwork-im-cloud-gateway', 'src', 'lib.rs');
const workflow = readRepoJson('sdkwork.workflow.json');
const componentSpec = readRepoJson('specs', 'component.spec.json');
const moduleRegistrySource = readText('packages', 'sdkwork-im-pc-shell', 'src', 'moduleRegistry.ts');
const appAuthRuntimeSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'appAuthRuntime.ts',
);
const knowledgebaseClientSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'knowledgebaseAppSdkClient.ts',
);
const knowledgebasePcIntegrationSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'knowledgebasePcIntegration.ts',
);
const knowledgebaseBootstrapSource = readText('src', 'bootstrap', 'knowledgebasePc.ts');
const shellLoadersSource = readText(
  'packages',
  'sdkwork-im-pc-shell',
  'src',
  'capabilityModuleLoaders.ts',
);
const knowledgebasePcRoot = path.resolve(
  repoRoot,
  '..',
  'sdkwork-knowledgebase',
  'apps',
  'sdkwork-knowledgebase-pc',
);
const knowledgeViewSource = fs.readFileSync(
  path.join(knowledgebasePcRoot, 'packages', 'sdkwork-knowledgebase-pc-knowledge', 'src', 'KnowledgeView.tsx'),
  'utf8',
);

assert.equal(
  packageJson.scripts?.['test:knowledgebase-app-sdk-integration'],
  'node scripts/knowledgebase-app-sdk-integration-contract.test.mjs',
  'Chat PC must expose a dedicated knowledgebase app SDK integration contract script.',
);

assert.equal(
  packageJson.dependencies?.['@sdkwork/knowledgebase-app-sdk'],
  'workspace:*',
  'Chat PC must consume sdkwork-knowledgebase through the workspace app SDK package.',
);

assert.equal(
  packageJson.dependencies?.['@sdkwork/knowledgebase-pc-knowledge'],
  'workspace:*',
  'Chat PC must consume the sdkwork-knowledgebase-pc-knowledge embed package through workspace:*.',
);

assert.equal(
  readRepoJson('package.json').pnpm?.overrides?.['@sdkwork/knowledgebase-app-sdk'],
  'workspace:*',
  'Repository root pnpm overrides must keep sdkwork-knowledgebase app SDK on workspace:*.',
);

assert.match(
  pnpmWorkspaceSource,
  /sdkwork-knowledgebase-pc-knowledge/u,
  'pnpm-workspace.yaml must include the sdkwork-knowledgebase-pc-knowledge package.',
);

assert.equal(
  fs.existsSync(path.join(appRoot, 'packages', 'sdkwork-im-pc-knowledge')),
  false,
  'apps/sdkwork-im-pc must not keep a local sdkwork-im-pc-knowledge package.',
);

assert.match(
  releaseSources.sources?.['sdkwork-knowledgebase']?.repoUrl ?? '',
  /^(?:https:\/\/github\.com\/|git@github\.com:)Sdkwork-Cloud\/sdkwork-knowledgebase\.git$/u,
  'Shared SDK release config must materialize sdkwork-knowledgebase from the canonical git repository.',
);

assert.ok(
  typeof releaseSources.sources?.['sdkwork-knowledgebase']?.ref === 'string'
    && releaseSources.sources['sdkwork-knowledgebase'].ref.trim().length > 0,
  'Shared SDK release config must pin a non-empty sdkwork-knowledgebase git ref.',
);

assert.equal(
  releaseSources.sources?.['sdkwork-knowledgebase']?.ref,
  workflow.dependencies?.find((dependency) => dependency.id === 'sdkwork-knowledgebase')?.ref,
  'Shared SDK release config must use the same pinned sdkwork-knowledgebase ref as sdkwork.workflow.json.',
);

assert.match(
  sharedSdkGitSource,
  /id:\s*['"]sdkwork-knowledgebase['"][\s\S]*sdkwork-knowledgebase-app-sdk[\\/]sdkwork-knowledgebase-app-sdk-typescript[\\/]package\.json/u,
  'Shared SDK git materializer must know how to prepare the sdkwork-knowledgebase app SDK source.',
);

assert.match(
  sharedSdkGitSource,
  /SDKWORK_SHARED_KNOWLEDGEBASE_REPO_URL[\s\S]*SDKWORK_SHARED_KNOWLEDGEBASE_GIT_REF/u,
  'Shared SDK git materializer must expose sdkwork-knowledgebase repo/ref override environment variables.',
);

assert.match(
  releaseBuildSource,
  /SDKWORK_SHARED_KNOWLEDGEBASE_GIT_REF[\s\S]*SDKWORK_KNOWLEDGEBASE_REF/u,
  'Release build plan must bridge SDKWORK_KNOWLEDGEBASE_REF into the shared SDK materializer ref for the knowledgebase app SDK.',
);

assert.match(
  devRunnerSource,
  /SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL[\s\S]*explicitKnowledgebaseAppApiUpstream[\s\S]*SDKWORK_IM_KNOWLEDGEBASE_APP_API_UPSTREAM/u,
  'PC dev runner must default sdkwork-knowledgebase traffic through the shared gateway root while preserving explicit Knowledgebase split upstream overrides.',
);

assert.match(
  gatewayConfigSource,
  /sdkwork-knowledgebase-app-api[\s\S]*SDKWORK_IM_KNOWLEDGEBASE_APP_API_UPSTREAM[\s\S]*SDKWORK_KNOWLEDGEBASE_APP_API_UPSTREAM[\s\S]*SDKWORK_KNOWLEDGEBASE_APP_API_BASE_URL/u,
  'Gateway config must expose deterministic sdkwork-knowledgebase app-api upstream environment keys.',
);

assert.match(
  gatewaySource,
  /"sdkwork-knowledgebase-app-api"[\s\S]*\/app\/v3\/api\/knowledge\/\{\*path\}[\s\S]*SdkworkKnowledgebaseAppSdk/u,
  'Web gateway must route sdkwork-knowledgebase app-api paths to the Knowledgebase app SDK upstream.',
);

assert.ok(
  workflow.dependencies?.some((dependency) => (
    dependency.id === 'sdkwork-knowledgebase'
      && dependency.repository === 'Sdkwork-Cloud/sdkwork-knowledgebase'
      && dependency.refInput === 'SDKWORK_KNOWLEDGEBASE_REF'
      && dependency.tokenSecret === 'SDKWORK_RELEASE_TOKEN'
  )),
  'sdkwork.workflow.json must declare sdkwork-knowledgebase as a release dependency.',
);

const dependencySurface = componentSpec.contracts?.dependencyApiSurfaces?.find(
  (surface) => surface.apiAuthority === 'sdkwork-knowledgebase-app-api',
);
assert.ok(
  dependencySurface,
  'component.spec.json must declare sdkwork-knowledgebase-app-api dependency surface.',
);
assert.equal(
  dependencySurface.sdkFamily,
  'sdkwork-knowledgebase-app-sdk',
  'component.spec.json must bind sdkwork-knowledgebase-app-api to sdkwork-knowledgebase-app-sdk.',
);
assert.equal(
  dependencySurface.targetRuntimeIntegration?.gatewayApplication,
  'sdkwork-api-cloud-gateway',
  'sdkwork-knowledgebase app API must route through the shared sdkwork-api-cloud-gateway root.',
);

const splitOverrideEnvKeys =
  componentSpec.integration?.foundationApiGateway?.splitOverrideEnvKeys?.['sdkwork-knowledgebase-app-api'];
assert.deepEqual(
  splitOverrideEnvKeys,
  [
    'SDKWORK_IM_KNOWLEDGEBASE_APP_API_UPSTREAM',
    'SDKWORK_KNOWLEDGEBASE_APP_API_UPSTREAM',
    'SDKWORK_KNOWLEDGEBASE_APP_API_BASE_URL',
  ],
  'component.spec.json must document knowledgebase split upstream override env keys.',
);

assert.ok(
  componentSpec.integration?.foundationApiGateway?.standaloneUnifiedEmbeddedAuthorities?.includes(
    'sdkwork-knowledgebase-app-api',
  ),
  'component.spec.json must embed sdkwork-knowledgebase-app-api in standalone unified-process mode.',
);

assert.equal(
  componentSpec.integration?.foundationApiGateway?.standaloneUnifiedDeferredAuthorities?.includes(
    'sdkwork-knowledgebase-app-api',
  ),
  false,
  'component.spec.json must not defer sdkwork-knowledgebase-app-api after embedded bootstrap wiring.',
);

const embeddedRoutesSource = readRepoText(
  'services',
  'sdkwork-im-standalone-gateway',
  'src',
  'embedded_dependency_routes.rs',
);
assert.match(
  embeddedRoutesSource,
  /bootstrap_embedded_knowledgebase_routes[\s\S]*sdkwork_knowledgebase_gateway_assembly::assemble_application_business_router/u,
  'IM standalone gateway must embed sdkwork-knowledgebase routes in unified-process mode.',
);

assert.match(
  moduleRegistrySource,
  /COMMERCIAL_RUNTIME_MODULES[\s\S]*"knowledge"/u,
  'Knowledge must be enabled in commercial runtime modules after SDK wiring.',
);

assert.match(
  knowledgebaseClientSource,
  /@sdkwork\/knowledgebase-app-sdk/u,
  'Knowledgebase app SDK client wrapper must import the composed knowledgebase app SDK package.',
);

assert.match(
  knowledgebaseClientSource,
  /createKnowledgebaseAppClient/u,
  'Core knowledgebase client must use the sdkwork-knowledgebase composed app SDK factory.',
);

assert.match(
  knowledgebaseClientSource,
  /tokenManager:\s*getSdkworkChatGlobalTokenManager\(\)/u,
  'Core knowledgebase client must share the Sdkwork IM global token manager.',
);

assert.doesNotMatch(
  knowledgebaseClientSource,
  /fetch\(|axios|Authorization|Access-Token/u,
  'Core knowledgebase client must not assemble raw HTTP or auth headers.',
);

assert.match(
  knowledgebasePcIntegrationSource,
  /bootstrapKnowledgebasePcForIm/u,
  'IM core must expose knowledgebase PC integration bootstrap.',
);

assert.match(
  knowledgebasePcIntegrationSource,
  /configureKnowledgebasePcRuntime/u,
  'IM core must configure sdkwork-knowledgebase-pc runtime through the integration module.',
);

assert.match(
  functionBody(appAuthRuntimeSource, 'createSdkworkChatIamRuntime'),
  /rebootstrapKnowledgebasePcRuntimeForIm\(\)/u,
  'IM auth runtime must re-bootstrap knowledgebase PC runtime after session changes.',
);

assert.match(
  knowledgebaseBootstrapSource,
  /bootstrapKnowledgebasePcForIm/u,
  'IM app bootstrap must wire sdkwork-knowledgebase-pc host adapters.',
);

assert.match(
  shellLoadersSource,
  /import\('@sdkwork\/knowledgebase-pc-knowledge'\)/u,
  'IM shell must lazy-load the sdkwork-knowledgebase-pc-knowledge capability package.',
);

assert.match(
  knowledgebasePcIntegrationSource,
  /subscribeHostLanguage/u,
  'IM core must bridge host language changes into sdkwork-knowledgebase-pc runtime.',
);

assert.match(
  knowledgeViewSource,
  /createHostManagedKnowledgebaseRuntime/u,
  'Knowledgebase embed package must use host-managed runtime without standalone auth.',
);

assert.match(
  knowledgeViewSource,
  /syncKnowledgebaseHostLanguage/u,
  'Knowledgebase embed package must sync host-managed language on mount.',
);

assert.match(
  knowledgeViewSource,
  /<I18nextProvider i18n=\{i18n(?:\s+as\s+[^}]+)?\}>/u,
  'Knowledgebase embed package must isolate its i18next instance from the IM host provider.',
);

assert.match(
  knowledgeViewSource,
  /index\.css/u,
  'Knowledgebase embed package must import the knowledgebase application stylesheet for host-managed UI fidelity.',
);

assert.match(
  viteConfigSource,
  /sdkwork-knowledgebase-app-sdk\/sdkwork-knowledgebase-app-sdk-typescript\/src\/index\.ts/u,
  'Vite must alias @sdkwork/knowledgebase-app-sdk to the composed knowledgebase app SDK entry.',
);

assert.match(
  tsconfig.compilerOptions?.paths?.['@sdkwork/knowledgebase-app-sdk']?.join('/') ?? '',
  /sdkwork-knowledgebase-app-sdk-typescript\/src\/index\.ts/u,
  'tsconfig must map @sdkwork/knowledgebase-app-sdk to the composed knowledgebase app SDK entry.',
);

assert.match(
  pnpmWorkspaceSource,
  /sdkwork-knowledgebase-app-sdk\/sdkwork-knowledgebase-app-sdk-typescript/u,
  'pnpm-workspace.yaml must include the composed sdkwork-knowledgebase-app-sdk-typescript package.',
);

assert.match(
  viteConfigSource,
  /@sdkwork\/knowledgebase-pc-knowledge/u,
  'Vite must alias @sdkwork/knowledgebase-pc-knowledge for host-managed knowledgebase embedding.',
);

assert.ok(
  tsconfig.compilerOptions?.paths?.['@sdkwork/knowledgebase-pc-knowledge'],
  'tsconfig must map @sdkwork/knowledgebase-pc-knowledge for host-managed knowledgebase embedding.',
);

console.log('sdkwork im knowledgebase app SDK integration contract passed.');
