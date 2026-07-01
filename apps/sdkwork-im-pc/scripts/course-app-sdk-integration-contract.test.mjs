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
const pnpmWorkspaceSource = readRepoText('pnpm-workspace.yaml');
const viteConfigSource = readText('vite.config.ts');
const releaseSources = readRepoJson('config', 'shared-sdk-release-sources.json');
const sharedSdkGitSource = readRepoText('scripts', 'dev', 'prepare-shared-sdk-git-sources.mjs');
const releaseBuildSource = readRepoText('scripts', 'release', 'run-sdkwork-im-pc-release-build.mjs');
const devRunnerSource = readRepoText('scripts', 'lib', 'im-pc-dev.mjs');
const gatewayConfigSource = readRepoText('crates', 'sdkwork-im-cloud-gateway-config', 'src', 'lib.rs');
const gatewaySource = readRepoText('services', 'sdkwork-im-cloud-gateway', 'src', 'lib.rs');
const gatewayRegistrySource = readRepoText('services', 'sdkwork-im-cloud-gateway', 'src', 'registry.rs');
const gatewayConstantsSource = readRepoText('services', 'sdkwork-im-cloud-gateway', 'src', 'constants.rs');
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
const courseClientSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'courseAppSdkClient.ts',
);
const coursePcIntegrationSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'coursePcIntegration.ts',
);
const courseBootstrapSource = readText('src', 'bootstrap', 'coursePc.ts');
const shellLoadersSource = readText(
  'packages',
  'sdkwork-im-pc-shell',
  'src',
  'capabilityModuleLoaders.ts',
);
const coursePcRoot = path.resolve(
  repoRoot,
  '..',
  'sdkwork-course',
  'apps',
  'sdkwork-course-pc',
);
const courseViewSource = fs.readFileSync(
  path.join(coursePcRoot, 'packages', 'sdkwork-course-pc-course', 'src', 'components', 'CourseView.tsx'),
  'utf8',
);
const courseServiceSource = fs.readFileSync(
  path.join(coursePcRoot, 'packages', 'sdkwork-course-pc-course', 'src', 'services', 'CourseService.ts'),
  'utf8',
);

assert.equal(
  packageJson.scripts?.['test:course-app-sdk-integration'],
  'node scripts/course-app-sdk-integration-contract.test.mjs',
  'Chat PC must expose a dedicated course app SDK integration contract script.',
);

assert.equal(
  corePackageJson.dependencies?.['@sdkwork/course-app-sdk'],
  'workspace:*',
  '@sdkwork/im-pc-core must consume sdkwork-course through the workspace app SDK package.',
);

assert.equal(
  shellPackageJson.dependencies?.['@sdkwork/course-pc-course'],
  'workspace:*',
  '@sdkwork/im-pc-shell must consume the sdkwork-course-pc-course embed package through workspace:*.',
);

assert.equal(
  readRepoJson('package.json').pnpm?.overrides?.['@sdkwork/course-app-sdk'],
  'workspace:*',
  'Repository root pnpm overrides must keep sdkwork-course app SDK on workspace:*.',
);

assert.match(
  pnpmWorkspaceSource,
  /sdkwork-course-pc-course/u,
  'pnpm-workspace.yaml must include the sdkwork-course-pc-course package.',
);

assert.equal(
  fs.existsSync(path.join(appRoot, 'packages', 'sdkwork-im-pc-course')),
  false,
  'apps/sdkwork-im-pc must not keep a local sdkwork-im-pc-course package.',
);

assert.match(
  releaseSources.sources?.['sdkwork-course']?.repoUrl ?? '',
  /^(?:https:\/\/github\.com\/|git@github\.com:)Sdkwork-Cloud\/sdkwork-course\.git$/u,
  'Shared SDK release config must materialize sdkwork-course from the canonical git repository.',
);

assert.ok(
  typeof releaseSources.sources?.['sdkwork-course']?.ref === 'string'
    && releaseSources.sources['sdkwork-course'].ref.trim().length > 0,
  'Shared SDK release config must pin a non-empty sdkwork-course git ref.',
);

