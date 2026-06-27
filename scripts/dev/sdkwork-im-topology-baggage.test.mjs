#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

const scanRoots = [
  'adapters',
  'artifacts',
  'crates',
  'services',
  'tools',
  'scripts',
  'configs',
  'deployments',
  'apps',
  'bin',
  'docs/sites',
  'docs/部署',
  'docs/架构',
  'docs/step',
  'docs/review',
  'docs/architecture',
  'specs',
  'sdks',
  'README.md',
  'AGENTS.md',
  '.env.postgres.example',
];

const skipPathFragments = [
  '/target/',
  '/node_modules/',
  '/generated/',
  '/vendor/',
  'sdkwork-im-topology-baggage.test.mjs',
  'migrate-step11-artifact-profiles.mjs',
  'migrate-topology-v2-baggage.mjs',
  'topology-greenfield.md',
  'docs/superpowers/',
  'docs/architecture/tech/TECH-changelog.md',
  'docs/architecture/tech/TECH-2026-04-',
  'docs/architecture/tech/TECH-2026-05-',
  'docs/architecture/tech/TECH-2026-06-12-',
  'docs/architecture/tech/TECH-2026-06-15-',
];

const allowlistPathFragments = [
  'specs/topology.spec.json',
  'docs/sites/deployment/local-binary.md',
  'docs/sites/deployment/docker.md',
  'docs/sites/deployment/profiles-and-env.md',
  'docs/sites/architecture/overview.md',
  'docs/sites/architecture/runtime-topology.md',
  'docs/架构/README.md',
  'docs/step/README.md',
  'docs/review/README.md',
  'docs/architecture/decisions/README.md',
];

const bannedPatterns = [
  { id: 'local-minimal-node', pattern: /(?<![\w-])local-minimal-node(?![\w-])/u },
  { id: 'local-minimal profile', pattern: /(?<![\w-])local-minimal(?![\w-])/u },
  { id: 'local-default profile', pattern: /(?<![\w-])local-default(?![\w-])/u },
  { id: 'install-local script', pattern: /\binstall-local\b/u },
  { id: 'start-local script', pattern: /\bstart-local\b/u },
  { id: 'deploy-local script', pattern: /\bdeploy-local\b/u },
  { id: 'run-local-minimal script', pattern: /\brun-local-minimal\b/u },
  { id: 'legacy bind env', pattern: /SDKWORK_IM_BIND_ADDR/u },
  { id: 'legacy dev port', pattern: /127\.0\.0\.1:18090/u },
  { id: 'legacy runtime dir', pattern: /\.runtime\/local-/u },
  {
    id: 'local rtc sdk path',
    pattern: /(?<!(?:\.\.\/sdkwork-rtc\/|sdkwork-rtc\/))(?<![\w-])sdks\/sdkwork-rtc-sdk(?![\w/-])/u,
  },
];

function slash(value) {
  return String(value).replaceAll('\\', '/');
}

function shouldSkip(relativePath) {
  const normalized = slash(relativePath);
  if (skipPathFragments.some((fragment) => normalized.includes(fragment))) {
    return true;
  }
  if (allowlistPathFragments.some((fragment) => normalized.endsWith(fragment) || normalized.includes(`/${fragment}`))) {
    return true;
  }
  return false;
}

function listFiles(relativeRoot) {
  const absoluteRoot = path.join(repoRoot, relativeRoot);
  if (!absoluteRoot.startsWith(repoRoot)) {
    return [];
  }
  if (!fs.existsSync(absoluteRoot)) {
    return [];
  }
  const stat = fs.statSync(absoluteRoot);
  if (stat.isFile()) {
    return [relativeRoot];
  }
  const files = [];
  for (const entry of fs.readdirSync(absoluteRoot, { withFileTypes: true })) {
    const relativePath = path.join(relativeRoot, entry.name);
    if (entry.isDirectory()) {
      files.push(...listFiles(relativePath));
      continue;
    }
    files.push(relativePath);
  }
  return files;
}

function isTextCandidate(relativePath) {
  return /\.(?:md|mjs|json|yml|yaml|toml|rs|ps1|sh|cmd|ts|tsx|env\.example|txt)$/u.test(relativePath);
}

const violations = [];

