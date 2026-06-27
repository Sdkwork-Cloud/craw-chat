import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');
const workspaceRoot = path.resolve(repoRoot, '..');
const rtcSdkRoot = path.join(
  workspaceRoot,
  'sdkwork-rtc',
  'sdks',
  'sdkwork-rtc-sdk',
  'sdkwork-rtc-sdk-typescript',
);
const rtcSdkFamilyRoot = path.join(workspaceRoot, 'sdkwork-rtc', 'sdks', 'sdkwork-rtc-sdk');
const rtcSignalingImportPattern =
  /sdkwork_(?:rtc_core|communication_rtc_service)::\{[^}]*\b(?:RtcSession|RtcSessionState|RtcSignalEvent|RtcSignalSender|RtcStateRecord|RtcStateStore)\b|sdkwork_(?:rtc_core|communication_rtc_service)::(?:RtcSession|RtcSessionState|RtcSignalEvent|RtcSignalSender|RtcStateRecord|RtcStateStore)|sdkwork_ai_prod_state_store/u;

function readFile(absolutePath) {
  return fs.readFileSync(absolutePath, 'utf8');
}

function read(relativePath) {
  return readFile(path.join(repoRoot, relativePath));
}

function listFiles(root, predicate = () => true) {
  const files = [];
  const walk = (dir) => {
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      const fullPath = path.join(dir, entry.name);
      if (entry.isDirectory()) {
        if (entry.name === 'dist' || entry.name === 'node_modules' || entry.name === 'target') {
          continue;
        }
        walk(fullPath);
        continue;
      }
      if (predicate(fullPath)) {
        files.push(fullPath);
      }
    }
  };
  walk(root);
  return files;
}

function relativeToWorkspace(absolutePath) {
  return path.relative(workspaceRoot, absolutePath).replaceAll('\\', '/');
}

const callServicePath = 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/CallService.ts';
const callServiceSource = read(callServicePath);

