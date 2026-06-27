import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function repoPath(relativePath) {
  return path.join(repoRoot, relativePath);
}

function read(relativePath) {
  return fs.readFileSync(repoPath(relativePath), 'utf8');
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function assertFile(relativePath) {
  assert.ok(fs.existsSync(repoPath(relativePath)), `${relativePath} must exist.`);
}

function listFiles(relativeRoot) {
  return fs.readdirSync(repoPath(relativeRoot), { recursive: true })
    .filter((entry) => typeof entry === 'string')
    .map((entry) => path.join(relativeRoot, entry))
    .filter((entryPath) => fs.statSync(repoPath(entryPath)).isFile());
}

function isRpcGeneratedTextSource(relativePath) {
  return ['.go', '.java', '.proto', '.py', '.rs', '.ts'].includes(path.extname(relativePath));
}

function escapeRegex(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function firstCapture(relativePath, pattern, description) {
  const match = read(relativePath).match(pattern);
  assert.ok(match, `${relativePath} must declare ${description}.`);
  return match[1];
}

function parseProtoMethods(relativePath) {
  const source = read(relativePath);
  const services = new Map();
  const servicePattern = /service\s+([A-Za-z0-9_]+)\s*\{([\s\S]*?)\n\}/gu;
  for (const serviceMatch of source.matchAll(servicePattern)) {
    const serviceName = serviceMatch[1];
    const body = serviceMatch[2];
    const methods = [...body.matchAll(/rpc\s+([A-Za-z0-9_]+)\s*\(/gu)].map((match) => match[1]);
    services.set(serviceName, methods);
  }
  return services;
}

const requiredFiles = [
  'apis/rpc/buf.yaml',
  'apis/rpc/sdkwork/common/v1/context.proto',
  'apis/rpc/sdkwork/common/v1/media.proto',
  'apis/rpc/sdkwork/communication/app/v3/conversation_service.proto',
  'apis/rpc/sdkwork/communication/app/v3/message_service.proto',
  'apis/rpc/sdkwork/communication/app/v3/room_service.proto',
  'apis/rpc/sdkwork/communication/app/v3/realtime_service.proto',
  'apis/rpc/sdkwork/communication/app/v3/social_service.proto',
  'apis/rpc/sdkwork/communication/app/v3/stream_service.proto',
  'apis/rpc/sdkwork/communication/app/v3/call_service.proto',
  'apis/rpc/sdkwork/communication/app/v3/notification_service.proto',
  'apis/rpc/sdkwork/communication/app/v3/automation_service.proto',
  'apis/rpc/sdkwork/communication/backend/v3/admin_service.proto',
  'apis/rpc/sdkwork/communication/internal/v1/distributed_runtime_service.proto',
  'apis/rpc/sdkwork/communication/internal/v1/message_dispatch_service.proto',
  'apis/rpc/sdkwork/communication/internal/v1/room_orchestration_service.proto',
  'sdks/sdkwork-im-rpc-sdk/.sdkwork-assembly.json',
  'sdks/sdkwork-im-rpc-sdk/README.md',
  'sdks/sdkwork-im-rpc-sdk/rpc/sdkwork-im-rpc.manifest.json',
  'sdks/sdkwork-im-rpc-sdk/specs/README.md',
  'sdks/sdkwork-im-rpc-sdk/specs/component.spec.json',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-typescript/buf.gen.yaml',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-typescript/package.json',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-typescript/rpc-methods.json',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-typescript/src/index.ts',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-typescript/generated/proto/sdkwork/communication/app/v3/message_service_pb.ts',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-typescript/generated/proto/sdkwork/communication/app/v3/message_service_connect.ts',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-typescript/generated/proto/sdkwork/communication/backend/v3/admin_service_pb.ts',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-typescript/generated/proto/sdkwork/communication/internal/v1/distributed_runtime_service_connect.ts',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-go/buf.gen.yaml',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-go/go.mod',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-go/rpc-methods.json',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-go/sdk.go',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-go/generated/proto/sdkwork/communication/app/v3/message_service.pb.go',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-go/generated/proto/sdkwork/communication/app/v3/message_service_grpc.pb.go',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-go/generated/proto/sdkwork/communication/backend/v3/admin_service_grpc.pb.go',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-java/buf.gen.yaml',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-java/pom.xml',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-java/rpc-methods.json',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-java/src/main/java/com/sdkwork/im/rpc/SdkworkImRpc.java',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-java/generated/proto/java/com/sdkwork/communication/app/v3/MessageServiceGrpc.java',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-java/generated/proto/java/com/sdkwork/communication/backend/v3/CommunicationOpsServiceGrpc.java',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-python/buf.gen.yaml',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-python/pyproject.toml',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-python/rpc-methods.json',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-python/src/sdkwork_im_rpc_sdk/__init__.py',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-python/generated/proto/sdkwork/communication/app/v3/message_service_pb2.py',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-python/generated/proto/sdkwork/communication/app/v3/message_service_pb2_grpc.py',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-python/generated/proto/sdkwork/communication/internal/v1/distributed_runtime_service_pb2_grpc.py',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-rust/buf.gen.yaml',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-rust/Cargo.toml',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-rust/rpc-methods.json',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-rust/src/lib.rs',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-rust/generated/proto/sdkwork/communication/app/v3/sdkwork.communication.app.v3.rs',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-rust/generated/proto/sdkwork/communication/app/v3/sdkwork.communication.app.v3.tonic.rs',
  'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-rust/generated/proto/sdkwork/communication/internal/v1/sdkwork.communication.internal.v1.rs',
  'crates/sdkwork-im-rpc-service-rust/Cargo.toml',
  'crates/sdkwork-im-rpc-service-rust/src/lib.rs',
  'crates/sdkwork-im-rpc-service-rust/src/service_manifest.rs',
  'crates/sdkwork-im-rpc-service-rust/src/service_binding.rs',
  'crates/sdkwork-im-rpc-service-rust/tests/rpc_service_manifest_contract_test.rs',
  'services/session-gateway-rpc-bin/Cargo.toml',
  'services/session-gateway-rpc-bin/README.md',
  'services/session-gateway-rpc-bin/src/main.rs',
];

for (const relativePath of requiredFiles) {
  assertFile(relativePath);
}

const assembly = readJson('sdks/sdkwork-im-rpc-sdk/.sdkwork-assembly.json');
assert.equal(assembly.workspace, 'sdkwork-im-rpc-sdk');
assert.equal(assembly.domain, 'communication');
assert.equal(assembly.capability, 'chat');
assert.deepEqual(assembly.discoverySurface.generatedProtocols, ['rpc']);
assert.equal(assembly.discoverySurface.protoRoot, '../../apis/rpc');
assert.equal(assembly.rpcManifest, 'rpc/sdkwork-im-rpc.manifest.json');
assert.equal(assembly.httpFamilyMapping.openApiSdkFamily, 'sdkwork-im-sdk');
assert.equal(assembly.httpFamilyMapping.appApiSdkFamily, 'sdkwork-im-app-sdk');
assert.equal(assembly.httpFamilyMapping.backendApiSdkFamily, 'sdkwork-im-backend-sdk');
assert.deepEqual(
  assembly.inspectionPolicy,
  {
    mode: 'convention',
    protocol: 'rpc',
    optionalControlPlane: {
      emitFlag: '--emit-control-plane',
      purpose: ['release', 'ci', 'audit', 'migration'],
    },
  },
  'assembly must declare the optional RPC control-plane policy once at the family level.',
);
assert.doesNotMatch(
  read('sdks/sdkwork-im-rpc-sdk/.sdkwork-assembly.json'),
  /sdkwork-generator-(manifest|changes|report)\.json/u,
  'assembly must not duplicate derived optional control-plane file names per language.',
);

const expectedRpcLanguages = ['go', 'java', 'python', 'rust', 'typescript'];
assert.deepEqual(
  assembly.languages.map((entry) => entry.language).sort(),
  expectedRpcLanguages,
  'RPC SDK assembly must declare every generated baseline language.',
);
for (const language of expectedRpcLanguages) {
  const entry = assembly.languages.find((candidate) => candidate.language === language);
  assert.ok(entry, `assembly must declare ${language}.`);
  assert.equal(entry.generationState, 'generated', `${language} generation state must be generated.`);
  assert.equal(entry.releaseState, 'not_published', `${language} release state must remain not_published.`);
  assert.match(entry.workspace, new RegExp(`^sdkwork-im-rpc-sdk-${language}$`, 'u'), `${language} workspace must use RPC SDK family naming.`);
  assert.match(entry.generatedPath, new RegExp(`^sdkwork-im-rpc-sdk-${language}/generated/proto`, 'u'), `${language} generated path must point at generated/proto.`);
  assert.ok(entry.packageName, `${language} must declare generator packageName.`);
  assert.ok(entry.verifyCommand, `${language} must declare compile verification command.`);
  assert.equal(entry.inspection.mode, 'convention', `${language} assembly entry must use convention inspection mode.`);
  assert.equal(entry.inspection.protocol, 'rpc', `${language} assembly entry must declare RPC inspection protocol.`);
  assert.equal(
    entry.inspection.inspectCommand,
    `node ../sdkwork-sdk-generator/bin/sdkgen.js inspect --protocol rpc --output sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-${language} --json`,
    `${language} assembly entry must declare the RPC inspect command.`,
  );
  assert.ok(
    entry.inspection.requiredEvidence.includes('../rpc/sdkwork-im-rpc.manifest.json'),
    `${language} assembly entry must declare the RPC manifest as convention evidence.`,
  );
  assert.equal(
    Object.hasOwn(entry.inspection, 'optionalControlPlane'),
    false,
    `${language} assembly entry must not duplicate optional control-plane policy.`,
  );
}

const packageVersionByLanguage = {
  typescript: readJson('sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-typescript/package.json').version,
  java: firstCapture(
    'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-java/pom.xml',
    /<project[\s\S]*?<version>([^<]+)<\/version>/u,
    'a project version',
  ),
  python: firstCapture(
    'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-python/pyproject.toml',
    /^version = "([^"]+)"/mu,
    'a project version',
  ),
  rust: firstCapture(
    'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-rust/Cargo.toml',
    /^version = "([^"]+)"/mu,
    'a crate version',
  ),
};

