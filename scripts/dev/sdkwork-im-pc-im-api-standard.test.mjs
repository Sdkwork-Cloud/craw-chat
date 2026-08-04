import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

const localSpec = read('specs/im-app-api-sdk-integration.spec.md');
const specsReadme = read('specs/README.md');
const componentSpec = readJson('specs/component.spec.json');
const appPackageJson = readJson('apps/sdkwork-im-pc/package.json');
const retiredGenericAppSdkPackage = `@sdkwork/${'app'}-sdk`;
const retiredGenericBackendSdkPackage = `@sdkwork/${'backend'}-sdk`;
const appSdkClientSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/appSdkClient.ts');
const appbaseAppSdkClientSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/appbaseAppSdkClient.ts');
const appAuthRuntimeSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/appAuthRuntime.ts');
const imSdkClientSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/imSdkClient.ts');
const appAuthServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/appAuthService.ts');
const viteConfigSource = read('apps/sdkwork-im-pc/vite.config.ts');
const localApiSource = read('apps/sdkwork-im-pc/local-api.ts');
const devCommandSource = read('scripts/lib/im-pc-dev.mjs');
const retiredLocalAppApiLauncher = path.join(repoRoot, 'scripts/dev/start-sdkwork-im-local-app-api.mjs');
const sharedDatabaseSource = read('scripts/dev/sdkwork-im-shared-database.mjs');
const releaseSources = readJson('config/shared-sdk-release-sources.json');
const workspaceSource = read('pnpm-workspace.yaml');
const deploymentIndex = readJson('etc/sdkwork.deployment.config.json');
const topologySpec = readJson('specs/topology.spec.json');
const standaloneGatewayMainSource = read('crates/sdkwork-api-im-standalone-gateway/src/main.rs');
const applicationAssemblySource = read('crates/sdkwork-api-im-assembly/src/bootstrap.rs');
const standaloneDependencySource = read(
  'crates/sdkwork-api-im-standalone-gateway/src/embedded_dependency_routes.rs',
);

for (const environment of ['development', 'test', 'staging', 'production']) {
  for (const deploymentProfile of ['standalone', 'cloud']) {
    const profileId = `${deploymentProfile}.${environment}`;
    const relativeProfilePath = `etc/topology/${profileId}.env`;
    const profileSource = read(relativeProfilePath);
    assert.equal(
      deploymentIndex.profiles?.[profileId]?.config,
      `topology/${profileId}.env`,
      `${profileId} must resolve through the source etc deployment index`,
    );
    assert.equal(
      topologySpec.profileFiles?.[profileId],
      relativeProfilePath,
      `${profileId} must resolve through topology v5`,
    );
    assert.match(
      profileSource,
      new RegExp(`^SDKWORK_IM_DEPLOYMENT_PROFILE=${deploymentProfile}$`, 'mu'),
      `${profileId} must declare its deployment profile in the tracked source config`,
    );
  }
}

for (const environment of ['development', 'test', 'staging', 'production']) {
  assert.equal(
    fs.existsSync(path.join(repoRoot, `etc/platform.api-gateway.sdkwork-im.${environment}.toml`)),
    false,
    `${environment} must not restore the retired per-environment API cloud gateway config`,
  );
}

assert.match(
  standaloneGatewayMainSource,
  /sdkwork_api_im_assembly::assemble_api_router_with_realtime_bootstrap/u,
  'standalone gateway must consume the canonical IM API assembly in process',
);
assert.match(
  applicationAssemblySource,
  /sdkwork_routes_im_realtime_open_api::build_public_app_with_realtime_bootstrap_from_env/u,
  'IM API assembly must mount the realtime route crate through its live bootstrap',
);
for (const dependency of ['drive', 'mail', 'notary']) {
  assert.match(
    standaloneDependencySource,
    new RegExp(`bootstrap_embedded_${dependency}_routes`, 'u'),
    `standalone gateway must publish ${dependency} through an embedded assembly`,
  );
}

