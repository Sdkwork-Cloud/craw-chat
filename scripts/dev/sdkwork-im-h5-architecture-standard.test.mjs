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
const aiImagePageSource = read('packages/sdkwork-im-h5-ai-image/src/pages/AIImagePage.tsx');
const aiVideoPageSource = read('packages/sdkwork-im-h5-ai-video/src/pages/AIVideoPage.tsx');
const aiWritingPageSource = read('packages/sdkwork-im-h5-ai-writing/src/pages/AIWritingPage.tsx');
const aiMusicPageSource = read('packages/sdkwork-im-h5-ai-music/src/pages/AIMusicPage.tsx');
const aiVoiceSynthPageSource = read('packages/sdkwork-im-h5-ai-voice/src/pages/AIVoiceSynthPage.tsx');
const voiceSummaryPageSource = read('packages/sdkwork-im-h5-ai-voice/src/pages/VoiceSummaryApp.tsx');
const voiceSummarySource = read('packages/sdkwork-im-h5-ai-voice/src/services/VoiceSummaryService.ts');
const voiceCatalogSource = read('packages/sdkwork-im-h5-commons/src/services/VoiceService.ts');
const legacyApiClientSource = read('packages/sdkwork-im-h5-commons/src/ApiClient.ts');
const deferredCapabilityServices = [
  ['CalendarService', 'CalendarCapabilityUnavailableError', 'packages/sdkwork-im-h5-calendar/src/services/CalendarService.ts'],
  ['ApprovalService', 'ApprovalCapabilityUnavailableError', 'packages/sdkwork-im-h5-approval/src/services/ApprovalService.ts'],
  ['AttendanceService', 'AttendanceCapabilityUnavailableError', 'packages/sdkwork-im-h5-attendance/src/services/AttendanceService.ts'],
  ['ReportService', 'ReportCapabilityUnavailableError', 'packages/sdkwork-im-h5-report/src/services/ReportService.ts'],
  ['CloudDriveService', 'CloudDriveCapabilityUnavailableError', 'packages/sdkwork-im-h5-cloud-drive/src/services/CloudDriveService.ts'],
  ['MeetingService', 'MeetingCapabilityUnavailableError', 'packages/sdkwork-im-h5-meeting/src/services/MeetingService.ts'],
  ['ChannelService', 'ChannelCapabilityUnavailableError', 'packages/sdkwork-im-h5-channels/src/services/ChannelService.ts'],
  ['HardwareService', 'HardwareCapabilityUnavailableError', 'packages/sdkwork-im-h5-hardware/src/services/HardwareService.ts'],
  ['RecruitmentService', 'RecruitmentCapabilityUnavailableError', 'packages/sdkwork-im-h5-recruitment/src/services/RecruitmentService.ts'],
  ['KnowledgeBaseService', 'KnowledgeBaseCapabilityUnavailableError', 'packages/sdkwork-im-h5-knowledge/src/services/KnowledgeBaseService.ts'],
  ['ProductService', 'ShoppingCapabilityUnavailableError', 'packages/sdkwork-im-h5-shopping/src/services/ProductService.ts'],
  ['CartService', 'ShoppingCapabilityUnavailableError', 'packages/sdkwork-im-h5-shopping/src/services/CartService.ts'],
  ['OrderService', 'OrderCapabilityUnavailableError', 'packages/sdkwork-im-h5-orders/src/services/OrderService.ts'],
  ['CommunityService', 'CommunityCapabilityUnavailableError', 'packages/sdkwork-im-h5-community/src/services/CommunityService.ts'],
  ['CourseService', 'CourseCapabilityUnavailableError', 'packages/sdkwork-im-h5-course/src/services/CourseService.ts'],
  ['ProfileService', 'UserCapabilityUnavailableError', 'packages/sdkwork-im-h5-user/src/services/ProfileService.ts'],
  ['SettingsService', 'UserCapabilityUnavailableError', 'packages/sdkwork-im-h5-user/src/services/SettingsService.ts'],
  ['CharacterService', 'UserCapabilityUnavailableError', 'packages/sdkwork-im-h5-user/src/services/CharacterService.ts'],
  ['WorkService', 'UserCapabilityUnavailableError', 'packages/sdkwork-im-h5-user/src/services/WorkService.ts'],
  ['MomentService', 'UserCapabilityUnavailableError', 'packages/sdkwork-im-h5-user/src/services/MomentService.ts'],
  ['UserProductService', 'UserCapabilityUnavailableError', 'packages/sdkwork-im-h5-user/src/services/ProductService.ts'],
  ['LifeService', 'UserCapabilityUnavailableError', 'packages/sdkwork-im-h5-user/src/services/LifeService.ts'],
].map(([name, errorName, relativePath]) => [name, errorName, read(relativePath)]);
const deferredCapabilityPages = [
  'packages/sdkwork-im-h5-calendar/src/pages/CalendarWorkspace.tsx',
  'packages/sdkwork-im-h5-approval/src/pages/ApprovalApp.tsx',
  'packages/sdkwork-im-h5-approval/src/pages/ApprovalDetail.tsx',
  'packages/sdkwork-im-h5-approval/src/pages/CreateApproval.tsx',
  'packages/sdkwork-im-h5-attendance/src/pages/AttendanceApp.tsx',
  'packages/sdkwork-im-h5-report/src/pages/ReportApp.tsx',
  'packages/sdkwork-im-h5-report/src/pages/ReportDetail.tsx',
  'packages/sdkwork-im-h5-report/src/pages/CreateReport.tsx',
  'packages/sdkwork-im-h5-cloud-drive/src/pages/CloudDriveApp.tsx',
  'packages/sdkwork-im-h5-meeting/src/pages/MeetingApp.tsx',
  'packages/sdkwork-im-h5-meeting/src/pages/MeetingDetail.tsx',
  'packages/sdkwork-im-h5-meeting/src/pages/CreateMeeting.tsx',
  'packages/sdkwork-im-h5-channels/src/pages/ChannelsPage.tsx',
  'packages/sdkwork-im-h5-hardware/src/pages/HardwareList.tsx',
  'packages/sdkwork-im-h5-hardware/src/pages/HardwareDetail.tsx',
  'packages/sdkwork-im-h5-hardware/src/pages/HardwareBind.tsx',
  'packages/sdkwork-im-h5-recruitment/src/pages/RecruitmentApp.tsx',
  'packages/sdkwork-im-h5-recruitment/src/pages/CandidateDetail.tsx',
  'packages/sdkwork-im-h5-recruitment/src/pages/CreateJob.tsx',
  'packages/sdkwork-im-h5-knowledge/src/pages/KnowledgeBaseApp.tsx',
  'packages/sdkwork-im-h5-knowledge/src/pages/KnowledgeBaseDetail.tsx',
  'packages/sdkwork-im-h5-knowledge/src/pages/CreateKnowledgeBase.tsx',
  'packages/sdkwork-im-h5-knowledge/src/pages/KnowledgeBaseDocumentList.tsx',
  'packages/sdkwork-im-h5-knowledge/src/pages/CreateDocument.tsx',
  'packages/sdkwork-im-h5-shopping/src/pages/ShoppingCart.tsx',
  'packages/sdkwork-im-h5-shopping/src/pages/Shopping.tsx',
  'packages/sdkwork-im-h5-shopping/src/pages/ShopDetails.tsx',
  'packages/sdkwork-im-h5-shopping/src/pages/ProductDetails.tsx',
  'packages/sdkwork-im-h5-shopping/src/pages/CustomerServiceChat.tsx',
  'packages/sdkwork-im-h5-shopping/src/pages/CheckoutPage.tsx',
  'packages/sdkwork-im-h5-shopping/src/pages/CategoryPage.tsx',
  'packages/sdkwork-im-h5-shopping/src/pages/CashierPage.tsx',
  'packages/sdkwork-im-h5-orders/src/pages/VoucherCodePage.tsx',
  'packages/sdkwork-im-h5-orders/src/pages/OrderDetail.tsx',
  'packages/sdkwork-im-h5-orders/src/pages/OrderCenter.tsx',
  'packages/sdkwork-im-h5-community/src/pages/CommunityDetail.tsx',
  'packages/sdkwork-im-h5-community/src/pages/CommunityEditField.tsx',
  'packages/sdkwork-im-h5-community/src/pages/CommunityEditImage.tsx',
  'packages/sdkwork-im-h5-community/src/pages/CommunityEditTabs.tsx',
  'packages/sdkwork-im-h5-community/src/pages/CommunityGroupManagement.tsx',
  'packages/sdkwork-im-h5-community/src/pages/CommunityGroupQRs.tsx',
  'packages/sdkwork-im-h5-community/src/pages/CommunityList.tsx',
  'packages/sdkwork-im-h5-community/src/pages/CommunityMembers.tsx',
  'packages/sdkwork-im-h5-community/src/pages/CommunityPostCreate.tsx',
  'packages/sdkwork-im-h5-community/src/pages/CommunityProfile.tsx',
  'packages/sdkwork-im-h5-community/src/pages/CommunityQRCode.tsx',
  'packages/sdkwork-im-h5-community/src/pages/CreateCommunity.tsx',
  'packages/sdkwork-im-h5-community/src/pages/CreateCommunityGroup.tsx',
  'packages/sdkwork-im-h5-community/src/pages/MyCommunities.tsx',
  'packages/sdkwork-im-h5-course/src/pages/CourseDetail.tsx',
  'packages/sdkwork-im-h5-course/src/pages/CourseHome.tsx',
  'packages/sdkwork-im-h5-course/src/pages/CourseLiveRoom.tsx',
  'packages/sdkwork-im-h5-course/src/pages/CoursePlayer.tsx',
  'packages/sdkwork-im-h5-course/src/pages/CoursePurchase.tsx',
  'packages/sdkwork-im-h5-course/src/pages/MyCourses.tsx',
  'packages/sdkwork-im-h5-enterprise/src/pages/EnterpriseCenter.tsx',
  'packages/sdkwork-im-h5-enterprise/src/pages/EnterpriseInvite.tsx',
  'packages/sdkwork-im-h5-enterprise/src/pages/EnterpriseJoin.tsx',
  'packages/sdkwork-im-h5-enterprise/src/pages/EnterprisePostDemand.tsx',
  'packages/sdkwork-im-h5-enterprise/src/pages/EnterprisePostJob.tsx',
  'packages/sdkwork-im-h5-enterprise/src/pages/EnterprisePostSupply.tsx',
  'packages/sdkwork-im-h5-enterprise/src/pages/EnterpriseProducts.tsx',
  'packages/sdkwork-im-h5-enterprise/src/pages/EnterpriseRecruitment.tsx',
  'packages/sdkwork-im-h5-enterprise/src/pages/EnterpriseSearch.tsx',
  'packages/sdkwork-im-h5-enterprise/src/pages/EnterpriseSite.tsx',
  'packages/sdkwork-im-h5-enterprise/src/pages/EnterpriseYellowPages.tsx',
  'packages/sdkwork-im-h5-chat/src/pages/ChatDetail.tsx',
  'packages/sdkwork-im-h5-user/src/components/UnavailableUserPages.tsx',
].map(read);

