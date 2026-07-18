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
      assert.ok(
        profileSource.includes(
          'SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=http://127.0.0.1:18079',
        ),
      );
      assert.ok(
        profileSource.includes(
          'VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=http://127.0.0.1:18079',
        ),
      );
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
const h5ImSdkSource = readText(
  'apps/sdkwork-im-h5/packages/sdkwork-im-h5-core/src/sdk/imSdkClient.ts',
);
assert.match(
  h5ImSdkSource,
  /resolveImSdkApiBaseUrl[\s\S]*VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL[\s\S]*VITE_SDKWORK_IM_H5_APPLICATION_PUBLIC_HTTP_URL/u,
);
const pcDevSource = readText('scripts/lib/im-pc-dev.mjs');
assert.match(
  pcDevSource,
  /createManagedSdkworkApiGatewayProcess[\s\S]*isStandaloneSingleIngress\(env\)/u,
);
assert.deepEqual(routing.routing, {
  hostMatch: 'exact',
  pathPrefix: '/',
  selection: 'user-agent',
  defaultClient: 'pc',
  desktopClient: 'pc',
  mobileClient: 'h5',
});

console.log('sdkwork-im web domain routing standard passed');
