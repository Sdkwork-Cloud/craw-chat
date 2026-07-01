import assert from 'node:assert/strict';
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');
const appRoot = path.join(repoRoot, 'apps', 'sdkwork-im-flutter-mobile');
const coreRoot = path.join(appRoot, 'packages', 'sdkwork_im_flutter_mobile_core');

function read(relativePath) {
  return readFileSync(path.join(appRoot, relativePath), 'utf8');
}

function listDartFiles(root) {
  if (!existsSync(root)) {
    return [];
  }
  const files = [];
  for (const entry of readdirSync(root)) {
    const absolute = path.join(root, entry);
    const stat = statSync(absolute);
    if (stat.isDirectory()) {
      if (['build', '.dart_tool', 'ios', 'android'].includes(entry)) {
        continue;
      }
      files.push(...listDartFiles(absolute));
      continue;
    }
    if (entry.endsWith('.dart')) {
      files.push(absolute);
    }
  }
  return files;
}

assert.ok(existsSync(appRoot), 'apps/sdkwork-im-flutter-mobile application root must exist');

for (const required of [
  '.sdkwork/README.md',
  'AGENTS.md',
  'sdkwork.app.config.json',
  'config/app/runtime-env.development.example.json',
  'config/host/flutter.development.example.json',
  'lib/main.dart',
  'lib/bootstrap/runtime.dart',
  'packages/sdkwork_im_flutter_mobile_core/lib/sdkwork_im_flutter_mobile_core.dart',
  'packages/sdkwork_im_flutter_mobile_chat/lib/sdkwork_im_flutter_mobile_chat.dart',
  'packages/sdkwork_im_flutter_mobile_chat/lib/src/pages/chat_conversation_page.dart',
  'packages/sdkwork_im_flutter_mobile_chat/lib/src/services/chat_conversation_service.dart',
  'packages/sdkwork_im_flutter_mobile_chat/lib/src/services/chat_realtime_service.dart',
  'specs/component.spec.json',
  'test/widget_test.dart',
]) {
  assert.ok(existsSync(path.join(appRoot, required)), `missing ${required}`);
}

const mainDart = read('lib/main.dart');
const appDart = read('lib/app.dart');
const runtime = read('lib/bootstrap/runtime.dart');

assert.match(mainDart, /bootstrap\(\)/u);
assert.match(appDart, /AuthGate/u);
assert.match(runtime, /createIamRuntime/u);
assert.match(runtime, /createSdkClients/u);

const packageDirs = readdirSync(path.join(appRoot, 'packages'));
for (const dir of packageDirs) {
  assert.match(dir, /^sdkwork_im_flutter_mobile_/u, `package must use sdkwork_im_flutter_mobile_* naming: ${dir}`);
}

const coreSource = listDartFiles(coreRoot).map((file) => readFileSync(file, 'utf8')).join('\n');
const driveClientSource = readFileSync(
  path.join(coreRoot, 'lib/src/drive/drive_app_sdk_client.dart'),
  'utf8',
);
assert.match(coreSource, /im_sdk_generated/u);
assert.match(driveClientSource, /package:http\/http\.dart/u, 'Drive presigned upload may use http for external storage boundary only.');
assert.equal(
  coreSource.replace(driveClientSource, '').includes('package:http/http.dart'),
  false,
);

const chatRoot = path.join(appRoot, 'packages', 'sdkwork_im_flutter_mobile_chat');
const chatSource = listDartFiles(chatRoot).map((file) => readFileSync(file, 'utf8')).join('\n');
assert.match(chatSource, /inboxRetrieve/u);
assert.match(chatSource, /conversationsMessagesList/u);
assert.match(chatSource, /conversationsMessagesCreate/u);
assert.match(chatSource, /ChatConversationPage/u);
assert.match(chatSource, /startInbox/u);
assert.match(chatSource, /startConversation/u);
assert.match(chatSource, /onScope/u);
assert.match(chatSource, /stopConversation/u);
assert.match(chatSource, /disposeChatRealtimeHub/u);
assert.match(chatSource, /_ChatLiveHub/u);
assert.match(coreSource, /im_sdk_composed/u);
assert.match(coreSource, /drive_app_sdk_client/u);
assert.match(
  readFileSync(path.join(chatRoot, 'lib/src/services/chat_media_upload_service.dart'), 'utf8'),
  /DriveAppSdkClient\.create/u,
);
const sdkClients = read('lib/bootstrap/sdk_clients.dart');
assert.match(sdkClients, /disposeChatRealtimeHub/u);
assert.match(sdkClients, /resetSdkClients/u);
assert.equal(chatSource.includes('package:http/http.dart'), false);

console.log('sdkwork-im Flutter mobile architecture standard passed.');
