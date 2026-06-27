import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import type {
  isSupportedQrImageFile as IsSupportedQrImageFile,
} from '../packages/sdkwork-im-pc-chat/src/services/QrCodeDecodeService';
import type {
  getQrCodeResultLabelKey as GetQrCodeResultLabelKey,
  getQrCodeScanActions as GetQrCodeScanActions,
  parseQrCodeContent as ParseQrCodeContent,
} from '../packages/sdkwork-im-pc-chat/src/services/QrCodeScanService';

const qrDecodeServiceModule = await import('../packages/sdkwork-im-pc-chat/src/services/QrCodeDecodeService') as typeof import('../packages/sdkwork-im-pc-chat/src/services/QrCodeDecodeService') & {
  default?: {
    isSupportedQrImageFile?: typeof IsSupportedQrImageFile;
  };
};
const qrScanServiceModule = await import('../packages/sdkwork-im-pc-chat/src/services/QrCodeScanService') as typeof import('../packages/sdkwork-im-pc-chat/src/services/QrCodeScanService') & {
  default?: {
    getQrCodeResultLabelKey?: typeof GetQrCodeResultLabelKey;
    getQrCodeScanActions?: typeof GetQrCodeScanActions;
    parseQrCodeContent?: typeof ParseQrCodeContent;
  };
};
const isSupportedQrImageFile = (
  qrDecodeServiceModule.isSupportedQrImageFile
  ?? qrDecodeServiceModule.default?.isSupportedQrImageFile
) as typeof IsSupportedQrImageFile;
const parseQrCodeContent = (
  qrScanServiceModule.parseQrCodeContent
  ?? (qrScanServiceModule.default as { parseQrCodeContent?: typeof ParseQrCodeContent } | undefined)?.parseQrCodeContent
) as typeof ParseQrCodeContent;
const getQrCodeScanActions = (
  qrScanServiceModule.getQrCodeScanActions
  ?? qrScanServiceModule.default?.getQrCodeScanActions
) as typeof GetQrCodeScanActions;
const getQrCodeResultLabelKey = (
  qrScanServiceModule.getQrCodeResultLabelKey
  ?? qrScanServiceModule.default?.getQrCodeResultLabelKey
) as typeof GetQrCodeResultLabelKey;

assert.equal(typeof isSupportedQrImageFile, 'function', 'QR decode service must export supported image file validation.');
assert.equal(typeof parseQrCodeContent, 'function', 'QR scan service must export parseQrCodeContent.');
assert.equal(typeof getQrCodeScanActions, 'function', 'QR scan service must export standardized scan actions.');
assert.equal(typeof getQrCodeResultLabelKey, 'function', 'QR scan service must export standardized result label lookup.');

const appRoot = path.resolve(import.meta.dirname, '..');

function readText(...segments: string[]): string {
  return fs.readFileSync(path.join(appRoot, ...segments), 'utf8');
}

function readJson(...segments: string[]): Record<string, unknown> {
  return JSON.parse(readText(...segments)) as Record<string, unknown>;
}

const userFromJson = parseQrCodeContent(JSON.stringify({
  sdkworkQrVersion: 1,
  scenario: 'user',
  userId: 'user-123',
  chatId: 'chat-123',
}));

assert.equal(userFromJson.kind, 'user');
assert.equal(userFromJson.userId, 'user-123');
assert.equal(userFromJson.chatId, 'chat-123');

const userFromChatIdOnlyJson = parseQrCodeContent(JSON.stringify({
  sdkworkQrVersion: 1,
  scenario: 'user',
  chatId: 'chat-only-123',
}));

assert.equal(userFromChatIdOnlyJson.kind, 'user');
assert.equal(userFromChatIdOnlyJson.userId, 'chat-only-123');
assert.equal(userFromChatIdOnlyJson.chatId, 'chat-only-123');

const userFromChatIdOnlyUrl = parseQrCodeContent('sdkwork://chat/user/chat-only-456?chatId=chat-only-456');
assert.equal(userFromChatIdOnlyUrl.kind, 'user');
assert.equal(userFromChatIdOnlyUrl.userId, 'chat-only-456');
assert.equal(userFromChatIdOnlyUrl.chatId, 'chat-only-456');

const userFromQueryOnlyUrl = parseQrCodeContent('sdkwork://chat/user?chatId=query-chat-123');
assert.equal(userFromQueryOnlyUrl.kind, 'user');
assert.equal(userFromQueryOnlyUrl.userId, 'query-chat-123');
assert.equal(userFromQueryOnlyUrl.chatId, 'query-chat-123');