const typeScriptPackageLock = readJson('sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-typescript/package-lock.json');
assert.equal(
  typeScriptPackageLock.version,
  packageVersionByLanguage.typescript,
  'TypeScript RPC SDK package-lock version must match package.json.',
);
assert.equal(
  typeScriptPackageLock.packages[''].version,
  packageVersionByLanguage.typescript,
  'TypeScript RPC SDK package-lock root package version must match package.json.',
);

for (const [language, packageVersion] of Object.entries(packageVersionByLanguage)) {
  const entry = assembly.languages.find((candidate) => candidate.language === language);
  assert.equal(
    entry.version,
    packageVersion,
    `${language} assembly version must match the generated package manifest version.`,
  );
}

const componentSpec = readJson('sdks/sdkwork-im-rpc-sdk/specs/component.spec.json');
assert.equal(componentSpec.component.name, 'sdkwork-im-rpc-sdk');
assert.equal(componentSpec.component.domain, 'communication');
assert.deepEqual(
  componentSpec.component.languages.sort(),
  expectedRpcLanguages,
  'component spec must declare every generated baseline RPC SDK language.',
);
assert.ok(
  componentSpec.canonicalSpecs.some((entry) => entry.file === 'RPC_SPEC.md'),
  'RPC SDK component spec must cite RPC_SPEC.md.',
);
assert.ok(
  componentSpec.canonicalSpecs.some((entry) => entry.file === 'RPC_SDK_WORKSPACE_SPEC.md'),
  'RPC SDK component spec must cite RPC_SDK_WORKSPACE_SPEC.md.',
);
assert.equal(
  componentSpec.contracts.rpcManifest,
  'rpc/sdkwork-im-rpc.manifest.json',
  'component spec must point to the RPC manifest.',
);
assert.deepEqual(componentSpec.contracts.sdkDependencies, []);
assert.deepEqual(
  componentSpec.contracts.inspectionPolicy,
  {
    mode: 'convention',
    protocol: 'rpc',
    optionalControlPlane: {
      emitFlag: '--emit-control-plane',
      purpose: ['release', 'ci', 'audit', 'migration'],
    },
  },
  'component spec must declare the optional RPC control-plane policy once at the family level.',
);
assert.doesNotMatch(
  read('sdks/sdkwork-im-rpc-sdk/specs/component.spec.json'),
  /sdkwork-generator-(manifest|changes|report)\.json/u,
  'component spec must not duplicate derived optional control-plane file names per language.',
);
assert.deepEqual(
  componentSpec.contracts.generatedSdkWorkspaces.map((entry) => entry.language).sort(),
  expectedRpcLanguages,
  'component spec must list every generated RPC SDK workspace.',
);
for (const language of expectedRpcLanguages) {
  const entry = componentSpec.contracts.generatedSdkWorkspaces.find((candidate) => candidate.language === language);
  assert.ok(entry, `component spec must list ${language} generated workspace.`);
  assert.match(entry.workspace, new RegExp(`^sdkwork-im-rpc-sdk-${language}$`, 'u'));
  assert.ok(entry.packageName, `${language} generated workspace must declare packageName.`);
  assert.ok(entry.bufTemplate, `${language} generated workspace must declare bufTemplate.`);
  assert.equal(entry.inspection.mode, 'convention', `${language} component spec entry must use convention inspection mode.`);
  assert.equal(entry.inspection.protocol, 'rpc', `${language} component spec entry must declare RPC inspection protocol.`);
  assert.equal(
    entry.inspection.inspectCommand,
    `node ../sdkwork-sdk-generator/bin/sdkgen.js inspect --protocol rpc --output sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-${language} --json`,
    `${language} component spec entry must declare RPC inspect command.`,
  );
  assert.ok(
    entry.inspection.requiredEvidence.includes('../rpc/sdkwork-im-rpc.manifest.json'),
    `${language} component spec entry must declare the RPC manifest as convention evidence.`,
  );
  assert.equal(
    Object.hasOwn(entry.inspection, 'optionalControlPlane'),
    false,
    `${language} component spec entry must not duplicate optional control-plane policy.`,
  );
}
for (const language of expectedRpcLanguages) {
  assert.ok(
    componentSpec.verification.commands.some((command) => (
      command.includes('sdkgen.js inspect --protocol rpc')
      && command.includes(`sdkwork-im-rpc-sdk-${language}`)
    )),
    `component spec verification must include sdkgen inspect --protocol rpc for ${language}.`,
  );
}