assert.match(
  specsReadme,
  /im-app-api-sdk-integration\.spec\.md/u,
  'Sdkwork IM specs README must register the local IM app API and SDK integration standard.',
);
assert.ok(
  componentSpec.localExtensionSpecs?.some((entry) => entry.file === 'im-app-api-sdk-integration.spec.md'),
  'component.spec.json must register the local IM app API and SDK integration standard.',
);

for (const requiredText of [
  'IM API',
  'im-open-api',
  '/im/v3/api',
  'IM App API',
  '/app/v3/api',
  'IM Backend API',
  '/backend/v3/api',
  'sdkwork-im-app-sdk',
  'SdkworkImAppClient',
  'sdkwork-im-backend-sdk',
  'SdkworkImBackendClient',
  '@sdkwork/im-sdk',
  'live/chat/game rooms',
  'oauth.deviceAuthorizations.create',
  'oauth.deviceAuthorizations.retrieve',
  'scans.create',
  'passwordCompletions.create',
  'PostgreSQL',
  'SDKWORK_SHARED_SDK_MODE=git',
]) {
  assert.match(localSpec, new RegExp(requiredText.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'));
}
assert.doesNotMatch(
  localSpec,
  /QR scan login is enabled through `openPlatform\.qrAuth|-> im-app-api auth\.sessions \/ registrations \/ verificationCodes \/ openPlatform\.qrAuth/u,
  'local standard must not document legacy appbase QR auth as the current Sdkwork IM integration surface.',
);

assert.match(
  localSpec,
  /must not import retired generic Spring app\/backend SDK packages or authorities/iu,
  'local standard must explicitly forbid generic app/backend SDK imports for Sdkwork IM.',
);
assert.match(
  localSpec,
  /After login, chat and RTC code must be able to call IM SDK methods without a second login/iu,
  'local standard must require login-to-IM session continuity.',
);
assert.match(
  localSpec,
  /must not pass unsupported runtime-config fields such as `qrLoginType`/iu,
  'local standard must document that current canonical appbase auth config only accepts qrLoginEnabled.',
);

assert.ok(appPackageJson.dependencies['@sdkwork/im-app-sdk']);
assert.ok(appPackageJson.dependencies['@sdkwork/im-backend-sdk']);
assert.ok(!appPackageJson.dependencies['@sdkwork-internal/im-app-api-generated']);
assert.ok(!appPackageJson.dependencies['@sdkwork-internal/im-backend-api-generated']);
assert.ok(appPackageJson.dependencies['@sdkwork/iam-app-sdk']);
assert.ok(!appPackageJson.dependencies[retiredGenericAppSdkPackage]);
assert.ok(!appPackageJson.dependencies[retiredGenericBackendSdkPackage]);

assert.match(appSdkClientSource, /SdkworkImAppClient/u);
assert.match(appSdkClientSource, /@sdkwork\/im-app-sdk/u);
assert.doesNotMatch(appSdkClientSource, /@sdkwork-internal\/im-app-api-generated/u);
assert.doesNotMatch(
  appSdkClientSource,
  /@sdkwork\/(?:app|backend)-sdk|spring-ai-plus-(?:app|backend)-api/u,
);
assert.match(appbaseAppSdkClientSource, /SdkworkAppbaseAppClient/u);
assert.match(appbaseAppSdkClientSource, /@sdkwork\/iam-app-sdk/u);
assert.doesNotMatch(appbaseAppSdkClientSource, /@sdkwork-internal\/im-app-api-generated/u);

assert.match(appAuthServiceSource, /createSdkworkAuthAppbaseIntegration/u);
assert.match(appAuthServiceSource, /getSdkworkChatIamRuntime\(\)\.service\.auth\.sessions\.current\.retrieve/u);
assert.match(appAuthServiceSource, /getSdkworkChatIamRuntime\(\)\.service\.auth\.sessions\.current\.delete/u);
assert.doesNotMatch(appAuthServiceSource, /@sdkwork-internal\/im-app-api-generated/u);
assert.doesNotMatch(appAuthServiceSource, /\bfetch\s*\(/u);

assert.match(appAuthRuntimeSource, /loginMethods:\s*\[\s*['"]password['"]\s*\]/u);
assert.match(appAuthRuntimeSource, /createSdkworkAppbasePcAuthRuntime/u);
assert.match(appAuthRuntimeSource, /sdkClients:\s*getAuthenticatedSdkClients\(\)/u);
assert.match(appAuthRuntimeSource, /tokenManager:\s*getSdkworkChatGlobalTokenManager\(\)/u);
assert.match(appAuthRuntimeSource, /sessionBridge:\s*\{/u);
assert.match(appAuthRuntimeSource, /commitSession:\s*\(session\)\s*=>\s*applyAppSdkSessionTokens/u);
assert.match(appAuthRuntimeSource, /readSession:\s*readAppSdkSessionTokens/u);
assert.match(appAuthRuntimeSource, /appbaseAppApiBaseUrl:\s*resolveAppSdkBaseUrl\(\)/u);
assert.doesNotMatch(
  appAuthRuntimeSource,
  /appbaseBackendApiBaseUrl|resolveBackendSdkBaseUrl/u,
  'Sdkwork IM app auth runtime must not construct backend SDK base URLs on the app UI surface.',
);
assert.match(appAuthRuntimeSource, /qrLoginEnabled:\s*true/u);
assert.doesNotMatch(
  appAuthRuntimeSource,
  /qrLoginType/u,
  'Sdkwork IM runtime config must not pass qrLoginType until canonical @sdkwork/auth-pc-react exposes it.',
);

for (const requiredImSessionFragment of [
  'tokenProvider: tokenManager',
  'accessToken: resolveAppSdkAccessToken',
  'ImWebSocketAuthOptions.automatic',
]) {
  assert.match(
    imSdkClientSource,
    new RegExp(requiredImSessionFragment.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
    `IM SDK client must preserve IAM session continuity via ${requiredImSessionFragment}`,
  );
}
assert.doesNotMatch(
  imSdkClientSource,
  /tenantId:\s*resolveAppSdkTenantId|organizationId:\s*resolveAppSdkOrganizationId/u,
  'IM SDK client must not pass current tenantId/organizationId as static options; server context comes from tokens.',
);
assert.doesNotMatch(
  imSdkClientSource,
  /headerProvider:\s*\(\)\s*=>|buildImSdkContextHeaders|Sdkwork-(?:Tenant|Organization|User|Session|Actor|Device)-Id/u,
  'IM SDK client must not inject request context through custom headers.',
);
assert.doesNotMatch(imSdkClientSource, /\bfetch\s*\(/u);

assert.match(
  viteConfigSource,
  /@sdkwork\/im-sdk/u,
  'Vite must alias @sdkwork/im-sdk to the composed IM SDK facade.',
);
assert.match(
  viteConfigSource,
  /sdkwork-im-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
  'Vite must resolve @sdkwork/im-sdk to the composed facade source.',
);
assert.doesNotMatch(
  viteConfigSource,
  /@sdkwork\/im-sdk-generated|sdkwork-im-sdk-typescript[\\\/]generated[\\\/]server-openapi/u,
  'Vite must not expose the IM generated transport alias to PC consumers.',
);
assert.match(viteConfigSource, /@sdkwork\/sdkwork-knowledgebase-pc-commons/u);
assert.match(viteConfigSource, /sdkworkChatLocalApiPlugin/u);
assert.match(viteConfigSource, /handleSdkworkChatLocalApiRequest/u);
assert.doesNotMatch(
  viteConfigSource,
  /GEMINI_API_KEY|AI Studio|@google\/genai/u,
  'vite config must not retain AI Studio or Gemini scaffold wiring',
);
assert.doesNotMatch(
  localApiSource,
  /GEMINI_API_KEY|AI Studio|@google\/genai/u,
  'local-api must not retain AI Studio or Gemini scaffold dependencies',
);
for (const localShellEndpoint of ['/api/config/modules', '/api/agent/doc', '/api/agent/icon']) {
  assert.match(localSpec, new RegExp(localShellEndpoint.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'));
  assert.match(localApiSource, new RegExp(localShellEndpoint.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'));
}
assert.match(
  localSpec,
  /must be classified under `im-app-api`, added to `\/app\/v3\/api` OpenAPI, regenerated through `sdkwork-im-app-sdk`/u,
  'local shell endpoints must have an explicit im-app-api OpenAPI promotion rule.',
);
assert.match(
  localSpec,
  /Local PC development exposes the IM application plane on `http:\/\/127\.0\.0\.1:18079`[\s\S]*Foundation APIs use the single platform gateway root/u,
  'local standard must separate the IM application plane from the platform assembly gateway.',
);

assert.equal(
  fs.existsSync(retiredLocalAppApiLauncher),
  false,
  'retired local app-api sidecar launcher must not remain as a compatibility wrapper',
);
assert.match(sharedDatabaseSource, /SDKWORK_DATABASE_ENGINE/u);
assert.match(sharedDatabaseSource, /SDKWORK_DATABASE_SSL_MODE/u);
assert.match(sharedDatabaseSource, /postgres(?:ql)?:/u);
assert.doesNotMatch(
  sharedDatabaseSource,
  /SDKWORK_[A-Z0-9]+_DATABASE_/u,
  'shared database resolution must not define application- or module-prefixed database keys',
);
assert.match(devCommandSource, /resolveSdkworkImSharedDatabaseConfig/u);
assert.match(devCommandSource, /SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL/u);
assert.doesNotMatch(
  devCommandSource,
  /SDKWORK_IM_DRIVE_APP_API_UPSTREAM:\s*resolveDriveAppApiUpstream|SDKWORK_IM_NOTARY_APP_API_UPSTREAM:\s*resolveNotaryAppApiUpstream|SDKWORK_IM_CATALOG_APP_API_UPSTREAM:\s*resolveCatalogAppApiUpstream|SDKWORK_IM_MAIL_APP_API_UPSTREAM:\s*resolveMailAppApiUpstream|SDKWORK_IM_COMMUNITY_APP_API_UPSTREAM:\s*resolveCommunityAppApiUpstream|SDKWORK_IM_COURSE_APP_API_UPSTREAM:\s*resolveCourseAppApiUpstream/u,
  'local PC development must use one shared platform.api-gateway root by default instead of materializing per-module foundation upstreams.',
);
assert.doesNotMatch(
  devCommandSource,
  /\bmvn(?:\.cmd)?\b|spring-ai-plus-server-app|spring-boot:run|SDKWORK_IM_APPBASE_APP_API_UPSTREAM|SDKWORK_APPBASE_APP_API_BIND_ADDR|SDKWORK_APPBASE_BROWSER_ORIGINS/u,
  'local PC development must use the Rust unified server for app-api instead of Java/appbase upstream startup.',
);

for (const sourceName of [
  'sdkwork-im-app-sdk',
  'sdkwork-im-backend-sdk',
  'sdkwork-im-sdk',
  'sdkwork-notary',
  'sdkwork-appbase',
  'sdkwork-drive',
  'sdkwork-catalog',
  'sdkwork-shop',
  'sdkwork-order',
  'sdkwork-mail',
  'sdkwork-community',
  'sdkwork-course',
  'sdkwork-core',
  'sdkwork-ui',
  'sdkwork-cloudrouter',
]) {
  assert.ok(releaseSources.sources?.[sourceName], `release shared SDK config must include ${sourceName}`);
}

assert.doesNotMatch(
  workspaceSource,
  /\.\.\/\.\.\/\.\.\/\.\.\/apps\/sdkwork-(?:appbase|core|ui)\//u,
  'chat-pc workspace must not register sibling appbase/core/ui packages as install targets.',
);

console.log('sdkwork-im-pc IM app API standard contract passed');
