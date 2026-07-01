import assert from 'node:assert/strict';
import { EventEmitter } from 'node:events';
import fs from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function deploymentDocsRoot() {
  const docsRoot = path.join(repoRoot, 'docs');
  for (const entry of fs.readdirSync(docsRoot, { withFileTypes: true })) {
    if (!entry.isDirectory()) {
      continue;
    }
    const marker = path.join(docsRoot, entry.name, 'postgresql-database-configuration.md');
    if (fs.existsSync(marker)) {
      return path.join(docsRoot, entry.name);
    }
  }
  throw new Error('deployment documentation directory must include postgresql-database-configuration.md');
}

function readDeploymentDoc(fileName) {
  return fs.readFileSync(path.join(deploymentDocsRoot(), fileName), 'utf8');
}

function readDeploymentDocsMatching(pattern) {
  const deploymentDir = deploymentDocsRoot();
  return fs
    .readdirSync(deploymentDir)
    .filter((fileName) => pattern.test(fileName))
    .map((fileName) => fs.readFileSync(path.join(deploymentDir, fileName), 'utf8'));
}

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(repoRoot, relativePath), 'utf8'));
}

const packageJson = readJson('package.json');
const chatPcPackageJson = readJson('apps/sdkwork-im-pc/package.json');
assert.equal(
  chatPcPackageJson.workspaces,
  undefined,
  'apps/sdkwork-im-pc must not declare nested npm workspaces; pnpm workspace membership belongs in repository-root pnpm-workspace.yaml only',
);
assert.equal(
  chatPcPackageJson.scripts.lint,
  'node ../../scripts/dev/run-tsc-cli.mjs --noEmit -p tsconfig.app.json',
  'apps/sdkwork-im-pc lint must typecheck IM-owned sources via tsconfig.app.json instead of sibling source path maps',
);
assert.ok(
  !chatPcPackageJson.dependencies?.['@google/genai'],
  'apps/sdkwork-im-pc must not depend on retired @google/genai AI Studio scaffold',
);
const desktopWorkspaceSource = fs.readFileSync(
  path.join(repoRoot, 'pnpm-workspace.yaml'),
  'utf8',
);
const desktopNpmrcSource = fs.existsSync(path.join(repoRoot, 'apps/sdkwork-im-pc/.npmrc'))
  ? fs.readFileSync(path.join(repoRoot, 'apps/sdkwork-im-pc/.npmrc'), 'utf8')
  : '';
const desktopLockfileSource = fs.readFileSync(
  path.join(repoRoot, 'pnpm-lock.yaml'),
  'utf8',
);
const unifiedWebSource = fs.readFileSync(
  path.join(repoRoot, 'scripts/im-server-dev.mjs'),
  'utf8',
);
const imGatewayCargoSource = fs.readFileSync(
  path.join(repoRoot, 'services/sdkwork-im-cloud-gateway/Cargo.toml'),
  'utf8',
);
const imDomainCoreCargoSource = fs.readFileSync(
  path.join(repoRoot, 'crates/im-domain-core/Cargo.toml'),
  'utf8',
);
const imPlatformContractsCargoSource = fs.readFileSync(
  path.join(repoRoot, 'crates/im-platform-contracts/Cargo.toml'),
  'utf8',
);
const postgresEnvExampleSource = fs.readFileSync(
  path.join(repoRoot, '.env.postgres.example'),
  'utf8',
);
const postgresDatabaseConfigIndexSource = readDeploymentDoc('postgresql-database-configuration.md');
const postgresDevelopmentGuideSource = readDeploymentDocsMatching(/PostgreSQL.*\.md$/u).join('\n');
const postgresProductionGuideSource = postgresDevelopmentGuideSource;
const localAppApiSource = fs.readFileSync(
  path.join(repoRoot, 'scripts/dev/start-sdkwork-im-local-app-api.mjs'),
  'utf8',
);
const sharedDatabaseSource = fs.readFileSync(
  path.join(repoRoot, 'scripts/dev/sdkwork-im-shared-database.mjs'),
  'utf8',
);
const {
  createSdkworkChatBrowserOrigins,
  createSdkworkChatPcDevPlan,
  resolveNotaryAppApiUpstream,
  resolveCatalogAppApiUpstream,
  resolveAvailableSdkworkChatPcDevPort,
  runSdkworkChatPcDev,
} = await import(pathToFileURL(path.join(repoRoot, 'scripts/lib/im-pc-dev.mjs')).href);
const {
  resolveSdkworkImSharedDatabaseConfig,
} = await import(pathToFileURL(path.join(repoRoot, 'scripts/dev/sdkwork-im-shared-database.mjs')).href);
const {
  ensureSdkworkUiDist,
  resolveSdkworkUiPackageRoot,
} = await import(pathToFileURL(path.join(repoRoot, 'scripts/dev/sdkwork-ui-runtime-lib.mjs')).href);