const manifest = readJson('sdks/sdkwork-im-rpc-sdk/rpc/sdkwork-im-rpc.manifest.json');
assert.equal(manifest.schemaVersion, 1);
assert.equal(manifest.kind, 'sdkwork.rpc.manifest');
assert.equal(manifest.domain, 'communication');
assert.equal(manifest.sdkFamily, 'sdkwork-im-rpc-sdk');

for (const relativePath of [
  ...listFiles('apis/rpc').filter((entryPath) => entryPath.endsWith('.proto')),
  ...listFiles('sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-typescript/generated/proto').filter(isRpcGeneratedTextSource),
  ...listFiles('sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-go/generated/proto').filter(isRpcGeneratedTextSource),
  ...listFiles('sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-java/generated/proto').filter(isRpcGeneratedTextSource),
  ...listFiles('sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-python/generated/proto').filter(isRpcGeneratedTextSource),
  ...listFiles('sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-rust/generated/proto').filter(isRpcGeneratedTextSource),
]) {
  const source = read(relativePath);
  assert.doesNotMatch(
    source,
    /communication-rpc-go/u,
    `${relativePath} must not reference the legacy communication RPC Go package family.`,
  );
}

const expectedServices = {
  'sdkwork.communication.app.v3': [
    'PresenceService',
    'RealtimeService',
    'ConversationService',
    'MessageService',
    'RoomService',
    'ContactService',
    'SocialService',
    'StreamService',
    'CallService',
    'NotificationService',
    'AutomationService',
  ],
  'sdkwork.communication.backend.v3': [
    'CommunicationOpsService',
    'RealtimeNodeAdminService',
    'CommunicationControlService',
    'SocialAdminService',
    'SocialRuntimeAdminService',
    'AuditAdminService',
  ],
  'sdkwork.communication.internal.v1': [
    'RuntimeTopologyService',
    'RouteLeaseService',
    'DomainEventRelayService',
    'RoomOrchestrationService',
    'MessageDispatchService',
  ],
};

