#!/usr/bin/env node

import { spawn } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(__filename), '..', '..');

const SAFE_PLAN_ENV_KEYS = Object.freeze([
  'SDKWORK_IM_ADMIN_SITE_DIR',
  'SDKWORK_IM_PORTAL_SITE_DIR',
  'SDKWORK_IM_SERVER_BINARY_PATH',
  'SDKWORK_IM_CONFIG_FILE',
  'SDKWORK_IM_DEPLOYMENT_MODE',
  'SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND',
  'SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL',
  'SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL',
  'SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL',
]);

function pnpmCommand(platform = process.platform) {
  return platform === 'win32' ? 'pnpm.cmd' : 'pnpm';
}

function printHelp() {
  console.log(`Usage: node scripts/release/run-sdkwork-im-source-server.mjs <plan|build|start> [options]

Build or start a production Sdkwork IM server directly from a source checkout.

Actions:
  plan                    Print the source deployment plan only.
  build                   Build web assets and the release sdkwork-im-server binary.
  start                   Start the source-built release binary through bin/start-server.

Options:
  --env-file <path>       Runtime env file. Defaults to <config-dir>/server.env.
  --config-dir <path>     Runtime config directory. Defaults to /etc/sdkwork/chat on Linux/macOS or ProgramData on Windows.
  --install-root <path>   Source checkout/install root. Defaults to this repository root.
  --binary-path <path>    Release binary path. Defaults to target/release/sdkwork-im-server.
  --background            Start as a background process instead of foreground/systemd mode.
  --health-url <url>      Health check URL forwarded to bin/start-server.
  --skip-health-check     Skip bin/start-server background health check.
  --json                  Print machine-readable JSON without secret values.
  --dry-run               Print the plan without executing build/start.
  -h, --help              Show help.
`);
}

function requireValue(argv, index, flag) {
  const value = argv[index + 1];
  if (!value || value.startsWith('--')) {
    throw new Error(`${flag} requires a value`);
  }
  return value;
}

function defaultConfigDir(platform = process.platform, env = process.env) {
  if (platform === 'win32') {
    return path.join(env.ProgramData || 'C:\\ProgramData', 'sdkwork', 'chat');
  }
  return '/etc/sdkwork/chat';
}

function parseSourceServerArgs(argv = process.argv.slice(2), { env = process.env, platform = process.platform } = {}) {
  const settings = {
    action: 'plan',
    background: false,
    binaryPath: null,
    configDir: null,
    dryRun: false,
    envFile: null,
    healthUrl: null,
    help: false,
    installRoot: repoRoot,
    json: false,
    platform,
    skipHealthCheck: false,
  };

  let actionSeen = false;
  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    switch (arg) {
      case 'plan':
      case 'build':
      case 'start':
        if (actionSeen) {
          throw new Error(`Only one source server action is allowed, got ${arg}`);
        }
        settings.action = arg;
        actionSeen = true;
        break;
      case '--env-file':
        settings.envFile = path.resolve(requireValue(argv, index, arg));
        index += 1;
        break;
      case '--config-dir':
        settings.configDir = path.resolve(requireValue(argv, index, arg));
        index += 1;
        break;
      case '--install-root':
        settings.installRoot = path.resolve(requireValue(argv, index, arg));
        index += 1;
        break;
      case '--binary-path':
        settings.binaryPath = path.resolve(requireValue(argv, index, arg));
        index += 1;
        break;
      case '--background':
        settings.background = true;
        break;
      case '--health-url':
        settings.healthUrl = requireValue(argv, index, arg);
        index += 1;
        break;
      case '--skip-health-check':
        settings.skipHealthCheck = true;
        break;
      case '--json':
        settings.json = true;
        break;
      case '--dry-run':
        settings.dryRun = true;
        break;
      case '--help':
      case '-h':
        settings.help = true;
        break;
      default:
        throw new Error(`Unsupported source server option: ${arg}`);
    }
  }

  if (!settings.configDir && settings.envFile) {
    settings.configDir = path.dirname(settings.envFile);
  }
  if (!settings.configDir) {
    settings.configDir = defaultConfigDir(platform, env);
  }
  if (!settings.envFile) {
    settings.envFile = path.join(settings.configDir, 'server.env');
  }

  return settings;
}

function readSourceServerEnvFile(envFilePath) {
  const values = {};
  if (!envFilePath || !fs.existsSync(envFilePath)) {
    return values;
  }

  for (const rawLine of fs.readFileSync(envFilePath, 'utf8').split(/\r?\n/u)) {
    let line = rawLine.trim();
    if (!line || line.startsWith('#')) {
      continue;
    }
    if (line.startsWith('export ')) {
      line = line.slice('export '.length).trim();
    }
    const separatorIndex = line.indexOf('=');
    if (separatorIndex <= 0) {
      continue;
    }
    const key = line.slice(0, separatorIndex).trim();
    let value = line.slice(separatorIndex + 1).trim();
    if (!key) {
      continue;
    }
    if (value.length >= 2) {
      const first = value.at(0);
      const last = value.at(-1);
      if ((first === '"' && last === '"') || (first === "'" && last === "'")) {
        value = value.slice(1, -1);
      }
    }
    values[key] = value;
  }
  return values;
}