assert.equal(
  releaseSources.sources?.['sdkwork-course']?.ref,
  workflow.dependencies?.find((dependency) => dependency.id === 'sdkwork-course')?.ref,
  'Shared SDK release config must use the same pinned sdkwork-course ref as sdkwork.workflow.json.',
);

assert.match(
  sharedSdkGitSource,
  /id:\s*['"]sdkwork-course['"][\s\S]*sdkwork-course-app-sdk[\\/]sdkwork-course-app-sdk-typescript[\\/]generated[\\/]server-openapi[\\/]package\.json/u,
  'Shared SDK git materializer must know how to prepare the sdkwork-course app SDK source.',
);

assert.match(
  sharedSdkGitSource,
  /SDKWORK_SHARED_COURSE_REPO_URL[\s\S]*SDKWORK_SHARED_COURSE_GIT_REF/u,
  'Shared SDK git materializer must expose sdkwork-course repo/ref override environment variables.',
);

assert.match(
  releaseBuildSource,
  /SDKWORK_SHARED_COURSE_GIT_REF[\s\S]*SDKWORK_COURSE_REF/u,
  'Release build plan must bridge SDKWORK_COURSE_REF into the shared SDK materializer ref for the course app SDK.',
);

assert.match(
  devRunnerSource,
  /SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL[\s\S]*explicitCourseAppApiUpstream[\s\S]*SDKWORK_IM_COURSE_APP_API_UPSTREAM/u,
  'PC dev runner must default sdkwork-course traffic through the shared gateway root while preserving explicit Course split upstream overrides.',
);

assert.match(
  gatewayConfigSource,
  /sdkwork-course-app-api[\s\S]*SDKWORK_IM_COURSE_APP_API_UPSTREAM[\s\S]*SDKWORK_COURSE_APP_API_UPSTREAM[\s\S]*SDKWORK_COURSE_APP_API_BASE_URL/u,
  'Gateway config must expose deterministic sdkwork-course app-api upstream environment keys.',
);

assert.match(
  gatewayConstantsSource,
  /COURSE_APP_API_SEGMENTS[\s\S]*"courses"[\s\S]*"course_applications"/u,
  'Web gateway must declare sdkwork-course app-api route segments.',
);

assert.match(
  gatewayRegistrySource,
  /"sdkwork-course-app-api"[\s\S]*SdkworkCourseAppSdk/u,
  'Web gateway must route sdkwork-course app-api paths to the Course app SDK upstream.',
);

assert.ok(
  workflow.dependencies?.some((dependency) => (
    dependency.id === 'sdkwork-course'
      && dependency.repository === 'Sdkwork-Cloud/sdkwork-course'
      && dependency.refInput === 'SDKWORK_COURSE_REF'
      && dependency.tokenSecret === 'SDKWORK_RELEASE_TOKEN'
  )),
  'sdkwork.workflow.json must declare sdkwork-course as a release dependency.',
);

const dependencySurface = componentSpec.contracts?.dependencyApiSurfaces?.find(
  (surface) => surface.apiAuthority === 'sdkwork-course-app-api',
);
assert.ok(
  dependencySurface,
  'component.spec.json must declare sdkwork-course-app-api dependency surface.',
);
assert.equal(
  dependencySurface.sdkFamily,
  'sdkwork-course-app-sdk',
  'component.spec.json must bind sdkwork-course-app-api to sdkwork-course-app-sdk.',
);
assert.equal(
  dependencySurface.targetRuntimeIntegration?.gatewayApplication,
  'sdkwork-api-cloud-gateway',
  'sdkwork-course app API must route through the shared sdkwork-api-cloud-gateway root.',
);

const splitOverrideEnvKeys =
  componentSpec.integration?.foundationApiGateway?.splitOverrideEnvKeys?.['sdkwork-course-app-api'];
assert.deepEqual(
  splitOverrideEnvKeys,
  [
    'SDKWORK_IM_COURSE_APP_API_UPSTREAM',
    'SDKWORK_COURSE_APP_API_UPSTREAM',
    'SDKWORK_COURSE_APP_API_BASE_URL',
  ],
  'component.spec.json must document course split upstream override env keys.',
);

assert.ok(
  componentSpec.integration?.foundationApiGateway?.standaloneUnifiedEmbeddedAuthorities?.includes(
    'sdkwork-course-app-api',
  ),
  'component.spec.json must embed sdkwork-course-app-api in standalone unified-process mode.',
);

assert.equal(
  componentSpec.integration?.foundationApiGateway?.standaloneUnifiedDeferredAuthorities?.includes(
    'sdkwork-course-app-api',
  ),
  false,
  'component.spec.json must not defer sdkwork-course-app-api after embedded bootstrap wiring.',
);

const embeddedRoutesSource = readRepoText(
  'services',
  'sdkwork-im-standalone-gateway',
  'src',
  'embedded_dependency_routes.rs',
);
assert.match(
  embeddedRoutesSource,
  /bootstrap_embedded_course_routes[\s\S]*sdkwork_course_gateway_assembly::assemble_application_business_router/u,
  'IM standalone gateway must embed sdkwork-course routes in unified-process mode.',
);

assert.match(
  moduleRegistrySource,
  /COMMERCIAL_RUNTIME_MODULES[\s\S]*"course"/u,
  'Course must be enabled in commercial runtime modules after SDK wiring.',
);

assert.match(
  courseClientSource,
  /@sdkwork\/course-app-sdk/u,
  'Course app SDK client wrapper must import the composed course app SDK package.',
);

assert.match(
  courseClientSource,
  /tokenManager:\s*getSdkworkChatGlobalTokenManager\(\)/u,
  'Core course client must share the Sdkwork IM global token manager.',
);

assert.doesNotMatch(
  courseClientSource,
  /fetch\(|axios|Authorization|Access-Token/u,
  'Core course client must not assemble raw HTTP or auth headers.',
);

assert.match(
  coursePcIntegrationSource,
  /bootstrapCoursePcForIm/u,
  'IM core must expose course PC integration bootstrap.',
);

assert.match(
  coursePcIntegrationSource,
  /configureCoursePcRuntime/u,
  'IM core must configure sdkwork-course-pc runtime through the integration module.',
);

assert.match(
  functionBody(appAuthRuntimeSource, 'createSdkworkChatIamRuntime'),
  /rebootstrapCoursePcRuntimeForIm\(\)/u,
  'IM auth runtime must re-bootstrap course PC runtime after session changes.',
);

assert.match(
  courseBootstrapSource,
  /bootstrapCoursePcForIm/u,
  'IM app bootstrap must wire sdkwork-course-pc host adapters.',
);

assert.match(
  shellLoadersSource,
  /import\('@sdkwork\/course-pc-course'\)/u,
  'IM shell must lazy-load the sdkwork-course-pc-course capability package.',
);

assert.match(
  coursePcIntegrationSource,
  /subscribeHostLanguage/u,
  'IM core must bridge host language changes into sdkwork-course-pc runtime.',
);

assert.match(
  courseViewSource,
  /syncCoursePcHostLanguage/u,
  'Course embed package must sync host-managed language on mount.',
);

assert.match(
  courseServiceSource,
  /getCoursePcSdkPorts\(\)\.getCourseClient/u,
  'Course embed service must consume the host-injected course app SDK client.',
);

assert.doesNotMatch(
  courseServiceSource,
  /unsplash|COURSES\s*=\s*\[/u,
  'Course embed service must not keep local mock catalog data.',
);

assert.match(
  viteConfigSource,
  /@sdkwork\/course-pc-course/u,
  'Vite must alias @sdkwork/course-pc-course for host-managed course embedding.',
);

assert.ok(
  tsconfig.compilerOptions?.paths?.['@sdkwork/course-pc-course'],
  'tsconfig must map @sdkwork/course-pc-course for host-managed course embedding.',
);

console.log('sdkwork im course app SDK integration contract passed.');