const serviceKeys = new Set();
const manifestMethods = new Map();
for (const service of manifest.services) {
  assert.ok(expectedServices[service.package]?.includes(service.service), `${service.package}.${service.service} is not in the approved service catalog.`);
  serviceKeys.add(`${service.package}.${service.service}`);
  assert.ok(['app', 'backend', 'internal'].includes(service.surface), `${service.service} must use a standard surface.`);
  for (const method of service.methods) {
    const key = `${service.package}.${service.service}.${method.method}`;
    assert.ok(!manifestMethods.has(key), `${key} must be unique.`);
    manifestMethods.set(key, method);
    assert.match(method.operationId, /^[a-z][A-Za-z0-9]*(?:\.[a-z][A-Za-z0-9]*)+$/u, `${key} must have a dotted operationId.`);
    assert.ok(['none', 'optional', 'required'].includes(method.idempotency), `${key} must declare idempotency.`);
    assert.ok(['unary', 'server', 'client', 'bidi'].includes(method.streaming), `${key} must declare streaming mode.`);
    assert.ok(method.auth, `${key} must declare auth.`);
    assert.ok(method.owner, `${key} must declare owner.`);
    assert.ok(method.compatibility, `${key} must declare compatibility.`);
  }
}

for (const [packageName, services] of Object.entries(expectedServices)) {
  for (const service of services) {
    assert.ok(serviceKeys.has(`${packageName}.${service}`), `manifest must include ${packageName}.${service}.`);
  }
}

