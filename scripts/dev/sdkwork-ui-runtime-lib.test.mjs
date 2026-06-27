import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  ensureSdkworkUiDist,
  resolveSdkworkUiPackageRoot,
} from './sdkwork-ui-runtime-lib.mjs';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');
const testRoot = path.join(repoRoot, '.runtime', 'sdkwork-ui-runtime-lib-test');
const appRoot = path.join(testRoot, 'apps', 'sdkwork-im-pc');
const uiPackageRoot = path.join(
  testRoot,
  '..',
  'sdkwork-ui',
  'sdkwork-ui-pc-react',
);

fs.rmSync(testRoot, { force: true, recursive: true });
fs.mkdirSync(appRoot, { recursive: true });
fs.mkdirSync(uiPackageRoot, { recursive: true });
fs.writeFileSync(path.join(appRoot, 'package.json'), '{"name":"@sdkwork/im-pc"}\n');
fs.writeFileSync(path.join(appRoot, 'pnpm-workspace.yaml'), 'packages:\n  - packages/*\n');
fs.writeFileSync(
  path.join(uiPackageRoot, 'package.json'),
  JSON.stringify(
    {
      name: '@sdkwork/ui-pc-react',
      dependencies: {
        '@sdkwork/drive-app-sdk': 'workspace:*',
        clsx: '^2.1.1',
      },
      devDependencies: {
        '@types/react': '^19.0.0',
        '@types/react-dom': '^19.0.0',
        vite: '^6.0.0',
      },
      peerDependencies: {
        react: '>=18.0.0 <20.0.0',
        'react-dom': '>=18.0.0 <20.0.0',
      },
    },
    null,
    2,
  ),
);

for (const entryPath of [
  'index.js',
  'theme.js',
  'components-ui.js',
  'ui-feedback.js',
  'patterns-app-shell.js',
  'patterns-desktop-shell.js',
  'sdkwork-ui.css',
]) {
  const filePath = path.join(uiPackageRoot, 'dist', entryPath);
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, '');
}

assert.equal(
  resolveSdkworkUiPackageRoot(appRoot),
  uiPackageRoot,
  'sdkwork ui package root must resolve through the sibling workspace dependency root',
);

const calls = [];
const resolvedRoot = ensureSdkworkUiDist({
  appRoot,
  runProcess(command, args, options) {
    calls.push({ args, command, cwd: options.cwd });
    const installPackageJson = JSON.parse(
      fs.readFileSync(path.join(options.cwd, 'package.json'), 'utf8'),
    );
    assert.equal(
      installPackageJson.name,
      '@sdkwork/im-pc',
      'sdkwork ui dependency installation must run from the app workspace root when pnpm-workspace.yaml is present',
    );
    assert.equal(
      installPackageJson.dependencies?.['@sdkwork/drive-app-sdk'],
      undefined,
      'app workspace install fixture must not rely on workspace-only SDK dependencies for this runtime dependency check',
    );
    for (const packageName of ['clsx', 'react', 'react-dom', '@types/react', '@types/react-dom']) {
      const packageRoot = path.join(uiPackageRoot, 'node_modules', ...packageName.split('/'));
      fs.mkdirSync(packageRoot, { recursive: true });
      fs.writeFileSync(path.join(packageRoot, 'package.json'), `{"name":"${packageName}"}\n`);
    }
    return { status: 0 };
  },
});

assert.equal(resolvedRoot, uiPackageRoot);
assert.equal(calls.length, 1, 'sdkwork ui runtime dependency materialization must install once');
assert.equal(
  path.relative(testRoot, calls[0].cwd).replaceAll('\\', '/'),
  'apps/sdkwork-im-pc',
  'sdkwork ui dependency packages must be installed from the app workspace root',
);
assert.match(
  [calls[0].command, ...calls[0].args].join(' '),
  /pnpm(?:\.cjs)?['"]?\s+['"]?install|pnpm(?:\.cmd)?\s+install/u,
  'sdkwork ui runtime dependency preparation must run pnpm install',
);
assert.ok(
  fs.existsSync(path.join(uiPackageRoot, 'node_modules', 'clsx', 'package.json')),
  'sdkwork ui package root must expose installed third-party runtime dependencies',
);
assert.deepEqual(
  calls.map((call) => ({
    cwd: path.relative(testRoot, call.cwd).replaceAll('\\', '/'),
  })),
  [
    {
      cwd: 'apps/sdkwork-im-pc',
    },
  ],
  'sdkwork ui dependency packages must be prepared through the chat PC app workspace',
);

console.log('sdkwork ui runtime dependency contract passed');
