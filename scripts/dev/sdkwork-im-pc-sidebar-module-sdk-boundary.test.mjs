import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function read(relativePath) {
  return readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const mailServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-mail/src/services/MailService.ts',
);
const ordersServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-orders/src/services/OrdersService.ts',
);
const shopServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-shop/src/services/ShopService.ts',
);
const communityServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-community/src/services/CommunityService.ts',
);
const calendarServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-calendar/src/services/CalendarService.ts',
);
const courseServiceSource = read(
  '../sdkwork-course/apps/sdkwork-course-pc/packages/sdkwork-course-pc-course/src/services/CourseService.ts',
);
const communityViewSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-community/src/components/CommunityView.tsx',
);
const communitySettingsSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-community/src/components/CommunitySettings.tsx',
);
const shopHomeSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-shop/src/components/ShopHome.tsx',
);
const videoPlayerViewSource = read(
  '../sdkwork-course/apps/sdkwork-course-pc/packages/sdkwork-course-pc-course/src/components/VideoPlayerView.tsx',
);
const liveRoomViewSource = read(
  '../sdkwork-course/apps/sdkwork-course-pc/packages/sdkwork-course-pc-course/src/components/LiveRoomView.tsx',
);
const checkoutViewSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-shop/src/components/CheckoutView.tsx',
);
const cashierViewSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-shop/src/components/CashierView.tsx',
);
const videoGenServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-video-gen/src/services/VideoGenService.ts',
);
const imageGenServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-image-gen/src/services/ImageGenService.ts',
);
const voiceGenServiceSource = read(
  '../sdkwork-voice/apps/sdkwork-voice-pc/packages/sdkwork-voice-pc-speech/src/services/voiceSpeechService.ts',
);
const musicGenServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-music-gen/src/services/MusicGenService.ts',
);
const writingServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-writing/src/services/WritingService.ts',
);
const approvalsServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-approvals/src/services/ApprovalsService.ts',
);
const attendanceServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-attendance/src/services/AttendanceService.ts',
);
const reportServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-reports/src/services/ReportService.ts',
);
const enterpriseMarketplaceServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-enterprise/src/services/EnterpriseMarketplaceService.ts',
);
const sessionSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/session.ts',
);