for (const [relativePath, packageName] of [
  ['apis/rpc/sdkwork/communication/app/v3/conversation_service.proto', 'sdkwork.communication.app.v3'],
  ['apis/rpc/sdkwork/communication/app/v3/message_service.proto', 'sdkwork.communication.app.v3'],
  ['apis/rpc/sdkwork/communication/app/v3/room_service.proto', 'sdkwork.communication.app.v3'],
  ['apis/rpc/sdkwork/communication/app/v3/realtime_service.proto', 'sdkwork.communication.app.v3'],
  ['apis/rpc/sdkwork/communication/app/v3/social_service.proto', 'sdkwork.communication.app.v3'],
  ['apis/rpc/sdkwork/communication/app/v3/stream_service.proto', 'sdkwork.communication.app.v3'],
  ['apis/rpc/sdkwork/communication/app/v3/call_service.proto', 'sdkwork.communication.app.v3'],
  ['apis/rpc/sdkwork/communication/app/v3/notification_service.proto', 'sdkwork.communication.app.v3'],
  ['apis/rpc/sdkwork/communication/app/v3/automation_service.proto', 'sdkwork.communication.app.v3'],
  ['apis/rpc/sdkwork/communication/backend/v3/admin_service.proto', 'sdkwork.communication.backend.v3'],
  ['apis/rpc/sdkwork/communication/internal/v1/distributed_runtime_service.proto', 'sdkwork.communication.internal.v1'],
  ['apis/rpc/sdkwork/communication/internal/v1/room_orchestration_service.proto', 'sdkwork.communication.internal.v1'],
  ['apis/rpc/sdkwork/communication/internal/v1/message_dispatch_service.proto', 'sdkwork.communication.internal.v1'],
]) {
  const source = read(relativePath);
  assert.match(source, /syntax = "proto3";/u, `${relativePath} must use proto3.`);
  assert.match(source, new RegExp(`package ${escapeRegex(packageName)};`, 'u'), `${relativePath} must use ${packageName}.`);
  const protoServices = parseProtoMethods(relativePath);
  for (const [serviceName, methods] of protoServices) {
    for (const methodName of methods) {
      const manifestKey = `${packageName}.${serviceName}.${methodName}`;
      assert.ok(manifestMethods.has(manifestKey), `${manifestKey} must be covered by the RPC manifest.`);
    }
  }
}

for (const requiredOperationId of [
  'presence.heartbeat.create',
  'realtime.subscriptions.sync',
  'realtime.events.watch',
  'conversations.create',
  'conversations.members.add',
  'conversations.messages.create',
  'rooms.create',
  'messages.edit',
  'social.friendRequests.create',
  'streams.frames.create',
  'calls.sessions.invite',
  'notifications.requests.create',
  'automation.executions.create',
  'nodes.routes.migrate',
  'social.runtime.repairSharedChannelSync.create',
  'internal.routeLeases.claim',
  'internal.domainEvents.watch',
]) {
  assert.ok(
    [...manifestMethods.values()].some((method) => method.operationId === requiredOperationId),
    `manifest must cover ${requiredOperationId}.`,
  );
}

