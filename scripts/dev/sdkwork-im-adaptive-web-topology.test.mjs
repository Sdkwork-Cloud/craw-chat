import assert from 'node:assert/strict';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import {
  WEB_DEVICE_CLASSES,
  createTopologyRuntime,
  detectWebDeviceClass,
  loadTopologySpec,
  matchCanonicalApiPath,
  resolveAvailableWebClient,
  webClientFallbackOrder,
} from '@sdkwork/app-topology';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');

function browserPlan(profileId) {
  const specPath = path.join(repoRoot, 'specs', 'topology.spec.json');
  const runtime = createTopologyRuntime(loadTopologySpec(specPath), repoRoot, specPath);
  return runtime.resolvePlan(profileId, 'browser');
}

function adaptiveDelivery(plan) {
  const delivery = plan.browserDeliveries.find((entry) => entry.id === 'im-adaptive-web');
  assert.ok(delivery, 'browser plan must include the im-adaptive-web delivery');
  return delivery;
}

test('standalone.development resolves the adaptive browser delivery with both renderers', () => {
  const plan = browserPlan('standalone.development');
  const delivery = adaptiveDelivery(plan);
  assert.equal(delivery.deliveryMode, 'dev-server-proxy');
  assert.equal(delivery.adaptive, true);
  assert.equal(delivery.browserVisibleOrigin, 'http://127.0.0.1:3801');
  assert.equal(delivery.apiTargetOrigin, 'http://127.0.0.1:18089');
  assert.deepEqual(delivery.renderers.map((renderer) => ({
    architecture: renderer.architecture,
    port: renderer.port,
    host: renderer.host,
    label: renderer.label,
    defaultPort: renderer.defaultPort,
    portEnv: renderer.portEnv,
  })), [
    {
      architecture: 'pc-web',
      port: 4176,
      host: '127.0.0.1',
      label: 'sdkwork-im-pc',
      defaultPort: 4176,
      portEnv: 'SDKWORK_IM_PC_INTERNAL_DEV_PORT',
    },
    {
      architecture: 'h5',
      port: 4178,
      host: '127.0.0.1',
      label: 'sdkwork-im-h5',
      defaultPort: 4178,
      portEnv: 'SDKWORK_IM_H5_INTERNAL_DEV_PORT',
    },
  ]);
  const browserProcess = plan.localProcesses.find((process) => process.id === 'im-browser');
  assert.equal(browserProcess.bindEnv, 'SDKWORK_IM_WEB_DEV_INGRESS_BIND');
  assert.equal(plan.primaryAccessEndpoint.url, 'http://127.0.0.1:3801/');
});

test('cloud.development resolves the adaptive browser delivery against the deployed API origin', () => {
  const plan = browserPlan('cloud.development');
  const delivery = adaptiveDelivery(plan);
  assert.equal(delivery.adaptive, true);
  assert.equal(delivery.browserVisibleOrigin, 'http://127.0.0.1:3801');
  assert.equal(delivery.apiTargetOrigin, 'https://api-dev.sdkwork.com');
  assert.deepEqual(delivery.renderers.map((renderer) => renderer.architecture), ['pc-web', 'h5']);
});

test('device classification follows the shared adaptive contract (SDKWORK_DEPLOY_SPEC §8)', () => {
  assert.equal(
    detectWebDeviceClass({ userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/136.0' }),
    WEB_DEVICE_CLASSES.DESKTOP,
  );
  for (const mobileUserAgent of [
    'Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X) Mobile/15E148',
    'Mozilla/5.0 (Linux; Android 15; Pixel 9 Pro) Mobile Safari/537.36',
    'MicroMessenger/8.0.50(0x28003237)',
  ]) {
    assert.equal(detectWebDeviceClass({ userAgent: mobileUserAgent }), WEB_DEVICE_CLASSES.MOBILE);
  }
  assert.equal(
    detectWebDeviceClass({ userAgent: 'Mozilla/5.0 (iPad; CPU OS 18_0 like Mac OS X) Mobile/15E148' }),
    WEB_DEVICE_CLASSES.DESKTOP,
    'iPad defaults to desktop per the shared contract',
  );
  assert.equal(
    detectWebDeviceClass({
      userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/136.0',
      secChUaMobile: '?1',
    }),
    WEB_DEVICE_CLASSES.MOBILE,
  );
});

test('renderer selection falls back in both directions', () => {
  assert.deepEqual(webClientFallbackOrder(WEB_DEVICE_CLASSES.DESKTOP, ['pc-web', 'h5']), ['pc-web', 'h5']);
  assert.deepEqual(webClientFallbackOrder(WEB_DEVICE_CLASSES.MOBILE, ['pc-web', 'h5']), ['h5', 'pc-web']);
  assert.equal(
    resolveAvailableWebClient({
      deviceClass: WEB_DEVICE_CLASSES.MOBILE,
      availableClients: ['pc-web'],
      clientArchitectures: ['pc-web', 'h5'],
    }),
    'pc-web',
  );
});

test('canonical API paths bypass renderer routing', () => {
  for (const canonicalPath of [
    '/im/v3/api/realtime/ws',
    '/open/v1/api/users',
    '/healthz',
    '/openapi.json',
  ]) {
    assert.equal(matchCanonicalApiPath(canonicalPath), true, canonicalPath);
  }
  for (const rendererPath of ['/', '/workspace/inbox', '/chat/123']) {
    assert.equal(matchCanonicalApiPath(rendererPath), false, rendererPath);
  }
});