function mergeEnvValues(...sources) {
  const merged = {};
  for (const source of sources) {
    for (const [key, value] of Object.entries(source ?? {})) {
      if (value !== undefined && value !== null) {
        merged[key] = String(value);
      }
    }
  }
  return merged;
}

function defaultBinaryPath(root, platform = process.platform) {
  return path.join(root, 'target', 'release', platform === 'win32' ? 'sdkwork-im-server.exe' : 'sdkwork-im-server');
}

function createResolvedSourceServerEnv({
  env = process.env,
  envFile,
  configDir,
  repoRoot: root = repoRoot,
  binaryPath = null,
  platform = process.platform,
} = {}) {
  const fileEnv = readSourceServerEnvFile(envFile);
  const sourceDistDir = path.join(root, 'apps', 'sdkwork-im-pc', 'dist');
  const resolvedBinaryPath = binaryPath
    || env.SDKWORK_IM_SERVER_BINARY_PATH
    || fileEnv.SDKWORK_IM_SERVER_BINARY_PATH
    || defaultBinaryPath(root, platform);
  const merged = mergeEnvValues(
    fileEnv,
    env,
    {
      SDKWORK_IM_ADMIN_SITE_DIR: env.SDKWORK_IM_ADMIN_SITE_DIR
        || fileEnv.SDKWORK_IM_ADMIN_SITE_DIR
        || sourceDistDir,
      SDKWORK_IM_PORTAL_SITE_DIR: env.SDKWORK_IM_PORTAL_SITE_DIR
        || fileEnv.SDKWORK_IM_PORTAL_SITE_DIR
        || sourceDistDir,
      SDKWORK_IM_SERVER_BINARY_PATH: resolvedBinaryPath,
      SDKWORK_IM_CONFIG_FILE: env.SDKWORK_IM_CONFIG_FILE
        || fileEnv.SDKWORK_IM_CONFIG_FILE
        || path.join(configDir, 'chat.toml'),
      SDKWORK_IM_DEPLOYMENT_MODE: env.SDKWORK_IM_DEPLOYMENT_MODE
        || fileEnv.SDKWORK_IM_DEPLOYMENT_MODE
        || 'server',
    },
  );

  return merged;
}

function createBuildStep({ env, platform, root }) {
  return {
    label: 'build sdkwork-im source server artifacts',
    command: pnpmCommand(platform),
    args: ['run', 'release:build:prod', '--', '--target', 'server'],
    cwd: root,
    env,
    shell: false,
  };
}

function createStartStep({
  background = false,
  binaryPath,
  configDir,
  env,
  envFile,
  healthUrl = null,
  installRoot,
  platform = process.platform,
  repoRoot: root = repoRoot,
  skipHealthCheck = false,
}) {
  if (platform === 'win32') {
    const args = [
      '-NoProfile',
      '-ExecutionPolicy',
      'Bypass',
      '-File',
      path.join(root, 'bin', 'start-server.ps1'),
      '-Release',
    ];
    if (!background) {
      args.push('-Foreground');
    }
    args.push(
      '-InstallRoot',
      installRoot,
      '-ConfigDir',
      configDir,
      '-EnvFile',
      envFile,
      '-BinaryPath',
      binaryPath,
    );
    if (healthUrl) {
      args.push('-HealthUrl', healthUrl);
    }
    if (skipHealthCheck) {
      args.push('-SkipHealthCheck');
    }
    return {
      label: 'start sdkwork-im source server',
      command: 'powershell.exe',
      args,
      cwd: root,
      env,
      shell: false,
    };
  }

  const args = [
    path.join(root, 'bin', 'start-server.sh'),
    '--release',
  ];
  if (!background) {
    args.push('--foreground');
  }
  args.push(
    '--install-root',
    installRoot,
    '--config-dir',
    configDir,
    '--env-file',
    envFile,
    '--binary-path',
    binaryPath,
  );
  if (healthUrl) {
    args.push('--health-url', healthUrl);
  }
  if (skipHealthCheck) {
    args.push('--skip-health-check');
  }
  return {
    label: 'start sdkwork-im source server',
    command: 'bash',
    args,
    cwd: root,
    env,
    shell: false,
  };
}