for (const [key, method] of manifestMethods) {
  if (
    /\.(create|update|delete|add|remove|transferOwner|changeRole|leave|edit|recall|invite|accept|reject|end|sync|ack|complete|abort|migrate|repair|claim|release|renew|publish|replay)/u.test(method.operationId)
    && method.streaming === 'unary'
  ) {
    assert.notEqual(method.idempotency, 'none', `${key} is a retryable write and must not declare idempotency none.`);
  }
}

const readme = read('sdks/sdkwork-im-rpc-sdk/README.md');
assert.doesNotMatch(
  readme,
  /\.sdkwork\/sdkwork-generator/u,
  'RPC SDK README must not list derived optional control-plane paths as day-to-day source evidence.',
);
for (const requiredText of [
  'Current Capability Inventory',
  'comms-conversation-service',
  'session-gateway',
  'streaming-service',
  'notification-service',
  'projection-service',
  'automation-service',
  'control-plane-api',
  'ops-service',
  'sdkwork-im-cloud-gateway calls runtime',
  'Flexible Distributed Deployment',
  'single-process local mode',
  'split service mode',
  'sharded realtime mode',
  'mTLS',
  'health checking',
  'reflection',
  'sdkgen generate --protocol rpc',
  'TypeScript',
  'Go',
  'Java',
  'Python',
  'Rust',
  'All baseline SDKWork RPC languages',
  'sdkwork-im-rpc-sdk-go',
  'sdkwork-im-rpc-sdk-java',
  'sdkwork-im-rpc-sdk-python',
  'sdkwork-im-rpc-sdk-rust',
  'convention mode',
  '--emit-control-plane',
  'sdkgen.js inspect --protocol rpc',
  'sdkwork-im-sdk',
  'sdkwork-im-app-sdk',
  'sdkwork-im-backend-sdk',
]) {
  assert.match(readme, new RegExp(escapeRegex(requiredText), 'u'), `RPC SDK README must document ${requiredText}.`);
}

const sdkReadme = read('sdks/README.md');
assert.match(sdkReadme, /sdkwork-im-rpc-sdk/u, 'sdks README must index the RPC SDK family.');
assert.match(sdkReadme, /gRPC/u, 'sdks README must describe the RPC family as gRPC.');
assert.match(sdkReadme, /sdkgen\.js inspect --protocol rpc/u, 'sdks README must document RPC inspect verification.');

const componentReadme = read('sdks/sdkwork-im-rpc-sdk/specs/README.md');
assert.match(componentReadme, /convention evidence/u, 'component README must document RPC convention evidence.');
assert.match(componentReadme, /sdkgen\.js inspect --protocol rpc/u, 'component README must document RPC inspect verification.');
assert.doesNotMatch(
  componentReadme,
  /\.sdkwork\/sdkwork-generator/u,
  'component README must keep optional RPC control-plane paths convention-derived instead of listing them.',
);
for (const language of expectedRpcLanguages) {
  assert.match(
    componentReadme,
    new RegExp(`sdkwork-im-rpc-sdk-${language}`, 'u'),
    `component README must document the ${language} RPC SDK workspace.`,
  );
}

const languageScaffolds = [
  {
    language: 'typescript',
    root: 'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-typescript',
    requiredText: ['@sdkwork/im-rpc-sdk', 'RPC_SDK_FAMILY', 'sdkwork-im-rpc-sdk'],
  },
  {
    language: 'go',
    root: 'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-go',
    requiredText: ['github.com/sdkwork/im-rpc-sdk-go', 'RpcSdkFamily', 'sdkwork-im-rpc-sdk'],
  },
  {
    language: 'java',
    root: 'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-java',
    requiredText: ['com.sdkwork.im.rpc', 'RPC_SDK_FAMILY', 'sdkwork-im-rpc-sdk'],
  },
  {
    language: 'python',
    root: 'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-python',
    requiredText: ['sdkwork-im-rpc-sdk', 'RPC_SDK_FAMILY'],
  },
  {
    language: 'rust',
    root: 'sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-rust',
    requiredText: ['sdkwork-im-rpc-sdk', 'RPC_SDK_PROTOCOL'],
  },
];