const groupFromDeepLink = parseQrCodeContent('sdkwork://chat/group/group-1?inviteCode=invite-1');
assert.equal(groupFromDeepLink.kind, 'group');
assert.equal(groupFromDeepLink.groupId, 'group-1');
assert.equal(groupFromDeepLink.inviteCode, 'invite-1');

const groupFromQueryOnlyDeepLink = parseQrCodeContent('sdkwork://chat/group?groupId=query-group-1&inviteCode=query-invite-1');
assert.equal(groupFromQueryOnlyDeepLink.kind, 'group');
assert.equal(groupFromQueryOnlyDeepLink.groupId, 'query-group-1');
assert.equal(groupFromQueryOnlyDeepLink.inviteCode, 'query-invite-1');

const communityFromDeepLink = parseQrCodeContent('im://community/community-1');
assert.equal(communityFromDeepLink.kind, 'community');
assert.equal(communityFromDeepLink.communityId, 'community-1');

const communityFromQueryOnlyDeepLink = parseQrCodeContent('im://community?communityId=query-community-1');
assert.equal(communityFromQueryOnlyDeepLink.kind, 'community');
assert.equal(communityFromQueryOnlyDeepLink.communityId, 'query-community-1');

const publicProfileUrl = parseQrCodeContent('https://sdkwork.com/chat/users/user-456?chatId=chat-456');
assert.equal(publicProfileUrl.kind, 'user');
assert.equal(publicProfileUrl.userId, 'user-456');
assert.equal(publicProfileUrl.chatId, 'chat-456');

const normalUrl = parseQrCodeContent('https://example.com/docs?a=1');
assert.equal(normalUrl.kind, 'url');
assert.equal(normalUrl.url, 'https://example.com/docs?a=1');

const unsafeUrl = parseQrCodeContent('javascript:alert(1)');
assert.equal(unsafeUrl.kind, 'unknown');
assert.equal(unsafeUrl.rawContent, 'javascript:alert(1)');

const malformedScenarioUrl = parseQrCodeContent('https://sdkwork.com/chat/users/%E0%A4%A');
assert.equal(malformedScenarioUrl.kind, 'unknown');
assert.equal(malformedScenarioUrl.rawContent, 'https://sdkwork.com/chat/users/%E0%A4%A');

const unknownContent = parseQrCodeContent('hello from qr');
assert.equal(unknownContent.kind, 'unknown');
assert.equal(unknownContent.rawContent, 'hello from qr');

assert.equal(getQrCodeResultLabelKey(userFromJson), 'scanQr.result.user');
assert.deepEqual(
  getQrCodeScanActions(userFromJson).map((action) => action.kind),
  ['viewUserProfile', 'sendFriendRequest'],
);
assert.deepEqual(
  getQrCodeScanActions(groupFromDeepLink).map((action) => action.kind),
  ['openGroup', 'joinGroup'],
);
assert.deepEqual(
  getQrCodeScanActions(communityFromDeepLink).map((action) => action.kind),
  ['openCommunity', 'joinCommunity'],
);
assert.deepEqual(
  getQrCodeScanActions(normalUrl).map((action) => action.kind),
  ['openEmbeddedBrowser', 'copyRawContent'],
);
assert.deepEqual(
  getQrCodeScanActions(unknownContent).map((action) => action.kind),
  ['showUnknownContentModal', 'copyRawContent'],
);

assert.throws(
  () => parseQrCodeContent('   '),
  /QR code content is required/u,
);

assert.equal(isSupportedQrImageFile(new File([''], 'qr.png', { type: 'image/png' })), true);
assert.equal(isSupportedQrImageFile(new File([''], 'qr.jpg', { type: 'image/jpeg' })), true);
assert.equal(isSupportedQrImageFile(new File([''], 'qr.webp', { type: 'image/webp' })), true);
assert.equal(
  isSupportedQrImageFile(new File([new Uint8Array(8 * 1024 * 1024)], 'qr.png', { type: 'image/png' })),
  true,
  'QR upload validation must allow images up to the desktop native decoder size limit.',
);
assert.equal(
  isSupportedQrImageFile(new File([new Uint8Array((8 * 1024 * 1024) + 1)], 'qr.png', { type: 'image/png' })),
  false,
  'QR upload validation must reject images larger than the desktop native decoder size limit.',
);
assert.equal(
  isSupportedQrImageFile(new File([''], 'qr.gif', { type: 'image/gif' })),
  false,
  'QR upload drag/drop validation must reject image formats not supported by the desktop native decoder.',
);
assert.equal(
  isSupportedQrImageFile(new File([''], 'qr.png', { type: 'image/gif' })),
  false,
  'QR upload drag/drop validation must reject unsupported MIME types even when the file extension is allowed.',
);
assert.equal(
  isSupportedQrImageFile(new File([''], 'qr.heic', { type: 'image/heic' })),
  false,
  'QR upload drag/drop validation must reject HEIC because the desktop native decoder does not enable HEIC.',
);