assert.equal(appSource.includes('SPDX-License-Identifier'), false, 'AI Studio @license block must be removed from src/App.tsx');
assert.equal(appSource.includes('@license'), false, 'AI Studio @license block must be removed from src/App.tsx');
assert.equal(readmeSource.includes('AI Studio'), false, 'README.md must not reference AI Studio');
assert.equal(readmeSource.includes('ai.studio'), false, 'README.md must not reference ai.studio');
assert.equal(readmeSource.includes('GEMINI_API_KEY'), false, 'README.md must not reference GEMINI_API_KEY');
assert.equal(readmeSource.includes('ai.google.dev'), false, 'README.md must not reference ai.google.dev');
assert.equal(viteSource.includes('DISABLE_HMR'), false, 'AI Studio DISABLE_HMR env var must be removed from vite.config.ts');
assert.doesNotMatch(legacyApiClientSource, /fetch\s*\(/u, 'legacy ApiClient must not provide raw HTTP');
assert.match(legacyApiClientSource, /RawApiClientForbiddenError/u);
for (const [name, source] of [
  ['AIImageService', aiImageSource],
  ['AIVideoService', aiVideoSource],
  ['AIWritingService', aiWritingSource],
  ['VoiceSummaryService', voiceSummarySource],
  ['VoiceService', voiceCatalogSource],
]) {
  assert.doesNotMatch(source, /fetch\s*\(/u, `${name} must not use raw HTTP`);
  assert.doesNotMatch(source, /localStorage/u, `${name} must not persist local fake history`);
  assert.doesNotMatch(source, /Math\.random/u, `${name} must not generate random fake results`);
  assert.doesNotMatch(source, /setInterval|setTimeout/u, `${name} must not simulate work with timers`);
  assert.doesNotMatch(source, /\/mock\//u, `${name} must not return mock media`);
}
assert.match(aiImageSource, /AIImageCapabilityUnavailableError/u);
assert.match(aiVideoSource, /AIVideoCapabilityUnavailableError/u);
assert.match(aiWritingSource, /AIWritingCapabilityUnavailableError/u);
assert.match(voiceSummarySource, /VoiceSummaryCapabilityUnavailableError/u);
assert.match(voiceCatalogSource, /VoiceCapabilityUnavailableError/u);
for (const [name, errorName, source] of deferredCapabilityServices) {
  assert.doesNotMatch(source, /fetch\s*\(|localStorage|sessionStorage/u, `${name} must not own transport or browser business state`);
  assert.doesNotMatch(source, /Math\.random|Date\.now|setInterval|setTimeout|\/mock\//u, `${name} must not fabricate work or results`);
  assert.match(source, new RegExp(errorName, 'u'), `${name} must fail closed with a typed error`);
}
for (const source of [
  aiImagePageSource,
  aiVideoPageSource,
  aiWritingPageSource,
  aiMusicPageSource,
  aiVoiceSynthPageSource,
  voiceSummaryPageSource,
]) {
  assert.match(source, /CapabilityUnavailablePage/u);
  assert.doesNotMatch(source, /Math\.random|setInterval|setTimeout|\/mock\//u);
}
for (const source of deferredCapabilityPages) {
  assert.match(source, /CapabilityUnavailablePage/u);
}
const channelPageSource = read('packages/sdkwork-im-h5-channels/src/pages/ChannelsPage.tsx');
const channelServiceSource = read('packages/sdkwork-im-h5-channels/src/services/ChannelService.ts');
assert.doesNotMatch(channelPageSource, /mockData|CREATIVE_WORKS/u);
assert.doesNotMatch(channelServiceSource, /mockData|CREATIVE_WORKS/u);
const shoppingCartStoreSource = read('packages/sdkwork-im-h5-shopping/src/store/useCartStore.ts');
const shoppingAddressStoreSource = read('packages/sdkwork-im-h5-shopping/src/store/useAddressStore.ts');
assert.doesNotMatch(shoppingCartStoreSource, /localStorage|sessionStorage|persist\s*\(/u);
assert.doesNotMatch(shoppingCartStoreSource, /catch\s*\([^)]*\)\s*\{\s*\}/u);
assert.doesNotMatch(shoppingAddressStoreSource, /localStorage|sessionStorage|persist\s*\(/u);
assert.doesNotMatch(shoppingAddressStoreSource, /INITIAL_ADDRESSES|Date\.now|Math\.random/u);
const legacyChatActionPanelSource = read(
  'packages/sdkwork-im-h5-chat/src/components/Chat/ChatActionPanel.tsx',
);
assert.match(legacyChatActionPanelSource, /attachments_unavailable/u);
assert.doesNotMatch(
  legacyChatActionPanelSource,
  /Math\.random|Date\.now|setInterval|setTimeout|\/mock\//u,
);
const unavailableUserPageSource = read(
  'packages/sdkwork-im-h5-user/src/components/UnavailableUserPages.tsx',
);
assert.match(unavailableUserPageSource, /CapabilityUnavailablePage/u);
assert.doesNotMatch(
  unavailableUserPageSource,
  /localStorage|sessionStorage|Math\.random|Date\.now|setInterval|setTimeout|\/mock\//u,
);

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
for (const sourceFile of listFiles(appRoot)) {
  assert.doesNotMatch(
    readFileSync(sourceFile, 'utf8'),
    /\/mock\//u,
    `${path.relative(appRoot, sourceFile)} must not depend on mock media resources`,
  );
}

console.log('sdkwork-im H5 architecture standard passed.');