for (const scaffold of languageScaffolds) {
  assert.ok(!fs.existsSync(repoPath(`${scaffold.root}/.sdkwork`)), `${scaffold.language} RPC SDK convention output must not commit .sdkwork generator state.`);

  const methodCatalog = readJson(`${scaffold.root}/rpc-methods.json`);
  assert.equal(methodCatalog.kind, 'sdkwork.rpc.methodCatalog');
  assert.equal(methodCatalog.sdkFamily, 'sdkwork-im-rpc-sdk');
  assert.equal(methodCatalog.domain, 'communication');
  assert.equal(methodCatalog.services.length, manifest.services.length);
  assert.equal(
    methodCatalog.methods.length,
    manifestMethods.size,
    `${scaffold.language} RPC SDK method catalog must include every manifest method.`,
  );
  for (const requiredMethodKey of [
    'sdkwork.communication.app.v3.MessageService/CreateConversationMessage',
    'sdkwork.communication.backend.v3.SocialRuntimeAdminService/RepairSharedChannelSync',
    'sdkwork.communication.internal.v1.DomainEventRelayService/WatchDomainEvents',
  ]) {
    assert.ok(
      methodCatalog.methods.some((method) => method.methodKey === requiredMethodKey),
      `${scaffold.language} RPC SDK method catalog must include ${requiredMethodKey}.`,
    );
  }

  const joined = [
    listFiles(scaffold.root)
      .map((entryPath) => read(entryPath))
      .join('\n'),
  ].join('\n');
  for (const requiredText of scaffold.requiredText) {
    assert.match(joined, new RegExp(escapeRegex(requiredText), 'u'), `${scaffold.language} RPC SDK scaffold must include ${requiredText}.`);
  }
}

const typeScriptRpcPackage = readJson('sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-typescript/package.json');
assert.ok(
  typeScriptRpcPackage.files.includes('rpc-methods.json'),
  'TypeScript RPC SDK package files must include rpc-methods.json.',
);
assert.equal(typeScriptRpcPackage.dependencies['@bufbuild/protobuf'], '^2.12.0');
assert.equal(typeScriptRpcPackage.dependencies['@connectrpc/connect'], '^2.1.0');

const typeScriptBufGen = read('sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-typescript/buf.gen.yaml');
assert.match(typeScriptBufGen, /remote: buf\.build\/bufbuild\/es/u);
assert.match(typeScriptBufGen, /remote: buf\.build\/connectrpc\/es/u);
assert.doesNotMatch(typeScriptBufGen, /local: protoc-gen-/u);

const goBufGen = read('sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-go/buf.gen.yaml');
assert.match(goBufGen, /remote: buf\.build\/protocolbuffers\/go/u);
assert.match(goBufGen, /remote: buf\.build\/grpc\/go/u);
assert.match(goBufGen, /go_package_prefix/u);
assert.match(goBufGen, /github\.com\/sdkwork\/im-rpc-sdk-go\/generated\/proto/u);
assert.doesNotMatch(goBufGen, /local: protoc-gen-/u);

const goGeneratedMessage = read('sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-go/generated/proto/sdkwork/communication/app/v3/message_service.pb.go');
assert.match(goGeneratedMessage, /github\.com\/sdkwork\/im-rpc-sdk-go\/generated\/proto\/sdkwork\/common\/v1/u);
assert.doesNotMatch(goGeneratedMessage, /communication-rpc-go/u);

const javaPom = read('sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-java/pom.xml');
assert.match(javaPom, /<grpc\.version>1\.81\.0<\/grpc\.version>/u);
assert.match(javaPom, /<protobuf\.version>4\.33\.2<\/protobuf\.version>/u);
assert.match(javaPom, /<sourceDirectory>generated\/proto\/java<\/sourceDirectory>/u);

const pythonPyproject = read('sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-python/pyproject.toml');
assert.match(pythonPyproject, /package-dir = \{"" = "src", "sdkwork" = "generated\/proto\/sdkwork"\}/u);