assert.doesNotMatch(
  callServiceSource,
  /from\s+['"]@sdkwork\/rtc-sdk['"]/u,
  'PC CallService must not import @sdkwork/rtc-sdk signaling or call-controller surfaces directly',
);
assert.doesNotMatch(
  callServiceSource,
  /createRtcAppHttpClient|createStandardRtcCallControllerStack|RtcSignalingTransportLike|RtcCallControllerSnapshot|RtcDataSourceConfig/u,
  'PC CallService must not construct RTC signaling transports or RTC call-controller stacks',
);
assert.doesNotMatch(
  callServiceSource,
  /\b(Authorization|Access-Token|X-API-Key)\b/u,
  'PC CallService must not assemble SDKWork auth headers manually',
);
assert.match(
  callServiceSource,
  /\.calls\.(?:retrieve|start|watchIncoming|accept|reject|end|subscribe)\b/u,
  'PC CallService must orchestrate call signaling through the IM SDK calls facade',
);

const imSdkSource = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/sdk.ts');
const imIndexSource = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/index.ts');
assert.match(imSdkSource, /readonly\s+calls:\s+ImCallsModule/u, 'IM SDK client must expose a calls facade');
assert.match(imSdkSource, /this\.calls\s*=\s*new\s+ImCallsModule/u, 'IM SDK client must construct the calls facade');
assert.doesNotMatch(imSdkSource, /readonly\s+rtc:\s+ImRtcModule/u, 'IM SDK client must not expose legacy rtc facade');
assert.doesNotMatch(imIndexSource, /rtc-module/u, 'IM SDK public exports must not expose legacy rtc-module');
assert.match(imIndexSource, /calls-module/u, 'IM SDK public exports must expose calls-module');

const rtcIndexPath = path.join(rtcSdkRoot, 'src', 'index.ts');
const rtcIndexSource = readFile(rtcIndexPath);
for (const forbiddenExport of [
  './signaling-transport.js',
  './call-types.js',
  './call-controller.js',
  './call-session.js',
  './signaling-adapter.js',
  './standard-call-stack.js',
  './app-http-client.js',
]) {
  assert.doesNotMatch(
    rtcIndexSource,
    new RegExp(forbiddenExport.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'),
    `@sdkwork/rtc-sdk root export must not expose IM call signaling surface ${forbiddenExport}`,
  );
}

const rtcProductionSources = listFiles(path.join(rtcSdkRoot, 'src'), (filePath) => filePath.endsWith('.ts'));
for (const filePath of rtcProductionSources) {
  const relative = relativeToWorkspace(filePath);
  const source = readFile(filePath);
  assert.doesNotMatch(
    source,
    /\/app\/v3\/api\/rtc|Authorization|Access-Token|X-API-Key/u,
    `${relative} must not contain app-api RTC signaling HTTP or manual credential handling`,
  );
  assert.doesNotMatch(
    source,
    /createRtcAppHttpClient|RtcSignalingTransportLike|RtcCallSignalingAdapter|StandardRtcCallController|createStandardRtcCallControllerStack/u,
    `${relative} must not keep call signaling/controller implementation in sdkwork-rtc`,
  );
}

const rtcReadmeSource = readFile(path.join(rtcSdkRoot, 'README.md'));
assert.doesNotMatch(
  rtcReadmeSource,
  /signaling|StandardRtcCallController|createStandardRtcCallControllerStack|sdk-call-smoke/u,
  'RTC SDK README must describe provider/media runtime capability, not IM signaling or call orchestration',
);

for (const relativePath of [
  '.sdkwork-assembly.json',
  'bin/rtc-standard-contract-constants.mjs',
  'bin/rtc-standard-assembly-baseline.mjs',
  'bin/materialize-sdk.mjs',
]) {
  const absolutePath = path.join(rtcSdkFamilyRoot, relativePath);
  const source = readFile(absolutePath);
  assert.doesNotMatch(
    source,
    /signalingTransportStandard|RTC_SIGNALING|createStandardRtcCallControllerStack|sdk-call-smoke|call-controller|call-session|app-http-client|signalingSdkPackage|signalingSdkImportPath/u,
    `RTC SDK generation source ${relativeToWorkspace(absolutePath)} must not regenerate IM call signaling debt`,
  );
}

const webGatewaySource = read('services/sdkwork-im-cloud-gateway/src/lib.rs');
assert.doesNotMatch(
  webGatewaySource,
  /sdkwork-rtc-signaling-service|\/app\/v3\/api\/rtc/u,
  'web-gateway must not classify call signaling as an sdkwork-rtc app-api target',
);

for (const relativePath of ['Cargo.toml', 'services/sdkwork-im-cloud-gateway/Cargo.toml']) {
  const source = read(relativePath);
  assert.doesNotMatch(
    source,
    /sdkwork-rtc-signaling-service/u,
    `${relativePath} must not depend on the retired RTC-owned signaling service`,
  );
}

const webGatewayCallsSource = read('services/sdkwork-im-cloud-gateway/src/lib.rs');
assert.match(
  webGatewayCallsSource,
  /im-calls-service/u,
  'sdkwork-im-cloud-gateway must route IM-owned call signaling through im-calls-service upstream',
);

for (const relativePath of [
  'crates/im-domain-core/src/rtc.rs',
  'crates/im-platform-contracts/src/lib.rs',
]) {
  const source = read(relativePath);
  assert.doesNotMatch(
    source,
    rtcSignalingImportPattern,
    `${relativePath} must source IM call signaling DTOs/state stores from im-domain-core or local IM adapters, not sdkwork-rtc`,
  );
}

for (const relativePath of ['README.md', 'specs/database-prefix-registry.json', 'specs/database-table-registry.json']) {
  const source = read(relativePath);
  assert.doesNotMatch(
    source,
    /sdkwork-rtc-signaling-service/u,
    `${relativePath} must not keep the retired RTC-owned signaling service as an active owner`,
  );
}

const tableRegistry = JSON.parse(read('specs/database-table-registry.json'));
for (const tableName of ['im_rtc_sessions', 'im_rtc_signals']) {
  const entry = tableRegistry.tables.find((table) => table.tableName === tableName);
  assert.ok(entry, `${tableName} must remain registered as an IM-owned table`);
  assert.equal(
    entry.writeOwner,
    'im-call-runtime',
    `${tableName} must be written by the IM-owned call runtime`,
  );
}

const activeDocsAndSdkDocs = [
  'docs/sites',
  'sdks/sdkwork-im-app-sdk/README.md',
].flatMap((relativePath) => {
  const absolutePath = path.join(repoRoot, relativePath);
  if (fs.statSync(absolutePath).isDirectory()) {
    return listFiles(absolutePath, (filePath) => /\.(?:md|mjs)$/u.test(filePath));
  }
  return [absolutePath];
});
for (const absolutePath of activeDocsAndSdkDocs) {
  const relative = path.relative(repoRoot, absolutePath).replaceAll('\\', '/');
  const source = readFile(absolutePath);
  assert.doesNotMatch(
    source,
    /\/app\/v3\/api\/rtc\/provider_(?:health|callbacks)|RTC provider health\/callback app APIs/u,
    `${relative} must not publish retired RTC app-api provider routes`,
  );
}

const historicalDocs = [
  'docs/架构',
  'docs/review',
  'docs/superpowers',
  'docs/step',
].flatMap((relativePath) => listFiles(
  path.join(repoRoot, relativePath),
  (filePath) => filePath.endsWith('.md'),
));
for (const absolutePath of historicalDocs) {
  const relative = path.relative(repoRoot, absolutePath).replaceAll('\\', '/');
  const source = readFile(absolutePath);
  assert.doesNotMatch(
    source,
    /\/im\/v3\/api\/rtc|sdk\.rtc|sdkwork-rtc-signaling-service|rtc-signaling-service|createStandardRtcCallControllerStack|sdk-call-smoke/u,
    `${relative} must not keep superseded IM call signaling guidance`,
  );
}

console.log('sdkwork IM/RTC signaling boundary contract passed');
