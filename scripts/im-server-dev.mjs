import { spawn, spawnSync } from 'node:child_process';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { ensurePostgresDevDatabaseReady } from './dev/ensure-postgres-dev-database.mjs';
import { resolvePostgresDevProfile } from './dev/sdkwork-im-postgres-dev-profile.mjs';
import { resolveSdkworkImSharedDatabaseConfig } from './dev/sdkwork-im-shared-database.mjs';
import {
  createManagedSdkworkApiGatewayProcess,
  createSdkworkChatBrowserOrigins,
  createStandaloneGatewayProcess,
  isStandaloneUnifiedProcess,
  isSdkworkApiGatewayManagedExternally,
  resolveSdkworkApiGatewayBaseUrl,
  resolveSdkworkApiGatewayBind,
  resolveSdkworkChatPcDevServer,
} from './lib/im-pc-dev.mjs';
import { IAM_APPLICATION_BOOTSTRAP_ENV, resolveIamDevEnv, resolveImProductSiteDirEnv } from './lib/im-topology.mjs';
import {
  createSdkworkImServerCargoEnv,
  resolveSdkworkImServerBindEnv,
} from './dev/sdkwork-im-server-dev-runtime.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '..');
const runtimeSiteRoot = path.join(repoRoot, '.runtime', 'dev-sites');
const defaultBrowserOrigins = createSdkworkChatBrowserOrigins(resolveSdkworkChatPcDevServer());
const activeChildren = new Set();
let shuttingDown = false;

function createBaseEnv() {
  return {
    ...process.env,
  };
}

async function createRuntimeEnv(baseEnv) {
  const postgresProfile = resolvePostgresDevProfile({
    env: baseEnv,
    repoRoot,
  });
  const cargoEnv = createSdkworkImServerCargoEnv({
    env: {
      ...baseEnv,
      ...postgresProfile.env,
    },
    repoRoot,
  });
  const bindEnv = await resolveSdkworkImServerBindEnv({
    env: cargoEnv.env,
  });
  const sharedDatabaseEnv = resolveSdkworkImSharedDatabaseConfig({
    env: bindEnv.env,
    repoRoot,
  }).env;
  const iamDevEnv = resolveIamDevEnv({ ...bindEnv.env, ...sharedDatabaseEnv }, repoRoot);
  const env = {
    ...bindEnv.env,
    ...sharedDatabaseEnv,
    ...iamDevEnv,
    ...IAM_APPLICATION_BOOTSTRAP_ENV,
  };
  env.SDKWORK_IM_BROWSER_ORIGINS = env.SDKWORK_IM_BROWSER_ORIGINS ?? defaultBrowserOrigins;
  if (!env.SDKWORK_API_CLOUD_GATEWAY_BIND?.trim()) {
    env.SDKWORK_API_CLOUD_GATEWAY_BIND = resolveSdkworkApiGatewayBind(env);
  }
  if (!env.SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL?.trim()) {
    env.SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL = resolveSdkworkApiGatewayBaseUrl(env);
  }

  if (
    !env.SDKWORK_ADMIN_PROXY_TARGET?.trim()
    && !env.SDKWORK_ADMIN_BIND?.trim()
    && !env.SDKWORK_ADMIN_SANDBOX?.trim()
    && !env.SDKWORK_ADMIN_SANDBOX_MODE?.trim()
  ) {
    env.SDKWORK_ADMIN_SANDBOX = 'true';
    process.stdout.write('[sdkwork-im-server] SDKWORK_ADMIN_SANDBOX=true\n');
  }
  if (cargoEnv.usingDefaultTargetDir) {
    process.stdout.write(
      `[sdkwork-im-server] CARGO_TARGET_DIR=${path.relative(repoRoot, env.CARGO_TARGET_DIR)}\n`,
    );
  }
  if (bindEnv.portChanged) {
    process.stdout.write(
      `[sdkwork-im-server] 127.0.0.1:18079 is busy; using http://${bindEnv.bindAddr}\n`,
    );
  }

  return env;
}

function terminateProcessTree(childOrPid) {
  const pid = typeof childOrPid === 'number' ? childOrPid : childOrPid?.pid;
  if (!pid) {
    return;
  }

  if (process.platform === 'win32') {
    spawnSync('taskkill.exe', ['/PID', String(pid), '/T', '/F'], {
      stdio: 'ignore',
      windowsHide: true,
    });
    return;
  }

  try {
    if (typeof childOrPid !== 'number' && typeof childOrPid.kill === 'function') {
      childOrPid.kill('SIGTERM');
      return;
    }
    process.kill(pid, 'SIGTERM');
  } catch {
    // Process already exited.
  }
}

function shutdownActiveChildren() {
  if (shuttingDown) {
    return;
  }
  shuttingDown = true;
  for (const child of activeChildren) {
    terminateProcessTree(child);
  }
}

function escapePowerShellSingleQuotedString(value) {
  return String(value).replaceAll("'", "''");
}