assert.equal(
  packageJson.scripts.dev,
  'pnpm dev:browser',
  'root pnpm dev must delegate to the browser dev stack',
);
assert.equal(
  packageJson.scripts['dev:browser'],
  'pnpm dev:browser:postgres:unified-process:standalone',
  'root pnpm dev:browser must delegate to the PostgreSQL standalone profile',
);
assert.equal(
  packageJson.scripts['dev:browser:postgres:unified-process:standalone'],
  'node scripts/im-dev.mjs --target browser --deployment-profile standalone --service-layout unified-process --database postgres',
  'root pnpm dev:browser full profile must start PostgreSQL standalone browser dev',
);
assert.doesNotMatch(
  packageJson.scripts['dev:browser:postgres:unified-process:standalone'],
  / --env-file /u,
  'root pnpm dev:browser full profile must not use Node 22 reserved --env-file argument',
);
assert.equal(
  packageJson.scripts['dev:desktop'],
  'pnpm dev:desktop:postgres:unified-process:standalone',
  'root pnpm dev:desktop must delegate to the PostgreSQL standalone profile',
);
assert.equal(
  packageJson.scripts['dev:desktop:postgres:unified-process:standalone'],
  'node scripts/im-dev.mjs --target desktop --deployment-profile standalone --service-layout unified-process --database postgres',
  'root pnpm dev:desktop full profile must start PostgreSQL standalone desktop dev',
);
assert.equal(
  packageJson.scripts['dev:desktop:sqlite'],
  'node scripts/im-dev.mjs --target desktop --deployment-profile standalone --service-layout unified-process --database sqlite',
  'root pnpm dev:desktop:sqlite must explicitly start desktop dev with SQLite',
);
assert.doesNotMatch(
  packageJson.scripts['dev:desktop:postgres:unified-process:standalone'],
  / --env-file /u,
  'root pnpm dev:desktop full profile must not use Node 22 reserved --env-file argument',
);
assert.equal(
  packageJson.scripts['dev:server'],
  'node scripts/im-server-dev.mjs',
  'root pnpm dev:server must remain the canonical Sdkwork IM server startup command',
);
assert.ok(
  unifiedWebSource.includes('resolveImProductSiteDirEnv'),
  'root pnpm dev:server must create local dev site fallbacks when admin or portal sources are absent',
);
assert.ok(
  unifiedWebSource.includes('createManagedSdkworkApiGatewayProcess')
    && unifiedWebSource.includes('SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL')
    && unifiedWebSource.includes('SDKWORK_API_CLOUD_GATEWAY_BIND'),
  'root pnpm dev:server must manage sdkwork-api-cloud-gateway and expose one shared foundation gateway root',
);
assert.ok(
  !unifiedWebSource.includes("?? 'embedded'")
    && !unifiedWebSource.includes('?? "embedded"'),
  'root pnpm dev:server must not default the product gateway runtime to embedded foundation aggregation',
);
assert.ok(
  unifiedWebSource.includes('cargo')
    && unifiedWebSource.includes('sdkwork-im-cloud-gateway')
    && unifiedWebSource.includes('sdkwork-im-server')
    && unifiedWebSource.includes('resolveSdkworkImSharedDatabaseConfig'),
  'root pnpm dev:server must start the Rust sdkwork-im-cloud-gateway in shared-gateway split mode with the shared database config',
);
assert.doesNotMatch(
  unifiedWebSource,
  /\bmvn(?:\.cmd)?\b|mavenCommand|spring-ai-plus-server-app|spring-boot:run|ensureAppbaseAppApi|waitForAppbaseAppApiReady|sdkwork-iam-app-api|SDKWORK_IM_APPBASE_APP_API_UPSTREAM|SDKWORK_APPBASE_APP_API_BIND_ADDR|SDKWORK_APPBASE_BROWSER_ORIGINS/u,
  'root pnpm dev:server must not start or wait for any Java/appbase app-api upstream',
);
assert.ok(
  unifiedWebSource.includes('resolveImProductSiteDirEnv')
    && unifiedWebSource.includes('Object.assign'),
  'root pnpm dev:server must pass the resolved admin and portal site dirs to the Rust gateway',
);
assert.ok(
  unifiedWebSource.includes('terminateStaleSdkworkImServerProcesses')
    && unifiedWebSource.includes('target')
    && unifiedWebSource.includes('sdkwork-im-server.exe'),
  'root pnpm dev:server must clean up stale same-workspace Windows sdkwork-im-server.exe processes before cargo rebuilds the locked binary',
);
assert.ok(
  unifiedWebSource.includes('terminateProcessTree')
    && unifiedWebSource.includes('taskkill')
    && unifiedWebSource.includes('/T')
    && unifiedWebSource.includes('/F'),
  'root pnpm dev:server must terminate the Windows cargo/server process tree instead of leaving sdkwork-im-server.exe behind',
);
assert.ok(
  localAppApiSource.includes('Sdkwork IM app-api is provided by the Rust unified server')
    && localAppApiSource.includes('resolveSdkworkImSharedDatabaseConfig'),
  'legacy local app API wrapper must point developers to the Rust unified server instead of starting a separate app-api host',
);
assert.doesNotMatch(
  localAppApiSource,
  /\bmvn(?:\.cmd)?\b|mavenCommand|spring-ai-plus-server-app|spring-boot:run|SDKWORK_APPBASE_APP_API_BIND_ADDR|SDKWORK_APPBASE_BROWSER_ORIGINS|SDKWORK_APPBASE_APP_API_STARTUP_TIMEOUT_MS/u,
  'legacy local app API wrapper must not retain Java or appbase startup residue',
);
assert.ok(
  !localAppApiSource.includes(['run-local', '-minimal.mjs'].join('')),
  'legacy local app API wrapper must not start a second runtime beside the unified Rust server',
);
assert.doesNotMatch(
  imGatewayCargoSource,
  /[A-Za-z]:[\\/]/u,
  'sdkwork-im-cloud-gateway Rust manifest must not point to an absolute checkout path',
);
assert.doesNotMatch(
  imGatewayCargoSource,
  /sdkwork-agent-business\.workspace\s*=\s*true/u,
  'sdkwork-im-cloud-gateway must not consume sdkwork-agent-business; Agent API runtime is routed through sdkwork-api-cloud-gateway',
);
for (const cargoSource of [
  imDomainCoreCargoSource,
  imPlatformContractsCargoSource,
]) {
  assert.doesNotMatch(
    cargoSource,
    /[A-Za-z]:[\\/]([^\\/]+[\\/])*sdkwork-rtc/u,
    'Sdkwork IM Rust manifests must not point RTC dependencies at an absolute checkout path',
  );
  assert.match(
    cargoSource,
    /sdkwork-communication-rtc-service\.workspace = true/u,
    'Sdkwork IM Rust manifests that depend on sdkwork-communication-rtc-service must consume it through [workspace.dependencies] with `workspace = true`',
  );
  assert.doesNotMatch(
    cargoSource,
    /path\s*=\s*"\.\.\/\.\.\/\.\.\/sdkwork-[A-Za-z0-9-]+/u,
    'Sdkwork IM Rust manifests must not redeclare cross-workspace SDKWork source paths in member crates; the path belongs in root [workspace.dependencies] only',
  );
}
assert.ok(
  desktopWorkspaceSource.includes('catalog:'),
  'desktop workspace must define catalog entries used by imported SDKWork packages',
);
assert.doesNotMatch(
  desktopWorkspaceSource,
  /\.\.\/\.\.\/\.\.\/\.\.\/apps\/sdkwork-(?:appbase|core|ui)\//u,
  'desktop workspace must not register sibling sdkwork-appbase/core/ui packages as workspace importers; they stay source-linked dependencies so install never rewrites sibling node_modules',
);
assert.ok(
  desktopWorkspaceSource.includes('link:') === false || desktopWorkspaceSource.includes('../../../sdkwork-iam/apps/sdkwork-iam-common/packages/sdkwork-iam-contracts') === true,
  'desktop workspace packages: section must declare every SDKWork sibling source path (not just the local packages/* glob)',
);
assert.ok(
  fs.existsSync(path.join(repoRoot, 'apps/sdkwork-im-pc/pnpm-lock.yaml')),
  'desktop app must keep dependency resolution in a pnpm lockfile',
);
assert.ok(
  !fs.existsSync(path.join(repoRoot, 'apps/sdkwork-im-pc/package-lock.json')),
  'desktop app must not keep an npm package-lock.json alongside pnpm workspace management',
);
assert.ok(
  desktopWorkspaceSource.includes('lucide-react: ^1.7.0'),
  'desktop workspace catalog must resolve lucide-react catalog dependencies',
);
assert.ok(
  desktopWorkspaceSource.includes('vite-plugin-dts: ^4.5.4'),
  'desktop workspace catalog must resolve sdkwork-core build tooling dependencies',
);
assert.match(
  desktopNpmrcSource,
  /^link-workspace-packages=true$/mu,
  'desktop workspace must link workspace packages during local development',
);
assert.match(
  desktopNpmrcSource,
  /^auto-install-peers=false$/mu,
  'desktop workspace must not auto-install peer SDK packages from the registry',
);
assert.match(
  desktopNpmrcSource,
  /^virtual-store-dir=node_modules\/\.pnpm-codex-new$/mu,
  'desktop workspace must use the dedicated local virtual store so stale external workspace links do not block reinstall on Windows',
);