const scanServiceSource = readText(
  'packages',
  'sdkwork-im-pc-chat',
  'src',
  'services',
  'QrCodeScanService.ts',
);

const scanModalSource = readText(
  'packages',
  'sdkwork-im-pc-chat',
  'src',
  'components',
  'ScanQrCodeModal.tsx',
);
const decoderSource = readText(
  'packages',
  'sdkwork-im-pc-chat',
  'src',
  'services',
  'QrCodeDecodeService.ts',
);
const desktopCargoSource = readText(
  'packages',
  'sdkwork-im-pc-desktop',
  'src-tauri',
  'Cargo.toml',
);
const desktopLibSource = readText(
  'packages',
  'sdkwork-im-pc-desktop',
  'src-tauri',
  'src',
  'lib.rs',
);
const desktopQrCodeSource = readText(
  'packages',
  'sdkwork-im-pc-desktop',
  'src-tauri',
  'src',
  'qr_code.rs',
);
const desktopWindowControlSource = readText(
  'packages',
  'sdkwork-im-pc-desktop',
  'src-tauri',
  'src',
  'window_control.rs',
);
const desktopTraySource = readText(
  'packages',
  'sdkwork-im-pc-desktop',
  'src-tauri',
  'src',
  'tray.rs',
);
const desktopDefaultCapabilitySource = readText(
  'packages',
  'sdkwork-im-pc-desktop',
  'src-tauri',
  'capabilities',
  'default.json',
);
const desktopQrPermissionSource = readText(
  'packages',
  'sdkwork-im-pc-desktop',
  'src-tauri',
  'permissions',
  'qr-code.toml',
);
const chatLayoutSource = readText(
  'packages',
  'sdkwork-im-pc-chat',
  'src',
  'pages',
  'ChatLayout.tsx',
);
const capabilityModuleSurfaceSource = readText(
  'packages',
  'sdkwork-im-pc-chat',
  'src',
  'surfaces',
  'CapabilityModuleSurface.tsx',
);
const communityViewSource = readText(
  'packages',
  'sdkwork-im-pc-community',
  'src',
  'components',
  'CommunityView.tsx',
);
const packageJson = readJson('package.json') as {
  dependencies?: Record<string, string>;
  scripts?: Record<string, string>;
};
const pnpmWorkspaceSource = readText('pnpm-workspace.yaml');