const forbiddenMockPattern =
  /mock|MockMailService|MockOrdersService|MockShopService|MockCommunityService|setTimeout|new Promise\s*\(|\bunsplash\b|\bpravatar\b|Date\.now\s*\(\s*\)|Math\.random\s*\(|\bfetch\s*\(|\b(Authorization|Access-Token|X-API-Key)\b/u;

for (const [label, source] of [
  ['pc mail service', mailServiceSource],
  ['pc orders service', ordersServiceSource],
  ['pc shop service', shopServiceSource],
  ['pc community service', communityServiceSource],
  ['pc calendar service', calendarServiceSource],
  ['pc course service', courseServiceSource],
  ['pc approvals service', approvalsServiceSource],
  ['pc attendance service', attendanceServiceSource],
  ['pc reports service', reportServiceSource],
  ['pc enterprise marketplace service', enterpriseMarketplaceServiceSource],
]) {
  assert.doesNotMatch(
    source,
    forbiddenMockPattern,
    `${label} must not keep local stand-ins, artificial delays, demo media, raw HTTP, or manual auth header logic.`,
  );
}

assert.match(
  mailServiceSource,
  /getMailAppSdkClientWithSession/u,
  'pc mail service must consume the generated mail app SDK wrapper.',
);
assert.match(
  ordersServiceSource,
  /getOrderAppSdkClientWithSession/u,
  'pc orders service must consume the generated order app SDK wrapper.',
);
assert.match(
  ordersServiceSource,
  /pc orders write contract requires order command headers/u,
  'pc orders write mutations must fail closed until order command headers are wired through the SDK.',
);
assert.match(
  shopServiceSource,
  /getCatalogAppSdkClientWithSession[\s\S]*getOrderAppSdkClientWithSession/u,
  'pc shop service must consume the generated catalog and order app SDK wrappers.',
);
assert.match(
  shopServiceSource,
  /pc shop favorites contract is not available/u,
  'pc shop favorites must fail closed until the commerce favorites contract exists.',
);
assert.match(
  shopServiceSource,
  /pc shop shipping address contract is not available/u,
  'pc shop shipping address mutations must fail closed until the commerce address contract exists.',
);
assert.match(
  shopServiceSource,
  /pc shop payment contract is not available/u,
  'pc shop payment mutations must fail closed until the commerce payment contract exists.',
);
assert.match(
  communityServiceSource,
  /getCommunityAppSdkClientWithSession/u,
  'pc community service must consume the generated community app SDK wrapper.',
);
assert.match(
  communityServiceSource,
  /pc community groups contract is not available/u,
  'pc community group mutations must fail closed until the community groups contract exists.',
);
assert.match(
  communityServiceSource,
  /pc community comments contract is not available/u,
  'pc community comment mutations must fail closed until the community comments contract exists.',
);
assert.match(
  calendarServiceSource,
  /pc calendar contract is not available/u,
  'pc calendar mutations must fail closed until the calendar SDK contract exists.',
);
assert.match(
  courseServiceSource,
  /getCoursePcSdkPorts\(\)\.getCourseClient/u,
  'pc course service must consume the host-injected course app SDK client.',
);
assert.match(
  courseServiceSource,
  /pc course comments contract is not available/u,
  'pc course comment mutations must fail closed until the course comments contract exists.',
);
assert.doesNotMatch(
  courseServiceSource,
  /unsplash/u,
  'pc course service must not keep demo media or local mock catalog data.',
);
assert.doesNotMatch(
  `${communityViewSource}${communitySettingsSource}`,
  /pravatar|unsplash/u,
  'pc community surfaces must not keep demo avatar or media placeholders.',
);
assert.doesNotMatch(
  shopHomeSource,
  /unsplash/u,
  'pc shop home must not keep demo banner media.',
);
assert.doesNotMatch(
  `${videoPlayerViewSource}${liveRoomViewSource}`,
  /unsplash|ui-avatars/u,
  'pc course player surfaces must not keep demo avatar or media placeholders.',
);
assert.doesNotMatch(
  checkoutViewSource,
  /MOCK_ADDRESSES|138 \*\*\*\* 0000/u,
  'pc shop checkout must not keep mock shipping addresses or demo account labels.',
);
assert.doesNotMatch(
  cashierViewSource,
  /Math\.random|setTimeout/u,
  'pc shop cashier must not simulate payment status with timers or random qr codes.',
);
assert.match(
  `${cashierViewSource}${shopServiceSource}`,
  /pc shop payment contract is not available|PC_SHOP_PAYMENT_CONTRACT_UNAVAILABLE/u,
  'pc shop cashier must fail closed until the commerce payment contract exists.',
);
assert.match(videoGenServiceSource, /pc videogen contract is not available/u, 'pc videogen mutations must fail closed until the videogen SDK contract exists.');
assert.match(imageGenServiceSource, /pc imagegen contract is not available/u, 'pc imagegen mutations must fail closed until the imagegen SDK contract exists.');
assert.match(voiceGenServiceSource, /getConfiguredVoiceAppSdkClient|voice\.speech\.create|listVoiceAudioAssetOptions/u, 'pc voice speech must consume the generated voice app SDK contract.');
assert.match(musicGenServiceSource, /pc musicgen contract is not available/u, 'pc musicgen mutations must fail closed until the musicgen SDK contract exists.');
assert.match(writingServiceSource, /pc writing contract is not available/u, 'pc writing mutations must fail closed until the writing SDK contract exists.');
const emojiPickerSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/EmojiPicker.tsx');
const musicPlayerSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/MusicPlayer.tsx');
const messageItemsSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/MessageItems.tsx');
assert.doesNotMatch(
  `${emojiPickerSource}${musicPlayerSource}${messageItemsSource}`,
  /picsum\.photos/u,
  'pc chat media surfaces must not keep external placeholder image hosts.',
);
assert.match(
  emojiPickerSource,
  /pc sticker pack contract is not available/u,
  'pc chat sticker picker must fail closed until the sticker pack SDK contract exists.',
);
assert.match(
  approvalsServiceSource,
  /pc approvals contract is not available/u,
  'pc approvals mutations must fail closed until the approvals SDK contract exists.',
);
assert.match(
  attendanceServiceSource,
  /pc attendance contract is not available/u,
  'pc attendance mutations must fail closed until the attendance SDK contract exists.',
);
assert.match(
  reportServiceSource,
  /pc reports contract is not available/u,
  'pc reports mutations must fail closed until the reports SDK contract exists.',
);
assert.match(
  enterpriseMarketplaceServiceSource,
  /pc enterprise marketplace contract is not available/u,
  'pc enterprise marketplace mutations must fail closed until the enterprise marketplace SDK contract exists.',
);

assert.match(
  sessionSource,
  /getSessionStorage\(\)/u,
  'IM PC browser session storage must persist auth tokens in sessionStorage instead of localStorage.',
);
assert.match(
  sessionSource,
  /isDesktopSecureSessionStorageEnabled/u,
  'IM PC session storage must route desktop auth tokens through native secure storage.',
);
assert.match(
  sessionSource,
  /hydrateAppSdkSessionFromSecureStorage/u,
  'IM PC session storage must hydrate desktop secure storage before auth bootstrap.',
);
assert.match(
  sessionSource,
  /migrateLegacySessionStorage/u,
  'IM PC session storage must migrate legacy localStorage auth sessions into sessionStorage.',
);
assert.match(
  sessionSource,
  /localStorage\.removeItem\(SDKWORK_IM_LEGACY_SESSION_STORAGE_KEY\)/u,
  'IM PC session storage must clear legacy localStorage auth sessions after migration.',
);

console.log('sdkwork im pc sidebar module SDK boundary contract passed.');