const chatPcIndexCssSource = fs.readFileSync(path.join(repoRoot, 'apps/sdkwork-im-pc/src/index.css'), 'utf8');
const chatPcViteConfigSource = fs.readFileSync(path.join(repoRoot, 'apps/sdkwork-im-pc/vite.config.ts'), 'utf8');
assert.match(
  chatPcIndexCssSource,
  /@import\s+"tailwindcss"\s*;/u,
  'desktop shell index.css must own the single Tailwind bootstrap entry',
);
assert.doesNotMatch(
  chatPcViteConfigSource,
  /find:\s*['"]tailwindcss['"]/u,
  'desktop Vite config must not alias bare specifier tailwindcss',
);
assert.equal(
  chatPcPackageJson.dependencies?.tailwindcss,
  'catalog:',
  'desktop app must declare tailwindcss in dependencies for shell bootstrap import closure',
);
assert.equal(
  chatPcPackageJson.dependencies?.['@tailwindcss/vite'],
  'catalog:',
  'desktop app must declare @tailwindcss/vite in dependencies',
);
assert.equal(
  chatPcPackageJson.devDependencies?.tailwindcss,
  undefined,
  'desktop app must not keep tailwindcss only in devDependencies',
);

const sharedDependencyNames = [
  '@sdkwork-internal/im-app-api-generated',
  '@sdkwork-internal/im-backend-api-generated',
  '@sdkwork/iam-app-sdk',
  '@sdkwork/appbase-pc-react',
  '@sdkwork/auth-pc-react',
  '@sdkwork/auth-runtime-pc-react',
  '@sdkwork/core-pc-react',
  '@sdkwork/iam-sdk-ports',
  '@sdkwork/im-sdk',
  '@sdkwork/i18n-pc-react',
  '@sdkwork/ui-pc-react',
];
for (const dependencyName of sharedDependencyNames) {
  const version = chatPcPackageJson.dependencies?.[dependencyName];
  assert.equal(
    version,
    'workspace:*',
    `local dev dependency ${dependencyName} must use the workspace: protocol declared once in pnpm-workspace.yaml packages`,
  );
  assert.doesNotMatch(
    version,
    /^(?:https?:|git\+|github:|git@|link:)/u,
    `local dev dependency ${dependencyName} must not use a git, link:, or registry URL`,
  );
}

assert.equal(
  chatPcPackageJson.pnpm?.overrides,
  undefined,
  'apps/sdkwork-im-pc must not declare pnpm.overrides; repository root package.json owns workspace overrides',
);
const sharedSdkOverrides = readJson('package.json').pnpm?.overrides ?? {};
for (const [dependencyName, expectedVersion] of Object.entries({
  '@sdkwork-internal/im-app-api-generated': 'workspace:*',
  '@sdkwork-internal/im-backend-api-generated': 'workspace:*',
  '@sdkwork/iam-app-sdk': 'workspace:*',
  '@sdkwork/appbase-pc-react': 'workspace:*',
  '@sdkwork/im-sdk': 'workspace:*',
  '@sdkwork/rtc-sdk': 'workspace:*',
})) {
  assert.equal(
    sharedSdkOverrides[dependencyName],
    expectedVersion,
    `desktop workspace must override transitive ${dependencyName} to workspace:* (the local source path is declared in pnpm-workspace.yaml packages: only)`,
  );
  assert.doesNotMatch(
    desktopLockfileSource,
    new RegExp(`^\\s{2}'${dependencyName.replaceAll('/', '\\/')}@`, 'mu'),
    `desktop lockfile must not resolve registry package snapshots for ${dependencyName}`,
  );
}

const sharedSdkModeSource = fs.readFileSync(
  path.join(repoRoot, 'scripts/dev/shared-sdk-mode.mjs'),
  'utf8',
);
const sharedSdkGitSource = fs.readFileSync(
  path.join(repoRoot, 'scripts/dev/prepare-shared-sdk-git-sources.mjs'),
  'utf8',
);
const releaseBuildSource = fs.readFileSync(
  path.join(repoRoot, 'scripts/release/run-sdkwork-im-pc-release-build.mjs'),
  'utf8',
);
const releaseBuildModule = await import(
  pathToFileURL(path.join(repoRoot, 'scripts/release/run-sdkwork-im-pc-release-build.mjs')).href
);
const sharedSdkReleaseConfig = readJson('config/shared-sdk-release-sources.json');
assert.match(
  packageJson.scripts['sdk:shared:prepare'],
  /scripts\/dev\/prepare-shared-sdk-git-sources\.mjs/u,
  'root sdk:shared:prepare must materialize git-backed shared SDK sources for release builds',
);
assert.match(
  packageJson.scripts['release:build'],
  /scripts\/release\/run-sdkwork-im-pc-release-build\.mjs/u,
  'root release:build must delegate to the repository-owned release build wrapper',
);
assert.match(
  releaseBuildSource,
  /SDKWORK_SHARED_SDK_MODE.*git/u,
  'release build wrapper must enable git-backed shared SDK mode',
);
assert.match(
  releaseBuildSource,
  /sdk:shared:prepare/u,
  'root release:build must prepare shared SDK sources before building the Vite app',
);
assert.equal(
  typeof releaseBuildModule.createSdkworkChatPcReleaseBuildPlan,
  'function',
  'release build wrapper must expose an auditable command plan',
);
assert.equal(
  typeof resolveNotaryAppApiUpstream,
  'function',
  'PC dev command module must expose an auditable Notary app-api upstream resolver',
);
assert.equal(
  typeof resolveCatalogAppApiUpstream,
  'function',
  'PC dev command module must expose an auditable catalog app-api upstream resolver',
);
const releaseBuildPlan = releaseBuildModule.createSdkworkChatPcReleaseBuildPlan({
  env: {
    SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: 'https://im.example.com/',
    SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL: 'wss://im.example.com',
    SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL: 'https://api.example.com/',
    SDKWORK_DRIVE_REF: 'drive-release-ref',
    SDKWORK_NOTARY_REF: 'notary-release-ref',
    SDKWORK_SHARED_SDK_MODE: 'source',
  },
  repoRoot,
});
assert.deepEqual(
  releaseBuildPlan.steps.map((step) => step.label),
  [
    'prepare shared SDK git sources',
    'install sdkwork-im-pc workspace',
    'build sdkwork-im-pc Vite app',
  ],
  'release build must prepare git sources, install once, then build the Vite app from linked source packages',
);
assert.equal(
  releaseBuildPlan.env.SDKWORK_SHARED_SDK_MODE,
  'git',
  'release build plan must force git-backed shared SDK mode',
);
assert.equal(
  releaseBuildPlan.env.SDKWORK_SHARED_DRIVE_GIT_REF,
  'drive-release-ref',
  'release build plan must bridge SDKWORK_DRIVE_REF into the shared SDK materializer ref for Drive-backed notary files',
);
assert.equal(
  releaseBuildPlan.env.SDKWORK_SHARED_NOTARY_GIT_REF,
  'notary-release-ref',
  'release build plan must bridge SDKWORK_NOTARY_REF into the shared SDK materializer ref for the notary app SDK',
);
assert.equal(
  releaseBuildPlan.env.SDKWORK_IAM_MODE,
  'private',
  'release build plan must use server-private IAM mode by default',
);
assert.equal(
  releaseBuildPlan.env.VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL,
  'https://api.example.com',
  'release build plan must expose the platform api gateway domain to the Vite IAM wrapper',
);
assert.equal(
  releaseBuildPlan.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  'https://im.example.com',
  'release build plan must expose the application public HTTP domain to the Vite app SDK wrapper',
);
assert.equal(
  releaseBuildPlan.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL,
  'wss://im.example.com',
  'release build plan must expose the application public websocket domain to the Vite IM websocket wrapper',
);
assert.deepEqual(
  releaseBuildPlan.steps.at(-1).args,
  ['build'],
  'release build final step must build the sdkwork-im-pc Vite package from the app workspace cwd',
);
assert.equal(
  path.relative(repoRoot, releaseBuildPlan.steps.at(1).cwd).replaceAll('\\', '/'),
  'apps/sdkwork-im-pc',
  'release build install step must run from the sdkwork-im-pc workspace cwd so pnpm uses the app-local .npmrc and virtual store',
);
assert.equal(
  path.relative(repoRoot, releaseBuildPlan.steps.at(-1).cwd).replaceAll('\\', '/'),
  'apps/sdkwork-im-pc',
  'release build final step must run from the sdkwork-im-pc workspace cwd',
);
const releaseSpawnCalls = [];
releaseBuildModule.runSdkworkChatPcReleaseBuild({
  env: { SDKWORK_SHARED_SDK_MODE: 'source' },
  repoRoot,
  spawnSyncImpl(command, args, options) {
    releaseSpawnCalls.push({ args, command, options });
    return { status: 0 };
  },
});
assert.deepEqual(
  releaseSpawnCalls.map((call) => call.args),
  releaseBuildPlan.steps.map((step) => step.args),
  'release build runner must execute the audited command plan in order',
);
assert.deepEqual(
  releaseSpawnCalls.map((call) => path.relative(repoRoot, call.options.cwd).replaceAll('\\', '/')),
  releaseBuildPlan.steps.map((step) => path.relative(repoRoot, step.cwd).replaceAll('\\', '/')),
  'release build runner must execute every pnpm step from the audited cwd',
);
assert.ok(
  releaseSpawnCalls.every((call) => call.options.env.SDKWORK_SHARED_SDK_MODE === 'git'),
  'release build runner must force git-backed shared SDK mode for every pnpm command',
);
assert.ok(
  releaseSpawnCalls.every((call) => !Object.hasOwn(call.options.env, 'SDKWORK_SHARED_DRIVE_GIT_REF')),
  'release build runner must not pass an undefined Drive shared SDK ref when no release ref override is present',
);
assert.ok(
  releaseSpawnCalls.every((call) => !Object.hasOwn(call.options.env, 'SDKWORK_SHARED_NOTARY_GIT_REF')),
  'release build runner must not pass an undefined notary shared SDK ref when no release ref override is present',
);
assert.match(
  sharedSdkModeSource,
  /SDKWORK_SHARED_SDK_MODE/u,
  'shared SDK mode helper must expose SDKWORK_SHARED_SDK_MODE',
);
assert.match(
  sharedSdkGitSource,
  /SDKWORK_SHARED_SDK_GIT_PROTOCOL/u,
  'shared SDK git materializer must allow release jobs to select https or ssh transport',
);
assert.match(
  sharedSdkGitSource,
  /config\/shared-sdk-release-sources\.json/u,
  'shared SDK git materializer must load the release source map from config/shared-sdk-release-sources.json',
);
for (const sourceName of [
  'sdkwork-appbase',
  'sdkwork-core',
  'sdkwork-ui',
  'sdkwork-drive',
  'sdkwork-im-app-sdk',
  'sdkwork-im-backend-sdk',
  'sdkwork-im-sdk',
  'sdkwork-notary',
  'sdkwork-clawrouter',
  'sdkwork-birdcoder',
]) {
  const sourceConfig = sharedSdkReleaseConfig.sources?.[sourceName];
  assert.ok(sourceConfig, `release shared SDK config must define ${sourceName}`);
  assert.match(
    sourceConfig.repoUrl,
    /^(?:https:\/\/github\.com\/|git@github\.com:).+\.git$/u,
    `${sourceName} release repoUrl must be a git repository URL`,
  );
  assert.ok(
    typeof sourceConfig.ref === 'string' && sourceConfig.ref.trim().length > 0,
    `${sourceName} release config must pin a non-empty git ref`,
  );
}

const thirdPartyDependencyNames = new Set([
  '@sdkwork/sdk-common',
  '@tailwindcss/typography',
  '@tailwindcss/vite',
  '@tauri-apps/api',
  '@tauri-apps/cli',
  '@tiptap/extension-bubble-menu',
  '@tiptap/extension-floating-menu',
  '@tiptap/extension-image',
  '@tiptap/extension-link',
  '@tiptap/extension-placeholder',
  '@tiptap/pm',
  '@tiptap/react',
  '@tiptap/starter-kit',
  '@types/express',
  '@types/node',
  '@vitejs/plugin-react',
  'autoprefixer',
  'clsx',
  'dotenv',
  'emoji-picker-react',
  'esbuild',
  'express',
  'framer-motion',
  'i18next',
  'lucide-react',
  'motion',
  'qrcode',
  'react',
  'react-dom',
  'react-i18next',
  'react-markdown',
  'react-qr-code',
  'react-router',
  'react-router-dom',
  'signature_pad',
  'tailwind-merge',
  'tailwindcss',
  'tiptap-markdown',
  'tsx',
  'typescript',
  'vite',
]);

for (const sectionName of ['dependencies', 'devDependencies']) {
  for (const [name, version] of Object.entries(chatPcPackageJson[sectionName] ?? {})) {
    if (thirdPartyDependencyNames.has(name)) {
      assert.equal(
        version,
        'catalog:',
        `apps/sdkwork-im-pc ${sectionName}.${name} must use the pnpm catalog`,
      );
    }
  }
}

const packageJsonFiles = fs
  .readdirSync(path.join(repoRoot, 'apps/sdkwork-im-pc/packages'), { withFileTypes: true })
  .filter((entry) => entry.isDirectory())
  .map((entry) => path.join(repoRoot, 'apps/sdkwork-im-pc/packages', entry.name, 'package.json'))
  .filter((candidate) => fs.existsSync(candidate));
for (const packageJsonPath of packageJsonFiles) {
  const workspacePackage = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
  for (const sectionName of ['dependencies', 'devDependencies', 'peerDependencies']) {
    for (const [name, version] of Object.entries(workspacePackage[sectionName] ?? {})) {
      if (name.startsWith('@sdkwork/im-')) {
        assert.equal(
          version,
          'workspace:*',
          `${path.relative(repoRoot, packageJsonPath)} ${sectionName}.${name} must use workspace:*`,
        );
        continue;
      }
      if (thirdPartyDependencyNames.has(name)) {
        assert.equal(
          version,
          'catalog:',
          `${path.relative(repoRoot, packageJsonPath)} ${sectionName}.${name} must use catalog:`,
        );
      }
    }
  }
}

const pnpmCommand = process.platform === 'win32' ? 'pnpm.cmd' : 'pnpm';
const pnpmShell = process.platform === 'win32';
const cargoCommand = process.platform === 'win32' ? 'cargo.exe' : 'cargo';
const apiGatewayWorkspaceRoot = path.resolve(repoRoot, '..', 'sdkwork-api-cloud-gateway');

const browserPlan = createSdkworkChatPcDevPlan({
  argv: ['--target', 'browser'],
  env: {
    SDKWORK_IM_DEPLOYMENT_PROFILE: 'standalone',
    SDKWORK_IM_SERVICE_LAYOUT: 'unified-process',
  },
  repoRoot,
});
assert.equal(browserPlan.target, 'browser');
assert.deepEqual(
  browserPlan.devServer,
  {
    host: '127.0.0.1',
    port: 4176,
    url: 'http://127.0.0.1:4176',
  },
  'browser dev must default to the Sdkwork IM PC dev port instead of the retired legacy port',
);
assert.deepEqual(
  browserPlan.processes.map((entry) => entry.label),
  ['sdkwork-im-standalone-gateway', 'sdkwork-im-pc-browser'],
  'standalone unified-process browser dev must start the embedded standalone gateway and browser renderer',
);
const browserStandaloneGatewayProcess = browserPlan.processes.find(
  (entry) => entry.label === 'sdkwork-im-standalone-gateway',
);
assert.deepEqual(browserStandaloneGatewayProcess, {
  args: [
    'run',
    '-p',
    'sdkwork-im-standalone-gateway',
    '--bin',
    'sdkwork-im-standalone-gateway',
    '--',
    '--config',
    browserStandaloneGatewayProcess.env.SDKWORK_IM_STANDALONE_GATEWAY_CONFIG,
  ],
  command: cargoCommand,
  cwd: repoRoot,
  env: browserStandaloneGatewayProcess.env,
  label: 'sdkwork-im-standalone-gateway',
  shell: false,
});
assert.equal(
  browserPlan.processes[0].env.SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL,
  'http://127.0.0.1:18079',
  'standalone unified-process must collapse platform SDK traffic onto application.public-ingress',
);
assert.equal(
  browserPlan.processes[0].env.SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  'http://127.0.0.1:18079',
  'standalone unified-process must keep application HTTP on application.public-ingress',
);
assert.equal(
  browserPlan.processes[0].env.SDKWORK_IM_DATABASE_URL,
  'postgresql://sdkwork_ai_dev:sdkworkdev123@127.0.0.1:5432/sdkwork_ai_dev?sslmode=disable',
  'standalone gateway must receive the canonical PostgreSQL dev database URL by default',
);
assert.equal(
  browserPlan.processes[0].env.SDKWORK_CLAW_DATABASE_URL,
  browserPlan.processes[0].env.SDKWORK_IM_DATABASE_URL,
  'standalone gateway must receive the current compatibility database URL bridge',
);
assert.equal(
  browserPlan.processes[0].env.SDKWORK_IM_BROWSER_ORIGINS,
  'http://127.0.0.1:4176,http://localhost:4176',
  'standalone gateway must allow the selected desktop Vite origin at the gateway CORS layer',
);
assert.equal(
  browserPlan.processes[1].env.SDKWORK_IM_PC_DEV_PORT,
  '4176',
  'browser renderer must receive the selected PC renderer dev port',
);
assert.deepEqual(browserPlan.processes[1], {
  args: ['--dir', 'apps/sdkwork-im-pc', 'dev'],
  command: pnpmCommand,
  cwd: repoRoot,
  env: browserPlan.processes[1].env,
  label: 'sdkwork-im-pc-browser',
  shell: pnpmShell,
});
assert.equal(
  browserPlan.processes[1].env.VITE_SDKWORK_IAM_APP_API_BASE_URL,
  'http://127.0.0.1:18079',
  'browser renderer must point IAM traffic at the collapsed standalone gateway root',
);
assert.equal(
  browserPlan.processes[1].env.VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL,
  'http://127.0.0.1:18079',
  'browser renderer must point platform SDK traffic at the collapsed standalone gateway root',
);
assert.equal(
  browserPlan.processes[1].env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  'http://127.0.0.1:18079',
  'browser renderer must point IM HTTP traffic at application.public-ingress',
);
assert.equal(
  browserPlan.processes[1].env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL,
  'ws://127.0.0.1:18079',
  'browser renderer must point IM websocket traffic at application.public-ingress',
);

const customDriveUpstreamPlan = createSdkworkChatPcDevPlan({
  argv: ['--target', 'browser'],
  env: {
    SDKWORK_IM_DEPLOYMENT_PROFILE: 'cloud',
    SDKWORK_IM_SERVICE_LAYOUT: 'split-services',
    SDKWORK_DRIVE_APP_API_UPSTREAM: 'http://127.0.0.1:28080/',
  },
  repoRoot,
});
assert.equal(
  customDriveUpstreamPlan.processes[0].env.SDKWORK_IM_DRIVE_APP_API_UPSTREAM,
  'http://127.0.0.1:28080',
  'PC dev must allow the Drive app-api dependency upstream to be overridden for split Drive deployments',
);
assert.deepEqual(
  customDriveUpstreamPlan.processes.map((entry) => entry.label),
  ['sdkwork-im-server', 'sdkwork-im-pc-browser', 'sdkwork-api-cloud-gateway'],
  'cloud split-services dev must keep the shared gateway available for remaining foundation surfaces when Drive uses an explicit split upstream',
);
const customNotaryUpstreamPlan = createSdkworkChatPcDevPlan({
  argv: ['--target', 'browser'],
  env: {
    SDKWORK_IM_DEPLOYMENT_PROFILE: 'cloud',
    SDKWORK_IM_SERVICE_LAYOUT: 'split-services',
    SDKWORK_NOTARY_APP_API_UPSTREAM: 'http://127.0.0.1:28092/',
  },
  repoRoot,
});
assert.equal(
  customNotaryUpstreamPlan.processes[0].env.SDKWORK_IM_NOTARY_APP_API_UPSTREAM,
  'http://127.0.0.1:28092',
  'PC dev must allow the Notary app-api dependency upstream to be overridden for split Notary deployments',
);
assert.deepEqual(
  customNotaryUpstreamPlan.processes.map((entry) => entry.label),
  ['sdkwork-im-server', 'sdkwork-im-pc-browser', 'sdkwork-api-cloud-gateway'],
  'cloud split-services dev must keep the shared gateway available for remaining foundation surfaces when Notary uses an explicit split upstream',
);
const customCatalogUpstreamPlan = createSdkworkChatPcDevPlan({
  argv: ['--target', 'browser'],
  env: {
    SDKWORK_IM_DEPLOYMENT_PROFILE: 'cloud',
    SDKWORK_IM_SERVICE_LAYOUT: 'split-services',
    SDKWORK_CATALOG_APP_API_UPSTREAM: 'http://127.0.0.1:28094/',
  },
  repoRoot,
});
assert.equal(
  customCatalogUpstreamPlan.processes[0].env.SDKWORK_IM_CATALOG_APP_API_UPSTREAM,
  'http://127.0.0.1:28094',
  'PC dev must allow the catalog app-api dependency upstream to be overridden for split catalog deployments',
);
assert.deepEqual(
  customCatalogUpstreamPlan.processes.map((entry) => entry.label),
  ['sdkwork-im-server', 'sdkwork-im-pc-browser', 'sdkwork-api-cloud-gateway'],
  'cloud split-services dev must keep the shared gateway available for remaining foundation surfaces when catalog uses an explicit split upstream',
);
assert.equal(
  createSdkworkChatBrowserOrigins({ port: 4188 }),
  'http://127.0.0.1:4188,http://localhost:4188',
  'browser origin helper must derive CORS origins from the selected dev port',
);

const shiftedDevPort = await resolveAvailableSdkworkChatPcDevPort({
  env: { SDKWORK_IM_PC_DEV_PORT: '4176' },
  isPortAvailable: async (port) => port >= 4178,
});
assert.equal(
  shiftedDevPort,
  4178,
  'dev port resolver must skip occupied ports and return the next available port',
);

const shiftedPortPlan = createSdkworkChatPcDevPlan({
  argv: ['--target', 'browser'],
  devServerPort: shiftedDevPort,
  env: {
    SDKWORK_IM_DEPLOYMENT_PROFILE: 'standalone',
    SDKWORK_IM_SERVICE_LAYOUT: 'unified-process',
  },
  repoRoot,
});
assert.equal(
  shiftedPortPlan.processes[0].env.SDKWORK_IM_BROWSER_ORIGINS,
  'http://127.0.0.1:4178,http://localhost:4178',
  'standalone gateway CORS origins must follow the resolved fallback dev port',
);
assert.equal(
  shiftedPortPlan.processes[1].env.SDKWORK_IM_PC_DEV_PORT,
  '4178',
  'browser renderer env must follow the resolved fallback dev port',
);

const postgresDatabaseConfig = resolveSdkworkImSharedDatabaseConfig({
  env: {
    SDKWORK_CLAW_DATABASE_URL: 'postgresql://sdkwork_ai_dev:sdkworkdev123@127.0.0.1:15432/sdkwork_ai_dev?sslmode=disable',
  },
  repoRoot,
});
assert.equal(postgresDatabaseConfig.kind, 'postgresql');
assert.equal(
  postgresDatabaseConfig.env.SDKWORK_CLAW_DATABASE_URL,
  'postgresql://sdkwork_ai_dev:sdkworkdev123@127.0.0.1:15432/sdkwork_ai_dev?sslmode=disable',
  'shared DB helper must pass PostgreSQL URLs to the Rust server unchanged',
);
assert.equal(postgresDatabaseConfig.postgres.username, 'sdkwork_ai_dev');
assert.equal(postgresDatabaseConfig.postgres.password, 'sdkworkdev123');
assert.equal(postgresDatabaseConfig.postgres.database, 'sdkwork_ai_dev');

const postgresSplitDatabaseConfig = resolveSdkworkImSharedDatabaseConfig({
  env: {
    SDKWORK_CLAW_DATABASE_PROVIDER: 'postgresql',
    SDKWORK_CLAW_DATABASE_HOST: '127.0.0.1',
    SDKWORK_CLAW_DATABASE_PORT: '15432',
    SDKWORK_CLAW_DATABASE_NAME: 'sdkwork_ai_dev',
    SDKWORK_CLAW_DATABASE_USERNAME: 'sdkwork_ai_dev',
    SDKWORK_CLAW_DATABASE_PASSWORD: 'chat pass',
    SDKWORK_CLAW_DATABASE_SSLMODE: 'disable',
    SDKWORK_CLAW_DATABASE_MAX_CONNECTIONS: '12',
  },
  repoRoot,
});
assert.equal(postgresSplitDatabaseConfig.kind, 'postgresql');
assert.equal(
  postgresSplitDatabaseConfig.env.SDKWORK_CLAW_DATABASE_URL,
  'postgresql://sdkwork_ai_dev:chat%20pass@127.0.0.1:15432/sdkwork_ai_dev?sslmode=disable',
  'shared DB helper must assemble PostgreSQL URLs from split database fields',
);
assert.equal(
  postgresSplitDatabaseConfig.env.SDKWORK_CLAW_DATABASE_MAX_CONNECTIONS,
  '12',
  'shared DB helper must pass split-field PostgreSQL max connection settings to the Rust server',
);
assert.equal(postgresSplitDatabaseConfig.postgres.username, 'sdkwork_ai_dev');
assert.equal(postgresSplitDatabaseConfig.postgres.password, 'chat pass');
assert.equal(postgresSplitDatabaseConfig.postgres.database, 'sdkwork_ai_dev');
assert.throws(
  () => resolveSdkworkImSharedDatabaseConfig({
    env: {
      SDKWORK_CLAW_DATABASE_PROVIDER: 'postgres',
      SDKWORK_CLAW_DATABASE_HOST: '127.0.0.1',
      SDKWORK_CLAW_DATABASE_NAME: 'sdkwork_ai_dev',
      SDKWORK_CLAW_DATABASE_USERNAME: 'sdkwork_ai_dev',
    },
    repoRoot,
  }),
  /SDKWORK_IM_DATABASE_PASSWORD/u,
  'split-field PostgreSQL configuration must require an explicit password',
);
assert.throws(
  () => resolveSdkworkImSharedDatabaseConfig({
    env: {
      SDKWORK_CLAW_DATABASE_PROVIDER: 'mysql',
      SDKWORK_CLAW_DATABASE_HOST: '127.0.0.1',
      SDKWORK_CLAW_DATABASE_NAME: 'sdkwork_ai_dev',
      SDKWORK_CLAW_DATABASE_USERNAME: 'sdkwork_ai_dev',
      SDKWORK_CLAW_DATABASE_PASSWORD: 'sdkworkdev123',
    },
    repoRoot,
  }),
  /unsupported Sdkwork IM database engine/u,
  'shared DB helper must reject unsupported split-field database engines instead of silently falling back to SQLite',
);

const postgresUrlPrecedenceConfig = resolveSdkworkImSharedDatabaseConfig({
  env: {
    SDKWORK_CLAW_DATABASE_URL: 'postgresql://url_user:url_pass@127.0.0.1:25432/url_db?sslmode=require',
    SDKWORK_CLAW_DATABASE_PROVIDER: 'postgresql',
    SDKWORK_CLAW_DATABASE_HOST: '127.0.0.1',
    SDKWORK_CLAW_DATABASE_PORT: '15432',
    SDKWORK_CLAW_DATABASE_NAME: 'split_db',
    SDKWORK_CLAW_DATABASE_USERNAME: 'split_user',
    SDKWORK_CLAW_DATABASE_PASSWORD: 'split_pass',
    SDKWORK_CLAW_DATABASE_SSLMODE: 'disable',
  },
  repoRoot,
});
assert.equal(
  postgresUrlPrecedenceConfig.env.SDKWORK_CLAW_DATABASE_URL,
  'postgresql://url_user:url_pass@127.0.0.1:25432/url_db?sslmode=require',
  'explicit SDKWORK_CLAW_DATABASE_URL must take precedence over split PostgreSQL fields',
);
assert.doesNotMatch(
  sharedDatabaseSource,
  /SPRING_DATASOURCE|org\.sqlite\.JDBC|org\.postgresql\.Driver/u,
  'shared DB helper must not carry Spring datasource configuration in the Rust server architecture',
);
for (const requiredName of [
  'SDKWORK_IM_DATABASE_ENGINE=postgresql',
  'SDKWORK_IM_DATABASE_HOST=127.0.0.1',
  'SDKWORK_IM_DATABASE_PORT=5432',
  'SDKWORK_CLAW_DATABASE_NAME=sdkwork_ai_dev',
  'SDKWORK_CLAW_DATABASE_USERNAME=sdkwork_ai_dev',
  'SDKWORK_CLAW_DATABASE_PASSWORD=sdkworkdev123',
  'SDKWORK_IM_DATABASE_SSL_MODE=disable',
  'SDKWORK_IM_DATABASE_MAX_CONNECTIONS=10',
]) {
  assert.ok(
    postgresEnvExampleSource.includes(requiredName),
    `.env.postgres.example must document ${requiredName}`,
  );
}
assert.ok(
  postgresDatabaseConfigIndexSource.includes('./开发环境PostgreSQL数据库配置教程.md')
    && postgresDatabaseConfigIndexSource.includes('./线上环境PostgreSQL数据库配置教程.md')
    && postgresDatabaseConfigIndexSource.includes('pnpm dev')
    && postgresDatabaseConfigIndexSource.includes('pnpm dev:desktop')
    && postgresDatabaseConfigIndexSource.includes('pnpm dev:browser:sqlite')
    && postgresDatabaseConfigIndexSource.includes('/etc/sdkwork/chat/chat.toml')
    && postgresDatabaseConfigIndexSource.includes('/etc/sdkwork/chat/database.secret')
    && postgresDatabaseConfigIndexSource.includes('~/.sdkwork/chat/data/chat.sqlite'),
  'PostgreSQL database configuration index must link the environment-specific development and production guides',
);
assert.ok(
  postgresDevelopmentGuideSource.includes('pnpm dev')
    && postgresDevelopmentGuideSource.includes('pnpm dev:desktop')
    && postgresDevelopmentGuideSource.includes('pnpm dev:browser:postgres')
    && postgresDevelopmentGuideSource.includes('pnpm dev:desktop:postgres')
    && postgresDevelopmentGuideSource.includes('pnpm dev:browser:sqlite')
    && postgresDevelopmentGuideSource.includes('pnpm dev:desktop:sqlite')
    && postgresDevelopmentGuideSource.includes('.env.postgres')
    && postgresDevelopmentGuideSource.includes('SDKWORK_IM_DATABASE_ENGINE=postgresql')
    && postgresDevelopmentGuideSource.includes('SDKWORK_IM_DATABASE_SSL_MODE=disable')
    && postgresDevelopmentGuideSource.includes('pnpm dev:desktop 默认使用 PostgreSQL')
    && postgresDevelopmentGuideSource.includes('Copy-Item .env.postgres.example .env.postgres'),
  'development PostgreSQL guide must document local env profile setup and both dev startup commands',
);
assert.ok(
  postgresProductionGuideSource.includes('/etc/sdkwork/chat/')
    && postgresProductionGuideSource.includes('chat.toml')
    && postgresProductionGuideSource.includes('server.env')
    && postgresProductionGuideSource.includes('postgresql.yaml')
    && postgresProductionGuideSource.includes('database.secret')
    && postgresProductionGuideSource.includes('Windows Service')
    && postgresProductionGuideSource.includes('PGPASSWORD='),
  'production PostgreSQL guide must document config-root, password-file, and service deployment workflow',
);

const tempEnvDir = path.join(repoRoot, '.runtime', 'dev-command-test');
fs.mkdirSync(tempEnvDir, { recursive: true });
const tempPostgresEnvFile = path.join(tempEnvDir, 'postgres.env');
fs.writeFileSync(
  tempPostgresEnvFile,
  [
    'SDKWORK_IM_DATABASE_ENGINE=postgresql',
    'SDKWORK_IM_DATABASE_HOST=127.0.0.1',
    'SDKWORK_IM_DATABASE_PORT=15433',
    'SDKWORK_IM_DATABASE_NAME=env_file_db',
    'SDKWORK_IM_DATABASE_USERNAME=env_file_user',
    'SDKWORK_IM_DATABASE_PASSWORD=env file pass',
    'SDKWORK_IM_DATABASE_SSL_MODE=disable',
    'SDKWORK_IM_DATABASE_MAX_CONNECTIONS=15',
    '',
  ].join('\n'),
);
const postgresEnvFilePlan = createSdkworkChatPcDevPlan({
  argv: ['--target', 'browser', '--dev-env-file', tempPostgresEnvFile],
  env: {},
  repoRoot,
});
assert.equal(
  postgresEnvFilePlan.processes[0].env.SDKWORK_IM_DATABASE_URL,
  'postgresql://env_file_user:env%20file%20pass@127.0.0.1:15433/env_file_db?sslmode=disable',
  'dev command must load --dev-env-file and pass the canonical assembled PostgreSQL URL to the unified server',
);
assert.equal(
  postgresEnvFilePlan.processes[0].env.SDKWORK_CLAW_DATABASE_URL,
  'postgresql://env_file_user:env%20file%20pass@127.0.0.1:15433/env_file_db?sslmode=disable',
  'dev command must keep the current Rust-compatible PostgreSQL URL bridge',
);
assert.equal(
  postgresEnvFilePlan.processes[0].env.SDKWORK_IM_DATABASE_MAX_CONNECTIONS,
  '15',
  'dev command must load canonical PostgreSQL max connections from --dev-env-file',
);

const sqliteBrowserPlan = createSdkworkChatPcDevPlan({
  argv: ['--target', 'browser', '--database', 'sqlite'],
  env: {},
  repoRoot,
});
assert.match(
  sqliteBrowserPlan.processes[0].env.SDKWORK_IM_DATABASE_URL,
  /sqlite:\/\/.*[/\\]\.sdkwork[/\\]chat[/\\]data[/\\]chat\.sqlite$/u,
  'pnpm dev:browser:sqlite must explicitly use the Sdkwork IM user-private SQLite database URL',
);

const desktopPlan = createSdkworkChatPcDevPlan({
  argv: ['--target', 'desktop'],
  env: {
    SDKWORK_IM_DEPLOYMENT_PROFILE: 'standalone',
    SDKWORK_IM_SERVICE_LAYOUT: 'unified-process',
  },
  repoRoot,
});
assert.equal(desktopPlan.target, 'desktop');
assert.deepEqual(
  desktopPlan.processes.map((entry) => entry.label),
  ['sdkwork-im-standalone-gateway', 'sdkwork-im-pc-desktop'],
  'standalone unified-process desktop dev must start the embedded standalone gateway and Tauri desktop process',
);
assert.deepEqual(desktopPlan.processes[1], {
  args: ['--dir', 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-desktop', 'dev:desktop'],
  command: pnpmCommand,
  cwd: repoRoot,
  env: desktopPlan.processes[1].env,
  label: 'sdkwork-im-pc-desktop',
  shell: pnpmShell,
});
assert.notDeepEqual(
  desktopPlan.processes[1].args,
  ['--dir', 'apps/sdkwork-im-pc', 'dev:desktop'],
  'desktop dev must run the dedicated desktop package instead of the root web package',
);
assert.equal(
  desktopPlan.processes[1].env.VITE_SDKWORK_IAM_APP_API_BASE_URL,
  'http://127.0.0.1:18079',
  'desktop renderer must point IAM traffic at the collapsed standalone gateway root',
);
assert.equal(
  desktopPlan.processes[1].env.VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL,
  'http://127.0.0.1:18079',
  'desktop renderer must point platform SDK traffic at the collapsed standalone gateway root',
);
assert.equal(
  desktopPlan.processes[1].env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  'http://127.0.0.1:18079',
  'desktop renderer must point IM HTTP traffic at application.public-ingress',
);
assert.equal(
  desktopPlan.processes[0].env.SDKWORK_IM_DATABASE_ENGINE,
  'postgresql',
  'desktop dev must default to PostgreSQL for the standalone development stack',
);
assert.match(
  desktopPlan.processes[0].env.SDKWORK_IM_DATABASE_URL,
  /^postgresql:\/\//u,
  'desktop dev must use the PostgreSQL development database URL by default',
);
assert.equal(
  desktopPlan.processes[0].env.SDKWORK_CLAW_DATABASE_URL,
  desktopPlan.processes[0].env.SDKWORK_IM_DATABASE_URL,
  'desktop dev must bridge the canonical PostgreSQL URL to the current Rust-compatible env name',
);
assert.ok(
  !('SDKWORK_IM_PLATFORM_API_GATEWAY_MANAGED_EXTERNALLY' in desktopPlan.processes[0].env),
  'standalone unified-process desktop dev must not mark platform gateway as externally managed',
);

const spawned = [];
function createFakeChild() {
  const child = new EventEmitter();
  child.stdout = new EventEmitter();
  child.stderr = new EventEmitter();
  child.exitCode = null;
  child.signalCode = null;
  child.kill = () => {};
  return child;
}

await runSdkworkChatPcDev({
  argv: ['--target', 'desktop'],
  env: {
    SDKWORK_IM_DEPLOYMENT_PROFILE: 'standalone',
    SDKWORK_IM_SERVICE_LAYOUT: 'unified-process',
  },
  findAvailableDevPort: async () => 4179,
  resolveServerBindEnv: async ({ env }) => ({
    bindAddr: '127.0.0.1:18081',
    env: {
      ...env,
      SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND: '127.0.0.1:18081',
      SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: 'http://127.0.0.1:18081',
    },
    portChanged: true,
  }),
  repoRoot,
  spawnImpl(command, args, options) {
    spawned.push({ command, args, options });
    return createFakeChild();
  },
  stdout: { write() {} },
  stderr: { write() {} },
});

assert.equal(
  spawned.length,
  2,
  'standalone unified-process desktop dev runner must spawn standalone gateway and desktop renderer processes',
);
assert.equal(
  spawned[0].options.env.SDKWORK_IM_BROWSER_ORIGINS,
  'http://127.0.0.1:4179,http://localhost:4179',
  'dev runner must pass the resolved available port to the standalone gateway CORS origins',
);
assert.equal(
  spawned[1].options.env.SDKWORK_IM_PC_DEV_PORT,
  '4179',
  'dev runner must pass the resolved available port to the selected renderer target',
);
assert.equal(
  spawned[0].options.env.SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND,
  '127.0.0.1:18081',
  'dev runner must pass the resolved application ingress bind to the standalone gateway process',
);
assert.equal(
  spawned[0].options.env.SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  'http://127.0.0.1:18081',
  'dev runner must pass the resolved application HTTP URL to the standalone gateway process',
);
assert.equal(
  spawned[0].options.env.SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL,
  'http://127.0.0.1:18081',
  'dev runner must collapse platform SDK traffic onto the resolved application ingress bind',
);
assert.ok(
  !('SDKWORK_IM_PLATFORM_API_GATEWAY_MANAGED_EXTERNALLY' in spawned[0].options.env),
  'standalone unified-process dev runner must not mark platform gateway as externally managed',
);
assert.ok(
  !('SDKWORK_IM_DRIVE_APP_API_UPSTREAM' in spawned[0].options.env)
    && !('SDKWORK_IM_NOTARY_APP_API_UPSTREAM' in spawned[0].options.env)
    && !('SDKWORK_IM_CATALOG_APP_API_UPSTREAM' in spawned[0].options.env),
  'standalone unified-process dev runner must not pass split foundation upstreams',
);
assert.equal(
  spawned[1].options.env.VITE_SDKWORK_IAM_APP_API_BASE_URL,
  'http://127.0.0.1:18081',
  'dev runner must point IAM traffic at the collapsed standalone gateway root',
);
assert.equal(
  spawned[1].options.env.VITE_SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL,
  'http://127.0.0.1:18081',
  'dev runner must point platform SDK traffic at the collapsed standalone gateway root',
);
assert.equal(
  spawned[1].options.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  'http://127.0.0.1:18081',
  'dev runner must point IM HTTP traffic at the resolved application ingress when 18079 is unavailable',
);
assert.equal(
  spawned[1].options.env.VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL,
  'ws://127.0.0.1:18081',
  'dev runner must point IM websocket traffic at the resolved application ingress when 18079 is unavailable',
);
assert.deepEqual(
  spawned[0].args,
  [
    'run',
    '-p',
    'sdkwork-im-standalone-gateway',
    '--bin',
    'sdkwork-im-standalone-gateway',
    '--',
    '--config',
    spawned[0].options.env.SDKWORK_IM_STANDALONE_GATEWAY_CONFIG,
  ],
  'dev runner must spawn the embedded standalone gateway for standalone unified-process profiles',
);
assert.equal(
  spawned[0].options.cwd,
  repoRoot,
  'standalone gateway must run from the sdkwork-im repository root',
);
assert.deepEqual(
  spawned.map((entry) => entry.options.shell),
  [false, pnpmShell],
  'dev runner must pass the planned shell mode to every child process',
);
const devRunnerSource = fs.readFileSync(
  path.join(repoRoot, 'scripts/lib/im-pc-dev.mjs'),
  'utf8',
);
assert.match(
  devRunnerSource,
  /createSdkworkImServerCargoEnv/u,
  'dev runner plan must use the shared cargo target isolation helper',
);
assert.match(
  devRunnerSource,
  /resolveSdkworkImServerBindEnv/u,
  'dev runner must resolve the shared gateway bind before spawning server and renderer children',
);
assert.match(
  devRunnerSource,
  /resolveImProductSiteDirEnv/u,
  'dev runner must resolve admin/portal site dirs before spawning the standalone gateway',
);
assert.match(
  devRunnerSource,
  /resolveRealtimeClusterDevEnv/u,
  'dev runner must inject development realtime cluster defaults when redis route store is configured',
);
assert.ok(
  devRunnerSource.includes('terminateProcessTree')
    && devRunnerSource.includes('taskkill')
    && devRunnerSource.includes('/T')
    && devRunnerSource.includes('/F'),
  'dev runner shutdown must terminate Windows process trees so cargo grandchildren cannot keep target/debug/sdkwork-im-server.exe locked',
);

const chatPcAppRoot = path.join(repoRoot, 'apps/sdkwork-im-pc');
const sdkworkUiDependencyPackageRoot = path.join(
  repoRoot,
  '../sdkwork-ui/sdkwork-ui-pc-react',
);
assert.equal(
  resolveSdkworkUiPackageRoot(chatPcAppRoot),
  sdkworkUiDependencyPackageRoot,
  'sdkwork ui runtime helper must resolve the source-linked UI package from the sibling workspace',
);
const sdkworkUiRuntimeFiles = new Set([
  'pnpm-workspace.yaml',
  '../sdkwork-ui/sdkwork-ui-pc-react/package.json',
  '../sdkwork-ui/sdkwork-ui-pc-react/dist/index.js',
  '../sdkwork-ui/sdkwork-ui-pc-react/dist/theme.js',
  '../sdkwork-ui/sdkwork-ui-pc-react/dist/components-ui.js',
  '../sdkwork-ui/sdkwork-ui-pc-react/dist/ui-feedback.js',
  '../sdkwork-ui/sdkwork-ui-pc-react/dist/patterns-app-shell.js',
  '../sdkwork-ui/sdkwork-ui-pc-react/dist/patterns-desktop-shell.js',
  '../sdkwork-ui/sdkwork-ui-pc-react/dist/sdkwork-ui.css',
]);
const sdkworkUiInstallCalls = [];
assert.equal(
  ensureSdkworkUiDist({
    appRoot: chatPcAppRoot,
    fileExists(filePath) {
      return sdkworkUiRuntimeFiles.has(
        path.relative(repoRoot, path.resolve(filePath)).replaceAll('\\', '/'),
      );
    },
    runProcess(command, args, options) {
      sdkworkUiInstallCalls.push({ args, command, cwd: options.cwd });
      return { status: 0 };
    },
  }),
  sdkworkUiDependencyPackageRoot,
  'sdkwork ui runtime helper must prepare the sibling source dependency root',
);
assert.ok(
  sdkworkUiInstallCalls.some((call) => (
    call.args.join(' ').includes('install')
      && path.resolve(call.cwd) === repoRoot
  )),
  'sdkwork ui runtime helper must install missing dependency packages from the repository workspace root when pnpm-workspace.yaml is present',
);
const appRequire = createRequire(path.join(chatPcAppRoot, 'package.json'));
const { createServer } = await import(pathToFileURL(appRequire.resolve('vite')).href);
const viteServer = await createServer({
  configFile: path.join(chatPcAppRoot, 'vite.config.ts'),
  logLevel: 'silent',
  root: chatPcAppRoot,
  server: {
    hmr: false,
    middlewareMode: true,
  },
});
try {
  const mainEntry = path.join(chatPcAppRoot, 'src/main.tsx');
  const jsxDevRuntime = await viteServer.pluginContainer.resolveId('react/jsx-dev-runtime', mainEntry);
  assert.ok(jsxDevRuntime, 'Vite must resolve the React JSX dev runtime import injected for TSX dev builds');
  const resolvedJsxDevRuntimePath = path.normalize(jsxDevRuntime.id.split('?')[0]);
  assert.doesNotMatch(
    resolvedJsxDevRuntimePath,
    /react[\\/]index\.js[\\/]jsx-dev-runtime$/u,
    'Vite must not append react/jsx-dev-runtime to the bare react/index.js alias',
  );
  assert.ok(
    [
      path.normalize(path.join(chatPcAppRoot, 'node_modules/react/jsx-dev-runtime.js')),
      path.normalize(path.join(chatPcAppRoot, 'node_modules/.vite/deps/react_jsx-dev-runtime.js')),
    ].includes(resolvedJsxDevRuntimePath),
    'Vite must resolve react/jsx-dev-runtime from the chat PC dependency root instead of appending it to react/index.js',
  );
  const sdkworkUiContextMenuSource = path.join(
    sdkworkUiDependencyPackageRoot,
    'src/components/ui/overlays/context-menu.tsx',
  );
  const resolvedRadixContextMenu = await viteServer.pluginContainer.resolveId(
    '@radix-ui/react-context-menu',
    sdkworkUiContextMenuSource,
  );
  assert.ok(
    resolvedRadixContextMenu,
    'Vite must resolve Radix context menu imports from the source-linked SDKWork UI package',
  );
} finally {
  await viteServer.close();
}

assert.throws(
  () => createSdkworkChatPcDevPlan({ argv: ['--target', 'mobile'], env: {}, repoRoot }),
  /Unsupported sdkwork-im-pc dev target/u,
  'dev command must reject unsupported targets',
);
assert.throws(
  () => createSdkworkChatPcDevPlan({ argv: ['--target', 'browser', '--database', 'mysql'], env: {}, repoRoot }),
  /Unsupported sdkwork-im-pc dev database/u,
  'dev command must reject unsupported database profiles',
);

console.log('sdkwork-im-pc root dev command contract passed');
