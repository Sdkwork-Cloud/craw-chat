import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');
const appRoot = path.join(repoRoot, 'apps/sdkwork-im-pc');
const packagesRoot = path.join(appRoot, 'packages');

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(repoRoot, relativePath), 'utf8'));
}

function listPackageJsonFiles() {
  return fs
    .readdirSync(packagesRoot, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => path.join(packagesRoot, entry.name, 'package.json'))
    .filter((candidate) => fs.existsSync(candidate));
}

function listSourceFiles(packageDir) {
  const srcRoot = path.join(packageDir, 'src');
  if (!fs.existsSync(srcRoot)) {
    return [];
  }
  const files = [];
  const walk = (dir) => {
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      const fullPath = path.join(dir, entry.name);
      if (entry.isDirectory()) {
        walk(fullPath);
        continue;
      }
      if (/\.(ts|tsx)$/.test(entry.name)) {
        files.push(fullPath);
      }
    }
  };
  walk(srcRoot);
  return files;
}

function collectImportSpecifiers(source) {
  const specifiers = new Set();
  for (const match of source.matchAll(/(?:import|export)\s+(?:type\s+)?(?:[^'";]*?\sfrom\s+)?['"]([^'"]+)['"]/gu)) {
    specifiers.add(match[1]);
  }
  return specifiers;
}

function resolveDeclaredDependencyName(specifier) {
  if (specifier.startsWith('@sdkwork/')) {
    const segments = specifier.split('/');
    return segments.length >= 2 ? `${segments[0]}/${segments[1]}` : specifier;
  }
  if (specifier.startsWith('sdkwork-')) {
    return specifier.split('/')[0];
  }
  if (specifier.startsWith('.') || specifier.startsWith('@/')) {
    return null;
  }
  if (specifier.startsWith('motion/')) {
    return 'motion';
  }
  return specifier.split('/')[0];
}

const appPackageJson = readJson('apps/sdkwork-im-pc/package.json');
const h5PackageJson = readJson('apps/sdkwork-im-h5/package.json');

assert.equal(
  appPackageJson.workspaces,
  undefined,
  'apps/sdkwork-im-pc must not declare nested npm workspaces',
);
assert.equal(
  appPackageJson.scripts.lint,
  'node ../../scripts/dev/run-tsc-cli.mjs --noEmit -p tsconfig.app.json',
  'apps/sdkwork-im-pc lint must use tsconfig.app.json',
);
assert.ok(
  fs.existsSync(path.join(appRoot, 'tsconfig.app.json')),
  'apps/sdkwork-im-pc must provide tsconfig.app.json for IM-owned typecheck scope',
);
assert.equal(
  appPackageJson.pnpm?.overrides,
  undefined,
  'apps/sdkwork-im-pc must not declare pnpm.overrides; repository root owns overrides',
);

const forbiddenAppRootHoistPatterns = [
  /^@sdkwork\/agents-pc-/u,
  /^@sdkwork\/course-pc-/u,
  /^@sdkwork\/drive-pc-/u,
  /^@sdkwork\/knowledgebase-pc-/u,
  /^@sdkwork\/voice-pc-/u,
  /^sdkwork-drive-pc-/u,
  /^sdkwork-knowledgebase-pc-/u,
  /^sdkwork-voice-pc-/u,
];
for (const dependencyName of Object.keys(appPackageJson.dependencies ?? {})) {
  assert.ok(
    forbiddenAppRootHoistPatterns.every((pattern) => !pattern.test(dependencyName)),
    `apps/sdkwork-im-pc app root must not hoist domain facade ${dependencyName}; declare it on shell/core/feature packages instead`,
  );
}

const requiredAppRootDeps = [
  '@sdkwork-internal/im-app-api-generated',
  '@sdkwork-internal/im-backend-api-generated',
  '@sdkwork/appbase-pc-react',
  '@sdkwork/auth-runtime-pc-react',
  '@sdkwork/drive-app-sdk',
  '@sdkwork/im-pc-core',
  '@sdkwork/im-sdk',
  '@sdkwork/iam-sdk-ports',
  '@sdkwork/notary-app-sdk',
  '@sdkwork/rtc-sdk',
  '@sdkwork/ui-pc-react',
];
for (const dependencyName of requiredAppRootDeps) {
  assert.equal(
    appPackageJson.dependencies?.[dependencyName],
    'workspace:*',
    `apps/sdkwork-im-pc app root must declare ${dependencyName} with workspace:*`,
  );
}

assert.ok(
  h5PackageJson.dependencies['@sdkwork/im-h5-core'],
  'H5 reference app must keep a thin app-root dependency on its core package',
);
assert.ok(
  appPackageJson.dependencies['@sdkwork/im-pc-core'],
  'PC app root must depend on @sdkwork/im-pc-core as the SDK composition entry',
);

const corePackageJson = readJson('apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/package.json');
for (const dependencyName of [
  '@sdkwork/agents-app-sdk',
  '@sdkwork/agents-pc-agents',
  '@sdkwork/drive-app-sdk',
  '@sdkwork/im-sdk',
  '@sdkwork/utils',
  '@sdkwork/voice-app-sdk',
]) {
  assert.equal(
    corePackageJson.dependencies?.[dependencyName],
    'workspace:*',
    `@sdkwork/im-pc-core must declare ${dependencyName} for cross-repository SDK composition`,
  );
}

const shellPackageJson = readJson('apps/sdkwork-im-pc/packages/sdkwork-im-pc-shell/package.json');
for (const dependencyName of [
  '@sdkwork/drive-pc-drive',
  '@sdkwork/knowledgebase-pc-knowledge',
  '@sdkwork/course-pc-course',
  '@sdkwork/notary-pc-notary',
  '@sdkwork/voice-pc-market',
  '@sdkwork/voice-pc-speech',
]) {
  assert.equal(
    shellPackageJson.dependencies?.[dependencyName],
    'workspace:*',
    `@sdkwork/im-pc-shell must declare ${dependencyName} for capability module loaders`,
  );
}

const catalogThirdPartyNames = new Set([
  'react',
  'react-dom',
  'lucide-react',
  'motion',
  'i18next',
  'react-i18next',
  'clsx',
  'tailwind-merge',
  'dompurify',
  'framer-motion',
  'react-qr-code',
  'react-router-dom',
]);

for (const packageJsonPath of listPackageJsonFiles()) {
  const relativePath = path.relative(repoRoot, packageJsonPath);
  const workspacePackage = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
  const packageName = workspacePackage.name;
  const declared = new Set(Object.keys(workspacePackage.dependencies ?? {}));

  assert.notEqual(
    JSON.stringify(workspacePackage.dependencies ?? {}),
    '{}',
    `${relativePath} must not use empty dependencies; APP_COMPOSITION_SPEC forbids app-root hoisting compensation`,
  );

  for (const sectionName of ['dependencies', 'devDependencies', 'peerDependencies']) {
    for (const [name, version] of Object.entries(workspacePackage[sectionName] ?? {})) {
      if (name.startsWith('@sdkwork/im-')) {
        assert.equal(
          version,
          'workspace:*',
          `${relativePath} ${sectionName}.${name} must use workspace:*`,
        );
      }
      if (catalogThirdPartyNames.has(name)) {
        assert.equal(
          version,
          'catalog:',
          `${relativePath} ${sectionName}.${name} must use catalog:`,
        );
      }
    }
  }

  if (packageName === '@sdkwork/im-pc-desktop') {
    continue;
  }

  const packageDir = path.dirname(packageJsonPath);
  const importSpecifiers = new Set();
  for (const sourceFile of listSourceFiles(packageDir)) {
    const source = fs.readFileSync(sourceFile, 'utf8');
    for (const specifier of collectImportSpecifiers(source)) {
      importSpecifiers.add(specifier);
    }
  }

  for (const specifier of importSpecifiers) {
    const dependencyName = resolveDeclaredDependencyName(specifier);
    if (!dependencyName || dependencyName.startsWith('node:')) {
      continue;
    }
    if (dependencyName.startsWith('@sdkwork/im-pc-') && dependencyName !== packageName) {
      assert.ok(
        declared.has(dependencyName),
        `${relativePath} imports ${specifier} but does not declare ${dependencyName} in dependencies`,
      );
      continue;
    }
    if (
      dependencyName.startsWith('@sdkwork/')
      || dependencyName.startsWith('sdkwork-')
      || catalogThirdPartyNames.has(dependencyName)
    ) {
      assert.ok(
        declared.has(dependencyName),
        `${relativePath} imports ${specifier} but does not declare ${dependencyName} in dependencies`,
      );
    }
  }
}

console.log('sdkwork-im-pc architecture standard: ok');
