import assert from 'node:assert/strict';
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
  'sdks/sdkwork-im-app-sdk/sdkwork-im-app-sdk-typescript',
  'sdks/sdkwork-im-backend-sdk/sdkwork-im-backend-sdk-typescript',
  '../sdkwork-iam/sdks/sdkwork-iam-app-sdk/sdkwork-iam-app-sdk-typescript',
  '../sdkwork-voice/sdks/sdkwork-voice-app-sdk/sdkwork-voice-app-sdk-typescript',
  '../sdkwork-agents/sdks/sdkwork-agents-app-sdk/sdkwork-agents-app-sdk-typescript',
  '../sdkwork-skills/sdks/sdkwork-skills-app-sdk/sdkwork-skills-app-sdk-typescript',
  '../sdkwork-knowledgebase/sdks/sdkwork-knowledgebase-backend-sdk/sdkwork-knowledgebase-backend-sdk-typescript',
]) {
  assert.ok(
    workspaceYaml.includes(composedFacade),
    `pnpm-workspace.yaml must include composed consumer facade ${composedFacade}`,
  );
}

const rootPackage = readJsonExists('package.json');
const overrides = rootPackage.pnpm?.overrides ?? {};
for (const overrideKey of [
  '@sdkwork/agents-app-sdk',
  '@sdkwork/voice-app-sdk',
  '@sdkwork/skills-app-sdk',
  '@sdkwork/im-sdk-generated',
  '@sdkwork/knowledgebase-backend-sdk',
  '@sdkwork-internal/im-backend-api-generated',
]) {
  assert.equal(
    overrides[overrideKey],
    'workspace:*',
    `package.json pnpm.overrides must map ${overrideKey} to workspace:*`,
  );
}

const imBackendTransport = readJsonExists(
  'sdks/sdkwork-im-backend-sdk/sdkwork-im-backend-sdk-typescript/generated/server-openapi/package.json',
);
assert.equal(
  imBackendTransport.name,
  '@sdkwork-internal/im-backend-api-generated',
  'IM backend transport package name must match workspace consumer override',
);

const imOpenTransport = readJsonExists(
  'sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/generated/server-openapi/package.json',
);
assert.equal(
  imOpenTransport.name,
  '@sdkwork/im-sdk-generated',
  'IM open transport package name must match composed facade dependency',
);

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

process.stdout.write('sdkwork-im monorepo frozen install standard passed\n');
