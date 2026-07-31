import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const readJson = (relativePath) => JSON.parse(readFileSync(path.join(repoRoot, relativePath), 'utf8'));
const readText = (relativePath) => readFileSync(path.join(repoRoot, relativePath), 'utf8');

const authority = readJson('sdkwork.app.config.json');
const pc = readJson('apps/sdkwork-im-pc/sdkwork.app.config.json');
const h5 = readJson('apps/sdkwork-im-h5/sdkwork.app.config.json');
const routing = readJson('specs/im-web-ingress-domain.spec.json');
const apiDeployment = readJson('specs/im-api-deployment.spec.json');
const topology = readJson('specs/topology.spec.json');
const deployment = readJson('etc/sdkwork.deployment.config.json');

const expectedUrls = {
  development: 'http://im-dev.sdkwork.com:3801/',
  test: 'https://im-test.sdkwork.com/',
  staging: 'https://im-staging.sdkwork.com/',
  production: 'https://im.sdkwork.com/',
};
const expectedCloudApiBaseUrls = {
  development: 'https://api-dev.sdkwork.com/',
  test: 'https://api-test.sdkwork.com/',
  staging: 'https://api-staging.sdkwork.com/',
  production: 'https://api.sdkwork.com/',
};

for (const [environment, expectedUrl] of Object.entries(expectedUrls)) {
  const canonicalEnvironment = deployment.environments?.[environment];
  assert.ok(canonicalEnvironment, `deployment config must declare ${environment}`);
  assert.equal(canonicalEnvironment.applicationOrigin, expectedUrl);

  const parsed = new URL(expectedUrl);
  assert.equal(parsed.pathname, '/', `${environment} must be served at the origin root`);
  assert.equal(parsed.search, '');
  assert.equal(parsed.hash, '');
  assert.doesNotMatch(parsed.hostname, /^api(?:-|\.)/u);
  assert.equal(
    canonicalEnvironment.cloudApiBaseUrl,
    expectedCloudApiBaseUrls[environment],
  );
}
assert.equal(authority.environments, undefined);
assert.equal(pc.environments, undefined);
assert.equal(h5.environments, undefined);

assert.equal(routing.authority.manifestPath, '../sdkwork.app.config.json');
assert.equal(
  routing.authority.deploymentConfigPath,
  '../etc/sdkwork.deployment.config.json',
);
assert.deepEqual(routing.environments, Object.keys(expectedUrls));
assert.equal(routing.authority.apiDeploymentSpecPath, 'im-api-deployment.spec.json');
assert.deepEqual(topology.vocabulary.environment.allowed, Object.keys(expectedUrls));
for (const deploymentProfile of ['cloud', 'standalone']) {
  for (const environment of Object.keys(expectedUrls)) {
    assert.equal(
      topology.profileFiles[`${deploymentProfile}.${environment}`],
      `etc/topology/${deploymentProfile}.${environment}.env`,
    );
    const profileSource = readText(`etc/topology/${deploymentProfile}.${environment}.env`);
    assert.match(
      profileSource,
      new RegExp(`SDKWORK_IM_DEPLOYMENT_PROFILE=${deploymentProfile}`, 'u'),
    );
    assert.match(profileSource, new RegExp(`SDKWORK_IM_ENVIRONMENT=${environment}`, 'u'));
    if (deploymentProfile === 'cloud') {
      const cloudApiBaseUrl = expectedCloudApiBaseUrls[environment].replace(/\/$/u, '');
      assert.ok(
        profileSource.includes(`SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=${cloudApiBaseUrl}`),
      );
      assert.ok(
        profileSource.includes(`VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=${cloudApiBaseUrl}`),
      );
    } else {
      if (environment === 'development') {
        assert.doesNotMatch(profileSource, /PLATFORM_API_GATEWAY_HTTP_URL/u);
        assert.match(profileSource, /SDKWORK_IM_WEB_DEV_INGRESS_BIND=0\.0\.0\.0:3801/u);
      }
    }
  }
}
assert.equal(apiDeployment.profiles.cloud.applicationAndApiOriginsAreDistinct, true);
assert.equal(apiDeployment.profiles.standalone.singleIngress, true);
assert.equal(apiDeployment.profiles.standalone.apiBaseUrlField, 'applicationOrigin');
assert.equal(
  apiDeployment.authority.deploymentConfigPath,
  '../etc/sdkwork.deployment.config.json',
);
const pcSdkBaseUrlSource = readText(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/sdkBaseUrls.ts',
);
assert.match(
  pcSdkBaseUrlSource,
  /resolveImApiBaseUrl[\s\S]*VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL[\s\S]*VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL/u,
);
const h5ImSdkSource = readText('apps/sdkwork-im-h5/src/bootstrap/environment.ts');
assert.match(
  h5ImSdkSource,
  /resolveH5RuntimeEnvironment[\s\S]*VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL[\s\S]*VITE_SDKWORK_IM_API_BASE_URL/u,
);
const pcDevSource = readText('scripts/lib/im-pc-dev.mjs');
assert.match(
  pcDevSource,
  /const standaloneSingleIngress = isStandaloneSingleIngress\(mergedEnv\)[\s\S]*createStandaloneGatewayProcess/u,
);
assert.deepEqual(routing.routing, {
  hostMatch: 'exact',
  pathPrefix: '/',
  selection: 'user-agent',
  defaultClient: 'pc',
  desktopClient: 'pc',
  mobileClient: 'h5',
  fallbackOrder: {
    desktop: ['pc', 'h5'],
    mobile: ['h5', 'pc'],
  },
});
const packageManifest = readJson('package.json');
assert.equal(
  packageManifest.scripts['_sdkwork:client:browser:standalone'],
  'node scripts/dev/run-sdkwork-im-adaptive-web-dev.mjs',
);
assert.equal(
  packageManifest.scripts['_sdkwork:client:browser:cloud'],
  'node scripts/dev/run-sdkwork-im-adaptive-web-dev.mjs',
);
const standaloneBrowser = topology.orchestration.profiles['standalone.development'].processes
  .find((process) => process.id === 'im-browser');
assert.deepEqual(standaloneBrowser.clientArchitectures, ['pc-web', 'h5']);
assert.equal(standaloneBrowser.bindEnv, 'SDKWORK_IM_WEB_DEV_INGRESS_BIND');
assert.equal(
  topology.orchestration.profiles['standalone.development'].processes
    .some((process) => process.id === 'im-h5'),
  false,
);
const adaptiveIngressSource = readText('scripts/dev/run-sdkwork-im-adaptive-web-dev.mjs');
assert.match(
  adaptiveIngressSource,
  /isCanonicalImApiPath[\s\S]*resolveAvailableImWebClient/u,
);
const pcViteSource = readText('apps/sdkwork-im-pc/vite.config.ts');
const h5ViteSource = readText('apps/sdkwork-im-h5/vite.config.ts');
assert.match(pcViteSource, /node_modules[\s\S]*\.vite[\s\S]*sdkwork-im-pc/u);
assert.match(h5ViteSource, /node_modules[\s\S]*\.vite[\s\S]*sdkwork-im-h5/u);
assert.match(
  pcViteSource,
  /optimizeDeps:\s*\{[\s\S]*include:\s*\[[\s\S]*['"]dompurify['"]/u,
  'PC Vite must prebundle dompurify before serving browser modules',
);

console.log('sdkwork-im web domain routing standard passed');
