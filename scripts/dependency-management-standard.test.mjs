#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { COMMERCE_T1_REPOSITORY_IDS } from './dev/commerce-t1-capabilities.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const expectedDependencyIds = [
  'sdkwork-app-topology',
  'sdkwork-appbase',
  'sdkwork-core',
  'sdkwork-database',
  'sdkwork-web-framework',
  'sdkwork-rpc-framework',
  'sdkwork-utils',
  'sdkwork-drive',
  'sdkwork-voice',
  'sdkwork-iam',
  ...COMMERCE_T1_REPOSITORY_IDS,
  'sdkwork-agents',
  'sdkwork-mail',
  'sdkwork-community',
  'sdkwork-company',
  'sdkwork-course',
  'sdkwork-ui',
  'sdkwork-rtc',
  'sdkwork-kernel',
  'sdkwork-aiot',
  'sdkwork-notary',
  'sdkwork-knowledgebase',
  'sdkwork-sdk-commons',
  'sdkwork-sdk-generator',
];
const siblingDependencyAliases = {};
const sourceDependencyFiles = [
  'package.json',
  'Cargo.toml',
  '.github/workflows/im-commercial-gates.yml',
  '.github/workflows/package.yml',
  'apps/sdkwork-im-pc/package.json',
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/package.json',
  'apps/sdkwork-im-pc/tsconfig.json',
  'apps/sdkwork-im-pc/vite.config.ts',
  'crates/im-domain-core/Cargo.toml',
  'crates/im-platform-contracts/Cargo.toml',
  'crates/sdkwork-api-im-standalone-gateway/Cargo.toml',
  'artifacts/releases/sync-sdk-release-catalog.mjs',
  'sdks/sdkwork-im-app-sdk/bin/verify-flutter-composed-workspace.mjs',
  'sdks/sdkwork-im-app-sdk/sdkwork-im-app-sdk-flutter/composed/pubspec_overrides.yaml',
  'sdks/sdkwork-im-app-sdk/sdkwork-im-app-sdk-flutter/composed/pubspec.lock',
];
const activeDocumentationFiles = [
  'README.md',
  'sdks/README.md',
  'specs/README.md',
  '.sdkwork/README.md',
  'docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md',
];
const retiredDependencyRoot = ['.sdkwork', 'dependencies'].join('/');
const retiredLocalScript = ['prepare-local', 'dependencies.mjs'].join('-');
const retiredDepsLocalPrefix = ['deps', 'local'].join(':');
const failures = [];

function readText(relativePath) {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!fs.existsSync(absolutePath)) {
    failures.push(`${relativePath} must exist`);
    return '';
  }
  return fs.readFileSync(absolutePath, 'utf8');
}

function readJson(relativePath) {
  const text = readText(relativePath);
  if (!text) {
    return {};
  }
  return JSON.parse(text);
}

function assert(condition, message) {
  if (!condition) {
    failures.push(message);
  }
}

function listFilesRecursive(rootDir) {
  const files = [];
  if (!fs.existsSync(rootDir)) {
    return files;
  }
  for (const entry of fs.readdirSync(rootDir, { withFileTypes: true })) {
    const entryPath = path.join(rootDir, entry.name);
    if (entry.isDirectory()) {
      files.push(...listFilesRecursive(entryPath));
      continue;
    }
    files.push(entryPath);
  }
  return files;
}

function slashPath(value) {
  return String(value).replaceAll('\\', '/');
}

