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
  'src/App.tsx',
  'src/AuthGate.tsx',
  'src/ImApp.tsx',
  'src/main.tsx',
  'src/index.css',
  'src/bootstrap/environment.ts',
  'src/bootstrap/runtime.ts',
  'src/bootstrap/sdkClients.ts',
  'src/bootstrap/iamRuntime.ts',
  'src/bootstrap/tokenManager.ts',
  'src/bootstrap/hostAdapters.ts',
  'src/bootstrap/routes.ts',
  'packages/sdkwork-im-h5-chat/src/pages/ChatInboxPage.tsx',
  'packages/sdkwork-im-h5-chat/src/pages/ChatConversationPage.tsx',
  'packages/sdkwork-im-h5-chat/src/services/chatRealtimeService.ts',
  'config/browser/runtime-env.development.example.json',
  'config/browser/runtime-env.test.example.json',
  'config/browser/runtime-env.staging.example.json',
  'config/browser/runtime-env.production.example.json',
  'config/host/capacitor.development.example.json',
  'config/host/capacitor.staging.example.json',
  'config/host/capacitor.production.example.json',
  'config/host/capacitor.test.example.json',
]) {
  assert.ok(existsSync(path.join(appRoot, required)), `missing ${required}`);
}

for (const requiredDir of [
  'bin',
  'config/browser',
  'config/host',
  'config/server',
  'config/container',
  'docs',
  'public',
  'scripts',
  'sdks',
  'tests',
  'src/providers',
  'src/shell',
  'src/routes',
]) {
  assert.ok(
    existsSync(path.join(appRoot, requiredDir)),
    `missing standard directory ${requiredDir}`,
  );
}

for (const forbidden of [
  'src/AppAuthGate.tsx',
  'src/AuthGuard.tsx',
  'auto-i18n.js',
  'bun.lock',
  'check-all-zh.cjs',
  'check-zh.cjs',
  'fix-functions.cjs',
  'fix-nested-2.cjs',
  'gen_voice_comps.sh',
  'hooks_usage.txt',
  'loc.txt',
  'metadata.json',
  'sort_loc.cjs',
  'sort_loc.js',
  'transform_i18n.cjs',
  'translate.cjs',
  'translate-user-pages.cjs',
  'update-user-settings.cjs',
]) {
  assert.equal(
    existsSync(path.join(appRoot, forbidden)),
    false,
    `non-standard ${forbidden} must not exist`,
  );
}

// AI Studio legacy content must not remain in source files.
const appSource = read('src/App.tsx');
const readmeSource = read('README.md');
const viteSource = read('vite.config.ts');
const aiImageSource = read('packages/sdkwork-im-h5-ai-image/src/services/AIImageService.ts');
const aiVideoSource = read('packages/sdkwork-im-h5-ai-video/src/services/AIVideoService.ts');
const aiWritingSource = read('packages/sdkwork-im-h5-ai-writing/src/services/AIWritingService.ts');

assert.equal(appSource.includes('SPDX-License-Identifier'), false, 'AI Studio @license block must be removed from src/App.tsx');
assert.equal(appSource.includes('@license'), false, 'AI Studio @license block must be removed from src/App.tsx');
assert.equal(readmeSource.includes('AI Studio'), false, 'README.md must not reference AI Studio');
assert.equal(readmeSource.includes('ai.studio'), false, 'README.md must not reference ai.studio');
assert.equal(readmeSource.includes('GEMINI_API_KEY'), false, 'README.md must not reference GEMINI_API_KEY');
assert.equal(readmeSource.includes('ai.google.dev'), false, 'README.md must not reference ai.google.dev');
assert.equal(viteSource.includes('DISABLE_HMR'), false, 'AI Studio DISABLE_HMR env var must be removed from vite.config.ts');
assert.equal(aiImageSource.includes('"/api/ai/optimize-prompt"'), false, 'AI Studio mock API /api/ai/optimize-prompt must be /im/v3/api/ai/optimize-prompt');
assert.equal(aiImageSource.includes('"/api/ai/image"'), false, 'AI Studio mock API /api/ai/image must be /im/v3/api/ai/image');
assert.equal(aiVideoSource.includes('"/api/ai/video"'), false, 'AI Studio mock API /api/ai/video must be /im/v3/api/ai/video');
assert.equal(aiWritingSource.includes('"/api/ai/writing"'), false, 'AI Studio mock API /api/ai/writing must be /im/v3/api/ai/writing');
assert.match(aiImageSource, /\/im\/v3\/api\/ai\/optimize-prompt/u, 'AIImageService must use /im/v3/api/ai/optimize-prompt');
assert.match(aiImageSource, /\/im\/v3\/api\/ai\/image/u, 'AIImageService must use /im/v3/api/ai/image');
assert.match(aiVideoSource, /\/im\/v3\/api\/ai\/video/u, 'AIVideoService must use /im/v3/api/ai/video');
assert.match(aiWritingSource, /\/im\/v3\/api\/ai\/writing/u, 'AIWritingService must use /im/v3/api/ai/writing');

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
assert.match(chatConversation, /fetchConversationMessages/u);
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
const environment = read('src/bootstrap/environment.ts');
const sdkClients = read('src/bootstrap/sdkClients.ts');
const tokenManager = read('src/bootstrap/tokenManager.ts');
const hostAdapters = read('src/bootstrap/hostAdapters.ts');
const routesBootstrap = read('src/bootstrap/routes.ts');
assert.match(app, /HashRouter/u);
assert.match(app, /ImApp/u);
assert.match(app, /AuthGate/u);
assert.match(app, /IM_APP_HOME_PATH/u);
assert.match(runtime, /createIamRuntime/u);
assert.match(environment, /resolveH5RuntimeEnvironment/u);
assert.match(environment, /deploymentProfile/u);
assert.match(sdkClients, /initSdkClients/u);
assert.match(sdkClients, /getDriveAppSdkClientFromBootstrap/u);
assert.match(tokenManager, /resolveTokenManagerBinding/u);
assert.match(tokenManager, /isTokenManagerBound/u);
assert.match(hostAdapters, /registerHostAdapter/u);
assert.match(hostAdapters, /getHostAdapter/u);
assert.match(routesBootstrap, /registerRoute/u);
assert.match(routesBootstrap, /listRoutes/u);

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

const authGate = read('src/AuthGate.tsx');
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
