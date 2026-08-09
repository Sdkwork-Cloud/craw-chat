import { spawnSync } from 'node:child_process';
import process from 'node:process';

export const STALE_DEV_GATEWAY_PROCESS_NAMES = [
  'sdkwork-api-im-standalone-gateway.exe',
  'sdkwork-cloudrouter-standalone-gateway.exe',
];

function taskkillDetail(result) {
  const stderr = String(result.stderr ?? '').trim();
  const stdout = String(result.stdout ?? '').trim();
  return stderr || stdout;
}

function isProcessMissing(result) {
  // taskkill exits 128 when the target does not exist (locale-independent);
  // the text check covers systems that report "not found" with other codes.
  return result.status === 128 || /没有找到进程|not found|not running/iu.test(taskkillDetail(result));
}

export function windowsListeningPids(ports, { run = spawnSync } = {}) {
  const portSet = new Set(ports.map(Number));
  if (portSet.size === 0) {
    return new Set();
  }
  const result = run('netstat.exe', ['-ano', '-p', 'tcp'], {
    encoding: 'utf8',
    windowsHide: true,
  });
  if (result.error || result.status !== 0) {
    return new Set();
  }
  const pids = new Set();
  for (const line of String(result.stdout ?? '').split(/\r?\n/u)) {
    const match = line.match(/^\s*TCP\s+\S+:(\d+)\s+\S+\s+LISTENING\s+(\d+)\s*$/u);
    if (match && portSet.has(Number(match[1]))) {
      pids.add(Number(match[2]));
    }
  }
  return pids;
}

/**
 * Terminate stale Windows standalone gateway processes that keep PostgreSQL
 * connections open and block sdkwork-database bootstrap during dev startup.
 * Kill-by-image covers renamed-or-not gateway binaries; failures are reported
 * instead of swallowed so access-denied cases surface in the dev log.
 */
export function terminateStaleDevGatewayProcesses({
  platform = process.platform,
  processNames = STALE_DEV_GATEWAY_PROCESS_NAMES,
  spawnSyncImpl = spawnSync,
  stdout = process.stdout,
} = {}) {
  if (platform !== 'win32') {
    return { terminated: [] };
  }

  const terminated = [];
  for (const imageName of processNames) {
    const result = spawnSyncImpl('taskkill.exe', ['/F', '/IM', imageName], {
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
    });
    if (result.status === 0) {
      terminated.push(imageName);
      stdout.write(`[sdkwork-postgres] terminated stale ${imageName} process(es)\n`);
    } else if (!isProcessMissing(result)) {
      stdout.write(`[sdkwork-postgres] could not terminate stale ${imageName}: ${taskkillDetail(result)}\n`);
    }
  }
  return { terminated };
}

/**
 * Force-kill any Windows process listening on the given gateway ports and
 * verify the bindings are actually released. This covers stale listeners that
 * are not one of the known gateway image names (renamed binaries, wrapper
 * processes, leftover dev servers). Returns once the ports are free or after
 * the bounded retry window, mirroring what the dev supervisor expects of a
 * clean restart.
 */
export async function terminateStaleGatewayPortListeners({
  ports = [],
  platform = process.platform,
  spawnSyncImpl = spawnSync,
  listListeningPids = windowsListeningPids,
  stdout = process.stdout,
  maxAttempts = 10,
  waitMs = 300,
} = {}) {
  if (platform !== 'win32' || ports.length === 0) {
    return { terminated: [] };
  }

  const terminated = [];
  for (let attempt = 0; attempt < maxAttempts; attempt += 1) {
    const pidsByPort = new Map();
    let pending = 0;
    for (const port of ports) {
      const pids = [...listListeningPids([port])].filter((pid) => pid !== process.pid);
      if (pids.length > 0) {
        pidsByPort.set(port, pids);
        pending += pids.length;
      }
    }
    if (pending === 0) {
      break;
    }
    for (const [port, pids] of pidsByPort) {
      for (const pid of pids) {
        const result = spawnSyncImpl('taskkill.exe', ['/F', '/PID', String(pid)], {
          encoding: 'utf8',
          stdio: ['ignore', 'pipe', 'pipe'],
          windowsHide: true,
        });
        terminated.push({ port, pid });
        if (result.status === 0) {
          stdout.write(`[sdkwork-postgres] terminated stale gateway listener PID ${pid} on port ${port}\n`);
        } else if (!isProcessMissing(result)) {
          stdout.write(
            `[sdkwork-postgres] could not terminate stale gateway listener PID ${pid} on port ${port}: ${taskkillDetail(result)}\n`,
          );
        }
      }
    }
    await new Promise((resolve) => setTimeout(resolve, waitMs));
  }
  return { terminated };
}
