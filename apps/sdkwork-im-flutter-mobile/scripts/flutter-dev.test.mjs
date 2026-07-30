import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import {
  createFlutterDefineConfig,
  createFlutterDevPlan,
  runFlutterDevelopment,
} from './flutter-dev.mjs';

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
  assert.equal(config.SDKWORK_PROFILE_ID, 'cloud.development');
  assert.equal(config.SDKWORK_RUNTIME_TARGET, 'flutter-android');
  assert.equal(config.SDKWORK_IAM_APP_API_BASE_URL, 'https://api-dev.sdkwork.com');
  assert.equal(config.SDKWORK_ACCESS_TOKEN, undefined);
  assert.ok(Object.keys(config).every((key) => !/(?:PASSWORD|SECRET|PRIVATE_KEY)$/u.test(key)));
});

test('creates a profile-scoped local dart-define plan', () => {
  const temporaryDirectory = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-flutter-state-'));
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-flutter-app-'));
  const plan = createFlutterDevPlan({
    args: ['--target', 'android'],
    env: { ...cloudEnv, SDKWORK_FLUTTER_DEVICE_ID: 'emulator-5554' },
    repoRoot: root,
    root,
    runtimeStateOptions: { env: {}, temporaryDirectory },
  });
  assert.equal(plan.target, 'android');
  assert.equal(plan.config.SDKWORK_RUNTIME_TARGET, 'flutter-android');
  assert.equal(path.relative(root, plan.configPath).startsWith('..'), true);
  assert.match(path.basename(plan.configPath), /^cloud\.development\.android\.dart-define-.*\.json$/u);
  assert.deepEqual(plan.flutterArgs, [
    'run',
    '--device-id',
    'emulator-5554',
    '--dart-define-from-file',
    plan.configPath,
  ]);
  fs.rmSync(root, { force: true, recursive: true });
  fs.rmSync(temporaryDirectory, { force: true, recursive: true });
});

test('removes the private dart-define file after Flutter success and failure', () => {
  for (const status of [0, 1]) {
    const temporaryDirectory = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-flutter-state-'));
    const root = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-flutter-app-'));
    const plan = createFlutterDevPlan({
      args: ['--target', 'android'],
      env: cloudEnv,
      repoRoot: root,
      root,
      runtimeStateOptions: { env: {}, temporaryDirectory },
    });
    const execute = () => runFlutterDevelopment(plan, {
      spawn: () => {
        assert.equal(fs.existsSync(plan.configPath), true);
        return { status };
      },
    });
    if (status === 0) execute();
    else assert.throws(execute, /flutter run exited/u);
    assert.equal(fs.existsSync(plan.configPath), false);
    fs.rmSync(root, { force: true, recursive: true });
    fs.rmSync(temporaryDirectory, { force: true, recursive: true });
  }
});

test('projects an iOS runtime target into the selected profile', () => {
  const plan = createFlutterDevPlan({
    args: ['--target', 'ios'],
    env: cloudEnv,
  });
  assert.equal(plan.config.SDKWORK_RUNTIME_TARGET, 'flutter-ios');
  assert.equal(plan.config.SDKWORK_IM_RUNTIME_TARGET, 'flutter-ios');
});

test('rejects profile identity drift before writing dart-define JSON', () => {
  assert.throws(
    () => createFlutterDefineConfig({
      ...cloudEnv,
      SDKWORK_IM_PROFILE_ID: 'cloud.production',
    }),
    /must match deployment profile and environment/u,
  );
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
