import assert from 'node:assert/strict';
import path from 'node:path';
import test from 'node:test';

import { createFlutterDefineConfig, createFlutterDevPlan } from './flutter-dev.mjs';

const cloudEnv = {
  SDKWORK_IM_DEPLOYMENT_PROFILE: 'cloud',
  SDKWORK_IM_ENVIRONMENT: 'development',
  SDKWORK_IM_PROFILE_ID: 'cloud.development',
  SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: 'https://api-dev.sdkwork.com',
  SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: 'wss://api-dev.sdkwork.com',
  SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: 'https://api-dev.sdkwork.com',
};

test('materializes topology values without token or secret fields', () => {
  const config = createFlutterDefineConfig(cloudEnv);
  assert.equal(config.SDKWORK_IM_DEPLOYMENT_PROFILE, 'cloud');
  assert.equal(config.SDKWORK_IAM_APP_API_BASE_URL, 'https://api-dev.sdkwork.com');
  assert.equal(config.SDKWORK_ACCESS_TOKEN, undefined);
  assert.ok(Object.keys(config).every((key) => !/(?:PASSWORD|SECRET|PRIVATE_KEY)$/u.test(key)));
});

test('creates a profile-scoped local dart-define plan', () => {
  const root = path.resolve('fixture-flutter-root');
  const plan = createFlutterDevPlan({
    args: ['--target', 'android'],
    env: { ...cloudEnv, SDKWORK_FLUTTER_DEVICE_ID: 'emulator-5554' },
    root,
  });
  assert.equal(plan.target, 'android');
  assert.equal(
    plan.configPath,
    path.join(root, '.runtime', 'sdkwork-app', 'flutter', 'cloud.development.android.json'),
  );
  assert.deepEqual(plan.flutterArgs, [
    'run',
    '--device-id',
    'emulator-5554',
    '--dart-define-from-file',
    plan.configPath,
  ]);
});

test('fails before Flutter startup when topology URLs are missing', () => {
  assert.throws(
    () => createFlutterDefineConfig({
      SDKWORK_IM_DEPLOYMENT_PROFILE: 'cloud',
      SDKWORK_IM_ENVIRONMENT: 'development',
      SDKWORK_IM_PROFILE_ID: 'cloud.development',
    }),
    /SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL/u,
  );
});

test('rejects unsafe device identifiers before invoking the Windows Flutter shell wrapper', () => {
  assert.throws(
    () => createFlutterDevPlan({
      args: ['--target', 'android'],
      env: { ...cloudEnv, SDKWORK_FLUTTER_DEVICE_ID: 'emulator-5554 & whoami' },
    }),
    /unsupported characters/u,
  );
});