assert.equal(
  packageJson.dependencies?.['@zxing/browser'],
  'catalog:',
  'PC app package must depend on the QR decoder through the pnpm catalog.',
);
assert.match(
  pnpmWorkspaceSource,
  /['"]?@zxing\/browser['"]?:\s*\^/u,
  'pnpm catalog must own the @zxing/browser version.',
);
assert.equal(
  packageJson.scripts?.['test:qr-scan-standard'],
  'tsx scripts/qr-scan-standard-contract.test.ts',
  'PC app package must expose the QR scan standard contract test.',
);

assert.match(
  chatLayoutSource,
  /<ScanQrCodeModal[\s\S]*isOpen=\{isScanQrOpen\}/u,
  'ChatLayout must render the scan QR modal from the conversation action surface.',
);
assert.match(
  chatLayoutSource,
  /setIsScanQrOpen\(true\)[\s\S]*chat\.menu\.scanQrCode/u,
  'The new conversation menu must include the scan QR entry.',
);

for (const marker of [
  'decodeQrCodeFromImageFile',
  'createQrCameraScanner',
  'isSupportedQrImageFile',
  'SUPPORTED_QR_IMAGE_FILE_EXTENSION_PATTERN',
  'MAX_QR_IMAGE_FILE_SIZE_BYTES',
  'decodeQrCodeWithDesktopNative',
  'sdkwork_chat_pc_decode_qr_code_image',
  'sdkwork_chat_pc_decode_qr_code_rgba',
]) {
  assert.match(
    decoderSource,
    new RegExp(marker, 'u'),
    `QR decode service must include ${marker}.`,
  );
}

assert.doesNotMatch(
  decoderSource,
  /import\s+\{[\s\S]*BrowserQRCodeReader[\s\S]*\}\s+from\s+['"]@zxing\/browser['"]/u,
  'QR decode service must not statically import @zxing/browser because desktop builds use native Rust decoding.',
);
assert.match(
  decoderSource,
  /import\(['"]@zxing\/browser['"]\)/u,
  'QR decode service may only load @zxing/browser dynamically for browser runtime fallback.',
);
assert.match(
  decoderSource,
  /isSdkworkChatDesktopRuntime\(\)[\s\S]*decodeQrCodeWithDesktopNative/u,
  'Desktop QR image decoding must route to native Tauri/Rust before browser ZXing fallback.',
);
assert.match(
  decoderSource,
  /requestAnimationFrame[\s\S]*decodeQrCodeRgbaWithDesktopNative/u,
  'Desktop camera scanning must decode captured frames through native Tauri/Rust.',
);

for (const marker of [
  'rqrr',
  'image',
  'base64',
]) {
  assert.match(
    desktopCargoSource,
    new RegExp(marker, 'u'),
    `Desktop Cargo.toml must include native QR dependency ${marker}.`,
  );
}

for (const marker of [
  'sdkwork_chat_pc_decode_qr_code_image',
  'sdkwork_chat_pc_decode_qr_code_rgba',
  'decode_qr_code_from_dynamic_image',
  'MAX_QR_IMAGE_BYTES',
  'MAX_QR_IMAGE_DIMENSION',
  'MAX_QR_CAMERA_FRAME_PIXELS',
  'native_qr_decoder_reads_png_qr_payload',
  'native_qr_decoder_reads_rgba_camera_frame_payload',
  'native_qr_decoder_rejects_oversized_image_payload',
  'native_qr_decoder_rejects_oversized_image_dimensions',
  'native_qr_decoder_rejects_oversized_rgba_frame_dimensions',
]) {
  assert.match(
    desktopQrCodeSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
    `Desktop native QR module must include ${marker}.`,
  );
}
assert.match(
  desktopLibSource,
  /mod notification;\s*mod qr_code;\s*mod session_store;\s*mod tray;\s*mod window_control;/u,
  'Desktop lib.rs must stay a thin module assembly and declare focused host capability modules.',
);
assert.match(
  desktopLibSource,
  /tauri::generate_handler!\[[\s\S]*window_control::sdkwork_chat_pc_window_control[\s\S]*notification::sdkwork_chat_pc_show_notification[\s\S]*qr_code::sdkwork_chat_pc_decode_qr_code_image[\s\S]*qr_code::sdkwork_chat_pc_decode_qr_code_rgba[\s\S]*session_store::sdkwork_im_pc_session_read[\s\S]*\]/u,
  'Desktop Rust shell must register window control, notification, native QR decode, and session store commands.',
);
assert.doesNotMatch(
  desktopLibSource,
  /fn decode_qr_code_from_dynamic_image|struct DecodeQrCodeImageRequest|MenuBuilder|TrayIconBuilder|enum SdkworkChatPcWindowControlAction/u,
  'Desktop lib.rs must not contain QR implementation, tray implementation, or window-control command details.',
);
assert.match(
  desktopWindowControlSource,
  /sdkwork_chat_pc_window_control/u,
  'Desktop window-control command must live in its focused module.',
);
assert.match(
  desktopTraySource,
  /ensure_tray[\s\S]*handle_window_event/u,
  'Desktop tray and close-to-tray behavior must live in its focused module.',
);
assert.match(
  desktopDefaultCapabilitySource,
  /allow-sdkwork-im-pc-qr-code/u,
  'Desktop default capability must grant only the Sdkwork IM PC QR native command permission.',
);
assert.match(
  desktopQrPermissionSource,
  /commands\.allow\s*=\s*\[\s*["']sdkwork_chat_pc_decode_qr_code_image["']\s*,\s*["']sdkwork_chat_pc_decode_qr_code_rgba["']\s*\]/u,
  'Desktop QR permission must allow only the native QR decode commands.',
);

for (const marker of [
  'QrCodeScanActionKind',
  'QrCodeScanAction',
  'QR_CODE_SCENARIO_ACTION_DEFINITIONS',
  'getQrCodeScanActions',
  'getQrCodeResultLabelKey',
  'scanQr.modal.title',
  'scanQr.tabs.upload',
  'scanQr.tabs.camera',
  'scanQr.camera.device',
  'scanQr.camera.defaultDevice',
  'scanQr.camera.noDevices',
  'scanQr.actions.copyContent',
  'scanQr.actions.addFriend',
  'scanQr.actions.openLink',
  'scanQr.result.unknown',
]) {
  assert.match(
    `${scanServiceSource}\n${scanModalSource}`,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
    `Scan QR modal must use i18n key ${marker}.`,
  );
}

assert.match(
  scanModalSource,
  /accept=["']\.png,\.jpg,\.jpeg,\.webp["']/u,
  'Scan QR upload input must advertise only the QR image formats that the desktop native decoder supports.',
);
assert.match(
  scanModalSource,
  /!\s*community\s*&&\s*\([\s\S]*scanQr\.state\.joinCapabilityUnavailable/u,
  'Scan QR community result must only show join-unavailable guidance when no existing community can be opened.',
);
assert.match(
  chatLayoutSource,
  /const\s+\[pendingCommunityId,\s*setPendingCommunityId\]\s*=\s*useState<string\s*\|\s*null>\(null\)/u,
  'ChatLayout must keep the scanned community id so the community tab can open the exact target.',
);
assert.match(
  `${chatLayoutSource}\n${capabilityModuleSurfaceSource}`,
  /initialCommunityId=\{pendingCommunityId\s*\?\?\s*undefined\}[\s\S]*onInitialCommunityHandled=/u,
  'Chat layout must pass the scanned community id into CommunityView and clear it after handling.',
);
assert.match(
  chatLayoutSource,
  /onOpenCommunity=\{\(communityId\)\s*=>\s*\{[\s\S]*setPendingCommunityId\(communityId\)[\s\S]*setActiveTab\("community"\)/u,
  'Scan QR community open action must preserve the scanned community id before navigating to the community tab.',
);
assert.match(
  communityViewSource,
  /initialCommunityId\?:\s*string[\s\S]*onInitialCommunityHandled\?:\s*\(\)\s*=>\s*void/u,
  'CommunityView must expose a typed initialCommunityId prop for QR-driven navigation.',
);
assert.match(
  communityViewSource,
  /communityService\.getCommunity\(initialCommunityId\)[\s\S]*setActiveCommunity\(community\)[\s\S]*onInitialCommunityHandled\?\.\(\)/u,
  'CommunityView must resolve and open the scanned community by id.',
);

for (const marker of [
  'scanQr.actions.viewContent',
  'scanQr.actions.viewUserProfile',
  'scanQr.actions.joinCommunity',
  'scanQr.userProfile.modalTitle',
  'userProfileModalUser',
  'setUserProfileModalUser(user)',
  'contacts.detail.chatId',
  'contacts.detail.email',
  'contacts.detail.phone',
  'scanQr.unknownContent.modalTitle',
  'unknownContentModalPayload',
  'setUnknownContentModalPayload(payload)',
  'showUnknownContentModal',
  'getQrCodeScanActions',
  'getQrCodeResultLabelKey',
]) {
  assert.match(
    scanModalSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
    `Scan QR modal must include standardized action or unknown-content modal marker ${marker}.`,
  );
}

assert.doesNotMatch(
  `${scanModalSource}\n${decoderSource}`,
  /fetch\s*\(|axios\.|Authorization|X-API-Key|accessToken|mock\s+success|fake\s+success/iu,
  'Scan QR UI and decoder must not introduce raw HTTP, manual auth, or fake success branches.',
);
assert.match(
  scanModalSource,
  /joinCapabilityUnavailable/u,
  'Group and community QR joins must fail closed when no real SDK-backed join contract is available.',
);
assert.match(
  scanModalSource,
  /enumerateDevices[\s\S]*videoinput/u,
  'Scan QR camera pane must enumerate video input devices for polished PC camera scanning.',
);
assert.match(
  scanModalSource,
  /selectedCameraDeviceId[\s\S]*createQrCameraScanner/u,
  'Scan QR camera pane must pass the selected camera device into the QR scanner.',
);

console.log('sdkwork im qr scan standard contract passed.');
