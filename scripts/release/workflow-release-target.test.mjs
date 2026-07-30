import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import {
  artifactPathFor,
  createBuildPlan,
  packageReleaseTarget,
  runBuildPlan,
  validateReleaseTarget,
} from './workflow-release-target.mjs';

let tempRoot;

test.beforeEach(() => {
  tempRoot = mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-workflow-release-target-'));
});
test.afterEach(() => rmSync(tempRoot, { recursive: true, force: true }));

test('packages and validates the H5 build output deterministically', () => {
  const h5Dist = path.join(tempRoot, 'apps', 'sdkwork-im-h5', 'dist');
  mkdirSync(path.join(h5Dist, 'assets'), { recursive: true });
  writeFileSync(path.join(h5Dist, 'index.html'), '<!doctype html>');
  writeFileSync(path.join(h5Dist, 'assets', 'app.js'), 'console.log("im")');

  const result = packageReleaseTarget({
    packageId: 'h5-universal-cloud-mobile-zip',
    root: tempRoot,
    version: '0.1.0',
  });
  assert.equal(result.artifactPath, artifactPathFor('h5-universal-cloud-mobile-zip', '0.1.0', tempRoot));
  assert.equal(result.manifest.clientArchitecture, 'h5');
  assert.equal(result.manifest.targetPlatform, 'h5');
  assert.doesNotThrow(() => validateReleaseTarget({
    packageId: 'h5-universal-cloud-mobile-zip',
    root: tempRoot,
    version: '0.1.0',
  }));
});

test('copies a real Flutter AAB without changing its bytes', () => {
  const source = path.join(
    tempRoot,
    'apps',
    'sdkwork-im-flutter-mobile',
    'build',
    'app',
    'outputs',
    'bundle',
    'release',
    'app-release.aab',
  );
  mkdirSync(path.dirname(source), { recursive: true });
  const bytes = Buffer.from('PK\u0003\u0004real-aab-fixture');
  writeFileSync(source, bytes);

  const result = packageReleaseTarget({
    packageId: 'android-universal-cloud-mobile-aab',
    root: tempRoot,
    version: '0.1.0',
  });
  assert.equal(result.manifest.sizeBytes, bytes.length);
  assert.equal(result.manifest.runtimeTarget, 'flutter-android');
});

test('build plans bind H5 and Flutter to cloud.production source config', () => {
  const h5Plan = createBuildPlan({
    packageId: 'h5-universal-cloud-mobile-zip',
    root: tempRoot,
    version: '0.1.0',
  });
  assert.equal(h5Plan.steps[0].env.SDKWORK_IM_PROFILE_ID, 'cloud.production');
  assert.equal(h5Plan.steps[0].cwd, path.join(tempRoot, 'apps', 'sdkwork-im-h5'));

  const flutterPlan = createBuildPlan({
    packageId: 'android-universal-cloud-mobile-apk',
    root: tempRoot,
    version: '0.1.0',
  });
  assert.equal(flutterPlan.flutterConfig.SDKWORK_IM_PROFILE_ID, 'cloud.production');
  assert.equal(path.relative(tempRoot, flutterPlan.flutterConfigPath).startsWith('..'), true);
  assert.ok(flutterPlan.steps[0].args.includes('--dart-define-from-file'));
});

test('IPA planning fails closed away from a macOS/Xcode runner', () => {
  assert.throws(
    () => createBuildPlan({
      env: {},
      packageId: 'ios-universal-cloud-mobile-ipa',
      platform: 'win32',
      root: tempRoot,
      version: '0.1.0',
    }),
    /macOS\/Xcode runner/u,
  );
});

test('removes Flutter config and decoded Android signing material after build failure', () => {
  const plan = createBuildPlan({
    env: {
      SDKWORK_ANDROID_KEYSTORE_BASE64: Buffer.from('test-keystore').toString('base64'),
      SDKWORK_ANDROID_KEYSTORE_PASSWORD: 'keystore-password',
      SDKWORK_ANDROID_KEY_ALIAS: 'release',
      SDKWORK_ANDROID_KEY_PASSWORD: 'key-password',
    },
    packageId: 'android-universal-cloud-mobile-apk',
    root: tempRoot,
    version: '0.1.0',
  });
  let keystorePath;
  assert.throws(
    () => runBuildPlan(plan, {
      spawn: (_command, _args, options) => {
        keystorePath = options.env.ORG_GRADLE_PROJECT_SDKWORK_RELEASE_KEYSTORE_FILE;
        assert.equal(path.relative(tempRoot, keystorePath).startsWith('..'), true);
        assert.equal(path.relative(tempRoot, plan.flutterConfigPath).startsWith('..'), true);
        assert.equal(mkdirSync(path.dirname(plan.flutterConfigPath), { recursive: true }), undefined);
        assert.equal(require('node:fs').existsSync(plan.flutterConfigPath), true);
        assert.equal(require('node:fs').existsSync(keystorePath), true);
        return { status: 1 };
      },
    }),
    /failed with exit code 1/u,
  );
  assert.equal(require('node:fs').existsSync(plan.flutterConfigPath), false);
  assert.equal(require('node:fs').existsSync(keystorePath), false);
});
