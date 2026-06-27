import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(repoRoot, relativePath), 'utf8'));
}

const packageJson = readJson('package.json');

assert.equal(
  packageJson.scripts['deploy:source:plan'],
  'node scripts/release/run-sdkwork-im-source-server.mjs plan',
  'root deploy:source:plan must print the production source deployment plan',
);
assert.equal(
  packageJson.scripts['build:server:source'],
  'node scripts/release/run-sdkwork-im-source-server.mjs build',
  'root build:server:source must build production server artifacts from source without packaging',
);
assert.equal(
  packageJson.scripts['start:server:source'],
  'node scripts/release/run-sdkwork-im-source-server.mjs start',
  'root start:server:source must start the source-built server through the runtime lifecycle script',
);
assert.equal(
  packageJson.scripts['test:source-server-deploy'],
  'node scripts/release/sdkwork-im-source-server-command.test.mjs',
  'root test:source-server-deploy must verify the source deployment command contract',
);

const sourceServerModule = await import(
  pathToFileURL(path.join(repoRoot, 'scripts/release/run-sdkwork-im-source-server.mjs')).href
);

assert.equal(
  typeof sourceServerModule.createSdkworkImSourceServerPlan,
  'function',
  'source server command must expose an auditable plan creator',
);
assert.equal(
  typeof sourceServerModule.runSdkworkImSourceServerPlan,
  'function',
  'source server command must expose a plan runner for tests and package scripts',
);
assert.equal(
  typeof sourceServerModule.serializableSdkworkImSourceServerPlan,
  'function',
  'source server command must expose a secret-safe serializable plan',
);

const tempDir = path.join(repoRoot, '.runtime', 'source-server-command-test');
fs.mkdirSync(tempDir, { recursive: true });
const envFile = path.join(tempDir, 'server.env');
const configFile = path.join(tempDir, 'chat.toml');
fs.writeFileSync(
  envFile,
  [
    '# source deployment test env',
    'export SDKWORK_IM_DEPLOYMENT_MODE=server',
    'SDKWORK_IM_CONFIG_FILE=' + configFile.replaceAll('\\', '/'),
    'SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND=0.0.0.0:18080',
    'SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=https://im.sdkwork.com',
    'SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=wss://im.sdkwork.com',
    'SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=https://api.sdkwork.com',
    'SDKWORK_IM_DATABASE_PASSWORD=secret-password',
    '',
  ].join('\n'),
);

const buildPlan = sourceServerModule.createSdkworkImSourceServerPlan({
  action: 'build',
  env: {
    SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL: 'https://override.example.com',
  },
  envFile,
  platform: 'linux',
  repoRoot,
});

assert.equal(buildPlan.action, 'build');
assert.deepEqual(
  buildPlan.steps.map((step) => step.label),
  ['build sdkwork-im source server artifacts'],
  'source build plan must keep package creation out of the source deployment path',
);
assert.deepEqual(
  buildPlan.steps[0].args,
  ['run', 'release:build:prod', '--', '--target', 'server'],
  'source build plan must reuse the existing production server build without invoking release packaging',
);
assert.equal(
  buildPlan.steps[0].env.SDKWORK_IM_DEPLOYMENT_MODE,
  'server',
  'source build plan must load deployment mode from server.env',
);
assert.equal(
  buildPlan.steps[0].env.SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  'https://override.example.com',
  'explicit process env must override server.env when building public frontend base URLs',
);
assert.equal(
  buildPlan.steps[0].env.SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL,
  'wss://im.sdkwork.com',
  'source build plan must load websocket base URL from server.env',
);
assert.equal(
  buildPlan.steps[0].env.SDKWORK_IM_ADMIN_SITE_DIR,
  path.join(repoRoot, 'apps', 'sdkwork-im-pc', 'dist'),
  'source build plan must default admin static site assets to the source checkout dist directory',
);
assert.equal(
  buildPlan.steps[0].env.SDKWORK_IM_PORTAL_SITE_DIR,
  path.join(repoRoot, 'apps', 'sdkwork-im-pc', 'dist'),
  'source build plan must default portal static site assets to the source checkout dist directory',
);
assert.equal(
  buildPlan.steps[0].env.SDKWORK_IM_SERVER_BINARY_PATH,
  path.join(repoRoot, 'target', 'release', 'sdkwork-im-server'),
  'source build plan must default the runtime binary path to the release binary built in the source checkout',
);

