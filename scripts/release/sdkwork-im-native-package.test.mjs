#!/usr/bin/env node

// Contract tests for the Sdkwork IM native install package pipeline:
//   plan-sdkwork-im-native-install-packages.mjs  (native package matrix)
//   build-sdkwork-im-native-installer.mjs        (deb/msi/pkg builders)
//   validate-sdkwork-im-install-artifacts.mjs    (byte-level .deb gate)
//
// Run: node --test scripts/release/sdkwork-im-native-package.test.mjs

import assert from 'node:assert/strict';
import { test } from 'node:test';
import { gunzipSync } from 'node:zlib';

import {
  createSdkworkImNativeInstallPackagePlan,
  validateSdkworkImNativeInstallPackagePlan,
} from './plan-sdkwork-im-native-install-packages.mjs';
import {
  createDebianPackage,
  createNativeInstallLayout,
  createSystemdUnit,
} from './build-sdkwork-im-native-installer.mjs';
import { readArArchive, readTarEntries, validateDebArtifact } from './validate-sdkwork-im-install-artifacts.mjs';

const VERSION = '0.1.0';

test('native package plan covers the canonical installer matrix', () => {
  const plan = createSdkworkImNativeInstallPackagePlan({ version: VERSION });
  const issues = validateSdkworkImNativeInstallPackagePlan(plan);
  assert.deepEqual(issues, [], 'native plan must validate');
  assert.equal(plan.deploymentProfiles.join(','), 'standalone');

  const serverIds = plan.packages.filter((item) => item.profile === 'server').map((item) => item.id);
  for (const expected of [
    'linux-ubuntu-x64-standalone-server-deb',
    'linux-ubuntu-arm64-standalone-server-deb',
    'windows-x64-standalone-server-msi',
    'windows-arm64-standalone-server-msi',
    'macos-x64-standalone-server-pkg',
    'macos-arm64-standalone-server-pkg',
  ]) {
    assert.ok(serverIds.includes(expected), `server matrix must include ${expected}`);
  }
  // Linux native items carry the distribution segment (GITHUB_WORKFLOW_SPEC §5).
  assert.ok(plan.packages.every((item) => item.platform !== 'linux' || item.distribution === 'ubuntu'));
  // Desktop formats are platform-native (no cross-platform Tauri formats).
  const desktopByPlatform = {};
  for (const item of plan.packages.filter((p) => p.profile === 'desktop')) {
    desktopByPlatform[item.platform] ??= new Set();
    desktopByPlatform[item.platform].add(item.format);
  }
  assert.deepEqual([...desktopByPlatform.linux].sort(), ['appimage', 'deb']);
  assert.deepEqual([...desktopByPlatform.windows].sort(), ['exe', 'msi']);
  assert.deepEqual([...desktopByPlatform.macos].sort(), ['dmg']);
});

test('installer names follow the canonical artifact grammar', () => {
  const plan = createSdkworkImNativeInstallPackagePlan({ version: VERSION });
  for (const item of plan.packages) {
    assert.match(
      item.installerName,
      /^sdkwork-im-[a-z0-9-]+-0\.1\.0\.[A-Za-z0-9.]+$/u,
      `${item.id} installerName must be canonical`,
    );
    if (item.profile === 'server') {
      assert.ok(item.stagingPackageId, `${item.id} must declare staging inputs`);
      assert.ok(item.serviceName, `${item.id} must declare the service name`);
    }
  }
});

test('deb builder emits a dpkg-parseable archive with canonical paths', () => {
  const plan = createSdkworkImNativeInstallPackagePlan({ version: VERSION });
  const packageItem = plan.packages.find((item) => item.id === 'linux-ubuntu-x64-standalone-server-deb');
  assert.ok(packageItem);

  const entries = [
    { relativePath: 'bin/sdkwork-api-im-standalone-gateway', data: Buffer.from('ELF'), mode: 0o755 },
    { relativePath: 'config/config.toml.example', data: Buffer.from('template'), mode: 0o644 },
    { relativePath: 'web/sdkwork-im-pc/dist/index.html', data: Buffer.from('<html/>'), mode: 0o644 },
    { relativePath: 'modules/sdkwork-iam/database/database.manifest.json', data: Buffer.from('{}'), mode: 0o644 },
    { relativePath: 'sdkwork.app.config.json', data: Buffer.from('{}'), mode: 0o644 },
    { relativePath: 'INSTALL.md', data: Buffer.from('# guide'), mode: 0o644 },
    { relativePath: 'install-manifest.json', data: Buffer.from('{}'), mode: 0o644 },
    { relativePath: 'service/linux/sdkwork-im.service', data: Buffer.from(createSystemdUnit()), mode: 0o644 },
  ];
  const debBytes = createDebianPackage({ package: packageItem }, entries);

  const members = readArArchive(debBytes);
  assert.ok(members.has('debian-binary'));
  assert.equal(members.get('debian-binary').data.toString('utf8').trim(), '2.0');
  const controlEntries = readTarEntries(gunzipSync(members.get('control.tar.gz').data));
  for (const required of ['./control', './postinst', './prerm']) {
    assert.ok(controlEntries.has(required), `control must include ${required}`);
  }
  assert.equal(controlEntries.get('./postinst').mode, 0o755);
  const controlText = controlEntries.get('./control').data.toString('utf8');
  assert.match(controlText, /^Package: sdkwork-chat$/mu);
  assert.match(controlText, /Depends: .*libssl3/mu);
  assert.match(controlText, /^Architecture: amd64$/mu);

  const dataEntries = readTarEntries(gunzipSync(members.get('data.tar.gz').data));
  for (const required of [
    './usr/lib/sdkwork/im/bin/sdkwork-api-im-standalone-gateway',
    './etc/sdkwork/im/config.toml.example',
    './usr/share/sdkwork/im/web/sdkwork-im-pc/dist/index.html',
    './var/lib/sdkwork/im/modules/sdkwork-iam/database/database.manifest.json',
    './usr/lib/sdkwork/im/sdkwork.app.config.json',
    './usr/share/doc/sdkwork/im/INSTALL.md',
    './usr/share/sdkwork/im/install-manifest.json',
    './usr/lib/systemd/system/sdkwork-im.service',
  ]) {
    assert.ok(dataEntries.has(required), `data.tar.gz must include ${required}`);
  }
  assert.equal(dataEntries.get('./usr/lib/sdkwork/im/bin/sdkwork-api-im-standalone-gateway').mode, 0o755);
  assert.equal(dataEntries.get('./etc/sdkwork/im/config.toml.example').mode, 0o640);
  assert.equal(dataEntries.get('./var/lib/sdkwork/im/modules/sdkwork-iam/database/database.manifest.json').mode, 0o644);
});

