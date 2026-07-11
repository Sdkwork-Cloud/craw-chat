import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');
const appRoot = path.join(repoRoot, 'apps/sdkwork-im-pc');
const desktopPackageRoot = path.join(appRoot, 'packages/sdkwork-im-pc-desktop');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8').replace(/\r\n/gu, '\n');
}

function readSibling(relativePath) {
  return fs.readFileSync(path.resolve(repoRoot, '..', relativePath), 'utf8').replace(/\r\n/gu, '\n');
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function mergeJson(relativePaths) {
  return Object.assign({}, ...relativePaths.map(readJson));
}

function readChatLocale(locale) {
  return mergeJson([
    `apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/i18n/${locale}/communication/im-pc-chat/sidebar.json`,
    `apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/i18n/${locale}/communication/im-pc-chat/agent.json`,
    `apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/i18n/${locale}/communication/im-pc-chat/profile.json`,
    `apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/i18n/${locale}/communication/im-pc-chat/contacts.json`,
    `apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/i18n/${locale}/communication/im-pc-chat/favorites.json`,
    `apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/i18n/${locale}/communication/im-pc-chat/settings-modal.json`,
    `apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/i18n/${locale}/communication/im-pc-chat/chat.json`,
    `apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/i18n/${locale}/communication/im-pc-chat/scan-qr.json`,
  ]);
}

function extractYamlSchemaBlock(source, schemaName) {
  const marker = `    ${schemaName}:\n`;
  const start = source.indexOf(marker);
  assert.notEqual(start, -1, `IM OpenAPI must define ${schemaName}`);

  const rest = source.slice(start + marker.length);
  const nextSchema = /\n    [A-Za-z0-9][A-Za-z0-9_]*:\n/u.exec(rest);
  const end = nextSchema ? start + marker.length + nextSchema.index : source.length;
  return source.slice(start, end);
}

function extractYamlOperationBlock(source, pathKey, method) {
  const pathMarker = `  ${pathKey}:\n`;
  const pathStart = source.indexOf(pathMarker);
  assert.notEqual(pathStart, -1, `IM OpenAPI must define path ${pathKey}`);

  const pathRest = source.slice(pathStart + pathMarker.length);
  const nextPath = /\n  \/[^\n]+:\n/u.exec(pathRest);
  const pathEnd = nextPath ? pathStart + pathMarker.length + nextPath.index : source.length;
  const pathBlock = source.slice(pathStart, pathEnd);
  const operationMarker = `    ${method}:\n`;
  const operationStart = pathBlock.indexOf(operationMarker);
  assert.notEqual(operationStart, -1, `IM OpenAPI path ${pathKey} must define ${method}`);

  const operationRest = pathBlock.slice(operationStart + operationMarker.length);
  const nextOperation = /\n    (get|put|post|delete|patch|options|head|trace):\n/u.exec(operationRest);
  const operationEnd = nextOperation
    ? operationStart + operationMarker.length + nextOperation.index
    : pathBlock.length;
  return pathBlock.slice(operationStart, operationEnd);
}

function assertFile(relativePath, message) {
  assert.ok(fs.existsSync(path.join(repoRoot, relativePath)), message ?? `${relativePath} must exist`);
}

function listFiles(rootRelativePath, predicate) {
  const root = path.join(repoRoot, rootRelativePath);
  const result = [];
  const walk = (dir) => {
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      const fullPath = path.join(dir, entry.name);
      if (entry.isDirectory()) {
        walk(fullPath);
        continue;
      }
      if (!predicate || predicate(fullPath)) {
        result.push(fullPath);
      }
    }
  };
  walk(root);
  return result;
}

function assertNoImDeviceApiUsage(source, label) {
  for (const forbidden of [
    /client\.device/u,
    /\.device\.registrations/u,
    /\.device\.syncFeed/u,
    /DeviceSyncFeedService/u,
    /retrieveDeviceSyncFeedWindow/u,
    /syncDeviceFeed/u,
    /syncContactsFromDeviceFeed/u,
    /syncGroupMembersFromDeviceFeed/u,
    /DeviceSyncFeed/u,
    /RegisterDevice/u,
    /RegisteredDevice/u,
    /DeviceSession/u,
    /\/im\/v3\/api\/device/u,
    /\/im\/v3\/api\/devices/u,
  ]) {
    assert.doesNotMatch(source, forbidden, `${label} must not consume retired IM device API surface: ${forbidden}`);
  }
}

const appPackageJson = readJson('apps/sdkwork-im-pc/package.json');
const chatEnUsMessages = readChatLocale('en-US');
const chatZhCnMessages = readChatLocale('zh-CN');
const runTauriCliSource = read('scripts/run-tauri-cli.mjs');
const retiredGenericAppSdkPackage = `@sdkwork/${'app'}-sdk`;
const retiredGenericBackendSdkPackage = `@sdkwork/${'backend'}-sdk`;

assert.equal(appPackageJson.name, '@sdkwork/im-pc', 'desktop app package must use a standard SDKWork package name');
assert.equal(
  appPackageJson.scripts.dev,
  'node ../../scripts/dev/run-sdkwork-im-pc-vite-dev.mjs',
);
assert.equal(appPackageJson.scripts['dev:tauri'], undefined);
assert.match(
  appPackageJson.scripts.build,
  /^node \.\.\/\.\.\/scripts\/dev\/run-vite-cli\.mjs build/u,
  'desktop app build must prepare linked SDKWork UI dependencies before Vite build',
);
assert.equal(
  appPackageJson.scripts.lint,
  'node ../../scripts/dev/run-tsc-cli.mjs --noEmit -p tsconfig.app.json',
  'desktop app lint must prepare linked SDKWork UI dependencies before TypeScript checks',
);
assert.equal(appPackageJson.scripts['dev:desktop'], 'pnpm --filter @sdkwork/im-pc-desktop dev:desktop');
assert.equal(appPackageJson.scripts['desktop:dev:local'], undefined);
assert.equal(appPackageJson.scripts['desktop:build:local'], undefined);
assert.equal(appPackageJson.scripts['build:desktop:local'], 'pnpm --filter @sdkwork/im-pc-desktop build:desktop:local');
assert.ok(
  appPackageJson.dependencies['@sdkwork/im-app-sdk'],
  'PC app must depend on the composed @sdkwork/im-app-sdk consumer package',
);
assert.ok(
  appPackageJson.dependencies['@sdkwork/im-backend-sdk'],
  'PC app must depend on the composed @sdkwork/im-backend-sdk consumer package so backend SDK ownership is explicit',
);
assert.ok(
  !appPackageJson.dependencies['@sdkwork-internal/im-app-api-generated'],
  'PC app must not depend directly on generated IM app transport packages',
);
assert.ok(
  !appPackageJson.dependencies['@sdkwork-internal/im-backend-api-generated'],
  'PC app must not depend directly on generated IM backend transport packages',
);
assert.ok(
  !appPackageJson.dependencies[retiredGenericAppSdkPackage],
  'PC app must not depend on the generic spring-ai-plus app SDK package',
);
assert.ok(
  !appPackageJson.dependencies[retiredGenericBackendSdkPackage],
  'PC app must not depend on the generic spring-ai-plus backend SDK package',
);
assert.ok(appPackageJson.dependencies['@sdkwork/im-sdk'], 'PC app must depend on generated @sdkwork/im-sdk');
assert.ok(appPackageJson.dependencies['@sdkwork/rtc-sdk'], 'PC app may depend on standard @sdkwork/rtc-sdk for RTC media capability');
assert.ok(appPackageJson.dependencies['@sdkwork/appbase-pc-react'], 'PC app must depend on sdkwork-appbase PC wrapper');
assert.ok(appPackageJson.dependencies['@sdkwork/auth-runtime-pc-react'], 'PC app must depend on the appbase high-level auth runtime');
assert.ok(appPackageJson.dependencies['@sdkwork/drive-app-sdk'], 'PC app must depend on sdkwork-drive app SDK for chat media uploads');
assert.ok(appPackageJson.dependencies['@sdkwork/notary-app-sdk'], 'PC app must depend on sdkwork-notary app SDK for notary workflows');
assert.equal(
  appPackageJson.dependencies['@sdkwork/iam-sdk-adapter'],
  undefined,
  'PC app must not depend on the lower-level appbase IAM SDK adapter after appbase auth runtime migration',
);
assert.ok(appPackageJson.dependencies['@sdkwork/iam-sdk-ports'], 'PC app must depend on the appbase IAM SDK ports');
assert.ok(!appPackageJson.dependencies['@tauri-apps/api'], 'Tauri renderer API must live in the desktop workspace package');
assert.ok(!appPackageJson.devDependencies['@tauri-apps/cli'], 'Tauri CLI must live in the desktop workspace package');
assert.ok(!fs.existsSync(path.join(appRoot, 'src-tauri')), 'root sdkwork-im-pc must not own the Tauri shell directly');
assert.ok(fs.existsSync(path.join(desktopPackageRoot, 'node_modules/@tauri-apps/cli')), 'desktop package must have its own Tauri CLI dependency installed');

const desktopPackageJson = readJson('apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop/package.json');
assert.equal(desktopPackageJson.name, '@sdkwork/im-pc-desktop');
assert.equal(desktopPackageJson.scripts['dev:desktop'], 'node ../../../../scripts/run-tauri-cli.mjs dev');
assert.equal(
  desktopPackageJson.scripts['start:renderer'],
  'node ../../../../scripts/dev/run-sdkwork-im-pc-tauri-renderer-dev.mjs',
);
assert.equal(desktopPackageJson.scripts['dev:renderer'], undefined);
assert.equal(desktopPackageJson.scripts['desktop:dev:local'], undefined);
assert.equal(desktopPackageJson.scripts['desktop:build:local'], undefined);
assert.equal(desktopPackageJson.scripts['build:desktop:local'], 'node ../../../../scripts/run-tauri-cli.mjs build');
assert.equal(desktopPackageJson.scripts['dev:tauri'], undefined);
assert.equal(desktopPackageJson.scripts.build, 'pnpm --dir ../.. build');
assert.equal(desktopPackageJson.devDependencies['@tauri-apps/cli'], 'catalog:');
assert.equal(desktopPackageJson.dependencies['@tauri-apps/api'], 'catalog:');

assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop/src-tauri/tauri.conf.json');
assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop/src-tauri/Cargo.toml');
assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop/src-tauri/src/main.rs');
assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop/src-tauri/build.rs');
assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop/src-tauri/icons/icon.ico');
assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop/src-tauri/icons/32x32.png');
assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop/src-tauri/icons/128x128.png');
assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop/src-tauri/icons/128x128@2x.png');

const tauriCargoToml = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop/src-tauri/Cargo.toml');
assert.match(tauriCargoToml, /tauri\s*=\s*\{[^}]*version\s*=\s*"~2\.11\.0"/u);
assert.match(tauriCargoToml, /tauri-plugin-shell\s*=\s*"2"/u);
assert.match(tauriCargoToml, /serde_json\s*=\s*"1"/u);
assert.match(tauriCargoToml, /\n\[workspace\]\s*$/u, 'Tauri crate must be an independent workspace inside the Sdkwork IM Rust workspace');

const tauriConfig = readJson('apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop/src-tauri/tauri.conf.json');
assert.equal(tauriConfig.productName, 'Sdkwork IM PC');
assert.equal(tauriConfig.identifier, 'com.sdkwork.chatpc');
assert.equal(tauriConfig.build.devUrl, 'http://127.0.0.1:4176');
assert.equal(
  tauriConfig.build.beforeDevCommand,
  'node ../../../../scripts/dev/run-sdkwork-im-pc-tauri-renderer-dev.mjs',
  'Tauri beforeDevCommand must start the PC renderer without relying on pnpm PATH',
);
assert.equal(
  tauriConfig.build.beforeBuildCommand,
  'node ../../../../scripts/dev/run-sdkwork-im-pc-tauri-renderer-build.mjs',
  'Tauri beforeBuildCommand must build the PC renderer without relying on pnpm PATH',
);
assert.equal(tauriConfig.build.frontendDist, '../../../dist');
assert.match(
  runTauriCliSource,
  /SDKWORK_IM_PC_DEV_PORT/u,
  'Tauri dev launcher must read the resolved Sdkwork IM PC dev port',
);
assert.match(
  runTauriCliSource,
  /devUrl/u,
  'Tauri dev launcher must be able to merge the selected devUrl into Tauri dev config',
);
assert.equal(tauriConfig.app.windows[0].decorations, false, 'Tauri shell must preserve the current custom titlebar');
assert.equal(tauriConfig.app.windows[0].minWidth, 1200);
assert.equal(tauriConfig.app.windows[0].minHeight, 760);
assert.deepEqual(tauriConfig.bundle.icon, [
  'icons/32x32.png',
  'icons/128x128.png',
  'icons/128x128@2x.png',
  'icons/icon.ico',
]);