const serializableBuildPlan = sourceServerModule.serializableSdkworkImSourceServerPlan(buildPlan);
assert.deepEqual(
  serializableBuildPlan.steps[0].envKeys,
  [
    'SDKWORK_IM_ADMIN_SITE_DIR',
    'SDKWORK_IM_PORTAL_SITE_DIR',
    'SDKWORK_IM_SERVER_BINARY_PATH',
    'SDKWORK_IM_CONFIG_FILE',
    'SDKWORK_IM_DEPLOYMENT_MODE',
    'SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND',
    'SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL',
    'SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL',
    'SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL',
  ],
  'source deployment plan JSON must expose only safe deployment keys and omit secret-bearing env values',
);
assert.ok(
  !JSON.stringify(serializableBuildPlan).includes('secret-password'),
  'source deployment plan JSON must not include secret values from server.env',
);

const startPlan = sourceServerModule.createSdkworkImSourceServerPlan({
  action: 'start',
  env: {},
  envFile,
  platform: 'linux',
  repoRoot,
});

assert.equal(startPlan.action, 'start');
assert.equal(startPlan.steps[0].command, 'bash');
assert.deepEqual(
  startPlan.steps[0].args,
  [
    path.join(repoRoot, 'bin', 'start-server.sh'),
    '--release',
    '--foreground',
    '--install-root',
    repoRoot,
    '--config-dir',
    tempDir,
    '--env-file',
    envFile,
    '--binary-path',
    path.join(repoRoot, 'target', 'release', 'sdkwork-im-server'),
  ],
  'Linux source start plan must reuse bin/start-server.sh in foreground mode for systemd-compatible operation',
);

const windowsStartPlan = sourceServerModule.createSdkworkImSourceServerPlan({
  action: 'start',
  env: {},
  envFile,
  platform: 'win32',
  repoRoot,
});

assert.equal(windowsStartPlan.steps[0].command, 'powershell.exe');
assert.deepEqual(
  windowsStartPlan.steps[0].args,
  [
    '-NoProfile',
    '-ExecutionPolicy',
    'Bypass',
    '-File',
    path.join(repoRoot, 'bin', 'start-server.ps1'),
    '-Release',
    '-Foreground',
    '-InstallRoot',
    repoRoot,
    '-ConfigDir',
    tempDir,
    '-EnvFile',
    envFile,
    '-BinaryPath',
    path.join(repoRoot, 'target', 'release', 'sdkwork-im-server.exe'),
  ],
  'Windows source start plan must reuse bin/start-server.ps1 with the source checkout release binary',
);

const spawnedSteps = [];
await sourceServerModule.runSdkworkImSourceServerPlan({
  plan: buildPlan,
  spawnImpl(command, args, options) {
    spawnedSteps.push({ args, command, cwd: options.cwd, env: options.env, shell: options.shell });
    return Promise.resolve({ code: 0 });
  },
});

assert.equal(spawnedSteps.length, 1);
assert.equal(spawnedSteps[0].command, buildPlan.steps[0].command);
assert.deepEqual(spawnedSteps[0].args, buildPlan.steps[0].args);
assert.equal(spawnedSteps[0].cwd, repoRoot);
assert.equal(
  spawnedSteps[0].env.SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  'https://override.example.com',
  'source deploy runner must execute the audited plan with the resolved deployment env',
);

const sourceDeployGuide = fs.readFileSync(
  path.join(repoRoot, 'docs', '部署', '源码部署.md'),
  'utf8',
);
const deploymentReadme = fs.readFileSync(
  path.join(repoRoot, 'docs', '部署', 'README.md'),
  'utf8',
);
assert.ok(
  sourceDeployGuide.includes('pnpm run build:server:source')
    && sourceDeployGuide.includes('pnpm run start:server:source')
    && sourceDeployGuide.includes('/etc/sdkwork/chat/server.env')
    && sourceDeployGuide.includes('SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL'),
  'source deployment guide must document the pnpm workflow and base URL source of truth',
);
assert.ok(
  deploymentReadme.includes('[源码部署](./源码部署.md)'),
  'deployment README must link the source deployment guide',
);

console.log('sdkwork-im source server command contract passed');