test('byte-level deb validation gate accepts the built deb and rejects garbage', () => {
  const plan = createSdkworkImNativeInstallPackagePlan({ version: VERSION });
  const packageItem = plan.packages.find((item) => item.id === 'linux-ubuntu-x64-standalone-server-deb');
  const entries = [
    { relativePath: 'bin/sdkwork-api-im-standalone-gateway', data: Buffer.from('ELF'), mode: 0o755 },
    { relativePath: 'config/config.toml.example', data: Buffer.from('t'), mode: 0o644 },
    { relativePath: 'config/server.env.example', data: Buffer.from('t'), mode: 0o644 },
    { relativePath: 'config/postgresql.yaml.example', data: Buffer.from('t'), mode: 0o644 },
    { relativePath: 'web/sdkwork-im-pc/dist/index.html', data: Buffer.from('<html/>'), mode: 0o644 },
    { relativePath: 'modules/sdkwork-iam/database/database.manifest.json', data: Buffer.from('{}'), mode: 0o644 },
    { relativePath: 'sdkwork.app.config.json', data: Buffer.from('{}'), mode: 0o644 },
    { relativePath: 'INSTALL.md', data: Buffer.from('# guide'), mode: 0o644 },
    {
      relativePath: 'install-manifest.json',
      data: Buffer.from(JSON.stringify({
        package: { id: 'linux-ubuntu-x64-standalone-server-deb', version: VERSION },
        nativeInstall: { schemaVersion: '2026-08-08.sdkwork-im.native-install-layout.v1' },
      })),
      mode: 0o644,
    },
    { relativePath: 'service/linux/sdkwork-im.service', data: Buffer.from(createSystemdUnit()), mode: 0o644 },
    { relativePath: 'database/database.manifest.json', data: Buffer.from('{}'), mode: 0o644 },
  ];
  const debBytes = createDebianPackage({ package: packageItem }, entries);
  assert.deepEqual(validateDebArtifact(packageItem, debBytes), [], 'built deb must pass the release gate');

  assert.ok(
    validateDebArtifact(packageItem, Buffer.from('not an ar archive')).length > 0,
    'garbage must fail the release gate',
  );
});

test('native install layouts carry the canonical spec paths per platform', () => {
  const plan = createSdkworkImNativeInstallPackagePlan({ version: VERSION });
  for (const item of plan.packages.filter((p) => p.profile === 'server')) {
    const layout = createNativeInstallLayout(item);
    assert.equal(layout.packageId, item.id);
    assert.equal(layout.schemaVersion, '2026-08-08.sdkwork-im.native-install-layout.v1');
    if (item.platform === 'linux') {
      assert.equal(layout.files.binary, '/usr/lib/sdkwork/im/bin/sdkwork-api-im-standalone-gateway');
      assert.equal(layout.service.unitPath, '/usr/lib/systemd/system/sdkwork-im.service');
      assert.equal(layout.files.passwordFile, '/etc/sdkwork/database/database.secret');
      assert.equal(layout.files.modules, '/var/lib/sdkwork/im/modules');
    } else if (item.platform === 'windows') {
      assert.equal(layout.service.name, 'sdkwork-im');
      assert.match(layout.installRoot, /%ProgramFiles%[\\/]sdkwork[\\/]chat/u);
      assert.match(layout.files.dataDirectory, /%ProgramData%[\\/]sdkwork[\\/]chat[\\/]Data/u);
    } else {
      assert.equal(layout.service.manager, 'launchd');
      assert.match(layout.installRoot, /usr\/lib\/sdkwork\/chat/u);
    }
  }
});
