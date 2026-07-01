import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

const rootPackage = readJson('package.json');
const sidebarSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/Sidebar.tsx');
const settingsServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/SettingsService.ts');
const hostAppearanceBridgeSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-commons/src/hostAppearanceBridge.ts');
const moduleRegistrySource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-shell/src/moduleRegistry.ts');
const settingsModalSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/SettingsModal.tsx');
const chatLayoutSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/pages/ChatLayout.tsx');
const capabilityModuleSurfaceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/surfaces/CapabilityModuleSurface.tsx',
);
const capabilityModuleLoadersSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-shell/src/capabilityModuleLoaders.ts',
);
const moduleLayoutSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-shell/src/moduleLayout.ts');
const notaryAccessServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/NotaryAccessService.ts');
const workspaceServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-workspace/src/services/WorkspaceService.ts');
const workspaceViewSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-workspace/src/index.tsx');
const localApiSource = read('apps/sdkwork-im-pc/local-api.ts');
const devServerSource = read('apps/sdkwork-im-pc/server.ts');
const newFriendsContainerSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/contacts/NewFriendsContainer.tsx');
const contactServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ContactService.ts');
const messageListSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/MessageList.tsx');
const chatListSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ChatList.tsx');
const toastSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/Toast.tsx');
const zhLocale = readJson('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/i18n/locales/zh-CN.json');
const enLocale = readJson('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/i18n/locales/en-US.json');

const expectedDefaultSidebarModules = [
  'chat',
  'workspace',
  'contacts',
  'knowledge',
  'drive',
  'agent',
  'favorites',
];

function extractStringArray(source, exportName) {
  const match = source.match(
    new RegExp(`export\\s+const\\s+${exportName}(?::[^=]+)?\\s*=\\s*\\[([\\s\\S]*?)\\]`, 'u'),
  );
  assert.ok(match, `${exportName} must be exported as a string array`);
  return [...match[1].matchAll(/"([^"]+)"/gu)].map((item) => item[1]);
}

assert.equal(
  rootPackage.scripts?.['test:sdkwork-im-pc-sidebar-modules'],
  'node scripts/dev/sdkwork-im-pc-sidebar-modules.test.mjs',
  'root package must expose the sdkwork im pc sidebar modules regression test',
);