function createSdkworkImSourceServerPlan({
  action = 'plan',
  background = false,
  binaryPath = null,
  configDir = null,
  env = process.env,
  envFile = null,
  healthUrl = null,
  installRoot = null,
  platform = process.platform,
  repoRoot: root = repoRoot,
  skipHealthCheck = false,
} = {}) {
  if (!['plan', 'build', 'start'].includes(action)) {
    throw new Error(`Unsupported source server action: ${action}`);
  }

  const resolvedConfigDir = configDir
    ? path.resolve(configDir)
    : envFile
      ? path.dirname(path.resolve(envFile))
      : defaultConfigDir(platform, env);
  const resolvedEnvFile = envFile ? path.resolve(envFile) : path.join(resolvedConfigDir, 'server.env');
  const resolvedInstallRoot = installRoot ? path.resolve(installRoot) : root;
  const resolvedEnv = createResolvedSourceServerEnv({
    binaryPath: binaryPath ? path.resolve(binaryPath) : null,
    configDir: resolvedConfigDir,
    env,
    envFile: resolvedEnvFile,
    platform,
    repoRoot: root,
  });
  const resolvedBinaryPath = resolvedEnv.SDKWORK_IM_SERVER_BINARY_PATH;

  const buildStep = createBuildStep({
    env: resolvedEnv,
    platform,
    root,
  });
  const startStep = createStartStep({
    background,
    binaryPath: resolvedEnv.SDKWORK_IM_SERVER_BINARY_PATH,
    configDir: resolvedConfigDir,
    env: resolvedEnv,
    envFile: resolvedEnvFile,
    healthUrl,
    installRoot: resolvedInstallRoot,
    platform,
    repoRoot: root,
    skipHealthCheck,
  });

  const steps = action === 'build'
    ? [buildStep]
    : action === 'start'
      ? [startStep]
      : [buildStep, startStep];

  return {
    schemaVersion: '2026-06-14.sdkwork-im.source-server.v1',
    action,
    background,
    configDir: resolvedConfigDir,
    envFile: resolvedEnvFile,
    installRoot: resolvedInstallRoot,
    platform,
    repoRoot: root,
    steps,
  };
}

function renderSdkworkImSourceServerPlan(plan) {
  return [
    `[sdkwork-im-source-server] action: ${plan.action}`,
    `[sdkwork-im-source-server] repo root: ${plan.repoRoot}`,
    `[sdkwork-im-source-server] config dir: ${plan.configDir}`,
    `[sdkwork-im-source-server] env file: ${plan.envFile}`,
    `[sdkwork-im-source-server] install root: ${plan.installRoot}`,
    ...plan.steps.map((step) => `[sdkwork-im-source-server]   ${step.label}: ${step.command} ${step.args.join(' ')}`),
  ];
}

function selectedSourceDeployEnvKeys(env = {}) {
  return SAFE_PLAN_ENV_KEYS.filter((key) => Object.hasOwn(env, key));
}

function serializableSdkworkImSourceServerPlan(plan) {
  return {
    ...plan,
    steps: plan.steps.map((step) => {
      const { env, ...publicStep } = step;
      return {
        ...publicStep,
        envKeys: selectedSourceDeployEnvKeys(env),
      };
    }),
  };
}

function spawnStep(command, args, options) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd: options.cwd,
      env: options.env,
      shell: options.shell,
      stdio: 'inherit',
      windowsHide: process.platform === 'win32',
    });
    child.on('error', reject);
    child.on('exit', (code, signal) => {
      resolve({ code, signal });
    });
  });
}

async function runSdkworkImSourceServerPlan({
  plan,
  spawnImpl = spawnStep,
} = {}) {
  if (!plan) {
    throw new Error('runSdkworkImSourceServerPlan requires a plan');
  }

  for (const step of plan.steps) {
    console.error(`[sdkwork-im-source-server] ${step.label}: ${step.command} ${step.args.join(' ')}`);
    const result = await spawnImpl(step.command, step.args, {
      cwd: step.cwd,
      env: step.env,
      shell: step.shell,
    });
    if (result?.error) {
      throw new Error(`${step.label} failed: ${result.error.message}`);
    }
    if (result?.signal) {
      throw new Error(`${step.label} exited with signal ${result.signal}`);
    }
    const exitCode = result?.code ?? result?.status ?? 0;
    if (exitCode !== 0) {
      throw new Error(`${step.label} exited with code ${exitCode}`);
    }
  }
}

async function main(argv = process.argv.slice(2)) {
  const settings = parseSourceServerArgs(argv);
  if (settings.help) {
    printHelp();
    return 0;
  }

  const plan = createSdkworkImSourceServerPlan({
    ...settings,
    env: process.env,
    repoRoot,
  });

  if (settings.json) {
    console.log(JSON.stringify({
      ok: true,
      dryRun: settings.dryRun || settings.action === 'plan',
      plan: serializableSdkworkImSourceServerPlan(plan),
    }, null, 2));
  } else {
    for (const line of renderSdkworkImSourceServerPlan(plan)) {
      console.log(line);
    }
  }

  if (settings.dryRun || settings.action === 'plan') {
    return 0;
  }

  await runSdkworkImSourceServerPlan({ plan });
  return 0;
}

if (process.argv[1] && path.resolve(process.argv[1]) === __filename) {
  main().then((code) => {
    process.exitCode = code;
  }).catch((error) => {
    console.error(`[sdkwork-im-source-server] ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  });
}

export {
  createSdkworkImSourceServerPlan,
  createResolvedSourceServerEnv,
  defaultConfigDir,
  defaultBinaryPath,
  main,
  parseSourceServerArgs,
  pnpmCommand,
  readSourceServerEnvFile,
  renderSdkworkImSourceServerPlan,
  runSdkworkImSourceServerPlan,
  serializableSdkworkImSourceServerPlan,
};