function terminateStaleSdkworkImServerProcesses(env) {
  if (process.platform !== 'win32') {
    return;
  }

  const serverBinaryPath = path.join(
    env.CARGO_TARGET_DIR ?? path.join(repoRoot, 'target'),
    'debug',
    process.platform === 'win32' ? 'sdkwork-im-server.exe' : 'sdkwork-im-server',
  );
  const normalizedServerBinaryPath = path.resolve(serverBinaryPath).toLowerCase();
  const escapedServerBinaryPath = escapePowerShellSingleQuotedString(normalizedServerBinaryPath);
  const command = [
    '$target = ' + `'${escapedServerBinaryPath}'`,
    '$all = Get-CimInstance Win32_Process',
    '$all | Where-Object { $_.Name -eq "sdkwork-im-server.exe" -and $_.ExecutablePath -and $_.ExecutablePath.ToLowerInvariant() -eq $target }',
    '| ForEach-Object {',
    '  $server = $_;',
    '  $parent = $all | Where-Object { $_.ProcessId -eq $server.ParentProcessId } | Select-Object -First 1;',
    '  if ($parent -and $parent.Name -eq "cargo.exe" -and $parent.CommandLine -match "run\\s+-p\\s+sdkwork-im-cloud-gateway\\s+--bin\\s+sdkwork-im-server") {',
    '    $parent.ProcessId',
    '  } else {',
    '    $server.ProcessId',
    '  }',
    '}',
  ].join(' ');
  const result = spawnSync(
    'powershell.exe',
    ['-NoProfile', '-NonInteractive', '-Command', command],
    {
      encoding: 'utf8',
      windowsHide: true,
    },
  );

  if (result.error) {
    process.stderr.write(
      `[sdkwork-im-server] failed to inspect stale sdkwork-im-server.exe processes: ${result.error.message}\n`,
    );
    return;
  }

  const pids = String(result.stdout ?? '')
    .split(/\r?\n/u)
    .map((line) => Number.parseInt(line.trim(), 10))
    .filter((pid) => Number.isInteger(pid) && pid > 0 && pid !== process.pid);

  for (const pid of new Set(pids)) {
    process.stdout.write(
      `[sdkwork-im-server] terminating stale ${path.relative(repoRoot, serverBinaryPath)} process ${pid}\n`,
    );
    terminateProcessTree(pid);
  }
}

function spawnCommand(command, args, { cwd = repoRoot, env, label, shell = process.platform === 'win32' } = {}) {
  const child = spawn(command, args, {
    cwd,
    env,
    stdio: 'inherit',
    shell,
  });
  activeChildren.add(child);

  child.on('error', (error) => {
    process.stderr.write(`[${label}] ${error instanceof Error ? error.message : String(error)}\n`);
  });
  child.on('exit', () => {
    activeChildren.delete(child);
  });

  return child;
}

function runCommand(command, args, { env } = {}) {
  return new Promise((resolve, reject) => {
    const child = spawnCommand(command, args, { env, label: command });

    child.on('error', reject);

    child.on('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }

      reject(new Error(`${command} ${args.join(' ')} exited with code ${code ?? 'unknown'}`));
    });
  });
}

const buildEnv = createBaseEnv();
const runtimeEnv = await createRuntimeEnv(buildEnv);

process.once('SIGINT', () => {
  shutdownActiveChildren();
  process.exitCode = 130;
});
process.once('SIGTERM', () => {
  shutdownActiveChildren();
  process.exitCode = 143;
});

Object.assign(
  runtimeEnv,
  await resolveImProductSiteDirEnv({
    buildEnv,
    env: runtimeEnv,
    onFallback: ({ fallbackDir, label, sourceDir }) => {
      process.stdout.write(
        `[sdkwork-im-server] ${label} source not found at ${path.relative(repoRoot, sourceDir)}; using ${path.relative(repoRoot, fallbackDir)}\n`,
      );
    },
    repoRoot,
    runtimeSiteRoot,
  }),
);

const managedSdkworkApiGatewayProcess = isSdkworkApiGatewayManagedExternally(runtimeEnv)
  ? undefined
  : createManagedSdkworkApiGatewayProcess({
    env: runtimeEnv,
    repoRoot,
  });
if (managedSdkworkApiGatewayProcess) {
  spawnCommand(managedSdkworkApiGatewayProcess.command, managedSdkworkApiGatewayProcess.args, {
    cwd: managedSdkworkApiGatewayProcess.cwd,
    env: managedSdkworkApiGatewayProcess.env,
    label: managedSdkworkApiGatewayProcess.label,
    shell: managedSdkworkApiGatewayProcess.shell,
  });
}

terminateStaleSdkworkImServerProcesses(runtimeEnv);

await ensurePostgresDevDatabaseReady({
  env: runtimeEnv,
  repoRoot,
});

const standaloneGatewayProcess = isStandaloneUnifiedProcess(runtimeEnv)
  ? createStandaloneGatewayProcess({
    env: runtimeEnv,
    repoRoot,
  })
  : undefined;

if (standaloneGatewayProcess) {
  process.stdout.write('[sdkwork-im-server] using sdkwork-im-standalone-gateway for standalone unified-process IAM ingress\n');
  await runCommand(standaloneGatewayProcess.command, standaloneGatewayProcess.args, {
    env: standaloneGatewayProcess.env,
  });
} else {
  await runCommand('cargo', ['run', '-p', 'sdkwork-im-cloud-gateway', '--bin', 'sdkwork-im-server'], {
    env: runtimeEnv,
  });
}
