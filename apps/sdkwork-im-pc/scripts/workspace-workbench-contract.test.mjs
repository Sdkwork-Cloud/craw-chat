import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const appRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');

function read(relativePath) {
  return fs.readFileSync(path.join(appRoot, relativePath), 'utf8');
}

const entrySource = read('packages/sdkwork-im-pc-workspace/src/index.tsx');
const workspaceViewPath = path.join(
  appRoot,
  'packages/sdkwork-im-pc-workspace/src/WorkspaceView.tsx',
);
const source = fs.existsSync(workspaceViewPath)
  ? fs.readFileSync(workspaceViewPath, 'utf8')
  : entrySource;
const shortcutDialogPath = path.join(
  appRoot,
  'packages/sdkwork-im-pc-workspace/src/components/WorkspaceShortcutDialog.tsx',
);
const shortcutDialogSource = fs.existsSync(shortcutDialogPath)
  ? fs.readFileSync(shortcutDialogPath, 'utf8')
  : '';
const uiSource = `${source}\n${shortcutDialogSource}`;
const serviceSource = read('packages/sdkwork-im-pc-workspace/src/services/WorkspaceService.ts');
const moduleRenderHostSource = read('packages/sdkwork-im-pc-shell/src/ModuleRenderHost.tsx');
const appErrorBoundarySource = read('packages/sdkwork-im-pc-commons/src/components/AppErrorBoundary.tsx');
const chatLayoutSource = read('packages/sdkwork-im-pc-chat/src/pages/ChatLayout.tsx');
const capabilityModuleSurfaceSource = read(
  'packages/sdkwork-im-pc-chat/src/surfaces/CapabilityModuleSurface.tsx',
);
const capabilityModuleLoadersSource = read(
  'packages/sdkwork-im-pc-shell/src/capabilityModuleLoaders.ts',
);
const drivePcStubSource = read('types/stubs/drive-pc-drive.d.ts');
const componentSpec = JSON.parse(
  read('packages/sdkwork-im-pc-workspace/specs/component.spec.json'),
);

assert.match(source, /searchQuery/u, 'workbench must keep controlled search state');
assert.doesNotMatch(entrySource, /useState|workspaceService/u, 'workspace public entrypoint must remain a thin export');
assert.match(source, /onAppSelect\(/u, 'workbench must expose app navigation through its host callback');
assert.match(uiSource, /role=["']dialog["']/u, 'shortcut management must be an accessible dialog');
assert.match(uiSource, /aria-modal=["']true["']/u, 'shortcut management dialog must be modal to assistive technology');
assert.match(source, /aria-busy/u, 'workbench loading state must be announced');
assert.match(source, /retry/u, 'workbench must expose a retry action for load failures');
assert.match(source, /savePinnedAppIds/u, 'workbench must persist shortcut management changes through the service');
assert.match(source, /getWorkspaceData/u, 'workbench must consume explicit remote or fallback data status');
assert.match(source, /fallback/u, 'workbench must surface degraded data status');
assert.match(uiSource, /keydown|onKeyDown/u, 'workbench search/dialog interactions must support keyboard input');
assert.match(shortcutDialogSource, /event\.key === ['"]Tab['"]/u, 'shortcut dialog must contain keyboard focus');
assert.doesNotMatch(source, /about:blank\?doc=/u, 'workbench must not open a fake about:blank document URL');
assert.doesNotMatch(source, /t\(['"]openDocument['"]/u, 'workbench must not claim direct document navigation without a Drive handoff contract');
assert.match(source, /onDocumentOpen/u, 'workbench must expose document targets through a host callback');
assert.match(source, /doc\.openTarget/u, 'workbench document clicks must use the service-provided open target');
assert.match(
  chatLayoutSource,
  /driveOpenRequestSequenceRef\.current \+= 1/u,
  'the persistent chat host must assign monotonically increasing Drive open request ids',
);
assert.match(
  chatLayoutSource,
  /section:\s*["']recent["']/u,
  'workspace Drive handoff must open the recent section',
);
assert.match(
  chatLayoutSource,
  /intent:\s*["']preview["']/u,
  'workspace Drive handoff must request a node preview',
);
assert.match(
  chatLayoutSource,
  /nodeId:\s*target\.resourceId/u,
  'workspace Drive handoff must preserve the stable node id',
);
assert.match(
  capabilityModuleSurfaceSource,
  /openRequest=\{driveOpenRequest\}/u,
  'the Drive capability module must receive the typed open request',
);
assert.match(
  chatLayoutSource,
  /driveOpenRequest=\{driveOpenRequest\}/u,
  'the pending Drive request must be stored above the active-tab-keyed module boundary',
);
assert.doesNotMatch(
  `${chatLayoutSource}\n${capabilityModuleSurfaceSource}`,
  /signedSourceUrl|downloadUrl/u,
  'the host handoff must not carry temporary Drive URLs',
);
assert.match(
  capabilityModuleLoadersSource,
  /DriveOpenRequest/u,
  'the IM shell must re-export the Drive-owned open request type',
);
assert.match(drivePcStubSource, /interface DriveOpenRequest/u, 'the app type stub must mirror the Drive host contract');

assert.match(serviceSource, /WORKSPACE_PINNED_APPS_STORAGE_KEY/u, 'workspace shortcut preferences need a dedicated storage key');
assert.match(serviceSource, /savePinnedAppIds/u, 'workspace service must expose pinned shortcut persistence');
assert.match(serviceSource, /pinned:/u, 'workspace app view models must expose pinned state');
assert.match(serviceSource, /permission-denied/u, 'workspace data status must distinguish permission denial');
assert.match(serviceSource, /resolveAppSdkTenantId/u, 'workspace persistence must include tenant scope');
assert.match(serviceSource, /resolveAppSdkUserId/u, 'workspace persistence must include user scope');

assert.match(
  moduleRenderHostSource,
  /<AppErrorBoundary[^>]*key=\{activeTab\}/u,
  'module errors must reset when the user changes the active tab',
);
assert.match(
  chatLayoutSource,
  /errorFallback=\{/u,
  'the client host must provide a localized module error fallback',
);
assert.doesNotMatch(
  appErrorBoundarySource,
  /this\.state\.error\.message/u,
  'the default error boundary must not expose internal exception messages',
);
assert.equal(componentSpec.component.surface, 'app', 'workspace package must declare its app-side surface');
assert.equal(
  componentSpec.contracts.layerRole,
  'frontend-feature',
  'workspace package must declare its frontend feature layer role',
);
assert.deepEqual(
  componentSpec.contracts.sdkClients,
  [],
  'workspace feature must not claim ownership of generated SDK clients',
);

console.log('sdkwork-im-pc workspace workbench contract passed');
