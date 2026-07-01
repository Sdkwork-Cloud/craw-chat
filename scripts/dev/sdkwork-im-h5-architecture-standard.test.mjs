import assert from 'node:assert/strict';
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');
const appRoot = path.join(repoRoot, 'apps', 'sdkwork-im-h5');
const coreRoot = path.join(appRoot, 'packages', 'sdkwork-im-h5-core');

function read(relativePath) {
  return readFileSync(path.join(appRoot, relativePath), 'utf8');
}

function listFiles(root, extensions = ['.ts', '.tsx']) {
  if (!existsSync(root)) {
    return [];
  }

  const files = [];
  for (const entry of readdirSync(root)) {
    const absolute = path.join(root, entry);
    const stat = statSync(absolute);
    if (stat.isDirectory()) {
      if (['node_modules', 'dist', 'target', '__tests__'].includes(entry)) {
        continue;
      }
      files.push(...listFiles(absolute, extensions));
      continue;
    }

    if (extensions.includes(path.extname(entry))) {
      files.push(absolute);
    }
  }
  return files;
}

function readAll(root) {
  return listFiles(root)
    .map((file) => `\n// ${path.relative(appRoot, file)}\n${readFileSync(file, 'utf8')}`)
    .join('\n');
}

assert.ok(existsSync(appRoot), 'apps/sdkwork-im-h5 application root must exist');

for (const required of [
  'AGENTS.md',
  'sdkwork.app.config.json',
  'specs/component.spec.json',
  'src/ImApp.tsx',
  'src/bootstrap/runtime.ts',
  'packages/sdkwork-im-h5-chat/src/pages/ChatInboxPage.tsx',
  'packages/sdkwork-im-h5-chat/src/pages/ChatConversationPage.tsx',
  'packages/sdkwork-im-h5-chat/src/services/chatRealtimeService.ts',
]) {
  assert.ok(existsSync(path.join(appRoot, required)), `missing ${required}`);
}

const chatInbox = read('packages/sdkwork-im-h5-chat/src/pages/ChatInboxPage.tsx');

const imApp = read('src/ImApp.tsx');
const chatConversation = read('packages/sdkwork-im-h5-chat/src/pages/ChatConversationPage.tsx');
const chatConversationService = read('packages/sdkwork-im-h5-chat/src/services/chatConversationService.ts');
const chatRealtime = read('packages/sdkwork-im-h5-chat/src/services/chatRealtimeService.ts');

assert.match(imApp, /parseConversationRoute/u);
assert.match(imApp, /ChatConversationPage/u);
assert.match(chatConversationService, /listMessages/u);
assert.match(chatConversationService, /postText/u);
assert.match(
  read('packages/sdkwork-im-h5-core/src/sdk/driveAppSdkClient.ts'),
  /createDriveAppClient/u,
);
assert.match(
  read('packages/sdkwork-im-h5-chat/src/services/chatMediaUploadService.ts'),
  /getDriveAppSdkClientWithSession/u,
);
assert.match(chatConversation, /fetchConversationTimeline/u);
assert.match(chatConversation, /sendConversationText/u);
assert.match(chatConversation, /subscribeConversationLiveMessages/u);
assert.match(chatInbox, /subscribeInboxLiveRefresh/u);
assert.match(chatRealtime, /\.connect\(/u);
assert.match(chatRealtime, /messages\.onConversation/u);
assert.match(chatRealtime, /subscribeInboxLiveRefresh/u);
assert.match(chatRealtime, /events\.onScope/u);
assert.match(chatRealtime, /sharedConnection/u);
assert.match(chatRealtime, /state\.status === "open"[\s\S]*syncLiveSubscriptions/u);
assert.match(chatRealtime, /teardownConnectionIfIdle/u);
assert.match(chatRealtime, /disposeChatLiveConnection/u);

const app = read('src/App.tsx');
const runtime = read('src/bootstrap/runtime.ts');
assert.match(app, /HashRouter/u);
assert.match(app, /ImApp/u);
assert.match(app, /AppAuthGate/u);
assert.match(app, /IM_APP_HOME_PATH/u);
assert.match(runtime, /createIamRuntime/u);

const authRuntime = read('src/bootstrap/imAppAuthRuntime.ts');
const iamRuntime = read('src/bootstrap/iamRuntime.ts');
const appPackageJson = JSON.parse(readFileSync(path.join(appRoot, 'package.json'), 'utf8'));
const corePackageJson = JSON.parse(readFileSync(path.join(coreRoot, 'package.json'), 'utf8'));

assert.match(authRuntime, /platform:\s*"h5"/u);
assert.match(authRuntime, /createSdkworkAppbasePcAuthRuntime/u);
assert.match(authRuntime, /disposeChatLiveConnection/u);
assert.match(iamRuntime, /createImAppAuthRuntime/u);
assert.ok(appPackageJson.dependencies['@sdkwork/auth-runtime-pc-react']);
assert.ok(appPackageJson.dependencies['react-router-dom']);
assert.equal(corePackageJson.dependencies?.['@sdkwork/auth-runtime-pc-react'], undefined);

const authGate = read('src/AppAuthGate.tsx');
const authConfig = read('src/bootstrap/imAuthConfig.ts');
assert.match(authGate, /IM_H5_IAM_SESSION_CHANGED_EVENT/u);
assert.match(authGate, /SdkworkIamAuthRoutes/u);
assert.match(authGate, /viewportMode="flow"/u);
assert.match(authConfig, /resolveImAuthRuntimeConfig/u);
assert.ok(appPackageJson.dependencies['@sdkwork/auth-pc-react']);

const coreSource = readAll(coreRoot);
assert.equal(coreSource.includes('@sdkwork/auth-pc-react'), false);
assert.equal(coreSource.includes('@sdkwork/auth-runtime-pc-react'), false);

console.log('sdkwork-im H5 architecture standard passed.');
