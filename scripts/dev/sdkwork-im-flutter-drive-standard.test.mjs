#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const appRoot = path.join(repoRoot, 'apps', 'sdkwork-im-flutter-mobile');
const coreRoot = path.join(appRoot, 'packages', 'sdkwork_im_flutter_mobile_core');
const chatRoot = path.join(appRoot, 'packages', 'sdkwork_im_flutter_mobile_chat');

function read(relativePath) {
  return fs.readFileSync(path.join(appRoot, relativePath), 'utf8');
}

function readCore(relativePath) {
  return fs.readFileSync(path.join(coreRoot, relativePath), 'utf8');
}

function readChat(relativePath) {
  return fs.readFileSync(path.join(chatRoot, relativePath), 'utf8');
}

const corePubspec = readCore('pubspec.yaml');
const driveClientSource = readCore('lib/src/drive/drive_app_sdk_client.dart');
const imSdkClientSource = readCore('lib/src/im_sdk_client.dart');
const sdkClientsBootstrap = read('lib/bootstrap/sdk_clients.dart');
const chatUploadSource = readChat('lib/src/services/chat_media_upload_service.dart');
const componentSpec = JSON.parse(
  fs.readFileSync(path.join(repoRoot, 'specs', 'component.spec.json'), 'utf8'),
);

assert.match(
  corePubspec,
  /sdkwork_common_flutter/u,
  'Flutter core must depend on sdkwork_common_flutter for Drive HTTP integration.',
);

assert.match(
  driveClientSource,
  /sdkwork_common_flutter/u,
  'Flutter drive client must use sdkwork_common_flutter HttpClient instead of raw auth headers.',
);

assert.match(
  driveClientSource,
  /drive\/uploader\/uploads/u,
  'Flutter drive client must upload through sdkwork-drive uploader API.',
);

assert.doesNotMatch(
  driveClientSource,
  /Authorization|Access-Token/u,
  'Flutter drive client must not assemble manual credential headers.',
);

assert.doesNotMatch(
  driveClientSource,
  /x-sdkwork-tenant-id|x-sdkwork-organization-id|x-sdkwork-user-id|x-sdkwork-actor-id/u,
  'Flutter drive client must not assemble manual AppContext scope headers; tokens resolve scope server-side.',
);

assert.doesNotMatch(
  imSdkClientSource,
  /x-sdkwork-tenant-id|x-sdkwork-organization-id|x-sdkwork-user-id|x-sdkwork-actor-id/u,
  'Flutter IM SDK client must not assemble manual AppContext scope headers; tokens resolve scope server-side.',
);

assert.doesNotMatch(
  sdkClientsBootstrap,
  /createImSdkClient\([\s\S]*tenantId:/u,
  'Flutter SDK bootstrap must not pass tenant scope into createImSdkClient.',
);

assert.match(
  chatUploadSource,
  /DriveAppSdkClient\.create/u,
  'Flutter chat media upload service must route uploads through the drive app SDK client.',
);

assert.doesNotMatch(
  chatUploadSource,
  /sdkwork_im_flutter_mobile_core\/src\//u,
  'Flutter chat media upload must consume sdkwork_im_flutter_mobile_core public exports, not src implementation imports.',
);

assert.match(
  chatUploadSource,
  /_chatDriveAppResourceType = 'im_conversation'/u,
  'Flutter chat media upload must bind files to im_conversation per im-app-api-sdk-integration.spec.md.',
);

assert.match(
  chatUploadSource,
  /_chatDriveScene = 'im'/u,
  'Flutter chat media upload must use scene=im per im-app-api-sdk-integration.spec.md.',
);

assert.match(
  chatUploadSource,
  /_chatDriveSource = 'chat_message'/u,
  'Flutter chat media upload must tag source=chat_message per im-app-api-sdk-integration.spec.md.',
);

const dependencySurface = componentSpec.contracts?.dependencyApiSurfaces?.find(
  (surface) => surface.apiAuthority === 'sdkwork-drive-app-api',
);
assert.ok(
  dependencySurface,
  'component.spec.json must declare sdkwork-drive-app-api dependency surface.',
);

assert.match(
  read('packages/sdkwork_im_flutter_mobile_core/lib/sdkwork_im_flutter_mobile_core.dart'),
  /drive_app_sdk_client/u,
  'Flutter core barrel must export the drive app SDK client.',
);

process.stdout.write('sdkwork-im Flutter drive standard passed\n');
