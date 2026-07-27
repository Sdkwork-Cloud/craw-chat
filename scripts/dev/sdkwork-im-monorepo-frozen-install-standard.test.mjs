import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function readExists(relativePath) {
  const absolutePath = path.join(repoRoot, ...relativePath.split('/'));
  assert.ok(fs.existsSync(absolutePath), `expected file ${relativePath}`);
  return fs.readFileSync(absolutePath, 'utf8');
}

function readJsonExists(relativePath) {
  return JSON.parse(readExists(relativePath));
}

const workspaceYaml = readExists('pnpm-workspace.yaml');
for (const composedFacade of [
  'sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript',
  'sdks/sdkwork-im-app-sdk/sdkwork-im-app-sdk-typescript',
  'sdks/sdkwork-im-backend-sdk/sdkwork-im-backend-sdk-typescript',
  '../sdkwork-iam/sdks/sdkwork-iam-app-sdk/sdkwork-iam-app-sdk-typescript',
  '../sdkwork-voice/sdks/sdkwork-voice-app-sdk/sdkwork-voice-app-sdk-typescript',
  '../sdkwork-agents/sdks/sdkwork-agents-app-sdk/sdkwork-agents-app-sdk-typescript',
  '../sdkwork-skills/sdks/sdkwork-skills-app-sdk/sdkwork-skills-app-sdk-typescript',
  '../sdkwork-knowledgebase/sdks/sdkwork-knowledgebase-backend-sdk/sdkwork-knowledgebase-backend-sdk-typescript',
  '../sdkwork-account/sdks/sdkwork-account-app-sdk/sdkwork-account-app-sdk-typescript',
  '../sdkwork-payment/sdks/sdkwork-payment-app-sdk/sdkwork-payment-app-sdk-typescript',
  '../sdkwork-promotion/sdks/sdkwork-promotion-app-sdk/sdkwork-promotion-app-sdk-typescript',
  '../sdkwork-prompts/sdks/sdkwork-prompts-app-sdk/sdkwork-prompts-app-sdk-typescript',
]) {
  assert.ok(
    workspaceYaml.includes(composedFacade),
    `pnpm-workspace.yaml must include composed consumer facade ${composedFacade}`,
  );
}

assert.ok(
  workspaceYaml.includes('../sdkwork-prompts/sdks/sdkwork-prompts-app-sdk/generated/server-openapi'),
  'pnpm-workspace.yaml must include the Prompts generated transport required by its composed facade',
);

const rootPackage = readJsonExists('package.json');
const overrides = rootPackage.pnpm?.overrides ?? {};
for (const overrideKey of [
  '@sdkwork/agents-app-sdk',
  '@sdkwork/voice-app-sdk',
  '@sdkwork/skills-app-sdk',
  '@sdkwork/im-sdk',
  '@sdkwork/im-app-sdk',
  '@sdkwork/im-backend-sdk',
  '@sdkwork/knowledgebase-backend-sdk',
]) {
  assert.equal(
    overrides[overrideKey],
    'workspace:*',
    `package.json pnpm.overrides must map ${overrideKey} to workspace:*`,
  );
}

for (const forbiddenOverrideKey of [
  '@sdkwork/im-sdk-generated',
  '@sdkwork-internal/im-sdk-generated',
  '@sdkwork-internal/im-app-api-generated',
  '@sdkwork-internal/im-backend-api-generated',
  'sdkwork-im-sdk-generated-typescript',
  'sdkwork-im-app-sdk-generated-typescript',
  'sdkwork-im-backend-sdk-generated-typescript',
]) {
  assert.equal(
    overrides[forbiddenOverrideKey],
    undefined,
    `package.json pnpm.overrides must not expose generated transport ${forbiddenOverrideKey}`,
  );
}

for (const [label, manifestPath, expectedName] of [
  [
    'IM open transport',
    'sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/generated/server-openapi/package.json',
    'sdkwork-im-sdk-generated-typescript',
  ],
  [
    'IM app transport',
    'sdks/sdkwork-im-app-sdk/sdkwork-im-app-sdk-typescript/generated/server-openapi/package.json',
    'sdkwork-im-app-sdk-generated-typescript',
  ],
  [
    'IM backend transport',
    'sdks/sdkwork-im-backend-sdk/sdkwork-im-backend-sdk-typescript/generated/server-openapi/package.json',
    'sdkwork-im-backend-sdk-generated-typescript',
  ],
]) {
  const transportManifest = readJsonExists(manifestPath);
  assert.equal(
    transportManifest.name,
    expectedName,
    `${label} package name must be the generated transport id, not a consumer workspace package`,
  );
}

const commercialReadiness = readExists('scripts/release/commercial-readiness.mjs');
assert.match(
  commercialReadiness,
  /id:\s*'pc-install'[\s\S]*--frozen-lockfile[\s\S]*--lockfile-only/,
  'commercial-readiness must keep a non-destructive frozen lockfile check as the first gate',
);
assert.doesNotMatch(
  commercialReadiness,
  /id:\s*'pc-install'[\s\S]*args:\s*\[\s*'install',\s*'--frozen-lockfile',\s*'--ignore-scripts'\s*\]/,
  'commercial-readiness must not run a full pnpm install that can purge or rewrite node_modules',
);

const commercialGates = readExists('.github/workflows/im-commercial-gates.yml');
assert.ok(
  commercialGates.includes('sdkwork-im-monorepo-frozen-install-standard.test.mjs'),
  'im-commercial-gates.yml must run monorepo frozen install standard test',
);

const installArgs = ['install', '--frozen-lockfile', '--lockfile-only', '--ignore-scripts'];
const pnpmExecPath = process.env.npm_execpath;
const pnpmCommand = pnpmExecPath
  ? process.execPath
  : process.platform === 'win32'
    ? 'cmd.exe'
    : 'pnpm';
const pnpmArgs = pnpmExecPath
  ? [pnpmExecPath, ...installArgs]
  : process.platform === 'win32'
    ? ['/d', '/s', '/c', 'pnpm', ...installArgs]
    : installArgs;
const frozenInstall = spawnSync(
  pnpmCommand,
  pnpmArgs,
  {
    cwd: repoRoot,
    encoding: 'utf8',
    env: { ...process.env, CI: 'true' },
    timeout: 120_000,
  },
);
assert.ifError(frozenInstall.error);
assert.equal(
  frozenInstall.status,
  0,
  `real frozen lockfile verification failed:\n${frozenInstall.stdout}\n${frozenInstall.stderr}`,
);

process.stdout.write('sdkwork-im monorepo frozen install standard passed\n');