function sdkworkSiblingDependencyIdsFromCargo(relativePath) {
  const text = readText(relativePath);
  return [...new Set(
    [...text.matchAll(/path\s*=\s*"[^"]*\.\.\/(sdkwork-[A-Za-z0-9-]+)(?:[\/\\]|")/g)]
      .map((match) => match[1]),
  )].sort();
}

function assertNoRetiredDependencyModel(relativePath) {
  const text = readText(relativePath);
  assert(!text.includes(retiredDependencyRoot), `${relativePath} must not reference the retired SDKWork dependency root`);
  assert(!text.includes(retiredLocalScript.replace(/\.mjs$/u, '')), `${relativePath} must not reference the retired local dependency script`);
  assert(!text.includes(retiredDepsLocalPrefix), `${relativePath} must not reference retired local dependency scripts`);
}

function assertPnpmWorkspaceOnlyProtocol(relativePath) {
  if (!relativePath.endsWith('package.json') || relativePath === 'package.json') {
    return;
  }
  const text = readText(relativePath);
  const linkMatches = [...text.matchAll(/['"](link:[^'"]+)['"]/g)];
  for (const match of linkMatches) {
    const specifier = match[1];
    assert(
      !specifier.includes('sdkwork-'),
      `${relativePath} must not use ${specifier}; SDKWork cross-workspace sources must use the workspace: protocol declared in pnpm-workspace.yaml packages:`,
    );
  }
}

function assertCargoWorkspaceOnlyProtocol(relativePath) {
  if (!relativePath.endsWith('Cargo.toml') || relativePath === 'Cargo.toml') {
    return;
  }
  const text = readText(relativePath);
  const pathMatches = [...text.matchAll(/path\s*=\s*"((?:\.\.\/)+sdkwork-[A-Za-z0-9-]+[^"]*)"/g)];
  for (const match of pathMatches) {
    const path = match[1];
    assert(
      false,
      `${relativePath} must not redeclare cross-workspace SDKWork source path "${path}"; declare it in root [workspace.dependencies] and consume with \`crate_name.workspace = true\``,
    );
  }
}

function assertSiblingDependencyPathsAreKnown(relativePath) {
  const text = readText(relativePath);
  const absolutePath = path.join(repoRoot, relativePath);
  const sourceDir = path.dirname(absolutePath);
  const matches = [...text.matchAll(/(?:\.\.\/|\.{2}\\)+(sdkwork-[A-Za-z0-9-]*)/g)];
  for (const match of matches) {
    const dependencyId = siblingDependencyAliases[match[1]] ?? match[1];
    if (dependencyId === 'sdkwork-specs') {
      continue;
    }
    const resolvedTarget = path.resolve(sourceDir, match[0].replaceAll('\\', path.sep));
    const relativeToRepoRoot = path.relative(repoRoot, resolvedTarget);
    if (relativeToRepoRoot && !relativeToRepoRoot.startsWith('..') && !path.isAbsolute(relativeToRepoRoot)) {
      continue;
    }
    assert(
      expectedDependencyIds.includes(dependencyId),
      `${relativePath} uses undeclared SDKWork sibling dependency path ${match[0]}`,
    );
  }
}

function assertNativeDependencyFile(relativePath) {
  assertNoRetiredDependencyModel(relativePath);
  assertSiblingDependencyPathsAreKnown(relativePath);
  assertPnpmWorkspaceOnlyProtocol(relativePath);
  assertCargoWorkspaceOnlyProtocol(relativePath);
}

function assertDependencyDeclaration() {
  const workflow = readJson('sdkwork.workflow.json');
  assert(Array.isArray(workflow.dependencies), 'sdkwork.workflow.json must declare dependencies[]');
  const dependencyIds = new Set((workflow.dependencies || []).map((dependency) => dependency.id));
  for (const expectedId of expectedDependencyIds) {
    assert(dependencyIds.has(expectedId), `sdkwork.workflow.json must declare ${expectedId}`);
  }
  for (const dependency of workflow.dependencies || []) {
    assert(typeof dependency.id === 'string' && expectedDependencyIds.includes(dependency.id), `unexpected dependency id ${dependency.id}`);
    assert(/^Sdkwork-Cloud\/sdkwork-[a-z0-9-]+$/.test(dependency.repository || ''), `${dependency.id} must use owner/repo repository form`);
    assert(Boolean(dependency.ref || dependency.refInput), `${dependency.id} must declare ref or refInput`);
    assert(dependency.tokenSecret === 'SDKWORK_RELEASE_TOKEN', `${dependency.id} must use SDKWORK_RELEASE_TOKEN`);
    assert(!Object.hasOwn(dependency, 'path'), `${dependency.id} must omit dependencies[].path`);
  }
}

function assertNoLocalMaterializer() {
  const packageJson = readJson('package.json');
  assert(packageJson.scripts?.[[retiredDepsLocalPrefix, 'link'].join(':')] === undefined, 'package.json must not expose retired local link script');
  assert(packageJson.scripts?.[[retiredDepsLocalPrefix, 'check'].join(':')] === undefined, 'package.json must not expose retired local check script');
  assert(!readText('.gitignore').includes(retiredDependencyRoot), 'gitignore must not keep the retired SDKWork dependency ignore rule');
  assert(!fs.existsSync(path.join(repoRoot, 'scripts', retiredLocalScript)), 'retired local dependency script must not exist');
  assert(!fs.existsSync(path.join(repoRoot, ...retiredDependencyRoot.split('/'))), 'retired SDKWork dependency directory must not exist');
}

function assertCiMaterializer() {
  const materializer = readText('scripts/prepare-ci-dependencies.mjs');
  assert(materializer.includes('sdkwork.workflow.json'), 'prepare-ci-dependencies must read sdkwork.workflow.json');
  assert(materializer.includes("path.resolve(repoRoot, '..')"), 'prepare-ci-dependencies must use the workspace sibling repository root');
  assert(!materializer.includes(retiredDependencyRoot), 'prepare-ci-dependencies must not use the retired SDKWork dependency root');
  assert(materializer.includes('dependencies'), 'prepare-ci-dependencies must process dependency entries');
  assert(materializer.includes('tokenSecret'), 'prepare-ci-dependencies must honor dependency tokenSecret declarations');
  assert(!materializer.includes('const dependencyIds = ['), 'prepare-ci-dependencies must not duplicate a hard-coded dependency id list');

  const workflowYaml = readText('.github/workflows/im-commercial-gates.yml');
  assert(workflowYaml.includes('node scripts/prepare-ci-dependencies.mjs'), 'im commercial gates workflow must prepare SDKWork dependencies through the repository CI materializer');
}

function assertWorkflowRefs() {
  const workflowYaml = readText('.github/workflows/package.yml');
  assert(!workflowYaml.includes("dependency_refs_json: '{}'"), 'package workflow must not pass an empty dependency_refs_json');
  for (const dependencyId of expectedDependencyIds) {
    const inputName = `${dependencyId.replaceAll('-', '_')}_ref`;
    const envName = dependencyId.replaceAll('-', '_').toUpperCase();
    assert(workflowYaml.includes(inputName), `.github/workflows/package.yml must expose ${inputName}`);
    assert(workflowYaml.includes(envName), `.github/workflows/package.yml dependency_refs_json must include ${envName}`);
  }
}

function assertReleaseLifecycleDependencyGate() {
  const workflow = readJson('sdkwork.workflow.json');
  const buildLifecycleSource = (workflow.lifecycle?.build || [])
    .map((step) => step.run || '')
    .join('\n');
  assert(
    /pnpm\s+(?:run\s+)?check:dependency-management/u.test(buildLifecycleSource),
    'sdkwork.workflow.json build lifecycle must run pnpm check:dependency-management before release packaging so sdkwork-notary and sdkwork-drive app SDK dependency refs are verified in package jobs',
  );
}

function assertDiscoveryIntegrationDeferred() {
  const workflow = readJson('sdkwork.workflow.json');
  const dependencyIds = new Set((workflow.dependencies || []).map((dependency) => dependency.id));
  assert(
    !dependencyIds.has('sdkwork-discovery'),
    'sdkwork.workflow.json must not declare sdkwork-discovery until ADR-20260619 Phase 1 RPC hosts ship',
  );

  const rootCargo = readText('Cargo.toml');
  assert(
    !/^\s*sdkwork[_-]discovery\s*=/mu.test(rootCargo)
      && !/path\s*=\s*"\.\.\/sdkwork-discovery/u.test(rootCargo),
    'Cargo.toml must not declare sdkwork-discovery workspace dependencies until hosted gRPC service processes ship',
  );
  assert(
    /sdkwork-discovery is deferred until hosted gRPC RPC service processes ship/u.test(rootCargo),
    'Cargo.toml must document deferred sdkwork-discovery integration',
  );

  const adrPath = 'docs/architecture/decisions/ADR-20260619-im-rpc-discovery-integration-deferred.md';
  assert(fs.existsSync(path.join(repoRoot, adrPath)), `${adrPath} must document deferred sdkwork-discovery adoption`);

  const specsReadme = readText('specs/README.md');
  assert(
    specsReadme.includes('ADR-20260619-im-rpc-discovery-integration-deferred.md'),
    'specs/README.md must link the deferred sdkwork-discovery ADR',
  );
  assert(
    /sdkwork-discovery[\s\S]*Deferred/u.test(specsReadme),
    'specs/README.md must keep sdkwork-discovery status Deferred until RPC hosts ship',
  );
}

function assertProfileResolvedPlatformIntegration() {
  const componentSpec = readJson('specs/component.spec.json');
  const componentSpecText = readText('specs/component.spec.json');
  const platformGateway = componentSpec.integration?.platformApiGateway;

  assert(
    platformGateway?.connectivitySurface === 'platform.api-gateway',
    'specs/component.spec.json must declare platform.api-gateway as a topology surface',
  );
  assert(
    platformGateway?.targetMode === 'profile-resolved',
    'specs/component.spec.json platformApiGateway.targetMode must be profile-resolved',
  );
  assert(
    platformGateway?.commonSdkRootEnv === 'SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL',
    'specs/component.spec.json must use SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL as the server platform SDK root',
  );
  assert(
    platformGateway?.browserSdkRootEnv === 'VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL',
    'specs/component.spec.json must use VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL as the browser platform SDK root',
  );
  assert(
    platformGateway?.authority === 'topology-profile',
    'Sdkwork IM platform API gateway resolution must use topology profiles as authority',
  );
  assert(
    platformGateway?.productApiPolicy === 'Sdkwork IM IM APIs remain product-owned SDKWork API surfaces',
    'Sdkwork IM component spec must keep IM APIs product-owned',
  );
  assert(
    platformGateway?.alignmentState === 'current',
    'Sdkwork IM platform API gateway contract must be aligned to the current topology model',
  );

  assert(
    !componentSpecText.includes('legacyCompatibilityComponents')
      && !componentSpecText.includes('legacyDirectFoundationRuntimeDependencies')
      && !componentSpecText.includes('legacy-web-gateway'),
    'Sdkwork IM alignment is complete only when component.spec.json has no legacy gateway compatibility or direct platform runtime dependencies',
  );

  assert(
    !Array.isArray(platformGateway?.legacyCompatibilityDefaultFoundationUpstreams),
    'Sdkwork IM must not document per-module upstreams beside the topology surface',
  );
  assert(
    platformGateway?.explicitExternalFoundationUpstreams === undefined
      && platformGateway?.explicitExternalUpstreamEnvKeys === undefined,
    'Sdkwork IM must not publish per-module upstream overrides',
  );

  for (const relativePath of ['Cargo.toml', 'crates/sdkwork-api-im-standalone-gateway/Cargo.toml']) {
    assert(
      !/^sdkwork_iam_http\s*=/mu.test(readText(relativePath)),
      `${relativePath} must not depend on sdkwork_iam_http; appbase app API runtime is owned by platform.api-gateway`,
    );
  }

  const rootCargoSource = readText('Cargo.toml');
  for (const [relativePath, source] of [
    ['Cargo.toml', rootCargoSource],
  ]) {
    for (const dependencyName of [
      'sdkwork-agent-business',
      'sdkwork-aiot-contract',
      'sdkwork-aiot-http-api',
      'sdkwork-aiot-runtime',
      'sdkwork-aiot-transport',
    ]) {
      assert(
        !source.includes(dependencyName),
        `${relativePath} must not depend on ${dependencyName}; Agent and AIoT runtime APIs are served through platform.api-gateway`,
      );
    }
  }

  const dependencyApiSurfaces = componentSpec.contracts?.dependencyApiSurfaces ?? [];
  const platformSurfaceIds = dependencyApiSurfaces
    .filter((surface) =>
      surface.targetRuntimeIntegration?.connectivitySurface === 'platform.api-gateway'
        && surface.targetRuntimeIntegration?.mode === 'profile-resolved')
    .map((surface) => surface.apiAuthority)
    .sort();
  const expectedSharedGatewaySurfaceIds = [
    'sdkwork-account-app-api',
    'sdkwork-iam-app-api',
    'sdkwork-agents-app-api',
    'sdkwork-agents-backend-api',
    'sdkwork-agents-open-api',
    'sdkwork-aiot-app-api',
    'sdkwork-aiot-backend-api',
    'sdkwork-drive-app-api',
    'sdkwork-notary-app-api',
    'sdkwork-catalog-app-api',
    'sdkwork-shop-app-api',
    'sdkwork-order-app-api',
    'sdkwork-membership-app-api',
    'sdkwork-mail-app-api',
    'sdkwork-community-app-api',
    'sdkwork-course-app-api',
    'sdkwork-knowledgebase-app-api',
    'sdkwork-rtc-app-api',
    'sdkwork-rtc-backend-api',
  ].sort();
  assert(
    JSON.stringify(platformSurfaceIds) === JSON.stringify(expectedSharedGatewaySurfaceIds),
    `component spec must declare the current profile-resolved dependency API surfaces, got ${platformSurfaceIds.join(',')}`,
  );
  for (const surface of dependencyApiSurfaces) {
    assert(
      surface.targetRuntimeIntegration?.catalogPolicy === undefined,
      `${surface.apiAuthority} must not carry a gateway catalog policy`,
    );
    assert(
      surface.currentCompatibility === undefined,
      `${surface.apiAuthority} must not keep legacy web-gateway compatibility after migration to platform.api-gateway`,
    );
  }

  for (const relativePath of [
    'crates/sdkwork-api-im-standalone-gateway/src/main.rs',
    'crates/sdkwork-api-im-standalone-gateway/src/embedded_dependency_routes.rs',
    'crates/sdkwork-api-im-assembly/src/bootstrap.rs',
  ]) {
    const source = readText(relativePath);
    for (const marker of [
      'GatewayRuntimeMode::Embedded',
      'Embedded,',
      'build_embedded_appbase',
      'embedded_appbase',
      'sdkwork_iam_http',
    ]) {
      assert(
        !source.includes(marker),
        `${relativePath} must not keep embedded/product-local foundation API runtime marker ${marker}`,
      );
    }
  }

  const forbiddenGatewayCatalogs = listFilesRecursive(path.join(repoRoot, 'specs'))
    .map((filePath) => slashPath(path.relative(repoRoot, filePath)))
    .filter((relativePath) =>
      /(^|\/)(platform.api-gateway-catalog|api-gateway-catalog|gateway-catalog|foundation-api-catalog)\.(json|ya?ml|toml)$/iu.test(relativePath)
    );
  assert(
    forbiddenGatewayCatalogs.length === 0,
    `gateway integration must not add standalone gateway catalog files: ${forbiddenGatewayCatalogs.join(', ')}`,
  );
}

function assertDocumentation() {
  for (const relativePath of activeDocumentationFiles) {
    assertNativeDependencyFile(relativePath);
  }
  const specsReadme = readText('specs/README.md');
  assert(specsReadme.includes('../sdkwork-specs/DEPENDENCY_MANAGEMENT_SPEC.md'), 'specs/README.md must link DEPENDENCY_MANAGEMENT_SPEC.md via ../sdkwork-specs');
  assert(!specsReadme.includes('../../../specs/'), 'specs/README.md must not link the old ../../../specs standards path');

  const imAppApiSpec = readText('specs/im-app-api-sdk-integration.spec.md');
  assert(
    !/current legacy web-gateway compatibility|long-term foundation API aggregation authority/u.test(imAppApiSpec),
    'specs/im-app-api-sdk-integration.spec.md must document platform.api-gateway as the current shared foundation API boundary without legacy web-gateway compatibility wording',
  );

  const imAgentsBoundarySpec = readText('specs/IM_AGENTS_DEPENDENCY_AND_DATABASE_SPEC.md');
  assert(
    imAgentsBoundarySpec.includes('sdkwork-im -> sdkwork-agents -> sdkwork-kernel'),
    'IM Agents boundary spec must declare the one-way sdkwork-im -> sdkwork-agents -> sdkwork-kernel dependency',
  );
  assert(
    imAgentsBoundarySpec.includes('IM code and migrations MUST NOT write any `ai_agent_*` table'),
    'IM Agents boundary spec must forbid cross-module writes to Agents tables',
  );
  assert(
    /Agents\s+MUST NOT import IM/u.test(imAgentsBoundarySpec),
    'IM Agents boundary spec must forbid the reverse Agents-to-IM dependency',
  );
  for (const targetTable of [
    'im_conversation_agent_assignments',
    'im_conversation_agent_binding',
    'im_agent_dispatch',
  ]) {
    assert(
      imAgentsBoundarySpec.includes(`\`${targetTable}\``),
      `IM Agents database target must define ${targetTable}`,
    );
  }
  assert(
    imAgentsBoundarySpec.includes('id, uuid, binding_id'),
    'IM Agents binding target must define a stable binding_id for dispatch correlation',
  );
  assert(
    /There is no\s+foreign key to an `ai_agent_\*` table/u.test(imAgentsBoundarySpec),
    'IM Agents database target must forbid cross-module foreign keys',
  );
}

function assertAgentsDependencyBoundary() {
  const cargoFiles = [path.join(repoRoot, 'Cargo.toml')];
  for (const moduleRoot of ['adapters', 'apps', 'crates', 'services']) {
    const absoluteModuleRoot = path.join(repoRoot, moduleRoot);
    if (!fs.existsSync(absoluteModuleRoot)) continue;
    for (const entry of fs.readdirSync(absoluteModuleRoot, { withFileTypes: true })) {
      if (!entry.isDirectory()) continue;
      const manifest = path.join(absoluteModuleRoot, entry.name, 'Cargo.toml');
      if (fs.existsSync(manifest)) cargoFiles.push(manifest);
    }
  }
  const forbiddenPrivateCrates = [
    'sdkwork-routes-agents-app-api',
    'sdkwork-intelligence-agents-service',
    'sdkwork-agents-database-host',
  ];
  for (const filePath of cargoFiles) {
    const relativePath = slashPath(path.relative(repoRoot, filePath));
    const source = fs.readFileSync(filePath, 'utf8');
    for (const crateName of forbiddenPrivateCrates) {
      assert(
        !new RegExp(`^\\s*${crateName}\\s*=`, 'mu').test(source),
        `${relativePath} must consume the public Agents facade/assembly instead of ${crateName}`,
      );
    }
  }
  const rootCargo = readText('Cargo.toml');
  const standaloneCargo = readText('crates/sdkwork-api-im-standalone-gateway/Cargo.toml');
  assert(
    rootCargo.includes('sdkwork-api-agents-assembly'),
    'Cargo.toml must declare the canonical Agents API assembly for embedded host composition',
  );
  assert(
    standaloneCargo.includes('sdkwork-api-agents-assembly = { workspace = true }'),
    'standalone gateway must consume Agents through the canonical API assembly',
  );
}

assertDependencyDeclaration();
assertNoLocalMaterializer();
assertCiMaterializer();
assertWorkflowRefs();
assertReleaseLifecycleDependencyGate();
assertDiscoveryIntegrationDeferred();
assertProfileResolvedPlatformIntegration();
assertAgentsDependencyBoundary();
for (const relativePath of sourceDependencyFiles) {
  assertNativeDependencyFile(relativePath);
}
assertDocumentation();

if (failures.length > 0) {
  process.stderr.write(`Dependency management standard failed:\n${failures.map((failure) => `- ${failure}`).join('\n')}\n`);
  process.exit(1);
}

process.stdout.write('Dependency management standard passed\n');
