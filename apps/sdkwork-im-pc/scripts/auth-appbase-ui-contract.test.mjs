#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { iamSourcePathRegex } from '../../../../sdkwork-specs/tools/iam-legacy-path-fragments.mjs';

const appRoot = path.resolve(import.meta.dirname, '..');

function readText(...segments) {
  return fs.readFileSync(path.join(appRoot, ...segments), 'utf8');
}

function readJson(...segments) {
  return JSON.parse(readText(...segments));
}

function extractObjectPropertyBlock(source, propertyName) {
  const propertyMatch = new RegExp(`\\b${propertyName}\\s*:\\s*\\{`, 'u').exec(source);
  assert.ok(propertyMatch, `Expected object property ${propertyName} in source.`);

  const openBraceIndex = source.indexOf('{', propertyMatch.index);
  let depth = 0;
  for (let index = openBraceIndex; index < source.length; index += 1) {
    const character = source[index];
    if (character === '{') {
      depth += 1;
    } else if (character === '}') {
      depth -= 1;
      if (depth === 0) {
        return source.slice(openBraceIndex, index + 1);
      }
    }
  }

  throw new Error(`Could not find closing brace for object property ${propertyName}.`);
}

const authGateSource = readText('src', 'AuthGate.tsx');
const appSource = readText('src', 'App.tsx');
const appRoutesSource = readText('src', 'bootstrap', 'routes.tsx');
const trayNavigationSource = readText('src', 'bootstrap', 'trayNavigation.ts');
const authStylesSource = readText('src', 'index.css');
const appAuthServiceSource = readText('packages', 'sdkwork-im-pc-core', 'src', 'sdk', 'appAuthService.ts');
const authRuntimeSource = readText('packages', 'sdkwork-im-pc-core', 'src', 'sdk', 'appAuthRuntime.ts');
const sessionSource = readText('packages', 'sdkwork-im-pc-core', 'src', 'sdk', 'session.ts');
const chatServiceSource = readText('packages', 'sdkwork-im-pc-chat', 'src', 'services', 'ChatService.ts');
const chatLayoutSource = readText('packages', 'sdkwork-im-pc-chat', 'src', 'pages', 'ChatLayout.tsx');
const appShellFrameSource = readText('packages', 'sdkwork-im-pc-shell', 'src', 'AppShellFrame.tsx');
const sidebarSource = readText('packages', 'sdkwork-im-pc-chat', 'src', 'components', 'Sidebar.tsx');
const profileMenuSource = readText('packages', 'sdkwork-im-pc-chat', 'src', 'components', 'ProfileMenuModal.tsx');
const settingsModalSource = readText('packages', 'sdkwork-im-pc-chat', 'src', 'components', 'SettingsModal.tsx');
const chatWindowControlsSource = readText('packages', 'sdkwork-im-pc-chat', 'src', 'components', 'WindowControls.tsx');
const desktopRustSource = readText('packages', 'sdkwork-im-pc-desktop', 'src-tauri', 'src', 'lib.rs');
const desktopWindowControlRustSource = readText('packages', 'sdkwork-im-pc-desktop', 'src-tauri', 'src', 'window_control.rs');
const desktopTrayRustSource = readText('packages', 'sdkwork-im-pc-desktop', 'src-tauri', 'src', 'tray.rs');
const desktopRustShellSource = [
  desktopRustSource,
  desktopWindowControlRustSource,
  desktopTrayRustSource,
].join('\n');
const tauriWindowControlPermissionSource = readText('packages', 'sdkwork-im-pc-desktop', 'src-tauri', 'permissions', 'window-control.toml');
const tauriConfig = readJson('packages', 'sdkwork-im-pc-desktop', 'src-tauri', 'tauri.conf.json');
const tauriDefaultCapability = readJson('packages', 'sdkwork-im-pc-desktop', 'src-tauri', 'capabilities', 'default.json');
const viteConfigSource = readText('vite.config.ts');
const pnpmWorkspaceSource = readText('pnpm-workspace.yaml');
const tsconfig = readJson('tsconfig.json');
const packageJson = readJson('package.json');

function assertWorkspaceDependency(dependencyName) {
  assert.equal(
    packageJson.dependencies?.[dependencyName],
    'workspace:*',
    `package.json must consume ${dependencyName} through workspace:*; the canonical source path belongs in pnpm-workspace.yaml packages.`,
  );
}

function assertWorkspacePackagePath(pattern, message) {
  assert.match(pnpmWorkspaceSource, pattern, message);
}