const viteConfig = read('apps/sdkwork-im-pc/vite.config.ts');
const tsconfig = read('apps/sdkwork-im-pc/tsconfig.json');
assert.match(viteConfig, /@sdkwork\/im-app-sdk/u, 'Vite must alias composed IM app SDK facade source');
assert.match(
  viteConfig,
  /sdks[\\\/]sdkwork-im-app-sdk[\\\/]sdkwork-im-app-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
);
assert.match(
  viteConfig,
  /@sdkwork\/im-backend-sdk/u,
  'Vite must alias composed IM backend SDK facade source',
);
assert.match(
  viteConfig,
  /sdks[\\\/]sdkwork-im-backend-sdk[\\\/]sdkwork-im-backend-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
);
assert.match(
  tsconfig,
  /sdks[\\\/]sdkwork-im-app-sdk[\\\/]sdkwork-im-app-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
  'TypeScript must resolve composed IM app SDK facade from source for live development',
);
assert.match(
  tsconfig,
  /sdks[\\\/]sdkwork-im-backend-sdk[\\\/]sdkwork-im-backend-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
  'TypeScript must resolve composed IM backend SDK facade from source for live development',
);
assert.doesNotMatch(
  viteConfig,
  /spring-ai-plus-(?:app|backend)-api|@sdkwork\/(?:app|backend)-sdk/u,
  'Vite must not route the PC app through generic spring-ai-plus app/backend SDKs',
);
assert.doesNotMatch(
  tsconfig,
  /spring-ai-plus-(?:app|backend)-api|@sdkwork\/(?:app|backend)-sdk/u,
  'TypeScript must not route the PC app through generic spring-ai-plus app/backend SDKs',
);
assert.match(viteConfig, /@sdkwork\/im-sdk/u, 'Vite must alias generated IM SDK source');
assert.match(viteConfig, /sdks[\\\/]sdkwork-im-sdk[\\\/]sdkwork-im-sdk-typescript[\\\/]src[\\\/]index\.ts/u);
assert.match(
  tsconfig,
  /sdks[\\\/]sdkwork-im-sdk[\\\/]sdkwork-im-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
  'TypeScript must resolve generated IM SDK from source for live development',
);
assert.doesNotMatch(
  `${viteConfig}\n${tsconfig}`,
  /@sdkwork\/im-sdk-generated|sdkwork-im-sdk-typescript[\\\/]generated[\\\/]server-openapi/u,
  'PC app must not alias IM generated transport; consumers use the composed @sdkwork/im-sdk facade',
);
assert.match(viteConfig, /@sdkwork\/rtc-sdk/u, 'Vite must alias generated RTC SDK source');
assert.match(
  viteConfig,
  /dependencyRoot\('sdkwork-rtc'\)[\s\S]*sdks[\\\/]sdkwork-rtc-sdk[\\\/]sdkwork-rtc-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
  'Vite must resolve RTC SDK through the sibling workspace dependency for portable local development',
);
assert.match(
  tsconfig,
  /\.\.[\\\/]\.\.[\\\/]\.\.[\\\/]sdkwork-rtc[\\\/]sdks[\\\/]sdkwork-rtc-sdk[\\\/]sdkwork-rtc-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
  'TypeScript must resolve generated RTC SDK from source for live development',
);
assert.match(viteConfig, /@sdkwork\/drive-app-sdk/u, 'Vite must alias generated Drive app SDK source for chat media upload');
assert.match(viteConfig, /@sdkwork\/catalog-app-sdk/u, 'Vite must alias composed Catalog app SDK source for shop modules');
assert.match(viteConfig, /@sdkwork\/shop-app-sdk/u, 'Vite must alias composed Shop app SDK source for merchant flows');
assert.match(viteConfig, /@sdkwork\/order-app-sdk/u, 'Vite must alias composed Order app SDK source for checkout and orders');
assert.match(
  viteConfig,
  /dependencyRoot\('sdkwork-catalog'\)[\s\S]*sdkwork-catalog-app-sdk[\\\/]sdkwork-catalog-app-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
  'Vite must resolve Catalog app SDK through the sibling sdkwork-catalog composed facade',
);
assert.match(
  viteConfig,
  /dependencyRoot\('sdkwork-shop'\)[\s\S]*sdkwork-shop-app-sdk[\\\/]sdkwork-shop-app-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
  'Vite must resolve Shop app SDK through the sibling sdkwork-shop composed facade',
);
assert.match(
  viteConfig,
  /dependencyRoot\('sdkwork-order'\)[\s\S]*sdkwork-order-app-sdk[\\\/]sdkwork-order-app-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
  'Vite must resolve Order app SDK through the sibling sdkwork-order composed facade',
);
assert.match(
  tsconfig,
  /\.\.[\\\/]\.\.[\\\/]\.\.[\\\/]sdkwork-drive[\\\/]sdks[\\\/]sdkwork-drive-app-sdk[\\\/]sdkwork-drive-app-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
  'TypeScript must resolve generated Drive app SDK from sibling source for chat media uploads',
);
assert.match(
  tsconfig,
  /\.\.[\\\/]\.\.[\\\/]\.\.[\\\/]sdkwork-catalog[\\\/]sdks[\\\/]sdkwork-catalog-app-sdk[\\\/]sdkwork-catalog-app-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
  'TypeScript must resolve composed Catalog app SDK from sibling source',
);
assert.match(
  tsconfig,
  /\.\.[\\\/]\.\.[\\\/]\.\.[\\\/]sdkwork-shop[\\\/]sdks[\\\/]sdkwork-shop-app-sdk[\\\/]sdkwork-shop-app-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
  'TypeScript must resolve composed Shop app SDK from sibling source',
);
assert.match(
  tsconfig,
  /\.\.[\\\/]\.\.[\\\/]\.\.[\\\/]sdkwork-order[\\\/]sdks[\\\/]sdkwork-order-app-sdk[\\\/]sdkwork-order-app-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
  'TypeScript must resolve composed Order app SDK from sibling source',
);
assert.match(
  viteConfig,
  /dependencyRoot\('sdkwork-drive'\)[\s\S]*sdks[\\\/]sdkwork-drive-app-sdk[\\\/]sdkwork-drive-app-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
  'Vite must resolve Drive app SDK through the sibling sdkwork-drive workspace for portable media upload development',
);
assert.match(viteConfig, /@sdkwork\/notary-app-sdk/u, 'Vite must alias generated Notary app SDK source for notary workflows');
assert.match(
  viteConfig,
  /dependencyRoot\('sdkwork-notary'\)[\s\S]*sdks[\\\/]sdkwork-notary-app-sdk[\\\/]sdkwork-notary-app-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
  'Vite must resolve Notary app SDK through the sibling sdkwork-notary workspace for portable notary development',
);
assert.match(
  tsconfig,
  /\.\.[\\\/]\.\.[\\\/]\.\.[\\\/]sdkwork-notary[\\\/]sdks[\\\/]sdkwork-notary-app-sdk[\\\/]sdkwork-notary-app-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
  'TypeScript must resolve generated Notary app SDK from sibling source for notary workflows',
);
assert.match(viteConfig, /@sdkwork\/appbase-pc-react/u, 'Vite must alias appbase PC package source');
assert.match(viteConfig, /@sdkwork\/core-pc-react/u, 'Vite must alias SDKWork core PC React package');
assert.match(
  viteConfig,
  /dependencyRoot\('sdkwork-core'\)[\s\S]*sdkwork-core-pc-react[\\\/]src/u,
  'Vite must alias SDKWork core PC React through the sibling workspace dependency for portable live development',
);
assert.doesNotMatch(
  viteConfig,
  /sdkwork-core[\\\/]sdkwork-core-pc-react[\\\/]dist[\\\/]index\.js/u,
  'Vite must not consume the prebuilt SDKWork core PC React dist artifact during local development',
);
assert.doesNotMatch(
  tsconfig,
  /sdkwork-core[\\\/]sdkwork-core-pc-react[\\\/]dist[\\\/]index\.d\.ts/u,
  'TypeScript must not consume the prebuilt SDKWork core PC React declaration artifact during local development',
);
assert.match(viteConfig, /@sdkwork\/ui-pc-react/u, 'Vite must alias SDKWork UI PC React package');
assert.match(
  viteConfig,
  /dependencyRoot\('sdkwork-ui'\)[\s\S]*sdkwork-ui-pc-react[\\\/]src/u,
  'Vite must alias SDKWork UI PC React through the sibling workspace dependency for portable live development',
);
assert.doesNotMatch(
  viteConfig,
  /sdkwork-ui[\\\/]sdkwork-ui-pc-react[\\\/]dist[\\\/]index\.js/u,
  'Vite must not consume the prebuilt SDKWork UI PC React dist artifact during local development',
);
assert.doesNotMatch(
  tsconfig,
  /sdkwork-ui[\\\/]sdkwork-ui-pc-react[\\\/]dist[\\\/]index\.d\.ts/u,
  'TypeScript must not consume the prebuilt SDKWork UI PC React declaration artifact during local development',
);
for (const localReactPackageName of ['@sdkwork/core-pc-react', '@sdkwork/ui-pc-react']) {
  assert.match(
    viteConfig,
    new RegExp(`optimizeDeps:[\\s\\S]*exclude:[\\s\\S]*['"]${localReactPackageName.replaceAll('/', '\\/')}['"]`, 'u'),
    `Vite must exclude ${localReactPackageName} from dependency pre-bundling so source edits stay live`,
  );
}
for (const localSdkPackageName of [
  '@sdkwork/im-app-sdk',
  '@sdkwork/im-backend-sdk',
  '@sdkwork/iam-app-sdk',
  '@sdkwork/drive-app-sdk',
  '@sdkwork/catalog-app-sdk',
  '@sdkwork/shop-app-sdk',
  '@sdkwork/order-app-sdk',
  '@sdkwork/im-sdk',
  '@sdkwork/notary-app-sdk',
  '@sdkwork/rtc-sdk',
  '@sdkwork/appbase-pc-react',
  '@sdkwork/auth-pc-react',
  '@sdkwork/auth-pc-react/auth',
  '@sdkwork/auth-runtime-pc-react',
  '@sdkwork/iam-contracts',
  '@sdkwork/iam-sdk-ports',
  '@sdkwork/i18n-pc-react',
]) {
  assert.match(
    viteConfig,
    new RegExp(`optimizeDeps:[\\s\\S]*exclude:[\\s\\S]*['"]${localSdkPackageName.replaceAll('/', '\\/')}['"]`, 'u'),
    `Vite must exclude ${localSdkPackageName} from dependency pre-bundling so source edits stay live`,
  );
}
assert.match(
  viteConfig,
  /manualChunks/u,
  'Vite release build must define manual chunks for stable dependency packaging',
);
for (const chunkName of ['react-vendor', 'editor-vendor']) {
  assert.match(
    viteConfig,
    new RegExp(chunkName, 'u'),
    `Vite release build must expose a ${chunkName} chunk`,
  );
}
assert.doesNotMatch(
  viteConfig,
  /ai-vendor|@google\/genai|GEMINI_API_KEY|AI Studio/u,
  'Vite release build must not retain AI Studio or Gemini vendor chunk wiring',
);
assert.doesNotMatch(
  viteConfig,
  /return ['"]sdkwork-shared['"]/u,
  'Vite must not split linked SDKWork source packages into a circular shared chunk',
);

const iamEnvModule = await import(pathToFileURL(path.join(appRoot, 'scripts/sdkwork-chat-iam-env.mjs')).href);
const resolvedEnv = iamEnvModule.resolveSdkworkChatIamCommandEnv({
  env: {},
  iamMode: 'desktop-local',
  target: 'desktop-dev',
});
assert.deepEqual(resolvedEnv.errors, []);
assert.equal(resolvedEnv.env.SDKWORK_IAM_MODE, 'private');
assert.equal(resolvedEnv.env.VITE_SDKWORK_DEPLOYMENT_MODE, 'saas');
assert.equal(resolvedEnv.env.VITE_SDKWORK_IAM_APP_API_BASE_URL, 'http://127.0.0.1:18079');
assert.equal(resolvedEnv.env.VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL, 'http://127.0.0.1:18079');
assert.equal(resolvedEnv.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL, 'http://127.0.0.1:18079');
assert.equal(resolvedEnv.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL, 'ws://127.0.0.1:18079');
assert.equal(resolvedEnv.env.SDKWORK_IAM_DEV_FIXED_VERIFY_CODE, '123456');

const webBuildSameOriginEnv = iamEnvModule.resolveSdkworkChatIamCommandEnv({
  env: {},
  target: 'web-build',
});
assert.deepEqual(webBuildSameOriginEnv.errors, []);
assert.equal(webBuildSameOriginEnv.env.SDKWORK_IAM_MODE, 'private');
assert.equal(webBuildSameOriginEnv.env.VITE_SDKWORK_DEPLOYMENT_MODE, 'private');
assert.equal(
  webBuildSameOriginEnv.env.VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL,
  undefined,
  'web release builds without explicit domain binding must not bake localhost platform gateway URLs into the bundle',
);
assert.equal(
  webBuildSameOriginEnv.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  undefined,
  'web release builds without explicit domain binding must let the runtime resolve IM HTTP from window.location.origin',
);
assert.equal(
  webBuildSameOriginEnv.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL,
  undefined,
  'web release builds without explicit domain binding must let the runtime derive IM websocket from the public origin',
);

const releaseDomainEnv = iamEnvModule.resolveSdkworkChatIamCommandEnv({
  env: {
    SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: 'https://api.sdkwork.com/',
    SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: 'https://im.sdkwork.com',
    SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: 'wss://im.sdkwork.com',
  },
  target: 'server-build',
});
assert.deepEqual(releaseDomainEnv.errors, []);
assert.equal(releaseDomainEnv.env.VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL, 'https://api.sdkwork.com');
assert.equal(releaseDomainEnv.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL, 'https://im.sdkwork.com');
assert.equal(releaseDomainEnv.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL, 'wss://im.sdkwork.com');

const releaseServerDomainEnv = iamEnvModule.resolveSdkworkChatIamCommandEnv({
  env: {
    SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: 'https://im.sdkwork.com/',
    SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: 'wss://im.sdkwork.com/im',
  },
  target: 'server-build',
});
assert.deepEqual(releaseServerDomainEnv.errors, []);
assert.equal(
  releaseServerDomainEnv.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  'https://im.sdkwork.com',
  'release builds must bind IM HTTP traffic from the application public HTTP env',
);
assert.equal(
  releaseServerDomainEnv.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL,
  'wss://im.sdkwork.com/im',
  'release builds must bind IM websocket traffic from the application public websocket env',
);

const releaseCanonicalServerDomainEnv = iamEnvModule.resolveSdkworkChatIamCommandEnv({
  env: {
    SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: 'https://im.sdkwork.com/im/v3/api/',
    SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: 'wss://im.sdkwork.com/im/v3/api/realtime/ws',
  },
  target: 'server-build',
});
assert.deepEqual(releaseCanonicalServerDomainEnv.errors, []);
assert.equal(
  releaseCanonicalServerDomainEnv.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  'https://im.sdkwork.com',
  'release IM API base URL must strip the SDK-owned /im/v3/api contract prefix',
);
assert.equal(
  releaseCanonicalServerDomainEnv.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL,
  'wss://im.sdkwork.com',
  'release IM websocket base URL must strip the SDK-owned realtime websocket path',
);

const releaseServerApiOnlyDomainEnv = iamEnvModule.resolveSdkworkChatIamCommandEnv({
  env: {
    SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: 'https://api.sdkwork.com/',
  },
  target: 'server-build',
});
assert.deepEqual(releaseServerApiOnlyDomainEnv.errors, []);
assert.equal(
  releaseServerApiOnlyDomainEnv.env.VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL,
  'https://api.sdkwork.com',
  'release builds must bind platform SDK traffic when only the platform gateway URL is configured',
);
assert.equal(
  releaseServerApiOnlyDomainEnv.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  undefined,
  'release builds without application public HTTP must not invent IM HTTP URLs',
);

const releaseFullContractPathEnv = iamEnvModule.resolveSdkworkChatIamCommandEnv({
  env: {
    SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: 'https://api.sdkwork.com/app/v3/api/',
    SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: 'https://im.sdkwork.com/im/v3/api/',
    SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: 'wss://im.sdkwork.com/im/v3/api/realtime/ws',
  },
  target: 'server-build',
});
assert.deepEqual(releaseFullContractPathEnv.errors, []);
assert.equal(
  releaseFullContractPathEnv.env.VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL,
  'https://api.sdkwork.com',
  'release platform gateway URL must strip the SDK-owned /app/v3/api contract prefix',
);
assert.equal(
  releaseFullContractPathEnv.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  'https://im.sdkwork.com',
  'release IM API base URL must strip the SDK-owned /im/v3/api contract prefix',
);
assert.equal(
  releaseFullContractPathEnv.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL,
  'wss://im.sdkwork.com',
  'release IM websocket base URL must strip the SDK-owned realtime websocket path',
);

const invalidReleaseDomainEnv = iamEnvModule.resolveSdkworkChatIamCommandEnv({
  env: {
    SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: 'im.sdkwork.com',
    SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: 'https://im.sdkwork.com',
  },
  target: 'server-build',
});
assert.match(
  invalidReleaseDomainEnv.errors.join('\n'),
  /SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL/u,
  'release builds must fail when an explicit application HTTP domain is not an absolute http(s) URL',
);
assert.match(
  invalidReleaseDomainEnv.errors.join('\n'),
  /SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL/u,
  'release builds must fail when an explicit application websocket domain is not a ws(s) URL',
);

const serverEnvTemplate = read('deployments/templates/server.env.example');
const quickstartServerEnvTemplate = read('deployments/templates/quickstart-server-compose.env.example');
assert.match(
  serverEnvTemplate,
  /SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=https:\/\/im\.sdkwork\.com/u,
  'server env template must document the canonical IM application public HTTP host',
);
assert.match(
  serverEnvTemplate,
  /SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=wss:\/\/im\.sdkwork\.com/u,
  'server env template must document the canonical IM application public websocket host',
);
assert.match(
  serverEnvTemplate,
  /SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=https:\/\/api\.sdkwork\.com/u,
  'server env template must document the platform api gateway host',
);
assert.doesNotMatch(
  serverEnvTemplate,
  /SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=.*\/im\/v3\/api\/realtime\/ws/u,
  'server env template must document websocket base URL, not the SDK-owned realtime websocket endpoint',
);
assert.doesNotMatch(
  quickstartServerEnvTemplate,
  /SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=.*\/im\/v3\/api\/realtime\/ws/u,
  'quickstart env template must document websocket base URL, not the SDK-owned realtime websocket endpoint',
);

assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/appSdkClient.ts');
assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/agentAppSdkClient.ts');
assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/appAuthService.ts');
assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/imSdkClient.ts');
assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/session.ts');

const coreIndex = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/index.ts');
assert.match(coreIndex, /export \* from '\.\/sdk\/appSdkClient'/u);
assert.match(coreIndex, /export \* from '\.\/sdk\/appAuthService'/u);
assert.match(coreIndex, /export \* from '\.\/sdk\/imSdkClient'/u);
assert.match(coreIndex, /export \* from '\.\/sdk\/session'/u);

const sdkBaseUrlsSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/sdkBaseUrls.ts');
const appSdkClientSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/appSdkClient.ts');
const agentAppSdkClientSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/agentAppSdkClient.ts');
const sessionSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/session.ts');
assert.match(
  appSdkClientSource,
  /from ['"]@sdkwork\/im-app-sdk['"]/u,
  'app SDK wrapper must use the composed @sdkwork/im-app-sdk consumer package',
);
assert.doesNotMatch(
  appSdkClientSource,
  /@sdkwork-internal\/im-app-api-generated/u,
  'app SDK wrapper must not import generated IM app transport directly',
);
assert.match(appSdkClientSource, /createClient/u);
assert.match(appSdkClientSource, /createAppSdkClientConfig/u);
assert.match(appSdkClientSource, /getAppSdkClientWithSession/u);
assert.match(appSdkClientSource, /SdkworkImAppClient/u, 'app SDK wrapper must expose product-scoped SdkworkImAppClient naming');
assert.match(appSdkClientSource, /resolveApplicationOrPlatformHttpBaseUrlOrThrow/u, 'app SDK wrapper must resolve base URLs through the shared topology resolver');
assert.match(sdkBaseUrlsSource, /window\.location\.origin/u, 'shared topology resolver must support release same-origin domain binding');
assert.match(sdkBaseUrlsSource, /if\s*\(\s*!import\.meta\.env\.DEV\s*\)/u, 'shared topology resolver must keep localhost defaults in a Vite-prunable dev-only branch');
assert.match(
  sessionSource,
  /export function resolveAppSdkUserId/u,
  'session helper must expose the user id from the IAM login token claims',
);
assert.match(
  sessionSource,
  /export function createSdkworkChatRequestContext/u,
  'session helper must encapsulate JWT claims and persisted session context for local UI state',
);
assert.match(
  sessionSource,
  /export function createSdkworkChatRequestContextInterceptors/u,
  'session helper must expose request interceptors for generated SDK composition',
);
assert.doesNotMatch(
  sessionSource,
  /buildSdkworkChatAppContextHeaders|SdkworkChatAppContextHeaders/u,
  'session helper must not expose an AppContext header builder; server request context comes from AuthToken and Access-Token',
);
assert.doesNotMatch(
  sessionSource,
  /Sdkwork-(?:Tenant|Organization|User|Session|Actor|Device)-Id|sdkwork-(?:tenant|organization|user|session|actor|device)-id/u,
  'session helper must not contain SDKWork context header names',
);
assert.match(
  appSdkClientSource,
  /createSdkworkChatRequestContextInterceptors/u,
  'app SDK wrapper must install the shared generated-SDK request interceptor',
);
assert.doesNotMatch(
  appSdkClientSource,
  /headers:\s*buildSdkworkChatAppContextHeaders/u,
  'app SDK wrapper must not keep stale static AppContext header injection after token refresh',
);
assert.doesNotMatch(
  appSdkClientSource,
  /tenantId:\s*resolveAppSdkTenantId|organizationId:\s*resolveAppSdkOrganizationId/u,
  'app SDK wrapper must not pass current tenantId/organizationId as static config; current scope belongs in the request Context interceptor',
);
assert.doesNotMatch(
  appSdkClientSource,
  /const\s+LOCAL_APP_API_BASE_URL\s*=\s*['"]http:\/\/127\.0\.0\.1:18079['"]/u,
  'app SDK wrapper must not keep localhost fallback as a production-retained top-level constant',
);
assert.doesNotMatch(appSdkClientSource, /\bfetch\s*\(/u, 'app SDK wrapper must not use raw fetch');

const driveAppSdkClientSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/drivePcIntegration.ts');
assert.match(
  driveAppSdkClientSource,
  /from ['"]@sdkwork\/drive-app-sdk['"]/u,
  'Drive app SDK wrapper must use the generated sdkwork-drive app SDK client',
);
assert.match(
  driveAppSdkClientSource,
  /createDriveAppClient/u,
  'Drive app SDK wrapper must construct the generated Drive app SDK client',
);
assert.match(
  driveAppSdkClientSource,
  /createSdkworkChatRequestContextInterceptors/u,
  'Drive app SDK wrapper must install the shared generated-SDK request interceptor',
);
assert.match(
  driveAppSdkClientSource,
  /getSdkworkChatGlobalTokenManager/u,
  'Drive app SDK wrapper must share the same global TokenManager as appbase and IM clients',
);
assert.doesNotMatch(
  driveAppSdkClientSource,
  /headers:\s*buildSdkworkChatAppContextHeaders/u,
  'Drive app SDK wrapper must not keep stale static AppContext header injection after token refresh',
);
assert.doesNotMatch(
  driveAppSdkClientSource,
  /tenantId:\s*resolveAppSdkTenantId|organizationId:\s*resolveAppSdkOrganizationId/u,
  'Drive app SDK wrapper must not pass current tenantId/organizationId as static config; current scope belongs in the request Context interceptor',
);
assert.doesNotMatch(driveAppSdkClientSource, /\bfetch\s*\(/u, 'Drive app SDK wrapper must not use raw fetch');

const drivePcIntegrationSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/drivePcIntegration.ts');
const drivePcBootstrapSource = read('apps/sdkwork-im-pc/src/bootstrap/drivePc.ts');
const driveShellLoadersSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-shell/src/capabilityModuleLoaders.ts');
const driveEmbedIndexSource = read(
  '../sdkwork-drive/apps/sdkwork-drive-pc/packages/sdkwork-drive-pc-drive/src/index.ts',
);

assert.equal(
  fs.existsSync(path.join(repoRoot, 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-drive')),
  false,
  'apps/sdkwork-im-pc must not keep a local sdkwork-im-pc-drive package.',
);
assert.match(
  driveShellLoadersSource,
  /drive:\s*\(\)\s*=>\s*import\(['"]@sdkwork\/drive-pc-drive['"]\)/u,
  'IM shell must lazy-load the sdkwork-drive-pc-drive capability package.',
);
assert.match(
  drivePcIntegrationSource,
  /configureDrivePcRuntime/u,
  'IM core must configure the embeddable Drive PC runtime through sdkPorts.',
);
assert.match(
  drivePcIntegrationSource,
  /getDriveAppSdkClient/u,
  'IM Drive integration must inject the shared Drive app SDK client.',
);
assert.match(
  drivePcBootstrapSource,
  /bootstrapDrivePcForIm/u,
  'IM bootstrap must initialize Drive PC integration before rendering shell modules.',
);
assert.match(
  driveEmbedIndexSource,
  /export\s*\{\s*DriveView\s*\}/u,
  'sdkwork-drive-pc-drive must export DriveView for host-managed embedding.',
);
assert.match(
  driveEmbedIndexSource,
  /configureDrivePcRuntime/u,
  'sdkwork-drive-pc-drive must export configureDrivePcRuntime for host applications.',
);
assert.doesNotMatch(
  drivePcIntegrationSource,
  /\bfetch\s*\(/u,
  'IM Drive integration must not use raw fetch',
);

const commercePcIntegrationSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/commercePcIntegration.ts',
);
const shopPcCatalogClientSource = readSibling(
  'sdkwork-shop/apps/sdkwork-shop-pc/packages/sdkwork-shop-pc-core/src/sdk/catalogAppSdkClient.ts',
);
const shopPcShopClientSource = readSibling(
  'sdkwork-shop/apps/sdkwork-shop-pc/packages/sdkwork-shop-pc-core/src/sdk/shopAppSdkClient.ts',
);
const shopPcOrderClientSource = readSibling(
  'sdkwork-shop/apps/sdkwork-shop-pc/packages/sdkwork-shop-pc-core/src/sdk/orderAppSdkClient.ts',
);

assert.ok(
  !fs.existsSync(path.join(repoRoot, 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/catalogAppSdkClient.ts')),
  'IM PC core must not duplicate sdkwork-shop catalog app SDK wrapper',
);
assert.ok(
  !fs.existsSync(path.join(repoRoot, 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/shopAppSdkClient.ts')),
  'IM PC core must not duplicate sdkwork-shop shop app SDK wrapper',
);
assert.ok(
  !fs.existsSync(path.join(repoRoot, 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/orderAppSdkClient.ts')),
  'IM PC core must not duplicate sdkwork-shop order app SDK wrapper',
);

assert.match(
  commercePcIntegrationSource,
  /from ['"]@sdkwork\/shop-pc-core['"]/u,
  'Commerce PC integration must delegate SDK clients to sdkwork-shop-pc-core',
);
assert.match(
  commercePcIntegrationSource,
  /getImHostedCatalogAppSdkClient|getImHostedOrderAppSdkClient|getImHostedShopAppSdkClient/u,
  'Commerce PC integration must expose IM-hosted commerce SDK accessors',
);
assert.doesNotMatch(commercePcIntegrationSource, /\bfetch\s*\(/u, 'Commerce PC integration must not use raw fetch');

assert.match(
  shopPcCatalogClientSource,
  /from ['"]@sdkwork\/catalog-app-sdk['"]/u,
  'Canonical catalog app SDK wrapper must use the generated sdkwork-catalog app SDK client',
);
assert.match(
  shopPcCatalogClientSource,
  /getCatalogAppSdkClientWithSession/u,
  'Canonical catalog app SDK wrapper must expose session-aware client access',
);
assert.match(
  shopPcShopClientSource,
  /from ['"]@sdkwork\/shop-app-sdk['"]/u,
  'Canonical shop app SDK wrapper must use the generated sdkwork-shop app SDK client',
);
assert.match(
  shopPcShopClientSource,
  /getShopAppSdkClientWithSession/u,
  'Canonical shop app SDK wrapper must expose session-aware client access',
);
assert.match(
  shopPcOrderClientSource,
  /from ['"]@sdkwork\/order-app-sdk['"]/u,
  'Canonical order app SDK wrapper must use the generated sdkwork-order app SDK client',
);
assert.match(
  shopPcOrderClientSource,
  /getOrderAppSdkClientWithSession/u,
  'Canonical order app SDK wrapper must expose session-aware client access',
);
assert.doesNotMatch(
  `${shopPcCatalogClientSource}\n${shopPcShopClientSource}\n${shopPcOrderClientSource}`,
  /\bfetch\s*\(/u,
  'Canonical commerce T1 app SDK wrappers must not use raw fetch',
);

const shopPackageJson = readJson('apps/sdkwork-im-pc/packages/sdkwork-im-pc-shop/package.json');
assert.ok(
  shopPackageJson.dependencies['@sdkwork/shop-pc-consumer'],
  'IM shop adapter must depend on canonical shop PC consumer package',
);
assert.ok(
  shopPackageJson.dependencies['@sdkwork/shop-pc-core'],
  'IM shop adapter must depend on canonical shop PC core integration package',
);
assert.equal(
  shopPackageJson.dependencies['@sdkwork/catalog-app-sdk'],
  undefined,
  'IM shop adapter must not declare direct catalog app SDK dependency',
);

const ordersPackageJson = readJson('apps/sdkwork-im-pc/packages/sdkwork-im-pc-orders/package.json');
assert.ok(
  ordersPackageJson.dependencies['@sdkwork/shop-pc-orders'],
  'IM orders adapter must depend on canonical shop PC orders package',
);
assert.equal(
  ordersPackageJson.dependencies['@sdkwork/order-app-sdk'],
  undefined,
  'IM orders adapter must not declare direct order app SDK dependency',
);

assert.ok(
  !fs.existsSync(path.join(repoRoot, 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-shop/src/services/ShopService.ts')),
  'IM shop adapter must not keep duplicate ShopService implementation',
);
assert.ok(
  !fs.existsSync(path.join(repoRoot, 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-orders/src/services/OrdersService.ts')),
  'IM orders adapter must not keep duplicate OrdersService implementation',
);

const shopServiceSource = readSibling(
  'sdkwork-shop/apps/sdkwork-shop-pc/packages/sdkwork-shop-pc-consumer/src/services/ShopService.ts',
);
const ordersServiceSource = readSibling(
  'sdkwork-shop/apps/sdkwork-shop-pc/packages/sdkwork-shop-pc-orders/src/services/OrdersService.ts',
);
const commercePackageSource = `${shopServiceSource}\n${ordersServiceSource}`;

assert.match(
  shopServiceSource,
  /getCatalogAppSdkClientWithSession[\s\S]*getOrderAppSdkClientWithSession/u,
  'Canonical shop service must consume catalog and order T1 app SDK wrappers',
);
assert.match(shopServiceSource, /\.catalog\.categories\.list\s*\(/u, 'Canonical shop service must list categories through catalog app SDK');
assert.match(shopServiceSource, /\.catalog\.products\.list\s*\(/u, 'Canonical shop service must list products through catalog app SDK');
assert.match(shopServiceSource, /\.cart\.current\.retrieve\s*\(/u, 'Canonical shop service must read the cart through catalog app SDK');
assert.match(shopServiceSource, /\.checkout\.sessions\.create\s*\(/u, 'Canonical shop service must create checkout sessions through order app SDK');
assert.match(
  ordersServiceSource,
  /getOrderAppSdkClientWithSession[\s\S]*getShopAppSdkClientWithSession/u,
  'Canonical orders service must consume order and shop T1 app SDK wrappers',
);
assert.match(
  ordersServiceSource,
  /\.shops\.current\.orders\.list\s*\(/u,
  'Canonical orders service must list merchant orders through shop app SDK',
);
assert.doesNotMatch(commercePackageSource, /\bfetch\s*\(/u, 'Canonical commerce PC packages must not bypass generated SDKs with raw fetch');
assert.doesNotMatch(
  commercePackageSource,
  /\/(?:im|app|backend)\/v3/u,
  'Canonical commerce PC packages must not hand-code SDKWork API paths',
);

const moduleRegistrySource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-shell/src/moduleRegistry.ts');
assert.match(
  moduleRegistrySource,
  /COMMERCIAL_RUNTIME_MODULES[\s\S]*["']shop["'][\s\S]*["']orders["']/u,
  'Commercial runtime modules must include shop and orders after Commerce SDK integration',
);

const aiotPcIntegrationSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/aiotPcIntegration.ts');
assert.ok(
  !fs.existsSync(path.join(repoRoot, 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/aiotAppSdkClient.ts')),
  'IM PC core must not duplicate sdkwork-aiot app SDK wrapper',
);
assert.match(
  aiotPcIntegrationSource,
  /from ['"]@sdkwork\/aiot-pc-core['"]/u,
  'AIoT PC integration must delegate SDK clients to sdkwork-aiot-pc-core',
);
assert.match(
  aiotPcIntegrationSource,
  /getImHostedAiotAppSdkClient/u,
  'AIoT PC integration must expose IM-hosted AIoT SDK accessor',
);
assert.doesNotMatch(
  aiotPcIntegrationSource,
  /createAiotSdkPermissionParams|xSdkworkTenantId|xSdkworkOrganizationId/u,
  'AIoT PC integration must not map tenant or organization scope into generated SDK params',
);

assert.match(
  agentAppSdkClientSource,
  /from ['"]@sdkwork\/agents-app-sdk['"]/u,
  'agent app SDK wrapper must use the generated agent app SDK client',
);
assert.match(
  agentAppSdkClientSource,
  /createSdkworkChatRequestContextInterceptors/u,
  'agent app SDK wrapper must install the shared generated-SDK request interceptor',
);
assert.doesNotMatch(
  agentAppSdkClientSource,
  /headers:\s*buildSdkworkChatAppContextHeaders/u,
  'agent app SDK wrapper must not keep stale static AppContext header injection after token refresh',
);
assert.doesNotMatch(
  agentAppSdkClientSource,
  /tenantId:\s*resolveAppSdkTenantId|organizationId:\s*resolveAppSdkOrganizationId/u,
  'agent app SDK wrapper must not pass current tenantId/organizationId as static config; current scope belongs in the request Context interceptor',
);
assert.doesNotMatch(agentAppSdkClientSource, /\bfetch\s*\(/u, 'agent app SDK wrapper must not use raw fetch');

const appAuthSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/appAuthService.ts');
const appAuthRuntimeSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/appAuthRuntime.ts');
const appbaseAppSdkClientSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/appbaseAppSdkClient.ts');
assert.match(
  appbaseAppSdkClientSource,
  /from ['"]@sdkwork\/iam-app-sdk['"]/u,
  'appbase app SDK wrapper must use the generated sdkwork-iam-app-sdk client for appbase-owned app surfaces',
);
assert.match(
  appAuthSource,
  /import\s+\{[\s\S]*createSdkworkAuthAppbaseIntegration[\s\S]*\}\s+from\s+['"]@sdkwork\/auth-pc-react['"]/u,
  'auth service must integrate the appbase PC wrapper through the high-level auth appbase integration factory',
);
assert.doesNotMatch(
  appAuthSource,
  /@sdkwork\/appbase-pc-react|@sdkwork\/auth-pc-react\/auth|createSdkworkAppCapabilityPresetManifest|createAuthRouteCatalog|appbasePackageMeta|authPackageMeta/u,
  'auth service must not hand-build appbase/auth metadata or route catalogs after the high-level auth appbase integration migration',
);
assert.match(
  appAuthRuntimeSource,
  /@sdkwork\/auth-runtime-pc-react/u,
  'auth runtime must integrate the appbase high-level PC auth runtime package',
);
assert.match(
  appAuthRuntimeSource,
  /createSdkworkAppbasePcAuthRuntime/u,
  'auth runtime must create the appbase auth runtime through the standard high-level factory',
);
assert.match(
  appAuthRuntimeSource,
  /appId:\s*['"]sdkwork-im-pc['"]/u,
  'auth runtime must use the provisioned IAM runtime appId sdkwork-im-pc',
);
const standaloneGatewayMain = read('services/sdkwork-im-standalone-gateway/src/main.rs');
assert.match(
  standaloneGatewayMain,
  /ensure_im_tenant_application_runtime_from_env/u,
  'standalone gateway must provision IM IAM tenant application before serving credential-entry routes',
);
assert.match(
  sessionSource,
  /transientAccessToken/u,
  'session TokenManager must support transient credential-entry bootstrap access tokens without persisting partial sessions',
);
const imPcDevSource = read('scripts/lib/im-pc-dev.mjs');
const authRuntimePcReactSource = read('../sdkwork-iam/apps/sdkwork-iam-pc/packages/sdkwork-auth-runtime-pc-react/src/appbasePcAuthRuntime.ts');
assert.match(
  imPcDevSource,
  /mergeSdkworkImBootstrapAccessTokenEnv/u,
  'IM PC dev orchestration must inject private bootstrap SDKWORK_ACCESS_TOKEN before starting the renderer',
);
assert.match(
  authRuntimePcReactSource,
  /wrapCredentialEntryClient/u,
  'appbase PC auth runtime factory must wrap credential-entry IAM app SDK methods with bootstrap Access-Token preparation',
);
for (const method of ['logout', 'getCurrentSession']) {
  assert.match(appAuthSource, new RegExp(`\\b${method}\\s*\\(`, 'u'), `auth service must expose ${method}`);
}
for (const localAuthMethod of ['login', 'register', 'refreshToken', 'sendVerifyCode', 'verifyCode']) {
  assert.doesNotMatch(
    appAuthSource,
    new RegExp(`\\b${localAuthMethod}\\s*\\(`, 'u'),
    `auth service must not expose product-local ${localAuthMethod}; appbase runtime owns auth flows`,
  );
}
for (const generatedMethod of [
  'auth.sessions.create',
  'auth.registrations.create',
  'auth.sessions.refresh',
  'oauth.deviceAuthorizations.create',
  'oauth.deviceAuthorizations.retrieve',
  'oauth.deviceAuthorizations.scans.create',
  'oauth.deviceAuthorizations.passwordCompletions.create',
]) {
  assert.doesNotMatch(
    `${appAuthSource}\n${appAuthRuntimeSource}`,
    new RegExp(generatedMethod.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
    `PC product code must not remap ${generatedMethod}; appbase auth runtime owns auth validation and session creation`,
  );
}
assert.match(
  appAuthSource,
  /service\.auth\.sessions\.current\.retrieve[\s\S]*service\.auth\.sessions\.current\.delete/u,
  'auth service may only bridge current-session bootstrap and logout through the appbase runtime service',
);
assert.doesNotMatch(
  `${appAuthSource}\n${appAuthRuntimeSource}`,
  /@sdkwork\/iam-sdk-adapter|createIamAppSdkAdapter|unwrapIamSdkResponse|getIam\(\)/u,
  'auth integration must not import or call the lower-level IAM SDK adapter after appbase auth runtime migration',
);
assert.doesNotMatch(appAuthSource, /\bfetch\s*\(/u, 'auth service must not use raw fetch');
assert.doesNotMatch(appAuthSource, /PlusApiResult|LoginVO|assertSuccess/u, 'auth service must not keep generic spring app SDK envelope shims');

const pcImSdkClientSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/imSdkClient.ts');
assert.match(pcImSdkClientSource, /from ['"]@sdkwork\/im-sdk['"]/u, 'IM wrapper must use generated/composed @sdkwork/im-sdk');
assert.match(pcImSdkClientSource, /new ImSdkClient/u);
assert.match(pcImSdkClientSource, /getImSdkClientWithSession/u);
assert.match(pcImSdkClientSource, /tokenProvider:\s*tokenManager/u, 'IM wrapper must use the same dynamic token manager as IAM login');
assert.match(pcImSdkClientSource, /accessToken:\s*resolveAppSdkAccessToken/u, 'IM wrapper must pass accessToken from IAM login session');
assert.match(pcImSdkClientSource, /from ['"].\/sdkBaseUrls['"]/u, 'IM wrapper must resolve API base URLs through the shared topology resolver');
assert.match(pcImSdkClientSource, /resolveImApiBaseUrlOrThrow/u, 'IM wrapper must resolve IM API base URL through topology APPLICATION_PUBLIC env');
assert.match(pcImSdkClientSource, /resolveImWebSocketBaseUrlOrThrow/u, 'IM wrapper must resolve IM websocket base URL through topology APPLICATION_PUBLIC env');
assert.doesNotMatch(
  pcImSdkClientSource,
  /headerProvider:\s*\(\)\s*=>\s*buildImSdkContextHeaders|buildImSdkContextHeaders/u,
  'IM wrapper must not attach tenant/organization/user context through request headers',
);
assert.doesNotMatch(
  pcImSdkClientSource,
  /tenantId:\s*resolveAppSdkTenantId|organizationId:\s*resolveAppSdkOrganizationId/u,
  'IM wrapper must not pass current tenantId/organizationId as static options; server context comes from tokens',
);
assert.doesNotMatch(
  pcImSdkClientSource,
  /buildSdkworkChatAppContextHeaders/u,
  'IM wrapper must not use an AppContext header builder',
);
assert.doesNotMatch(
  pcImSdkClientSource,
  /VITE_SDKWORK_IAM_APP_API_BASE_URL/u,
  'IM wrapper must not fall back to the appbase App API URL directly',
);
assert.doesNotMatch(pcImSdkClientSource, /\bfetch\s*\(/u, 'IM wrapper must not use raw fetch');

const chatServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService.ts');
const imOpenApiSource = read('sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml');
const imSdkSource = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/sdk.ts');
const imConversationsModuleSource = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/conversations-module.ts');
const imMessagesModuleSource = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/messages-module.ts');
const imCallsModuleSource = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/calls-module.ts');
const imRoomsModuleSource = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/rooms-module.ts');
const imTransportClientLikeSource = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/transport-client-like.ts');
assert.match(chatServiceSource, /@sdkwork\/im-sdk/u, 'chat service must route IM behavior through @sdkwork/im-sdk types');
assert.match(chatServiceSource, /getImSdkClientWithSession/u, 'chat service must use the shared IM SDK client wrapper');
assert.match(
  chatServiceSource,
  /subscribeMessages|subscribeConversationMessages/u,
  'chat service must expose SDK-backed realtime message subscription',
);
assert.match(
  chatServiceSource,
  /subscribePcConversationMessages|recoverPcLiveConnection/u,
  'chat service must open IM realtime through @sdkwork/im-pc-core pcRealtimeConnectionManager',
);
assert.match(
  chatServiceSource,
  /subscribePcConversationMessages/u,
  'chat service must subscribe to conversation messages through the shared PC live connection manager',
);
assert.match(
  chatServiceSource,
  /startDirectChat/u,
  'chat service must expose a semantic SDK-backed direct chat starter for contact flows',
);
assert.match(
  chatServiceSource,
  /\.conversations\.bindDirectChat\s*\(/u,
  'chat service startDirectChat must bind direct chats through the generated IM SDK',
);
assert.match(
  chatServiceSource,
  /\.conversations\.updateProfile\s*\(\s*boundConversationId/u,
  'chat service startDirectChat must sync the direct chat display profile through the IM SDK',
);
assert.match(
  chatServiceSource,
  /\.conversations\.updatePreferences\s*\(\s*boundConversationId\s*,\s*\{\s*isHidden:\s*false\s*\}/u,
  'chat service startDirectChat must unhide the real direct chat conversation through the IM SDK',
);
assert.match(
  chatServiceSource,
  /mapReplyReferenceToMessageReply/u,
  'chat service must map standard MessageReplyReference values into the existing PC message reply shape',
);
assert.match(
  chatServiceSource,
  /buildReplyReference/u,
  'chat service must map PC reply state into the standard MessageReplyReference request shape',
);
assert.match(
  chatServiceSource,
  /entry\.body\?\.replyTo/u,
  'chat service must restore reply references from message history MessageBody.replyTo',
);
assert.match(
  chatServiceSource,
  /decodedMessage\.replyTo/u,
  'chat service must restore reply references from realtime decoded messages',
);
assert.match(
  chatServiceSource,
  /replyTo:\s*replyReference/u,
  'chat service must send reply references through the generated IM SDK message body',
);
assert.match(
  chatServiceSource,
  /function\s+buildMediaMessageParts\s*\(/u,
  'chat service rich send path must build standard IM media content parts for PC attachments',
);
assert.match(
  chatServiceSource,
  /kind:\s*['"]media['"]/u,
  'chat service rich send path must persist attachments as kind=media ContentPart values',
);
assert.match(
  chatServiceSource,
  /driveUri:[\s\r\n]*`drive:\/\/spaces\/\$\{spaceId\}\/nodes\/\$\{nodeId\}`/u,
  'chat service media ContentPart must include canonical Drive references returned by Drive upload',
);
assert.match(
  chatServiceSource,
  /uploadChatMediaFile\s*\([\s\S]*?getDriveUploader:[\s\S]*?this\.getDriveUploader[\s\S]*?type,[\s\S]*?\)/u,
  'chat service rich send path must upload media through Drive before building IM message parts',
);
assert.match(
  chatServiceSource,
  /buildMessageParts\s*\([\s\S]*?mediaUpload\?\.content\s*\?\?\s*content[\s\S]*?mediaUpload[\s\S]*?\)/u,
  'chat service rich send path must send Drive-backed media parts through the IM SDK postMessage request',
);
assert.match(
  chatServiceSource,
  /getDriveAppSdkClientWithSession/u,
  'chat service default uploader must consume the Drive app SDK bootstrap from PC core instead of constructing SDK clients locally',
);
assert.match(
  chatServiceSource,
  /from ['"]@sdkwork\/im-pc-core\/sdk\/driveAppSdkClient['"]/u,
  'chat service media upload types must come from the PC core Drive SDK public port',
);
assert.doesNotMatch(
  chatServiceSource,
  /from ['"]@sdkwork\/drive-app-sdk['"]/u,
  'chat service must not import the generated Drive SDK package directly',
);
assert.match(chatServiceSource, /const CHAT_DRIVE_SCENE = ['"]im['"]/u, 'chat service Drive uploads must use scene=im');
assert.match(
  chatServiceSource,
  /const CHAT_DRIVE_SOURCE = ['"]chat_message['"]/u,
  'chat service Drive uploads must tag source=chat_message for uploader statistics',
);
assert.match(
  chatServiceSource,
  /const CHAT_DRIVE_APP_RESOURCE_TYPE = ['"]im_conversation['"]/u,
  'chat service Drive uploads must bind files to IM conversation resources',
);
assert.match(
  chatServiceSource,
  /appResourceType:\s*CHAT_DRIVE_APP_RESOURCE_TYPE[\s\S]*appResourceId:\s*chatId[\s\S]*scene:\s*CHAT_DRIVE_SCENE[\s\S]*source:\s*CHAT_DRIVE_SOURCE/u,
  'chat service Drive uploader request must include IM conversation attribution with scene=im and source=chat_message',
);
assert.match(chatServiceSource, /uploadImage\s*\(/u, 'chat service image messages must upload through Drive uploader image flow');
assert.match(chatServiceSource, /uploadAudio\s*\(/u, 'chat service voice messages must upload through Drive uploader audio flow');
assert.match(chatServiceSource, /uploadVideo\s*\(/u, 'chat service video messages must upload through Drive uploader video flow');
assert.match(chatServiceSource, /uploadAttachment\s*\(/u, 'chat service file messages must upload through Drive uploader attachment flow');
assert.doesNotMatch(
  chatServiceSource,
  /drive:\/\/spaces\/\$\{[^}]*chatId|drive:\/\/spaces\/\$\{[^}]*content|normalizeResourceNodeSegment\(chatId|normalizeResourceNodeSegment\(content/u,
  'chat service must not synthesize Drive URIs from chat ids, content, or local previews',
);
assert.doesNotMatch(
  chatServiceSource,
  /\/app\/v3\/api\/drive|\/drive\/uploader|upload_sessions|download_grants/u,
  'chat service must not hand-code Drive HTTP paths or upload-session routes; it must use sdkwork-drive-app-sdk',
);
assert.doesNotMatch(
  chatServiceSource,
  /postMessage\s*\(\s*chatId\s*,\s*\{[\s\S]*?\n\s*text:\s*content\s*,[\s\S]*?renderHints:/u,
  'chat service media send path must not persist local object URLs as plain text bodies',
);
const pcRealtimeConnectionManagerSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/pcRealtimeConnectionManager.ts',
);
assert.match(
  pcRealtimeConnectionManagerSource,
  /lifecycle\.onStateChange/u,
  'PC live connection manager must observe IM live lifecycle so dropped realtime connections can be resubscribed',
);
assert.match(
  pcRealtimeConnectionManagerSource,
  /scheduleReconnect/u,
  'PC live connection manager must schedule reconnect after dropped IM live connections',
);
assert.match(
  pcRealtimeConnectionManagerSource,
  /connection\.subscriptions\.syncConversations\(conversationIds\)/u,
  'PC live connection manager must synchronize active conversations over the shared IM realtime connection',
);
assert.match(
  chatServiceSource,
  /recoverPcLiveConnection|onPcLiveConnectionOpen/u,
  'chat service must delegate dropped IM live recovery to the shared PC live connection manager',
);
assert.match(
  chatServiceSource,
  /const\s+CHAT_LIST_REALTIME_EVENT_TYPES\s*=\s*\[[\s\S]*['"]conversation\.member_joined['"]/u,
  'chat service conversation-list realtime subscription must include group member joined events so invitees refresh newly available groups',
);
for (const groupMemberEventType of [
  'conversation.member_joined',
  'conversation.member_role_changed',
  'conversation.member_removed',
  'conversation.member_left',
  'conversation.owner_transferred',
]) {
  assert.match(
    chatServiceSource,
    new RegExp(`CHAT_LIST_REALTIME_EVENT_TYPES[\\s\\S]*['"]${groupMemberEventType.replaceAll('.', '\\.')}['"]`, 'u'),
    `chat service conversation-list realtime subscription must include ${groupMemberEventType} so group management changes refresh the chat list`,
  );
}
const messageInputSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/MessageInput.tsx');
assert.doesNotMatch(
  messageInputSource,
  /mock to work|Sending message:|console\.log\s*\(/u,
  'message input must not keep mock-send branches or console-only fake delivery paths',
);
assert.match(
  messageInputSource,
  /type\?:\s*'text'\|'image'\|'file'\|'voice'\|'video'/u,
  'message input send contract must expose video as a first-class chat message type',
);
assert.match(
  messageInputSource,
  /file\.type\.startsWith\('video\/'\)/u,
  'message input must classify selected or dropped video files as video messages before calling ChatService',
);
assert.doesNotMatch(
  messageInputSource,
  /readBlobAsDataUrl|readAsDataURL|new\s+FileReader/u,
  'message input must not serialize attachments as data URLs; it must pass real File/Blob objects to ChatService for Drive upload',
);
assert.doesNotMatch(
  messageInputSource,
  /\bfetch\s*\(/u,
  'message input must not use browser raw fetch to turn remote sticker URLs into fake upload files',
);
assert.match(
  messageInputSource,
  /onSend\([^,]+,\s*type,\s*\{[^}]*file,/u,
  'message input file send path must pass the selected File object through extraInfo for Drive upload',
);
assert.match(
  messageInputSource,
  /sendFileMessage\s*\(\s*file\s*,\s*onSend\s*,\s*['"]voice['"]/u,
  'message input voice send path must pass the recorded voice Blob/File through extraInfo for Drive upload',
);
assert.match(
  messageInputSource,
  /URL\.createObjectURL/u,
  'message input may create browser-local blob URLs only as transient local previews while ChatService uploads the real File to Drive',
);
const voiceRecorderFailureStart = messageInputSource.indexOf('} catch (err)');
assert.notEqual(voiceRecorderFailureStart, -1, 'message input voice recorder failure path must remain auditable');
const voiceRecorderFailureSource = messageInputSource.slice(voiceRecorderFailureStart, messageInputSource.indexOf('  return (', voiceRecorderFailureStart));
assert.doesNotMatch(
  voiceRecorderFailureSource,
  /模拟语音|mock voice|startTimer\s*\(\s*\)/u,
  'message input must fail closed when microphone access is unavailable instead of starting a mock voice recording',
);
const settingsServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/SettingsService.ts');
assert.match(
  settingsServiceSource,
  /getAppSdkClientWithSession/u,
  'settings service remote configuration must use the generated im-app-sdk wrapper',
);
assert.match(
  settingsServiceSource,
  /retrievePortalHome\s*\(\s*this\.getClient\(\)\s*\)/u,
  'settings service module configuration must use the shared generated-SDK portal snapshot adapter',
);
assert.match(
  appSdkClientSource,
  /retrievePortalHome[\s\S]*?client\.portal\.home\.retrieve\s*\(\s*\)/u,
  'the shared portal snapshot adapter must call the generated app SDK operation',
);
assert.match(
  settingsServiceSource,
  /getImHostedAiotAppSdkClient/u,
  'settings service device capability must use the IM-hosted AIoT PC integration bridge',
);
assert.match(
  settingsServiceSource,
  /\.iot\.devices\.twin\.retrieve\s*\(/u,
  'settings service device list must read device state through sdkwork-aiot iot.devices.twin.retrieve',
);
assertNoImDeviceApiUsage(settingsServiceSource, 'settings service');
assert.doesNotMatch(
  settingsServiceSource,
  /\.device\.twin|\.device\.twin\.desired|getClient\(\)\.device/u,
  'settings service must not consume Sdkwork IM app SDK device twin after device ownership moves to sdkwork-aiot',
);
assert.doesNotMatch(settingsServiceSource, /class\s+MockSettingsService/u, 'settings service must not be mock-backed');
assert.doesNotMatch(settingsServiceSource, /\bfetch\s*\(/u, 'settings service must not use raw fetch');
assert.doesNotMatch(
  settingsServiceSource,
  /\btenantId\b|\borganizationId\b|resolveAppSdkTenantId|resolveAppSdkOrganizationId|xSdkworkTenantId|createAiotSdkPermissionParams/u,
  'settings service must not pass tenant or organization scope to sdkwork-aiot requests',
);
assert.doesNotMatch(settingsServiceSource, /\/api\/config\/modules/u, 'settings service must not hand-code module config paths');
assert.ok(
  !fs.existsSync(path.join(repoRoot, 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/DeviceSyncFeedService.ts')),
  'DeviceSyncFeedService must be removed because IM no longer owns device registration or device sync feed APIs',
);
const pcDevicesPackageJson = readJson('apps/sdkwork-im-pc/packages/sdkwork-im-pc-devices/package.json');
const pcDevicesAdapterSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-devices/src/DevicesView.tsx');
assert.equal(
  pcDevicesPackageJson.dependencies['@sdkwork/aiot-backend-sdk'],
  undefined,
  'pc-devices is not backend-admin and must not depend on sdkwork-aiot backend SDK',
);
assert.ok(
  pcDevicesPackageJson.dependencies['@sdkwork/aiot-pc-console-device'],
  'pc-devices adapter must depend on canonical AIoT PC device package',
);
assert.match(
  pcDevicesAdapterSource,
  /SdkworkDevicePage/u,
  'pc-devices adapter must embed canonical AIoT device page only',
);
assert.ok(
  !fs.existsSync(path.join(repoRoot, 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-devices/src/services/DeviceService.ts')),
  'pc-devices adapter must not keep duplicate DeviceService implementation',
);
assert.ok(
  !fs.existsSync(path.join(repoRoot, 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-devices/src/components/BindAgentModal.tsx')),
  'pc-devices adapter must not keep duplicate bind-agent UI',
);
assert.ok(
  !fs.existsSync(path.join(repoRoot, 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-devices/src/components/DeviceDetailPanel.tsx')),
  'pc-devices adapter must not keep duplicate device detail UI',
);
const pcDevicesServiceSource = readSibling(
  'sdkwork-aiot/apps/sdkwork-aiot-pc/packages/sdkwork-aiot-pc-console-device/src/device-service.ts',
);
assert.doesNotMatch(
  pcDevicesServiceSource,
  /@sdkwork\/aiot-backend-sdk|backendClient|BackendClient|getBackendClient/u,
  'canonical AIoT device service must not import or route through backend SDK clients',
);
assert.match(
  pcDevicesServiceSource,
  /listDevicePage/u,
  'canonical AIoT device service must list devices through paginated AIoT app SDK helpers',
);
assert.doesNotMatch(
  pcDevicesServiceSource,
  /\btenantId\b|\borganizationId\b|resolveAppSdkTenantId|resolveAppSdkOrganizationId|xSdkworkTenantId|createAiotSdkPermissionParams/u,
  'canonical AIoT device service must not pass tenant or organization scope to sdkwork-aiot requests',
);
assertNoImDeviceApiUsage(chatServiceSource, 'chat service');
assert.match(chatServiceSource, /syncOfflineMessages/u, 'chat service must expose offline message window sync');
const syncOfflineMessagesImplementation = chatServiceSource.match(
  /async\s+syncOfflineMessages\([^{]*\{([\s\S]*?)\n\s+recoverRealtimeConnection/u,
)?.[1] ?? '';
assert.match(
  syncOfflineMessagesImplementation,
  /this\.listChatsPage\s*\(\s*\)/u,
  'chat service startup sync must refresh only the first IM inbox page through listChatsPage',
);
assert.doesNotMatch(
  syncOfflineMessagesImplementation,
  /this\.getChats\s*\(\s*\)|forEachCursorPage/u,
  'chat service startup sync must not scan all inbox pages when many conversations exist',
);
assert.doesNotMatch(
  syncOfflineMessagesImplementation,
  /getMessages\s*\(|conversations\.listMessages\s*\(/u,
  'chat service startup sync must not preload message history for every inbox conversation',
);
assert.match(
  chatServiceSource,
  /\.conversations\.listMessages\s*\(/u,
  'chat service active conversation message history must load through generated IM conversation messages APIs',
);
assert.doesNotMatch(chatServiceSource, /class MockChatService/u, 'chat service must not be mock-backed');
assert.doesNotMatch(chatServiceSource, /mockChats|mockMessages/u, 'chat service must not keep mock branches');
assert.doesNotMatch(
  chatServiceSource,
  /\btenantId\s*:/u,
  'chat service must not pass tenantId in SDK requests; server context comes from AuthToken and Access-Token',
);
assert.doesNotMatch(
  chatServiceSource,
  /resolveAppSdkTenantId/u,
  'chat service must not derive tenant scope in the frontend',
);
for (const requiredOperation of [
  'messages.reactions.create',
  'messages.reactions.remove',
  'messages.pin',
  'messages.unpin',
  'messages.visibility.delete',
  'messages.favorites.list',
  'messages.favorites.create',
  'messages.favorites.delete',
  'conversations.messages.interactionSummary.retrieve',
  'conversations.pins.list',
  'conversations.profile.retrieve',
  'conversations.profile.update',
  'conversations.preferences.retrieve',
  'conversations.preferences.update',
  'social.contacts.preferences.retrieve',
  'social.contacts.preferences.update',
  'social.contacts.tags.list',
  'social.contacts.tags.create',
  'social.contacts.tags.update',
  'social.contacts.tags.delete',
  'social.contacts.recommendations.create',
  'calls.sessions.retrieve',
  'rooms.create',
  'rooms.retrieve',
  'rooms.enter',
  'rooms.leave',
]) {
  assert.match(
    imOpenApiSource,
    new RegExp(requiredOperation.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
    `IM OpenAPI must expose SDKWork v3 operationId ${requiredOperation}`,
  );
}
assert.match(
  imCallsModuleSource,
  /retrieve\s*\(\s*rtcSessionId:\s*string\s*\)\s*:\s*Promise<RtcSession>/u,
  'IM SDK calls module must expose a semantic retrieve method for call session state backfill',
);
assert.match(
  imCallsModuleSource,
  /transportClient\.calls\.sessions\.retrieve\s*\(\s*requireStringIdentifier\s*\(\s*rtcSessionId,\s*['"]rtcSessionId['"]\s*\)\s*\)/u,
  'IM SDK calls.retrieve must validate rtcSessionId before delegating to the generated calls.sessions.retrieve transport method',
);
assert.match(
  imTransportClientLikeSource,
  /retrieve\s*\(\s*rtcSessionId:\s*string\s*\)\s*:\s*Promise<RtcSession>/u,
  'IM SDK transport client contract must include generated calls.sessions.retrieve with string-only identifiers',
);
for (const requiredSchema of [
  'MessageReactionRequest',
  'MessageReactionMutationResult',
  'MessagePinMutationResult',
  'MessageFavoriteType',
  'FavoriteMessageRequest',
  'MessageFavoriteView',
  'MessageInteractionSummaryView',
  'MessageReactionCountView',
  'MessagePinView',
  'MessageBody',
  'MessageReplyReference',
  'MessageType',
  'ConversationMessageEntry',
  'ConversationInboxPeerView',
  'ConversationInboxPreferencesView',
  'ConversationProfileView',
  'UpdateConversationProfileRequest',
  'ConversationPreferencesView',
  'UpdateConversationPreferencesRequest',
  'ContactPreferencesView',
  'UpdateContactPreferencesRequest',
  'ContactRecommendationView',
  'ContactTagView',
  'CreateContactRecommendationRequest',
  'CreateContactTagRequest',
  'UpdateContactTagRequest',
  'CreateRoomRequest',
  'RoomView',
  'EnterRoomResponse',
]) {
  assert.match(
    imOpenApiSource,
    new RegExp(`\\b${requiredSchema}\\b`, 'u'),
    `IM OpenAPI must define ${requiredSchema} for generated SDK typing`,
  );
}
const conversationMessageEntrySchema = extractYamlSchemaBlock(imOpenApiSource, 'ConversationMessageEntry');
for (const requiredField of [
  'sender',
  'body',
  'messageType',
  'deliveryMode',
  'occurredAt',
]) {
  assert.match(
    conversationMessageEntrySchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `ConversationMessageEntry must expose ${requiredField} so PC history does not fall back to local mock fields`,
  );
  assert.match(
    conversationMessageEntrySchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `ConversationMessageEntry must require ${requiredField} in the IM OpenAPI contract`,
  );
}
assert.match(
  conversationMessageEntrySchema,
  /sender:\s*\n\s+\$ref:\s*'#\/components\/schemas\/Sender'/u,
  'ConversationMessageEntry.sender must reuse the standard Sender schema',
);
assert.match(
  conversationMessageEntrySchema,
  /body:\s*\n\s+\$ref:\s*'#\/components\/schemas\/MessageBody'/u,
  'ConversationMessageEntry.body must reuse the standard MessageBody schema',
);
const messageBodySchema = extractYamlSchemaBlock(imOpenApiSource, 'MessageBody');
assert.match(
  messageBodySchema,
  /replyTo:\s*\n\s+\$ref:\s*'#\/components\/schemas\/MessageReplyReference'/u,
  'MessageBody.replyTo must use the standard reply reference schema so PC reply previews survive sync',
);
const postMessageRequestSchema = extractYamlSchemaBlock(imOpenApiSource, 'PostMessageRequest');
assert.match(
  postMessageRequestSchema,
  /replyTo:\s*\n\s+\$ref:\s*'#\/components\/schemas\/MessageReplyReference'/u,
  'PostMessageRequest.replyTo must use the standard reply reference schema so PC sendMessage persists replies',
);
const editMessageRequestSchema = extractYamlSchemaBlock(imOpenApiSource, 'EditMessageRequest');
assert.match(
  editMessageRequestSchema,
  /replyTo:\s*\n\s+\$ref:\s*'#\/components\/schemas\/MessageReplyReference'/u,
  'EditMessageRequest.replyTo must use the standard reply reference schema for future reply-aware edits',
);
const messageReplyReferenceSchema = extractYamlSchemaBlock(imOpenApiSource, 'MessageReplyReference');
for (const requiredField of ['messageId', 'senderDisplayName', 'contentPreview']) {
  assert.match(
    messageReplyReferenceSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `MessageReplyReference must expose ${requiredField} for PC reply rendering`,
  );
  assert.match(
    messageReplyReferenceSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `MessageReplyReference must require ${requiredField} for deterministic reply sync`,
  );
}
assert.match(
  conversationMessageEntrySchema,
  /messageType:\s*\n\s+\$ref:\s*'#\/components\/schemas\/MessageType'/u,
  'ConversationMessageEntry.messageType must reuse the standard MessageType schema',
);
assert.match(
  extractYamlSchemaBlock(imOpenApiSource, 'MessageType'),
  /enum:\s*\n\s+- standard\s*\n\s+- system\s*\n\s+- signal/u,
  'MessageType must standardize standard/system/signal values for generated SDK typing',
);
const conversationInboxEntrySchema = extractYamlSchemaBlock(imOpenApiSource, 'ConversationInboxEntry');
for (const projectedField of [
  'displayName',
  'avatarUrl',
  'displaySource',
  'peer',
  'preferences',
]) {
  assert.match(
    conversationInboxEntrySchema,
    new RegExp(`\\b${projectedField}:`, 'u'),
    `ConversationInboxEntry must expose ${projectedField} so PC chat list titles are display-ready without N+1 hydration`,
  );
}
assert.match(
  conversationInboxEntrySchema,
  /peer:\s*\n\s+\$ref:\s*'#\/components\/schemas\/ConversationInboxPeerView'/u,
  'ConversationInboxEntry.peer must use the standard inbox peer projection schema',
);
assert.match(
  conversationInboxEntrySchema,
  /preferences:\s*\n\s+\$ref:\s*'#\/components\/schemas\/ConversationInboxPreferencesView'/u,
  'ConversationInboxEntry.preferences must use the standard inbox preferences projection schema',
);
const inboxListOperation = extractYamlOperationBlock(imOpenApiSource, '/im/v3/api/chat/inbox', 'get');
assert.match(
  inboxListOperation,
  /#\/components\/parameters\/ConversationTypeQuery/u,
  'IM OpenAPI inbox list must expose server-side conversation_type filtering so group lists do not page the mixed inbox locally',
);
const conversationInboxPeerViewSchema = extractYamlSchemaBlock(imOpenApiSource, 'ConversationInboxPeerView');
for (const requiredField of ['principalKind', 'principalId']) {
  assert.match(
    conversationInboxPeerViewSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `ConversationInboxPeerView must expose ${requiredField} for per-viewer direct chat display`,
  );
  assert.match(
    conversationInboxPeerViewSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `ConversationInboxPeerView must require ${requiredField} for deterministic direct chat display`,
  );
}
for (const optionalField of ['userId', 'chatId', 'displayName', 'avatarUrl', 'relationshipState']) {
  assert.match(
    conversationInboxPeerViewSchema,
    new RegExp(`\\b${optionalField}:`, 'u'),
    `ConversationInboxPeerView must expose optional ${optionalField} for professional chat list rendering`,
  );
}
const conversationInboxPreferencesViewSchema = extractYamlSchemaBlock(
  imOpenApiSource,
  'ConversationInboxPreferencesView',
);
for (const requiredField of ['isPinned', 'isMuted', 'isMarkedUnread', 'isHidden']) {
  assert.match(
    conversationInboxPreferencesViewSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `ConversationInboxPreferencesView must expose ${requiredField} to avoid per-conversation preference reads`,
  );
  assert.match(
    conversationInboxPreferencesViewSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `ConversationInboxPreferencesView must require ${requiredField} for deterministic chat list state`,
  );
}
const conversationProfileViewSchema = extractYamlSchemaBlock(imOpenApiSource, 'ConversationProfileView');
for (const requiredField of [
  'tenantId',
  'conversationId',
  'displayName',
  'avatarUrl',
  'notice',
  'updatedAt',
]) {
  assert.match(
    conversationProfileViewSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `ConversationProfileView must expose ${requiredField} so PC group profile sync is typed`,
  );
  assert.match(
    conversationProfileViewSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `ConversationProfileView must require ${requiredField} for deterministic group profile sync`,
  );
}
const updateConversationProfileRequestSchema = extractYamlSchemaBlock(
  imOpenApiSource,
  'UpdateConversationProfileRequest',
);
for (const optionalField of ['displayName', 'avatarUrl', 'notice']) {
  assert.match(
    updateConversationProfileRequestSchema,
    new RegExp(`\\b${optionalField}:`, 'u'),
    `UpdateConversationProfileRequest must expose partial ${optionalField} updates`,
  );
}
assert.doesNotMatch(
  updateConversationProfileRequestSchema,
  /\n\s+required:/u,
  'UpdateConversationProfileRequest must remain a partial update request',
);
const conversationPreferencesViewSchema = extractYamlSchemaBlock(imOpenApiSource, 'ConversationPreferencesView');
for (const requiredField of [
  'tenantId',
  'conversationId',
  'principalKind',
  'principalId',
  'isPinned',
  'isMuted',
  'isMarkedUnread',
  'isHidden',
  'updatedAt',
]) {
  assert.match(
    conversationPreferencesViewSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `ConversationPreferencesView must expose ${requiredField} so PC chat preference sync is typed`,
  );
  assert.match(
    conversationPreferencesViewSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `ConversationPreferencesView must require ${requiredField} for deterministic per-user chat preference sync`,
  );
}
const updateConversationPreferencesRequestSchema = extractYamlSchemaBlock(
  imOpenApiSource,
  'UpdateConversationPreferencesRequest',
);
for (const optionalField of ['isPinned', 'isMuted', 'isMarkedUnread', 'isHidden']) {
  assert.match(
    updateConversationPreferencesRequestSchema,
    new RegExp(`\\b${optionalField}:`, 'u'),
    `UpdateConversationPreferencesRequest must expose partial ${optionalField} updates`,
  );
}
assert.doesNotMatch(
  updateConversationPreferencesRequestSchema,
  /\n\s+required:/u,
  'UpdateConversationPreferencesRequest must remain a partial update request',
);
const contactPreferencesViewSchema = extractYamlSchemaBlock(imOpenApiSource, 'ContactPreferencesView');
for (const requiredField of [
  'tenantId',
  'ownerUserId',
  'targetUserId',
  'isStarred',
  'remark',
  'isBlocked',
  'updatedAt',
]) {
  assert.match(
    contactPreferencesViewSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `ContactPreferencesView must expose ${requiredField} so PC contact preference sync is typed`,
  );
  assert.match(
    contactPreferencesViewSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `ContactPreferencesView must require ${requiredField} for deterministic per-user contact preference sync`,
  );
}
const updateContactPreferencesRequestSchema = extractYamlSchemaBlock(
  imOpenApiSource,
  'UpdateContactPreferencesRequest',
);
for (const optionalField of ['isStarred', 'remark', 'isBlocked']) {
  assert.match(
    updateContactPreferencesRequestSchema,
    new RegExp(`\\b${optionalField}:`, 'u'),
    `UpdateContactPreferencesRequest must expose partial ${optionalField} updates`,
  );
}
assert.match(
  updateContactPreferencesRequestSchema,
  /remark:\s*\n\s+maxLength:\s*256/u,
  'UpdateContactPreferencesRequest.remark must be length bounded for user-controlled contact display text',
);
assert.doesNotMatch(
  updateContactPreferencesRequestSchema,
  /\n\s+required:/u,
  'UpdateContactPreferencesRequest must remain a partial update request',
);
const contactTagViewSchema = extractYamlSchemaBlock(imOpenApiSource, 'ContactTagView');
for (const requiredField of [
  'tenantId',
  'ownerUserId',
  'tagId',
  'name',
  'color',
  'count',
  'bg',
  'border',
  'createdAt',
  'updatedAt',
]) {
  assert.match(
    contactTagViewSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `ContactTagView must expose ${requiredField} so PC contact tag sync is typed`,
  );
  assert.match(
    contactTagViewSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `ContactTagView must require ${requiredField} for deterministic per-user contact tag sync`,
  );
}
const createContactTagRequestSchema = extractYamlSchemaBlock(imOpenApiSource, 'CreateContactTagRequest');
for (const requiredField of ['name', 'color']) {
  assert.match(
    createContactTagRequestSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `CreateContactTagRequest must expose ${requiredField} for PC contact tag creation`,
  );
  assert.match(
    createContactTagRequestSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `CreateContactTagRequest must require ${requiredField} for PC contact tag creation`,
  );
}
for (const optionalField of ['count', 'bg', 'border']) {
  assert.match(
    createContactTagRequestSchema,
    new RegExp(`\\b${optionalField}:`, 'u'),
    `CreateContactTagRequest must expose optional ${optionalField} for PC tag styling parity`,
  );
}
const updateContactTagRequestSchema = extractYamlSchemaBlock(imOpenApiSource, 'UpdateContactTagRequest');
for (const optionalField of ['name', 'color', 'count', 'bg', 'border']) {
  assert.match(
    updateContactTagRequestSchema,
    new RegExp(`\\b${optionalField}:`, 'u'),
    `UpdateContactTagRequest must expose partial ${optionalField} updates for PC contact tags`,
  );
}
assert.doesNotMatch(
  updateContactTagRequestSchema,
  /\n\s+required:/u,
  'UpdateContactTagRequest must remain a partial update request',
);
const contactRecommendationViewSchema = extractYamlSchemaBlock(imOpenApiSource, 'ContactRecommendationView');
for (const requiredField of [
  'tenantId',
  'ownerUserId',
  'targetUserId',
  'recommendationId',
  'createdAt',
]) {
  assert.match(
    contactRecommendationViewSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `ContactRecommendationView must expose ${requiredField} so PC contact recommendation sync is typed`,
  );
  assert.match(
    contactRecommendationViewSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `ContactRecommendationView must require ${requiredField} for deterministic contact recommendation sync`,
  );
}
const blockUserRequestSchema = extractYamlSchemaBlock(imOpenApiSource, 'BlockUserRequest');
for (const requiredField of ['blockedUserId', 'scope']) {
  assert.match(
    blockUserRequestSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `BlockUserRequest must expose ${requiredField} for SDK-backed blacklist creation`,
  );
  assert.match(
    blockUserRequestSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `BlockUserRequest must require ${requiredField} for SDK-backed blacklist creation`,
  );
}
for (const optionalField of ['directChatId', 'expiresAt']) {
  assert.match(
    blockUserRequestSchema,
    new RegExp(`\\b${optionalField}:`, 'u'),
    `BlockUserRequest must expose optional ${optionalField} for scoped user blocks`,
  );
}
assert.match(
  extractYamlSchemaBlock(imOpenApiSource, 'BlockScope'),
  /enum:\s*\n\s+- all\s*\n\s+- friendship\s*\n\s+- direct_chat/u,
  'BlockScope must expose the runtime social block scopes',
);
const userBlockSchema = extractYamlSchemaBlock(imOpenApiSource, 'UserBlock');
for (const requiredField of [
  'tenantId',
  'blockId',
  'blockerUserId',
  'blockedUserId',
  'scope',
  'status',
  'createdAt',
  'updatedAt',
]) {
  assert.match(
    userBlockSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `UserBlock must expose ${requiredField} so SDK consumers can inspect blacklist writes`,
  );
  assert.match(
    userBlockSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `UserBlock must require ${requiredField} for deterministic blacklist write responses`,
  );
}
const userBlockCreateOperation = extractYamlOperationBlock(
  imOpenApiSource,
  '/im/v3/api/social/user_blocks',
  'post',
);
assert.match(
  userBlockCreateOperation,
  /operationId:\s*social\.userBlocks\.create/u,
  'social user block create path must expose the generated social.userBlocks.create SDK method',
);
assert.match(
  userBlockCreateOperation,
  /\$ref:\s*'#\/components\/schemas\/BlockUserRequest'/u,
  'social.userBlocks.create must accept the BlockUserRequest contract',
);
assert.match(
  userBlockCreateOperation,
  /\$ref:\s*'#\/components\/schemas\/OpenApiUserBlockResponse'/u,
  'social.userBlocks.create must return the runtime OpenApiUserBlockResponse contract',
);
const userBlockDeleteOperation = extractYamlOperationBlock(
  imOpenApiSource,
  '/im/v3/api/social/user_blocks/{blockId}',
  'delete',
);
assert.match(
  userBlockDeleteOperation,
  /operationId:\s*social\.userBlocks\.delete/u,
  'social user block delete path must expose the generated social.userBlocks.delete SDK method',
);
assert.match(
  userBlockDeleteOperation,
  /'204':\s*\n\s+description: No Content/u,
  'social.userBlocks.delete must return HTTP 204 No Content for SDKWork delete semantics',
);
assert.doesNotMatch(
  userBlockDeleteOperation,
  /application\/json/u,
  'social.userBlocks.delete must not declare a JSON success body',
);
const messageVisibilityDeleteOperation = extractYamlOperationBlock(
  imOpenApiSource,
  '/im/v3/api/chat/messages/{messageId}/visibility',
  'delete',
);
assert.match(
  messageVisibilityDeleteOperation,
  /'204':\s*\n\s+description: No Content/u,
  'messages.visibility.delete must return HTTP 204 No Content for SDKWork delete semantics',
);
assert.doesNotMatch(
  messageVisibilityDeleteOperation,
  /application\/json|MessageVisibilityMutationResult|SdkWorkApiResponse/u,
  'messages.visibility.delete must not declare a JSON success body',
);
assert.doesNotMatch(
  imOpenApiSource,
  /\bMessageVisibilityMutationResult:/u,
  'IM OpenAPI must not keep the retired MessageVisibilityMutationResult HTTP success schema',
);
const createContactRecommendationRequestSchema = extractYamlSchemaBlock(
  imOpenApiSource,
  'CreateContactRecommendationRequest',
);
assert.match(
  createContactRecommendationRequestSchema,
  /targetConversationId:/u,
  'CreateContactRecommendationRequest must expose targetConversationId for conversation-scoped contact recommendations',
);
const favoriteMessageRequestSchema = extractYamlSchemaBlock(imOpenApiSource, 'FavoriteMessageRequest');
for (const requiredField of ['conversationId', 'favoriteType', 'title', 'contentPreview', 'sourceDisplayName']) {
  assert.match(
    favoriteMessageRequestSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `FavoriteMessageRequest must expose ${requiredField} for PC message favorite sync`,
  );
  assert.match(
    favoriteMessageRequestSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `FavoriteMessageRequest must require ${requiredField} for deterministic PC message favorite sync`,
  );
}
assert.match(
  extractYamlSchemaBlock(imOpenApiSource, 'MessageFavoriteType'),
  /enum:\s*\n\s+- link\s*\n\s+- image\s*\n\s+- file\s*\n\s+- chat/u,
  'MessageFavoriteType must standardize the PC favorite filter taxonomy',
);
const messageFavoriteViewSchema = extractYamlSchemaBlock(imOpenApiSource, 'MessageFavoriteView');
for (const requiredField of [
  'tenantId',
  'principalKind',
  'principalId',
  'favoriteId',
  'favoriteType',
  'conversationId',
  'messageId',
  'messageSeq',
  'title',
  'contentPreview',
  'sourceDisplayName',
  'favoritedAt',
]) {
  assert.match(
    messageFavoriteViewSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `MessageFavoriteView must expose ${requiredField} so PC favorites are principal-scoped and renderable`,
  );
  assert.match(
    messageFavoriteViewSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `MessageFavoriteView must require ${requiredField} for deterministic PC favorites sync`,
  );
}
for (const sdkMethod of ['addReaction', 'removeReaction', 'pinMessage', 'unpinMessage']) {
  assert.match(
    imSdkSource,
    new RegExp(`\\b${sdkMethod}\\s*\\(`, 'u'),
    `IM SDK client must expose semantic ${sdkMethod} method`,
  );
  assert.match(
    imMessagesModuleSource,
    new RegExp(`\\b${sdkMethod}\\s*\\(`, 'u'),
    `IM messages module must implement semantic ${sdkMethod} method`,
  );
}
assert.match(
  imSdkSource,
  /\bdeleteMessageForMe\s*\(/u,
  'IM SDK client must expose semantic deleteMessageForMe method for current-principal message visibility delete',
);
assert.match(
  imMessagesModuleSource,
  /\bdeleteForMe\s*\(/u,
  'IM messages module must implement semantic deleteForMe method for current-principal message visibility delete',
);
assert.match(
  imMessagesModuleSource,
  /chat\.messages\.visibility\.delete\s*\(/u,
  'IM messages module deleteForMe must route through the generated chat.messages.visibility.delete transport resource',
);
assert.match(
  imMessagesModuleSource,
  /\bdeleteForMe\s*\(\s*messageId:\s*string\s*\)\s*:\s*Promise<void>/u,
  'IM messages module deleteForMe must expose a string-only 204 delete contract as Promise<void>',
);
assert.match(
  imSdkSource,
  /\bdeleteMessageForMe\s*\(\s*messageId:\s*string\s*\)\s*:\s*Promise<void>/u,
  'IM SDK client deleteMessageForMe must expose a string-only 204 delete contract as Promise<void>',
);
for (const sdkMethod of ['listFavorites', 'favoriteMessage', 'deleteFavorite']) {
  assert.match(
    imMessagesModuleSource,
    new RegExp(`\\b${sdkMethod}\\s*\\(`, 'u'),
    `IM messages module must implement semantic ${sdkMethod} method for message favorites`,
  );
}
assert.match(
  imMessagesModuleSource,
  /readonly\s+favorites\s*=/u,
  'IM messages module must expose messages.favorites as the app-facing composed SDK resource',
);
assert.match(
  imMessagesModuleSource,
  /chat\.messages\.favorites\.list\s*\(/u,
  'IM messages module listFavorites must route through generated chat.messages.favorites.list',
);
assert.match(
  imMessagesModuleSource,
  /chat\.messages\.favorites\.create\s*\(/u,
  'IM messages module favoriteMessage must route through generated chat.messages.favorites.create',
);
assert.match(
  imMessagesModuleSource,
  /chat\.messages\.favorites\.delete\s*\(/u,
  'IM messages module deleteFavorite must route through generated chat.messages.favorites.delete',
);
for (const sdkMethod of ['listMessageFavorites', 'favoriteMessage', 'deleteMessageFavorite']) {
  assert.match(
    imSdkSource,
    new RegExp(`\\b${sdkMethod}\\s*\\(`, 'u'),
    `IM SDK client must expose semantic ${sdkMethod} method for message favorites`,
  );
}
for (const sdkMethod of ['getMessageInteractionSummary', 'listPinnedMessages']) {
  assert.match(
    imConversationsModuleSource,
    new RegExp(`\\b${sdkMethod}\\s*\\(`, 'u'),
    `IM conversations module must implement semantic ${sdkMethod} method`,
  );
}
for (const sdkMethod of ['getPreferences', 'updatePreferences']) {
  assert.match(
    imConversationsModuleSource,
    new RegExp(`\\b${sdkMethod}\\s*\\(`, 'u'),
    `IM conversations module must implement semantic ${sdkMethod} method for per-user chat preferences`,
  );
}
for (const sdkMethod of ['getProfile', 'updateProfile']) {
  assert.match(
    imConversationsModuleSource,
    new RegExp(`\\b${sdkMethod}\\s*\\(`, 'u'),
    `IM conversations module must implement semantic ${sdkMethod} method for shared group profile sync`,
  );
}
for (const sdkMethod of ['create', 'get', 'enter', 'leave']) {
  assert.match(
    imRoomsModuleSource,
    new RegExp(`\\b${sdkMethod}\\s*\\(`, 'u'),
    `IM rooms module must implement semantic ${sdkMethod} method`,
  );
}
assert.match(
  imSdkSource,
  /readonly rooms: ImRoomsModule/u,
  'IM SDK client must expose composed rooms module for live, chat, and game room flows',
);
assert.match(
  imRoomsModuleSource,
  /transportClient\.chat\.rooms\.create\s*\(/u,
  'IM rooms module create must route through generated chat.rooms.create transport resource',
);
assert.match(
  imRoomsModuleSource,
  /transportClient\.chat\.rooms\.retrieve\s*\(/u,
  'IM rooms module get must route through generated chat.rooms.retrieve transport resource',
);
assert.match(
  imRoomsModuleSource,
  /transportClient\.chat\.rooms\.enter\s*\(/u,
  'IM rooms module enter must route through generated chat.rooms.enter transport resource',
);
assert.match(
  imTransportClientLikeSource,
  /rooms:\s*\{[\s\S]*create\(body: ImCreateRoomRequest\): Promise<CreateConversationResult>;/u,
  'IM transport client type must expose composed chat.rooms create resource with server-owned conversation id support',
);
assert.match(
  imTransportClientLikeSource,
  /retrieve\(roomId: string\): Promise<RoomView>;/u,
  'IM transport client type must expose generated chat.rooms retrieve resource with a string-only roomId',
);
assert.match(
  imTransportClientLikeSource,
  /enter\(roomId: string\): Promise<EnterRoomResponse>;/u,
  'IM transport client type must expose generated chat.rooms enter resource with a string-only roomId',
);
assert.match(
  imTransportClientLikeSource,
  /leave\(roomId: string\): Promise<EnterRoomResponse>;/u,
  'IM transport client type must expose generated chat.rooms leave resource with a string-only roomId',
);
assert.match(
  imTransportClientLikeSource,
  /reactions:\s*\{/u,
  'IM transport client type must expose generated chat.messages.reactions resource',
);
assert.match(
  imTransportClientLikeSource,
  /pin\(messageId: string\): Promise<MessagePinMutationResult>;/u,
  'IM transport client type must expose generated chat.messages.pin method with a string-only messageId',
);
assert.match(
  imTransportClientLikeSource,
  /unpin\(messageId: string\): Promise<MessagePinMutationResult>;/u,
  'IM transport client type must expose generated chat.messages.unpin method with a string-only messageId',
);
assert.match(
  imTransportClientLikeSource,
  /visibility:\s*\{[\s\S]*delete\(messageId: string\): Promise<void>;/u,
  'IM transport client type must expose generated chat.messages.visibility delete with a string-only messageId as Promise<void>',
);
assert.match(
  imTransportClientLikeSource,
  /favorites:\s*\{[\s\S]*list\(params\?: QueryParams & \{ favoriteType\?: MessageFavoriteType \}\): Promise<FavoriteMessagesResponse>;/u,
  'IM transport client type must expose generated chat.messages.favorites list resource',
);
assert.match(
  imTransportClientLikeSource,
  /create\(messageId: string, body: FavoriteMessageRequest\): Promise<MessageFavoriteView>;/u,
  'IM transport client type must expose generated chat.messages.favorites create resource with a string-only messageId',
);
assert.match(
  imTransportClientLikeSource,
  /delete\(favoriteId: string\): Promise<DeleteMessageFavoriteResponse>;/u,
  'IM transport client type must expose generated chat.messages.favorites delete resource with a string-only favoriteId',
);
assert.match(
  imTransportClientLikeSource,
  /interactionSummary:\s*\{/u,
  'IM transport client type must expose generated chat.conversations.messages.interactionSummary resource',
);
assert.match(
  imTransportClientLikeSource,
  /pins:\s*\{/u,
  'IM transport client type must expose generated chat.conversations.pins resource',
);
assert.match(
  imTransportClientLikeSource,
  /preferences:\s*\{/u,
  'IM transport client type must expose generated chat.conversations.preferences resource',
);
assert.match(
  imTransportClientLikeSource,
  /profile:\s*\{/u,
  'IM transport client type must expose generated chat.conversations.profile resource',
);
assert.match(
  imTransportClientLikeSource,
  /tags:\s*\{[\s\S]*list\(params\?: QueryParams\): Promise<ContactTagsResponse>;/u,
  'IM transport client type must expose generated social.contacts.tags list resource',
);
assert.match(
  imTransportClientLikeSource,
  /create\(body: CreateContactTagRequest\): Promise<ContactTagView>;/u,
  'IM transport client type must expose generated social.contacts.tags create resource',
);
assert.match(
  imTransportClientLikeSource,
  /update\(\s*tagId: string,\s*body: UpdateContactTagRequest,?\s*\): Promise<ContactTagView>;/u,
  'IM transport client type must expose generated social.contacts.tags update resource with a string-only tagId',
);
assert.match(
  imTransportClientLikeSource,
  /delete\(tagId: string\): Promise<DeleteContactTagResponse>;/u,
  'IM transport client type must expose generated social.contacts.tags delete resource with a string-only tagId',
);
assert.match(
  imTransportClientLikeSource,
  /recommendations:\s*\{[\s\S]*create\(\s*targetUserId: string,\s*body: CreateContactRecommendationRequest,?\s*\): Promise<ContactRecommendationView>;/u,
  'IM transport client type must expose generated social.contacts.recommendations create resource with a string-only targetUserId',
);
assert.match(
  imTransportClientLikeSource,
  /users:\s*\{[\s\S]*list\(params\?:\s*\{[\s\S]*q\?: string;[\s\S]*pageSize\?: number;[\s\S]*cursor\?: string;[\s\S]*\}\): Promise<SocialUserSearchResponse>;/u,
  'IM transport client type must expose generated social.users.list for add-friend search',
);
function extractChatServiceCommand(methodName, nextMethodName) {
  const methodStart = chatServiceSource.indexOf(`  async ${methodName}(`);
  assert.notEqual(methodStart, -1, `chat service ${methodName} command must stay discoverable`);
  const methodEnd = chatServiceSource.indexOf(`  async ${nextMethodName}(`, methodStart);
  assert.notEqual(methodEnd, -1, `chat service ${methodName} command must end before ${nextMethodName}`);
  return chatServiceSource.slice(methodStart, methodEnd);
}

const deleteMessageImplementation = extractChatServiceCommand('deleteMessage', 'recallMessage');
const addReactionImplementation = extractChatServiceCommand('addReaction', 'removeReaction');
const removeReactionImplementation = extractChatServiceCommand('removeReaction', 'updateChat');
for (const [methodName, implementation, orderedMutation] of [
  [
    'deleteMessage',
    deleteMessageImplementation,
    /await\s+client\.messages\.deleteForMe\s*\(\s*messageId\s*\)[\s\S]*?this\.assertAuthSessionGenerationCurrent\s*\(\s*generation\s*,\s*operation\s*\)[\s\S]*?this\.setLocalMessages\s*\(/u,
  ],
  [
    'addReaction',
    addReactionImplementation,
    /await\s+client\.addReaction\s*\(\s*messageId\s*,\s*emoji\s*\)[\s\S]*?this\.assertAuthSessionGenerationCurrent\s*\(\s*generation\s*,\s*operation\s*\)[\s\S]*?this\.updateLocalMessage\s*\(/u,
  ],
  [
    'removeReaction',
    removeReactionImplementation,
    /await\s+client\.removeReaction\s*\(\s*messageId\s*,\s*emoji\s*\)[\s\S]*?this\.assertAuthSessionGenerationCurrent\s*\(\s*generation\s*,\s*operation\s*\)[\s\S]*?this\.updateLocalMessage\s*\(/u,
  ],
]) {
  assert.match(
    implementation,
    /const\s+\{\s*client\s*,\s*generation\s*\}\s*=\s*await\s+this\.beginAuthSessionOperation\s*\(\s*operation\s*\)/u,
    `chat service ${methodName} must capture one authenticated IM SDK client and generation`,
  );
  assert.match(
    implementation,
    orderedMutation,
    `chat service ${methodName} must await the captured IM SDK mutation, fence its generation, then update local projection state`,
  );
  assert.doesNotMatch(
    implementation,
    /this\.client\s*\(\s*\)/u,
    `chat service ${methodName} must not reacquire a dynamic IM SDK client after capturing the operation client`,
  );
  assert.doesNotMatch(
    implementation,
    /\bfetch\s*\(|\baxios(?:\.|\s*\()|\/im\/v3\/|\b(?:Authorization|Access-Token|X-API-Key)\b/u,
    `chat service ${methodName} must not bypass the captured IM SDK client with raw HTTP or manual auth headers`,
  );
}
assert.doesNotMatch(
  deleteMessageImplementation,
  /recallMessage/u,
  'chat service deleteMessage must not recall messages when deleting only the current user copy',
);
assert.match(
  chatServiceSource,
  /MESSAGE_PAGE_LIMIT\s*=\s*SDKWORK_DEFAULT_PAGE_SIZE/u,
  'chat service message sync must use a bounded page size',
);
assert.match(
  chatServiceSource,
  /loadMoreMessages/u,
  'chat service must expose explicit loadMoreMessages for additional message history pages',
);
const getMessagesImplementation = chatServiceSource.match(
  /async\s+getMessages\([^{]*\{([\s\S]*?)\n\s+hasMoreMessages\(/u,
)?.[1] ?? '';
assert.match(
  getMessagesImplementation,
  /conversations\.listMessages\s*\(\s*chatId,\s*\{\s*pageSize,\s*\}/u,
  'chat service getMessages must request exactly one bounded latest message page without constructing a cursor',
);
assert.doesNotMatch(
  getMessagesImplementation,
  /while\s*\(|loadMoreMessages\s*\(/u,
  'chat service getMessages must not auto-load additional message history pages; loadMoreMessages is the explicit next-page path',
);
assert.match(
  chatServiceSource,
  /nextCursor\?:\s*string/u,
  'chat service message history sync must retain the server-issued opaque cursor',
);
assert.doesNotMatch(
  chatServiceSource,
  /afterSeq|nextAfterSeq|resolveInitialMessageHistoryAfterSeq/u,
  'chat service must not parse or construct numeric message-history cursors',
);
assert.match(
  chatServiceSource,
  /resolveMessageEntryType/u,
  'chat service must derive PC message type from the complete message history body/render hints',
);
assert.match(
  chatServiceSource,
  /resolveMessageEntryContent/u,
  'chat service must derive PC message content from the complete message history body/resource projection',
);
assert.match(
  chatServiceSource,
  /entry\.sender\?\.id/u,
  'chat service historical messages must use ConversationMessageEntry.sender instead of system fallback',
);
assert.match(
  chatServiceSource,
  /entry\.body/u,
  'chat service historical messages must use ConversationMessageEntry.body instead of summary-only fallback',
);
assert.match(
  chatServiceSource,
  /entry\.committedAt\s*\?\?\s*entry\.occurredAt/u,
  'chat service historical messages must use backend message history timestamps',
);
assert.doesNotMatch(
  getMessagesImplementation,
  /getMessageInteractionSummary\s*\(/u,
  'chat service getMessages must not issue per-message interaction_summary requests while loading message history',
);
assert.doesNotMatch(
  chatServiceSource,
  /MessageInteractionSummaryView|interactionSummaryCache|entry\.reactionCounts|entry\.pin/u,
  'chat service message history must not depend on inline interaction fields that ConversationMessageEntry does not define',
);
const resolveReadSeqForMarkAsReadImplementation = chatServiceSource.match(
  /private\s+resolveReadSeqForMarkAsRead\(chatId:\s*string\):\s*number\s*\{([\s\S]*?)\n\s+\}/u,
)?.[1] ?? '';
assert.match(
  chatServiceSource,
  /private\s+resolveReadSeqForMarkAsRead\(chatId:\s*string\):\s*number/u,
  'chat service markAsRead must resolve read cursor sequence from local inbox/message projection state synchronously',
);
assert.match(
  resolveReadSeqForMarkAsReadImplementation,
  /return\s+this\.latestReadSeq\.get\(chatId\)\s*\?\?\s*0/u,
  'chat service markAsRead must use the cached latest sequence from inbox, websocket, or message history synchronization',
);
assert.doesNotMatch(
  resolveReadSeqForMarkAsReadImplementation,
  /listMessages|pageSize:\s*1|await|response\.highWatermark|response\.items\?\.at\(-1\)|latestEntry\?\.messageSeq/u,
  'chat service markAsRead must not issue an extra message-history/highWatermark request or infer state from a pageSize=1 peek',
);
const hydrateInboxEntriesToChatsImplementation = chatServiceSource.match(
  /private\s+async\s+hydrateInboxEntriesToChats\([^{]*\{([\s\S]*?)\n\s+async\s+listChatsPage/u,
)?.[1] ?? '';
assert.match(
  hydrateInboxEntriesToChatsImplementation,
  /this\.writeLatestReadSeq\(entry\.conversationId,\s*Math\.max\([\s\S]*?entry\.lastMessageSeq/u,
  'chat service inbox hydration must seed the bounded latestReadSeq cache from ConversationInboxEntry.lastMessageSeq for zero-extra-request markAsRead',
);
assert.doesNotMatch(
  hydrateInboxEntriesToChatsImplementation,
  /conversations\.(?:getPreferences|getProfile)\s*\(|hydrateDirectConversationViewState\s*\(|resolveDirectChatPeerProfile\s*\(|syncConversationMembers\s*\(/u,
  'chat service chat-list hydration must not issue per-conversation preference, profile, member, or social fallback reads',
);
assert.match(
  hydrateInboxEntriesToChatsImplementation,
  /applyInboxProjectionToViewState[\s\S]*?mapInboxEntryToChat/u,
  'chat service chat-list hydration must use ConversationInboxEntry projections and safe local fallbacks only',
);
assert.match(
  chatServiceSource,
  /conversations\.updateProfile\s*\(/u,
  'chat service updateChat/createChat must persist shared group profile fields through the IM SDK',
);
const emitChatListImplementation = chatServiceSource.match(
  /private\s+emitChatList\([^{]*\{([\s\S]*?)\n\s+private\s+async\s+doEmitChatList/u,
)?.[1] ?? '';
assert.match(
  emitChatListImplementation,
  /chatListRefreshPromise/u,
  'chat service realtime conversation-list refresh must coalesce burst events through a single in-flight inbox request',
);
const realtimeReadCursorSyncImplementation = chatServiceSource.match(
  /private\s+queueRealtimeReadCursorSync\([^{]*\{([\s\S]*?)\n\s+async\s+markAsRead/u,
)?.[1] ?? '';
assert.match(
  realtimeReadCursorSyncImplementation,
  /pendingRealtimeReadCursorSeqs/u,
  'chat service active realtime read sync must coalesce read cursor updates by conversation',
);
assert.match(
  realtimeReadCursorSyncImplementation,
  /conversations\.updateReadCursor/u,
  'chat service active realtime read sync must advance the read cursor through the IM SDK read protocol',
);
assert.doesNotMatch(
  realtimeReadCursorSyncImplementation,
  /listMessages\s*\(|updatePreferences\s*\(/u,
  'chat service active realtime read sync must not issue per-message history or preference requests',
);
const handleLiveScopeEventImplementation = chatServiceSource.match(
  /private\s+handleLiveScopeEvent\([^{]*\{([\s\S]*?)\n\s+private\s+handleLiveMessage/u,
)?.[1] ?? '';
assert.doesNotMatch(
  handleLiveScopeEventImplementation,
  /markAsRead\s*\(/u,
  'chat service user-scope realtime messages must not call manual markAsRead because that path performs durable history lookup',
);
assert.match(
  handleLiveScopeEventImplementation,
  /queueRealtimeReadCursorSync\s*\(\s*message\.chatId\s*,\s*messageSeq\s*\)/u,
  'chat service user-scope realtime messages must sync active reads from the event sequence',
);
const handleLiveMessageImplementation = chatServiceSource.match(
  /private\s+handleLiveMessage\([^{]*\{([\s\S]*?)\n\s+private\s+emitChatList/u,
)?.[1] ?? '';
assert.doesNotMatch(
  handleLiveMessageImplementation,
  /markAsRead\s*\(/u,
  'chat service conversation-scope realtime messages must not call manual markAsRead because that path performs durable history lookup',
);
assert.match(
  handleLiveMessageImplementation,
  /queueRealtimeReadCursorSync\s*\(\s*message\.chatId\s*,\s*messageSeq\s*\)/u,
  'chat service conversation-scope realtime messages must sync active reads from the event sequence',
);
for (const profileField of ['displayName', 'avatarUrl', 'notice']) {
  assert.match(
    chatServiceSource,
    new RegExp(`\\b${profileField}\\b`, 'u'),
    `chat service must map PC chat profile state through ${profileField}`,
  );
}
assert.match(
  chatServiceSource,
  /conversations\.updatePreferences\s*\(\s*chatId,\s*\{\s*isPinned/u,
  'chat service pinChat must persist pin state through the IM SDK conversation preferences API',
);
assert.match(
  chatServiceSource,
  /conversations\.updatePreferences\s*\(\s*chatId,\s*\{\s*isMuted/u,
  'chat service muteChat must persist mute state through the IM SDK conversation preferences API',
);
assert.match(
  chatServiceSource,
  /markAsUnread\s*\(/u,
  'chat service must expose a semantic markAsUnread method for PC conversation unread preference sync',
);
assert.match(
  chatServiceSource,
  /conversations\.updatePreferences\s*\(\s*chatId,\s*\{\s*isMarkedUnread/u,
  'chat service markAsUnread/markAsRead must persist manual unread state through the IM SDK conversation preferences API',
);
assert.match(
  chatServiceSource,
  /isMarkedUnread/u,
  'chat service must map PC manual unread state through ConversationPreferencesView.isMarkedUnread',
);
assert.match(
  chatServiceSource,
  /projectedPreferences\.isHidden/u,
  'chat service must map hidden chat-list state from ConversationInboxEntry.preferences.isHidden',
);
assert.match(
  chatServiceSource,
  /if\s*\(\s*viewState\?\.isHidden\s*\)\s*\{\s*return undefined;\s*\}/u,
  'chat service getChats must filter hidden conversations using SDK-backed preferences',
);
const chatGetChatsSource = chatServiceSource.slice(
  chatServiceSource.indexOf('  async getChats('),
  chatServiceSource.indexOf('  subscribeChats(', chatServiceSource.indexOf('  async getChats(')),
);
assert.match(
  chatGetChatsSource,
  /const\s+page\s*=\s*await\s+this\.listChatsPage\(\s*\)/u,
  'chat service getChats compatibility entry must read only the first inbox page',
);
assert.doesNotMatch(
  chatGetChatsSource,
  /forEachCursorPage|MAX_INBOX_CONVERSATIONS|cursor/u,
  'chat service getChats compatibility entry must not aggregate every inbox page',
);
assert.match(
  chatServiceSource,
  /conversations\.updatePreferences\s*\(\s*chatId,\s*\{\s*isHidden:\s*true\s*\}\s*\)/u,
  'chat service deleteChat must hide the chat list item through the IM SDK conversation preferences API',
);
assert.match(
  chatServiceSource,
  /conversations\.updatePreferences\s*\(\s*conversationId,\s*\{\s*isHidden:\s*false\s*\}\s*\)/u,
  'chat service createChat must reopen the server-created canonical conversation through the IM SDK conversation preferences API',
);
assert.doesNotMatch(
  chatServiceSource,
  /hiddenConversationIds/u,
  'chat service must not keep hidden conversation state in local memory',
);
assert.doesNotMatch(
  chatServiceSource,
  /conversations\.leave\s*\(/u,
  'chat service deleteChat must not leave a conversation when deleting the current user chat-list item',
);

const favoriteServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/FavoriteService.ts');
assert.match(favoriteServiceSource, /@sdkwork\/im-sdk/u, 'favorite service must route message favorites through @sdkwork/im-sdk types');
assert.match(favoriteServiceSource, /getImSdkClientWithSession/u, 'favorite service must use the shared IM SDK client wrapper');
assert.match(
  favoriteServiceSource,
  /class\s+SdkworkFavoriteService/u,
  'favorite service must expose an SDK-backed service implementation',
);
assert.match(
  favoriteServiceSource,
  /FAVORITES_PAGE_LIMIT\s*=\s*SDKWORK_DEFAULT_PAGE_SIZE/u,
  'favorite service message favorite sync must use a bounded SDK page size',
);
assert.match(
  favoriteServiceSource,
  /this\.client\(\)\.messages\.favorites\.list\s*\(/u,
  'favorite service getFavorites must list message favorites through the composed IM SDK',
);
assert.match(
  favoriteServiceSource,
  /this\.client\(\)\.messages\.favorites\.create\s*\(/u,
  'favorite service addFavorite must create message favorites through the composed IM SDK',
);
assert.match(
  favoriteServiceSource,
  /this\.client\(\)\.messages\.favorites\.delete\s*\(/u,
  'favorite service removeFavorite must delete message favorites through the composed IM SDK',
);
assert.match(
  favoriteServiceSource,
  /mapFilterToFavoriteType/u,
  'favorite service must map PC favorite filters into the standard MessageFavoriteType taxonomy',
);
assert.match(
  favoriteServiceSource,
  /messageId.*is required|Favorite messageId is required/u,
  'favorite service must reject favorite creation without a backend message id',
);
assert.match(
  favoriteServiceSource,
  /conversationId.*is required|Favorite conversationId is required/u,
  'favorite service must reject favorite creation without a backend conversation id',
);
assert.doesNotMatch(favoriteServiceSource, /class\s+MockFavoriteService/u, 'favorite service must not be mock-backed');
assert.doesNotMatch(favoriteServiceSource, /mockFavorites|setTimeout|console\.log/u, 'favorite service must not keep mock favorite branches');
assert.doesNotMatch(favoriteServiceSource, /\bfetch\s*\(/u, 'favorite service must not use raw fetch');
assert.doesNotMatch(favoriteServiceSource, /\/im\/v3/u, 'favorite service must not hand-code IM HTTP paths');
assert.doesNotMatch(favoriteServiceSource, /\b(Authorization|Access-Token|X-API-Key)\b/u, 'favorite service must not assemble auth headers manually');

const contactServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ContactService.ts');
const organizationDirectoryServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/OrganizationDirectoryService.ts');
assert.match(contactServiceSource, /@sdkwork\/im-sdk/u, 'contact service must route contacts and friendship behavior through @sdkwork/im-sdk types');
assert.match(contactServiceSource, /getImSdkClientWithSession/u, 'contact service must use the shared IM SDK client wrapper');
assert.match(
  organizationDirectoryServiceSource,
  /getAppbaseAppSdkClientWithSession/u,
  'organization directory service must use the generated appbase app SDK wrapper for IAM organization directory resources',
);
assert.match(
  contactServiceSource,
  /class\s+SdkworkContactService/u,
  'contact service must expose an SDK-backed service implementation',
);
const listContactsPageSource = contactServiceSource.match(/async\s+listContactsPage\s*\([\s\S]*?\n\s+async\s+listContactConversationIds/u)?.[0] ?? '';
assert.ok(listContactsPageSource, 'contact service listContactsPage block must stay discoverable');
assert.match(
  contactServiceSource,
  /getContacts\s*\([\s\S]*?this\.listContactsPage\s*\(/u,
  'contact service getContacts must list contacts through the generated IM chat contacts SDK',
);
assert.match(
  contactServiceSource,
  /syncContactsFromServer\s*\([\s\S]*?this\.listContactsPage\s*\(/u,
  'contact service syncContacts must refresh a single bounded contact page through the generated IM chat contacts SDK',
);
assert.match(
  contactServiceSource,
  /\.chat\.contacts\.list\s*\(/u,
  'contact service must page contacts through chat.contacts.list',
);
assert.doesNotMatch(
  listContactsPageSource,
  /\.social\.contacts\.preferences\.retrieve\s*\(|\.social\.users\.list\s*\(/u,
  'contact list pages must not fan out to per-contact preferences or social user profile requests',
);
assert.match(
  contactServiceSource,
  /\.social\.friendRequests\.create\s*\(/u,
  'contact service addFriend must submit friend requests through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /addFriendBySearchQuery\s*\([\s\S]*?this\.findAddFriendTarget\s*\(\s*normalizedQuery\s*\)[\s\S]*?this\.assertRelationshipAllowsFriendRequest\s*\([\s\S]*?this\.client\(\)\.social\.friendRequests\.create/u,
  'contact service direct add-by-input must resolve a real searched user before submitting a friend request',
);
assert.match(
  contactServiceSource,
  /findAddFriendTarget\s*\([\s\S]*?this\.client\(\)\.social\.users\.list/u,
  'contact service add-by-input search must query users through the generated IM SDK social user search endpoint',
);
assert.match(
  contactServiceSource,
  /\.social\.users\.list\s*\(/u,
  'contact service searchContacts must query users through the generated IM SDK social user search endpoint',
);
assert.doesNotMatch(
  contactServiceSource,
  /loadContactPeerProfiles/u,
  'contact service contact list hydration must not resolve every contact through social user search',
);
assert.doesNotMatch(
  contactServiceSource,
  /createUserFromId\(normalizedQuery\)/u,
  'contact service searchContacts must not synthesize a user from the search input when backend search has no match',
);
assert.doesNotMatch(
  contactServiceSource,
  /getUserById[\s\S]*createUserFromId\(normalizedId\)/u,
  'contact service getUserById must not synthesize a user from arbitrary input when backend search has no match',
);
assert.match(
  contactServiceSource,
  /\.social\.friendRequests\.list\s*\(/u,
  'contact service getFriendRequests must list friend requests through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /getPendingFriendRequestCount\s*\(/u,
  'contact service must expose a pending friend request count for contacts red dot badges',
);
assert.match(
  contactServiceSource,
  /subscribePendingFriendRequestCount\s*\(/u,
  'contact service must expose a pending friend request count subscription for realtime-friendly red dot updates',
);
assert.match(
  contactServiceSource,
  /FRIEND_REQUESTS_PAGE_LIMIT/u,
  'contact service friend request sync must use a bounded SDK page size',
);
assert.match(
  contactServiceSource,
  /syncFriendRequestsFromServer/u,
  'contact service getFriendRequests must keep the legacy compatibility wrapper bounded to one request page',
);
assert.match(
  contactServiceSource,
  /syncFriendRequestsFromServer\(\s*['"]all['"],\s*['"]pending['"]\s*\)/u,
  'contact service getFriendRequests must request one pending friend-request page without incoming/outgoing full aggregation',
);
assert.match(
  contactServiceSource,
  /nextCursor/u,
  'contact service paginated friend request lists must expose SDK cursor windows',
);
assert.match(
  contactServiceSource,
  /\.social\.friendRequests\.accept\s*\(/u,
  'contact service handleFriendRequest must accept friend requests through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /\.social\.friendRequests\.decline\s*\(/u,
  'contact service handleFriendRequest must decline friend requests through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /cancelFriendRequest\s*\(/u,
  'contact service must expose cancelFriendRequest for outgoing pending requests',
);
assert.match(
  contactServiceSource,
  /\.social\.friendRequests\.cancel\s*\(/u,
  'contact service cancelFriendRequest must cancel friend requests through the generated IM SDK',
);
assert.match(
  imTransportClientLikeSource,
  /userBlocks:\s*\{/u,
  'IM transport client type must expose generated social.userBlocks resource',
);
assert.match(
  contactServiceSource,
  /\.social\.userBlocks\.create\s*\(/u,
  'contact service addToBlacklist must route through social.userBlocks.create',
);
assert.match(
  contactServiceSource,
  /direction:\s*isOutgoing\s*\?\s*['"]outgoing['"]\s*:\s*['"]incoming['"]/u,
  'contact service must expose friend request direction for incoming versus outgoing UI actions',
);
assert.match(
  contactServiceSource,
  /\.social\.friendships\.remove\s*\(/u,
  'contact service deleteContact must remove friendships through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /getContactPreferences\s*\([\s\S]*?\.social\.contacts\.preferences\.retrieve\s*\(/u,
  'contact service must load contact preferences only for explicit contact detail requests',
);
assert.match(
  contactServiceSource,
  /\.social\.contacts\.preferences\.update\s*\(/u,
  'contact service contact preference mutations must persist through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /CONTACT_TAGS_PAGE_LIMIT/u,
  'contact service contact tag sync must use a bounded SDK page size',
);
assert.match(
  contactServiceSource,
  /CONTACT_TAGS_PAGE_LIMIT/u,
  'contact service getTags must request contact tags with a bounded SDK page size',
);
assert.match(
  contactServiceSource,
  /\.social\.contacts\.tags\.list\s*\(/u,
  'contact service getTags must list contact tags through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /\.social\.contacts\.tags\.create\s*\(/u,
  'contact service addTag must create contact tags through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /\.social\.contacts\.tags\.update\s*\(/u,
  'contact service updateTag must update contact tags through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /\.social\.contacts\.tags\.delete\s*\(/u,
  'contact service removeTag must delete contact tags through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /\.social\.contacts\.recommendations\.create\s*\(/u,
  'contact service recommendToFriend must create contact recommendations through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /organizationDirectoryService/u,
  'contact service organization directory compatibility methods must delegate to the independent OrganizationDirectoryService',
);
assert.match(
  organizationDirectoryServiceSource,
  /\.iam\.departments\.list\s*\(/u,
  'organization directory service must list departments through the generated app SDK iam.departments resource',
);
assert.match(
  organizationDirectoryServiceSource,
  /\.iam\.departmentAssignments\.list\s*\(/u,
  'organization directory service must list department assignments through the generated app SDK iam.departmentAssignments resource',
);
assert.match(
  organizationDirectoryServiceSource,
  /\.iam\.organizations\.list\s*\(/u,
  'organization directory service must expose organization subjects through the generated app SDK iam.organizations resource',
);
assert.doesNotMatch(
  contactServiceSource,
  /\.portal\.home\.retrieve\s*\(\s*\)|retrievePortalHome|mapPortalSnapshotToDepartments/u,
  'contact service must not back the organization directory with portal home snapshots',
);
assert.doesNotMatch(
  organizationDirectoryServiceSource,
  /\.portal\.home\.retrieve\s*\(\s*\)|retrievePortalHome|mapPortalSnapshotToDepartments|iam_accounts|iamAccounts|department_members|departmentMembers/u,
  'organization directory service must not use portal home, iam_accounts, or direct department member modeling',
);
assert.match(
  organizationDirectoryServiceSource,
  /\.iam\.users\.current\.retrieve\s*\(\s*\)/u,
  'organization directory service must resolve the logged-in user through iam.users.current.retrieve',
);
assert.match(
  organizationDirectoryServiceSource,
  /\.iam\.roleBindings\.list\s*\(\s*\{[\s\S]*?principalId:[\s\S]*?scopeKind:\s*['"]organization['"][\s\S]*?scopeId:\s*resolvedOrganizationId/u,
  'organization directory service must derive organization permissions from scoped role bindings rather than direct user roles',
);
assert.match(
  organizationDirectoryServiceSource,
  /OrganizationDirectoryAdminCapability/u,
  'organization directory service must model member writes as an injected IAM admin capability boundary',
);
assert.match(
  organizationDirectoryServiceSource,
  /organizationMemberships\.create[\s\S]*departmentAssignments\.create/u,
  'organization directory service member management must create organization membership before department assignment',
);
assert.match(
  organizationDirectoryServiceSource,
  /roleBindings\.create/u,
  'organization directory service member management must use scoped role binding creation for admin-assigned roles',
);
assert.doesNotMatch(
  organizationDirectoryServiceSource,
  /\bfetch\s*\(|\/backend\/v3\/api|\/app\/v3\/api|\/iam\/(?:organizations|departments|organization_memberships|department_assignments)|\b(Authorization|Access-Token|X-API-Key)\b/u,
  'organization directory service must not hand-code IAM HTTP paths or manual auth headers',
);
assert.match(
  contactServiceSource,
  /isStarred|isBlocked|remark/u,
  'contact service must map PC star, blacklist, and remark state through ContactPreferencesView',
);
assertNoImDeviceApiUsage(contactServiceSource, 'contact service');
assert.match(
  contactServiceSource,
  /syncContactsFromServer\s*\([\s\S]*?this\.listContactsPage\s*\(/u,
  'contact service sync must refresh only one bounded contact page through the generated IM chat contacts SDK',
);
assert.match(
  contactServiceSource,
  /deleteContact\s*\([\s\S]*?this\.dispatchFriendRequestChange\s*\(/u,
  'contact service deleteContact must notify contact consumers after removing a friendship',
);
assert.match(
  contactServiceSource,
  /addToBlacklist\s*\([\s\S]*?this\.dispatchFriendRequestChange\s*\(/u,
  'contact service addToBlacklist must notify contact consumers after blocking a contact',
);
assert.doesNotMatch(contactServiceSource, /class\s+MockContactService/u, 'contact service must not be mock-backed');
assert.doesNotMatch(contactServiceSource, /mockUsers|mockFriendRequests/u, 'contact service must not keep mock contacts or friend requests');
assert.doesNotMatch(
  contactServiceSource,
  /starredContactIds|contactRemarks|blacklistedContactIds|recommendedContactIds|INITIAL_TAGS|\bprivate\s+tags\b|\bthis\.tags\b/u,
  'contact service must not persist star, remark, blacklist, tag, or recommendation contact state in local memory',
);
assert.doesNotMatch(
  contactServiceSource,
  /LOCAL_DEPARTMENTS|id:\s*['"]org-root['"]|name:\s*['"]SDKWork['"][\s\S]*name:\s*['"]Contacts['"]/u,
  'contact service must not expose a fixed mock organization tree when backend directory data is unavailable',
);
assert.doesNotMatch(contactServiceSource, /\bfetch\s*\(/u, 'contact service must not use raw fetch');
assert.doesNotMatch(contactServiceSource, /\/im\/v3/u, 'contact service must not hand-code IM HTTP paths');
assert.doesNotMatch(contactServiceSource, /\b(Authorization|Access-Token|X-API-Key)\b/u, 'contact service must not assemble auth headers manually');

const addFriendModalSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/AddFriendModal.tsx');
assert.match(
  addFriendModalSource,
  /const\s+normalizedQuery\s*=\s*searchQuery\.trim\s*\(\s*\)[\s\S]*contactService\.searchContacts\s*\(\s*normalizedQuery\s*\)/u,
  'add friend modal must search contacts through the SDK-backed contact service with a normalized target id',
);
assert.match(
  addFriendModalSource,
  /contactService\.addFriend\s*\(\s*result\.id\s*\)/u,
  'add friend modal must create friend requests through ContactService',
);
assert.match(
  addFriendModalSource,
  /buildSearchResultDescription\s*\(\s*results\[0\]\s*\)/u,
  'add friend modal must render backend profile fields for search results instead of fixed mock descriptions',
);
assert.doesNotMatch(
  addFriendModalSource,
  /id:\s*normalizedQuery/u,
  'add friend modal must not synthesize a user id from the search input when backend search has no match',
);
assert.doesNotMatch(
  addFriendModalSource,
  /api\.dice(?:bear)\.com\/7\.x\/avataaars\/svg\?seed=\$\{normalizedQuery\}/u,
  'add friend modal must not synthesize a remote generated avatar from the search input when backend search has no match',
);
assert.doesNotMatch(
  addFriendModalSource,
  /['"]new-friend['"]/u,
  'add friend modal must not synthesize a fake friend request target id',
);
assert.doesNotMatch(
  addFriendModalSource,
  /desc:\s*['"]Sdkwork IM\s+用户['"]/u,
  'add friend modal must not hard-code a mock-looking user description for SDK search results',
);
assert.doesNotMatch(
  addFriendModalSource,
  /Sdkwork IM\s+用户/u,
  'add friend modal search result fallback text must be derived from backend user data instead of a fixed mock-looking label',
);

const allContactsContainerSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/contacts/AllContactsContainer.tsx');
const contactsPageSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/pages/ContactsView.tsx');
assert.match(
  allContactsContainerSource,
  /onAddFriend\?:\s*\(\)\s*=>\s*void/u,
  'all contacts add-friend entry must delegate to the shared AddFriendModal opener',
);
assert.doesNotMatch(
  allContactsContainerSource,
  /customPrompt\s*\(|PromptModal|addFriendBySearchQuery|contactService\.addFriend\s*\(\s*qs\s*\)/u,
  'all contacts add-friend entry must not keep a second prompt-based add-friend flow',
);
assert.match(
  contactsPageSource,
  /<AllContactsContainer[\s\S]*onAddFriend=\{onAddFriend\}/u,
  'contacts page must route the all-contacts add-friend button to the shared AddFriendModal',
);
const orgContainerSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/contacts/OrgContainer.tsx');
assert.match(
  orgContainerSource,
  /organizationDirectoryService\.getOrganizationDirectoryTree\s*\(\s*\)/u,
  'organization contacts view must load one unified organization/department tree through the SDK-backed organization directory service',
);
assert.match(
  orgContainerSource,
  /renderDirectoryTreeNode/u,
  'organization contacts view must render one unified organization-directory tree',
);
assert.doesNotMatch(
  orgContainerSource,
  /organizationDirectoryService\.getOrganizations\s*\(\s*\)|organizationDirectoryService\.getOrganizationTree\s*\(\s*\)|organizationDirectoryService\.getDepartments\s*\(|organizationDirectoryService\.getDepartmentTree\s*\(/u,
  'organization contacts view must not keep separate organization and department loading paths',
);
assert.match(
  orgContainerSource,
  /flattenDirectoryOrganizations[\s\S]*flattenDirectoryDepartments/u,
  'organization contacts view must derive organizations and departments from the unified directory tree',
);
assert.match(
  orgContainerSource,
  /organizationDirectoryService\.getUsersByDepartment\s*\(/u,
  'organization contacts view must read users from department assignments through the organization directory service',
);
assert.match(
  orgContainerSource,
  /organizationDirectoryService\.getCurrentUser\s*\(\s*\)/u,
  'organization contacts view must load the logged-in IAM user before evaluating organization admin actions',
);
assert.match(
  orgContainerSource,
  /organizationDirectoryService\.getOrganizationPermissions\s*\(\s*organization\.organizationId\s*\)/u,
  'organization contacts view must evaluate scoped organization permissions for the selected organization',
);
assert.match(
  orgContainerSource,
  /currentOrganizationDepartments\s*=\s*useMemo[\s\S]*department\.organizationId\s*===\s*currentOrganization\.organizationId/u,
  'organization contacts member management must scope selectable departments to the selected organization',
);
assert.match(
  orgContainerSource,
  /currentOrganizationDepartments\.map\s*\(\s*\(department\)/u,
  'organization contacts member management must render department options from the current organization only',
);
assert.doesNotMatch(
  orgContainerSource,
  /allDepartments\.map\s*\(\s*\(department\)/u,
  'organization contacts member management must not render every department across organizations',
);
assert.doesNotMatch(
  orgContainerSource,
  /currentOrganization\?\.verificationStatus|grid\s+shrink-0\s+grid-cols-3|currentOrganization\.tenantBoundaryKind|currentOrganization\.dataBoundaryKind/u,
  'organization contacts view must not render a complex right-content top header; keep the right content focused on departments and members',
);
assert.match(
  orgContainerSource,
  /canManageMembers[\s\S]*UserPlus/u,
  'organization contacts view must expose member management controls only from the permission-aware admin state',
);
assert.match(
  orgContainerSource,
  /organizationDirectoryService\.(?:addOrganizationMember|inviteOrganizationMember)\s*\(/u,
  'organization contacts view admin flow must submit member add or invite through OrganizationDirectoryService',
);
assert.doesNotMatch(
  orgContainerSource,
  /toast\s*\(\s*['"][^'"]*(?:邀请|invite|添加|add)[^'"]*['"]\s*,\s*['"]success['"]\s*\)[\s\S]*?setUsers\s*\(/u,
  'organization contacts view must not fake member add or invite success by locally mutating the member list',
);
assert.match(
  orgContainerSource,
  /positionAssignments|roleBindings|roleCodes|departmentAssignmentId|organizationMembershipId/u,
  'organization contacts view must surface member assignment, position, and scoped role binding context',
);
assert.match(
  orgContainerSource,
  /searchQuery/u,
  'organization contacts view must support global contact search across organizations, departments, and assignment-backed members',
);
assert.doesNotMatch(
  orgContainerSource,
  /contactService\.getDepartments|contactService\.getUsersByDepartment/u,
  'organization contacts view must not route Organization/Department directory reads through ContactService',
);
assert.doesNotMatch(
  orgContainerSource,
  /toast\s*\(\s*`[^`]*\$\{selectedUser\.name\}[^`]*会话[^`]*`\s*,\s*['"]success['"]\s*\)/u,
  'organization contacts view must not show a fake successful chat navigation when no real send-message handler is connected',
);
assert.match(
  contactsPageSource,
  /<OrgContainer[\s\S]*searchQuery=\{searchQuery\}/u,
  'contacts page must pass the global search query into the organization directory view',
);
const contactDetailPaneSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/contacts/ContactDetailPane.tsx');
assert.doesNotMatch(
  contactDetailPaneSource,
  /else\s+toast\s*\(\s*`[^`]*\$\{user\.name\}[^`]*会话[^`]*`\s*,\s*['"]success['"]\s*\)/u,
  'contact detail send-message button must not fake successful chat navigation without a real SDK-backed handler',
);
assert.doesNotMatch(
  contactDetailPaneSource,
  /else\s+toast\s*\(\s*['"][^'"]*发起(?:语音|视频)?通话[^'"]*['"]\s*,\s*['"]success['"]\s*\)/u,
  'contact detail call buttons must not fake successful RTC start without a real SDK-backed handler',
);

const createGroupModalSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/CreateGroupModal.tsx');
assertFile(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ContactMemberPickerPanel.tsx',
  'group contact selection must live in a shared WeChat-style contact member picker panel',
);
const contactMemberPickerPanelSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ContactMemberPickerPanel.tsx',
);
assert.match(
  createGroupModalSource,
  /onCreated\?:\s*\(group:\s*Chat\)\s*=>\s*void/u,
  'create group modal must expose the backend-created group chat to the parent flow',
);
assert.match(
  createGroupModalSource,
  /const\s+group\s*=\s*await\s+groupService\.createGroup\s*\(\s*['"][^'"]*['"]\s*,\s*Array\.from\s*\(\s*selected\s*\)\s*\)/u,
  'create group modal must create group conversations through the SDK-backed group service',
);
assert.match(
  createGroupModalSource,
  /contactService\.listContactsPage\s*\(\s*\)[\s\S]*?\.catch\s*\(/u,
  'create group modal must load selectable members from the address book and fail closed',
);
assert.match(
  contactMemberPickerPanelSource,
  /function\s+createContactSearchText[\s\S]*contact\.chatId[\s\S]*contact\.email[\s\S]*contact\.phone[\s\S]*contact\.company[\s\S]*contact\.position[\s\S]*contact\.py/u,
  'shared contact member picker must search across address-book contact identity fields without calling user search',
);
assert.match(
  contactMemberPickerPanelSource,
  /function\s+createContactIndexKey[\s\S]*contact\.py[\s\S]*contact\.name[\s\S]*return\s+\/\[A-Z\]\//u,
  'shared contact member picker must derive stable A-Z index keys from pinyin or contact names',
);
assert.match(
  contactMemberPickerPanelSource,
  /function\s+groupContactsByIndex[\s\S]*createContactIndexKey[\s\S]*groups\.sort/u,
  'shared contact member picker must group address-book contacts for an indexed list',
);
assert.match(
  contactMemberPickerPanelSource,
  /const\s+selectedContacts\s*=\s*useMemo[\s\S]*contacts\.filter[\s\S]*selectedIds\.has\(contact\.id\)/u,
  'shared contact member picker must project selected contacts for the right-side selected member column',
);
assert.match(
  contactMemberPickerPanelSource,
  /grid\s+h-full\s+min-h-0\s+grid-cols-2\s+gap-4/u,
  'shared contact member picker must use equal-width columns with address-book list on the left and selected members on the right',
);
assert.match(
  contactMemberPickerPanelSource,
  /renderedIndexKeys\.map[\s\S]*scrollToIndexGroup/u,
  'shared contact member picker must render a real-group index list that jumps to contact groups',
);
assert.match(
  contactMemberPickerPanelSource,
  /const\s+renderedIndexKeys\s*=\s*useMemo[\s\S]*groupedContacts\.map\(\(group\)\s*=>\s*group\.key\)/u,
  'shared contact member picker must render only real contact group index keys instead of a full A-Z strip',
);
assert.match(
  contactMemberPickerPanelSource,
  /grid\s+h-full\s+min-h-0\s+grid-cols-\[minmax\(0,1fr\)_24px\]/u,
  'shared contact member picker must reserve a dedicated narrow index column inside the list area so letters cannot overlap search input',
);
assert.doesNotMatch(
  contactMemberPickerPanelSource,
  /absolute\s+right-1\s+top-1\/2/u,
  'shared contact member picker index strip must not be absolutely overlaid on top of the contact list',
);
assert.doesNotMatch(
  contactMemberPickerPanelSource,
  /CONTACT_INDEX_KEYS\.map/u,
  'shared contact member picker must not render the full alphabet when only a few real contact groups exist',
);
assert.match(
  contactMemberPickerPanelSource,
  /selectedContacts\.map[\s\S]*onToggleContact\(contact\.id\)/u,
  'shared contact member picker must render checked contacts on the right and allow removing them from that selected list',
);
assert.match(
  contactMemberPickerPanelSource,
  /t\(['"]chat\.modal\.selection\.selectedTitle['"]\)[\s\S]*t\(['"]chat\.modal\.selection\.emptySelected['"]\)/u,
  'shared contact member picker selected column copy must be localized',
);
assert.match(
  createGroupModalSource,
  /<ContactMemberPickerPanel[\s\S]*contacts=\{contacts\}[\s\S]*selectedIds=\{selected\}[\s\S]*onToggleContact=\{toggleSelect\}/u,
  'create group modal must reuse the shared indexed contact picker',
);
assert.match(
  createGroupModalSource,
  /width=["']w-\[760px\]["'][\s\S]*height=["']h-\[700px\]["']/u,
  'create group modal must use a taller fixed height for equal-width two-column contact selection',
);
assert.match(
  createGroupModalSource,
  /onCreated\?\.\(\s*group\s*\)/u,
  'create group modal must return the real backend-created group chat after creation',
);
assert.doesNotMatch(
  createGroupModalSource,
  /chatService\.createChat|id:\s*['"]group-/u,
  'create group modal must not locally synthesize group chat records',
);
assert.doesNotMatch(
  createGroupModalSource,
  /addMembersBySearchQuery|social\.users\.list/u,
  'create group modal must not resolve arbitrary text through user search',
);

const groupsContainerSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/contacts/GroupsContainer.tsx');
assert.match(
  groupsContainerSource,
  /groupService\.listGroupsPage\s*\(/u,
  'groups contacts container must load groups through paginated SDK inbox reads',
);
assert.match(
  groupsContainerSource,
  /onOpenGroup\?:\s*\(group:\s*Chat\)\s*=>\s*void/u,
  'groups contacts container must expose selected backend group chats to the parent flow',
);
assert.match(
  groupsContainerSource,
  /onOpenGroup\?\.\(\s*group\s*\)/u,
  'groups contacts container must open the real group chat selected from the SDK-backed group list',
);
assert.match(
  groupsContainerSource,
  /<CreateGroupModal[\s\S]*?onCreated=\{async\s*\(group\)\s*=>\s*\{[\s\S]*?setGroups\(\s*\(\s*previousGroups\s*\)\s*=>\s*\[group,\s*\.\.\.previousGroups(?:\.filter\([\s\S]*?\))?\]\s*\)[\s\S]*?onOpenGroup\?\.\(\s*group\s*\)/u,
  'groups contacts container must reuse the address-book create-group modal and open the real backend-created group chat without stale group-list closures',
);
assert.doesNotMatch(
  groupsContainerSource,
  /customPrompt\s*\(|groupService\.createGroup\s*\([^)]*\[\s*\]\s*\)/u,
  'groups contacts container must not bypass the address-book create-group flow with prompt-based empty group creation',
);
assert.doesNotMatch(
  groupsContainerSource,
  /chatService\.createChat|id:\s*['"]group-/u,
  'groups contacts container must not locally synthesize group chat records',
);

const groupServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/GroupService.ts');
const createGroupSource = groupServiceSource.slice(
  groupServiceSource.indexOf('  async createGroup('),
  groupServiceSource.indexOf('  async getGroups('),
);
assert.match(groupServiceSource, /@sdkwork\/im-sdk/u, 'group service must route group conversations through @sdkwork/im-sdk types');
assert.match(groupServiceSource, /getImSdkClientWithSession/u, 'group service must use the shared IM SDK client wrapper');
assert.match(
  groupServiceSource,
  /class\s+SdkworkGroupService/u,
  'group service must expose an SDK-backed service implementation',
);
assert.match(
  groupServiceSource,
  /\.conversations\.create\s*\(/u,
  'group service createGroup must create group conversations through the IM SDK',
);
assert.match(
  groupServiceSource,
  /crypto\.randomUUID/u,
  'group service createGroup must use standard UUID client request ids instead of Date.now/Math.random mock ids',
);
assert.match(
  groupServiceSource,
  /\.conversations\.listMembers\s*\(/u,
  'group service must list conversation members through the IM SDK',
);
assert.match(
  groupServiceSource,
  /\.conversations\.addMember\s*\(/u,
  'group service addMembers must add conversation members through the IM SDK',
);
assert.match(
  groupServiceSource,
  /\.conversations\.updateProfile\s*\(/u,
  'group service createGroup must persist group display profile through the IM SDK',
);
assert.match(
  groupServiceSource,
  /\.conversations\.updatePreferences\s*\([\s\S]*\{\s*isHidden:\s*false\s*\}/u,
  'group service createGroup must unhide the backend-created group conversation through the IM SDK',
);
assert.match(
  groupServiceSource,
  /\.conversations\.removeMember\s*\(/u,
  'group service removeMember must remove conversation members through the IM SDK',
);
assert.match(
  groupServiceSource,
  /\.conversations\.leave\s*\(/u,
  'group service deleteGroup must leave group conversations through the IM SDK',
);
assert.match(
  groupServiceSource,
  /async\s+deleteGroup\s*\(\s*groupId:\s*string\s*\)[\s\S]*?await\s+this\.client\(\)\.conversations\.leave\(groupId\)[\s\S]*?this\.chatClient\.deleteChat\(groupId\)/u,
  'group service deleteGroup must clear ChatService local view and message caches after a successful group leave',
);
assertNoImDeviceApiUsage(groupServiceSource, 'group service');
assert.match(
  groupServiceSource,
  /async\s+getGroups\s*\(\s*\)[\s\S]*?this\.listGroupsPage\s*\(/u,
  'group service getGroups must read SDK inbox group projections through listGroupsPage',
);
assert.match(
  groupServiceSource,
  /async\s+getGroupById\s*\(/u,
  'group service must hydrate a single group by conversation id without scanning the full inbox',
);
const groupListGroupsPageSource = groupServiceSource.slice(
  groupServiceSource.indexOf('  async listGroupsPage('),
  groupServiceSource.indexOf('  async getGroupById(', groupServiceSource.indexOf('  async listGroupsPage(')),
);
assert.match(
  groupListGroupsPageSource,
  /chat\?\.inbox\?\.list[\s\S]*?hydrateConversationEntryGroup\(entry\)/u,
  'group service listGroupsPage must hydrate group rows from paginated SDK inbox projections',
);
assert.match(
  groupListGroupsPageSource,
  /chat\?\.inbox\?\.list\s*\(\s*\{[\s\S]*conversationType:\s*['"]group['"]/u,
  'group service listGroupsPage must request server-side group filtering instead of paging the mixed inbox then filtering locally',
);
assert.doesNotMatch(
  groupListGroupsPageSource,
  /withMemberState\s*\(|conversations\.(?:getPreferences|getProfile|listMembers|list)\s*\(/u,
  'group service listGroupsPage must not issue per-group preference, profile, member, or conversation-list fallback reads',
);
assert.doesNotMatch(
  groupServiceSource,
  /async\s+getGroups\s*\(\s*\)[\s\S]*?this\.chatClient\.getChats\(\)/u,
  'group service getGroups must not call ChatService.getChats because that hydrates unrelated single chats and creates avoidable N+1 work',
);
const groupGetGroupsSource = groupServiceSource.slice(
  groupServiceSource.indexOf('  async getGroups('),
  groupServiceSource.indexOf('  async syncGroupMembers(', groupServiceSource.indexOf('  async getGroups(')),
);
assert.doesNotMatch(
  groupGetGroupsSource,
  /while\s*\(|cursor|MAX_GROUP_INBOX_ENTRIES|listMissingConversationGroupEntries|hydrateConversationEntryGroups|withMemberState\s*\(|conversations\.(?:listMembers|getProfile|getPreferences|list)\s*\(/u,
  'group service getGroups compatibility entry must not aggregate inbox pages or perform per-group detail hydration',
);
assert.match(
  groupGetGroupsSource,
  /const\s+page\s*=\s*await\s+this\.listGroupsPage\(\s*\{\s*pageSize:\s*GROUP_INBOX_PAGE_LIMIT\s*\}\s*\)[\s\S]*?return\s+page\.items/u,
  'group service getGroups compatibility entry must return one bounded group page',
);
const groupGetGroupByIdSource = groupServiceSource.slice(
  groupServiceSource.indexOf('  async getGroupById('),
  groupServiceSource.indexOf('  async getGroups(', groupServiceSource.indexOf('  async getGroupById(')),
);
assert.match(
  groupGetGroupByIdSource,
  /conversations\.getPreferences[\s\S]*?conversations\.getProfile[\s\S]*?this\.withMemberState/u,
  'group service getGroupById must keep explicit active-group detail hydration for one selected group',
);
assert.match(
  groupServiceSource,
  /GROUP_INVITE_DESCRIPTOR_PREFIX/u,
  'group service must define a stable group invitation card descriptor prefix for target-client click handling',
);
assert.match(
  groupServiceSource,
  /parseGroupInviteDescriptor/u,
  'group service must expose group invitation descriptor parsing for received card messages',
);
assert.match(
  groupServiceSource,
  /async\s+inviteUserToGroup\s*\(\s*group:\s*Chat,\s*targetUser:\s*User\s*\)[\s\S]*?this\.addMembers\s*\(\s*group\.id,\s*\[[\s\S]*?targetUserId[\s\S]*?\]\s*\)[\s\S]*?this\.chatClient\.startDirectChat\s*\([\s\S]*?targetUser[\s\S]*?\)[\s\S]*?this\.chatClient\.sendMessage\s*\([\s\S]*?directChat\.id[\s\S]*?['"]card['"]/u,
  'group service must add the non-contact invitee as a member, open a direct chat, and send a clickable group invitation card through ChatService',
);
assert.doesNotMatch(groupServiceSource, /class\s+MockGroupService/u, 'group service must not be mock-backed');
assert.doesNotMatch(groupServiceSource, /mockGroups|setTimeout|console\.log/u, 'group service must not keep mock group branches');
assert.doesNotMatch(groupServiceSource, /group-\$\{Date\.now\(\)\}-\$\{Math\.random/u, 'group service must not generate mock group ids with Date.now and Math.random');
assert.doesNotMatch(createGroupSource, /chatService\.updateChat\s*\(/u, 'group service createGroup must not mask backend group creation with local updateChat fallbacks');
assert.doesNotMatch(groupServiceSource, /addMembersBySearchQuery|social\.users\.list/u, 'group service must not resolve arbitrary text through social user search for group membership');
assert.doesNotMatch(groupServiceSource, /\bfetch\s*\(/u, 'group service must not use raw fetch');
assert.doesNotMatch(groupServiceSource, /\/im\/v3/u, 'group service must not hand-code IM HTTP paths');
assert.doesNotMatch(groupServiceSource, /\b(Authorization|Access-Token|X-API-Key)\b/u, 'group service must not assemble auth headers manually');

assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/RoomService.ts');
const roomServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/RoomService.ts');
assert.match(roomServiceSource, /@sdkwork\/im-sdk/u, 'room service must route room flows through @sdkwork/im-sdk types');
assert.match(roomServiceSource, /getImSdkClientWithSession/u, 'room service must use the shared IM SDK client wrapper');
assert.match(roomServiceSource, /class\s+SdkworkRoomService/u, 'room service must expose a concrete SDK-backed implementation');
assert.match(roomServiceSource, /\.rooms\.create\s*\(/u, 'room service must create rooms through composed IM SDK rooms.create');
assert.match(roomServiceSource, /\.rooms\.enter\s*\(/u, 'room service must enter rooms through composed IM SDK rooms.enter');
assert.match(roomServiceSource, /\.rooms\.leave\s*\(/u, 'room service must leave rooms through composed IM SDK rooms.leave');
assert.match(roomServiceSource, /\.rooms\.get\s*\(/u, 'room service must read room metadata through composed IM SDK rooms.get');
assert.match(
  roomServiceSource,
  /buildGameMoveSchemaRef\s*\(/u,
  'room service must build game move schema refs through the canonical IM custom schema prefix',
);
assert.match(
  roomServiceSource,
  /SDKWORK_IM_GAME_MOVE_SCHEMA_PREFIX\s*=\s*'urn:sdkwork:sdkwork-im:message:custom:game\.'/u,
  'room service must keep the game move schema prefix aligned with im-domain-core',
);
assert.match(
  roomServiceSource,
  /\.conversations\.postMessage\s*\([\s\S]*kind:\s*'data'/u,
  'room service postGameMove must publish game payloads through standard DataContentPart messages',
);
assert.match(
  roomServiceSource,
  /\.conversations\.postText\s*\(/u,
  'room service must publish live/chat room text through conversations.postText',
);
assertNoImDeviceApiUsage(roomServiceSource, 'room service');
assert.doesNotMatch(roomServiceSource, /\bfetch\s*\(/u, 'room service must not use raw fetch');
assert.doesNotMatch(roomServiceSource, /\/im\/v3/u, 'room service must not hand-code IM HTTP paths');
assert.doesNotMatch(roomServiceSource, /\b(Authorization|Access-Token|X-API-Key)\b/u, 'room service must not assemble auth headers manually');

const newFriendsContainerSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/contacts/NewFriendsContainer.tsx');
assert.match(
  newFriendsContainerSource,
  /Avatar\s+src=\{req\.avatar\}/u,
  'new friends list must render the request avatar field without hard-coded placeholder URLs',
);
assert.match(
  newFriendsContainerSource,
  /SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT/u,
  'new friends list must refresh on friend request change events',
);
assert.doesNotMatch(
  newFriendsContainerSource,
  /picsum\.photos\/seed\/\$\{req\.name\}/u,
  'new friends list must not synthesize friend request avatars from picsum',
);

const agentServiceSource = readSibling(
  'sdkwork-agents/apps/sdkwork-agents-pc/packages/sdkwork-agents-pc-agents/src/services/AgentService.ts',
);
const imAgentsPcIntegrationSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/agentsPcIntegration.ts',
);
assert.match(
  imAgentsPcIntegrationSource,
  /getAgentAppSdkClientWithSession/u,
  'IM PC agents integration must wire sdkwork-agents runtime through the shared IM core agent SDK client wrapper',
);
assert.match(
  agentServiceSource,
  /getAgentsAppSdkClientWithSession/u,
  'agent service catalog, lifecycle, and runtime operations must use the shared sdkwork-agents-app-sdk client wrapper',
);
assert.doesNotMatch(
  agentServiceSource,
  /readAppSdkSessionTokens|resolveAppSdkTenantId|resolveAppSdkOrganizationId|resolveAppSdkUserId/u,
  'agent service must not derive tenant, organization, or owner user scope in the frontend; appbase request context owns scope',
);
assert.doesNotMatch(
  agentServiceSource,
  /\b(?:tenantId|organizationId|ownerUserId)\b/u,
  'agent service must not pass tenant, organization, or owner user scope to sdkwork-agent-app-sdk requests',
);
assert.match(
  agentServiceSource,
  /@sdkwork\/agents-pc-core\/sdk/u,
  'agent service must consume typed agent SDK contracts from the agents PC core package',
);
assert.match(
  agentServiceSource,
  /class\s+SdkworkAgentService/u,
  'agent service must expose an SDK-backed implementation instead of a mock implementation',
);
assert.match(
  agentServiceSource,
  /\.ai\.agents\.list\s*\(/u,
  'agent service discovery lists must read backend agent records through sdkwork-agent-app-sdk',
);
assert.match(
  agentServiceSource,
  /\.ai\.agents\.create\s*\(/u,
  'agent create flow must persist through sdkwork-agent-app-sdk',
);
assert.match(
  agentServiceSource,
  /\.ai\.agents\.update\s*\(/u,
  'agent update flow must persist through sdkwork-agent-app-sdk',
);
assert.match(
  agentServiceSource,
  /\.ai\.agents\.providerBindings\.create\s*\(/u,
  'agent publish flow must prepare a provider binding through sdkwork-agent-app-sdk before deployment',
);
assert.doesNotMatch(
  agentServiceSource,
  /\.ai\.agents\.deployments\.create\s*\(/u,
  'agent publish flow must not call deployments.create until sdkwork-agents-app-api exposes agents.deployments.create in generated transport',
);
assert.match(
  agentServiceSource,
  /\.ai\.agents\.delete\s*\(/u,
  'agent delete flow must persist through sdkwork-agent-app-sdk',
);
assert.match(
  agentServiceSource,
  /\.ai\.agents\.previewResponses\.create\s*\(/u,
  'agent preview flow must execute through sdkwork-agent-app-sdk runtime preview responses',
);
assert.match(
  agentServiceSource,
  /\.ai\.agents\.promptOptimizations\.create\s*\(/u,
  'agent prompt optimizer must execute through sdkwork-agent-app-sdk runtime prompt optimizations',
);
assert.match(
  agentServiceSource,
  /model:\s*normalizeModelForRuntime\s*\(\s*config\.model\s*\)/u,
  'agent runtime payloads must normalize known UI model labels while preserving backend runtime model ids',
);
assert.match(
  agentServiceSource,
  /const\s+model\s*=\s*normalizeModelForRuntime\s*\(\s*request\.model\s*\?\?\s*request\.config\.model\s*\)/u,
  'agent preview runtime requests must use the runtime model normalization boundary',
);
assert.doesNotMatch(agentServiceSource, /class\s+MockAgentService/u, 'agent service must not be mock-backed');
assert.doesNotMatch(agentServiceSource, /mockAgents|mockMarketAgents/u, 'agent service must not keep mock agent catalogs');
assert.doesNotMatch(
  agentServiceSource,
  /getAppSdkClientWithSession|SdkworkImAppClient|\.automation\.executions\.create|pc\.agent\.(?:preview_response|prompt_optimize)/u,
  'agent service must not bypass sdkwork-agent-app-sdk runtime operations through im-app-sdk automation execution',
);
assert.doesNotMatch(agentServiceSource, /setTimeout|console\.log|Date\.now/u, 'agent service must not simulate async agent catalog behavior locally');
assert.doesNotMatch(agentServiceSource, /\bfetch\s*\(/u, 'agent service must not use raw fetch');
assert.doesNotMatch(agentServiceSource, /\/(?:im|app|backend)\/v3/u, 'agent service must not hand-code SDK-owned API paths');
assert.doesNotMatch(agentServiceSource, /\b(Authorization|Access-Token|X-API-Key)\b/u, 'agent service must not assemble auth headers manually');

const knowledgebasePcIntegrationSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/knowledgebasePcIntegration.ts',
);
const knowledgebaseAppSdkClientSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/knowledgebaseAppSdkClient.ts',
);
const knowledgebasePcBootstrapSource = read('apps/sdkwork-im-pc/src/bootstrap/knowledgebasePc.ts');
const knowledgeShellLoadersSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-shell/src/capabilityModuleLoaders.ts');
const knowledgeEmbedIndexSource = readSibling(
  'sdkwork-knowledgebase/apps/sdkwork-knowledgebase-pc/packages/sdkwork-knowledgebase-pc-knowledge/src/index.ts',
);
const selectKnowledgeModalSource = readSibling(
  'sdkwork-agents/apps/sdkwork-agents-pc/packages/sdkwork-agents-pc-agents/src/components/SelectKnowledgeModal.tsx',
);

assert.equal(
  fs.existsSync(path.join(repoRoot, 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-knowledge')),
  false,
  'apps/sdkwork-im-pc must not keep a local sdkwork-im-pc-knowledge package.',
);
assert.match(
  knowledgeShellLoadersSource,
  /knowledge:\s*async\s*\(\)\s*=>\s*\{[\s\S]*import\(['"]@sdkwork\/knowledgebase-pc-knowledge['"]\)[\s\S]*ensureKnowledgebasePcRuntimeOnModule/u,
  'IM shell must lazy-load the sdkwork-knowledgebase-pc-knowledge capability package with host runtime wiring.',
);
assert.match(
  knowledgebasePcIntegrationSource,
  /configureKnowledgebasePcRuntime/u,
  'IM core must configure the embeddable Knowledgebase PC runtime through sdkPorts.',
);
assert.match(
  knowledgebaseAppSdkClientSource,
  /from ['"]@sdkwork\/knowledgebase-app-sdk['"]/u,
  'IM must consume knowledgebase capabilities through the composed knowledgebase app SDK.',
);
assert.match(
  knowledgebaseAppSdkClientSource,
  /createKnowledgebaseAppClient/u,
  'IM must initialize knowledgebase SDK through createKnowledgebaseAppClient.',
);
assert.match(
  knowledgebasePcBootstrapSource,
  /bootstrapKnowledgebasePcForIm/u,
  'IM bootstrap must initialize Knowledgebase PC integration before rendering shell modules.',
);
assert.match(
  selectKnowledgeModalSource,
  /knowledgeSelectionService\.getBasesPage\s*\(/u,
  'agent knowledge picker must list bases through paginated knowledgebase embed selection service.',
);
assert.match(
  knowledgeEmbedIndexSource,
  /export\s*\{\s*KnowledgeView\s*\}/u,
  'sdkwork-knowledgebase-pc-knowledge must export KnowledgeView for host-managed embedding.',
);
assert.doesNotMatch(
  knowledgebasePcIntegrationSource,
  /\bfetch\s*\(/u,
  'IM Knowledgebase integration must not use raw fetch',
);

const createAgentViewSource = readSibling(
  'sdkwork-agents/apps/sdkwork-agents-pc/packages/sdkwork-agents-pc-agents/src/pages/CreateAgentView.tsx',
);
assert.match(
  createAgentViewSource,
  /agentService\.requestPreviewResponse\s*\(/u,
  'create agent preview chat must request real backend agent runtime execution through AgentService',
);
assert.match(
  createAgentViewSource,
  /ensurePersistedAgentForRuntime\s*\(/u,
  'create agent runtime preview and prompt optimizer must persist a standard backend agent before execution',
);
assert.match(
  createAgentViewSource,
  /config:\s*await\s+ensurePersistedAgentForRuntime\s*\(/u,
  'create agent runtime requests must not send local draft ids to backend agent runtime endpoints',
);
assert.match(
  createAgentViewSource,
  /agentService\.optimizePrompt\s*\(/u,
  'create agent prompt optimizer must request real backend agent runtime execution through AgentService',
);
assert.match(
  createAgentViewSource,
  /knowledgeBaseIds:\s*selectedKnowledgeIds/u,
  'create agent save and publish flows must persist selected knowledge base bindings through AgentService',
);
assert.match(
  createAgentViewSource,
  /welcomeMessage/u,
  'create agent save and publish flows must persist the configured welcome message through AgentService',
);
assert.match(
  createAgentViewSource,
  /agentService\.getAgent\s*\(\s*initialAgentId\s*\)/u,
  'create agent edit mode must load editable targets from the current user owned agent record',
);
assert.doesNotMatch(
  createAgentViewSource,
  /agentService\.getMarketAgents\s*\(/u,
  'create agent edit mode must not treat marketplace agents as editable targets',
);
assert.match(
  createAgentViewSource,
  /const\s+resolveMutableAgentId\s*=/u,
  'create agent save, publish, preview, and prompt optimization must guard mutations behind a confirmed owned draft id',
);
assert.doesNotMatch(
  createAgentViewSource,
  /draftId\s*\?\?\s*initialAgentId/u,
  'create agent mutations must not fall back to route initialAgentId when ownership was not confirmed',
);
assert.doesNotMatch(
  createAgentViewSource,
  /id:\s*draftId\s*\?\?\s*initialAgentId\s*\?\?\s*createDraftAgentId\s*\(/u,
  'create agent runtime configs must not synthesize non-standard draft-* ids for backend execution',
);
assert.doesNotMatch(
  createAgentViewSource,
  /Mock response|杩欐槸鍩轰簬褰撳墠閰嶇疆鐨勬祴璇曞洖澶嶃€俓n|setTimeout\s*\(\s*\(\s*\)\s*=>\s*\{[\s\S]*setTestMessages|setTimeout\s*\(\s*\(\s*\)\s*=>\s*\{[\s\S]*setPrompt/u,
  'create agent view must not synthesize local preview replies or prompt optimization results',
);

assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ImSyncCoordinatorService.ts');
const imSyncCoordinatorServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ImSyncCoordinatorService.ts');
assert.match(
  imSyncCoordinatorServiceSource,
  /class\s+SdkworkImSyncCoordinatorService/u,
  'IM sync coordinator must expose a real SDK-backed startup synchronization service',
);
assert.match(
  imSyncCoordinatorServiceSource,
  /syncOfflineMessages\s*\(\s*\)/u,
  'IM sync coordinator startup sync must run offline message window synchronization',
);
assert.doesNotMatch(
  imSyncCoordinatorServiceSource,
  /this\.contactService|result\.contacts\s*=/u,
  'IM sync coordinator startup sync must not refresh contacts before the contacts surface is opened',
);
assert.doesNotMatch(
  imSyncCoordinatorServiceSource,
  /syncGroupMembers\s*\(\s*\)/u,
  'IM sync coordinator startup sync must not enumerate every group member list before a group is active',
);
assertNoImDeviceApiUsage(imSyncCoordinatorServiceSource, 'IM sync coordinator');
assert.doesNotMatch(imSyncCoordinatorServiceSource, /\bfetch\s*\(/u, 'IM sync coordinator must not use raw fetch');
assert.doesNotMatch(imSyncCoordinatorServiceSource, /\/im\/v3/u, 'IM sync coordinator must not hand-code IM HTTP paths');
assert.doesNotMatch(imSyncCoordinatorServiceSource, /\b(Authorization|Access-Token|X-API-Key)\b/u, 'IM sync coordinator must not assemble auth headers manually');

const chatPackageIndexSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/index.ts');
assert.match(
  chatPackageIndexSource,
  /export\s+\{\s*imSyncCoordinatorService\s*\}\s+from\s+["']\.\/services\/ImSyncCoordinatorService["']/u,
  'chat package must export the SDK-backed IM sync coordinator as a first-class service',
);
assert.match(
  chatPackageIndexSource,
  /export\s+type\s+\{[^}]*ImStartupSyncResult[^}]*ImSyncCoordinatorService[^}]*\}\s+from\s+["']\.\/services\/ImSyncCoordinatorService["']/u,
  'chat package must export IM startup sync coordinator types for package consumers',
);

const chatRightPanelSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ChatRightPanel.tsx');
const addGroupMembersModalSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/AddGroupMembersModal.tsx');
const chatLayoutSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/pages/ChatLayout.tsx');
const capabilityModuleSurfaceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/surfaces/CapabilityModuleSurface.tsx',
);
const mergeGroupProjectionsStart = chatLayoutSource.indexOf('const mergeGroupProjections = async');
assert.notEqual(mergeGroupProjectionsStart, -1, 'chat layout must keep realtime group projection merge behavior auditable');
const mergeGroupProjectionsEnd = chatLayoutSource.indexOf('  const needsGroupDetailHydration', mergeGroupProjectionsStart);
assert.notEqual(mergeGroupProjectionsEnd, -1, 'chat layout group projection merge helper must end before refreshChats');
const mergeGroupProjectionsSource = chatLayoutSource.slice(mergeGroupProjectionsStart, mergeGroupProjectionsEnd);
const openHydratedChatStart = chatLayoutSource.indexOf('const openHydratedChat = async');
assert.notEqual(openHydratedChatStart, -1, 'chat layout must keep backend-hydrated chat opening behavior auditable');
const openHydratedChatEnd = chatLayoutSource.indexOf('  const loadChatStartup = async', openHydratedChatStart);
assert.notEqual(openHydratedChatEnd, -1, 'chat layout backend-hydrated chat opening helper must end before startup loading');
const openHydratedChatSource = chatLayoutSource.slice(openHydratedChatStart, openHydratedChatEnd);
const subscribeChatsStart = chatLayoutSource.indexOf('return chatService.subscribeChats((nextChats) => {');
assert.notEqual(subscribeChatsStart, -1, 'chat layout must keep realtime chat-list subscription behavior auditable');
const subscribeChatsEnd = chatLayoutSource.indexOf('  }, []);', subscribeChatsStart);
assert.notEqual(subscribeChatsEnd, -1, 'chat layout realtime chat-list subscription block must end at its effect dependency list');
const subscribeChatsSource = chatLayoutSource.slice(subscribeChatsStart, subscribeChatsEnd);
const editNameModalStart = chatLayoutSource.indexOf('{activeModal === "editName" && (');
assert.notEqual(editNameModalStart, -1, 'chat layout must keep edit-name modal behavior auditable');
const editNameModalEnd = chatLayoutSource.indexOf('{activeModal === "editNotice"', editNameModalStart);
assert.notEqual(editNameModalEnd, -1, 'chat layout edit-name modal block must end before edit-notice modal');
const editNameModalSource = chatLayoutSource.slice(editNameModalStart, editNameModalEnd);
const editNoticeModalStart = chatLayoutSource.indexOf('{activeModal === "editNotice" && (');
assert.notEqual(editNoticeModalStart, -1, 'chat layout must keep edit-notice modal behavior auditable');
const editNoticeModalEnd = chatLayoutSource.indexOf('</motion.div>', editNoticeModalStart);
assert.notEqual(editNoticeModalEnd, -1, 'chat layout edit-notice modal block must end before inline modal closes');
const editNoticeModalSource = chatLayoutSource.slice(editNoticeModalStart, editNoticeModalEnd);
const contactsViewStart = capabilityModuleSurfaceSource.indexOf('case "contacts":');
assert.notEqual(contactsViewStart, -1, 'capability module surface must keep the contacts view behavior auditable');
const contactsViewEnd = capabilityModuleSurfaceSource.indexOf('case "favorites":', contactsViewStart);
assert.notEqual(contactsViewEnd, -1, 'capability module surface contacts branch must end before favorites branch');
const contactsViewSource = capabilityModuleSurfaceSource.slice(contactsViewStart, contactsViewEnd);
const contactsSendMessageStart = chatLayoutSource.indexOf('onContactSendMessage={async');
assert.notEqual(contactsSendMessageStart, -1, 'chat layout must keep contacts send-message behavior auditable');
const contactsSendMessageEnd = chatLayoutSource.indexOf('onContactStartCall=', contactsSendMessageStart);
assert.notEqual(contactsSendMessageEnd, -1, 'chat layout contacts send-message block must end before start-call block');
const contactsSendMessageSource = chatLayoutSource.slice(contactsSendMessageStart, contactsSendMessageEnd);
const agentStartHandlerStart = chatLayoutSource.indexOf('const handleStartAgentChat = async');
assert.notEqual(agentStartHandlerStart, -1, 'chat layout must keep the agent start-chat handler auditable');
const agentStartHandlerEnd = chatLayoutSource.indexOf('  const renderHeaderContent', agentStartHandlerStart);
assert.notEqual(agentStartHandlerEnd, -1, 'chat layout agent start-chat handler must end before renderHeaderContent');
const agentStartHandlerSource = chatLayoutSource.slice(agentStartHandlerStart, agentStartHandlerEnd);
const enterpriseViewStart = capabilityModuleSurfaceSource.indexOf('case "enterprise":');
assert.notEqual(enterpriseViewStart, -1, 'capability module surface must keep the enterprise view behavior auditable');
const enterpriseViewEnd = capabilityModuleSurfaceSource.indexOf('case "devices":', enterpriseViewStart);
assert.notEqual(enterpriseViewEnd, -1, 'capability module surface enterprise branch must end before devices branch');
const enterpriseViewSource = capabilityModuleSurfaceSource.slice(enterpriseViewStart, enterpriseViewEnd);
const enterpriseStartChatStart = chatLayoutSource.indexOf('onEnterpriseStartChat={async');
assert.notEqual(enterpriseStartChatStart, -1, 'chat layout must keep enterprise start-chat behavior auditable');
const enterpriseStartChatEnd = chatLayoutSource.indexOf('onEnterpriseCall=', enterpriseStartChatStart);
assert.notEqual(enterpriseStartChatEnd, -1, 'chat layout enterprise start-chat block must end before enterprise call block');
const enterpriseStartChatSource = chatLayoutSource.slice(enterpriseStartChatStart, enterpriseStartChatEnd);
assert.match(
  chatLayoutSource,
  /import\s+\{\s*groupService\s*\}\s+from\s+["']\.\.\/services\/GroupService["']/u,
  'chat layout group member invite flow must import the SDK-backed group service',
);
assert.match(
  chatLayoutSource,
  /import\s+\{\s*imSyncCoordinatorService\s*\}\s+from\s+["']\.\.\/services\/ImSyncCoordinatorService["']/u,
  'chat layout startup lifecycle must import the SDK-backed IM sync coordinator',
);
assert.match(
  chatLayoutSource,
  /imSyncCoordinatorService\.syncStartup\s*\(\s*\)/u,
  'chat layout startup lifecycle must run the lightweight IM startup coordinator before initial chat hydration',
);
assert.doesNotMatch(
  chatLayoutSource,
  /listContactConversationIds\s*\(/u,
  'chat layout startup and call watch flows must not enumerate every contact conversation id',
);
assert.match(
  chatLayoutSource,
  /<CreateGroupModal[\s\S]*?onCreated=\{async\s*\(group\)\s*=>\s*\{[\s\S]*?await\s+openHydratedChat\(group\)[\s\S]*?\}/u,
  'chat layout create-group flow must hydrate backend conversations and open the real group chat after creation',
);
assert.match(
  chatLayoutSource,
  /contactService\.getUserById\s*\(\s*user\.id\s*\)/u,
  'chat layout contacts send-message flow must hydrate contact projection ids before starting a direct chat',
);
assert.match(
  chatLayoutSource,
  /contactService\.listContactsPage\s*\(\s*\)/u,
  'chat layout contacts send-message flow must fall back to the contact projection list when a cached user lacks conversation ids',
);
assert.match(
  contactsSendMessageSource,
  /resolveContactChatTarget\s*\(\s*user\s*\)/u,
  'chat layout contacts send-message flow must resolve a hydrated contact target before starting a direct chat',
);
assert.match(
  contactsSendMessageSource,
  /chatService\.startDirectChat\s*\(\s*chatTarget\s*\)/u,
  'chat layout contacts send-message flow must start a real SDK-backed direct chat with the hydrated contact target',
);
assert.match(
  contactsSendMessageSource,
  /refreshChats\s*\(\s*\)/u,
  'chat layout contacts send-message flow must hydrate from backend conversations after direct chat binding',
);
assert.match(
  contactsViewSource,
  /onOpenGroup=\{onOpenGroup\}/u,
  'capability module surface contacts flow must delegate group open behavior to ChatLayout',
);
assert.match(
  chatLayoutSource,
  /onOpenGroup=\{openHydratedChat\}/u,
  'chat layout contacts group flow must hydrate backend conversations and open the selected real group chat',
);
assert.doesNotMatch(
  contactsSendMessageSource,
  /id:\s*user\.id/u,
  'chat layout contacts send-message flow must not synthesize a local chat id from the contact user id',
);
assert.doesNotMatch(
  contactsSendMessageSource,
  /lastMessage:\s*\{/u,
  'chat layout contacts send-message flow must not synthesize a local empty lastMessage for a direct chat',
);
assert.match(
  chatServiceSource,
  /startAgentChat\s*\([\s\S]*?const\s+auth\s*=\s*await\s+this\.beginAuthSessionOperation\s*\(\s*operation\s*\)[\s\S]*?findExistingAgentDialogEntry\s*\(\s*agentId\s*,\s*auth\s*\)[\s\S]*?auth\.client\.conversations\.createAgentDialog\s*\([\s\S]*?this\.assertAuthSessionGenerationCurrent\s*\(\s*auth\.generation\s*,\s*operation\s*\)/u,
  'chat service agent start flow must reuse one authenticated SDK context to check the unified inbox before creating and generation-fencing an agent dialog',
);
assert.match(
  chatServiceSource,
  /findExistingAgentDialogEntry\s*\([\s\S]*?\.chat\.inbox\.list\s*\(/u,
  'chat service must discover existing agent conversations from the unified inbox list',
);
assert.match(
  chatServiceSource,
  /inboxEntryContainsAgent\s*\([\s\S]*?syncConversationMembers\s*\(/u,
  'chat service must confirm the requested agent through conversation members before reusing an inbox agent dialog',
);
assert.match(
  agentStartHandlerSource,
  /chatService\.startAgentChat\s*\(\s*agent\s*\)/u,
  'chat layout agent start flow must delegate to the SDK-backed chat service instead of synthesizing a local agent chat',
);
assert.doesNotMatch(
  agentStartHandlerSource,
  /const\s+chatId\s*=\s*`agent-\$\{agent\.id\}`/u,
  'chat layout agent start flow must not synthesize a local agent chat id',
);
assert.doesNotMatch(
  agentStartHandlerSource,
  /senderId:\s*["']bot["'][\s\S]*type:\s*["']text["']/u,
  'chat layout agent start flow must not synthesize a local bot welcome message',
);
assert.match(
  chatServiceSource,
  /startEnterpriseChat\s*\([\s\S]*?\.conversations\.bindDirectChat\s*\(/u,
  'chat service must start enterprise chats through the SDK-backed direct chat binding API',
);
assert.match(
  chatServiceSource,
  /rightActorKind:\s*['"]enterprise['"]/u,
  'chat service enterprise chat binding must model the target as an enterprise principal',
);
assert.match(
  enterpriseViewSource,
  /onStartChat=\{onEnterpriseStartChat\}/u,
  'capability module surface enterprise start flow must delegate to ChatLayout handlers',
);
assert.match(
  enterpriseStartChatSource,
  /chatService\.startEnterpriseChat\s*\(\s*\{\s*id:\s*enterpriseId,\s*name:\s*enterpriseName,\s*\}\s*\)/u,
  'chat layout enterprise start flow must create a real SDK-backed enterprise conversation',
);
assert.match(
  enterpriseStartChatSource,
  /refreshChats\s*\(\s*\)/u,
  'chat layout enterprise start flow must hydrate from backend conversations after enterprise binding',
);
assert.doesNotMatch(
  enterpriseStartChatSource,
  /const\s+chatId\s*=\s*`ent-\$\{enterpriseId\}`/u,
  'chat layout enterprise start flow must not synthesize a local enterprise chat id',
);
assert.doesNotMatch(
  enterpriseStartChatSource,
  /lastMessage:\s*\{/u,
  'chat layout enterprise start flow must not synthesize a local enterprise welcome message',
);
assert.doesNotMatch(
  enterpriseStartChatSource,
  /senderId:\s*["']bot["']/u,
  'chat layout enterprise start flow must not synthesize bot-authored enterprise messages',
);
assert.doesNotMatch(
  enterpriseStartChatSource,
  /members:\s*\[\s*["']currentUser["']\s*,\s*["']ent["']\s*\]/u,
  'chat layout enterprise start flow must not synthesize local enterprise members',
);
const enterpriseServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-enterprise/src/services/EnterpriseService.ts');
const enterpriseListSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-enterprise/src/components/EnterpriseList.tsx');
assert.match(
  enterpriseServiceSource,
  /getAppSdkClientWithSession/u,
  'enterprise service must use the shared generated im-app-sdk client wrapper',
);
assert.match(
  enterpriseServiceSource,
  /class\s+SdkworkEnterpriseService/u,
  'enterprise service must expose an SDK-backed enterprise catalog implementation',
);
assert.match(
  enterpriseServiceSource,
  /retrievePortalHome\s*\(\s*client\s*\)/u,
  'enterprise service must load the backend portal home snapshot through the shared generated-SDK adapter',
);
assert.match(
  enterpriseServiceSource,
  /\.portal\.workspace\.retrieve\s*\(\s*\)/u,
  'enterprise service must fall back to the real workspace snapshot instead of hard-coded enterprises',
);
assert.match(
  enterpriseListSource,
  /enterpriseService\.getEnterprises\s*\(\s*\)/u,
  'enterprise list must render enterprise principals from the SDK-backed enterprise service',
);
assert.doesNotMatch(enterpriseListSource, /mockEnterprises/u, 'enterprise list must not render a local mock enterprise catalog');
assert.doesNotMatch(enterpriseServiceSource, /mockEnterprises|dice(?:bear)/u, 'enterprise service must not keep mock enterprise catalog data');
assert.doesNotMatch(enterpriseServiceSource, /\bfetch\s*\(/u, 'enterprise service must not use raw fetch');
assert.doesNotMatch(enterpriseServiceSource, /\/(?:im|app|backend)\/v3/u, 'enterprise service must not hand-code SDK-owned API paths');
assert.doesNotMatch(enterpriseServiceSource, /\b(Authorization|Access-Token|X-API-Key)\b/u, 'enterprise service must not assemble auth headers manually');
const enterpriseMarketplaceServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-enterprise/src/services/EnterpriseMarketplaceService.ts');
const recruitListSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-enterprise/src/components/RecruitList.tsx');
const supplyListSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-enterprise/src/components/SupplyList.tsx');
const purchaseListSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-enterprise/src/components/PurchaseList.tsx');
const enterpriseDetailAboutSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-enterprise/src/components/EnterpriseDetailAbout.tsx');
const enterpriseDetailProductsSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-enterprise/src/components/EnterpriseDetailProducts.tsx');
const enterpriseDetailRecruitsSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-enterprise/src/components/EnterpriseDetailRecruits.tsx');
assert.match(
  enterpriseMarketplaceServiceSource,
  /pc enterprise marketplace contract is not available/u,
  'enterprise marketplace service must fail closed until the enterprise marketplace SDK contract exists',
);
assert.match(
  recruitListSource,
  /enterpriseMarketplaceService\.getRecruits\s*\(\s*\)/u,
  'enterprise recruit list must load listings from the enterprise marketplace service',
);
assert.match(
  supplyListSource,
  /enterpriseMarketplaceService\.getSupplies\s*\(\s*\)/u,
  'enterprise supply list must load listings from the enterprise marketplace service',
);
assert.match(
  purchaseListSource,
  /enterpriseMarketplaceService\.getPurchases\s*\(\s*\)/u,
  'enterprise purchase list must load listings from the enterprise marketplace service',
);
assert.doesNotMatch(recruitListSource, /mockRecruits/u, 'enterprise recruit list must not render a local mock recruit catalog');
assert.doesNotMatch(supplyListSource, /mockSupplies/u, 'enterprise supply list must not render a local mock supply catalog');
assert.doesNotMatch(purchaseListSource, /mockPurchases/u, 'enterprise purchase list must not render a local mock purchase catalog');
assert.doesNotMatch(
  `${enterpriseDetailAboutSource}${enterpriseDetailProductsSource}${enterpriseDetailRecruitsSource}`,
  /unsplash/u,
  'enterprise detail surfaces must not keep demo media or local mock catalog data',
);
assert.match(
  moduleRegistrySource,
  /COMMERCIAL_RUNTIME_MODULES[\s\S]*["']enterprise["']/u,
  'Commercial runtime modules must include enterprise after SDK-backed enterprise catalog integration',
);
const communityViewSource = read('../sdkwork-community/apps/sdkwork-community-pc/packages/sdkwork-community-pc-community/src/components/CommunityView.tsx');
const communitySettingsSource = read('../sdkwork-community/apps/sdkwork-community-pc/packages/sdkwork-community-pc-community/src/components/CommunitySettings.tsx');
const shopHomeSource = readSibling(
  'sdkwork-shop/apps/sdkwork-shop-pc/packages/sdkwork-shop-pc-consumer/src/components/ShopHome.tsx',
);
const checkoutViewSource = readSibling(
  'sdkwork-shop/apps/sdkwork-shop-pc/packages/sdkwork-shop-pc-consumer/src/components/CheckoutView.tsx',
);
const cashierViewSource = readSibling(
  'sdkwork-shop/apps/sdkwork-shop-pc/packages/sdkwork-shop-pc-consumer/src/components/CashierView.tsx',
);
const videoGenServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-video-gen/src/services/VideoGenService.ts');
const videoPlayerViewSource = read('../sdkwork-course/apps/sdkwork-course-pc/packages/sdkwork-course-pc-course/src/components/VideoPlayerView.tsx');
const liveRoomViewSource = read('../sdkwork-course/apps/sdkwork-course-pc/packages/sdkwork-course-pc-course/src/components/LiveRoomView.tsx');
const courseServiceSource = read('../sdkwork-course/apps/sdkwork-course-pc/packages/sdkwork-course-pc-course/src/services/CourseService.ts');
assert.doesNotMatch(
  `${communityViewSource}${communitySettingsSource}`,
  /pravatar|unsplash/u,
  'community surfaces must not keep demo avatar or media placeholders',
);
assert.doesNotMatch(shopHomeSource, /unsplash/u, 'shop home must not keep demo banner media');
assert.doesNotMatch(checkoutViewSource, /MOCK_ADDRESSES|138 \*\*\*\* 0000/u, 'shop checkout must not keep mock shipping addresses or demo account labels');
assert.doesNotMatch(cashierViewSource, /Math\.random|setTimeout/u, 'shop cashier must not simulate payment with timers or random qr codes');
assert.match(videoGenServiceSource, /pc videogen contract is not available/u, 'videogen service must fail closed until the videogen SDK contract exists');
const emojiPickerSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/EmojiPicker.tsx');
const musicPlayerSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/MusicPlayer.tsx');
const messageItemsSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/MessageItems.tsx');
const consoleAnalyticsSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-console-security/src/ConsoleAnalytics.tsx');
assert.doesNotMatch(
  `${emojiPickerSource}${musicPlayerSource}${messageItemsSource}`,
  /picsum\.photos/u,
  'chat media surfaces must not keep external placeholder image hosts',
);
assert.match(
  emojiPickerSource,
  /pc sticker pack contract is not available/u,
  'chat sticker picker must fail closed until the sticker pack SDK contract exists',
);
assert.match(
  messageItemsSource,
  /DEFAULT_MUSIC_COVER_URL/u,
  'chat music messages must use the local default music cover placeholder',
);
assert.match(
  consoleAnalyticsSource,
  /ConsoleContractEmptyState/u,
  'console analytics must render contract-empty state until analytics SDK exists',
);
assert.doesNotMatch(
  consoleAnalyticsSource,
  /8,245|Mock Chart/u,
  'console analytics must not keep embedded demo metrics or charts',
);
const consoleCoreDir = path.join(repoRoot, 'apps/sdkwork-im-pc/packages/sdkwork-im-console-core/src');
const consoleCoreFiles = fs
  .readdirSync(consoleCoreDir)
  .filter((name) => name.endsWith('.tsx') && name !== 'index.tsx');
assert.deepEqual(
  consoleCoreFiles.sort(),
  ['ConsoleLayout.tsx'],
  'console-core must keep layout shell only; domain consoles live in sibling packages',
);
assert.doesNotMatch(
  `${videoPlayerViewSource}${liveRoomViewSource}`,
  /unsplash|ui-avatars/u,
  'course player surfaces must not keep demo avatar or media placeholders',
);
assert.match(
  courseServiceSource,
  /pc course comments contract is not available/u,
  'course interaction service must fail closed until the course comments contract exists',
);
const commonsApiSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-commons/src/api.ts');
const consoleRolesSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-console-roles/src/ConsoleRoles.tsx');
const consoleCourseSource = read('../sdkwork-course/apps/sdkwork-course-pc/packages/sdkwork-course-pc-console/src/ConsoleCourse.tsx');
const consoleProductsSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-console-product/src/ConsoleProducts.tsx');
const consoleStoresSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-console-shop/src/ConsoleStores.tsx');
const integrationServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-console-integrations/src/services/IntegrationService.ts');
assert.match(
  commonsApiSource,
  /pc console api contract is not available/u,
  'pc commons api helpers must fail closed until console api contracts exist',
);
assert.doesNotMatch(
  commonsApiSource,
  /mockConsoleFetch|mockAdminFetch|setTimeout/u,
  'pc commons api must not keep mock fetch helpers or fake delays',
);
assert.doesNotMatch(
  `${consoleRolesSource}${consoleCourseSource}${consoleProductsSource}${consoleStoresSource}`,
  /mockRoles|MOCK_COURSES|mockProducts|mockStores|mockUsers|mockGroups|mockAnnouncements|mockApps/u,
  'console tenant admin surfaces must not keep embedded demo datasets',
);
assert.match(
  consoleRolesSource,
  /roleService\.getRoles/u,
  'console roles surface must load roles through the appbase SDK-backed role service',
);
assert.match(
  integrationServiceSource,
  /console integration contract is not available/u,
  'console integration service must fail closed until the integration SDK contract exists',
);
assert.match(
  consoleCourseSource,
  /courseConsoleService\.(listCourses|createCourse|publishCourse|listCategories|createCategory)/u,
  'console course surface must load and mutate courses through the backend SDK service',
);
assert.match(
  consoleCourseSource,
  /ConsoleContractEmptyState/u,
  'console course surface must render contract-empty state when no courses exist',
);
const consoleLayoutSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-console-core/src/ConsoleLayout.tsx');
assert.match(
  consoleLayoutSource,
  /ConsoleContractEmptyState/u,
  'console layout catch-all route must render contract-empty state for unknown modules',
);
assert.doesNotMatch(
  consoleLayoutSource,
  /模块正在开发中/u,
  'console layout must not keep generic under-development placeholder copy',
);
const workspaceServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-workspace/src/services/WorkspaceService.ts');
const workspaceEntrySource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-workspace/src/index.tsx');
const workspaceViewSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-workspace/src/WorkspaceView.tsx');
assert.match(
  workspaceServiceSource,
  /getAppSdkClientWithSession/u,
  'workspace service must use the shared generated im-app-sdk client wrapper',
);
assert.match(
  workspaceServiceSource,
  /getDriveAppSdkClientWithSession/u,
  'workspace service must use the shared drive app SDK client wrapper for recent documents',
);
assert.match(
  workspaceServiceSource,
  /class\s+SdkworkWorkspaceService/u,
  'workspace service must expose an SDK-backed workspace catalog implementation',
);
assert.match(
  workspaceServiceSource,
  /retrievePortalHome\s*\(\s*this\.getClient\(\)\s*\)/u,
  'workspace service must load the backend portal home snapshot through the shared generated-SDK adapter',
);
assert.match(
  workspaceServiceSource,
  /\.drive\.recent\.list\s*\(/u,
  'workspace service must load recent documents through the generated drive app SDK',
);
assert.match(
  workspaceViewSource,
  /service\.getWorkspaceData\s*\(\s*\)/u,
  'workspace view must render apps and recent documents from the SDK-backed aggregate service',
);
assert.match(
  workspaceServiceSource,
  /Promise\.all\s*\(\s*\[\s*this\.loadApps\(\),\s*this\.loadRecentDocuments\(\),?\s*\]\s*\)/u,
  'workspace service must load the app catalog and recent documents without serial SDK latency',
);
assert.doesNotMatch(
  workspaceEntrySource,
  /useState|workspaceService/u,
  'workspace public entrypoint must remain a thin export boundary',
);
assert.match(
  workspaceServiceSource,
  /isCommercialRuntimeModule/u,
  'workspace service must filter launcher apps to commercial runtime modules',
);
assert.match(
  workspaceViewSource,
  /todayOverview/u,
  'workspace view must render the date overview without hard-coded task counts',
);
assert.doesNotMatch(
  workspaceViewSource,
  /count:\s*3|leading-none">12<|Simulated notification badge/u,
  'workspace view must not keep hard-coded dashboard metrics or simulated badges',
);
assert.doesNotMatch(
  workspaceViewSource,
  /workspaceAppCenterUnavailable|connectingAppCenter/u,
  'workspace view must not expose a placeholder app-center flow without an integration contract',
);
assert.doesNotMatch(workspaceServiceSource, /class\s+WorkspaceCatalogService/u, 'workspace service must not be mock-backed');
assert.doesNotMatch(workspaceServiceSource, /setTimeout|mockMails|recentDocumentCatalog/u, 'workspace service must not keep mock catalog branches');
assert.doesNotMatch(workspaceServiceSource, /\bfetch\s*\(/u, 'workspace service must not use raw fetch');
assert.doesNotMatch(workspaceServiceSource, /\/(?:im|app|backend)\/v3/u, 'workspace service must not hand-code SDK-owned API paths');
assert.doesNotMatch(workspaceServiceSource, /\b(Authorization|Access-Token|X-API-Key)\b/u, 'workspace service must not assemble auth headers manually');
assert.doesNotMatch(
  workspaceServiceSource,
  /\btenantId\s*:/u,
  'workspace service must not pass tenantId in SDK request objects; server context comes from AuthToken and Access-Token',
);
assert.match(
  chatLayoutSource,
  /<AddGroupMembersModal[\s\S]*?chat=\{activeChat\}[\s\S]*?onAdded=\{async\s*\(\)\s*=>\s*\{[\s\S]*?groupService\.getGroupById\s*\(/u,
  'chat layout add-member flow must render the contact-picker modal and refresh SDK-backed group projection after invites',
);
assert.match(
  openHydratedChatSource,
  /mergeChatIntoList\(previousChats,\s*chat\)[\s\S]*?setActiveTab\s*\(\s*["']chat["']\s*\)[\s\S]*?refreshChats\s*\(\s*\)/u,
  'chat layout backend-hydrated chat opening helper must optimistically merge the target chat, switch to chat tab, and refresh inbox projection from the server',
);
assert.match(
  chatLayoutSource,
  /<AddGroupMembersModal[\s\S]*?onAdded=\{async\s*\(\)\s*=>\s*\{[\s\S]*?setChats\s*\(\s*\(\s*previousChats\s*\)\s*=>\s*previousChats\.map\(/u,
  'chat layout add-member refresh must apply the refreshed group projection with a functional chat-list update',
);
assert.match(
  addGroupMembersModalSource,
  /contactService\.listContactsPage\s*\(\s*\)[\s\S]*?\.catch\s*\(/u,
  'group add-member modal must load selectable members from the address book and fail closed',
);
assert.match(
  addGroupMembersModalSource,
  /width=["']w-\[820px\]["'][\s\S]*height=["']h-\[740px\]["']/u,
  'group add-member modal must use a taller fixed height for equal-width member selection',
);
assert.match(
  addGroupMembersModalSource,
  /type\s+InviteMemberTab\s*=\s*['"]contacts['"]\s*\|\s*['"]strangers['"]/u,
  'group add-member modal must model address-book and stranger invite modes as explicit tabs',
);
assert.match(
  addGroupMembersModalSource,
  /const\s+\[activeTab,\s*setActiveTab\]\s*=\s*useState<InviteMemberTab>\(['"]contacts['"]\)/u,
  'group add-member modal must default to the address-book friends tab',
);
assert.match(
  addGroupMembersModalSource,
  /t\(['"]chat\.modal\.tabs\.contacts['"]\)[\s\S]*t\(['"]chat\.modal\.tabs\.strangers['"]\)/u,
  'group add-member modal must render separate tabs for address-book friends and strangers',
);
assert.match(
  addGroupMembersModalSource,
  /if\s*\(\s*activeTab\s*!==\s*['"]contacts['"]\s*\)\s*\{[\s\S]*?return;[\s\S]*?contactService\.listContactsPage/u,
  'group add-member modal must keep address-book contact loading inside the contacts tab flow',
);
assert.match(
  addGroupMembersModalSource,
  /<ContactMemberPickerPanel[\s\S]*contacts=\{contacts\}[\s\S]*disabledContactIds=\{disabledContactIds\}[\s\S]*disabledReason=\{t\(['"]chat\.modal\.selection\.alreadyInGroup['"]\)\}[\s\S]*selectedIds=\{selected\}[\s\S]*onToggleContact=\{toggleContact\}/u,
  'group add-member contacts tab must reuse the shared indexed contact picker while marking existing members disabled',
);
assert.match(
  addGroupMembersModalSource,
  /if\s*\(\s*activeTab\s*!==\s*['"]strangers['"][\s\S]*?return;[\s\S]*?contactService\.searchContacts\s*\(\s*nonContactSearchQuery/u,
  'group add-member modal must keep global stranger search inside the strangers tab flow',
);
assert.equal(
  chatEnUsMessages.chat?.modal?.tabs?.contacts,
  'Address book friends',
  'English chat modal messages must name the contacts add-member tab',
);
assert.equal(
  chatEnUsMessages.chat?.modal?.tabs?.strangers,
  'Strangers',
  'English chat modal messages must name the strangers add-member tab',
);
assert.equal(
  chatEnUsMessages.chat?.modal?.selection?.selectedTitle,
  'Selected',
  'English chat modal messages must name the selected contact column',
);
assert.equal(
  chatEnUsMessages.chat?.modal?.selection?.emptySelected,
  'Selected members will appear here',
  'English chat modal messages must describe the empty selected contact column',
);
assert.equal(
  chatEnUsMessages.chat?.modal?.selection?.alreadyInGroup,
  'Already in group',
  'English chat modal messages must label contacts who are already group members',
);
assert.equal(
  chatZhCnMessages.chat?.modal?.tabs?.contacts,
  '通讯录好友',
  'Chinese chat modal messages must name the contacts add-member tab',
);
assert.equal(
  chatZhCnMessages.chat?.modal?.tabs?.strangers,
  '陌生人',
  'Chinese chat modal messages must name the strangers add-member tab',
);
assert.equal(
  chatZhCnMessages.chat?.modal?.selection?.selectedTitle,
  '已选择',
  'Chinese chat modal messages must name the selected contact column',
);
assert.equal(
  chatZhCnMessages.chat?.modal?.selection?.emptySelected,
  '已勾选成员会显示在这里',
  'Chinese chat modal messages must describe the empty selected contact column',
);
assert.equal(
  chatZhCnMessages.chat?.modal?.selection?.alreadyInGroup,
  '已在群中',
  'Chinese chat modal messages must label contacts who are already group members',
);
assert.doesNotMatch(
  addGroupMembersModalSource,
  /<div\s+className=["'][^"']*border-t border-white\/10 pt-4["']/u,
  'group add-member modal must not append the stranger invite section below contacts in the same panel',
);
assert.match(
  addGroupMembersModalSource,
  /const\s+disabledContactIds\s*=\s*useMemo[\s\S]*isExistingGroupMember\(existingMemberIds,\s*contact\)[\s\S]*disabledIds\.add\(contact\.id\)/u,
  'group add-member modal must mark contacts who are already in the group instead of filtering them out',
);
assert.match(
  addGroupMembersModalSource,
  /function\s+isExistingGroupMember[\s\S]*existingMemberIds\.has\(contact\.id\)[\s\S]*existingMemberIds\.has\(contact\.chatId/u,
  'group add-member modal must treat contact id and chat id as member identifiers when marking existing group members',
);
assert.match(
  addGroupMembersModalSource,
  /groupService\.addMembers\s*\(\s*chat\.id\s*,\s*selectedInviteIds\s*\)/u,
  'group add-member modal must invite selected contact ids through GroupService.addMembers',
);
assert.match(
  addGroupMembersModalSource,
  /contactService\.searchContacts\s*\(\s*nonContactSearchQuery/u,
  'group add-member modal must offer an explicit non-contact search path for owner invitations',
);
assert.match(
  addGroupMembersModalSource,
  /groupService\.inviteUserToGroup\s*\(\s*chat,\s*selectedNonContactUser\s*\)/u,
  'group add-member modal must send owner non-contact invites through GroupService.inviteUserToGroup',
);
assert.match(
  addGroupMembersModalSource,
  /groupService\.addMembers\s*\(\s*chat\.id,\s*selectedInviteIds\s*\)[\s\S]*groupService\.inviteUserToGroup\s*\(\s*chat,\s*selectedNonContactUser\s*\)/u,
  'group add-member modal must keep address-book member add and non-contact card invite as separate explicit flows',
);
assert.match(
  chatRightPanelSource,
  /activeChat\.type\s*===\s*['"]group['"][\s\S]*activeChat\.members\?\.map/u,
  'chat right panel group management must render the backend-projected member list',
);
assert.match(
  chatRightPanelSource,
  /onRemoveGroupMember:\s*\(memberId:\s*string\)\s*=>\s*Promise<void>/u,
  'chat right panel group management must expose an async remove-member command controlled by the parent flow',
);
assert.match(
  chatRightPanelSource,
  /onRemoveGroupMember\(memberId\)/u,
  'chat right panel group management must invoke the parent remove-member command for selected group members',
);
assert.match(
  chatRightPanelSource,
  /groupMemberProfiles\?:\s*User\[\]/u,
  'chat right panel group management must accept address-book member projections from the parent flow',
);
assert.match(
  chatRightPanelSource,
  /memberProfilesById\.get\(memberId\)[\s\S]*memberProfile\?\.name\s*\?\?\s*fallbackMemberName/u,
  'chat right panel group management must render readable address-book member names before falling back to a safe localized placeholder',
);
assert.doesNotMatch(
  chatRightPanelSource,
  />\s*\{\s*memberId\s*\}\s*<\/span>/u,
  'chat right panel group management must not display only raw backend member ids when address-book projections are available',
);
assert.match(
  chatLayoutSource,
  /contactService\.getUserById\s*\(\s*memberId\s*\)[\s\S]*setGroupMemberProfiles/u,
  'chat layout must hydrate group member display profiles from contact projections for the right-panel member list',
);
assert.match(
  chatLayoutSource,
  /profilesById\.set\(profile\.id,\s*profile\)[\s\S]*profile\.chatId[\s\S]*profilesById\.set\(profile\.chatId,\s*profile\)/u,
  'chat layout group member profile hydration must index profiles by both user id and chat id before passing them to the right panel',
);
assert.match(
  chatLayoutSource,
  /currentUserChatId=\{currentUser\.chatId\}/u,
  'chat layout must pass current user chat id so the right panel can hide self-removal for either user id shape',
);
assert.doesNotMatch(
  addGroupMembersModalSource,
  /addMembersBySearchQuery|social\.users\.list/u,
  'group add-member modal must not resolve arbitrary text through user search',
);
assert.doesNotMatch(
  chatLayoutSource,
  /addMembersBySearchQuery\s*\(/u,
  'chat layout add-member flow must not use arbitrary user search for group invites',
);
assert.match(
  editNameModalSource,
  /activeChat\.type\s*===\s*["']group["'][\s\S]*?await\s+groupService\.updateGroupInfo\s*\(\s*activeChat\.id,\s*\{[\s\S]*?name:\s*modalInput[\s\S]*?\}/u,
  'chat layout group edit-name flow must await the SDK-backed group profile update before mutating local UI state',
);
assert.match(
  chatLayoutSource,
  /const\s+mergeGroupProfileUpdate\s*=\s*\(chat:\s*Chat,\s*update:\s*Chat\)[\s\S]*?update\.name[\s\S]*?update\.notice/u,
  'chat layout must merge group profile updates without overwriting unread counters, last messages, or timestamps',
);
assert.match(
  editNameModalSource,
  /activeChat\.type\s*===\s*["']group["'][\s\S]*?\?[\s\S]*?:\s*await\s+chatService\.updateChat\s*\(\s*activeChat\.id,\s*\{[\s\S]*?name:\s*modalInput[\s\S]*?\}/u,
  'chat layout direct-chat edit-name flow must keep using the SDK-backed chat profile update',
);
assert.match(
  editNoticeModalSource,
  /await\s+groupService\.updateGroupInfo\s*\(\s*activeChat\.id,\s*\{[\s\S]*?notice:\s*modalInput[\s\S]*?\}/u,
  'chat layout edit-notice flow must await the SDK-backed group notice update before mutating local UI state',
);
assert.doesNotMatch(
  editNoticeModalSource,
  /chatService\.updateChat\s*\(\s*activeChat\.id,\s*\{[\s\S]*?notice:/u,
  'chat layout group notice edit flow must not bypass GroupService group state projection',
);
assert.doesNotMatch(
  addGroupMembersModalSource,
  /groupService\.addMembers\s*\(\s*activeChat\.id\s*,\s*addedIds\s*\)/u,
  'chat layout add-member modal must not submit raw input as group member ids',
);
assert.doesNotMatch(
  addGroupMembersModalSource,
  /new\s+Set\s*\(\s*\[\s*\.\.\.\(\s*activeChat\.members\s*\|\|\s*\[\]\s*\)\s*,\s*\.\.\.addedIds\s*\]\s*\)/u,
  'chat layout add-member modal must not locally synthesize group members after SDK invites',
);
assert.doesNotMatch(
  addGroupMembersModalSource,
  /memberCount\s*:\s*updatedMembers\.length/u,
  'chat layout add-member modal must not locally synthesize group member counts after SDK invites',
);
assert.doesNotMatch(
  addGroupMembersModalSource,
  /chatService\.updateChat\s*\(\s*activeChat\.id\s*,\s*\{[\s\S]*?\bmembers\s*:/u,
  'chat layout add-member modal must not mutate group members through local chat view-state updates',
);
assert.doesNotMatch(
  chatRightPanelSource,
  /chatService\.(muteChat|pinChat|deleteChat)\s*\(/u,
  'chat right panel must delegate chat preference mutations to the parent so SDK success controls local state',
);
assert.match(
  chatRightPanelSource,
  /onToggleMute:\s*\(\)\s*=>\s*Promise<void>/u,
  'chat right panel must expose an async mute command controlled by the parent flow',
);
assert.match(
  chatRightPanelSource,
  /onTogglePin:\s*\(\)\s*=>\s*Promise<void>/u,
  'chat right panel must expose an async pin command controlled by the parent flow',
);
assert.match(
  chatRightPanelSource,
  /onDeleteChat:\s*\(\)\s*=>\s*Promise<void>/u,
  'chat right panel must expose an async delete command controlled by the parent flow',
);
assert.match(
  chatLayoutSource,
  /onToggleMute=\{async\s*\(\)\s*=>\s*\{[\s\S]*?await\s+chatService\.muteChat\s*\(/u,
  'chat layout right-panel mute flow must await the SDK mutation before local state changes',
);
assert.match(
  chatLayoutSource,
  /onTogglePin=\{async\s*\(\)\s*=>\s*\{[\s\S]*?await\s+chatService\.pinChat\s*\(/u,
  'chat layout right-panel pin flow must await the SDK mutation before local state changes',
);
assert.match(
  chatLayoutSource,
  /onDeleteChat=\{async\s*\(\)\s*=>\s*\{[\s\S]*?await\s+chatService\.deleteChat\s*\(/u,
  'chat layout right-panel direct-chat delete flow must await the SDK mutation before local state changes',
);
assert.match(
  chatLayoutSource,
  /activeChat\.type\s*===\s*["']group["'][\s\S]*?await\s+groupService\.deleteGroup\s*\(\s*activeChat\.id\s*\)/u,
  'chat layout right-panel group leave flow must leave the SDK-backed group instead of only hiding the conversation',
);
assert.match(
  chatLayoutSource,
  /onRemoveGroupMember=\{async\s*\(memberId\)\s*=>\s*\{[\s\S]*?await\s+groupService\.removeMember\s*\(\s*activeChat\.id,\s*memberId\s*\)[\s\S]*?groupService\.getGroupById\s*\(/u,
  'chat layout right-panel group member removal flow must await the SDK mutation and refresh group projection',
);
assert.match(
  chatLayoutSource,
  /onRemoveGroupMember=\{async\s*\(memberId\)\s*=>\s*\{[\s\S]*?setChats\s*\(\s*\(\s*previousChats\s*\)\s*=>\s*previousChats\.map\(/u,
  'chat layout right-panel group member removal flow must update the latest chat list state after the SDK refresh',
);
assert.match(
  editNameModalSource,
  /setChats\s*\(\s*\(\s*previousChats\s*\)\s*=>\s*previousChats\.map\(/u,
  'chat layout edit-name flow must merge the SDK profile result into the latest chat list state',
);
assert.match(
  editNoticeModalSource,
  /setChats\s*\(\s*\(\s*previousChats\s*\)\s*=>\s*previousChats\.map\(/u,
  'chat layout edit-notice flow must merge the SDK notice result into the latest chat list state',
);
assert.match(
  chatLayoutSource,
  /activeChat\.type\s*!==\s*["']group["'][\s\S]*?await\s+chatService\.deleteChat\s*\(\s*activeChat\.id\s*\)/u,
  'chat layout right-panel direct-chat delete flow must remain a hidden-conversation preference update',
);

const messageListSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/MessageList.tsx');
assert.match(
  messageListSource,
  /useTranslation\s*\(/u,
  'message list user-facing failures must use react-i18next',
);
assert.match(
  messageListSource,
  /chatService\.subscribeMessages|chatService\.subscribeConversationMessages/u,
  'message list must subscribe to SDK realtime updates',
);
assert.match(
  messageListSource,
  /const\s+loadNextMessagePage\s*=\s*useCallback/u,
  'message list must expose an explicit backward history-page load',
);
assert.match(
  messageListSource,
  /onClick=\{\(\)\s*=>\s*\{\s*void\s+loadNextMessagePage\s*\(\s*\)/u,
  'message list must expose an explicit action for opaque-cursor message history windows',
);
const messageListHandleScrollImplementation = messageListSource.match(
  /const\s+handleScroll\s*=\s*useCallback\(\(\)\s*=>\s*\{([\s\S]*?)\n\s*\},\s*\[[^\]]*\]\);/u,
)?.[1] ?? '';
assert.doesNotMatch(
  messageListHandleScrollImplementation,
  /loadNextMessagePage/u,
  'message list must not auto-load history pages from scroll position because that can silently aggregate long conversations',
);
assert.match(
  messageListSource,
  /return\s+\[\s*\.\.\.nextMessages\.filter\([\s\S]{0,180}?\.\.\.previous/u,
  'message list must prepend older history pages before the current chronological page',
);
assert.doesNotMatch(
  messageListSource,
  /return\s+\[\.\.\.previous,\s*\.\.\.nextMessages\.filter|element\.scrollTop\s*<\s*80[\s\S]*?load/u,
  'message list must not append older pages or auto-load unbounded history near the top',
);
assert.match(
  messageListSource,
  /previousScrollHeight[\s\S]{0,1000}?scrollTop\s*\+=\s*element\.scrollHeight\s*-\s*previousScrollHeight/u,
  'message list must preserve the viewport anchor after prepending an older page',
);
assert.equal(
  chatEnUsMessages.chat?.messageList?.loadMore,
  'Load more messages',
  'English chat message list catalog must include the explicit next-page action label',
);
assert.equal(
  chatEnUsMessages.chat?.messageList?.loadingMore,
  'Loading more messages...',
  'English chat message list catalog must include the explicit next-page loading state',
);
assert.equal(
  chatZhCnMessages.chat?.messageList?.loadMore,
  '\u52a0\u8f7d\u66f4\u591a\u6d88\u606f',
  'Chinese chat message list catalog must include the explicit next-page action label',
);
assert.equal(
  chatZhCnMessages.chat?.messageList?.loadingMore,
  '\u6b63\u5728\u52a0\u8f7d\u66f4\u591a\u6d88\u606f...',
  'Chinese chat message list catalog must include the explicit next-page loading state',
);
assert.doesNotMatch(
  messageListSource,
  /chat\.messageList\.(loadMore|loadingMore)['"][\s\S]{0,120}defaultValue/u,
  'message list must not rely on defaultValue fallback for committed catalog keys',
);
assert.match(
  subscribeChatsSource,
  /chatService\.subscribeChats\s*\(/u,
  'chat layout must subscribe to conversation-list realtime updates so unknown incoming conversations appear without opening them first',
);
assert.doesNotMatch(
  subscribeChatsSource,
  /mergeGroupProjections\s*\(\s*nextChats\s*\)/u,
  'chat layout realtime conversation-list refresh must not batch-hydrate group details for every list row',
);
assert.match(
  chatLayoutSource,
  /const\s+chatListProjectionRevisionRef\s*=\s*useRef\(0\)/u,
  'chat layout realtime conversation-list refresh must track group projection revisions so stale async projections cannot resurrect removed groups',
);
assert.match(
  subscribeChatsSource,
  /const\s+projectionRevision\s*=\s*chatListProjectionRevisionRef\.current\s*\+\s*1[\s\S]*chatListProjectionRevisionRef\.current\s*=\s*projectionRevision/u,
  'chat layout realtime conversation-list refresh must increment a projection revision for every SDK chat-list snapshot',
);
assert.doesNotMatch(
  subscribeChatsSource,
  /groupService\.getGroupById\s*\(/u,
  'chat layout realtime conversation-list refresh must not request group profile/member details until one group is explicitly opened',
);
assert.doesNotMatch(
  subscribeChatsSource,
  /new\s+Map\s*\(\s*previousChats\.map\(/u,
  'chat layout realtime conversation-list refresh must not retain stale chats that disappeared from the SDK inbox, such as groups left or removed on another device',
);
assert.match(
  subscribeChatsSource,
  /const\s+byId\s*=\s*new\s+Map\s*\(\s*sourceChats\.map\(/u,
  'chat layout realtime conversation-list refresh must treat the latest SDK chat list as authoritative before applying local preserved projections',
);
assert.match(
  subscribeChatsSource,
  /\?\?\s*systemAssistantService\.selectInitialChat\(/u,
  'chat layout realtime conversation-list refresh must clear or reselect active chat when the current group disappears from the SDK inbox',
);
assert.doesNotMatch(
  mergeGroupProjectionsSource,
  /groupService\.getGroupById\s*\(/u,
  'chat layout group projection merge helper must stay list-only and avoid per-group detail hydration',
);
assert.match(
  openHydratedChatSource,
  /mergeChatIntoList\(previousChats,\s*chat\)[\s\S]*?refreshChats\s*\(\s*\)/u,
  'chat layout backend-hydrated chat opening helper must merge hydrated conversations into the latest chat list state and refresh from the server',
);
assert.match(
  openHydratedChatSource,
  /setActiveChat\s*\(\s*\(\s*previousActiveChat\s*\)\s*=>[\s\S]*?setActiveTab\s*\(\s*["']chat["']\s*\)/u,
  'chat layout backend-hydrated chat opening helper must open the hydrated chat without discarding concurrent active-chat state for unrelated conversations',
);
assert.match(
  messageListSource,
  /chatService\.deleteMessage\s*\(\s*chatId\s*,\s*messageId\s*\)/u,
  'message list delete action must persist current-user delete through chatService',
);
assert.match(
  messageListSource,
  /favoriteService\.addFavorite\s*\(\s*\{[\s\S]*conversationId:\s*contextMenu\.msg\.chatId\s*\?\?\s*chatId/u,
  'message list favorite action must pass the backend conversation id into FavoriteService',
);
assert.match(
  messageListSource,
  /favoriteService\.addFavorite\s*\(\s*\{[\s\S]*messageId:\s*contextMenu\.msg\.id/u,
  'message list favorite action must pass the backend message id into FavoriteService',
);
assert.match(
  messageListSource,
  /parseGroupInviteDescriptor\s*\(\s*msg\s*\)/u,
  'message list must parse received group invite cards before invoking the open-group callback',
);
assert.match(
  messageListSource,
  /<CardMessageItem[\s\S]*?onClick=\{parseGroupInviteDescriptor\(msg\)\s*\?\s*\(\)\s*=>\s*\{[\s\S]*?handleGroupInviteClick\(msg\)[\s\S]*?:\s*undefined\}/u,
  'message list must make group invitation cards clickable and route them through the group invite handler',
);

const chatListSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ChatList.tsx');
assert.match(
  chatListSource,
  /chat\.isMarkedUnread/u,
  'chat list must render manual unread state from the SDK-backed conversation preferences view',
);
assert.match(
  chatListSource,
  /chatService\.markAsUnread\s*\(\s*chat\.id\s*\)/u,
  'chat list mark-unread action must persist through the chat service semantic SDK-backed method',
);
assert.doesNotMatch(
  chatListSource,
  /updateChat\s*\(\s*chat\.id\s*,\s*\{\s*unreadCount\s*:/u,
  'chat list must not simulate manual unread state through local unreadCount updates',
);
assert.doesNotMatch(
  chatListSource,
  /_forceUnread/u,
  'chat list must not keep private _forceUnread state once manual unread is SDK-backed',
);

const chatWindowSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ChatWindow.tsx');
assert.match(
  chatWindowSource,
  /onOpenGroupInvite\?:\s*\(groupId:\s*string\)\s*=>\s*Promise<void>/u,
  'chat window must accept a group-invite click handler from the chat layout',
);
assert.match(
  chatWindowSource,
  /<MessageList[\s\S]*?onOpenGroupInvite=\{onOpenGroupInvite\}/u,
  'chat window must pass group invitation card clicks down to the message list',
);
assert.match(
  chatLayoutSource,
  /<ChatWindow[\s\S]*?onOpenGroupInvite=\{handleOpenGroupInvite\}/u,
  'chat layout must wire group invitation card clicks into backend-hydrated group opening',
);
assert.match(
  chatLayoutSource,
  /const\s+handleOpenGroupInvite\s*=\s*async\s*\(groupId:\s*string\)[\s\S]*?groupService\.getGroupById\s*\([\s\S]*?openHydratedChat/u,
  'chat layout group invite click handler must hydrate SDK-backed groups and open the invited group chat',
);
assert.doesNotMatch(
  chatWindowSource,
  /fakeAgentResponses|Mock a streaming response feel|setTimeout\s*\(\s*async\s*\(\s*\)\s*=>/u,
  'chat window must not simulate received messages locally',
);

assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/CallService.ts');
const callServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/CallService.ts');
assertFile('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/RtcMediaService.ts');
const rtcMediaServiceSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/RtcMediaService.ts');
const imRealtimeSource = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/realtime.ts');
assert.doesNotMatch(callServiceSource, /@sdkwork\/rtc-sdk/u, 'call service must not import RTC SDK signaling or call-controller surfaces');
assert.doesNotMatch(
  callServiceSource,
  /createStandardRtcCallControllerStack/u,
  'call service must not compose IM calls through the RTC call controller stack',
);
assert.match(
  callServiceSource,
  /getImSdkClientWithSession/u,
  'call service must reuse the shared IM SDK client created from the IAM login session',
);
assert.match(callServiceSource, /startOutgoingCall/u, 'call service must expose outgoing call orchestration');
assert.match(callServiceSource, /recoverRtcSession/u, 'call service must expose RTC session recovery for backend state backfill');
assert.match(
  callServiceSource,
  /\.calls\.retrieve\s*\(\s*rtcSessionId\s*\)/u,
  'call service recoverRtcSession must read call session state through the composed IM SDK',
);
assert.match(
  callServiceSource,
  /toRecoveredServiceState/u,
  'call service must map backend RTC session state into PC call snapshot state',
);
assert.match(
  callServiceSource,
  /providerSessionId\s*\?\?\s*(?:rtcSession|session)\.rtcSessionId/u,
  'call service must recover provider room id from backend RTC session metadata',
);
assert.match(
  callServiceSource,
  /\.calls\.watchIncoming\s*\(/u,
  'call service must watch incoming call signaling through the composed IM SDK',
);
assert.match(
  imRealtimeSource,
  /onScope\s*\(/u,
  'IM realtime authored facade must expose generic scope event subscriptions for social/user realtime events',
);
assert.match(
  imRealtimeSource,
  /syncScopes\s*\(/u,
  'IM realtime authored facade must sync generic non-conversation scopes through the SDK boundary',
);
assert.match(
  imOpenApiSource,
  /RealtimeSubscriptionItemInput:[\s\S]*scopeType:[\s\S]*scopeId:[\s\S]*eventTypes:/u,
  'IM OpenAPI source contract must describe generic realtime subscription items instead of only conversation shortcuts',
);
assert.match(
  contactServiceSource,
  /subscribePcRealtimeScope\s*\([\s\S]*scopeType:\s*['"]user['"][\s\S]*eventTypes:\s*FRIEND_REQUEST_REALTIME_EVENT_TYPES/u,
  'contact service must subscribe to current-user realtime friend request events through the shared PC live connection manager',
);
assert.doesNotMatch(
  contactServiceSource,
  /connection\.events\.onScope\s*\(/u,
  'contact service must not wire user-scope friend request realtime events through raw IM live connection handlers',
);
assert.match(callServiceSource, /setAudioMuted/u, 'call service must expose audio mute through the RTC media client');
assert.match(callServiceSource, /setVideoMuted/u, 'call service must expose video mute through the RTC media client');
assert.match(
  callServiceSource,
  /rtcMediaService\.join/u,
  'call service must hand connected IM call sessions to an injected RTC media service instead of stopping at credential readiness',
);
assert.match(
  rtcMediaServiceSource,
  /@sdkwork\/rtc-sdk/u,
  'RTC media service must be the app-side boundary that consumes the provider-neutral RTC SDK',
);
assert.match(
  rtcMediaServiceSource,
  /installRtcProviderPackage/u,
  'RTC media service must install provider package boundaries through the standard RTC SDK loader SPI',
);
assert.match(
  rtcMediaServiceSource,
  /@sdkwork\/rtc-sdk-provider-volcengine/u,
  'RTC media service must load the Volcengine provider package through the package-boundary module',
);
assert.doesNotMatch(
  rtcMediaServiceSource,
  /createBuiltinRtcDriverManager/u,
  'RTC media service must not use the retired builtin driver manager surface',
);
assert.doesNotMatch(rtcMediaServiceSource, /\bfetch\s*\(/u, 'RTC media service must not use raw fetch');
assert.doesNotMatch(rtcMediaServiceSource, /\/im\/v3/u, 'RTC media service must not hand-code IM HTTP paths');
assert.doesNotMatch(
  rtcMediaServiceSource,
  /\b(Authorization|Access-Token|X-API-Key)\b/u,
  'RTC media service must not assemble auth headers manually',
);
assert.match(callServiceSource, /endCall/u, 'call service must expose SDK-backed call termination');
assert.doesNotMatch(callServiceSource, /\bfetch\s*\(/u, 'call service must not use raw fetch');
assert.doesNotMatch(callServiceSource, /\/im\/v3/u, 'call service must not hand-code IM HTTP paths');
assert.doesNotMatch(callServiceSource, /\b(Authorization|Access-Token|X-API-Key)\b/u, 'call service must not assemble auth headers manually');

const callOverlaySource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/CallOverlay.tsx');
assert.match(callOverlaySource, /callService/u, 'call overlay must delegate RTC behavior to the call service');
assert.match(callOverlaySource, /startOutgoingCall/u, 'call overlay must start calls through the SDK-backed call service');
assert.match(callOverlaySource, /setAudioMuted/u, 'call overlay must mute audio through the SDK-backed call service');
assert.match(callOverlaySource, /setVideoMuted/u, 'call overlay must mute video through the SDK-backed call service');
assert.match(
  callOverlaySource,
  /bindLocalVideoElement/u,
  'call overlay must bind provider-owned local video through the SDK-backed call service instead of browser getUserMedia',
);
assert.doesNotMatch(
  callOverlaySource,
  /getUserMedia\s*\(/u,
  'call overlay must not directly capture camera or microphone while the RTC provider owns call media capture',
);
assert.match(callOverlaySource, /endCall/u, 'call overlay must end calls through the SDK-backed call service');
assert.doesNotMatch(
  callOverlaySource,
  /setTimeout\s*\(\s*\(\s*\)\s*=>\s*\{?\s*setCallState\s*\(\s*['"]connected['"]/u,
  'call overlay must not simulate successful RTC connection with a timeout',
);
assert.match(
  chatLayoutSource,
  /conversationId=\{callTarget\.id\}/u,
  'chat layout must pass the active conversation id into CallOverlay for RTC session creation',
);
assert.match(
  chatLayoutSource,
  /resolveIncomingCallWatchConversationIds/u,
  'chat layout must watch incoming RTC calls for both hydrated chats and projected direct-contact conversations',
);
assert.match(
  chatLayoutSource,
  /SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT/u,
  'chat layout must refresh chat projections after friend/contact changes',
);
assert.match(
  chatServiceSource,
  /resolveIncomingCallWatchConversationIds/u,
  'chat service must expose incoming call watch conversation id resolution for contact-only direct chats',
);
assert.match(
  chatServiceSource,
  /resolveIncomingCallWatchConversationIds/u,
  'chat service must expose incoming call watch conversation id resolution without client-side id projection',
);

const serviceFiles = listFiles('apps/sdkwork-im-pc/packages', (candidate) => {
  const normalized = candidate.replaceAll('\\', '/');
  return normalized.includes('/src/services/') && /\.(ts|tsx)$/.test(normalized);
});
for (const filePath of serviceFiles) {
  const source = fs.readFileSync(filePath, 'utf8');
  assert.doesNotMatch(source, /\/im\/v3/u, `${path.relative(repoRoot, filePath)} must not hand-code IM HTTP paths`);
  assert.doesNotMatch(source, /\b(Authorization|Access-Token|X-API-Key)\b/u, `${path.relative(repoRoot, filePath)} must not assemble auth headers manually`);
}

console.log('sdkwork-im-pc SDK integration contract passed');