const rustCargo = read('sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-rust/Cargo.toml');
assert.match(rustCargo, /\[workspace\]/u);
assert.match(rustCargo, /prost = "0\.14"/u);
assert.match(rustCargo, /tonic = \{ version = "0\.14"/u);
assert.match(rustCargo, /tonic-prost = "0\.14"/u);

const rustSdkLib = read('sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-rust/src/lib.rs');
assert.match(rustSdkLib, /sdkwork\.communication\.app\.v3\.rs/u);
assert.doesNotMatch(rustSdkLib, /sdkwork\.communication\.app\.v3\.tonic\.rs/u);

const generatedChecks = [
  ['sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-typescript/generated/proto', 20],
  ['sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-go/generated/proto', 20],
  ['sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-java/generated/proto', 700],
  ['sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-python/generated/proto', 20],
  ['sdks/sdkwork-im-rpc-sdk/sdkwork-im-rpc-sdk-rust/generated/proto', 7],
];

for (const [relativeRoot, minimumFiles] of generatedChecks) {
  const fileCount = listFiles(relativeRoot).length;
  assert.ok(fileCount >= minimumFiles, `${relativeRoot} must contain generated protobuf/gRPC output.`);
}

const serviceManifestSource = read('crates/sdkwork-im-rpc-service-rust/src/service_manifest.rs');
assert.match(serviceManifestSource, /RPC_SERVICE_BINDINGS/u, 'Rust RPC service scaffold must expose RPC_SERVICE_BINDINGS.');
assert.match(serviceManifestSource, /sdkwork-im-rpc-sdk/u, 'Rust RPC service scaffold must name sdkwork-im-rpc-sdk.');

for (const service of manifest.services) {
  const serviceKey = `${service.package}.${service.service}`;
  assert.match(
    serviceManifestSource,
    new RegExp(escapeRegex(serviceKey), 'u'),
    `Rust RPC service scaffold must bind ${serviceKey}.`,
  );
}

const methodManifestSource = read('crates/sdkwork-im-rpc-service-rust/src/method_manifest.rs');
assert.match(methodManifestSource, /RPC_METHOD_BINDINGS/u, 'Rust RPC service scaffold must expose RPC_METHOD_BINDINGS.');

const serviceBindingSource = read('crates/sdkwork-im-rpc-service-rust/src/service_binding.rs');
const rpcServiceBuildScript = read('crates/sdkwork-im-rpc-service-rust/build.rs');
assert.match(
  rpcServiceBuildScript,
  /apis[\s\S]*rpc[\s\S]*sdkwork[\s\S]*communication/u,
  'Rust RPC service build script must read proto sources from apis/rpc/sdkwork/communication.',
);
for (const requiredText of [
  'RpcRuntimeAdapter',
  'RpcServiceBinding',
  'RpcMethodRuntimeAdapter',
  'RpcMethodBinding',
  'bind_all_rpc_services',
  'bind_all_rpc_methods',
  'adapter.register_service',
  'adapter.register_method',
]) {
  assert.match(serviceBindingSource, new RegExp(escapeRegex(requiredText), 'u'), `Rust RPC service binding scaffold must include ${requiredText}.`);
}

const workflow = readJson('sdkwork.workflow.json');
const workflowDependencyIds = new Set((workflow.dependencies || []).map((dependency) => dependency.id));
assert.equal(
  workflowDependencyIds.has('sdkwork-discovery'),
  false,
  'sdkwork.workflow.json must not checkout sdkwork-discovery until ADR-20260619 Phase 2 product integration',
);
assert.match(
  read('Cargo.toml'),
  /sdkwork-discovery is deferred until hosted gRPC RPC service processes ship/u,
  'Cargo.toml must document deferred sdkwork-discovery integration',
);
assert.match(
  read('AGENTS.md'),
  /sdkwork-discovery.*deferred until Phase 2/u,
  'AGENTS.md must document deferred sdkwork-discovery product integration',
);
assert.ok(
  fs.existsSync(repoPath('docs/architecture/decisions/ADR-20260619-im-rpc-discovery-integration-deferred.md')),
  'ADR-20260619 must document phased sdkwork-discovery adoption',
);

console.log('sdkwork im RPC contract test passed');