assert.doesNotMatch(
  sidebarSource,
  /\bsetInterval\s*\(/u,
  'Sidebar must not poll /api/config/modules with setInterval',
);
assert.doesNotMatch(
  sidebarSource,
  /\bclearInterval\s*\(/u,
  'Sidebar should not own an interval cleanup for module config refresh',
);

assert.match(
  sidebarSource,
  /addEventListener\(["']focus["']/u,
  'Sidebar must refresh module config when the app window regains focus',
);
assert.match(
  sidebarSource,
  /addEventListener\(["']visibilitychange["']/u,
  'Sidebar must refresh module config when the app returns from hidden to visible',
);
assert.match(
  sidebarSource,
  /document\.visibilityState\s*===\s*["']visible["']/u,
  'Sidebar visibility handler must only refresh after the document becomes visible',
);

assert.match(
  sidebarSource,
  /sdkwork-im-pc:settings-changed/u,
  'Sidebar must listen for settings changes instead of relying on polling',
);
assert.match(
  hostAppearanceBridgeSource,
  /sdkwork-im-pc:settings-changed/u,
  'hostAppearanceBridge must own the canonical settings changed event name',
);
assert.match(
  settingsServiceSource,
  /SDKWORK_IM_PC_SETTINGS_CHANGED_EVENT/u,
  'SettingsService must notify same-window consumers via the shared settings changed event',
);
assert.match(
  settingsServiceSource,
  /dispatchEvent\(new CustomEvent/u,
  'SettingsService must dispatch a typed settings changed event after updates',
);

assert.deepEqual(
  extractStringArray(moduleRegistrySource, 'DEFAULT_SIDEBAR_MODULES'),
  expectedDefaultSidebarModules,
  'default sidebar modules must only keep chat, workspace, contacts, knowledge, drive, agent, and favorites visible',
);

assert.match(
  moduleRegistrySource,
  /export\s+const\s+ALWAYS_CONFIGURABLE_MODULES\s*=\s*new Set[\s\S]*["']notary["']/u,
  'shell moduleRegistry must own the always-configurable notary module contract shared by settings and sidebar',
);
assert.match(
  moduleRegistrySource,
  /export\s+const\s+ALL_APP_MODULES\s*=\s*\[[\s\S]*["']notary["'][\s\S]*\]/u,
  'shell moduleRegistry catalog must retain notary so the configuration center can offer the feature',
);
assert.match(
  settingsServiceSource,
  /from\s+["']@sdkwork\/im-pc-shell\/moduleRegistry["']/u,
  'SettingsService must import the module catalog from shell moduleRegistry',
);
assert.match(
  settingsServiceSource,
  /isCommercialRuntimeModule/u,
  'SettingsService must filter server modules to commercial runtime modules',
);
assert.match(
  moduleRegistrySource,
  /export const COMMERCIAL_RUNTIME_MODULES/u,
  'moduleRegistry must declare commercial runtime modules for production navigation',
);
assert.match(
  localApiSource,
  /listCommercialRuntimeModules\(\)/u,
  'Local configuration API must serve the commercial runtime module catalog',
);
assert.match(
  localApiSource,
  /requestPath\s*===\s*['"]\/api\/config\/modules['"][\s\S]*modules:\s*\[\.\.\.LOCAL_APP_MODULES\]/u,
  'Local configuration API must serve modules from LOCAL_APP_MODULES',
);
assert.match(
  devServerSource,
  /handleSdkworkChatLocalApiRequest/u,
  'Dev server must delegate shell endpoints to local-api instead of duplicating AI Studio routes',
);
assert.doesNotMatch(
  localApiSource,
  /GEMINI_API_KEY|AI Studio|@google\/genai/u,
  'local-api must not retain AI Studio or Gemini scaffold dependencies',
);
assert.doesNotMatch(
  devServerSource,
  /GEMINI_API_KEY|AI Studio|@google\/genai/u,
  'dev server must not retain AI Studio or Gemini scaffold dependencies',
);
assert.doesNotMatch(
  notaryAccessServiceSource,
  /canShowNotaryMenu/u,
  'NotaryAccessService must not expose menu visibility APIs; notary entry discovery is owned by SettingsService and workspace catalog',
);
assert.doesNotMatch(
  notaryAccessServiceSource,
  /\bvisible:\s*boolean/u,
  'NotaryAccessState must not expose a visible field that can be reused for menu or entry display gating',
);
assert.doesNotMatch(
  notaryAccessServiceSource,
  /\bvisible:\s*false/u,
  'NotaryAccessState denied fallback must not publish a UI visibility-shaped field',
);

for (const moduleId of expectedDefaultSidebarModules) {
  assert.match(
    moduleRegistrySource,
    new RegExp(`"(${moduleId})"`, 'u'),
    `shell moduleRegistry catalog must include ${moduleId}`,
  );
}

assert.match(
  settingsServiceSource,
  /sidebarModules:\s*\[\.\.\.DEFAULT_SIDEBAR_MODULES\]/u,
  'SettingsService default settings must use DEFAULT_SIDEBAR_MODULES instead of all business modules',
);
assert.doesNotMatch(
  settingsServiceSource,
  /ALL_APP_MODULES\.forEach\(\(feature\)[\s\S]*?parsed\.sidebarModules\.push\(feature\)/u,
  'loading saved settings must not append every unfinished business module back into sidebarModules',
);
assert.match(
  settingsServiceSource,
  /normalizeSidebarModules/u,
  'SettingsService must normalize persisted sidebar modules through an explicit helper',
);

assert.match(
  sidebarSource,
  /DEFAULT_SIDEBAR_MODULES/u,
  'Sidebar must initialize and fall back to the default visible module list',
);
assert.doesNotMatch(
  sidebarSource,
  /m\s*===\s*["']course["']/u,
  'Sidebar must not treat course as a pinned module',
);
assert.doesNotMatch(
  sidebarSource,
  /setSidebarModules\(Array\.from\(new Set\(\["chat",\s*\.\.\.actualModules,\s*"course"\]\)\)\)/u,
  'Sidebar must not force-add course to the visible module set',
);
assert.match(
  sidebarSource,
  /modId\s*===\s*["']drive["']/u,
  'Sidebar must render a visible drive entry',
);
assert.match(
  sidebarSource,
  /onTabChange\(["']drive["']\)/u,
  'Sidebar drive entry must route to the existing drive view',
);
assert.match(
  sidebarSource,
  /title=\{t\(["']sidebar\.drive["']\)\}/u,
  'Sidebar drive entry must use the standard sidebar.drive i18n title',
);
assert.match(
  sidebarSource,
  /ALWAYS_CONFIGURABLE_MODULES[\s\S]*?\.has\(m\)/u,
  'Sidebar must keep user-enabled notary in the sidebar candidate list without requiring the server module snapshot',
);
assert.doesNotMatch(
  sidebarSource,
  /notaryAccessService\.canShowNotaryMenu/u,
  'Sidebar must not use notary access state to decide whether the notary module is displayed',
);
assert.doesNotMatch(
  sidebarSource,
  /modId\s*===\s*["']notary["']\s*&&\s*canShowNotaryMenu/u,
  'Sidebar must render the configured notary entry without access-state display gating',
);
assert.match(
  sidebarSource,
  /modId\s*===\s*["']notary["']\)/u,
  'Sidebar must keep the configured notary entry render branch',
);
assert.equal(zhLocale.sidebar.drive, '网盘', 'zh-CN sidebar locale must name drive as 网盘');
assert.equal(enLocale.sidebar.drive, 'Cloud Drive', 'en-US sidebar locale must name drive as Cloud Drive');

assert.match(
  settingsModalSource,
  /DEFAULT_SIDEBAR_MODULES/u,
  'SettingsModal must fall back to the product-ready default sidebar modules',
);
assert.match(
  settingsModalSource,
  /\{\s*id:\s*["']drive["'][\s\S]*?icon:\s*Cloud/u,
  'SettingsModal module list must include the drive module with the standard cloud icon',
);
assert.match(
  settingsModalSource,
  /\{\s*id:\s*["']notary["'][\s\S]*?icon:\s*ShieldCheck/u,
  'SettingsModal module list must include the notary module option so users can enable it',
);
assert.match(
  settingsModalSource,
  /ALWAYS_CONFIGURABLE_MODULES[\s\S]*["']notary["']/u,
  'SettingsModal must keep notary visible as an always-configurable module option so users can enable it from the configuration center',
);
assert.match(
  settingsModalSource,
  /ALWAYS_CONFIGURABLE_MODULES[\s\S]*?\.has\(mod\.id\)/u,
  'SettingsModal must not require the server module snapshot before showing the notary enable option',
);
assert.doesNotMatch(
  settingsModalSource,
  /notaryAccessService\.canShowNotaryMenu/u,
  'SettingsModal must not hide the notary enable option behind current notary business access state',
);
assert.match(
  settingsModalSource,
  /DEFAULT_SIDEBAR_MODULES[\s\S]*?\.includes\(mod\.id\)/u,
  'SettingsModal must keep the product-ready default modules visible even when the server module list is incomplete',
);
assert.match(
  capabilityModuleLoadersSource,
  /drive:\s*\(\)\s*=>\s*import\(['"]@sdkwork\/drive-pc-drive['"]\)/u,
  'shell capability loaders must lazy-load the drive module',
);
assert.match(
  capabilityModuleSurfaceSource,
  /isShellCapabilityModule\(activeTab\)/u,
  'CapabilityModuleSurface must delegate shell capability modules to lazy loaders',
);
assert.match(
  capabilityModuleSurfaceSource,
  /resolveWorkspaceAppTab\(appId\)/u,
  'Workspace app selection must resolve launcher app ids through shell moduleRegistry',
);
assert.match(
  capabilityModuleLoadersSource,
  /notary:\s*\(\)\s*=>\s*import\(['"]@sdkwork\/notary-pc-notary['"]\)/u,
  'shell capability loaders must lazy-load the notary module when selected from sidebar or workspace',
);
assert.match(
  moduleLayoutSource,
  /FULLSCREEN_MODULE_TABS[\s\S]*["']drive["']/u,
  'shell moduleLayout must treat drive as a full-screen module page instead of rendering an empty unified header',
);
assert.match(
  chatLayoutSource,
  /ModuleRenderHost/u,
  'ChatLayout must delegate module routing to shell ModuleRenderHost',
);
assert.match(
  workspaceServiceSource,
  /\{\s*id:\s*["']notary["'][\s\S]*nameKey:\s*["']apps\.notary["'][\s\S]*iconName:\s*["']ShieldCheck["']/u,
  'Workspace app catalog must keep the notary business entry available from the workbench',
);
assert.doesNotMatch(
  workspaceServiceSource,
  /\b(?:MockWorkspaceService|mockApps|mockDocs)\b/u,
  'Workspace app catalog must use stable static catalog naming instead of mock naming for app entries such as notary',
);
assert.match(
  workspaceServiceSource,
  /const\s+REQUIRED_WORKSPACE_APP_IDS\s*=\s*new Set\(\[\s*['"]notary['"]\s*\]\)/u,
  'Workspace app catalog must protect notary as a required workbench entry',
);
assert.match(
  workspaceServiceSource,
  /return mergeApps\(buildCatalogApps\(enabledModules\), readStoredApps\(\)\)/u,
  'Workspace getApps must merge enabled catalog apps with stored optional entries',
);
assert.match(
  workspaceServiceSource,
  /REQUIRED_WORKSPACE_APP_IDS\.has\(id\)[\s\S]*?return/u,
  'Workspace removeApp must preserve required workbench entries such as notary',
);
assert.match(
  workspaceServiceSource,
  /REQUIRED_WORKSPACE_APP_IDS\.has\(app\.id\)[\s\S]*?return/u,
  'Workspace addApp must not overwrite required workbench entries such as notary',
);
assert.match(
  workspaceServiceSource,
  /async addApp\(app: AppItem\)[\s\S]*storedApps = readStoredApps\(\)\.filter\([\s\S]*item\.id !== app\.id[\s\S]*storedApps\.push\(app\)/u,
  'Workspace addApp must persist optional app entries in local storage without duplicating app ids',
);
assert.match(
  workspaceViewSource,
  /onAppSelect\(app\.id\)/u,
  'Workspace app tiles must pass the selected app id to the shell for direct navigation',
);
assert.doesNotMatch(
  `${workspaceServiceSource}\n${workspaceViewSource}`,
  /notaryAccessService|canUseNotary|canShowNotaryMenu/u,
  'Workspace notary entry and navigation must not be hidden behind notary access state',
);
assert.doesNotMatch(
  `${chatLayoutSource}\n${capabilityModuleSurfaceSource}`,
  /notaryAccessService\.canUseNotary/u,
  'Chat shell surfaces must not use notary access state to block sidebar or workspace navigation',
);
assert.match(
  chatLayoutSource,
  /subscribePendingFriendRequestCount/u,
  'ChatLayout must subscribe to pending friend request counts for the contacts red dot',
);
assert.match(
  chatLayoutSource,
  /friendRequestUnreadCount=\{friendRequestUnreadCount\}/u,
  'ChatLayout must pass the pending friend request count into Sidebar',
);
assert.match(
  sidebarSource,
  /friendRequestUnreadCount\?:\s*number/u,
  'Sidebar must accept the pending friend request count without owning SDK calls',
);
assert.match(
  sidebarSource,
  /modId\s*===\s*["']contacts["'][\s\S]*friendRequestUnreadCount\s*>\s*0[\s\S]*bg-red-500/u,
  'Sidebar contacts icon must reuse the existing red badge style for pending friend requests',
);
assert.match(
  newFriendsContainerSource,
  /subscribePendingFriendRequestCount/u,
  'NewFriendsContainer must refresh when friend request count changes so accepted/rejected requests disappear promptly',
);
assert.match(
  contactServiceSource,
  /startPendingFriendRequestRealtime/u,
  'ContactService must start realtime friend request subscriptions when red dot listeners are active',
);
assert.match(
  contactServiceSource,
  /friend_request\.submitted/u,
  'ContactService must listen for submitted friend request realtime events instead of relying only on polling',
);
assert.match(
  toastSource,
  /placement\?:\s*['"]top['"]\s*\|\s*['"]bottom-right['"]/u,
  'Toast API must support an optional bottom-right placement for desktop notification-style reminders',
);
assert.match(
  toastSource,
  /bottom-6\s+right-6/u,
  'ToastContainer must render bottom-right notifications as an overlay without changing the main layout',
);
assert.match(
  chatLayoutSource,
  /previousFriendRequestUnreadCountRef/u,
  'ChatLayout must track the previous friend request count so first load does not spam notifications',
);
assert.match(
  chatLayoutSource,
  /toast\([^)]*friendRequest[^)]*\{[^}]*placement:\s*["']bottom-right["']/u,
  'ChatLayout must show a bottom-right notification when pending friend requests increase',
);
assert.match(
  messageListSource,
  /formatVideoCallMessageContent/u,
  'MessageList must format RTC call content through a display-name resolver instead of rendering raw participant ids',
);
assert.match(
  messageListSource,
  /resolveDisplayName/u,
  'MessageList must resolve RTC call participant ids to current-user/contact display names before rendering',
);
assert.match(
  chatListSource,
  /formatChatListLastMessage/u,
  'ChatList must format RTC call previews before rendering the last message so raw participant ids are not shown in the conversation list',
);
assert.match(
  chatListSource,
  /replaceRtcPreviewParticipantId/u,
  'ChatList RTC call previews must replace known participant ids with the chat display name without changing the sidebar layout',
);

console.log('sdkwork-im-pc sidebar modules refresh contract passed');
