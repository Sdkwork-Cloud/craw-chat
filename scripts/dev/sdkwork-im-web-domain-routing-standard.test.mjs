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
  demo: 'https://im-demo.sdkwork.com/',
};
// Web ingress routing stays on the four canonical lifecycle environments; the
// demo environment is deployment/topology-only (no im-web-ingress entry).
const webIngressEnvironments = ['development', 'test', 'staging', 'production'];
const multiBrandBaseDomains = [
  'sdkwork.com',
  'birdcoder.com',
  'dtupay.com',
  'sdkwork.cn',
  'birdcoder.cn',
  'dtupay.cn',
  'skubc.com',
  'skubc.cn',
  'zowalk.com',
  'zowalk.cn',
  'offer86.com',
  'offer86.cn',
  '86offer.com',
  '86offer.cn',
];
const cloudApiBaseUrlList = (role, environmentSuffix) =>
  multiBrandBaseDomains.map((baseDomain) => `https://${role}${environmentSuffix}.${baseDomain}`).join(';');
const expectedCloudApiBaseUrls = {
  development: cloudApiBaseUrlList('api', '-dev'),
  test: cloudApiBaseUrlList('api', '-test'),
  staging: cloudApiBaseUrlList('api', '-staging'),
  production: cloudApiBaseUrlList('api', ''),
  demo: cloudApiBaseUrlList('api', '-demo'),
};
// Topology env files pin the primary brand origin for the server-side gateway
// variable; the full multi-brand edge set lives in the deployment config and is
// materialized into the browser runtime env at build time.
const expectedPrimaryGatewayOrigins = {
  development: 'https://api-dev.sdkwork.com',
  test: 'https://api-test.sdkwork.com',
  staging: 'https://api-staging.sdkwork.com',
  production: 'https://api.sdkwork.com',
  demo: 'https://api-demo.sdkwork.com',
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
assert.deepEqual(routing.environments, webIngressEnvironments);
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
      const appPublicUrl = expectedUrls[environment].replace(/\/$/u, '');
      // VITE build values carry no dev-port: the :3801 binding is a
      // dev-process/dev-server concern (SDKWORK_IM_WEB_DEV_INGRESS_BIND) and
      // must not leak into VITE build values.
      const viteAppPublicUrl = appPublicUrl.replace(
        /^http:\/\/im-dev\.sdkwork\.com:3801$/u,
        'http://im-dev.sdkwork.com',
      );
      // Cloud realtime WebSocket rides the platform api edge — the same base
      // domain as the SDK base URL — because the application im-* edge only
      // proxies /api/ and serves static files, so a WS handshake there fails
      // with an unexpected HTTP 200 response.
      const cloudWebSocketUrl = expectedPrimaryGatewayOrigins[environment]
        .replace(/^https:/u, 'wss:');
      assert.ok(
        profileSource.includes(`SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=${appPublicUrl}`),
        `cloud env application public http URL must be ${appPublicUrl}`,
      );
      assert.ok(
        profileSource.includes(`VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=${viteAppPublicUrl}`),
        `cloud env VITE application public http URL must be ${viteAppPublicUrl}`,
      );
      assert.ok(
        profileSource.includes(`SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=${cloudWebSocketUrl}`),
        `cloud env application public websocket URL must be ${cloudWebSocketUrl}`,
      );
      assert.ok(
        profileSource.includes(`VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=${cloudWebSocketUrl}`),
        `cloud env VITE application public websocket URL must be ${cloudWebSocketUrl}`,
      );
      const appHttpLine = profileSource.match(/SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=(.+)/u)?.[1] ?? '';
      assert.doesNotMatch(
        appHttpLine,
        /^https?:\/\/api(?:-|\.)/u,
        'cloud env application public URL must not use the api-* platform host',
      );
      const websocketLine = profileSource.match(
        /VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=(.+)/u,
      )?.[1] ?? '';
      assert.doesNotMatch(
        websocketLine,
        /^wss?:\/\/im(?:-|\.)/u,
        'cloud env websocket base must ride the api-* platform edge, not the im-* application web ingress edge',
      );
      const primaryGatewayOrigin = expectedPrimaryGatewayOrigins[environment];
      assert.ok(
        profileSource.includes(`SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=${primaryGatewayOrigin}`),
      );
      assert.ok(
        profileSource.includes(`VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=${primaryGatewayOrigin}`),
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
assert.equal(packageManifest.scripts['_sdkwork:client:browser:standalone'], undefined);
assert.equal(packageManifest.scripts['_sdkwork:client:browser:cloud'], undefined);
const standaloneBrowser = topology.orchestration.profiles['standalone.development'].processes
  .find((process) => process.id === 'im-browser');
assert.deepEqual(standaloneBrowser.clientArchitectures, ['pc-web', 'h5']);
assert.equal(standaloneBrowser.bindEnv, 'SDKWORK_IM_WEB_DEV_INGRESS_BIND');
assert.equal(standaloneBrowser.script, undefined);
assert.equal(
  topology.orchestration.profiles['standalone.development'].processes
    .some((process) => process.id === 'im-h5'),
  false,
);
const adaptiveDelivery = topology.orchestration.profiles['standalone.development']
  .browserDeliveries.find((delivery) => delivery.id === 'im-adaptive-web');
assert.equal(adaptiveDelivery.deliveryMode, 'dev-server-proxy');
assert.equal(adaptiveDelivery.preserveCanonicalPaths, true);
assert.equal(adaptiveDelivery.clientProcessId, 'im-browser');
assert.deepEqual(Object.keys(adaptiveDelivery.renderers).sort(), ['h5', 'pc-web']);
for (const [architecture, renderer] of Object.entries(adaptiveDelivery.renderers)) {
  assert.equal(renderer.command, 'node');
  assert.ok(
    renderer.args.some((arg) => arg.includes('run-vite-cli.mjs')),
    `${architecture} renderer must launch through the shared vite runner`,
  );
  assert.ok(renderer.defaultPort > 0, `${architecture} renderer needs defaultPort`);
  assert.match(
    renderer.portEnv,
    /^SDKWORK_IM_[A-Z0-9_]+_INTERNAL_DEV_PORT$/u,
    `${architecture} renderer needs a canonical portEnv`,
  );
  assert.equal(renderer.env.VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL, '{httpOrigin}');
}
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
