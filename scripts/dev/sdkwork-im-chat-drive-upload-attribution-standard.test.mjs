#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

const surfaces = [
  {
    id: 'pc',
    path: 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService.ts',
    patterns: [
      /const CHAT_DRIVE_APP_RESOURCE_TYPE = ['"]im_conversation['"]/u,
      /const CHAT_DRIVE_SCENE = ['"]im['"]/u,
      /const CHAT_DRIVE_SOURCE = ['"]chat_message['"]/u,
    ],
  },
  {
    id: 'h5',
    path: 'apps/sdkwork-im-h5/packages/sdkwork-im-h5-chat/src/services/chatMediaUploadService.ts',
    patterns: [
      /const CHAT_DRIVE_APP_RESOURCE_TYPE = "im_conversation"/u,
      /const CHAT_DRIVE_SCENE = "im"/u,
      /const CHAT_DRIVE_SOURCE = "chat_message"/u,
    ],
  },
  {
    id: 'flutter',
    path: 'apps/sdkwork-im-flutter-mobile/packages/sdkwork_im_flutter_mobile_chat/lib/src/services/chat_media_upload_service.dart',
    patterns: [
      /_chatDriveAppResourceType = 'im_conversation'/u,
      /_chatDriveScene = 'im'/u,
      /_chatDriveSource = 'chat_message'/u,
    ],
  },
];

const integrationSpec = fs.readFileSync(
  path.join(repoRoot, 'specs', 'im-app-api-sdk-integration.spec.md'),
  'utf8',
);

assert.match(
  integrationSpec,
  /\|\s*`appResourceType`\s*\|\s*`im_conversation`\s*\|/u,
  'im-app-api-sdk-integration.spec.md must remain the canonical Drive upload attribution authority.',
);

assert.match(
  integrationSpec,
  /\|\s*`scene`\s*\|\s*`im`\s*\|/u,
  'im-app-api-sdk-integration.spec.md must declare scene=im for chat Drive uploads.',
);

assert.match(
  integrationSpec,
  /\|\s*`source`\s*\|\s*`chat_message`\s*\|/u,
  'im-app-api-sdk-integration.spec.md must declare source=chat_message for chat Drive uploads.',
);

for (const surface of surfaces) {
  const source = fs.readFileSync(path.join(repoRoot, surface.path), 'utf8');
  for (const pattern of surface.patterns) {
    assert.match(
      source,
      pattern,
      `${surface.id} chat media upload must follow canonical Drive attribution from im-app-api-sdk-integration.spec.md.`,
    );
  }
}

process.stdout.write('sdkwork-im chat drive upload attribution standard passed\n');
