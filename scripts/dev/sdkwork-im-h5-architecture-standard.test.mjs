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
      if (
        entry === 'node_modules'
        || entry.startsWith('node_modules.')
        || ['dist', 'target', '__tests__'].includes(entry)
      ) {
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
  'src/bootstrap/composition.ts',
  'src/bootstrap/runtime.ts',
  'src/bootstrap/sdkClients.ts',
  'src/bootstrap/iamRuntime.ts',
  'src/bootstrap/tokenManager.ts',
  'src/bootstrap/hostAdapters.ts',
  'src/bootstrap/routes.ts',
  'packages/sdkwork-im-h5-chat/src/pages/ChatList.tsx',
  'packages/sdkwork-im-h5-chat/src/pages/ChatDetail.tsx',
  'packages/sdkwork-im-h5-chat/src/services/chatRealtimeService.ts',
  'packages/sdkwork-im-h5-core/src/routes/routeRegistry.ts',
  'packages/sdkwork-im-h5-core/src/composition/sdk-registry.ts',
  'packages/sdkwork-im-h5-core/src/composition/module-registry.ts',
  'packages/sdkwork-im-h5-core/src/composition/host-registry.ts',
  'packages/sdkwork-im-h5-core/src/session/index.ts',
  'packages/sdkwork-im-h5-shell/package.json',
  'packages/sdkwork-im-h5-shell/specs/component.spec.json',
  'packages/sdkwork-im-h5-shell/src/ImH5Shell.tsx',
  'packages/sdkwork-im-h5-shell/src/moduleCatalog.ts',
  'packages/sdkwork-im-h5-shell/src/moduleNavigation.ts',
  'packages/sdkwork-im-h5-shell/src/moduleRegistry.ts',
  'packages/sdkwork-im-h5-shell/src/moduleValidation.ts',
  'packages/sdkwork-im-h5-shell/src/modules/chatModule.tsx',
  'packages/sdkwork-im-h5-shell/src/modules/contactsModule.tsx',
  'packages/sdkwork-im-h5-shell/src/modules/notaryModule.tsx',
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
const imAppSource = read('src/ImApp.tsx');
const shellSource = read('packages/sdkwork-im-h5-shell/src/ImH5Shell.tsx');
const shellModuleRegistrySource = read('packages/sdkwork-im-h5-shell/src/moduleRegistry.ts');
const shellModuleCatalogSource = read('packages/sdkwork-im-h5-shell/src/moduleCatalog.ts');
const shellRouteCatalogSource = read('packages/sdkwork-im-h5-shell/src/routeCatalog.ts');
const shellChatModuleSource = read('packages/sdkwork-im-h5-shell/src/modules/chatModule.tsx');
const shellContactsModuleSource = read('packages/sdkwork-im-h5-shell/src/modules/contactsModule.tsx');
const shellDriveModuleSource = read('packages/sdkwork-im-h5-shell/src/modules/driveModule.tsx');
const shellNotaryModuleSource = read('packages/sdkwork-im-h5-shell/src/modules/notaryModule.tsx');
const readmeSource = read('README.md');
const serverSource = read('server.ts');
const viteSource = read('vite.config.ts');
const aiImageSource = read('../../../sdkwork-image/apps/sdkwork-image-common/packages/sdkwork-image-mobile-react-generation/src/services/AIImageService.ts');
const aiVideoSource = read('../../../sdkwork-video/apps/sdkwork-video-common/packages/sdkwork-video-mobile-react-generation/src/services/AIVideoService.ts');
const aiWritingSource = read('packages/sdkwork-im-h5-ai-writing/src/services/AIWritingService.ts');
const aiImagePageSource = read('../../../sdkwork-image/apps/sdkwork-image-common/packages/sdkwork-image-mobile-react-generation/src/pages/AIImagePage.tsx');
const aiVideoPageSource = read('../../../sdkwork-video/apps/sdkwork-video-common/packages/sdkwork-video-mobile-react-generation/src/pages/AIVideoPage.tsx');
const aiWritingPageSource = read('packages/sdkwork-im-h5-ai-writing/src/pages/AIWritingPage.tsx');
const aiMusicPageSource = read('../../../sdkwork-music/apps/sdkwork-music-common/packages/sdkwork-music-mobile-react-generation/src/pages/AIMusicPage.tsx');
const aiVoiceSynthPageSource = read('../../../sdkwork-voice/apps/sdkwork-voice-common/packages/sdkwork-voice-mobile-react-generation/src/pages/AIVoiceSynthPage.tsx');
const voiceSummaryPageSource = read('../../../sdkwork-voice/apps/sdkwork-voice-common/packages/sdkwork-voice-mobile-react-generation/src/pages/VoiceSummaryApp.tsx');
const voiceSummarySource = read('../../../sdkwork-voice/apps/sdkwork-voice-common/packages/sdkwork-voice-mobile-react-generation/src/services/VoiceSummaryService.ts');
const legacyApiClientSource = read('packages/sdkwork-im-h5-commons/src/ApiClient.ts');
const cloudDriveServiceSource = read('../../../sdkwork-drive/apps/sdkwork-drive-common/packages/sdkwork-drive-mobile-react-drive/src/services/CloudDriveService.ts');
const cloudDrivePageSource = read('../../../sdkwork-drive/apps/sdkwork-drive-common/packages/sdkwork-drive-mobile-react-drive/src/pages/CloudDriveApp.tsx');
const deferredCapabilityServices = [
  ['CalendarService', 'CalendarCapabilityUnavailableError', 'packages/sdkwork-im-h5-calendar/src/services/CalendarService.ts'],
  ['ApprovalService', 'ApprovalCapabilityUnavailableError', 'packages/sdkwork-im-h5-approval/src/services/ApprovalService.ts'],
  ['AttendanceService', 'AttendanceCapabilityUnavailableError', 'packages/sdkwork-im-h5-attendance/src/services/AttendanceService.ts'],
  ['ReportService', 'ReportCapabilityUnavailableError', 'packages/sdkwork-im-h5-report/src/services/ReportService.ts'],
  ['MeetingService', 'MeetingCapabilityUnavailableError', '../../../sdkwork-rtc/apps/sdkwork-rtc-h5/packages/sdkwork-rtc-mobile-react-meeting/src/services/MeetingService.ts'],
  ['ChannelService', 'ChannelCapabilityUnavailableError', 'packages/sdkwork-im-h5-channels/src/services/ChannelService.ts'],
  ['HardwareService', 'HardwareCapabilityUnavailableError', '../../../sdkwork-aiot/apps/sdkwork-aiot-shared/packages/sdkwork-aiot-mobile-react-hardware/src/services/HardwareService.ts'],
  ['RecruitmentService', 'RecruitmentCapabilityUnavailableError', 'packages/sdkwork-im-h5-recruitment/src/services/RecruitmentService.ts'],
  ['KnowledgeBaseService', 'KnowledgeBaseCapabilityUnavailableError', '../../../sdkwork-knowledgebase/apps/sdkwork-knowledgebase-common/packages/sdkwork-knowledgebase-mobile-react-knowledge/src/services/KnowledgeBaseService.ts'],
  ['ProductService', 'ShoppingCapabilityUnavailableError', '../../../sdkwork-shop/apps/sdkwork-shop-common/packages/sdkwork-shop-mobile-react-shopping/src/services/ProductService.ts'],
  ['CartService', 'ShoppingCapabilityUnavailableError', '../../../sdkwork-shop/apps/sdkwork-shop-common/packages/sdkwork-shop-mobile-react-shopping/src/services/CartService.ts'],
  ['OrderService', 'OrderCapabilityUnavailableError', '../../../sdkwork-order/apps/sdkwork-order-common/packages/sdkwork-order-mobile-react-orders/src/services/OrderService.ts'],
  ['ProfileService', 'UserCapabilityUnavailableError', 'packages/sdkwork-im-h5-user/src/services/ProfileService.ts'],
  ['SettingsService', 'UserCapabilityUnavailableError', 'packages/sdkwork-im-h5-user/src/services/SettingsService.ts'],
  ['WorkService', 'UserCapabilityUnavailableError', 'packages/sdkwork-im-h5-user/src/services/WorkService.ts'],
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
  '../../../sdkwork-rtc/apps/sdkwork-rtc-h5/packages/sdkwork-rtc-mobile-react-meeting/src/pages/MeetingApp.tsx',
  '../../../sdkwork-rtc/apps/sdkwork-rtc-h5/packages/sdkwork-rtc-mobile-react-meeting/src/pages/MeetingDetail.tsx',
  '../../../sdkwork-rtc/apps/sdkwork-rtc-h5/packages/sdkwork-rtc-mobile-react-meeting/src/pages/CreateMeeting.tsx',
  'packages/sdkwork-im-h5-channels/src/pages/ChannelsPage.tsx',
  '../../../sdkwork-aiot/apps/sdkwork-aiot-shared/packages/sdkwork-aiot-mobile-react-hardware/src/pages/HardwareList.tsx',
  '../../../sdkwork-aiot/apps/sdkwork-aiot-shared/packages/sdkwork-aiot-mobile-react-hardware/src/pages/HardwareDetail.tsx',
  '../../../sdkwork-aiot/apps/sdkwork-aiot-shared/packages/sdkwork-aiot-mobile-react-hardware/src/pages/HardwareBind.tsx',
  'packages/sdkwork-im-h5-recruitment/src/pages/RecruitmentApp.tsx',
  'packages/sdkwork-im-h5-recruitment/src/pages/CandidateDetail.tsx',
  'packages/sdkwork-im-h5-recruitment/src/pages/CreateJob.tsx',
  '../../../sdkwork-knowledgebase/apps/sdkwork-knowledgebase-common/packages/sdkwork-knowledgebase-mobile-react-knowledge/src/pages/KnowledgeBaseApp.tsx',
  '../../../sdkwork-knowledgebase/apps/sdkwork-knowledgebase-common/packages/sdkwork-knowledgebase-mobile-react-knowledge/src/pages/KnowledgeBaseDetail.tsx',
  '../../../sdkwork-knowledgebase/apps/sdkwork-knowledgebase-common/packages/sdkwork-knowledgebase-mobile-react-knowledge/src/pages/CreateKnowledgeBase.tsx',
  '../../../sdkwork-knowledgebase/apps/sdkwork-knowledgebase-common/packages/sdkwork-knowledgebase-mobile-react-knowledge/src/pages/KnowledgeBaseDocumentList.tsx',
  '../../../sdkwork-knowledgebase/apps/sdkwork-knowledgebase-common/packages/sdkwork-knowledgebase-mobile-react-knowledge/src/pages/CreateDocument.tsx',
  '../../../sdkwork-shop/apps/sdkwork-shop-common/packages/sdkwork-shop-mobile-react-shopping/src/pages/ShoppingCart.tsx',
  '../../../sdkwork-shop/apps/sdkwork-shop-common/packages/sdkwork-shop-mobile-react-shopping/src/pages/Shopping.tsx',
  '../../../sdkwork-shop/apps/sdkwork-shop-common/packages/sdkwork-shop-mobile-react-shopping/src/pages/ShopDetails.tsx',
  '../../../sdkwork-shop/apps/sdkwork-shop-common/packages/sdkwork-shop-mobile-react-shopping/src/pages/ProductDetails.tsx',
  '../../../sdkwork-shop/apps/sdkwork-shop-common/packages/sdkwork-shop-mobile-react-shopping/src/pages/CustomerServiceChat.tsx',
  '../../../sdkwork-shop/apps/sdkwork-shop-common/packages/sdkwork-shop-mobile-react-shopping/src/pages/CheckoutPage.tsx',
  '../../../sdkwork-shop/apps/sdkwork-shop-common/packages/sdkwork-shop-mobile-react-shopping/src/pages/CategoryPage.tsx',
  '../../../sdkwork-shop/apps/sdkwork-shop-common/packages/sdkwork-shop-mobile-react-shopping/src/pages/CashierPage.tsx',
  '../../../sdkwork-order/apps/sdkwork-order-common/packages/sdkwork-order-mobile-react-orders/src/pages/VoucherCodePage.tsx',
  '../../../sdkwork-order/apps/sdkwork-order-common/packages/sdkwork-order-mobile-react-orders/src/pages/OrderDetail.tsx',
  '../../../sdkwork-order/apps/sdkwork-order-common/packages/sdkwork-order-mobile-react-orders/src/pages/OrderCenter.tsx',
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
].map(read);

assert.equal(appSource.includes('SPDX-License-Identifier'), false, 'AI Studio @license block must be removed from src/App.tsx');
assert.equal(appSource.includes('@license'), false, 'AI Studio @license block must be removed from src/App.tsx');
assert.equal(readmeSource.includes('AI Studio'), false, 'README.md must not reference AI Studio');
assert.equal(readmeSource.includes('ai.studio'), false, 'README.md must not reference ai.studio');
assert.equal(readmeSource.includes('GEMINI_API_KEY'), false, 'README.md must not reference GEMINI_API_KEY');
assert.equal(readmeSource.includes('ai.google.dev'), false, 'README.md must not reference ai.google.dev');
assert.equal(viteSource.includes('DISABLE_HMR'), false, 'AI Studio DISABLE_HMR env var must be removed from vite.config.ts');
assert.match(imAppSource, /@sdkwork\/im-h5-shell/u, 'root ImApp must remain a shell compatibility export');
assert.doesNotMatch(imAppSource, /<Routes|ChatConversationPage|WorkspaceNotary/u, 'root ImApp must not own route UI');
assert.match(shellNotaryModuleSource, /React\.lazy/u, 'H5 optional capability routes must use lazy loading');
assert.match(
  shellNotaryModuleSource,
  /import\("@sdkwork\/im-h5-notary"\)/u,
  'H5 notary routes must load their capability package on demand',
);
assert.match(shellSource, /resolveImH5ShellModules/u, 'H5 shell must resolve selected capability modules');
assert.match(shellSource, /resolveImH5ShellHomePath/u, 'H5 shell must derive fallback navigation from selected modules');
assert.match(shellSource, /module\.routes/u, 'H5 shell must assemble capability route contributions');
assert.match(shellModuleCatalogSource, /CONTRACT_PENDING_IM_H5_MODULES/u, 'unavailable modules must remain fail closed');
assert.match(shellModuleRegistrySource, /contacts:\s*contactsModule/u, 'contacts must be available as an optional built-in module');
assert.match(shellModuleRegistrySource, /drive:\s*driveModule/u, 'Drive must be available as an optional built-in module');
assert.match(shellContactsModuleSource, /React\.lazy/u, 'H5 contacts routes must use lazy loading');
assert.match(
  shellContactsModuleSource,
  /import\("@sdkwork\/im-h5-contacts"\)/u,
  'H5 contacts routes must load their capability package on demand',
);
assert.match(shellDriveModuleSource, /React\.lazy/u, 'H5 Drive routes must use lazy loading');
assert.match(
  shellDriveModuleSource,
  /import\("@sdkwork\/im-h5-cloud-drive"\)/u,
  'H5 Drive routes must load the owner compatibility package on demand',
);
assert.match(
  viteSource,
  /cacheDir:\s*path\.resolve\(__dirname, '\.vite'\)/u,
  'H5 Vite cache must be isolated under the application-owned .vite directory',
);
for (const requiredReactSingletonBinding of [
  "dedupe: ['react', 'react-dom']",
  "find: 'react/jsx-runtime'",
  "find: 'react/jsx-dev-runtime'",
  "find: 'react-dom/client'",
  'find: /^react-dom$/',
  'find: /^react$/',
]) {
  assert.ok(
    viteSource.includes(requiredReactSingletonBinding),
    `H5 Vite config must preserve the React singleton binding ${requiredReactSingletonBinding}`,
  );
}
assert.match(viteSource, /manualChunks/u, 'H5 release build must define stable vendor chunks');
for (const chunkName of [
  'react-vendor',
  'editor-vendor',
  'i18n-vendor',
  'ui-vendor',
  'sdk-vendor',
  'auth-vendor',
]) {
  assert.ok(viteSource.includes(chunkName), `H5 release build must expose a ${chunkName} chunk`);
}
assert.ok(
  viteSource.indexOf("find: /^@sdkwork\\/im-h5-([^/]+)\\/(.+)$/") <
    viteSource.indexOf("find: /^@sdkwork\\/im-h5-([^/]+)$/"),
  'H5 package subpath aliases must be matched before package-root aliases',
);
assert.doesNotMatch(serverSource, /import\.meta/u, 'H5 CommonJS server must not depend on import.meta');
assert.match(
  serverSource,
  /path\.resolve\(process\.cwd\(\), 'dist'\)/u,
  'H5 production server must resolve the built static root from the application working directory',
);
assert.doesNotMatch(legacyApiClientSource, /fetch\s*\(/u, 'legacy ApiClient must not provide raw HTTP');
assert.match(legacyApiClientSource, /RawApiClientForbiddenError/u);
for (const [name, source] of [
  ['AIImageService', aiImageSource],
  ['AIVideoService', aiVideoSource],
  ['AIWritingService', aiWritingSource],
  ['VoiceSummaryService', voiceSummarySource],
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
assert.match(cloudDriveServiceSource, /configureCloudDriveRuntime/u);
assert.match(cloudDriveServiceSource, /client\.drive\.nodes\.list/u);
assert.match(cloudDriveServiceSource, /client\.uploader\.upload/u);
assert.doesNotMatch(
  `${cloudDriveServiceSource}\n${cloudDrivePageSource}`,
  /fetch\s*\(|axios|localStorage|sessionStorage|Math\.random|\/mock\//u,
  'Cloud Drive must use the injected owner SDK without local transport or fabricated state',
);
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
// Community (圈子) migrated to sdkwork-community: pages are real implementations
// backed by the injected Community App SDK port, not fail-closed placeholders.
const communityListSource = read('../../../sdkwork-community/apps/sdkwork-community-common/packages/sdkwork-community-mobile-react-community/src/pages/CommunityList.tsx');
const communityServiceSource = read('../../../sdkwork-community/apps/sdkwork-community-common/packages/sdkwork-community-mobile-react-community/src/services/CommunityService.ts');
assert.doesNotMatch(communityListSource, /CapabilityUnavailablePage/u, 'community pages must be real implementations');
assert.match(communityServiceSource, /getCommunityRuntimePort/u, 'community service must consume the injected App SDK port');
// Moments (朋友圈) migrated to a real implementation: pages and services live
// in the IM-owned `sdkwork-im-h5-moments` feature package and consume the
// injected Community App SDK port. The legacy localStorage mock was removed.
const momentsServiceSource = read('packages/sdkwork-im-h5-moments/src/services/MomentService.ts');
const momentsPageSource = read('packages/sdkwork-im-h5-moments/src/pages/MomentsPage.tsx');
assert.match(momentsServiceSource, /getMomentsRuntimePort/u, 'moments service must consume the injected App SDK port');
assert.doesNotMatch(momentsServiceSource, /localStorage|sessionStorage|fetch\s*\(/u, 'moments must not own transport or browser business state');
assert.doesNotMatch(`${momentsServiceSource}\n${momentsPageSource}`, /picsum|localStorage|\/mock\//u, 'moments must not fabricate media or browser state');
// Course (课程) migrated to sdkwork-course: pages are real implementations
// backed by the injected Course App SDK port, not fail-closed placeholders.
const courseHomeSource = read('../../../sdkwork-course/apps/sdkwork-course-common/packages/sdkwork-course-mobile-react-courses/src/pages/CourseHome.tsx');
const courseDetailSource = read('../../../sdkwork-course/apps/sdkwork-course-common/packages/sdkwork-course-mobile-react-courses/src/pages/CourseDetail.tsx');
const coursePurchaseSource = read('../../../sdkwork-course/apps/sdkwork-course-common/packages/sdkwork-course-mobile-react-courses/src/pages/CoursePurchase.tsx');
const coursePlayerSource = read('../../../sdkwork-course/apps/sdkwork-course-common/packages/sdkwork-course-mobile-react-courses/src/pages/CoursePlayer.tsx');
const courseLiveRoomSource = read('../../../sdkwork-course/apps/sdkwork-course-common/packages/sdkwork-course-mobile-react-courses/src/pages/CourseLiveRoom.tsx');
const myCoursesSource = read('../../../sdkwork-course/apps/sdkwork-course-common/packages/sdkwork-course-mobile-react-courses/src/pages/MyCourses.tsx');
const courseServiceSource = read('../../../sdkwork-course/apps/sdkwork-course-common/packages/sdkwork-course-mobile-react-courses/src/services/CourseService.ts');
for (const coursePageSource of [
  courseHomeSource,
  courseDetailSource,
  coursePurchaseSource,
  coursePlayerSource,
  courseLiveRoomSource,
  myCoursesSource,
]) {
  assert.doesNotMatch(coursePageSource, /CapabilityUnavailablePage/u, 'course pages must be real implementations');
}
assert.match(courseServiceSource, /getCourseRuntimePort/u, 'course service must consume the injected App SDK port');
const channelPageSource = read('packages/sdkwork-im-h5-channels/src/pages/ChannelsPage.tsx');
const channelServiceSource = read('packages/sdkwork-im-h5-channels/src/services/ChannelService.ts');
assert.doesNotMatch(channelPageSource, /mockData|CREATIVE_WORKS/u);
assert.doesNotMatch(channelServiceSource, /mockData|CREATIVE_WORKS/u);
const shoppingCartStoreSource = read('../../../sdkwork-shop/apps/sdkwork-shop-common/packages/sdkwork-shop-mobile-react-shopping/src/store/useCartStore.ts');
const shoppingAddressStoreSource = read('../../../sdkwork-shop/apps/sdkwork-shop-common/packages/sdkwork-shop-mobile-react-shopping/src/store/useAddressStore.ts');
assert.doesNotMatch(shoppingCartStoreSource, /localStorage|sessionStorage|persist\s*\(/u);
assert.doesNotMatch(shoppingCartStoreSource, /catch\s*\([^)]*\)\s*\{\s*\}/u);
assert.doesNotMatch(shoppingAddressStoreSource, /localStorage|sessionStorage|persist\s*\(/u);
assert.doesNotMatch(shoppingAddressStoreSource, /INITIAL_ADDRESSES|Date\.now|Math\.random/u);
const chatActionPanelSource = read(
  'packages/sdkwork-im-h5-chat/src/components/Chat/ChatActionPanel.tsx',
);
assert.match(chatActionPanelSource, /onFileSelected/u);
assert.match(chatActionPanelSource, /selectFile/u);
assert.doesNotMatch(
  chatActionPanelSource,
  /Math\.random|Date\.now|setInterval|setTimeout|\/mock\//u,
);
const unavailableUserPageSource = read(
  'packages/sdkwork-im-h5-user/src/components/UnavailableUserPages.tsx',
);
assert.match(unavailableUserPageSource, /CapabilityUnavailablePage/u);

const chatInbox = read('packages/sdkwork-im-h5-chat/src/pages/ChatList.tsx');

const imApp = read('src/ImApp.tsx');
const chatConversation = read('packages/sdkwork-im-h5-chat/src/pages/ChatDetail.tsx');
const chatConversationService = read('packages/sdkwork-im-h5-chat/src/services/chatConversationService.ts');
const chatRealtime = read('packages/sdkwork-im-h5-core/src/realtime/index.ts');

assert.match(imApp, /parseConversationRoute/u);
assert.match(shellChatModuleSource, /ChatDetail/u);
assert.match(shellRouteCatalogSource, /app\.communication\.chat\.conversation/u);
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
assert.match(chatConversation, /getMessagePage/u);
assert.match(chatConversation, /sendMessage/u);
assert.match(chatConversation, /subscribeConversationLiveMessages/u);
assert.match(chatInbox, /subscribeInboxLiveRefresh/u);
assert.match(chatRealtime, /\.connect\(/u);
assert.match(chatRealtime, /messages\.onConversation/u);
assert.match(chatRealtime, /subscribeInboxLiveRefresh/u);
assert.match(chatRealtime, /events\.onScope/u);
assert.match(chatRealtime, /sharedConnection/u);
assert.match(chatRealtime, /state\.status === ['"]open['"][\s\S]*syncLiveSubscriptions/u);
assert.match(chatRealtime, /teardownConnectionIfIdle/u);
assert.match(chatRealtime, /disposeImLiveConnection/u);

const app = read('src/App.tsx');
const composition = read('src/bootstrap/composition.ts');
const runtime = read('src/bootstrap/runtime.ts');
const environment = read('src/bootstrap/environment.ts');
const sdkClients = read('src/bootstrap/sdkClients.ts');
const tokenManager = read('src/bootstrap/tokenManager.ts');
const hostAdapters = read('src/bootstrap/hostAdapters.ts');
const routesBootstrap = read('src/bootstrap/routes.ts');
assert.match(app, /HashRouter/u);
assert.match(app, /ImH5Shell/u);
assert.match(app, /resolveConfiguredImH5ModuleIds/u);
assert.match(app, /moduleIds=\{moduleIds\}/u);
assert.match(app, /AuthGate/u);
assert.match(app, /IM_APP_HOME_PATH/u);
assert.match(runtime, /createIamRuntime/u);
assert.match(environment, /resolveH5RuntimeEnvironment/u);
assert.match(environment, /deploymentProfile/u);
assert.match(composition, /VITE_SDKWORK_IM_H5_MODULES/u);
assert.match(composition, /DEFAULT_IM_H5_MODULES/u);
assert.match(composition, /COMPOSABLE_IM_H5_MODULES/u);
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

for (const packageName of readdirSync(path.join(appRoot, 'packages'))) {
  const packageRoot = path.join(appRoot, 'packages', packageName);
  if (!statSync(packageRoot).isDirectory() || !existsSync(path.join(packageRoot, 'package.json'))) {
    continue;
  }
  assert.ok(
    existsSync(path.join(packageRoot, 'specs', 'component.spec.json')),
    `${packageName} must own specs/component.spec.json`,
  );
  assert.ok(
    existsSync(path.join(packageRoot, 'specs', 'README.md')),
    `${packageName} must index its local specs`,
  );
  assert.ok(
    existsSync(path.join(packageRoot, 'README.md')),
    `${packageName} must document its reusable module contract`,
  );
}

for (const forbiddenBusinessRoot of ['src/components', 'src/pages']) {
  assert.deepEqual(
    listFiles(path.join(appRoot, forbiddenBusinessRoot)),
    [],
    `${forbiddenBusinessRoot} must not contain business UI in the thin application root`,
  );
}

for (const packageName of readdirSync(path.join(appRoot, 'packages'))) {
  if (packageName === 'sdkwork-im-h5-core') {
    continue;
  }
  const featureSource = readAll(path.join(appRoot, 'packages', packageName, 'src'));
  assert.doesNotMatch(
    featureSource,
    /from\s+['"]@sdkwork\/(?:[^'"]+-(?:app|backend)-sdk|im-sdk)['"]/u,
    `${packageName} must consume SDK access through @sdkwork/im-h5-core public exports`,
  );
}

const migratedCapabilityAdapters = [
  ['ai-image', '@sdkwork/image-mobile-react-generation'],
  ['ai-music', '@sdkwork/music-mobile-react-generation'],
  ['ai-video', '@sdkwork/video-mobile-react-generation'],
  ['ai-voice', '@sdkwork/voice-mobile-react-generation'],
  ['cloud-drive', '@sdkwork/drive-mobile-react-drive'],
  ['community', '@sdkwork/community-mobile-react-community'],
  ['course', '@sdkwork/course-mobile-react-courses'],
  ['hardware', '@sdkwork/aiot-mobile-react-hardware'],
  ['knowledge', '@sdkwork/knowledgebase-mobile-react-knowledge'],
  ['meeting', '@sdkwork/rtc-mobile-react-meeting'],
  ['notary', '@sdkwork/notary-h5-notary'],
  ['orders', '@sdkwork/order-mobile-react-orders'],
  ['shopping', '@sdkwork/shop-mobile-react-shopping'],
  ['vip', '@sdkwork/membership-mobile-react-subscription'],
];

for (const musicPlaybackSource of [
  '../../../sdkwork-music/apps/sdkwork-music-common/packages/sdkwork-music-mobile-react-playback/src/store/audioStore.ts',
  '../../../sdkwork-music/apps/sdkwork-music-common/packages/sdkwork-music-mobile-react-playback/src/pages/MusicPlayerPage.tsx',
  '../../../sdkwork-music/apps/sdkwork-music-common/packages/sdkwork-music-mobile-react-playback/src/components/GlobalMiniPlayer.tsx',
]) {
  assert.ok(
    existsSync(path.join(appRoot, musicPlaybackSource)),
    `music playback source must be owned by sdkwork-music: ${musicPlaybackSource}`,
  );
}
for (const retiredImMusicSource of [
  'src/pages/MusicPlayerPage.tsx',
  'src/components/player/GlobalMiniPlayer.tsx',
  'packages/sdkwork-im-h5-core/src/store/audioStore.ts',
  'packages/sdkwork-im-h5-core/src/sdk/notaryAppSdkClient.ts',
]) {
  assert.equal(
    existsSync(path.join(appRoot, retiredImMusicSource)),
    false,
    `${retiredImMusicSource} must not remain in the IM application boundary`,
  );
}

for (const [capability, canonicalPackage] of migratedCapabilityAdapters) {
  const packageRoot = path.join(appRoot, 'packages', `sdkwork-im-h5-${capability}`);
  const sourceEntries = readdirSync(path.join(packageRoot, 'src')).sort();
  const packageJson = JSON.parse(readFileSync(path.join(packageRoot, 'package.json'), 'utf8'));
  // The adapter surface stays a thin compatibility layer; the ai-voice host
  // wiring additionally owns a `runtime/` directory that injects the
  // voice-owned my-voices ports with host-constructed SDK clients.
  assert.deepEqual(
    sourceEntries,
    capability === 'ai-voice' ? ['index.ts', 'runtime'] : ['index.ts'],
    `sdkwork-im-h5-${capability} must remain a thin compatibility adapter`,
  );
  assert.ok(
    packageJson.dependencies?.[canonicalPackage],
    `sdkwork-im-h5-${capability} must depend on ${canonicalPackage}`,
  );
}

assert.match(authRuntime, /platform:\s*"h5"/u);
assert.match(authRuntime, /createSdkworkAppbasePcAuthRuntime/u);
assert.match(authRuntime, /notifyImH5SessionChanged/u);
assert.doesNotMatch(authRuntime, /@sdkwork\/im-h5-chat|disposeChatLiveConnection/u);
assert.match(iamRuntime, /createImAppAuthRuntime/u);
assert.ok(appPackageJson.dependencies['@sdkwork/auth-runtime-pc-react']);
assert.ok(appPackageJson.dependencies['@sdkwork/im-h5-shell']);
assert.ok(appPackageJson.dependencies['react-router-dom']);
assert.equal(appPackageJson.dependencies?.['@sdkwork/iam-backend-sdk'], undefined);
assert.equal(appPackageJson.dependencies?.['@sdkwork/im-backend-sdk'], undefined);
assert.equal(corePackageJson.dependencies?.['@sdkwork/auth-runtime-pc-react'], undefined);

const authGate = read('src/AuthGate.tsx');
const authConfig = read('src/bootstrap/imAuthConfig.ts');
assert.match(authGate, /IM_H5_IAM_SESSION_CHANGED_EVENT/u);
assert.match(authGate, /SdkworkIamH5AuthRoutes/u);
assert.match(authGate, /basePath=\{AUTH_BASE_PATH\}/u);
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
