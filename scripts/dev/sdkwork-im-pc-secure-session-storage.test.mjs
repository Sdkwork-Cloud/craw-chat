import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const sessionSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/session.ts');
const secureStorageSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/secureSessionStorage.ts');
const authGateSource = read('apps/sdkwork-im-pc/src/AuthGate.tsx');
const sessionStoreSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop/src-tauri/src/session_store.rs',
);
const libSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop/src-tauri/src/lib.rs');
const desktopCargo = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop/src-tauri/Cargo.toml');

assert.match(
  secureStorageSource,
  /sdkwork_im_pc_session_read/u,
  'desktop secure session bridge must invoke sdkwork_im_pc_session_read',
);
assert.match(
  secureStorageSource,
  /sdkwork_im_pc_session_write/u,
  'desktop secure session bridge must invoke sdkwork_im_pc_session_write',
);
assert.match(
  secureStorageSource,
  /sdkwork_im_pc_session_clear/u,
  'desktop secure session bridge must invoke sdkwork_im_pc_session_clear',
);

assert.match(
  sessionSource,
  /hydrateAppSdkSessionFromSecureStorage/u,
  'session module must expose secure-storage hydration for desktop bootstrap',
);
assert.match(
  sessionSource,
  /removePersistedSessionRawValueFromWebStorageOnly/u,
  'desktop session migration must purge web storage after keyring persist',
);
assert.match(
  sessionSource,
  /handleDesktopSessionPersistError/u,
  'desktop session persist failures must invalidate cache and log errors',
);
assert.doesNotMatch(
  sessionSource,
  /writeDesktopSecureSessionRawValue\(value\)\.catch\(\(\) => \{\}\)/u,
  'desktop session persist must not swallow secure-storage write failures silently',
);

assert.match(
  authGateSource,
  /await hydrateAppSdkSessionFromSecureStorage\(\)/u,
  'AuthGate must hydrate desktop secure session before auth bootstrap',
);

assert.match(
  sessionStoreSource,
  /keyring::Entry/u,
  'desktop session store must use OS keyring instead of webview storage',
);
assert.match(
  sessionStoreSource,
  /set_password/u,
  'desktop session store must persist tokens through keyring set_password',
);
assert.match(
  sessionStoreSource,
  /migrate_legacy_store_value/u,
  'desktop session store must migrate legacy plugin-store files into keyring',
);
assert.doesNotMatch(
  sessionStoreSource,
  /tauri_plugin_store/u,
  'desktop session store must not keep tauri-plugin-store persistence',
);

assert.match(
  libSource,
  /session_store::sdkwork_im_pc_session_read/u,
  'Tauri invoke handler must expose secure session read command',
);
assert.doesNotMatch(
  libSource,
  /tauri_plugin_store/u,
  'desktop host must not register tauri-plugin-store for auth tokens',
);

assert.match(
  desktopCargo,
  /^keyring = "2"/mu,
  'desktop host must depend on keyring for native secure session storage',
);
assert.doesNotMatch(
  desktopCargo,
  /tauri-plugin-store/u,
  'desktop host must not depend on tauri-plugin-store for auth tokens',
);

console.log('sdkwork im pc secure session storage contract passed.');