assert.match(
  authGateSource,
  /import\s+\{\s*SdkworkIamAuthRoutes[\s\S]*\}\s+from\s+['"]@sdkwork\/auth-pc-react['"]/u,
  'AuthGate must render the SDKWork Appbase PC React IAM auth routes.',
);

assert.match(
  authGateSource,
  /<SdkworkIamAuthRoutes[\s\S]*basePath=["']\/auth["'][\s\S]*homePath=["']\/["']/u,
  'AuthGate must mount the appbase auth routes at /auth and return to the current chat PC home path.',
);

assert.match(
  authGateSource,
  /getRuntime=\{getSdkworkChatIamRuntime\}/u,
  'AuthGate must inject the sdkwork-chat generated-SDK backed IAM runtime.',
);

assert.match(
  authGateSource,
  /runtimeConfig=\{resolveSdkworkChatAuthRuntimeConfig\(\)\}/u,
  'AuthGate must pass the sdkwork-chat auth runtime config into the appbase auth UI.',
);

assert.match(
  authGateSource,
  /appearance=\{resolveSdkworkChatAuthAppearance\(\)\}/u,
  'AuthGate must pass a product-owned appearance override so login and verification backgrounds are controlled by the current app.',
);

assert.match(
  authGateSource,
  /viewportMode=["']fixed["']/u,
  'AuthGate must render appbase auth routes in fixed mode inside the desktop login shell, aligned with sdkwork-knowledgebase.',
);

for (const marker of [
  'SdkworkChatAuthShell',
  'Sdkwork IM',
  'MessageSquare',
  'Sun',
  'Moon',
  'isLightMode',
  'toggleAuthTheme',
  'data-tauri-drag-region',
  'drag-region',
  'no-drag',
  'data-no-drag',
  'WindowControlMinimizeIcon',
  'WindowControlMaximizeIcon',
  'WindowControlRestoreIcon',
  'X',
  'handleWindowControl',
  'invokeDesktopWindowControl',
  'sdkwork_chat_pc_window_control',
  'startDragging',
]) {
  assert.match(
    authGateSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
    `AuthGate must include login app header/window control marker ${marker}`,
  );
}

assert.doesNotMatch(
  authGateSource,
  /\/auth\/qr-login/u,
  'QR login must stay inside the standard /auth/login page instead of introducing a separate default QR login route.',
);

assert.doesNotMatch(
  authGateSource,
  /<form\b|appAuthService\.login|appAuthService\.register|appAuthService\.sendVerifyCode/u,
  'AuthGate must not keep a bespoke login/register form or direct auth service calls once appbase auth UI is mounted.',
);

assert.doesNotMatch(
  authGateSource,
  /appWindow\.close\?\(/u,
  'AuthGate close button must not directly close the Tauri window; desktop close must hide to tray through the native command.',
);

for (const marker of [
  'invokeDesktopWindowControl',
  'sdkwork_chat_pc_window_control',
  'closeToTray',
  'minimize',
  'toggleMaximize',
]) {
  assert.match(
    chatWindowControlsSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
    `Main chat AppHeader window controls must use native desktop command marker ${marker}`,
  );
}

assert.match(
  appShellFrameSource,
  /import\s+\{[\s\S]*isSdkworkChatDesktopRuntime[\s\S]*\}\s+from\s+['"]@sdkwork\/im-pc-core['"]/u,
  'AppShellFrame must import the shared desktop-runtime guard instead of deciding web/desktop locally.',
);

assert.match(
  appShellFrameSource,
  /const\s+shouldRenderDesktopAppHeader\s*=\s*isSdkworkChatDesktopRuntime\(\)/u,
  'AppShellFrame must compute AppHeader visibility from the shared desktop-runtime guard.',
);

assert.match(
  appShellFrameSource,
  /\{shouldRenderDesktopAppHeader\s*&&\s*desktopTitleBar\}/u,
  'AppShellFrame must render the native desktop title bar only in the desktop runtime.',
);

assert.match(
  chatLayoutSource,
  /desktopTitleBar=\{[\s\S]*?<WindowControls\s*\/>[\s\S]*?\}/u,
  'ChatLayout must pass native WindowControls through the shell desktop title bar slot.',
);

assert.match(
  authGateSource,
  /import\s+\{[\s\S]*isSdkworkChatDesktopRuntime[\s\S]*\}\s+from\s+['"]@sdkwork\/im-pc-core['"]/u,
  'AuthGate must import the shared desktop-runtime guard so web login does not show the desktop AppHeader.',
);

assert.match(
  authGateSource,
  /const\s+shouldRenderDesktopAppHeader\s*=\s*isSdkworkChatDesktopRuntime\(\)/u,
  'AuthGate must compute auth AppHeader visibility from the shared desktop-runtime guard.',
);

assert.match(
  authGateSource,
  /\{shouldRenderDesktopAppHeader\s*&&\s*\([\s\S]*?<header[\s\S]*?sdkwork-chat-auth-header[\s\S]*?<\/header>[\s\S]*?\)\}/u,
  'AuthGate must render the auth AppHeader only in the desktop runtime.',
);

assert.doesNotMatch(
  chatWindowControlsSource,
  /window\.close\(|requestFullscreen\(|exitFullscreen\(/u,
  'Main chat AppHeader must not use browser close/fullscreen APIs for desktop window controls; it must use the Tauri native command.',
);

assert.match(
  authRuntimeSource,
  /export function getSdkworkChatIamRuntime\(\)/u,
  'pc-core must expose a reusable Sdkwork IM IAM runtime for appbase auth routes.',
);

assert.match(
  readText('packages', 'sdkwork-im-pc-core', 'src', 'index.ts'),
  /export \* from ['"]\.\/runtime\/desktopEnvironment['"]/u,
  'pc-core must export the shared desktop runtime guard for auth and chat shells.',
);

const desktopEnvironmentSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'runtime',
  'desktopEnvironment.ts',
);

assert.match(
  desktopEnvironmentSource,
  /export function isSdkworkChatDesktopRuntime\(\): boolean/u,
  'pc-core must define a shared isSdkworkChatDesktopRuntime guard.',
);

assert.match(
  desktopEnvironmentSource,
  /__TAURI__[\s\S]*core[\s\S]*invoke/u,
  'The desktop runtime guard must detect the Tauri invoke bridge used by Sdkwork IM window controls.',
);

assert.match(
  desktopEnvironmentSource,
  /window[\s\S]*getCurrentWindow|window[\s\S]*appWindow/u,
  'The desktop runtime guard must also recognize the Tauri window bridge.',
);

assert.match(
  authRuntimeSource,
  /SDKWORK_IM_VERIFICATION_POLICY[\s\S]*emailCodeLoginEnabled:\s*false[\s\S]*emailRegistrationVerificationRequired:\s*false[\s\S]*phoneCodeLoginEnabled:\s*false[\s\S]*phoneRegistrationVerificationRequired:\s*false[\s\S]*verificationPolicy:\s*SDKWORK_IM_VERIFICATION_POLICY/u,
  'Sdkwork IM IAM runtime must disable email/phone code login and registration verification codes by default.',
);

assert.match(
  authRuntimeSource,
  /loginMethods:\s*\[\s*['"]password['"]\s*\]/u,
  'Sdkwork IM IAM runtime must default to password login only.',
);

assert.doesNotMatch(
  authRuntimeSource,
  /loginMethods:\s*\[[^\]]*(?:email-code|phone-code)/u,
  'Sdkwork IM IAM runtime must not enable verification-code login by default.',
);

assert.match(
  authRuntimeSource,
  /leftRailMode:\s*['"]qr-only['"][\s\S]*qrLoginEnabled:\s*true/u,
  'Sdkwork IM IAM runtime must render the appbase QR scan panel inside the normal login page rail.',
);

assert.match(
  authRuntimeSource,
  /recoveryMethods:\s*\[\s*\]/u,
  'Sdkwork IM IAM runtime must hide forgot-password email/phone verification-code recovery by default.',
);

assert.match(
  authRuntimeSource,
  /createSdkworkAppbasePcAuthRuntime/u,
  'Sdkwork IM IAM runtime must use the standard appbase PC auth runtime factory.',
);

assert.match(
  appAuthServiceSource,
  /import\s+\{[\s\S]*createSdkworkAuthAppbaseIntegration[\s\S]*\}\s+from\s+['"]@sdkwork\/auth-pc-react['"]/u,
  'Sdkwork IM auth service must consume the high-level appbase auth integration factory from @sdkwork/auth-pc-react.',
);

assert.match(
  appAuthServiceSource,
  /createSdkworkAuthAppbaseIntegration\(\s*\{[\s\S]*app:\s*\{[\s\S]*id:\s*['"]sdkwork-im-pc['"][\s\S]*title:\s*['"]Sdkwork IM PC['"][\s\S]*basePath:\s*['"]\/auth['"][\s\S]*extraPackageNames:\s*\[\s*['"]@sdkwork\/im-pc-react['"],?\s*\]/u,
  'Sdkwork IM auth service must provide only app identity, auth base path, and product extra packages to the high-level auth integration factory.',
);

assert.match(
  appAuthServiceSource,
  /function\s+isDuplicateCapabilityPackageError\([\s\S]*Duplicate capability package[\s\S]*\n\}/u,
  'Sdkwork IM auth service must detect duplicate capability package startup failures from appbase capability registry construction.',
);

assert.match(
  appAuthServiceSource,
  /try\s*\{[\s\S]*createSdkworkAuthAppbaseIntegration\([\s\S]*extraPackageNames:\s*\[[\s\S]*@sdkwork\/im-pc-react[\s\S]*\][\s\S]*\}\)[\s\S]*\}\s*catch\s*\(error\)\s*\{[\s\S]*isDuplicateCapabilityPackageError\(error\)[\s\S]*createSdkworkAuthAppbaseIntegration\([\s\S]*app:\s*\{[\s\S]*id:\s*['"]sdkwork-im-pc['"][\s\S]*title:\s*['"]Sdkwork IM PC['"][\s\S]*\}[\s\S]*basePath:\s*['"]\/auth['"][\s\S]*\}\)[\s\S]*throw\s+error[\s\S]*\}/u,
  'Sdkwork IM auth service must gracefully retry auth integration creation without extra capability packages when upstream package catalogs already include shell-level capabilities.',
);

assert.doesNotMatch(
  appAuthServiceSource,
  /createSdkworkAppCapabilityPresetManifest|createAuthRouteCatalog|appbasePackageMeta|authPackageMeta|@sdkwork\/appbase-pc-react|@sdkwork\/auth-pc-react\/auth/u,
  'Sdkwork IM auth service must not hand-build appbase/auth package metadata or auth route catalogs.',
);

assert.match(
  authRuntimeSource,
  /from\s+['"]@sdkwork\/auth-runtime-pc-react['"]/u,
  'Sdkwork IM IAM runtime must consume the high-level appbase auth runtime package.',
);

assert.match(
  authRuntimeSource,
  /appbaseAppApiBaseUrl:\s*resolveAppSdkBaseUrl\(\)/u,
  'Sdkwork IM IAM runtime must pass the appbase app base URL into the standard factory.',
);

assert.doesNotMatch(
  authRuntimeSource,
  /appbaseBackendApiBaseUrl|resolveBackendSdkBaseUrl|getBackendSdkClient|resetBackendSdkClient|resetAppbaseBackendSdkClient/u,
  'Sdkwork IM app auth runtime must not construct or reset backend SDK clients; backend SDKs are exported through the admin surface only.',
);

assert.match(
  authRuntimeSource,
  /sdkClients:\s*getAuthenticatedSdkClients\(\)[\s\S]*tokenManager:\s*getSdkworkChatGlobalTokenManager\(\)/u,
  'Sdkwork IM IAM runtime must inject downstream SDK clients and the shared global token manager through the appbase factory.',
);

assert.match(
  authRuntimeSource,
  /sessionBridge:\s*\{[\s\S]*clearSession:\s*clearSdkworkChatIamRuntimeSession[\s\S]*commitSession:\s*\(session\)\s*=>\s*applyAppSdkSessionTokens\(session\s+as\s+SdkworkChatSession\)[\s\S]*readSession:\s*readAppSdkSessionTokens/u,
  'Sdkwork IM IAM runtime must supply only app-owned read/commit/clear session hooks to the appbase factory.',
);

assert.doesNotMatch(
  authRuntimeSource,
  /tokenStore:\s*\{|contextStore:\s*|createIamShardingContext|IamAppContext|SdkworkChatAppContext|readStoredRuntimeSession|persistRuntimeTokenStoreSession|persistRuntimeAppContext|createSdkworkChatIamContextStore/u,
  'Sdkwork IM IAM runtime must not hand-build appbase tokenStore/contextStore or sharding context after sessionBridge migration.',
);

assert.doesNotMatch(
  `${authRuntimeSource}\n${appAuthServiceSource}`,
  /@sdkwork\/iam-sdk-adapter|createIamAppSdkAdapter|unwrapIamSdkResponse|getIam\(\)/u,
  'Sdkwork IM auth integration must not import or call the lower-level IAM SDK adapter after moving to appbase high-level auth runtime.',
);

assert.doesNotMatch(
  appAuthServiceSource,
  /login\s*\(|register\s*\(|refreshToken\s*\(|sendVerifyCode|verifyCode|createQrAuth/u,
  'Sdkwork IM auth service must not expose product-local login, registration, refresh, verification-code, or QR auth methods.',
);

assert.doesNotMatch(
  `${authRuntimeSource}\n${appAuthServiceSource}`,
  /(?:service\.)?auth\.sessions\.create|(?:service\.)?auth\.registrations\.create|(?:service\.)?auth\.sessions\.refresh/u,
  'Sdkwork IM product code must not remap login, registration, or refresh calls; appbase auth runtime owns those flows.',
);

assert.match(
  appAuthServiceSource,
  /getCurrentSession\(\)[\s\S]*service\.auth\.sessions\.current\.retrieve/u,
  'Sdkwork IM auth service may only bridge current-session bootstrap through the appbase runtime service.',
);

assert.match(
  appAuthServiceSource,
  /logout\(\)[\s\S]*service\.auth\.sessions\.current\.delete/u,
  'Sdkwork IM auth service may only bridge logout through the appbase runtime service.',
);

assert.match(
  appAuthServiceSource,
  /catch\s*\(error\)\s*\{[\s\S]*isAuthSessionRejectedError\(error\)[\s\S]*clearSdkworkChatIamRuntimeSession\(\)[\s\S]*return\s+null[\s\S]*return\s+storedSession/u,
  'Sdkwork IM current-session bootstrap must clear rejected sessions and preserve locally authenticated sessions on transient failures.',
);

assert.doesNotMatch(
  appAuthServiceSource,
  /data\.accessToken\s*\?\?\s*data\.authToken|data\.authToken\s*\?\?\s*data\.accessToken/u,
  'Sdkwork IM auth service must not synthesize one SDKWork token from the other.',
);

assert.doesNotMatch(
  authRuntimeSource,
  /refreshToken:\s*session\.refreshToken\s*\?\?\s*existingSession\?\.refreshToken/u,
  'Sdkwork IM IAM runtime must not own refresh-token continuation rules; the appbase session bridge owns token merge semantics.',
);

assert.match(
  authGateSource,
  /readAppSdkSessionTokens/u,
  'AuthGate must re-read persisted IAM session state so logout navigation cannot reuse a stale in-memory session.',
);

assert.match(
  authGateSource,
  /isAuthenticatedSession\(\s*readAppSdkSessionTokens\(\)\s*\)/u,
  'AuthGate must treat persisted dual-token session storage as the authentication source of truth.',
);
assert.match(
  authGateSource,
  /bootstrapLocalDevGatewayDiscovery/u,
  'AuthGate must discover the local dev gateway before authenticated SDK clients bootstrap.',
);

assert.match(
  authGateSource,
  /SDKWORK_IM_SESSION_CHANGED_EVENT/u,
  'AuthGate must subscribe to the shared auth-session changed event so logout, refresh failure, and account switch update route state immediately.',
);

assert.match(
  authGateSource,
  /window\.addEventListener\(\s*SDKWORK_IM_SESSION_CHANGED_EVENT/u,
  'AuthGate must listen for app-owned session changes instead of waiting for a later route transition.',
);

assert.match(
  authGateSource,
  /function\s+isAuthenticatedSession\([\s\S]*return\s+isAppSdkSessionAuthenticated\(session\)/u,
  'AuthGate must use the centralized dual-token and expiry-aware authenticated session predicate.',
);

assert.match(
  authGateSource,
  /const\s+storedSession\s*=\s*readAppSdkSessionTokens\(\)[\s\S]*if\s*\(\s*!isAuthenticatedSession\(storedSession\)\s*\)[\s\S]*setSession\(null\)[\s\S]*setIsBootstrapped\(true\)[\s\S]*return\s+\(\)\s*=>/u,
  'AuthGate must not call appbase current-session validation when no complete local dual-token session exists.',
);

assert.match(
  appAuthServiceSource,
  /const\s+storedSession\s*=\s*readAppSdkSessionTokens\(\)[\s\S]*if\s*\(\s*!isAppSdkSessionAuthenticated\(storedSession\)\s*\)[\s\S]*clearSdkworkChatIamRuntimeSession\(\)[\s\S]*return\s+null/u,
  'Sdkwork IM auth service must clear missing, incomplete, or expired local sessions before calling appbase current-session validation.',
);

assert.match(
  sessionSource,
  /export function isAppSdkSessionExpired[\s\S]*Date\.now\(\)\s*>=\s*expiresAt/u,
  'Session storage must classify expired appbase IAM sessions locally before protected bootstrap calls.',
);

assert.match(
  sessionSource,
  /export function isAppSdkSessionAuthenticated[\s\S]*Boolean\(session\?\.authToken\s*&&\s*session\?\.accessToken\)\s*&&\s*!isAppSdkSessionExpired\(session\)/u,
  'Authenticated app sessions require both SDKWork tokens and a non-expired local session.',
);

assert.match(
  chatLayoutSource,
  /import\s+\{\s*useNavigate\s*\}\s+from\s+['"]react-router-dom['"]/u,
  'ChatLayout must use router navigation for logout instead of reloading the whole page.',
);

assert.match(
  chatLayoutSource,
  /import\s+\{[\s\S]*appAuthService[\s\S]*\}\s+from\s+['"]@sdkwork\/im-pc-core['"]/u,
  'ChatLayout logout must call the generated-SDK backed product auth service.',
);

assert.match(
  chatLayoutSource,
  /const\s+handleLogout\s*=\s*async\s*\(\)\s*=>\s*\{[\s\S]*await\s+appAuthService\.logout\(\)[\s\S]*navigate\(\s*["']\/auth\/login\?redirect=%2F["']\s*,\s*\{\s*replace:\s*true\s*\}\s*\)/u,
  'ChatLayout must clear the IAM session through appAuthService.logout and then replace the current route with the login screen.',
);

assert.match(
  chatLayoutSource,
  /<Sidebar[\s\S]*onLogout=\{handleLogout\}/u,
  'ChatLayout must pass the real logout handler into the profile menu path.',
);

assert.match(
  chatLayoutSource,
  /<SettingsModal[\s\S]*onLogout=\{handleLogout\}/u,
  'ChatLayout must pass the real logout handler into the settings modal path.',
);

assert.match(
  sessionSource,
  /export const SDKWORK_IM_SESSION_CHANGED_EVENT\s*=\s*['"]sdkwork-im-pc:auth-session-changed['"]/u,
  'Session storage must expose a stable auth-session changed event for realtime/session bridges.',
);

assert.match(
  sessionSource,
  /window\.dispatchEvent\(\s*new CustomEvent\(\s*SDKWORK_IM_SESSION_CHANGED_EVENT/u,
  'Session storage must dispatch the auth-session changed event whenever appbase commits or clears tokens.',
);

assert.match(
  chatServiceSource,
  /SDKWORK_IM_SESSION_CHANGED_EVENT/u,
  'ChatService must subscribe to auth-session changes from the shared session module.',
);

assert.match(
  chatServiceSource,
  /window\.addEventListener\(\s*SDKWORK_IM_SESSION_CHANGED_EVENT/u,
  'ChatService must listen for auth-session changes so stale realtime sockets do not keep reconnecting.',
);

assert.match(
  chatServiceSource,
  /closeAllLiveSubscriptions\(\s*['"]auth session changed['"]\s*\)/u,
  'ChatService must close all live subscriptions when appbase commits or clears an auth session.',
);

assert.match(
  chatServiceSource,
  /connection\?\.disconnect\(\s*1000,\s*reason\s*\)/u,
  'ChatService must disconnect live IM websocket connections with the auth-session change reason.',
);

assert.match(
  sidebarSource,
  /onLogout:\s*\(\)\s*=>\s*void\s*\|\s*Promise<void>/u,
  'Sidebar must accept the real logout handler and pass it to profile actions.',
);

assert.match(
  sidebarSource,
  /<ProfileMenuModal[\s\S]*onLogout=\{onLogout\}/u,
  'Sidebar must wire profile-menu logout to the real logout handler.',
);

for (const [source, label] of [
  [profileMenuSource, 'Profile menu'],
  [settingsModalSource, 'Settings modal'],
]) {
  assert.match(
    source,
    /onLogout:\s*\(\)\s*=>\s*void\s*\|\s*Promise<void>/u,
    `${label} must require a real logout handler instead of implementing app-local reload behavior.`,
  );
  assert.match(
    source,
    /onClick=\{\(\)\s*=>\s*\{[\s\S]*void\s+onLogout\(\);?[\s\S]*\}\}/u,
    `${label} logout button must invoke the real logout handler.`,
  );
  assert.doesNotMatch(
    source,
    /window\.location\.reload|location\.reload|setTimeout\([\s\S]*reload/u,
    `${label} logout must not rely on page reload because persisted tokens can keep the user authenticated.`,
  );
}

assert.doesNotMatch(
  authStylesSource,
  /sdkwork-chat-auth-shell\s*\{[\s\S]*?min-height:\s*100vh/u,
  'Auth shell must not use min-height: 100vh because the app header makes the login page overflow vertically.',
);

assert.match(
  authStylesSource,
  /sdkwork-chat-auth-shell\s*\{[\s\S]*?height:\s*100dvh[\s\S]*?overflow:\s*hidden/u,
  'Auth shell must lock to the viewport and hide page-level overflow so the login screen does not show a scrollbar.',
);

assert.match(
  authStylesSource,
  /sdkwork-chat-auth-shell\s*\{[\s\S]*?--sdkwork-chat-auth-bg:\s*#0c0d11[\s\S]*?--sdkwork-chat-auth-content-bg:\s*rgba\(17,\s*18,\s*24,\s*0\.96\)[\s\S]*?radial-gradient/u,
  'Auth shell must use the sdkwork-knowledgebase-aligned auth palette and gradient background.',
);

assert.match(
  authStylesSource,
  /sdkwork-chat-auth-main\s*\{[\s\S]*?overflow:\s*auto/u,
  'Auth main must allow scrolling like sdkwork-knowledgebase instead of clipping fixed auth routes.',
);

assert.doesNotMatch(
  authStylesSource,
  /sdkwork-chat-auth-background\s*>\s*\*\s*\{[\s\S]*?display:\s*none/u,
  'Auth background must not hide appbase decorative layers; sdkwork-knowledgebase keeps the default auth surface.',
);

const lightAuthShellBlock = authStylesSource.match(
  /html\.light-mode \.sdkwork-chat-auth-shell\s*\{(?<body>[\s\S]*?)\n\}/u,
)?.groups?.body ?? '';

assert.match(
  authRuntimeSource,
  /tabActiveBackgroundColor:\s*['"]transparent['"][\s\S]*tabBackgroundColor:\s*['"]transparent['"]/u,
  'Appbase auth tabs must stay transparent like sdkwork-knowledgebase instead of using filled tab rails.',
);

assert.match(
  authRuntimeSource,
  /asidePanelBackgroundColor:\s*['"]var\(--sdkwork-chat-auth-aside-bg\)['"]/u,
  'Appbase auth QR rail background must use the product-owned auth theme variable so light/dark mode can switch it.',
);

assert.match(
  authRuntimeSource,
  /asidePanelClassName:\s*['"]sdkwork-chat-auth-aside-panel['"]/u,
  'Appbase auth QR rail must receive a product-owned class so the app can tune nested QR panel utility colors without editing appbase.',
);

assert.match(
  authRuntimeSource,
  /qrFrameClassName:\s*['"]sdkwork-chat-auth-qr-frame['"]/u,
  'Appbase auth QR frame must receive a product-owned class for mode-specific QR frame polish.',
);

assert.match(
  authRuntimeSource,
  /asideCardBackgroundColor:\s*['"]var\(--sdkwork-chat-auth-aside-card-bg\)['"]/u,
  'Appbase auth QR rail cards must use the product-owned auth theme variable so light/dark mode can switch them.',
);

assert.doesNotMatch(
  authRuntimeSource,
  /asidePanelBackgroundColor:\s*['"]#101114['"]|asidePanelColor:\s*['"]#f5f5f5['"]/u,
  'Appbase auth appearance must not hard-code a dark QR rail because it prevents light mode from switching cleanly.',
);

assert.match(
  authStylesSource,
  /sdkwork-chat-auth-shell\s*\{[\s\S]*--sdkwork-chat-auth-aside-bg:/u,
  'Auth shell must define QR rail theme tokens next to the rest of the auth palette.',
);

assert.match(
  authStylesSource,
  /html\.light-mode \.sdkwork-chat-auth-aside-panel \.text-white\s*\{[\s\S]*color:\s*var\(--sdkwork-chat-auth-aside-text\)\s*!important/u,
  'Light auth mode must override nested QR rail text-white utility classes so QR rail text changes with the app theme.',
);

assert.match(
  authStylesSource,
  /html\.light-mode \.sdkwork-chat-auth-aside-panel \.text-zinc-300/u,
  'Light auth mode must override nested QR rail muted zinc utility classes so QR rail copy remains readable on the light surface.',
);

assert.match(
  lightAuthShellBlock,
  /--sdkwork-chat-auth-content-bg:\s*rgba\(255,\s*255,\s*255,\s*0\.96\)/u,
  'Light auth mode must use an opaque light card surface aligned with sdkwork-knowledgebase auth styling.',
);

assert.match(
  lightAuthShellBlock,
  /--sdkwork-chat-auth-qr-bg:\s*#fff(?:fff)?;/u,
  'Light auth mode must use a white QR frame background so the QR area switches away from the dark theme.',
);

assert.match(
  lightAuthShellBlock,
  /--sdkwork-chat-auth-aside-bg:\s*#f8fbff;/u,
  'Light auth mode must use a quiet commercial light-blue QR rail surface.',
);

assert.doesNotMatch(
  lightAuthShellBlock,
  /--sdkwork-chat-auth-qr-bg:\s*rgba\(17,\s*24,\s*39,\s*0\.88\)|--sdkwork-chat-auth-aside-bg:\s*#101114/u,
  'Light auth mode must not keep the dark slate QR frame or rail background.',
);

assert.equal(
  tauriConfig.app.windows[0].decorations,
  false,
  'Desktop login header expects the Tauri window to keep native decorations disabled.',
);

assert.equal(
  tauriConfig.app.withGlobalTauri,
  true,
  'Desktop AppHeader controls use the Tauri global invoke bridge so live Vite code can call the native window-control command without adding @tauri-apps/api to the web package.',
);

for (const permission of [
  'core:default',
  'shell:allow-open',
  'core:window:allow-start-dragging',
  'core:window:allow-is-maximized',
  'core:window:allow-minimize',
  'core:window:allow-maximize',
  'core:window:allow-toggle-maximize',
  'core:window:allow-unmaximize',
  'core:window:allow-close',
]) {
  assert.ok(
    tauriDefaultCapability.permissions.includes(permission),
    `Tauri default capability must grant ${permission} for the custom auth AppHeader window controls.`,
  );
}

assert.ok(
  tauriDefaultCapability.permissions.includes('allow-sdkwork-im-pc-window-control'),
  'Tauri default capability must grant the Sdkwork IM PC native window-control command.',
);

assert.match(
  tauriWindowControlPermissionSource,
  /identifier\s*=\s*["']allow-sdkwork-im-pc-window-control["'][\s\S]*commands\.allow\s*=\s*\[\s*["']sdkwork_chat_pc_window_control["']\s*\]/u,
  'Tauri custom app command permission must allow only the Sdkwork IM PC native window-control command.',
);

assert.match(
  desktopRustShellSource,
  /enum\s+SdkworkChatPcWindowControlAction[\s\S]*Minimize[\s\S]*ToggleMaximize[\s\S]*CloseToTray[\s\S]*Show/u,
  'Desktop Rust shell must define cross-platform minimize, maximize, close-to-tray, and show actions.',
);

assert.match(
  desktopRustShellSource,
  /#\[tauri::command\][\s\S]*sdkwork_chat_pc_window_control/u,
  'Desktop Rust shell must expose a Tauri command for AppHeader window controls.',
);

assert.match(
  desktopRustShellSource,
  /CloseToTray[\s\S]*window\.hide\(\)/u,
  'Desktop close action must hide the main window to tray instead of terminating the process.',
);

assert.match(
  desktopRustShellSource,
  /WindowEvent::CloseRequested[\s\S]*api\.prevent_close\(\)[\s\S]*window\.hide\(\)/u,
  'Desktop native window close requests must be intercepted and hidden to tray so the backend remains running.',
);

assert.match(
  desktopRustShellSource,
  /TrayIconBuilder[\s\S]*on_tray_icon_event[\s\S]*handle_tray_icon_event[\s\S]*fn\s+handle_tray_icon_event[\s\S]*show_main_window\(app\)/u,
  'Desktop shell must install a tray icon that can restore the hidden main window.',
);

assert.match(
  desktopRustShellSource,
  /use\s+std::sync::atomic::\{AtomicBool,\s*Ordering\}/u,
  'Desktop shell must keep an explicit exiting flag so tray Quit can bypass close-to-tray interception.',
);

assert.match(
  desktopRustShellSource,
  /static\s+IS_EXITING:\s*AtomicBool\s*=\s*AtomicBool::new\(false\)/u,
  'Desktop shell must initialize the exiting flag to false.',
);

assert.match(
  desktopRustShellSource,
  /const\s+TRAY_MENU_CHAT_ID:\s*&str\s*=\s*["']sdkwork_chat_pc_tray_chat["'][\s\S]*const\s+TRAY_MENU_CALL_ID:\s*&str\s*=\s*["']sdkwork_chat_pc_tray_call["'][\s\S]*const\s+TRAY_MENU_SETTINGS_ID:\s*&str\s*=\s*["']sdkwork_chat_pc_tray_settings["'][\s\S]*const\s+TRAY_MENU_QUIT_ID:\s*&str\s*=\s*["']sdkwork_chat_pc_tray_quit["']/u,
  'Desktop tray menu must define stable IDs for Chat, active call, Settings, and Quit.',
);

assert.match(
  desktopRustShellSource,
  /const\s+TRAY_EVENT_OPEN_SETTINGS:\s*&str\s*=\s*["']sdkwork-im-pc:\/\/tray\/open-settings["']/u,
  'Desktop tray settings action must emit a stable frontend event name.',
);

assert.match(
  desktopRustShellSource,
  /const\s+TRAY_EVENT_SHOW_ACTIVE_CALL:\s*&str\s*=\s*["']sdkwork-im-pc:\/\/tray\/show-active-call["']/u,
  'Desktop tray call action must emit a stable frontend event name.',
);

assert.match(
  desktopRustShellSource,
  /MenuBuilder::new\(app\)[\s\S]*\.text\(TRAY_MENU_CHAT_ID,\s*"\\u\{804a\}\\u\{5929\}"\)[\s\S]*\.text\(TRAY_MENU_CALL_ID,\s*"\\u\{663e\}\\u\{793a\}\\u\{901a\}\\u\{8bdd\}"\)[\s\S]*\.text\(TRAY_MENU_SETTINGS_ID,\s*"\\u\{8bbe\}\\u\{7f6e\}"\)[\s\S]*\.separator\(\)[\s\S]*\.text\(TRAY_MENU_QUIT_ID,\s*"\\u\{9000\}\\u\{51fa\}"\)[\s\S]*\.build\(\)/u,
  'Desktop tray menu must expose right-click entries: chat, active call, settings, and quit.',
);

assert.match(
  desktopRustShellSource,
  /TrayIconBuilder::with_id\(TRAY_ICON_ID\)[\s\S]*\.menu\(&menu\)[\s\S]*\.show_menu_on_left_click\(false\)[\s\S]*\.on_menu_event\(\|app,\s*event\|\s*handle_tray_menu_event\(app,\s*event\)\)/u,
  'Desktop tray icon must bind the native menu and handle menu events while keeping left click as window restore.',
);

assert.match(
  desktopRustShellSource,
  /fn\s+handle_tray_menu_event<R:\s*Runtime>\(app:\s*&AppHandle<R>,\s*event:\s*tauri::menu::MenuEvent\)[\s\S]*TRAY_MENU_CHAT_ID[\s\S]*show_main_window\(app\)[\s\S]*TRAY_MENU_CALL_ID[\s\S]*show_main_window\(app\)[\s\S]*emit\(TRAY_EVENT_SHOW_ACTIVE_CALL[\s\S]*TRAY_MENU_SETTINGS_ID[\s\S]*show_main_window\(app\)[\s\S]*emit\(TRAY_EVENT_OPEN_SETTINGS/u,
  'Desktop tray Chat, active call, and Settings actions must restore the main window before emitting frontend events.',
);

assert.match(
  desktopRustShellSource,
  /fn\s+quit_app<R:\s*Runtime>\(app:\s*&AppHandle<R>\)[\s\S]*IS_EXITING\.store\(true,\s*Ordering::SeqCst\)[\s\S]*main_window\(app\)[\s\S]*window\.close\(\)[\s\S]*app\.exit\(0\)/u,
  'Desktop tray Quit must set the exiting flag, close the main window, and exit the Tauri app process.',
);

assert.match(
  desktopRustShellSource,
  /WindowEvent::CloseRequested[\s\S]*IS_EXITING\.load\(Ordering::SeqCst\)[\s\S]*return;[\s\S]*api\.prevent_close\(\)[\s\S]*window\.hide\(\)/u,
  'Desktop native close requests must only hide to tray when the app is not executing the tray Quit action.',
);

assert.match(
  appRoutesSource,
  /useTauriTrayNavigationBridge/u,
  'App routes must install a Tauri tray navigation bridge at the router level.',
);

assert.match(
  trayNavigationSource,
  /const\s+TRAY_PENDING_SETTINGS_STORAGE_KEY\s*=\s*['"]sdkwork-im-pc:pending-open-settings['"]/u,
  'Tray Settings must persist a pending settings intent so it survives navigation from console/admin routes.',
);

assert.match(
  trayNavigationSource,
  /sdkwork-im-pc:\/\/tray\/open-chat[\s\S]*navigate\(['"]\/['"],\s*\{\s*replace:\s*false\s*\}\)/u,
  'Tray Chat action must navigate the frontend back to the main chat route.',
);

assert.match(
  trayNavigationSource,
  /sdkwork-im-pc:\/\/tray\/open-settings[\s\S]*sessionStorage\.setItem\(TRAY_PENDING_SETTINGS_STORAGE_KEY,\s*['"]1['"]\)[\s\S]*navigate\(['"]\/['"],\s*\{\s*replace:\s*false\s*\}\)[\s\S]*window\.dispatchEvent\(new\s+CustomEvent\(['"]sdkwork-im-pc:open-settings['"]\)\)/u,
  'Tray Settings action must record a pending settings intent, navigate to the chat route, and dispatch the in-app open-settings event.',
);

assert.match(
  chatLayoutSource,
  /window\.addEventListener\(['"]sdkwork-im-pc:open-settings['"],\s*openSettingsFromTray\)/u,
  'ChatLayout must listen for the tray settings event.',
);

assert.match(
  chatLayoutSource,
  /const\s+openSettingsFromTray\s*=\s*\(\)\s*=>\s*\{[\s\S]*setActiveTab\(["']chat["']\)[\s\S]*setIsSettingsOpen\(true\)[\s\S]*\}/u,
  'ChatLayout tray settings handler must switch back to chat and open the settings modal.',
);

assert.match(
  chatLayoutSource,
  /sessionStorage\.getItem\(['"]sdkwork-im-pc:pending-open-settings['"]\)[\s\S]*sessionStorage\.removeItem\(['"]sdkwork-im-pc:pending-open-settings['"]\)[\s\S]*openSettingsFromTray\(\)/u,
  'ChatLayout must consume the pending tray settings intent after route navigation mounts the chat layout.',
);

assert.match(
  viteConfigSource,
  /sdkwork-iam[\\/]apps[\\/]sdkwork-iam-pc[\\/]packages[\\/]sdkwork-auth-pc-react[\\/]src[\\/]index\.ts/u,
  'Vite must resolve @sdkwork/auth-pc-react from canonical sdkwork-iam so the verification-code register flow is available.',
);

assert.match(
  viteConfigSource,
  /sdkwork-appbase[\\/]packages[\\/]pc-react[\\/]foundation[\\/]sdkwork-i18n-pc-react[\\/]src[\\/]index\.ts/u,
  'Vite must resolve @sdkwork/i18n-pc-react for canonical appbase auth routes.',
);

assert.match(
  viteConfigSource,
  /sdkwork-ui[\\/]sdkwork-ui-pc-react[\\/]src[\\/]index\.ts/u,
  'Vite must resolve @sdkwork/ui-pc-react from source so appbase auth UI does not load browser-unsafe dist chunks.',
);

assert.match(
  viteConfigSource,
  /\^@sdkwork\\\/ui-pc-react\\\/\(\.\+\)\$/u,
  'Vite must resolve @sdkwork/ui-pc-react subpaths from source for appbase PC React components.',
);

assert.doesNotMatch(
  viteConfigSource,
  /sdkwork-ui[\\/]sdkwork-ui-pc-react[\\/]dist[\\/]index\.js/u,
  'Vite must not resolve @sdkwork/ui-pc-react to dist/index.js because that bundle can emit browser runtime require("react") calls.',
);

assert.match(
  viteConfigSource,
  /dedupe:\s*\[[\s\S]*['"]react['"][\s\S]*['"]react-dom['"]/u,
  'Vite must dedupe React packages when composing appbase and sdkwork UI source packages.',
);
assert.match(
  viteConfigSource,
  /reactRouterDomEntry[\s\S]*find:\s*(?:['"]react-router-dom['"]|\/\^react-router-dom\$\/)[\s\S]*replacement:\s*reactRouterDomEntry/u,
  'Vite must resolve react-router-dom from the chat PC dependency root for source-linked appbase auth routes.',
);
assert.match(
  viteConfigSource,
  /reactJsxRuntimeEntry[\s\S]*find:\s*['"]react\/jsx-runtime['"][\s\S]*replacement:\s*reactJsxRuntimeEntry/u,
  'Vite must resolve React JSX runtime from the chat PC dependency root for source-linked appbase packages.',
);

assert.match(
  viteConfigSource,
  iamSourcePathRegex('apps', 'sdkwork-iam-pc', 'packages', 'sdkwork-auth-runtime-pc-react', 'src', 'index.ts'),
  'Vite must resolve @sdkwork/auth-runtime-pc-react from canonical sdkwork-iam source.',
);

assert.doesNotMatch(
  viteConfigSource,
  /@sdkwork\/iam-sdk-adapter|sdkwork-iam-sdk-adapter/u,
  'Vite must not keep a product-side IAM SDK adapter alias after auth runtime migration.',
);

assert.match(
  viteConfigSource,
  iamSourcePathRegex('apps', 'sdkwork-iam-common', 'packages', 'sdkwork-iam-sdk-ports', 'src', 'index.ts'),
  'Vite must resolve @sdkwork/iam-sdk-ports from canonical sdkwork-iam source.',
);

assertWorkspaceDependency('@sdkwork/auth-pc-react');
assertWorkspacePackagePath(
  iamSourcePathRegex('apps', 'sdkwork-iam-pc', 'packages', 'sdkwork-auth-pc-react'),
  'pnpm-workspace.yaml must declare @sdkwork/auth-pc-react canonical sdkwork-iam source path.',
);

assertWorkspaceDependency('@sdkwork/auth-runtime-pc-react');
assertWorkspacePackagePath(
  iamSourcePathRegex('apps', 'sdkwork-iam-pc', 'packages', 'sdkwork-auth-runtime-pc-react'),
  'pnpm-workspace.yaml must declare @sdkwork/auth-runtime-pc-react canonical IAM auth runtime source path.',
);

assertWorkspaceDependency('@sdkwork/iam-app-sdk');
assertWorkspacePackagePath(
  iamSourcePathRegex('sdks', 'sdkwork-iam-app-sdk', 'sdkwork-iam-app-sdk-typescript', 'generated', 'server-openapi'),
  'pnpm-workspace.yaml must declare @sdkwork/iam-app-sdk canonical sdkwork-iam app SDK source path.',
);

assertWorkspaceDependency('@sdkwork/i18n-pc-react');
assertWorkspacePackagePath(
  /sdkwork-appbase[\\/]packages[\\/]pc-react[\\/]foundation[\\/]sdkwork-i18n-pc-react/u,
  'pnpm-workspace.yaml must declare @sdkwork/i18n-pc-react canonical appbase i18n PC React source path.',
);

assert.equal(
  packageJson.dependencies?.['@sdkwork/iam-sdk-adapter'],
  undefined,
  'package.json must not depend on the lower-level IAM SDK adapter after auth runtime migration.',
);

assertWorkspaceDependency('@sdkwork/iam-sdk-ports');
assertWorkspacePackagePath(
  iamSourcePathRegex('apps', 'sdkwork-iam-common', 'packages', 'sdkwork-iam-sdk-ports'),
  'pnpm-workspace.yaml must declare @sdkwork/iam-sdk-ports canonical IAM SDK ports source path.',
);

assert.match(
  String(tsconfig.compilerOptions?.paths?.['@sdkwork/auth-pc-react']?.[0] ?? ''),
  iamSourcePathRegex('apps', 'sdkwork-iam-pc', 'packages', 'sdkwork-auth-pc-react', 'src', 'index.ts'),
  'tsconfig must resolve @sdkwork/auth-pc-react from canonical sdkwork-iam.',
);

assert.match(
  String(tsconfig.compilerOptions?.paths?.['@sdkwork/auth-runtime-pc-react']?.[0] ?? ''),
  iamSourcePathRegex('apps', 'sdkwork-iam-pc', 'packages', 'sdkwork-auth-runtime-pc-react', 'src', 'index.ts'),
  'tsconfig must resolve @sdkwork/auth-runtime-pc-react from canonical sdkwork-iam.',
);

assert.match(
  String(tsconfig.compilerOptions?.paths?.['@sdkwork/iam-app-sdk']?.[0] ?? ''),
  iamSourcePathRegex(
    'sdks',
    'sdkwork-iam-app-sdk',
    'sdkwork-iam-app-sdk-typescript',
    'generated',
    'server-openapi',
    'src',
    'index.ts',
  ),
  'tsconfig must resolve @sdkwork/iam-app-sdk from canonical sdkwork-iam app SDK.',
);

assert.match(
  String(tsconfig.compilerOptions?.paths?.['@sdkwork/i18n-pc-react']?.[0] ?? ''),
  /sdkwork-appbase[\\/]packages[\\/]pc-react[\\/]foundation[\\/]sdkwork-i18n-pc-react[\\/]src[\\/]index\.ts/u,
  'tsconfig must resolve @sdkwork/i18n-pc-react for canonical appbase auth routes.',
);

assert.equal(
  tsconfig.compilerOptions?.paths?.['@sdkwork/iam-sdk-adapter'],
  undefined,
  'tsconfig must not resolve the lower-level IAM SDK adapter after auth runtime migration.',
);

assert.match(
  String(tsconfig.compilerOptions?.paths?.['@sdkwork/iam-sdk-ports']?.[0] ?? ''),
  iamSourcePathRegex('apps', 'sdkwork-iam-common', 'packages', 'sdkwork-iam-sdk-ports', 'src', 'index.ts'),
  'tsconfig must resolve @sdkwork/iam-sdk-ports from canonical sdkwork-iam.',
);

console.log('sdkwork im appbase auth UI contract passed.');