for (const root of scanRoots) {
  for (const relativePath of listFiles(root)) {
    if (!isTextCandidate(relativePath) || shouldSkip(relativePath)) {
      continue;
    }
    const content = fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
    for (const banned of bannedPatterns) {
      if (banned.pattern.test(content)) {
        violations.push(`${slash(relativePath)}: ${banned.id}`);
      }
    }
  }
}

assert.equal(
  violations.length,
  0,
  `topology v2 baggage must be removed from active paths:\n${violations.join('\n')}`,
);

const localRtcSdkFamily = path.join(repoRoot, 'sdks', 'sdkwork-rtc-sdk');
assert.equal(
  fs.existsSync(localRtcSdkFamily),
  false,
  'sdks/sdkwork-rtc-sdk must not exist in sdkwork-im; RTC SDK authority lives in sibling ../sdkwork-rtc',
);

const postgresEnvExample = fs.readFileSync(path.join(repoRoot, '.env.postgres.example'), 'utf8');
assert.doesNotMatch(
  postgresEnvExample,
  /local-minimal|local-default|\.runtime\/local-/u,
  '.env.postgres.example must use topology v2 runtime paths only',
);
assert.match(
  postgresEnvExample,
  /SDKWORK_IM_RUNTIME_DIR=\.\/\.runtime\/standalone\.unified-process\.development/u,
  '.env.postgres.example must document the default dev profile runtime directory',
);

const rootReadme = fs.readFileSync(path.join(repoRoot, 'README.md'), 'utf8');
assert.doesNotMatch(
  rootReadme,
  /本地最小安装与运行|local-minimal-node|install-local/u,
  'README.md must not reference retired local-minimal lifecycle docs or scripts',
);
assert.match(
  rootReadme,
  /pnpm dev|topology-greenfield\.md|18079/u,
  'README.md must document topology v2 dev entrypoints',
);

const sdksReadme = fs.readFileSync(path.join(repoRoot, 'sdks', 'README.md'), 'utf8');
assert.match(
  sdksReadme,
  /\.\.\/sdkwork-rtc\/sdks\/sdkwork-rtc-sdk/u,
  'sdks/README.md must document RTC SDK authority in sibling ../sdkwork-rtc',
);
assert.match(
  sdksReadme,
  /must not.*be materialized under this repository/siu,
  'sdks/README.md must state RTC SDK must not live under local sdks/',
);
assert.match(
  sdksReadme,
  /copied into `sdkwork-im\/sdks\//u,
  'sdks/README.md must forbid copying RTC SDK into local sdks/',
);

const pcReadme = fs.readFileSync(
  path.join(repoRoot, 'apps', 'sdkwork-im-pc', 'README.md'),
  'utf8',
);
assert.doesNotMatch(
  pcReadme,
  /AI Studio|GEMINI_API_KEY|npm run dev/u,
  'apps/sdkwork-im-pc/README.md must not retain the retired AI Studio template',
);
assert.match(
  pcReadme,
  /pnpm dev|18079|topology-greenfield/u,
  'apps/sdkwork-im-pc/README.md must document topology v2 dev entrypoints',
);

const pcPackageJson = JSON.parse(
  fs.readFileSync(path.join(repoRoot, 'apps', 'sdkwork-im-pc', 'package.json'), 'utf8'),
);
assert.ok(
  !pcPackageJson.dependencies?.['@google/genai'],
  'apps/sdkwork-im-pc/package.json must not depend on retired @google/genai',
);

const pcScaffoldPaths = [
  'apps/sdkwork-im-pc/vite.config.ts',
  'apps/sdkwork-im-pc/local-api.ts',
  'apps/sdkwork-im-pc/server.ts',
  'apps/sdkwork-im-pc/.env.example',
];
for (const relativePath of pcScaffoldPaths) {
  const source = fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
  assert.doesNotMatch(
    source,
    /AI Studio|GEMINI_API_KEY|@google\/genai/u,
    `${relativePath} must not retain AI Studio or Gemini scaffold debt`,
  );
}

const legacyEvidenceDir = path.join(
  repoRoot,
  'artifacts',
  'releases',
  'wave-d-2026-04-08',
  'evidence',
  'local-default',
);
assert.equal(
  fs.existsSync(legacyEvidenceDir),
  false,
  'wave-d evidence must not keep a local-default directory name',
);

console.log('sdkwork-im topology baggage contract passed');
